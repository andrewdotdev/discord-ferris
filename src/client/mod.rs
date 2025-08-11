use crate::gateway::{self, Gateway};
use crate::structs::gateway::GatewayIntents;
use crate::{log};

#[inline]
fn custom_prng(min: u64, max: u64) -> u64 {
    // lcg seeded from current time
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as u64;
    let mut state = nanos ^ 0x5DEECE66D;
    state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
    min + (state % (max - min + 1))
}


/// Represents the state of the Client (authenticated or not).
enum ClientState {
    Unauthenticated,
    Authenticated,
}

/// High-level Discord client: manages connection state and the Gateway lifecycle.
pub struct Client {
    token: String,
    intents: GatewayIntents,
    state: ClientState,
    gateway: Option<Gateway>,
}

impl Client {


    /// Creates the client (starts unauthenticated)
    pub fn new(token: impl Into<String>, intents: GatewayIntents) -> Self {
        Self {
            token: token.into(),
            intents,
            state: ClientState::Unauthenticated,
            gateway: None,
        }
    }

    /// Connects to Discord's Gateway, starts heartbeating, sends IDENTIFY,
    /// waits for READY, and then **keeps the client alive** until either:
    /// - The Gateway connection closes (then it will attempt RESUME or re-IDENTIFY).
    /// - The process receives Ctrl+C (SIGINT).
    ///
    /// This method will not return until the client shuts down.
    pub async fn login(&mut self) -> anyhow::Result<()> {
        // Initial connection (IDENTIFY).
        let gateway = gateway::ws::connect(&self.token, self.intents).await?;
        self.state = ClientState::Authenticated;
        log!("OK", "Client authenticated (session_id={})", gateway.session_id);
        self.gateway = Some(gateway);

        log!("CLI", "Client is running. use Ctrl+C to exit.");

        loop {
            // Access the current Gateway to receive events.
            let gw = self.gateway.as_mut().expect("gateway must exist");

            tokio::select! {
                // --- Incoming Gateway event ---
                maybe_evt = gw.events_rx.recv() => {
                    match maybe_evt {
                        Some(evt) => {
                            if let Some(t) = evt.get("t").and_then(|v| v.as_str()) {
                                log!("EVT", "{}", t);
                            }
                        }
                        _none => {
                            // The reader task ended: the connection is closed.
                            log!("WARN", "events_rx closed — attempting reconnection…");

                            // Take ownership of the old Gateway and clear the Option.
                            let old_gw = self.gateway.take().expect("gateway must exist");

                            // Preserve session details for a potential RESUME attempt.
                            let session_id = old_gw.session_id.clone();
                            let resume_url = old_gw.resume_gateway_url.clone();
                            let last_seq = old_gw.last_seq_rx.borrow().clone();

                            // Exponential backoff for repeated failures.
                            let mut delay = std::time::Duration::from_secs(1);

                            use crate::gateway::ws::ResumeError;

                            loop {
                                // 1) Try RESUME first.
                                match gateway::ws::resume(&self.token, &session_id, &resume_url, last_seq).await {
                                    Ok(new_gw) => {
                                        log!("OK", "RESUMED successfully");
                                        self.gateway = Some(new_gw);
                                        break; // Back to the main event loop.
                                    }
                                    Err(ResumeError::InvalidSession { resumable: false }) => {
                                        // Expected when the session cannot be resumed — do a fresh IDENTIFY.
                                        log!("WARN", "Session not resumable — performing fresh IDENTIFY…");

                                        // Recommended: small jitter before IDENTIFY.
                                        let jitter_ms: u64 = custom_prng(1000, 5000);
                                        tokio::time::sleep(std::time::Duration::from_millis(jitter_ms)).await;

                                        match gateway::ws::connect(&self.token, self.intents).await {
                                            Ok(new_gw) => {
                                                log!("OK", "Re-IDENTIFY successful (fresh session_id={})", new_gw.session_id);
                                                self.gateway = Some(new_gw);
                                                break;
                                            }
                                            Err(e2) => {
                                                log!("ERR", "Re-connect (IDENTIFY) failed: {e2}. Retrying in {:?}…", delay);
                                                tokio::time::sleep(delay).await;
                                                delay = std::cmp::min(delay * 2, std::time::Duration::from_secs(60));
                                            }
                                        }
                                    }
                                    Err(ResumeError::InvalidSession { resumable: true }) => {
                                        // Discord says you can retry shortly.
                                        log!("WARN", "Session temporarily invalid; retrying RESUME…");
                                        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                                    }
                                    Err(ResumeError::Transport(err)) => {
                                        log!("WARN", "RESUME transport error: {err}. Retrying in {:?}…", delay);
                                        tokio::time::sleep(delay).await;
                                        delay = std::cmp::min(delay * 2, std::time::Duration::from_secs(60));
                                    }
                                }
                            }
                        }
                    }
                }

                // --- Shutdown signal (Ctrl+C) ---
                _ = tokio::signal::ctrl_c() => {
                    log!("CLI", "Keyboard Interrupt: Exiting");
                    if let Some(gw) = self.gateway.as_ref() {
                        use tokio_tungstenite::tungstenite::protocol::{CloseFrame, frame::coding::CloseCode};
                        use tokio_tungstenite::tungstenite::Message;
                        let _ = gw.writer_tx.send(Message::Close(Some(CloseFrame {
                            code: CloseCode::Normal,
                            reason: "shutdown".into(),
                        })));
                    }
                    break;
                }
            }
        }

        Ok(())
    }
}
