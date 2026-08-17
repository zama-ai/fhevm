use crate::core::event_processor::{
    KmsClient, KmsPollTarget, ProcessingError, RequestCheckError,
    context::ContextManager,
    decryption::{DecryptionProcessor, UserDecryptionExtraData},
    kms::KMSGenerationProcessor,
    protocol_config::ProtocolConfigProcessor,
};
use alloy::{primitives::B256, providers::Provider};
use anyhow::anyhow;
use connector_utils::types::{
    KmsGrpcRequest, KmsGrpcResponse, KmsResponseKind, KmsSendResponse, ProtocolEvent,
    ProtocolEventKind, db::invalidate_kms_epoch, extra_data::parse_extra_data, u256_to_request_id,
};
use kms_grpc::kms::v1::{DestroyMpcContextRequest, DestroyMpcEpochRequest};
use sqlx::{Pool, Postgres};
use tracing::{error, info, warn};

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

    /// The entity used to validate the KMS context referenced by a request.
    context_manager: C,

    /// The entity used to process decryption requests.
    decryption_processor: DecryptionProcessor<GP, HP>,

    /// The entity used to process key management requests.
    kms_generation_processor: KMSGenerationProcessor,

    /// The entity used to build `ProtocolConfig` event requests (context/epoch lifecycle).
    protocol_config_processor: ProtocolConfigProcessor<HP>,

    /// The maximum number of decryption attempts.
    max_decryption_attempts: u16,

    /// The DB connection pool used to update the events `status` field on error.
    db_pool: Pool<Postgres>,
}

impl<GP, HP, C> EventProcessor for DbEventProcessor<GP, HP, C>
where
    GP: Provider + Clone + 'static,
    HP: Provider,
    C: ContextManager,
{
    type Event = ProtocolEvent;

    #[tracing::instrument(skip_all)]
    async fn process(&mut self, event: &mut Self::Event) -> Option<KmsResponseKind> {
        info!("Starting to process {:?}...", event.kind);
        match (self.inner_process(event).await, &event.kind) {
            (Ok(response), _) => {
                info!("Event successfully processed!");
                response
            }
            (Err(ProcessingError::Irrecoverable(e)), _) => {
                error!("{}", ProcessingError::Irrecoverable(e));
                if let Err(e) = event.mark_as_failed(&self.db_pool).await {
                    warn!("{e}");
                }
                None
            }
            (Err(ProcessingError::Aborted), _) => {
                warn!("{}", ProcessingError::Aborted);
                if let Err(e) = event.mark_as_aborted(&self.db_pool).await {
                    warn!("{e}");
                }
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
                if let Err(e) = event.mark_as_failed(&self.db_pool).await {
                    warn!("{e}");
                }
                None
            }
            (Err(ProcessingError::Recoverable(e)), _) => {
                error!("{}", ProcessingError::Recoverable(e));
                if let Err(e) = event.mark_as_pending(&self.db_pool).await {
                    warn!("{e}");
                }
                None
            }
        }
    }
}

impl<GP: Provider + Clone + 'static, HP: Provider, C: ContextManager> DbEventProcessor<GP, HP, C> {
    pub fn new(
        kms_client: KmsClient,
        context_manager: C,
        decryption_processor: DecryptionProcessor<GP, HP>,
        kms_generation_processor: KMSGenerationProcessor,
        protocol_config_processor: ProtocolConfigProcessor<HP>,
        max_decryption_attempts: u16,
        db_pool: Pool<Postgres>,
    ) -> Self {
        Self {
            kms_client,
            context_manager,
            decryption_processor,
            kms_generation_processor,
            protocol_config_processor,
            max_decryption_attempts,
            db_pool,
        }
    }

    /// Checks the KMS context referenced by the request's `extra_data` is valid.
    async fn check_context(&self, extra_data: &[u8]) -> Result<(), ProcessingError> {
        let parsed_extra_data =
            parse_extra_data(extra_data).map_err(ProcessingError::Irrecoverable)?;
        self.context_manager
            .validate_context(&parsed_extra_data)
            .await
            .map_err(RequestCheckError::record)
    }

    /// Checks the request associated to the received `event` is valid to process.
    ///
    /// Checks living here validate live state and are re-run on every processing attempt,
    /// including poll-only retries of an already-sent request.
    #[tracing::instrument(skip_all)]
    async fn check_request(&self, event: &ProtocolEvent) -> Result<(), ProcessingError> {
        match &event.kind {
            ProtocolEventKind::PublicDecryption(req) => {
                tokio::try_join!(
                    biased;
                    async {
                        self.decryption_processor
                            .check_ciphertexts_allowed_for_public_decryption(&req.ctHandles)
                            .await
                            .map_err(RequestCheckError::record)
                    },
                    self.check_context(&req.extraData),
                )?;
                Ok(())
            }
            ProtocolEventKind::UserDecryption(req) => {
                let tx_hash = event.tx_hash.ok_or_else(|| {
                    ProcessingError::Irrecoverable(anyhow!(
                        "No `tx_hash` found for user decryption. Cannot perform ACL check."
                    ))
                })?;
                tokio::try_join!(
                    biased;
                    async {
                        let calldata = self.decryption_processor.fetch_calldata(tx_hash).await?;
                        self.decryption_processor
                            .check_ciphertexts_allowed_for_user_decryption(
                                calldata,
                                &req.ctHandles,
                                req.userAddress,
                            )
                            .await
                            .map_err(RequestCheckError::record)
                    },
                    self.check_context(&req.extraData),
                )?;
                Ok(())
            }
            ProtocolEventKind::UserDecryptionV2(req) => {
                tokio::try_join!(
                    biased;
                    async {
                        self.decryption_processor
                            .check_user_decryption_request_v2(req)
                            .await
                            .map_err(RequestCheckError::record)
                    },
                    self.check_context(&req.payload.extraData),
                )?;
                Ok(())
            }
            ProtocolEventKind::PrepKeygen(req) => self.check_context(&req.extraData).await,
            ProtocolEventKind::Keygen(req) => self.check_context(&req.extraData).await,
            ProtocolEventKind::Crsgen(req) => self.check_context(&req.extraData).await,
            // No live-state check applies to the remaining events. Note that the `NewKmsContext`
            // anchor-hash verification stays in `prepare_grpc_request`: it validates immutable
            // historical on-chain data, so re-running it on retries could not change its outcome.
            ProtocolEventKind::AbortKeygen(_)
            | ProtocolEventKind::AbortCrsgen(_)
            | ProtocolEventKind::NewKmsContext(_)
            | ProtocolEventKind::NewKmsEpoch(_)
            | ProtocolEventKind::KmsContextDestroyed(_)
            | ProtocolEventKind::KmsEpochDestroyed(_) => Ok(()),
        }
    }

    /// Prepares the GRPC request associated to the received `event`.
    ///
    /// Checks performed during preparation (e.g. ciphertext attestation consensus) validate the
    /// payload being sent, so running them is only meaningful at send time.
    /// They are deliberately skipped when retrying an already-sent request: no new payload is
    /// sent, and the KMS Core keeps working on the one verified here at send time.
    #[tracing::instrument(skip_all)]
    async fn prepare_grpc_request(
        &self,
        event: &ProtocolEvent,
    ) -> Result<KmsGrpcRequest, ProcessingError> {
        match &event.kind {
            ProtocolEventKind::PublicDecryption(req) => {
                self.decryption_processor
                    .prepare_decryption_request(
                        req.decryptionId,
                        &req.ctHandles,
                        &req.extraData,
                        None,
                    )
                    .await
            }
            ProtocolEventKind::UserDecryption(req) => {
                self.decryption_processor
                    .prepare_decryption_request(
                        req.decryptionId,
                        &req.ctHandles,
                        &req.extraData,
                        Some(UserDecryptionExtraData::new(
                            req.userAddress,
                            req.publicKey.clone(),
                        )),
                    )
                    .await
            }
            ProtocolEventKind::UserDecryptionV2(req) => {
                let payload = &req.payload;
                let handles: Vec<B256> = req.handles.iter().map(|h| h.handle).collect();
                self.decryption_processor
                    .prepare_decryption_request(
                        req.decryptionId,
                        &handles,
                        &payload.extraData,
                        Some(UserDecryptionExtraData::new(
                            payload.userAddress,
                            payload.publicKey.clone(),
                        )),
                    )
                    .await
            }
            ProtocolEventKind::PrepKeygen(req) => {
                self.kms_generation_processor
                    .prepare_prep_keygen_request(req)
                    .await
            }
            ProtocolEventKind::Keygen(req) => {
                self.kms_generation_processor
                    .prepare_keygen_request(req)
                    .await
            }
            ProtocolEventKind::Crsgen(req) => {
                self.kms_generation_processor
                    .prepare_crsgen_request(req)
                    .await
            }
            ProtocolEventKind::AbortKeygen(req) => Ok(KmsGrpcRequest::AbortKeygen(
                u256_to_request_id(req.prepKeygenId),
            )),
            ProtocolEventKind::AbortCrsgen(req) => {
                Ok(KmsGrpcRequest::AbortCrsgen(u256_to_request_id(req.crsId)))
            }
            ProtocolEventKind::NewKmsContext(req) => {
                self.protocol_config_processor
                    .prepare_new_kms_context_request(req)
                    .await
            }
            ProtocolEventKind::NewKmsEpoch(req) => {
                self.protocol_config_processor
                    .prepare_new_kms_epoch_request(req)
                    .await
            }
            ProtocolEventKind::KmsContextDestroyed(req) => Ok(KmsGrpcRequest::DestroyMpcContext(
                DestroyMpcContextRequest {
                    context_id: Some(u256_to_request_id(req.kmsContextId)),
                },
            )),
            ProtocolEventKind::KmsEpochDestroyed(req) => {
                Ok(KmsGrpcRequest::DestroyMpcEpoch(DestroyMpcEpochRequest {
                    epoch_id: Some(u256_to_request_id(req.epochId)),
                }))
            }
        }
    }

    /// Core event processing logic function.
    async fn inner_process(
        &mut self,
        event: &mut ProtocolEvent,
    ) -> Result<Option<KmsResponseKind>, ProcessingError> {
        self.check_request(event)
            .await
            .inspect_err(|_| event.error_counter += 1)?;

        if !event.already_sent {
            let request = self
                .prepare_grpc_request(event)
                .await
                .inspect_err(|_| event.error_counter += 1)?;

            let (error_count, result) = self.kms_client.send_request(&request).await;
            event.error_counter += error_count;
            let send_response = result?;
            event.already_sent = true;

            // Invalidate epochs associated to destroyed context returned by the KMS Core.
            // A failure must not prevent the event from completing, as retrying would result in an
            // `NotFound` from KMS Core with no epoch IDs.
            if let KmsSendResponse::DestroyedEpochs(epoch_ids) = send_response {
                for epoch_id in epoch_ids {
                    if let Err(e) = invalidate_kms_epoch(&self.db_pool, epoch_id).await {
                        warn!("Failed to invalidate destroyed KMS epoch #{epoch_id}: {e}");
                    }
                }
            }
        }

        let (error_count, grpc_result) = self
            .kms_client
            .poll_result(KmsPollTarget::from(&event.kind))
            .await;
        event.error_counter += error_count;
        let grpc_response = grpc_result?;

        if let KmsGrpcResponse::NoResponseExpected = &grpc_response {
            if let Err(e) = event.mark_as_completed(&self.db_pool).await {
                warn!("{e}");
            }
            return Ok(None);
        }

        let processed_response =
            KmsResponseKind::process(grpc_response).map_err(ProcessingError::Irrecoverable)?;
        Ok(Some(processed_response))
    }
}
