use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde_json::value::RawValue;
use std::collections::HashMap;
use std::future::Future;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use crate::framework::context::{Context, Ctx};
use crate::models::gateway::{GatewayDispatch, GatewayDispatchEvents as GwEvt};

#[async_trait]
pub trait DynHandler: Send + Sync {
    // Keep raw `d` and decode lazily.
    async fn call(&self, ctx: Arc<Context>, ev: Arc<GatewayDispatch<Box<RawValue>>>);
    fn event(&self) -> GwEvt;
    fn is_once(&self) -> bool {
        false
    }
}

struct Handler<T, F> {
    event: GwEvt,
    f: F,
    once: bool,
    _phantom: PhantomData<fn() -> T>,
}

#[async_trait]
impl<T, F, Fut> DynHandler for Handler<T, F>
where
    T: DeserializeOwned + Send + 'static,
    F: Send + Sync + 'static + Fn(Ctx, T) -> Fut,
    Fut: Future<Output = ()> + Send + 'static,
{
    async fn call(&self, ctx: Arc<Context>, ev: Arc<GatewayDispatch<Box<RawValue>>>) {
        if ev.t != self.event {
            return;
        }
        match serde_json::from_str::<T>(ev.d.get()) {
            Ok(payload) => {
                let c = Ctx::with_event(Arc::clone(&ctx), Arc::clone(&ev));
                (self.f)(c, payload).await;
            }
            Err(err) => {
                crate::log!("WARN", "decode failed for {:?}: {}", self.event, err);
            }
        }
    }

    fn event(&self) -> GwEvt {
        self.event.clone()
    }
    fn is_once(&self) -> bool {
        self.once
    }
}

#[async_trait]
pub trait DynAnyHandler: Send + Sync {
    async fn call(&self, ctx: Arc<Context>, ev: Arc<GatewayDispatch<Box<RawValue>>>);
}

#[async_trait]
impl<F, Fut> DynAnyHandler for F
where
    F: Send + Sync + 'static + Fn(Ctx) -> Fut,
    Fut: Future<Output = ()> + Send + 'static,
{
    async fn call(&self, ctx: Arc<Context>, ev: Arc<GatewayDispatch<Box<RawValue>>>) {
        let c = Ctx::with_event(Arc::clone(&ctx), Arc::clone(&ev));
        (self)(c).await;
    }
}

pub struct Router {
    routes: HashMap<GwEvt, Vec<Arc<dyn DynHandler>>>,
    once_routes: Mutex<HashMap<GwEvt, Vec<Arc<dyn DynHandler>>>>,
    any: Vec<Arc<dyn DynAnyHandler>>,
    unknown: Vec<Arc<dyn DynAnyHandler>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
            once_routes: Mutex::new(HashMap::new()),
            any: vec![],
            unknown: vec![],
        }
    }

    pub fn on<T, F, Fut>(&mut self, kind: GwEvt, handler: F)
    where
        T: DeserializeOwned + Send + 'static,
        F: Send + Sync + 'static + Fn(Ctx, T) -> Fut,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let h: Arc<dyn DynHandler> = Arc::new(Handler::<T, F> {
            event: kind.clone(),
            f: handler,
            once: false,
            _phantom: PhantomData,
        });
        self.routes.entry(kind).or_default().push(h);
    }

    pub fn once<T, F, Fut>(&mut self, kind: GwEvt, handler: F)
    where
        T: DeserializeOwned + Send + 'static,
        F: Send + Sync + 'static + Fn(Ctx, T) -> Fut,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let h: Arc<dyn DynHandler> = Arc::new(Handler::<T, F> {
            event: kind.clone(),
            f: handler,
            once: true,
            _phantom: PhantomData,
        });
        let mut map = self.once_routes.lock().unwrap();
        map.entry(kind).or_default().push(h);
    }

    pub fn on_all<F, Fut>(&mut self, handler: F)
    where
        F: Send + Sync + 'static + Fn(Ctx) -> Fut,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.any.push(Arc::new(handler));
    }

    pub fn on_unknown<F, Fut>(&mut self, handler: F)
    where
        F: Send + Sync + 'static + Fn(Ctx) -> Fut,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.unknown.push(Arc::new(handler));
    }

    pub async fn dispatch(&self, base: &Ctx, ev: Arc<GatewayDispatch<Box<RawValue>>>) {
        let ctx_arc = Arc::clone(&base.inner);

        // on_all
        for h in &self.any {
            h.call(Arc::clone(&ctx_arc), Arc::clone(&ev)).await;
        }

        let kind = ev.t.clone();

        // --- once handlers
        let mut had_handlers = false;
        if let Some(list) = self.once_routes.lock().unwrap().remove(&kind) {
            had_handlers |= !list.is_empty();
            for h in list {
                h.call(Arc::clone(&ctx_arc), Arc::clone(&ev)).await;
            }
        }

        // --- persistent handlers
        if let Some(list) = self.routes.get(&kind) {
            had_handlers |= !list.is_empty();
            for h in list {
                h.call(Arc::clone(&ctx_arc), Arc::clone(&ev)).await;
            }
        }

        use crate::framework::events;
        let ran_macros = events::dispatch_inventory_raw(
            Ctx::with_event(Arc::clone(&ctx_arc), Arc::clone(&ev)),
            kind.clone(),
            &ev.d,
        )
        .await;
        had_handlers |= ran_macros > 0;

        if !had_handlers {
            for h in &self.unknown {
                h.call(Arc::clone(&ctx_arc), Arc::clone(&ev)).await;
            }
        }
    }
}
