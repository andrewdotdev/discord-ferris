pub mod heartbeat;
pub mod ws;

use tokio::sync::{mpsc, watch};
use tokio_tungstenite::tungstenite::Message;

/// Represents an active Discord Gateway session with wired channels.
pub struct Gateway {
    /// Session ID returned in READY.
    pub session_id: String,
    /// URL that should be used for RESUME attempts.
    pub resume_gateway_url: String,

    /// Single channel for **sending** frames to the Gateway (consumed by a writer task).
    pub writer_tx: mpsc::UnboundedSender<Message>,

    /// Stream of **dispatch events** (`op=0`, field `t`) already parsed as JSON.
    /// Exposed so higher layers can consume events without touching the raw socket.
    pub events_rx: mpsc::UnboundedReceiver<serde_json::Value>,

    /// Holds the last observed sequence number (`s`). Used for heartbeats and RESUME.
    pub last_seq_rx: watch::Receiver<Option<i64>>,

    /// Shutdown signal for background tasks (heartbeat, etc.).
    /// When set to `true`, tasks should stop gracefully.
    pub shutdown_tx: watch::Sender<bool>,
}

impl Drop for Gateway {
    fn drop(&mut self) {
        // Best-effort: tell background tasks to stop when this Gateway is dropped.
        let _ = self.shutdown_tx.send(true);
        // Dropping `writer_tx` also helps the writer task exit if it’s idle.
    }
}
