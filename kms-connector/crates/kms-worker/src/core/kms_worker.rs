use crate::core::{
    KmsResponsePublisher,
    config::Config,
    event_picker::{DbEventPicker, EventPicker},
    event_processor::{
        DbEventProcessor, DecryptionProcessor, EventProcessor, KmsClient, s3::S3Service,
    },
    event_remover::{DbEventRemover, EventRemover},
    kms_response_publisher::DbKmsResponsePublisher,
};
use connector_utils::conn::{GatewayProvider, connect_to_db, connect_to_gateway};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

/// Struct processing stored Gateway's events.
pub struct KmsWorker<E, Proc, Publ, R> {
    /// The entity responsible for picking events to process.
    event_picker: E,

    /// The entity responsible for processing events.
    event_processor: Proc,

    /// The entity responsible for publishing KMS Core's responses.
    response_publisher: Publ,

    /// The entity responsible for removing events from the database.
    event_remover: R,
}

impl<E, Proc, Publ, R, T> KmsWorker<E, Proc, Publ, R>
where
    E: EventPicker<Event = T>,
    Proc: EventProcessor<Event = T> + Clone + Send + 'static,
    Publ: KmsResponsePublisher + Clone + Send + 'static,
    R: EventRemover<Event = T> + Clone + Send + 'static,
    T: Send + Sync + 'static,
{
    /// Creates a new `KmsWorker<E, Proc, Publ, R>`.
    pub fn new(
        event_picker: E,
        event_processor: Proc,
        response_publisher: Publ,
        event_remover: R,
    ) -> Self {
        Self {
            event_picker,
            event_processor,
            response_publisher,
            event_remover,
        }
    }

    /// Starts the `KmsWorker`.
    pub async fn start(self, cancel_token: CancellationToken) {
        info!("Starting KmsWorker");
        tokio::select! {
            _ = cancel_token.cancelled() => info!("Stopping KmsWorker"),
            _ = self.run() => (),
        }
    }

    /// Runs the event processing loop of the `KmsWorker`.
    async fn run(mut self) {
        loop {
            match self.event_picker.pick_event().await {
                Ok(event) => self.spawn_event_processing_task(event),
                Err(e) => {
                    warn!("Error while picking events: {e}");
                }
            };
        }
    }

    /// Spawns a new task dedicated to the processing of an event.
    fn spawn_event_processing_task(&self, event: T) {
        let event_processor = self.event_processor.clone();
        let response_publisher = self.response_publisher.clone();
        let event_remover = self.event_remover.clone();

        tokio::spawn(async move {
            let response = match event_processor.process(&event).await {
                Ok(response) => response,
                Err(e) => return error!("Failed to process event: {e}"),
            };

            if let Err(e) = response_publisher.publish(response.clone()).await {
                return error!("Failed to publish {response}: {e}");
            }

            event_remover.remove_event(event).await;
        });
    }
}

impl
    KmsWorker<
        DbEventPicker,
        DbEventProcessor<GatewayProvider>,
        DbKmsResponsePublisher,
        DbEventRemover,
    >
{
    /// Creates a new `KmsWorker` instance from a valid `Config`.
    pub async fn from_config(config: Config) -> anyhow::Result<Self> {
        let db_pool = connect_to_db(&config.database_url, config.database_pool_size).await?;
        let provider = connect_to_gateway(&config.gateway_url).await?;
        let kms_client = KmsClient::connect(&config).await?;

        let event_picker = DbEventPicker::connect(db_pool.clone()).await?;

        let s3_service = S3Service::new(&config, provider);
        let decryption_processor = DecryptionProcessor::new(&config, s3_service);
        let event_processor = DbEventProcessor::new(kms_client, decryption_processor);
        let response_publisher = DbKmsResponsePublisher::new(db_pool.clone());
        let event_remover = DbEventRemover::new(db_pool);

        Ok(Self::new(
            event_picker,
            event_processor,
            response_publisher,
            event_remover,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use connector_tests::rand::{rand_signature, rand_u256};
    use connector_utils::types::KmsResponse;
    use std::time::Duration;
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    async fn test_kms_worker() {
        let event_picker = MockEventPicker::new();
        let event_processor = MockEventProcessor {};
        let response_publisher = MockResponsePublisher {};
        let event_remover = MockEventRemover {};

        let worker = KmsWorker::new(
            event_picker,
            event_processor,
            response_publisher,
            event_remover,
        );

        let cancel_token = CancellationToken::new();
        let worker_task = tokio::spawn(worker.start(cancel_token.clone()));

        // Give time to the worker to process event
        tokio::time::sleep(Duration::from_millis(300)).await;

        cancel_token.cancel();
        worker_task.await.unwrap();

        logs_contain("Event has been picked");
        logs_contain("Response has been published");
        logs_contain("Event has been removed");
    }

    struct MockEventPicker {
        first_pick: bool,
    }

    impl MockEventPicker {
        fn new() -> Self {
            Self { first_pick: true }
        }
    }

    impl EventPicker for MockEventPicker {
        type Event = ();
        async fn pick_event(&mut self) -> anyhow::Result<Self::Event> {
            if self.first_pick {
                info!("Event has been picked");
                self.first_pick = false;
            } else {
                std::future::pending::<()>().await; // Wait forever
            }
            Ok(())
        }
    }

    #[derive(Clone)]
    struct MockEventProcessor {}

    impl EventProcessor for MockEventProcessor {
        type Event = ();
        async fn process(self, _event: &Self::Event) -> anyhow::Result<KmsResponse> {
            Ok(KmsResponse::UserDecryption {
                decryption_id: rand_u256(),
                user_decrypted_shares: vec![],
                signature: rand_signature(),
            })
        }
    }

    #[derive(Clone)]
    struct MockResponsePublisher {}

    impl KmsResponsePublisher for MockResponsePublisher {
        async fn publish(&self, _response: KmsResponse) -> anyhow::Result<()> {
            info!("Response has been published");
            Ok(())
        }
    }

    #[derive(Clone)]
    struct MockEventRemover {}

    impl EventRemover for MockEventRemover {
        type Event = ();
        async fn remove_event(&self, _event: Self::Event) -> () {
            info!("Event has been removed");
        }
    }
}
