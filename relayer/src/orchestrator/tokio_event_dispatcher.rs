use crate::core::event::RelayerEvent;
use crate::orchestrator::traits::{Event, EventHandler};
use anyhow::Error;
use dashmap::DashMap;
use std::sync::Arc;
use tokio::task::JoinHandle;
use tokio_util::task::TaskTracker;
use tracing::{debug, instrument, Instrument};

type EventHandlerMap = Arc<DashMap<u8, Vec<Arc<dyn EventHandler<RelayerEvent>>>>>;

pub struct TokioEventDispatcher {
    // (event-type-id) -> EventHandler
    subscribers: EventHandlerMap,
    /// Tracks the per-(event, handler) dispatch tasks spawned below. Shutdown abandons
    /// them rather than waiting: an event whose handlers have not returned never lets the
    /// block cursor past it, so the next start re-reads it from the chain.
    detached_tasks: TaskTracker,
}

impl TokioEventDispatcher {
    pub fn new(detached_tasks: TaskTracker) -> Self {
        Self {
            subscribers: Arc::new(DashMap::new()),
            detached_tasks,
        }
    }

    #[instrument(skip_all, fields(event_type=%(event.event_name()), job_id=?event.job_id()))]
    pub async fn dispatch_event(&self, event: RelayerEvent) -> Result<(), Error> {
        self.spawn_handlers(event);
        Ok(())
    }

    /// Dispatch, then wait until every subscribed handler has finished with the event.
    ///
    /// The handlers run as the same tracked tasks a plain dispatch spawns, so shutdown
    /// treats them identically; the caller merely also learns when they are done. Returns
    /// an error if any handler panicked, which `handled_events` logs before it lets the block
    /// cursor past the event.
    #[instrument(skip_all, fields(event_type=%(event.event_name()), job_id=?event.job_id()))]
    pub async fn dispatch_event_and_wait(&self, event: RelayerEvent) -> Result<(), Error> {
        let mut panicked = 0usize;
        for handle in self.spawn_handlers(event) {
            if let Err(e) = handle.await {
                panicked += 1;
                debug!(error = %e, "Handler task did not complete");
            }
        }

        if panicked > 0 {
            return Err(anyhow::anyhow!(
                "{panicked} handler task(s) did not complete"
            ));
        }

        Ok(())
    }

    /// Spawn one tracked task per subscribed handler, returning their join handles.
    fn spawn_handlers(&self, event: RelayerEvent) -> Vec<JoinHandle<()>> {
        let Some(handlers) = self.subscribers.get(&event.event_id()) else {
            debug!(
                "Dispatching event {}({:?}) didn't match any handler.",
                event.event_name(),
                event.job_id(),
            );
            return Vec::new();
        };

        let handlers = handlers.clone();
        debug!(
            "Dispatching {}({:?}) to {} handlers.",
            event.event_name(),
            event.job_id(),
            handlers.len()
        );

        handlers
            .into_iter()
            .map(|handler| {
                let event = event.clone();
                let current_span = tracing::Span::current();
                self.detached_tasks.spawn(async move {
                    handler.handle_event(event).instrument(current_span).await
                })
            })
            .collect()
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
