use super::traits::{EventDispatcher, HandlerRegistry};
use crate::core::job_id::JobId;
use crate::orchestrator::traits::Event;
use crate::orchestrator::traits::EventHandler;
use anyhow::Error;
use async_trait::async_trait;
use dashmap::DashMap;
use std::sync::Arc;
use tracing::{debug, instrument, Instrument};

type EventHandlerMap<K, E> = Arc<DashMap<K, Vec<Arc<dyn EventHandler<E>>>>>;

pub struct TokioEventDispatcher<E: Event + std::fmt::Debug> {
    // (event-type-id, workflow-id) -> EventHandler
    once_subscribers: EventHandlerMap<(u8, JobId), E>,
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
    #[instrument(skip_all, fields(event_type=%(event.event_name()), job_id=%(event.job_id())))]
    async fn dispatch_event(&self, event: E) -> Result<(), Error> {
        let event = event.clone();
        // Handle once subscriptions
        // In this situation we remove the handler from our mapping
        if let Some((_, handlers)) = self
            .once_subscribers
            .remove(&(event.event_id(), event.job_id()))
        {
            let handlers = handlers.clone();
            debug!(
                "Dispatching {}({}) to {} once-handlers.",
                event.event_name(),
                event.job_id(),
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
                event.job_id(),
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
                event.job_id(),
            );
        }
        Ok(())
    }
}

impl<E: Event + std::fmt::Debug> HandlerRegistry<E> for TokioEventDispatcher<E> {
    #[instrument(skip(self, handler))]
    fn register_handler(&self, event_ids: &[u8], handler: Arc<dyn EventHandler<E>>) {
        for event_id in event_ids {
            self.suscribers
                .entry(*event_id)
                .or_default()
                .push(Arc::clone(&handler));
        }
        debug!(
            "Generic-Handler registered for {} events: {:?}",
            event_ids.len(),
            event_ids
        );
    }

    #[instrument(skip(self, handler))]
    fn register_once_handler(
        &self,
        event_id: u8,
        job_id: JobId,
        handler: Arc<dyn EventHandler<E>>,
    ) {
        self.once_subscribers
            .entry((event_id, job_id))
            .or_default()
            .push(handler);
        debug!("Once-Handler registered for {},{}", event_id, job_id);
    }

    #[instrument(skip(self))]
    fn unregister_once_handler(&self, event_id: u8, job_id: JobId) {
        self.once_subscribers.remove(&(event_id, job_id));
        debug!("Once-Handler unregistered for {},{}", event_id, job_id);
    }
}
