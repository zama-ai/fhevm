use crate::core::event_processor::{
    KmsClient,
    context::ContextManager,
    decryption::{DecryptionProcessor, UserDecryptionExtraData},
    kms::KMSGenerationProcessor,
};
use alloy::providers::Provider;
use anyhow::anyhow;
use connector_utils::types::{
    KmsGrpcRequest, KmsGrpcResponse, KmsResponseKind, ProtocolEvent, ProtocolEventKind,
};
use sqlx::{Pool, Postgres};
use thiserror::Error;
use tonic::Code;
use tracing::{error, info};
use user_decryption_signature::Erc1271Error;

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
pub struct DbEventProcessor<GP: Provider, HP: Provider, C> {
    /// The GRPC client used to communicate with the KMS Core.
    kms_client: KmsClient,

    /// The entity used to process decryption requests.
    decryption_processor: DecryptionProcessor<GP, HP, C>,

    /// The entity used to process key management requests.
    kms_generation_processor: KMSGenerationProcessor,

    /// The maximum number of decryption attempts.
    max_decryption_attempts: u16,

    /// The DB connection pool used to update the events `status` field on error.
    db_pool: Pool<Postgres>,
}

impl<GP: Provider, HP: Provider, C: ContextManager> EventProcessor for DbEventProcessor<GP, HP, C> {
    type Event = ProtocolEvent;

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
                ProtocolEventKind::PrssInit(_) | ProtocolEventKind::KeyReshareSameSet(_),
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
                ProtocolEventKind::PublicDecryption(_)
                | ProtocolEventKind::UserDecryption(_)
                | ProtocolEventKind::UserDecryptionV2(_),
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

/// ERC-1271 (RFC-012) signature errors map onto `ProcessingError` so callers can use the `?`
/// operator. Missing code at an EOA is terminal, but smart-account validation can depend on
/// mutable wallet state, so negative ERC-1271 results are retried through the existing attempt
/// and validity-window limits.
impl From<Erc1271Error> for ProcessingError {
    fn from(err: Erc1271Error) -> Self {
        match err {
            Erc1271Error::EoaMismatchNoCode(_) | Erc1271Error::EmptySigOnEoa(_) => {
                Self::Irrecoverable(anyhow::Error::new(err))
            }
            Erc1271Error::Transport(_)
            | Erc1271Error::WrongMagic(..)
            | Erc1271Error::Rejected(..) => Self::Recoverable(anyhow::Error::new(err)),
        }
    }
}

impl<GP: Provider, HP: Provider, C: ContextManager> DbEventProcessor<GP, HP, C> {
    pub fn new(
        kms_client: KmsClient,
        decryption_processor: DecryptionProcessor<GP, HP, C>,
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
        event: &mut ProtocolEvent,
    ) -> Result<KmsGrpcRequest, ProcessingError> {
        let request = match &event.kind {
            ProtocolEventKind::PublicDecryption(req) => {
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
                    .await?
            }
            ProtocolEventKind::UserDecryption(req) => {
                // No need to check decryption is done for user decrypt, as MPC parties don't
                // communicate between each other for user decrypt

                let tx_hash = event.tx_hash.ok_or_else(|| {
                    ProcessingError::Irrecoverable(anyhow!(
                        "No `tx_hash` found for user decryption. Cannot perform ACL check."
                    ))
                })?;
                let calldata = self.decryption_processor.fetch_calldata(tx_hash).await?;
                self.decryption_processor
                    .check_ciphertexts_allowed_for_user_decryption(
                        calldata,
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
                    .await?
            }
            ProtocolEventKind::UserDecryptionV2(req) => {
                // The RFC016 event carries the full payload, so unlike the legacy path we don't
                // need to re-fetch the transaction calldata.
                self.decryption_processor
                    .check_user_decryption_request_v2(req)
                    .await?;
                let payload = &req.payload;
                self.decryption_processor
                    .prepare_decryption_request(
                        req.decryptionId,
                        &req.snsCtMaterials,
                        &payload.extraData,
                        Some(UserDecryptionExtraData::new(
                            payload.userAddress,
                            payload.publicKey.clone(),
                        )),
                    )
                    .await?
            }
            ProtocolEventKind::PrepKeygen(req) => self
                .kms_generation_processor
                .prepare_prep_keygen_request(req),
            ProtocolEventKind::Keygen(req) => {
                self.kms_generation_processor.prepare_keygen_request(req)
            }
            ProtocolEventKind::Crsgen(req) => {
                self.kms_generation_processor.prepare_crsgen_request(req)
            }
            ProtocolEventKind::PrssInit(id) => {
                self.kms_generation_processor.prepare_prss_init_request(*id)
            }
            ProtocolEventKind::KeyReshareSameSet(req) => self
                .kms_generation_processor
                .prepare_initiate_resharing_request(req),
        };
        Ok(request)
    }

    /// Core event processing logic function.
    async fn inner_process(
        &mut self,
        event: &mut ProtocolEvent,
    ) -> Result<Option<KmsResponseKind>, ProcessingError> {
        let request = self
            .prepare_request(event)
            .await
            .inspect_err(|_| event.error_counter += 1)?;

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
