use crate::core::event_processor::{
    KmsClient,
    decryption::{DecryptionProcessor, UserDecryptionExtraData},
    kms::KmsManagementProcessor,
};
use alloy::providers::Provider;
use anyhow::anyhow;
use connector_utils::types::{GatewayEvent, KmsGrpcRequest, KmsResponse};
use sqlx::{Pool, Postgres};
use thiserror::Error;
use tonic::Code;
use tracing::info;

/// Interface used to process Gateway's events.
pub trait EventProcessor: Send {
    type Event: Send;

    fn process(
        &mut self,
        event: &Self::Event,
    ) -> impl Future<Output = Result<KmsResponse, ProcessingError>> + Send;
}

/// Struct that processes Gateway's events coming from a `Postgres` database.
#[derive(Clone)]
pub struct DbEventProcessor<P: Provider> {
    /// The GRPC client used to communicate with the KMS Core.
    kms_client: KmsClient,

    /// The entity used to process decryption requests.
    decryption_processor: DecryptionProcessor<P>,

    /// The entity used to process key management requests.
    kms_management_processor: KmsManagementProcessor,

    /// The DB connection pool used to reset events `under_process` field on error.
    db_pool: Pool<Postgres>,
}

impl<P: Provider> EventProcessor for DbEventProcessor<P> {
    type Event = GatewayEvent;

    #[tracing::instrument(skip_all)]
    async fn process(&mut self, event: &Self::Event) -> Result<KmsResponse, ProcessingError> {
        info!("Starting to process {:?}...", event);
        match self.inner_process(event).await {
            Ok(response) => {
                info!("Event successfully processed!");
                Ok(response)
            }
            Err(ProcessingError::Recoverable(e)) => {
                event.mark_as_pending(&self.db_pool).await;
                Err(ProcessingError::Recoverable(e))
            }
            Err(ProcessingError::Irrecoverable(e)) => {
                event.delete_from_db(&self.db_pool).await;
                Err(ProcessingError::Irrecoverable(e))
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum ProcessingError {
    #[error("Processing failed with irrecoverable error : {0}")]
    Irrecoverable(anyhow::Error),
    #[error("Processing failed: {0}")]
    Recoverable(anyhow::Error),
}

impl<P: Provider> DbEventProcessor<P> {
    pub fn new(
        kms_client: KmsClient,
        decryption_processor: DecryptionProcessor<P>,
        kms_management_processor: KmsManagementProcessor,
        db_pool: Pool<Postgres>,
    ) -> Self {
        Self {
            kms_client,
            decryption_processor,
            kms_management_processor,
            db_pool,
        }
    }

    /// Prepares the GRPC request associated to the received `event`.
    #[tracing::instrument(skip_all)]
    async fn prepare_request(
        &self,
        event: GatewayEvent,
    ) -> Result<KmsGrpcRequest, ProcessingError> {
        match event {
            GatewayEvent::PublicDecryption(req) => {
                self.decryption_processor
                    .prepare_decryption_request(
                        req.decryptionId,
                        req.snsCtMaterials,
                        req.extraData.into(),
                        None,
                    )
                    .await
            }
            GatewayEvent::UserDecryption(req) => {
                self.decryption_processor
                    .prepare_decryption_request(
                        req.decryptionId,
                        req.snsCtMaterials,
                        req.extraData.into(),
                        Some(UserDecryptionExtraData::new(req.userAddress, req.publicKey)),
                    )
                    .await
            }
            GatewayEvent::PrepKeygen(req) => {
                self.kms_management_processor
                    .prepare_prep_keygen_request(req)
                    .await
            }
            GatewayEvent::Keygen(req) => {
                self.kms_management_processor
                    .prepare_keygen_request(req)
                    .await
            }
            _ => unimplemented!(),
        }
        .map_err(ProcessingError::Recoverable)
    }

    /// Core event processing logic function.
    async fn inner_process(
        &mut self,
        event: &GatewayEvent,
    ) -> Result<KmsResponse, ProcessingError> {
        let request = self.prepare_request(event.clone()).await?;
        let grpc_response = self.kms_client.send_request(request).await?;
        KmsResponse::process(grpc_response).map_err(ProcessingError::Irrecoverable)
    }
}

impl ProcessingError {
    /// Converts GRPC status of a request sent to the KMS into a `ProcessingError`.
    pub fn from_request_status(value: tonic::Status) -> Self {
        let anyhow_error = anyhow!("KMS GRPC error: {value}");
        match value.code() {
            Code::ResourceExhausted => Self::Recoverable(anyhow_error),
            _ => Self::Irrecoverable(anyhow_error),
        }
    }

    /// Converts GRPC status of the polling of a KMS Response into a `ProcessingError`.
    pub fn from_response_status(value: tonic::Status) -> Self {
        let anyhow_error = anyhow!("KMS GRPC error: {value}");
        match value.code() {
            Code::NotFound | Code::Unavailable | Code::ResourceExhausted => {
                Self::Recoverable(anyhow_error)
            }
            _ => Self::Irrecoverable(anyhow_error),
        }
    }
}
