use serde_json::value::RawValue;

use crate::{
    models::gateway::GatewayDispatchEvents as E,
    prelude::{Ctx, GatewayDispatch},
};

pub mod registry {
    use crate::prelude::Ctx;

    use super::*;
    use serde_json as json;
    use std::{future::Future, pin::Pin};

    pub type DynFut = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

    pub type HandlerFn = fn(Ctx, json::Value) -> DynFut;

    pub struct Handler {
        pub event: E,
        pub f: HandlerFn,
        pub name: &'static str,
    }

    inventory::collect!(Handler);

    #[inline]
    pub fn iter_for(event: E) -> impl Iterator<Item = &'static Handler> {
        inventory::iter::<Handler>
            .into_iter()
            .filter(move |h| h.event == event)
    }
}

pub async fn dispatch(ctx: Ctx, ev: GatewayDispatch<serde_json::Value>) {
    let which = ev.t;
    let raw = ev.d;
    for h in registry::iter_for(which) {
        (h.f)(ctx.clone(), raw.clone()).await;
    }
}

pub async fn dispatch_inventory_raw(ctx: Ctx, kind: E, raw: &RawValue) -> usize {
    let parsed: serde_json::Value = match serde_json::from_str(raw.get()) {
        Ok(v) => v,
        Err(e) => {
            crate::log!("WARN", "failed to parse RawValue for {:?}: {}", kind, e);
            return 0;
        }
    };

    let mut n = 0usize;
    for h in registry::iter_for(kind) {
        (h.f)(ctx.clone(), parsed.clone()).await;
        n += 1;
    }
    n
}
