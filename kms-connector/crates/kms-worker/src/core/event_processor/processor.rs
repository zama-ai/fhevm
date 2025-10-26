use crate::core::event_processor::{
    KmsClient,
    decryption::{DecryptionProcessor, UserDecryptionExtraData},
    kms::KMSGenerationProcessor,
};
use alloy::providers::Provider;
use anyhow::anyhow;
use connector_utils::types::{
    GatewayEvent, GatewayEventKind, KmsGrpcRequest, KmsGrpcResponse, KmsResponseKind,
};
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
    ) -> impl Future<Output = Result<Option<KmsResponseKind>, ProcessingError>> + Send;
}

/// Struct that processes Gateway's events coming from a `Postgres` database.
#[derive(Clone)]
pub struct DbEventProcessor<P: Provider> {
    /// The GRPC client used to communicate with the KMS Core.
    kms_client: KmsClient,

    /// The entity used to process decryption requests.
    decryption_processor: DecryptionProcessor<P>,

    /// The entity used to process key management requests.
    kms_generation_processor: KMSGenerationProcessor,

    /// The DB connection pool used to reset events `under_process` field on error.
    db_pool: Pool<Postgres>,
}

impl<P: Provider> EventProcessor for DbEventProcessor<P> {
    type Event = GatewayEvent;

    #[tracing::instrument(skip_all)]
    async fn process(
        &mut self,
        event: &Self::Event,
    ) -> Result<Option<KmsResponseKind>, ProcessingError> {
        info!("Starting to process {:?}...", event.kind);
        match (self.inner_process(event).await, &event.kind) {
            (Ok(response), _) => {
                info!("Event successfully processed!");
                Ok(response)
            }
            (Err(ProcessingError::Irrecoverable(e)), _)
            | (
                Err(ProcessingError::Recoverable(e)),
                GatewayEventKind::PrssInit(_) | GatewayEventKind::KeyReshareSameSet(_),
            ) => {
                event.delete_from_db(&self.db_pool).await;
                Err(ProcessingError::Irrecoverable(e))
            }
            (Err(ProcessingError::Recoverable(e)), _) => {
                event.mark_as_pending(&self.db_pool).await;
                Err(ProcessingError::Recoverable(e))
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
        kms_generation_processor: KMSGenerationProcessor,
        db_pool: Pool<Postgres>,
    ) -> Self {
        Self {
            kms_client,
            decryption_processor,
            kms_generation_processor,
            db_pool,
        }
    }

    /// Prepares the GRPC request associated to the received `event`.
    #[tracing::instrument(skip_all)]
    async fn prepare_request(
        &self,
        event: GatewayEvent,
    ) -> Result<KmsGrpcRequest, ProcessingError> {
        match event.kind {
            GatewayEventKind::PublicDecryption(req) => {
                self.decryption_processor
                    .prepare_decryption_request(
                        req.decryptionId,
                        req.snsCtMaterials,
                        req.extraData.into(),
                        None,
                    )
                    .await
            }
            GatewayEventKind::UserDecryption(req) => {
                self.decryption_processor
                    .prepare_decryption_request(
                        req.decryptionId,
                        req.snsCtMaterials,
                        req.extraData.into(),
                        Some(UserDecryptionExtraData::new(req.userAddress, req.publicKey)),
                    )
                    .await
            }
            GatewayEventKind::PrepKeygen(req) => self
                .kms_generation_processor
                .prepare_prep_keygen_request(req),
            GatewayEventKind::Keygen(req) => {
                self.kms_generation_processor.prepare_keygen_request(req)
            }
            GatewayEventKind::Crsgen(req) => {
                self.kms_generation_processor.prepare_crsgen_request(req)
            }
            GatewayEventKind::PrssInit(id) => {
                Ok(self.kms_generation_processor.prepare_prss_init_request(id))
            }
            GatewayEventKind::KeyReshareSameSet(req) => self
                .kms_generation_processor
                .prepare_initiate_resharing_request(req),
        }
        .map_err(ProcessingError::Recoverable)
    }

    /// Core event processing logic function.
    async fn inner_process(
        &mut self,
        event: &GatewayEvent,
    ) -> Result<Option<KmsResponseKind>, ProcessingError> {
        let request = self.prepare_request(event.clone()).await?;
        let grpc_response = self.kms_client.send_request(request).await?;

        if let KmsGrpcResponse::NoResponseExpected = &grpc_response {
            event.delete_from_db(&self.db_pool).await;
            return Ok(None);
        }

        let processed_response =
            KmsResponseKind::process(grpc_response).map_err(ProcessingError::Irrecoverable)?;
        Ok(Some(processed_response))
    }
}

impl ProcessingError {
    /// Converts GRPC status of the polling of a KMS Response into a `ProcessingError`.
    pub fn from_response_status(value: tonic::Status) -> Self {
        let anyhow_error = anyhow!("KMS GRPC error: {value}");
        match value.code() {
            Code::DeadlineExceeded | Code::Unavailable | Code::ResourceExhausted => {
                Self::Recoverable(anyhow_error)
            }
            _ => Self::Irrecoverable(anyhow_error),
        }
    }
}
