use tokio::sync::{mpsc, watch};
use tokio::time::{interval, Duration};
use tokio::select;
use tokio_tungstenite::tungstenite::Message;
use crate::log_hb;

/// Runs the Discord heartbeat loop.
///
/// Responsibilities:
/// - Sends heartbeats at a fixed `interval_ms` (from the Gateway HELLO payload).
/// - Sends an *immediate* heartbeat if `immediate_rx` receives a signal (op=1 request from Gateway).
/// - Always includes the latest known sequence number from `last_seq_rx` in the heartbeat payload.
/// - Listens for a shutdown signal (`shutdown_rx`) and stops gracefully when triggered.
///
/// This loop should be spawned in its own task after receiving HELLO, **before** sending IDENTIFY or RESUME.
/// The Gateway expects heartbeats to start immediately after HELLO, and to continue at the exact interval.
pub async fn run_heartbeat(
    writer_tx: mpsc::UnboundedSender<Message>,
    interval_ms: u64,
    mut immediate_rx: mpsc::Receiver<()>,
    last_seq_rx: watch::Receiver<Option<i64>>,
    mut shutdown_rx: watch::Receiver<bool>,
) {
    // A Tokio interval timer that ticks every `interval_ms`.
    let mut ticker = interval(Duration::from_millis(interval_ms));

    loop {
        select! {
            // --- Regular scheduled heartbeat ---
            _ = ticker.tick() => {
                let result = send_heartbeat(&writer_tx, last_seq_rx.borrow().clone());
                log_hb!("{:?}", result);
            }

            // --- Immediate heartbeat (Gateway op=1 request) ---
            maybe_msg = immediate_rx.recv() => {
                if maybe_msg.is_some() {
                    let result = send_heartbeat(&writer_tx, last_seq_rx.borrow().clone());
                    log_hb!("{:?}", result);
                }
            }

            // --- Shutdown signal ---
            _ = shutdown_rx.changed() => {
                if *shutdown_rx.borrow() {
                    // Gracefully exit the heartbeat loop.
                    log_hb!("Shutdown signal received — stopping heartbeat task.");
                    break;
                }
            }
        }
    }
}

/// Sends a single heartbeat frame (op=1) to the Gateway.
///
/// Arguments:
/// - `writer_tx`: The sending channel for Gateway messages (owned by the writer task).
/// - `seq`: The most recent sequence number (`s`) observed from Gateway dispatches.
///
/// Notes:
/// - The payload format is: `{ "op": 1, "d": <seq_or_null> }`.
/// - Discord requires this `d` value to be the last sequence received, or `null` if none has been received yet.
/// - This function is synchronous because `mpsc::UnboundedSender::send` is non-blocking.
pub fn send_heartbeat(
    writer_tx: &mpsc::UnboundedSender<Message>,
    seq: Option<i64>,
) -> anyhow::Result<()> {
    let heartbeat = serde_json::json!({ "op": 1, "d": seq });
    writer_tx.send(Message::Text(heartbeat.to_string().into()))?;
    Ok(())
}
