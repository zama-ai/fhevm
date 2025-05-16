use super::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::traits::Event;
use crate::orchestrator::traits::EventHandler;
use anyhow::Error;
use async_trait::async_trait;
use dashmap::DashMap;
use std::sync::Arc;
use uuid::Uuid;

type EventHandlerMap<K, E> = Arc<DashMap<K, Vec<Arc<dyn EventHandler<E>>>>>;

pub struct TokioEventDispatcher<E: Event> {
    once_subscribers: EventHandlerMap<(u8, Uuid), E>,
    suscribers: EventHandlerMap<u8, E>,
}

#[allow(clippy::new_without_default)]
impl<E: Event> TokioEventDispatcher<E> {
    pub fn new() -> Self {
        Self {
            once_subscribers: Arc::new(DashMap::new()),
            suscribers: Arc::new(DashMap::new()),
        }
    }
}

#[async_trait]
impl<E: Event> EventDispatcher<E> for TokioEventDispatcher<E> {
    async fn dispatch_event(&self, event: E) -> Result<(), Error> {
        let event = event.clone();
        if let Some((_, handlers)) = self
            .once_subscribers
            .remove(&(event.event_id(), event.request_id()))
        {
            let handlers = handlers.clone();
            for handler in handlers {
                let event = event.clone();
                tokio::spawn(async move { handler.handle_event(event).await });
            }
        } else if let Some(handlers) = self.suscribers.get(&event.event_id()) {
            let handlers = handlers.clone();
            for handler in handlers {
                let event = event.clone();
                tokio::spawn(async move { handler.handle_event(event).await });
            }
        } else {
            // Log warning and ignore event.
        }
        Ok(())
    }
}

impl<E: Event> HandlerRegistry<E> for TokioEventDispatcher<E> {
    fn register_handler(&self, event_id: u8, handler: Arc<dyn EventHandler<E>>) {
        self.suscribers.entry(event_id).or_default().push(handler);
    }

    fn register_once_handler(
        &self,
        event_id: u8,
        request_id: Uuid,
        handler: Arc<dyn EventHandler<E>>,
    ) {
        self.once_subscribers
            .entry((event_id, request_id))
            .or_default()
            .push(handler);
    }
}
