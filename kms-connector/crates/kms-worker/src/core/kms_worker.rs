use crate::core::{
    KmsResponsePublisher,
    config::Config,
    event_picker::{DbEventPicker, EventPicker},
    event_processor::{DbEventProcessor, DecryptionProcessor, EventProcessor, KmsClient},
    event_remover::{DbEventRemover, EventRemover},
    kms_response_publisher::DbKmsResponsePublisher,
    s3::S3Service,
};
use connector_utils::conn::{GatewayProvider, connect_to_db, connect_to_gateway};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

pub struct KmsWorker<E, Proc, Publ, R> {
    event_picker: E,
    event_processor: Proc,
    response_publisher: Publ,
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

    pub async fn start(self, cancel_token: CancellationToken) {
        info!("Starting KmsWorker");
        tokio::select! {
            _ = cancel_token.cancelled() => info!("Stopping KmsWorker"),
            _ = self.run() => (),
        }
    }

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

    fn spawn_event_processing_task(&self, event: E::Event) {
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
