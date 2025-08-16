use futures_util::{SinkExt, StreamExt};
use rustls::{ClientConfig, RootCertStore};
use serde::Deserialize;
use serde_json::value::RawValue;
use std::sync::Arc;
use tokio::sync::{mpsc, watch};
use tokio_tungstenite::{Connector, connect_async_tls_with_config, tungstenite::protocol::Message};

use crate::gateway::Gateway;
use crate::log;
use crate::models::gateway::{
    GatewayDispatch, GatewayDispatchEvents as GwEvt, GatewayIntents, GatewayOpcodes,
};

const DISCORD_GATEWAY_URL: &str = "wss://gateway.discord.gg/?v=10&encoding=json";

#[derive(thiserror::Error, Debug)]
pub enum ResumeError {
    #[error("invalid session on resume (resumable={resumable})")]
    InvalidSession { resumable: bool },
    #[error(transparent)]
    Transport(#[from] anyhow::Error),
}

#[derive(Deserialize)]
struct FrameBorrowed<'a> {
    op: i64,
    #[serde(default)]
    s: Option<i64>,
    #[serde(default)]
    t: Option<&'a str>,
    /// Borrowed raw payload slice.
    #[serde(borrow)]
    d: Option<&'a RawValue>,
}

#[derive(Deserialize)]
struct HelloD {
    heartbeat_interval: u64,
}

fn map_event(name: &str) -> Option<GwEvt> {
    serde_json::from_str::<GwEvt>(&format!("\"{name}\"")).ok()
}

pub async fn connect(token: &str, intents: GatewayIntents) -> anyhow::Result<Gateway> {
    log!("GW", "connecting to discord gateway");

    // TLS
    let provider = rustls::crypto::ring::default_provider().into();
    let mut root_store = RootCertStore::empty();
    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    let config = ClientConfig::builder_with_provider(provider)
        .with_safe_default_protocol_versions()?
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let connector = Connector::Rustls(Arc::new(config));

    // WS
    let (ws_stream, _) =
        connect_async_tls_with_config(DISCORD_GATEWAY_URL, None, true, Some(connector)).await?;
    log!("OK", "connection established");
    let (mut write, mut read) = ws_stream.split();

    // Single writer task
    let (writer_tx, mut writer_rx) = mpsc::unbounded_channel::<Message>();
    tokio::spawn(async move {
        while let Some(msg) = writer_rx.recv().await {
            if let Err(e) = write.send(msg).await {
                log!("ERR", "[writer] send error: {e}");
                break;
            }
        }
    });

    // Aux channels
    let (immediate_tx, immediate_rx) = mpsc::channel::<()>(8);
    let (last_seq_tx, last_seq_rx) = watch::channel::<Option<i64>>(None);
    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    // 1) HELLO
    let heartbeat_interval_ms = loop {
        let Some(frame) = read.next().await else {
            let _ = shutdown_tx.send(true);
            anyhow::bail!("gateway closed before HELLO");
        };
        let Message::Text(text) = frame? else {
            continue;
        };
        let Ok(f) = serde_json::from_str::<FrameBorrowed>(&text) else {
            continue;
        };
        if f.op == 10 {
            let d = f.d.ok_or_else(|| anyhow::anyhow!("HELLO without d"))?;
            let hello = serde_json::from_str::<HelloD>(d.get())?;
            log!(
                "OK",
                "received HELLO, heartbeat_interval={}ms",
                hello.heartbeat_interval
            );
            break hello.heartbeat_interval;
        }
    };

    // 2) heartbeat
    tokio::spawn(crate::gateway::heartbeat::run_heartbeat(
        writer_tx.clone(),
        heartbeat_interval_ms,
        immediate_rx,
        last_seq_rx.clone(),
        shutdown_rx,
    ));

    // 3) IDENTIFY
    let identify = serde_json::json!({
        "op": 2,
        "d": {
            "token": token,
            "intents": intents.bits(),
            "properties": { "os": "linux", "browser": "discord-ferris", "device": "discord-ferris" }
        }
    });
    writer_tx.send(Message::Text(identify.to_string().into()))?;
    log!("GW", "payload op2 (identify) queued");

    // Dispatch channel (raw `d`)
    let (events_tx, events_rx) = mpsc::unbounded_channel::<GatewayDispatch<Box<RawValue>>>();

    // 4) Read until READY
    let mut session_id: Option<String> = None;
    let mut resume_gateway_url: Option<String> = None;

    loop {
        let Some(msg) = read.next().await else {
            let _ = shutdown_tx.send(true);
            anyhow::bail!("gateway closed before READY");
        };
        let Message::Text(text) = msg? else { continue };
        let Ok(f) = serde_json::from_str::<FrameBorrowed>(&text) else {
            continue;
        };

        if let Some(s) = f.s {
            let _ = last_seq_tx.send_replace(Some(s));
        }

        match f.op {
            0 => {
                let Some(t) = f.t else { continue };
                if t == "READY" {
                    if let Some(d) = f.d {
                        #[derive(Deserialize)]
                        struct ReadyD<'a> {
                            session_id: &'a str,
                            #[serde(default)]
                            resume_gateway_url: Option<&'a str>,
                        }
                        if let Ok(r) = serde_json::from_str::<ReadyD>(d.get()) {
                            session_id = Some(r.session_id.to_string());
                            resume_gateway_url = Some(
                                r.resume_gateway_url
                                    .unwrap_or(DISCORD_GATEWAY_URL)
                                    .to_string(),
                            );
                        }
                    }
                    log!("EVT", "READY");
                    let ev = GatewayDispatch {
                        op: GatewayOpcodes::Dispatch,
                        t: GwEvt::Ready,
                        s: f.s.unwrap_or(0),
                        d: Box::from(RawValue::from_string(f.d.unwrap().get().to_owned()).unwrap()),
                    };
                    let _ = events_tx.send(ev);
                    break;
                } else if let Some(evt) = map_event(t) {
                    let ev = GatewayDispatch {
                        op: GatewayOpcodes::Dispatch,
                        t: evt,
                        s: f.s.unwrap_or(0),
                        d: Box::from(RawValue::from_string(f.d.unwrap().get().to_owned()).unwrap()),
                    };
                    let _ = events_tx.send(ev);
                }
            }
            1 => {
                let _ = immediate_tx.try_send(());
            } // HEARTBEAT request
            7 => {
                log!("WARN", "[gw] RECONNECT requested");
            }
            9 => {
                log!("WARN", "[gw] INVALID_SESSION: {:?}", f.d.map(|d| d.get()));
            }
            11 => {}
            _ => {}
        }
    }

    let session_id = session_id.unwrap_or_default();
    let resume_gateway_url = resume_gateway_url.unwrap_or_else(|| DISCORD_GATEWAY_URL.to_string());

    // Background reader after READY
    let events_tx_bg = events_tx.clone();
    let last_seq_tx_bg = last_seq_tx.clone();
    let immediate_tx_bg = immediate_tx.clone();
    tokio::spawn(async move {
        while let Some(msg) = read.next().await {
            let Ok(Message::Text(text)) = msg else {
                continue;
            };
            let Ok(f) = serde_json::from_str::<FrameBorrowed>(&text) else {
                continue;
            };

            if let Some(s) = f.s {
                let _ = last_seq_tx_bg.send_replace(Some(s));
            }

            match f.op {
                0 => {
                    if let (Some(t), Some(d)) = (f.t, f.d) {
                        if let Some(evt) = map_event(t) {
                            let ev = GatewayDispatch {
                                op: GatewayOpcodes::Dispatch,
                                t: evt,
                                s: f.s.unwrap_or(0),
                                d: Box::from(RawValue::from_string(d.get().to_owned()).unwrap()),
                            };
                            let _ = events_tx_bg.send(ev);
                        }
                    }
                }
                1 => {
                    let _ = immediate_tx_bg.try_send(());
                }
                7 => {
                    log!("WARN", "[gw] RECONNECT requested");
                }
                9 => {
                    log!("WARN", "[gw] INVALID_SESSION during running");
                }
                11 => {}
                _ => {}
            }
        }
        log!("WARN", "[reader] stream closed");
    });

    Ok(Gateway {
        session_id,
        resume_gateway_url,
        writer_tx,
        events_rx,
        last_seq_rx,
        shutdown_tx,
    })
}

pub async fn resume(
    token: &str,
    session_id: &str,
    resume_gateway_url: &str,
    last_seq: Option<i64>,
) -> Result<Gateway, ResumeError> {
    log!(
        "GW",
        "resuming session_id={} at {}",
        session_id,
        resume_gateway_url
    );

    // TLS
    let provider = rustls::crypto::ring::default_provider().into();
    let mut root_store = RootCertStore::empty();
    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    let config = ClientConfig::builder_with_provider(provider)
        .with_safe_default_protocol_versions()
        .map_err(|e| ResumeError::Transport(anyhow::anyhow!(e)))?
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let connector = Connector::Rustls(Arc::new(config));

    // WS
    let resume_url = normalize_gateway_url(resume_gateway_url);
    let (ws_stream, _) = connect_async_tls_with_config(resume_url, None, true, Some(connector))
        .await
        .map_err(|e| ResumeError::Transport(anyhow::anyhow!(e)))?;
    log!("OK", "reconnected websocket");
    let (mut write, mut read) = ws_stream.split();

    // Writer
    let (writer_tx, mut writer_rx) = mpsc::unbounded_channel::<Message>();
    tokio::spawn(async move {
        while let Some(msg) = writer_rx.recv().await {
            if let Err(e) = write.send(msg).await {
                log!("ERR", "[writer] send error: {e}");
                break;
            }
        }
    });

    // Aux channels
    let (immediate_tx, immediate_rx) = mpsc::channel::<()>(8);
    let (last_seq_tx, last_seq_rx) = watch::channel::<Option<i64>>(last_seq);
    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    // HELLO
    let heartbeat_interval_ms = loop {
        let Some(frame) = read.next().await else {
            let _ = shutdown_tx.send(true);
            return Err(ResumeError::Transport(anyhow::anyhow!(
                "gateway closed before HELLO (resume)"
            )));
        };
        let Message::Text(text) = frame.map_err(|e| ResumeError::Transport(anyhow::anyhow!(e)))?
        else {
            continue;
        };
        let Ok(f) = serde_json::from_str::<FrameBorrowed>(&text) else {
            continue;
        };
        if f.op == 10 {
            let d =
                f.d.ok_or_else(|| ResumeError::Transport(anyhow::anyhow!("HELLO without d")))?;
            let hello = serde_json::from_str::<HelloD>(d.get())
                .map_err(|e| ResumeError::Transport(anyhow::anyhow!(e)))?;
            log!(
                "OK",
                "received HELLO (resume), heartbeat_interval={}ms",
                hello.heartbeat_interval
            );
            break hello.heartbeat_interval;
        }
    };

    // Heartbeat before RESUME
    tokio::spawn(crate::gateway::heartbeat::run_heartbeat(
        writer_tx.clone(),
        heartbeat_interval_ms,
        immediate_rx,
        last_seq_rx.clone(),
        shutdown_rx,
    ));

    // RESUME
    let resume_payload = serde_json::json!({
        "op": 6,
        "d": { "token": token, "session_id": session_id, "seq": last_seq }
    });
    writer_tx
        .send(Message::Text(resume_payload.to_string().into()))
        .map_err(|e| ResumeError::Transport(anyhow::anyhow!("failed to queue RESUME: {e}")))?;
    log!("GW", "payload op6 (resume) queued with seq={last_seq:?}");

    // Dispatch channel
    let (events_tx, events_rx) = mpsc::unbounded_channel::<GatewayDispatch<Box<RawValue>>>();

    // Wait until RESUMED
    loop {
        let Some(msg) = read.next().await else {
            let _ = shutdown_tx.send(true);
            return Err(ResumeError::Transport(anyhow::anyhow!(
                "gateway closed before RESUMED"
            )));
        };
        let Message::Text(text) = msg.map_err(|e| ResumeError::Transport(anyhow::anyhow!(e)))?
        else {
            continue;
        };
        let Ok(f) = serde_json::from_str::<FrameBorrowed>(&text) else {
            continue;
        };

        if let Some(s) = f.s {
            let _ = last_seq_tx.send_replace(Some(s));
        }

        match f.op {
            0 => {
                if let Some("RESUMED") = f.t {
                    log!("EVT", "RESUMED");
                    let ev = GatewayDispatch {
                        op: GatewayOpcodes::Dispatch,
                        t: GwEvt::Resumed,
                        s: f.s.unwrap_or(0),
                        d: Box::from(RawValue::from_string(f.d.unwrap().get().to_owned()).unwrap()),
                    };
                    let _ = events_tx.send(ev);
                    break;
                } else if let (Some(t), Some(d)) = (f.t, f.d) {
                    if let Some(evt) = map_event(t) {
                        let ev = GatewayDispatch {
                            op: GatewayOpcodes::Dispatch,
                            t: evt,
                            s: f.s.unwrap_or(0),
                            d: Box::from(RawValue::from_string(d.get().to_owned()).unwrap()),
                        };
                        let _ = events_tx.send(ev);
                    }
                }
            }
            1 => {
                let _ = immediate_tx.try_send(());
            }
            7 => {
                log!("WARN", "[gw] RECONNECT requested during RESUME");
            }
            9 => {
                let resumable =
                    f.d.and_then(|v| v.get().parse::<bool>().ok())
                        .unwrap_or(false);
                log!("WARN", "[gw] INVALID_SESSION during RESUME: {resumable}");
                let _ = shutdown_tx.send(true);
                return Err(ResumeError::InvalidSession { resumable });
            }
            11 => {}
            _ => {}
        }
    }

    // Background reader after RESUMED
    let events_tx_bg = events_tx.clone();
    let last_seq_tx_bg = last_seq_tx.clone();
    let immediate_tx_bg = immediate_tx.clone();
    tokio::spawn(async move {
        while let Some(msg) = read.next().await {
            let Ok(Message::Text(text)) = msg else {
                continue;
            };
            let Ok(f) = serde_json::from_str::<FrameBorrowed>(&text) else {
                continue;
            };

            if let Some(s) = f.s {
                let _ = last_seq_tx_bg.send_replace(Some(s));
            }

            match f.op {
                0 => {
                    if let (Some(t), Some(d)) = (f.t, f.d) {
                        if let Some(evt) = map_event(t) {
                            let ev = GatewayDispatch {
                                op: GatewayOpcodes::Dispatch,
                                t: evt,
                                s: f.s.unwrap_or(0),
                                d: Box::from(RawValue::from_string(d.get().to_owned()).unwrap()),
                            };
                            let _ = events_tx_bg.send(ev);
                        }
                    }
                }
                1 => {
                    let _ = immediate_tx_bg.try_send(());
                }
                7 => {
                    log!("WARN", "[gw] RECONNECT requested");
                }
                9 => {
                    log!("WARN", "[gw] INVALID_SESSION during running");
                }
                11 => {}
                _ => {}
            }
        }
        log!("WARN", "[reader] stream closed (resumed)");
    });

    Ok(Gateway {
        session_id: session_id.to_string(),
        resume_gateway_url: resume_gateway_url.to_string(),
        writer_tx,
        events_rx,
        last_seq_rx,
        shutdown_tx,
    })
}

fn normalize_gateway_url(base: &str) -> String {
    if base.contains('?') {
        base.to_string()
    } else if base.ends_with('/') {
        format!("{base}?v=10&encoding=json")
    } else {
        format!("{base}/?v=10&encoding=json")
    }
}
