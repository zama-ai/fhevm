use crate::{
    core::{
        KmsResponsePublisher,
        config::Config,
        event_picker::{DbEventPicker, EventPicker},
        event_processor::{
            DbEventProcessor, DecryptionProcessor, EventProcessor, KMSGenerationProcessor,
            KmsClient, s3::S3Service,
        },
        kms_response_publisher::DbKmsResponsePublisher,
    },
    monitoring::health::{KmsHealthClient, State},
};
use alloy::transports::http::reqwest;
use anyhow::anyhow;
use connector_utils::{
    conn::{GatewayProvider, connect_to_db, connect_to_gateway},
    tasks::spawn_with_limit,
    types::{GatewayEvent, KmsResponse},
};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};
use tracing_opentelemetry::OpenTelemetrySpanExt;

/// Struct processing stored Gateway's events.
pub struct KmsWorker<E, Proc, Publ> {
    /// The entity responsible for picking events to process.
    event_picker: E,

    /// The entity responsible for processing events.
    event_processor: Proc,

    /// The entity responsible for publishing KMS Core's responses.
    response_publisher: Publ,
}

impl<E, Proc, Publ> KmsWorker<E, Proc, Publ>
where
    E: EventPicker<Event = GatewayEvent>,
    Proc: EventProcessor<Event = GatewayEvent> + Clone + Send + 'static,
    Publ: KmsResponsePublisher + Clone + Send + 'static,
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

    /// Runs the event processing loop of the `KmsWorker`.
    async fn run(mut self) {
        loop {
            match self.event_picker.pick_events().await {
                Ok(events) => self.spawn_event_processing_tasks(events).await,
                Err(e) => warn!("Error while picking events: {e}"),
            };
        }
    }

    /// Spawns a new task to process each event.
    async fn spawn_event_processing_tasks(&self, events: Vec<GatewayEvent>) {
        for event in events {
            let event_processor = self.event_processor.clone();
            let response_publisher = self.response_publisher.clone();

            spawn_with_limit(async move {
                Self::process_event(event_processor, response_publisher, event).await
            })
            .await;
        }
    }

    /// Processes an event coming from the Gateway.
    #[tracing::instrument(skip(event_processor, response_publisher), fields(event = % event.kind))]
    async fn process_event(
        mut event_processor: Proc,
        response_publisher: Publ,
        event: GatewayEvent,
    ) {
        let otlp_context = event.otlp_context.clone();
        tracing::Span::current().set_parent(otlp_context.extract());

        let response_kind = match event_processor.process(&event).await {
            Ok(response) => response,
            Err(e) => return error!("{e}"),
        };

        let response = KmsResponse::new(response_kind, otlp_context);
        if let Err(e) = response_publisher.publish_response(response).await {
            error!("Failed to publish response: {e}");
        }
    }
}

impl KmsWorker<DbEventPicker, DbEventProcessor<GatewayProvider>, DbKmsResponsePublisher> {
    /// Creates a new `KmsWorker` instance from a valid `Config`.
    pub async fn from_config(config: Config) -> anyhow::Result<(Self, State<GatewayProvider>)> {
        let db_pool = connect_to_db(&config.database_url, config.database_pool_size).await?;
        let provider = connect_to_gateway(&config.gateway_url, config.chain_id).await?;
        let kms_client = KmsClient::connect(&config).await?;
        let kms_health_client = KmsHealthClient::connect(&config.kms_core_endpoints).await?;
        let s3_client = reqwest::Client::builder()
            .connect_timeout(config.s3_connect_timeout)
            .build()
            .map_err(|e| anyhow!("Failed to create HTTP client: {}", e))?;

        let event_picker = DbEventPicker::connect(db_pool.clone(), &config).await?;

        let s3_service = S3Service::new(&config, provider.clone(), s3_client);
        let decryption_processor = DecryptionProcessor::new(&config, s3_service);
        let kms_generation_processor = KMSGenerationProcessor::new(&config);
        let event_processor = DbEventProcessor::new(
            kms_client.clone(),
            decryption_processor,
            kms_generation_processor,
            db_pool.clone(),
        );
        let response_publisher = DbKmsResponsePublisher::new(db_pool.clone());

        let state = State::new(
            db_pool,
            provider,
            kms_health_client,
            config.healthcheck_timeout,
        );
        let kms_worker = KmsWorker::new(event_picker, event_processor, response_publisher);
        Ok((kms_worker, state))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::event_processor::ProcessingError;
    use connector_utils::{
        tests::rand::{rand_signature, rand_u256},
        types::{GatewayEvent, KmsResponse, KmsResponseKind, UserDecryptionResponse},
    };
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
        async fn process(
            &mut self,
            _event: &Self::Event,
        ) -> Result<KmsResponseKind, ProcessingError> {
            Ok(KmsResponseKind::UserDecryption(UserDecryptionResponse {
                decryption_id: rand_u256(),
                user_decrypted_shares: vec![],
                signature: rand_signature(),
                extra_data: vec![],
            }))
        }
    }

    #[derive(Clone)]
    struct MockResponsePublisher {}

    impl KmsResponsePublisher for MockResponsePublisher {
        async fn publish_response(&self, _response: KmsResponse) -> anyhow::Result<()> {
            info!("Response has been published");
            Ok(())
        }
    }
}
