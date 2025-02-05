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
pub trait Dispatcher<E: Event>: Send + Sync {
    async fn dispatch(&self, event: E) -> Result<(), Error>;
}

#[async_trait]
pub trait EventHandler<E: Event>: Send + Sync {
    fn handle(&self, event: E);
}

pub trait HandleRegistry<E: Event> {
    fn register_handler(&self, event_id: u8, handler: Arc<dyn EventHandler<E>>);
    fn register_once_handler(
        &self,
        event_id: u8,
        request_id: Uuid,
        handler: Arc<dyn EventHandler<E>>,
    );
}
