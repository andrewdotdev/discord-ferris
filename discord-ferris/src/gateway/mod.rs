pub mod heartbeat;
pub mod ws;

use serde::Deserialize;
use serde_json::value::RawValue;
use tokio::sync::{mpsc, watch};
use tokio_tungstenite::tungstenite::Message;

use crate::models::gateway::GatewayDispatchEvents;

/// Active Gateway session wires.
pub struct Gateway {
    pub session_id: String,
    pub resume_gateway_url: String,
    pub writer_tx: mpsc::UnboundedSender<Message>,
    /// Dispatches with raw `d` (no eager JSON parse).
    pub events_rx: mpsc::UnboundedReceiver<crate::models::gateway::GatewayDispatch<Box<RawValue>>>,
    pub last_seq_rx: watch::Receiver<Option<i64>>,
    pub shutdown_tx: watch::Sender<bool>,
}

impl Drop for Gateway {
    fn drop(&mut self) {
        let _ = self.shutdown_tx.send(true);
    }
}

pub trait EventPayload: for<'de> Deserialize<'de> + Send + 'static {
    const EVENT: GatewayDispatchEvents;
}

impl EventPayload for crate::models::gateway::GatewayMessageCreateDispatchData {
    const EVENT: GatewayDispatchEvents =
        crate::models::gateway::GatewayDispatchEvents::MessageCreate;
}
