use crate::orchestrator::event::traits::Event;
use crate::orchestrator::handler::traits::EventHandler;
use anyhow::Error;
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait Publisher<E: Event>: Send + Sync {
    async fn publish(&self, event: E) -> Result<(), Error>;
}

pub trait Subscriber<E: Event> {
    fn subscribe(&self, event_id: u8, handler: Arc<dyn EventHandler<E>>);
    fn subscribe_once(&self, event_id: u8, request_id: Uuid, handler: Arc<dyn EventHandler<E>>);
}
