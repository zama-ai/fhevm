use anyhow::Error;
use async_trait::async_trait;
use std::fmt::Display;
use std::sync::Arc;
use uuid::Uuid;

pub trait Event: Clone + Send + Sync + 'static {
    fn event_name(&self) -> &str;
    fn event_id(&self) -> u8;
    fn request_id(&self) -> Uuid;
}

#[async_trait]
pub trait EventDispatcher<E: Event>: Send + Sync {
    async fn dispatch_event(&self, event: E) -> Result<(), Error>;
}

#[async_trait]
pub trait EventHandler<E: Event>: Send + Sync {
    async fn handle_event(&self, event: E);
}

pub trait HandlerRegistry<E: Event> {
    fn register_handler(&self, event_id: u8, handler: Arc<dyn EventHandler<E>>);
    fn register_once_handler(
        &self,
        event_id: u8,
        request_id: Uuid,
        handler: Arc<dyn EventHandler<E>>,
    );
}

pub trait HookRegistry<E: Event> {
    fn register_pre_dispatch_hook(&self, hook: Arc<dyn PreDispatchHook<E>>);
}

pub trait HooksRunner<E: Event> {
    fn run_hooks(&self, event: E);
}

pub trait PreDispatchHook<E: Event>: Display + Send + Sync {
    fn run(&self, event: E);
}
