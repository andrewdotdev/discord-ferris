use crate::log;
use tokio::select;
use tokio::sync::{mpsc, watch};
use tokio::time::{Duration, interval};
use tokio_tungstenite::tungstenite::Message;

/// Heartbeat loop (sends op=1 with latest seq).
pub async fn run_heartbeat(
    writer_tx: mpsc::UnboundedSender<Message>,
    interval_ms: u64,
    mut immediate_rx: mpsc::Receiver<()>,
    last_seq_rx: watch::Receiver<Option<i64>>,
    mut shutdown_rx: watch::Receiver<bool>,
) {
    let mut ticker = interval(Duration::from_millis(interval_ms));
    loop {
        select! {
            _ = ticker.tick() => {
                if let Err(e) = send_heartbeat(&writer_tx, last_seq_rx.borrow().clone()) {
                    log!("HB", "send error: {e}");
                }
            }
            m = immediate_rx.recv() => {
                if m.is_some() {
                    if let Err(e) = send_heartbeat(&writer_tx, last_seq_rx.borrow().clone()) {
                        log!("HB", "send error: {e}");
                    }
                }
            }
            _ = shutdown_rx.changed() => {
                if *shutdown_rx.borrow() { break; }
            }
        }
    }
}

pub fn send_heartbeat(
    writer_tx: &mpsc::UnboundedSender<Message>,
    seq: Option<i64>,
) -> anyhow::Result<()> {
    let heartbeat = serde_json::json!({ "op": 1, "d": seq });
    writer_tx.send(Message::Text(heartbeat.to_string().into()))?;
    Ok(())
}
