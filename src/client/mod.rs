use crate::gateway::{self, Gateway};
use crate::structs::gateway::GatewayIntents;
use crate::{log_cli, log_err, log_ok, log_warn};

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
        log_ok!("Client authenticated (session_id={})", gateway.session_id);
        self.gateway = Some(gateway);

        log_cli!("Client is now running. Press Ctrl+C to exit.");

        loop {
            // Access the current Gateway to receive events.
            let gw = self.gateway.as_mut().expect("gateway must exist");

            tokio::select! {
                // --- Incoming Gateway event ---
                maybe_evt = gw.events_rx.recv() => {
                    match maybe_evt {
                        Some(evt) => {
                            if let Some(t) = evt.get("t").and_then(|v| v.as_str()) {
                                crate::log_evt!("{}", t);
                            }
                        }
                        _none => {
                            // The reader task ended: the connection is closed.
                            log_warn!("events_rx closed — attempting reconnection…");

                            // Take ownership of the old Gateway and clear the Option.
                            let old_gw = self.gateway.take().expect("gateway must exist");

                            // Preserve session details for a potential RESUME attempt.
                            let session_id = old_gw.session_id.clone();
                            let resume_url = old_gw.resume_gateway_url.clone();
                            let last_seq = old_gw.last_seq_rx.borrow().clone();

                            // Exponential backoff for repeated failures.
                            let mut delay = std::time::Duration::from_secs(1);

                            use crate::gateway::ws::ResumeError;
                            use rand::{rng, Rng};

                            loop {
                                // 1) Try RESUME first.
                                match gateway::ws::resume(&self.token, &session_id, &resume_url, last_seq).await {
                                    Ok(new_gw) => {
                                        log_ok!("RESUMED successfully");
                                        self.gateway = Some(new_gw);
                                        break; // Back to the main event loop.
                                    }
                                    Err(ResumeError::InvalidSession { resumable: false }) => {
                                        // Expected when the session cannot be resumed — do a fresh IDENTIFY.
                                        log_warn!("Session not resumable — performing fresh IDENTIFY…");

                                        // Recommended: small jitter before IDENTIFY.
                                        let jitter_ms: u64 = rng().random_range(1000..=5000);
                                        tokio::time::sleep(std::time::Duration::from_millis(jitter_ms)).await;

                                        match gateway::ws::connect(&self.token, self.intents).await {
                                            Ok(new_gw) => {
                                                log_ok!("Re-IDENTIFY successful (fresh session_id={})", new_gw.session_id);
                                                self.gateway = Some(new_gw);
                                                break;
                                            }
                                            Err(e2) => {
                                                log_err!("Re-connect (IDENTIFY) failed: {e2}. Retrying in {:?}…", delay);
                                                tokio::time::sleep(delay).await;
                                                delay = std::cmp::min(delay * 2, std::time::Duration::from_secs(60));
                                            }
                                        }
                                    }
                                    Err(ResumeError::InvalidSession { resumable: true }) => {
                                        // Discord says you can retry shortly.
                                        log_warn!("Session temporarily invalid; retrying RESUME…");
                                        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                                    }
                                    Err(ResumeError::Transport(err)) => {
                                        log_warn!("RESUME transport error: {err}. Retrying in {:?}…", delay);
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
                    log_cli!("Ctrl+C received — shutting down.");
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
