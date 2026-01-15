use crate::core::job_id::JobId;
use anyhow::Error;
use async_trait::async_trait;
use std::fmt::Display;
use std::sync::Arc;

pub trait Event: Clone + Send + Sync + 'static {
    fn event_name(&self) -> &str;
    fn event_id(&self) -> u8;
    fn job_id(&self) -> JobId;
    fn timestamp(&self) -> u64;
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
    fn register_handler(&self, event_ids: &[u8], handler: Arc<dyn EventHandler<E>>);
    fn register_once_handler(&self, event_id: u8, job_id: JobId, handler: Arc<dyn EventHandler<E>>);
    fn unregister_once_handler(&self, event_id: u8, job_id: JobId);
}

pub trait HookRegistry<E: Event> {
    fn register_pre_dispatch_hook(&self, hook: Arc<dyn PreDispatchHook<E>>);
}

pub trait HooksRunner<E: Event> {
    fn run_hooks(&self, event: E);
}

#[async_trait]
pub trait PreDispatchHook<E: Event>: Display + Send + Sync {
    async fn run(&self, event: E);
}
