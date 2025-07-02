use std::fmt::Display;

use crate::core::{
    KmsResponsePublisher,
    config::Config,
    event_picker::{DbEventPicker, EventPicker},
    event_processor::{
        DbEventProcessor, DecryptionProcessor, EventProcessor, KmsClient, s3::S3Service,
    },
    kms_response_publisher::DbKmsResponsePublisher,
};
use connector_utils::conn::{GatewayProvider, connect_to_db, connect_to_gateway};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

/// Struct processing stored Gateway's events.
pub struct KmsWorker<E, Proc, Publ> {
    /// The entity responsible for picking events to process.
    event_picker: E,

    /// The entity responsible for processing events.
    event_processor: Proc,

    /// The entity responsible for publishing KMS Core's responses.
    response_publisher: Publ,
}

impl<E, Proc, Publ, T> KmsWorker<E, Proc, Publ>
where
    E: EventPicker<Event = T>,
    Proc: EventProcessor<Event = T> + Clone + Send + 'static,
    Publ: KmsResponsePublisher + Clone + Send + 'static,
    T: Send + Sync + 'static + Display,
{
    /// Creates a new `KmsWorker<E, Proc, Publ, R>`.
    pub fn new(event_picker: E, event_processor: Proc, response_publisher: Publ) -> Self {
        Self {
            event_picker,
            event_processor,
            response_publisher,
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

    /// Runs the event handling loop of the `KmsWorker`.
    async fn run(mut self) {
        loop {
            match self.event_picker.pick_events().await {
                Ok(events) => self.spawn_event_handling_tasks(events),
                Err(e) => warn!("Error while picking events: {e}"),
            };
        }
    }

    /// Spawns a new task dedicated to the handling of an event.
    fn spawn_event_handling_tasks(&self, events: Vec<T>) {
        for event in events {
            let event_processor = self.event_processor.clone();
            let response_publisher = self.response_publisher.clone();

            tokio::spawn(async move {
                Self::handle_event(event_processor, response_publisher, event).await
            });
        }
    }

    /// Handles an event coming from the Gateway.
    async fn handle_event(mut event_processor: Proc, response_publisher: Publ, event: T) {
        let response = match event_processor.process(&event).await {
            Ok(response) => response,
            Err(e) => return error!("Failed to process event: {e}"),
        };

        if let Err(e) = response_publisher.publish(response.clone()).await {
            error!("Failed to publish {response}: {e}");
        }
    }
}

impl KmsWorker<DbEventPicker, DbEventProcessor<GatewayProvider>, DbKmsResponsePublisher> {
    /// Creates a new `KmsWorker` instance from a valid `Config`.
    pub async fn from_config(config: Config) -> anyhow::Result<Self> {
        let db_pool = connect_to_db(&config.database_url, config.database_pool_size).await?;
        let provider = connect_to_gateway(&config.gateway_url).await?;
        let kms_client = KmsClient::connect(&config).await?;

        let event_picker =
            DbEventPicker::connect(db_pool.clone(), config.events_batch_size).await?;

        let s3_service = S3Service::new(&config, provider);
        let decryption_processor = DecryptionProcessor::new(&config, s3_service);
        let event_processor =
            DbEventProcessor::new(kms_client, decryption_processor, db_pool.clone());
        let response_publisher = DbKmsResponsePublisher::new(db_pool);

        Ok(Self::new(event_picker, event_processor, response_publisher))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use connector_tests::rand::{rand_signature, rand_u256};
    use connector_utils::types::{GatewayEvent, KmsResponse};
    use std::time::Duration;
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    async fn test_kms_worker() {
        let event_picker = MockEventPicker::new();
        let event_processor = MockEventProcessor {};
        let response_publisher = MockResponsePublisher {};

        let worker = KmsWorker::new(event_picker, event_processor, response_publisher);

        let cancel_token = CancellationToken::new();
        let worker_task = tokio::spawn(worker.start(cancel_token.clone()));

        // Give time to the worker to process event
        tokio::time::sleep(Duration::from_millis(300)).await;

        cancel_token.cancel();
        worker_task.await.unwrap();

        logs_contain("Event has been picked");
        logs_contain("Response has been published");
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
        type Event = GatewayEvent;
        async fn pick_events(&mut self) -> anyhow::Result<Vec<Self::Event>> {
            if self.first_pick {
                info!("Event has been picked");
                self.first_pick = false;
            } else {
                std::future::pending::<()>().await; // Wait forever
            }
            Ok(vec![])
        }
    }

    #[derive(Clone)]
    struct MockEventProcessor {}

    impl EventProcessor for MockEventProcessor {
        type Event = GatewayEvent;
        async fn process(&mut self, _event: &Self::Event) -> anyhow::Result<KmsResponse> {
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
}
