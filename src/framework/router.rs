use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;
use std::future::Future;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use crate::framework::context::{Context, Ctx};
use crate::models::gateway::{GatewayDispatch, GatewayDispatchEvents as GwEvt};

/// Typed handler: deserializes `d` into `T` and calls `F(Ctx, T)`.
#[async_trait]
pub trait DynHandler: Send + Sync {
    async fn call(&self, ctx: Arc<Context>, ev: Arc<GatewayDispatch<Value>>);
    fn event(&self) -> GwEvt;
    fn is_once(&self) -> bool {
        false
    }
}

struct Handler<T, F> {
    evt: GwEvt,
    f: F,
    once: bool,
    _phantom: PhantomData<T>,
}

#[async_trait]
impl<T, F, Fut> DynHandler for Handler<T, F>
where
    T: DeserializeOwned + Send + Sync + 'static,
    F: Send + Sync + 'static + Fn(Ctx, T) -> Fut,
    Fut: Future<Output = ()> + Send + 'static,
{
    async fn call(&self, ctx: Arc<Context>, ev: Arc<GatewayDispatch<Value>>) {
        // Defensive: ignore if different event
        if ev.t != self.evt {
            return;
        }

        match serde_json::from_value::<T>(ev.d.clone()) {
            Ok(payload) => {
                let c = Ctx {
                    app: Arc::clone(&ctx),
                    ev: Arc::clone(&ev),
                };
                (self.f)(c, payload).await;
            }
            Err(err) => {
                crate::log!(
                    "WARN",
                    "Failed to decode payload for {:?}: {}",
                    self.evt,
                    err
                );
            }
        }
    }

    #[inline]
    fn event(&self) -> GwEvt {
        self.evt.clone() // if GwEvt: Copy, you can return self.evt
    }

    #[inline]
    fn is_once(&self) -> bool {
        self.once
    }
}

/// “Any” handler: runs for every event, only receives `Ctx`.
#[async_trait]
pub trait DynAnyHandler: Send + Sync {
    async fn call(&self, ctx: Arc<Context>, ev: Arc<GatewayDispatch<Value>>);
}

#[async_trait]
impl<F, Fut> DynAnyHandler for F
where
    F: Send + Sync + 'static + Fn(Ctx) -> Fut,
    Fut: Future<Output = ()> + Send + 'static,
{
    async fn call(&self, ctx: Arc<Context>, ev: Arc<GatewayDispatch<Value>>) {
        let c = Ctx {
            app: Arc::clone(&ctx),
            ev: Arc::clone(&ev),
        };
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

    /// Register a persistent handler:
    /// client.on(GwEvt::MessageCreate, |ctx, mc: GatewayMessageCreateDispatchData| async move { ... });
    pub fn on<T, F, Fut>(&mut self, kind: GwEvt, handler: F)
    where
        T: DeserializeOwned + Send + Sync + 'static,
        F: Send + Sync + 'static + Fn(Ctx, T) -> Fut,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let evt_for_handler = kind.clone();
        let h: Arc<dyn DynHandler> = Arc::new(Handler::<T, F> {
            evt: evt_for_handler,
            f: handler,
            once: false,
            _phantom: PhantomData,
        });

        self.routes.entry(kind).or_default().push(h);
    }

    /// Register a one-shot handler (removed after first call).
    pub fn once<T, F, Fut>(&mut self, kind: GwEvt, handler: F)
    where
        T: DeserializeOwned + Send + Sync + 'static,
        F: Send + Sync + 'static + Fn(Ctx, T) -> Fut,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let evt_for_handler = kind.clone();
        let h: Arc<dyn DynHandler> = Arc::new(Handler::<T, F> {
            evt: evt_for_handler,
            f: handler,
            once: true,
            _phantom: PhantomData,
        });

        let mut map = self.once_routes.lock().unwrap();
        map.entry(kind).or_default().push(h);
    }

    /// Runs for every event; receives only `Ctx`.
    pub fn on_all<F, Fut>(&mut self, handler: F)
    where
        F: Send + Sync + 'static + Fn(Ctx) -> Fut,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.any.push(Arc::new(handler));
    }

    /// Runs when no typed route matched (optional).
    pub fn on_unknown<F, Fut>(&mut self, handler: F)
    where
        F: Send + Sync + 'static + Fn(Ctx) -> Fut,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.unknown.push(Arc::new(handler));
    }

    /// Dispatch a typed Gateway dispatch (op=0) to registered handlers.
    pub async fn dispatch(&self, ctx: &Arc<Context>, ev: &GatewayDispatch<Value>) {
        let ctx_arc = Arc::clone(ctx);
        let ev_arc = Arc::new(ev.clone());

        // Any-handlers
        for h in &self.any {
            h.call(Arc::clone(&ctx_arc), Arc::clone(&ev_arc)).await;
        }

        // Event key (clone to avoid moving from `&ev`)
        let kind = ev.t.clone();

        // Drain once-handlers for this event
        if let Some(list) = self.once_routes.lock().unwrap().remove(&kind) {
            for h in list {
                h.call(Arc::clone(&ctx_arc), Arc::clone(&ev_arc)).await;
            }
        }

        // Persistent handlers
        if let Some(list) = self.routes.get(&kind) {
            for h in list {
                h.call(Arc::clone(&ctx_arc), Arc::clone(&ev_arc)).await;
            }
        } else {
            for h in &self.unknown {
                h.call(Arc::clone(&ctx_arc), Arc::clone(&ev_arc)).await;
            }
        }
    }
}
