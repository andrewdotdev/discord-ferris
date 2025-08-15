use serde::Deserialize;
use serde_json::Value;

use crate::models::gateway::{GatewayDispatch, GatewayDispatchEvents as GwEvt, GatewayOpcodes};

#[derive(Debug, Clone, Deserialize)]
pub struct DispatchEnvelope {
    pub op: i64,
    pub s: Option<i64>,
    pub t: Option<String>,
    pub d: Value,
}

/// Decode only Gateway *dispatch* frames (op == 0).
/// Returns `None` for non-dispatch frames or unknown event names.
pub fn decode(envelope: DispatchEnvelope) -> Option<GatewayDispatch<Value>> {
    // Only dispatch
    if envelope.op != 0 {
        return None;
    }
    let t_str = envelope.t.as_ref()?; // must exist on dispatch

    // Map the SCREAMING_SNAKE_CASE string to the enum using serde.
    // If it doesn't map to a known variant, treat as unknown and skip.
    let evt: GwEvt = serde_json::from_value(Value::String(t_str.clone())).ok()?;

    Some(GatewayDispatch {
        op: GatewayOpcodes::Dispatch,
        t: evt,
        s: envelope.s.unwrap_or(0) as i64,
        d: envelope.d,
    })
}
