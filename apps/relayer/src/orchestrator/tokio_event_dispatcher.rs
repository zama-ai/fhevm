use super::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::traits::Event;
use crate::orchestrator::traits::EventHandler;
use anyhow::Error;
use async_trait::async_trait;
use dashmap::DashMap;
use std::sync::Arc;
use tracing::{debug, instrument, Instrument};

type EventHandlerMap<K, E> = Arc<DashMap<K, Vec<Arc<dyn EventHandler<E>>>>>;

pub struct TokioEventDispatcher<E: Event + std::fmt::Debug> {
    // (event-type-id) -> EventHandler
    subscribers: EventHandlerMap<u8, E>,
}

#[allow(clippy::new_without_default)]
impl<E: Event + std::fmt::Debug> TokioEventDispatcher<E> {
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(DashMap::new()),
        }
    }
}

#[async_trait]
impl<E: Event + std::fmt::Debug> EventDispatcher<E> for TokioEventDispatcher<E> {
    #[instrument(skip_all, fields(event_type=%(event.event_name()), job_id=?event.job_id()))]
    async fn dispatch_event(&self, event: E) -> Result<(), Error> {
        let event = event.clone();
        if let Some(handlers) = self.subscribers.get(&event.event_id()) {
            let handlers = handlers.clone();
            debug!(
                "Dispatching {}({:?}) to {} handlers.",
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
                "Dispatching event {}({:?}) didn't match any handler.",
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
            self.subscribers
                .entry(*event_id)
                .or_default()
                .push(Arc::clone(&handler));
        }
        debug!(
            "Handler registered for {} events: {:?}",
            event_ids.len(),
            event_ids
        );
    }
}
