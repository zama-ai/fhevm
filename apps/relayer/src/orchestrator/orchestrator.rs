use crate::orchestrator::traits::Event;
use crate::orchestrator::traits::{
    EventDispatcher, EventHandler, HandlerRegistry, HookRegistry, PreDispatchHook,
};
use anyhow::Error;
use async_trait::async_trait;
use std::sync::{Arc, RwLock};
use tracing::{error, info, instrument};
use uuid::Uuid;

pub struct Orchestrator<D: EventDispatcher<E> + HandlerRegistry<E>, E: Event> {
    event_dispatcher: Arc<D>,
    pre_dispatch_hooks: Arc<RwLock<Vec<Arc<dyn PreDispatchHook<E>>>>>,
    _marker: std::marker::PhantomData<E>,
}

impl<D: EventDispatcher<E> + HandlerRegistry<E>, E: Event> Orchestrator<D, E> {
    pub fn new(event_dispatcher: Arc<D>) -> Arc<Self> {
        Arc::new(Self {
            event_dispatcher,
            pre_dispatch_hooks: Arc::new(RwLock::new(Vec::new())),
            _marker: std::marker::PhantomData,
        })
    }

    pub fn new_request_id(&self) -> Uuid {
        Uuid::new_v4()
    }

    #[instrument(skip_all, fields(event_type=%(event.event_name()), request_id=%(event.request_id())))]
    async fn run_pre_dispatch_hooks_sequentially(&self, event: E) {
        // Acquire lock and prepare all hooks as Futures.
        let hooks: Vec<_> = if let Ok(hooks_guard) = self.pre_dispatch_hooks.read() {
            hooks_guard.iter().cloned().collect()
        } else {
            error!("Failed to acquire read lock on pre-dispatch hooks");
            return;
        };

        // Execute the futures sequentially.
        for hook in hooks {
            info!("Running pre-dispatch hook: {}", hook);
            hook.run(event.clone()).await;
        }
    }
}

#[async_trait]
impl<D: EventDispatcher<E> + HandlerRegistry<E>, E: Event> EventDispatcher<E>
    for Orchestrator<D, E>
{
    #[instrument(skip_all, fields(event_type=%(event.event_name()), request_id=%(event.request_id())))]
    async fn dispatch_event(&self, event: E) -> Result<(), Error> {
        self.run_pre_dispatch_hooks_sequentially(event.clone())
            .await;
        self.event_dispatcher.dispatch_event(event).await
    }
}
#[async_trait::async_trait]
impl<D: EventDispatcher<E> + HandlerRegistry<E>, E: Event> HookRegistry<E> for Orchestrator<D, E> {
    fn register_pre_dispatch_hook(&self, hook: Arc<dyn PreDispatchHook<E>>) {
        if let Ok(mut hooks) = self.pre_dispatch_hooks.write() {
            hooks.push(hook);
        } else {
            error!("Failed to acquire write lock on pre-dispatch hooks");
        }
    }
}

impl<D: EventDispatcher<E> + HandlerRegistry<E>, E: Event> HandlerRegistry<E>
    for Orchestrator<D, E>
{
    fn register_handler(&self, event_id: u8, handler: Arc<dyn EventHandler<E>>) {
        self.event_dispatcher.register_handler(event_id, handler);
    }

    fn register_once_handler(
        &self,
        event_id: u8,
        request_id: Uuid,
        handler: Arc<dyn EventHandler<E>>,
    ) {
        self.event_dispatcher
            .register_once_handler(event_id, request_id, handler);
    }
}
