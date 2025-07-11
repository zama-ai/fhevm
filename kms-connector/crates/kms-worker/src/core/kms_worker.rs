use crate::core::{
    config::Config,
    event_picker::DbEventPicker,
    event_processor::{
        DecryptionProcessor, EventProcessor, EventProcessorService, KmsClient, s3::S3Service,
    },
    kms_response_publisher::KmsResponsePublisher,
};
use connector_utils::conn::{GatewayProvider, connect_to_db, connect_to_gateway};
use tokio::{sync::mpsc, task::JoinSet};
use tokio_util::sync::CancellationToken;

/// The size of the channel used to send KMS Core's response.
const KMS_RESPONSE_CHANNEL_SIZE: usize = 50;

/// Struct processing stored Gateway's events.
pub struct KmsWorker {
    /// The entity responsible for processing events.
    event_processor_service: EventProcessorService<DbEventPicker, GatewayProvider>,

    /// The entity responsible for publishing KMS Core's responses.
    response_publisher: KmsResponsePublisher,
}

impl KmsWorker {
    /// Creates a new `KmsWorker`.
    pub fn new(
        event_processor_service: EventProcessorService<DbEventPicker, GatewayProvider>,
        response_publisher: KmsResponsePublisher,
    ) -> Self {
        Self {
            event_processor_service,
            response_publisher,
        }
    }

    /// Starts the `KmsWorker`.
    pub async fn start(self, cancel_token: CancellationToken) {
        let mut tasks = JoinSet::new();
        tasks.spawn(self.event_processor_service.start(cancel_token.clone()));
        tasks.spawn(self.response_publisher.start(cancel_token));
        tasks.join_all().await;
    }

    /// Creates a new `KmsWorker` instance from a valid `Config`.
    pub async fn from_config(config: Config) -> anyhow::Result<Self> {
        let (response_sender, response_receiver) = mpsc::channel(KMS_RESPONSE_CHANNEL_SIZE);
        let db_pool = connect_to_db(&config.database_url, config.database_pool_size).await?;
        let provider = connect_to_gateway(&config.gateway_url).await?;
        let kms_client = KmsClient::connect(&config, response_sender).await?;

        let event_picker =
            DbEventPicker::connect(db_pool.clone(), config.events_batch_size).await?;

        let s3_service = S3Service::new(&config, provider);
        let decryption_processor = DecryptionProcessor::new(&config, s3_service);
        let event_processor =
            EventProcessor::new(kms_client, decryption_processor, db_pool.clone());
        let event_processeor_service = EventProcessorService::new(event_picker, event_processor);

        let response_publisher = KmsResponsePublisher::new(db_pool, response_receiver);

        Ok(Self::new(event_processeor_service, response_publisher))
    }
}
