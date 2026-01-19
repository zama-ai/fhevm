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
use tracing::{error, info};

/// Interface used to process Gateway's events.
pub trait EventProcessor: Send {
    type Event: Send;

    fn process(
        &mut self,
        event: &mut Self::Event,
    ) -> impl Future<Output = Option<KmsResponseKind>> + Send;
}

/// Struct that processes Gateway's events coming from a `Postgres` database.
#[derive(Clone)]
pub struct DbEventProcessor<GP: Provider, HP: Provider> {
    /// The GRPC client used to communicate with the KMS Core.
    kms_client: KmsClient,

    /// The entity used to process decryption requests.
    decryption_processor: DecryptionProcessor<GP, HP>,

    /// The entity used to process key management requests.
    kms_generation_processor: KMSGenerationProcessor,

    /// The maximum number of decryption attempts.
    max_decryption_attempts: u16,

    /// The DB connection pool used to update the events `status` field on error.
    db_pool: Pool<Postgres>,
}

impl<GP: Provider, HP: Provider> EventProcessor for DbEventProcessor<GP, HP> {
    type Event = GatewayEvent;

    #[tracing::instrument(skip_all)]
    async fn process(&mut self, event: &mut Self::Event) -> Option<KmsResponseKind> {
        info!("Starting to process {:?}...", event.kind);
        match (self.inner_process(event).await, &event.kind) {
            (Ok(response), _) => {
                info!("Event successfully processed!");
                response
            }
            (Err(ProcessingError::Irrecoverable(e)), _)
            | (
                Err(ProcessingError::Recoverable(e)),
                // Consider all errors as irrecoverable for PrssInit and KeyReshareSameSet, as KMS does
                // not provide any response for these operations
                GatewayEventKind::PrssInit(_) | GatewayEventKind::KeyReshareSameSet(_),
            ) => {
                error!("{}", ProcessingError::Irrecoverable(e));
                event.mark_as_failed(&self.db_pool).await;
                None
            }
            // For now, we only check the error counter for public and user decryptions as they are
            // the most frequent operations, and we want to avoid infinite retry loop for them.
            // For key management operations, as they are not frequent at all, we currently rely on
            // a manual cleanup of the DB in such case. We want to avoid to "accidentally" remove a
            // key management operation at all cost.
            (
                Err(ProcessingError::Recoverable(e)),
                GatewayEventKind::PublicDecryption(_) | GatewayEventKind::UserDecryption(_),
            ) if event.error_counter as u16 >= self.max_decryption_attempts => {
                error!(
                    "{}. Maximum number of decryption attempts reached: {}",
                    ProcessingError::Irrecoverable(e),
                    event.error_counter
                );
                event.mark_as_failed(&self.db_pool).await;
                None
            }
            (Err(ProcessingError::Recoverable(e)), _) => {
                error!("{}", ProcessingError::Recoverable(e));
                event.mark_as_pending(&self.db_pool).await;
                None
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

impl<GP: Provider, HP: Provider> DbEventProcessor<GP, HP> {
    pub fn new(
        kms_client: KmsClient,
        decryption_processor: DecryptionProcessor<GP, HP>,
        kms_generation_processor: KMSGenerationProcessor,
        max_decryption_attempts: u16,
        db_pool: Pool<Postgres>,
    ) -> Self {
        Self {
            kms_client,
            decryption_processor,
            kms_generation_processor,
            max_decryption_attempts,
            db_pool,
        }
    }

    /// Prepares the GRPC request associated to the received `event`.
    #[tracing::instrument(skip_all)]
    async fn prepare_request(
        &self,
        event: &mut GatewayEvent,
    ) -> Result<KmsGrpcRequest, ProcessingError> {
        match &event.kind {
            GatewayEventKind::PublicDecryption(req) => {
                self.decryption_processor
                    .check_decryption_not_already_done(req.decryptionId)
                    .await?;
                self.decryption_processor
                    .check_ciphertexts_allowed_for_public_decryption(&req.snsCtMaterials)
                    .await?;

                self.decryption_processor
                    .prepare_decryption_request(
                        req.decryptionId,
                        &req.snsCtMaterials,
                        &req.extraData,
                        None,
                    )
                    .await
            }
            GatewayEventKind::UserDecryption(req) => {
                // No need to check decryption is done for user decrypt, as MPC parties don't
                // communicate between each other for user decrypt
                self.decryption_processor
                    .check_ciphertexts_allowed_for_user_decryption(
                        &req.snsCtMaterials,
                        req.userAddress,
                    )
                    .await?;
                self.decryption_processor
                    .prepare_decryption_request(
                        req.decryptionId,
                        &req.snsCtMaterials,
                        &req.extraData,
                        Some(UserDecryptionExtraData::new(
                            req.userAddress,
                            req.publicKey.clone(),
                        )),
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
                Ok(self.kms_generation_processor.prepare_prss_init_request(*id))
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
        event: &mut GatewayEvent,
    ) -> Result<Option<KmsResponseKind>, ProcessingError> {
        let request = self.prepare_request(event).await?;

        if !event.already_sent {
            let (error_count, result) = self.kms_client.send_request(&request).await;
            event.error_counter += error_count;
            result?;
            event.already_sent = true;
        }

        let (error_count, grpc_result) = self.kms_client.poll_result(request).await;
        event.error_counter += error_count;
        let grpc_response = grpc_result?;

        if let KmsGrpcResponse::NoResponseExpected = &grpc_response {
            event.mark_as_completed(&self.db_pool).await;
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
