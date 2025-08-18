use std::fmt;
use std::sync::Arc;

use crate::http::Http;
use crate::models::gateway::{
    GatewayDispatch, GatewayDispatchEvents as GwEvt, GatewayReadyDispatchData,
};
use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde_json::{Value, value::RawValue};

#[derive(Clone)]
pub struct Context {
    pub http: Arc<Http>,
}
impl Context {
    #[inline]
    pub fn new(http: Arc<Http>) -> Self {
        Self { http }
    }
    #[inline]
    pub fn http(&self) -> &Arc<Http> {
        &self.http
    }
}
impl fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Context")
            .field("http", &"Http { redacted }")
            .finish()
    }
}

#[derive(Clone, Debug)]
pub struct Ctx {
    pub(crate) inner: Arc<Context>,
    pub(crate) event: Option<Arc<GatewayDispatch<Box<RawValue>>>>,
}
impl Ctx {
    #[inline]
    pub fn new(inner: Arc<Context>) -> Self {
        Self { inner, event: None }
    }
    #[inline]
    pub(crate) fn with_event(inner: Arc<Context>, ev: Arc<GatewayDispatch<Box<RawValue>>>) -> Self {
        Self {
            inner,
            event: Some(ev),
        }
    }

    #[inline]
    pub fn event_name(&self) -> Option<GwEvt> {
        self.event.as_ref().map(|ev| ev.t.clone())
    }
    #[inline]
    pub fn is(&self, event: GwEvt) -> bool {
        matches!(self.event_name(), Some(t) if t == event)
    }

    #[inline]
    pub fn data_as<T: DeserializeOwned>(&self) -> Option<T> {
        let ev = self.event.as_ref()?;
        match serde_json::from_str::<T>(ev.d.get()) {
            Ok(v) => Some(v),
            Err(e) => {
                crate::log!("WARN", "Failed to decode payload for {:?}: {}", ev.t, e);
                if let Ok(v) = serde_json::from_str::<Value>(ev.d.get()) {
                    debug_scan_flags("d", &v);
                }
                None
            }
        }
    }

    #[inline]
    pub fn event_as<T: DeserializeOwned>(&self, event: GwEvt) -> Option<T> {
        if self.is(event) {
            self.data_as::<T>()
        } else {
            None
        }
    }
}

/* message helpers */
#[derive(Debug, Clone, Deserialize)]
struct MinimalMessage {
    pub id: String,
    pub channel_id: String,
    #[serde(default)]
    pub content: Option<String>,
}

impl Ctx {
    #[inline]
    pub fn text(&self) -> Option<String> {
        self.event_as::<MinimalMessage>(GwEvt::MessageCreate)
            .and_then(|m| m.content)
    }
    #[inline]
    pub fn channel_id(&self) -> Option<String> {
        self.event_as::<MinimalMessage>(GwEvt::MessageCreate)
            .map(|m| m.channel_id)
    }
    #[inline]
    pub async fn reply(&self, content: &str) -> anyhow::Result<()> {
        let m = self
            .event_as::<MinimalMessage>(GwEvt::MessageCreate)
            .ok_or_else(|| anyhow::anyhow!("not a MessageCreate dispatch"))?;
        self.inner
            .http
            .send_message(&m.channel_id, content, Some(&m.id), false)
            .await
    }
    #[inline]
    pub async fn say(&self, channel_id: &str, content: &str) -> anyhow::Result<()> {
        self.inner
            .http
            .send_message(channel_id, content, None, false)
            .await
    }
    #[inline]
    pub fn ready(&self) -> Option<GatewayReadyDispatchData> {
        self.event_as::<GatewayReadyDispatchData>(GwEvt::Ready)
    }
}

fn debug_scan_flags(prefix: &str, v: &Value) {
    match v {
        Value::Object(map) => {
            for (k, val) in map {
                let path = if prefix.is_empty() {
                    k.to_string()
                } else {
                    format!("{}/{}", prefix, k)
                };
                let looks_like_flags = k == "flags" || k.ends_with("_flags") || k == "public_flags";
                if looks_like_flags {
                    let is_num = val.is_u64() || val.is_i64();
                    if !is_num && !val.is_null() {
                        crate::log!("WARN", "Field '{}' is not numeric: {:?}", path, val);
                    }
                }
                debug_scan_flags(&path, val);
            }
        }
        Value::Array(arr) => {
            for (i, val) in arr.iter().enumerate() {
                debug_scan_flags(&format!("{}/{}", prefix, i), val);
            }
        }
        _ => {}
    }
}
