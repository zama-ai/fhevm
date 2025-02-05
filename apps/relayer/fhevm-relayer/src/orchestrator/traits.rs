use anyhow::Error;
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

pub trait Event: Clone + Send + Sync {
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
    fn handle_event(&self, event: E);
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
