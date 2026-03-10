use crate::core::event::RelayerEvent;
use crate::orchestrator::traits::{Event, EventHandler};
use anyhow::Error;
use dashmap::DashMap;
use std::sync::Arc;
use tracing::{debug, instrument, Instrument};

type EventHandlerMap = Arc<DashMap<u8, Vec<Arc<dyn EventHandler<RelayerEvent>>>>>;

pub struct TokioEventDispatcher {
    // (event-type-id) -> EventHandler
    subscribers: EventHandlerMap,
}

#[allow(clippy::new_without_default)]
impl TokioEventDispatcher {
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(DashMap::new()),
        }
    }

    #[instrument(skip_all, fields(event_type=%(event.event_name()), job_id=?event.job_id()))]
    pub async fn dispatch_event(&self, event: RelayerEvent) -> Result<(), Error> {
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

    #[instrument(skip(self, handler))]
    pub fn register_handler(&self, event_ids: &[u8], handler: Arc<dyn EventHandler<RelayerEvent>>) {
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
