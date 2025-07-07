use crate::core::{EventPicker, event_processor::EventProcessor};
use alloy::providers::Provider;
use connector_utils::types::GatewayEvent;
use tokio_util::sync::CancellationToken;
use tracing::{debug, warn};

/// Struct that processes Gateway's events coming from a `Postgres` database.
pub struct EventProcessorService<E, P: Provider> {
    /// The entity responsible for picking events to process.
    event_picker: E,

    /// The entity responsible of processing the incoming events.
    event_processor: EventProcessor<P>,
}

impl<E, P> EventProcessorService<E, P>
where
    E: EventPicker<Event = GatewayEvent>,
    P: Provider + Clone + 'static,
{
    /// Creates a new `EventProcessorService<E, P>` instance.
    pub fn new(event_picker: E, event_processor: EventProcessor<P>) -> Self {
        Self {
            event_picker,
            event_processor,
        }
    }

    /// Starts the `EventProcessorService`.
    pub async fn start(self, cancel_token: CancellationToken) {
        debug!("Starting EventProcessorService");

        self.event_processor
            .kms_client
            .spawn_requests_results_collection(cancel_token.clone());

        tokio::select! {
            _ = cancel_token.cancelled() => debug!("Stopping EventProcessorService"),
            _ = self.run() => (),
        }
    }

    /// Runs the event handling loop of the `EventProcessorService`.
    async fn run(mut self) {
        loop {
            match self.event_picker.pick_events().await {
                Ok(events) => {
                    for event in events {
                        let mut event_processor = self.event_processor.clone();
                        tokio::spawn(async move {
                            event_processor.process(&event).await;
                        });
                    }
                }
                Err(e) => warn!("Error while picking events: {e}"),
            };
        }
    }
}
