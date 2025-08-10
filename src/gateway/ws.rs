use futures_util::{SinkExt, StreamExt};
use rustls::{ClientConfig, RootCertStore};
use std::sync::Arc;
use tokio::sync::{mpsc, watch};
use tokio_tungstenite::{connect_async_tls_with_config, Connector, tungstenite::protocol::Message};

use crate::gateway::Gateway;
use crate::structs::gateway::GatewayIntents;
use crate::{log_err, log_evt, log_gw, log_hb, log_ok, log_warn};

/// Discord Gateway URL with API version and JSON encoding.
const DISCORD_GATEWAY_URL: &str = "wss://gateway.discord.gg/?v=10&encoding=json";

/// A typed error for RESUME attempts, allowing clearer caller behavior.
#[derive(thiserror::Error, Debug)]
pub enum ResumeError {
    /// The session cannot be resumed. If `resumable` is `true`, retry soon; if `false`, do a fresh IDENTIFY.
    #[error("invalid session on resume (resumable={resumable})")]
    InvalidSession { resumable: bool },

    /// Transport-level failure while attempting to resume.
    #[error(transparent)]
    Transport(#[from] anyhow::Error),
}

/// Connects to the Discord Gateway following the correct flow:
/// 1) Receive HELLO (op=10) and extract `heartbeat_interval`.
/// 2) Start the heartbeat loop immediately.
/// 3) Send IDENTIFY.
/// 4) Read synchronously until READY, then spawn a background reader task.
///
/// Returns a `Gateway` with channels for writing frames and receiving dispatch events.
pub async fn connect(token: &str, intents: GatewayIntents) -> anyhow::Result<Gateway> {
    log_gw!("connecting to discord gateway");

    // --- TLS (rustls + webpki roots) ---
    let provider = rustls::crypto::ring::default_provider().into();
    let mut root_store = RootCertStore::empty();
    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    let config = ClientConfig::builder_with_provider(provider)
        .with_safe_default_protocol_versions()?
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let connector = Connector::Rustls(Arc::new(config));

    // --- Open WebSocket ---
    let (ws_stream, _) =
        connect_async_tls_with_config(DISCORD_GATEWAY_URL, None, true, Some(connector)).await?;
    log_ok!("connection established");

    let (mut write, mut read) = ws_stream.split();

    // --- Dedicated writer task (single writer) ---
    let (writer_tx, mut writer_rx) = mpsc::unbounded_channel::<Message>();
    tokio::spawn(async move {
        while let Some(msg) = writer_rx.recv().await {
            if let Err(e) = write.send(msg).await {
                log_err!("[writer] send error: {e}");
                break;
            }
        }
        // Writer task exits -> socket will eventually close on read side too.
    });

    // --- Helper channels ---
    let (immediate_tx, immediate_rx) = mpsc::channel::<()>(8);
    let (last_seq_tx, last_seq_rx) = watch::channel::<Option<i64>>(None);
    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    // --- 1) Wait for HELLO to get heartbeat_interval ---
    let heartbeat_interval_ms = loop {
        let Some(frame) = read.next().await else {
            let _ = shutdown_tx.send(true);
            anyhow::bail!("gateway closed before HELLO");
        };

        let Message::Text(text) = frame? else {
            continue; // Ignore binary frames while compression is disabled.
        };

        let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) else { continue; };

        if json.get("op").and_then(|v| v.as_i64()) == Some(10) {
            let interval = json["d"]["heartbeat_interval"]
                .as_u64()
                .ok_or_else(|| anyhow::anyhow!("HELLO without heartbeat_interval"))?;
            log_ok!("received HELLO, heartbeat_interval={interval}ms");
            break interval;
        }
    };

    // --- 2) Start heartbeat task ---
    tokio::spawn(crate::gateway::heartbeat::run_heartbeat(
        writer_tx.clone(),
        heartbeat_interval_ms,
        immediate_rx,
        last_seq_rx.clone(),
        shutdown_rx,
    ));
    log_hb!("heartbeat task started");

    // --- 3) Send IDENTIFY ---
    let identify = serde_json::json!({
        "op": 2,
        "d": {
            "token": token,
            "intents": intents.bits(),
            "properties": { "os": "linux", "browser": "discord-ferris", "device": "discord-ferris" }
        }
    });
    writer_tx
        .send(Message::Text(identify.to_string().into()))
        .map_err(|e| anyhow::anyhow!("failed to queue IDENTIFY: {e}"))?;
    log_gw!("payload op2 (identify) queued");

    // --- Event channel for higher layers ---
    let (events_tx, events_rx) = mpsc::unbounded_channel::<serde_json::Value>();

    // --- 4) Read synchronously until READY (handle opcodes along the way) ---
    let mut session_id: Option<String> = None;
    let mut resume_gateway_url: Option<String> = None;

    loop {
        let Some(msg) = read.next().await else {
            let _ = shutdown_tx.send(true);
            anyhow::bail!("gateway closed before READY");
        };

        let Message::Text(text) = msg? else { continue; };
        let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) else { continue; };

        // Track the last sequence number (for heartbeats and RESUME).
        if let Some(s) = json.get("s").and_then(|v| v.as_i64()) {
            let _ = last_seq_tx.send_replace(Some(s));
        }

        match json.get("op").and_then(|v| v.as_i64()) {
            Some(0) => {
                // DISPATCH
                if let Some(t) = json.get("t").and_then(|v| v.as_str()) {
                    if t == "READY" {
                        if let Some(d) = json.get("d") {
                            if let Some(sid) = d.get("session_id").and_then(|v| v.as_str()) {
                                session_id = Some(sid.to_string());
                            }
                            if let Some(url) = d.get("resume_gateway_url").and_then(|v| v.as_str()) {
                                resume_gateway_url = Some(url.to_string());
                            }
                        }
                        log_evt!("READY");
                        let _ = events_tx.send(json);
                        break;
                    } else {
                        log_evt!("{}", t);
                        let _ = events_tx.send(json);
                    }
                }
            }
            Some(1) => {
                // HEARTBEAT request (send immediately).
                let _ = immediate_tx.try_send(());
            }
            Some(7) => {
                log_warn!("[gw] RECONNECT requested (will be handled by client if the connection drops)");
            }
            Some(9) => {
                log_warn!("[gw] INVALID_SESSION: {}", json.get("d").unwrap_or(&serde_json::Value::Null));
            }
            Some(11) => { /* HEARTBEAT_ACK — could be used to track latency. */ }
            _ => {}
        }
    }

    let session_id = session_id.unwrap_or_default();
    let resume_gateway_url = resume_gateway_url.unwrap_or_else(|| DISCORD_GATEWAY_URL.to_string());

    // --- Background reader task for post-READY traffic ---
    let events_tx_bg = events_tx.clone();
    let last_seq_tx_bg = last_seq_tx.clone();
    let immediate_tx_bg = immediate_tx.clone();
    tokio::spawn(async move {
        while let Some(msg) = read.next().await {
            let Ok(Message::Text(text)) = msg else { continue; };
            let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) else { continue; };

            if let Some(s) = json.get("s").and_then(|v| v.as_i64()) {
                let _ = last_seq_tx_bg.send_replace(Some(s));
            }

            match json.get("op").and_then(|v| v.as_i64()) {
                Some(0) => { let _ = events_tx_bg.send(json); }
                Some(1) => { let _ = immediate_tx_bg.try_send(()); }
                Some(7) => { log_warn!("[gw] RECONNECT requested"); }
                Some(9) => { log_warn!("[gw] INVALID_SESSION: {}", json.get("d").unwrap_or(&serde_json::Value::Null)); }
                Some(11) => { /* HEARTBEAT_ACK */ }
                _ => {}
            }
        }
        log_warn!("[reader] stream closed");
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

/// Attempts to resume an existing Gateway session using op=6 RESUME.
/// If the session cannot be resumed (`INVALID_SESSION: false`), this returns `ResumeError::InvalidSession`.
/// The caller should then fall back to a fresh IDENTIFY using `connect(...)`.
pub async fn resume(
    token: &str,
    session_id: &str,
    resume_gateway_url: &str,
    last_seq: Option<i64>,
) -> Result<Gateway, ResumeError> {
    log_gw!("resuming session_id={} at {}", session_id, resume_gateway_url);

    // --- TLS (rustls + webpki roots) ---
    let provider = rustls::crypto::ring::default_provider().into();
    let mut root_store = RootCertStore::empty();
    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    let config = ClientConfig::builder_with_provider(provider)
        .with_safe_default_protocol_versions()
        .map_err(|e| ResumeError::Transport(anyhow::anyhow!(e)))?
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let connector = Connector::Rustls(Arc::new(config));

    // --- Open WebSocket ---
    let resume_url = normalize_gateway_url(resume_gateway_url);
    let (ws_stream, _) =
        connect_async_tls_with_config(resume_url, None, true, Some(connector))
            .await
            .map_err(|e| ResumeError::Transport(anyhow::anyhow!(e)))?;
    log_ok!("reconnected websocket");

    let (mut write, mut read) = ws_stream.split();

    // --- Dedicated writer task (single writer) ---
    let (writer_tx, mut writer_rx) = mpsc::unbounded_channel::<Message>();
    tokio::spawn(async move {
        while let Some(msg) = writer_rx.recv().await {
            if let Err(e) = write.send(msg).await {
                log_err!("[writer] send error: {e}");
                break;
            }
        }
    });

    // --- Helper channels ---
    let (immediate_tx, immediate_rx) = mpsc::channel::<()>(8);
    let (last_seq_tx, last_seq_rx) = watch::channel::<Option<i64>>(last_seq);
    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    // --- 1) Wait for HELLO to get heartbeat_interval ---
    let heartbeat_interval_ms = loop {
        let Some(frame) = read.next().await else {
            let _ = shutdown_tx.send(true);
            return Err(ResumeError::Transport(anyhow::anyhow!("gateway closed before HELLO (resume)")));
        };

        let Message::Text(text) = frame.map_err(|e| ResumeError::Transport(anyhow::anyhow!(e)))? else {
            continue;
        };

        let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) else { continue; };

        if json.get("op").and_then(|v| v.as_i64()) == Some(10) {
            let Some(interval) = json["d"]["heartbeat_interval"].as_u64() else {
                return Err(ResumeError::Transport(anyhow::anyhow!("HELLO without heartbeat_interval")));
            };
            log_ok!("received HELLO (resume), heartbeat_interval={interval}ms");
            break interval;
        }
    };

    // --- 2) Start heartbeat task BEFORE RESUME ---
    tokio::spawn(crate::gateway::heartbeat::run_heartbeat(
        writer_tx.clone(),
        heartbeat_interval_ms,
        immediate_rx,
        last_seq_rx.clone(),
        shutdown_rx,
    ));
    log_hb!("heartbeat task started (resume)");

    // --- 3) Send RESUME ---
    let resume_payload = serde_json::json!({
        "op": 6,
        "d": { "token": token, "session_id": session_id, "seq": last_seq }
    });
    writer_tx
        .send(Message::Text(resume_payload.to_string().into()))
        .map_err(|e| ResumeError::Transport(anyhow::anyhow!("failed to queue RESUME: {e}")))?;
    log_gw!("payload op6 (resume) queued with seq={last_seq:?}");

    // --- Event channel for higher layers ---
    let (events_tx, events_rx) = mpsc::unbounded_channel::<serde_json::Value>();

    // --- 4) Read synchronously until RESUMED (handle failures along the way) ---
    loop {
        let Some(msg) = read.next().await else {
            let _ = shutdown_tx.send(true);
            return Err(ResumeError::Transport(anyhow::anyhow!("gateway closed before RESUMED")));
        };
        let Message::Text(text) = msg.map_err(|e| ResumeError::Transport(anyhow::anyhow!(e)))? else { continue; };
        let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) else { continue; };

        if let Some(s) = json.get("s").and_then(|v| v.as_i64()) {
            let _ = last_seq_tx.send_replace(Some(s));
        }

        match json.get("op").and_then(|v| v.as_i64()) {
            Some(0) => {
                if let Some(t) = json.get("t").and_then(|v| v.as_str()) {
                    if t == "RESUMED" {
                        log_evt!("RESUMED");
                        let _ = events_tx.send(json);
                        break;
                    } else {
                        log_evt!("{}", t);
                        let _ = events_tx.send(json);
                    }
                }
            }
            Some(1) => { let _ = immediate_tx.try_send(()); } // HEARTBEAT request
            Some(7) => { log_warn!("[gw] RECONNECT requested during RESUME"); }
            Some(9) => {
                let resumable = json.get("d").and_then(|v| v.as_bool()).unwrap_or(false);
                log_warn!("[gw] INVALID_SESSION during RESUME: {resumable}");
                let _ = shutdown_tx.send(true);
                return Err(ResumeError::InvalidSession { resumable });
            }
            Some(11) => { /* HEARTBEAT_ACK */ }
            _ => {}
        }
    }

    // --- Background reader task for post-RESUMED traffic ---
    let events_tx_bg = events_tx.clone();
    let last_seq_tx_bg = last_seq_tx.clone();
    let immediate_tx_bg = immediate_tx.clone();
    tokio::spawn(async move {
        while let Some(msg) = read.next().await {
            let Ok(Message::Text(text)) = msg else { continue; };
            let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) else { continue; };

            if let Some(s) = json.get("s").and_then(|v| v.as_i64()) {
                let _ = last_seq_tx_bg.send_replace(Some(s));
            }

            match json.get("op").and_then(|v| v.as_i64()) {
                Some(0) => { let _ = events_tx_bg.send(json); }
                Some(1) => { let _ = immediate_tx_bg.try_send(()); }
                Some(7) => { log_warn!("[gw] RECONNECT requested"); }
                Some(9) => { log_warn!("[gw] INVALID_SESSION during running"); }
                Some(11) => { /* HEARTBEAT_ACK */ }
                _ => {}
            }
        }
        log_warn!("[reader] stream closed (resumed)");
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
