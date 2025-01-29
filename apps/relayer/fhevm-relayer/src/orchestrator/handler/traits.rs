use crate::orchestrator::event::traits::Event;
use async_trait::async_trait;

#[async_trait]
pub trait EventHandler<E: Event>: Send + Sync {
    fn handle(&self, event: E);
}
