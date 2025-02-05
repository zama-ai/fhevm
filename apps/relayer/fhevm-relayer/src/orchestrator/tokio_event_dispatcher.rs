use super::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::traits::Event;
use crate::orchestrator::traits::EventHandler;
use anyhow::Error;
use async_trait::async_trait;
use dashmap::DashMap;
use std::sync::Arc;
use uuid::Uuid;

pub struct TokioEventDispatcher<E: Event> {
    once_subscribers: Arc<DashMap<(u8, Uuid), Arc<dyn EventHandler<E>>>>,
    suscribers: Arc<DashMap<u8, Arc<dyn EventHandler<E>>>>,
}

impl<E: Event> TokioEventDispatcher<E> {
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
impl<E: Event> EventDispatcher<E> for TokioEventDispatcher<E> {
    async fn dispatch(&self, event: E) -> Result<(), Error> {
        self.handle_event(event);
        Ok(())
    }
}

impl<E: Event> HandlerRegistry<E> for TokioEventDispatcher<E> {
    fn register_handler(&self, event_id: u8, handler: Arc<dyn EventHandler<E>>) {
        self.suscribers.insert(event_id, handler);
    }

    fn register_once_handler(
        &self,
        event_id: u8,
        request_id: Uuid,
        handler: Arc<dyn EventHandler<E>>,
    ) {
        self.once_subscribers
            .insert((event_id, request_id), handler);
    }
}
