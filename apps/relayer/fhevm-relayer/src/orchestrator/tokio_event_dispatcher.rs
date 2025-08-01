use super::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::traits::Event;
use crate::orchestrator::traits::EventHandler;
use anyhow::Error;
use async_trait::async_trait;
use dashmap::DashMap;
use std::sync::Arc;
use tracing::{debug, instrument, Instrument};
use uuid::Uuid;

type EventHandlerMap<K, E> = Arc<DashMap<K, Vec<Arc<dyn EventHandler<E>>>>>;

pub struct TokioEventDispatcher<E: Event + std::fmt::Debug> {
    // (event-type-id, event-id) -> EventHandler
    once_subscribers: EventHandlerMap<(u8, Uuid), E>,
    // (event-type-id) -> EventHandler
    suscribers: EventHandlerMap<u8, E>,
}

#[allow(clippy::new_without_default)]
impl<E: Event + std::fmt::Debug> TokioEventDispatcher<E> {
    pub fn new() -> Self {
        Self {
            once_subscribers: Arc::new(DashMap::new()),
            suscribers: Arc::new(DashMap::new()),
        }
    }
}

#[async_trait]
impl<E: Event + std::fmt::Debug> EventDispatcher<E> for TokioEventDispatcher<E> {
    #[instrument(skip_all, fields(event_type=%(event.event_name()), request_id=%(event.request_id())))]
    async fn dispatch_event(&self, event: E) -> Result<(), Error> {
        let event = event.clone();
        // Handle once subscriptions
        // In this situation we remove the handler from our mapping
        if let Some((_, handlers)) = self
            .once_subscribers
            .remove(&(event.event_id(), event.request_id()))
        {
            let handlers = handlers.clone();
            debug!(
                "Dispatching {}({}) to {} once-handlers.",
                event.event_name(),
                event.request_id(),
                handlers.len()
            );
            for handler in handlers {
                let event = event.clone();
                let current_span = tracing::Span::current();
                tokio::spawn(
                    async move { handler.handle_event(event).instrument(current_span).await },
                );
            }
        // Handle usual subscriptions if no once-handler were found
        } else if let Some(handlers) = self.suscribers.get(&event.event_id()) {
            let handlers = handlers.clone();
            debug!(
                "Dispatching {}({}) to {} generic-handlers.",
                event.event_name(),
                event.request_id(),
                handlers.len()
            );
            for handler in handlers {
                let event = event.clone();
                let current_span = tracing::Span::current();
                tokio::spawn(
                    async move { handler.handle_event(event).instrument(current_span).await },
                );
            }
        } else {
            debug!(
                "Dispatching event {}({}) didn't match any handler.",
                event.event_name(),
                event.request_id(),
            );
        }
        Ok(())
    }
}

impl<E: Event + std::fmt::Debug> HandlerRegistry<E> for TokioEventDispatcher<E> {
    #[instrument(skip(self, handler))]
    fn register_handler(&self, event_id: u8, handler: Arc<dyn EventHandler<E>>) {
        self.suscribers.entry(event_id).or_default().push(handler);
        debug!("Generic-Handler registered for {}", event_id);
    }

    #[instrument(skip(self, handler))]
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
        debug!("Once-Handler registered for {},{}", event_id, request_id);
    }
}
