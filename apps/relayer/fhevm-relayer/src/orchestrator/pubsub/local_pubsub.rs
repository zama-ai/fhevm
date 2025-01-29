use super::traits::{Publisher, Subscriber};
use crate::orchestrator::event::traits::Event;
use crate::orchestrator::handler::traits::EventHandler;
use anyhow::Error;
use async_trait::async_trait;
use dashmap::DashMap;
use std::sync::Arc;
use uuid::Uuid;

pub struct LocalPubsub<E: Event> {
    once_subscribers: Arc<DashMap<(u8, Uuid), Arc<dyn EventHandler<E>>>>,
    suscribers: Arc<DashMap<u8, Arc<dyn EventHandler<E>>>>,
}

impl<E: Event> LocalPubsub<E> {
    pub fn new() -> Self {
        Self {
            once_subscribers: Arc::new(DashMap::new()),
            suscribers: Arc::new(DashMap::new()),
        }
    }

    fn handle_event(&self, event: E) {
        let event = event.clone();
        if let Some((_, handler)) = self
            .once_subscribers
            .remove(&(event.event_id(), event.request_id()))
        {
            handler.handle(event.clone());
        } else if let Some(handler) = self.suscribers.get(&event.event_id()) {
            let handler = handler.clone();
            handler.handle(event);
        } else {
            // Log warning and ignore event.
        }
    }
}

#[async_trait]
impl<E: Event> Publisher<E> for LocalPubsub<E> {
    async fn publish(&self, event: E) -> Result<(), Error> {
        self.handle_event(event);
        Ok(())
    }
}

impl<E: Event> Subscriber<E> for LocalPubsub<E> {
    fn subscribe(&self, event_id: u8, handler: Arc<dyn EventHandler<E>>) {
        self.suscribers.insert(event_id, handler);
    }

    fn subscribe_once(&self, event_id: u8, request_id: Uuid, handler: Arc<dyn EventHandler<E>>) {
        self.once_subscribers
            .insert((event_id, request_id), handler);
    }
}
