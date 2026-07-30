use crate::{
    core::{Config, event_processor::ProcessingError},
    monitoring::metrics::{
        GRPC_REQUEST_SENT_COUNTER, GRPC_REQUEST_SENT_ERRORS, GRPC_RESPONSE_POLLED_COUNTER,
        GRPC_RESPONSE_POLLED_ERRORS,
    },
};
use alloy::primitives::U256;
use anyhow::anyhow;
use connector_utils::{
    conn::{CONNECTION_RETRY_DELAY, CONNECTION_RETRY_NUMBER},
    types::{
        KmsGrpcRequest, KmsGrpcResponse, SendResponse, db::EventType, request_id_to_u256,
        u256_to_u32,
    },
};
use kms_grpc::{
    kms::v1::{
        CrsGenRequest, DestroyMpcContextRequest, DestroyMpcContextResponse, DestroyMpcEpochRequest,
        KeyGenPreprocRequest, KeyGenRequest, NewMpcContextRequest, NewMpcEpochRequest,
        PublicDecryptionRequest, RequestId, UserDecryptionRequest,
    },
    kms_service::v1::core_service_endpoint_client::CoreServiceEndpointClient,
};
use tonic::{Code, Request, Response, Status, transport::Channel};
use tracing::{info, warn};

/// The struct handling the communication with the KMS Core.
#[derive(Clone, Debug)]
pub struct KmsClient {
    /// The internal KMS Core clients from the `kms_grpc` crate.
    inners: Vec<CoreServiceEndpointClient<Channel>>,

    /// Number of retries for GRPC requests sent to the KMS Core.
    grpc_request_retries: u8,
}

impl KmsClient {
    pub fn new(channels: Vec<Channel>, grpc_request_retries: u8) -> Self {
        let inners = channels
            .into_iter()
            .map(CoreServiceEndpointClient::new)
            .collect();

        Self {
            inners,
            grpc_request_retries,
        }
    }

    /// Connects to all the KMS Core shards.
    pub async fn connect(config: &Config) -> anyhow::Result<Self> {
        let mut channels = vec![];
        for (i, kms_shard_endpoint) in config.kms_core_endpoints.iter().enumerate() {
            channels.push(KmsClient::connect_single_shard(i, kms_shard_endpoint).await?);
        }

        Ok(Self::new(channels, config.grpc_request_retries))
    }

    async fn connect_single_shard(shard_id: usize, endpoint: &str) -> anyhow::Result<Channel> {
        let grpc_endpoint = Channel::from_shared(endpoint.to_string())
            .map_err(|e| anyhow!("Invalid KMS Core shard endpoint #{shard_id} {endpoint}: {e}"))?;

        for i in 1..=CONNECTION_RETRY_NUMBER {
            info!(
                "Attempting connection to KMS Core shard #{shard_id}... ({i}/{CONNECTION_RETRY_NUMBER})"
            );

            match grpc_endpoint.connect().await {
                Ok(channel) => {
                    info!("Connected to KMS Core shard #{shard_id} at {endpoint}");
                    return Ok(channel);
                }
                Err(e) => {
                    warn!("KMS Core shard #{shard_id} connection attempt #{i} failed: {e}")
                }
            }

            if i != CONNECTION_RETRY_NUMBER {
                tokio::time::sleep(CONNECTION_RETRY_DELAY).await;
            }
        }

        Err(anyhow!(
            "Could not connect to KMS Core shard #{shard_id} at {endpoint}"
        ))
    }

    /// Sends the GRPC request to the KMS Core.
    ///
    /// Returns the number of errors encountered while sending the request, and the result of the
    /// request.
    #[tracing::instrument(skip_all)]
    pub async fn send_request(
        &self,
        request: &KmsGrpcRequest,
    ) -> (i16, Result<SendResponse, ProcessingError>) {
        match request {
            KmsGrpcRequest::PublicDecryption(req) => self.request_public_decryption(req).await,
            KmsGrpcRequest::UserDecryption(req) => self.request_user_decryption(req).await,
            KmsGrpcRequest::PrepKeygen(req) => self.request_prep_keygen(req).await,
            KmsGrpcRequest::Keygen(req) => self.request_keygen(req).await,
            KmsGrpcRequest::Crsgen(req) => self.request_crsgen(req).await,
            KmsGrpcRequest::AbortKeygen(req) => self.request_abort_keygen(req).await,
            KmsGrpcRequest::AbortCrsgen(req) => self.request_abort_crsgen(req).await,
            KmsGrpcRequest::NewMpcContext { old, new } => {
                // Create the old context in case it doesn't exist, then the new context.
                match self.request_new_mpc_context(old).await {
                    // Both the `AlreadyExists` and `Ok(Empty)` cases are caught here.
                    (_, Ok(_)) => self.request_new_mpc_context(new).await,
                    error => error,
                }
            }
            KmsGrpcRequest::NewMpcEpoch(req) => self.request_new_mpc_epoch(req).await,
            KmsGrpcRequest::DestroyMpcContext(req) => self.request_destroy_mpc_context(req).await,
            KmsGrpcRequest::DestroyMpcEpoch(req) => self.request_destroy_mpc_epoch(req).await,
        }
    }

    /// Polls the GRPC result from the KMS Core.
    ///
    /// Returns the number of errors encountered while polling the result, and the result itself.
    #[tracing::instrument(skip_all)]
    pub async fn poll_result(
        &self,
        request: KmsGrpcRequest,
    ) -> (i16, Result<KmsGrpcResponse, ProcessingError>) {
        match request {
            KmsGrpcRequest::PublicDecryption(req) => self.poll_public_decryption_result(req).await,
            KmsGrpcRequest::UserDecryption(req) => self.poll_user_decryption_result(req).await,
            KmsGrpcRequest::PrepKeygen(req) => self.poll_prep_keygen_result(req).await,
            KmsGrpcRequest::Keygen(req) => self.poll_keygen_result(req).await,
            KmsGrpcRequest::Crsgen(req) => self.poll_crsgen_result(req).await,
            // Abort has no result-polling endpoint on the Core: the send-side ack is the only
            // signal. The original keygen/crsgen request is separately retired when its own result
            // poll returns `Code::Aborted`.
            KmsGrpcRequest::AbortKeygen(_) | KmsGrpcRequest::AbortCrsgen(_) => {
                (0, Ok(KmsGrpcResponse::NoResponseExpected))
            }
            // `NewMpcContext` has no result-polling endpoint: the Core's send-side ack is the
            // only signal we get. The caller has already observed a successful send; we emit a
            // synthetic response so the publisher can write the row. `context_id` is sourced
            // from the request's inner `MpcContext` to avoid duplicating it on the variant.
            KmsGrpcRequest::NewMpcContext { new, .. } => match extract_new_mpc_context_id(&new) {
                Ok(context_id) => (0, Ok(KmsGrpcResponse::NewKmsContext { context_id })),
                Err(e) => (0, Err(e)),
            },
            KmsGrpcRequest::NewMpcEpoch(req) => self.poll_epoch_result_response(req).await,
            // Like aborts, destructions have no result-polling endpoint on the Core: the
            // send-side ack is the only signal.
            KmsGrpcRequest::DestroyMpcContext(_) | KmsGrpcRequest::DestroyMpcEpoch(_) => {
                (0, Ok(KmsGrpcResponse::NoResponseExpected))
            }
        }
    }

    async fn request_public_decryption(
        &self,
        request: &PublicDecryptionRequest,
    ) -> (i16, Result<SendResponse, ProcessingError>) {
        let Some(request_id) = request.request_id.clone() else {
            return irrecoverable_error(anyhow!("Missing request ID"));
        };
        let inner_client = self.choose_client(request_id.clone());

        send_request_with_retries(
            self.grpc_request_retries,
            || {
                let mut client = inner_client.clone();
                let request = request.clone();
                async move { client.public_decrypt(request).await }
            },
            EventType::PublicDecryptionRequest,
            |_| Ok(SendResponse::Empty),
        )
        .await
    }

    async fn request_user_decryption(
        &self,
        request: &UserDecryptionRequest,
    ) -> (i16, Result<SendResponse, ProcessingError>) {
        let Some(request_id) = request.request_id.clone() else {
            return irrecoverable_error(anyhow!("Missing request ID"));
        };
        let inner_client = self.choose_client(request_id.clone());
        send_request_with_retries(
            self.grpc_request_retries,
            || {
                let mut client = inner_client.clone();
                let request = request.clone();
                async move { client.user_decrypt(request).await }
            },
            EventType::UserDecryptionRequest,
            |_| Ok(SendResponse::Empty),
        )
        .await
    }

    async fn request_prep_keygen(
        &self,
        request: &KeyGenPreprocRequest,
    ) -> (i16, Result<SendResponse, ProcessingError>) {
        let Some(request_id) = request.request_id.clone() else {
            return irrecoverable_error(anyhow!("Missing request ID"));
        };
        let inner_client = self.choose_client(request_id.clone());

        send_request_with_retries(
            self.grpc_request_retries,
            || {
                let mut client = inner_client.clone();
                let request = request.clone();
                async move { client.key_gen_preproc(request).await }
            },
            EventType::PrepKeygenRequest,
            |_| Ok(SendResponse::Empty),
        )
        .await
    }

    async fn request_keygen(
        &self,
        request: &KeyGenRequest,
    ) -> (i16, Result<SendResponse, ProcessingError>) {
        // Route to the shard holding this keygen's preprocessing material (keyed by the
        // preprocessing ID), so prep-keygen, keygen and abort-keygen all target the same shard.
        let Some(preproc_id) = request.preproc_id.clone() else {
            return irrecoverable_error(anyhow!("Missing preprocessing ID"));
        };
        let inner_client = self.choose_client(preproc_id);

        send_request_with_retries(
            self.grpc_request_retries,
            || {
                let mut client = inner_client.clone();
                let request = request.clone();
                async move { client.key_gen(request).await }
            },
            EventType::KeygenRequest,
            |_| Ok(SendResponse::Empty),
        )
        .await
    }

    async fn request_crsgen(
        &self,
        request: &CrsGenRequest,
    ) -> (i16, Result<SendResponse, ProcessingError>) {
        let Some(request_id) = request.request_id.clone() else {
            return irrecoverable_error(anyhow!("Missing request ID"));
        };
        let inner_client = self.choose_client(request_id.clone());

        send_request_with_retries(
            self.grpc_request_retries,
            || {
                let mut client = inner_client.clone();
                let request = request.clone();
                async move { client.crs_gen(request).await }
            },
            EventType::CrsgenRequest,
            |_| Ok(SendResponse::Empty),
        )
        .await
    }

    async fn request_abort_keygen(
        &self,
        request_id: &RequestId,
    ) -> (i16, Result<SendResponse, ProcessingError>) {
        let inner_client = self.choose_client(request_id.clone());

        send_request_with_retries(
            self.grpc_request_retries,
            || {
                let mut client = inner_client.clone();
                let request_id = request_id.clone();
                async move { client.abort_key_gen(request_id).await }
            },
            EventType::AbortKeygenRequest,
            |_| Ok(SendResponse::Empty),
        )
        .await
    }

    async fn request_abort_crsgen(
        &self,
        request_id: &RequestId,
    ) -> (i16, Result<SendResponse, ProcessingError>) {
        let inner_client = self.choose_client(request_id.clone());

        send_request_with_retries(
            self.grpc_request_retries,
            || {
                let mut client = inner_client.clone();
                let request_id = request_id.clone();
                async move { client.abort_crs_gen(request_id).await }
            },
            EventType::AbortCrsgenRequest,
            |_| Ok(SendResponse::Empty),
        )
        .await
    }

    async fn poll_public_decryption_result(
        &self,
        request: PublicDecryptionRequest,
    ) -> (i16, Result<KmsGrpcResponse, ProcessingError>) {
        let Some(request_id) = request.request_id.clone() else {
            return irrecoverable_error(anyhow!("Missing request ID"));
        };
        let inner_client = self.choose_client(request_id.clone());

        let (error_count, grpc_result) = poll_for_result(
            self.grpc_request_retries,
            || {
                let request = Request::new(request_id.clone());
                let mut client = inner_client.clone();
                async move { client.get_public_decryption_result(request).await }
            },
            EventType::PublicDecryptionRequest,
        )
        .await;

        match grpc_result.map_err(ProcessingError::from_response_status) {
            Err(e) => (error_count, Err(e)),
            Ok(grpc_response) => (
                error_count,
                KmsGrpcResponse::try_from((request_id, grpc_response))
                    .map_err(ProcessingError::Irrecoverable),
            ),
        }
    }

    async fn poll_user_decryption_result(
        &self,
        request: UserDecryptionRequest,
    ) -> (i16, Result<KmsGrpcResponse, ProcessingError>) {
        let Some(request_id) = request.request_id.clone() else {
            return irrecoverable_error(anyhow!("Missing request ID"));
        };
        let inner_client = self.choose_client(request_id.clone());

        let (error_count, grpc_result) = poll_for_result(
            self.grpc_request_retries,
            || {
                let mut client = inner_client.clone();
                let request = Request::new(request_id.clone());
                async move { client.get_user_decryption_result(request).await }
            },
            EventType::UserDecryptionRequest,
        )
        .await;

        match grpc_result.map_err(ProcessingError::from_response_status) {
            Err(e) => (error_count, Err(e)),
            Ok(grpc_response) => (
                error_count,
                KmsGrpcResponse::try_from((request_id, grpc_response))
                    .map_err(ProcessingError::Irrecoverable),
            ),
        }
    }

    async fn poll_prep_keygen_result(
        &self,
        request: KeyGenPreprocRequest,
    ) -> (i16, Result<KmsGrpcResponse, ProcessingError>) {
        let Some(request_id) = request.request_id.clone() else {
            return irrecoverable_error(anyhow!("Missing request ID"));
        };
        let inner_client = self.choose_client(request_id.clone());

        let (error_count, grpc_result) = poll_for_result(
            self.grpc_request_retries,
            || {
                let mut client = inner_client.clone();
                let request = Request::new(request_id.clone());
                async move { client.get_key_gen_preproc_result(request).await }
            },
            EventType::PrepKeygenRequest,
        )
        .await;

        match grpc_result {
            Err(status) => (
                error_count,
                Err(ProcessingError::from_response_status(status)),
            ),
            Ok(grpc_response) => (
                error_count,
                Ok(KmsGrpcResponse::PrepKeygen(grpc_response.into_inner())),
            ),
        }
    }

    async fn poll_keygen_result(
        &self,
        request: KeyGenRequest,
    ) -> (i16, Result<KmsGrpcResponse, ProcessingError>) {
        let Some(request_id) = request.request_id.clone() else {
            return irrecoverable_error(anyhow!("Missing request ID"));
        };
        // Poll the shard that ran the keygen, i.e. the one holding its preprocessing material
        // (keyed by the preprocessing ID). The result itself is still fetched by key ID.
        let Some(preproc_id) = request.preproc_id.clone() else {
            return irrecoverable_error(anyhow!("Missing preprocessing ID"));
        };
        let inner_client = self.choose_client(preproc_id);

        let (error_count, grpc_result) = poll_for_result(
            self.grpc_request_retries,
            || {
                let mut client = inner_client.clone();
                let request = Request::new(request_id.clone());
                async move { client.get_key_gen_result(request).await }
            },
            EventType::KeygenRequest,
        )
        .await;

        match grpc_result {
            Err(status) => (
                error_count,
                Err(ProcessingError::from_response_status(status)),
            ),
            Ok(grpc_response) => (
                error_count,
                Ok(KmsGrpcResponse::Keygen(grpc_response.into_inner())),
            ),
        }
    }

    async fn poll_crsgen_result(
        &self,
        request: CrsGenRequest,
    ) -> (i16, Result<KmsGrpcResponse, ProcessingError>) {
        let Some(request_id) = request.request_id.clone() else {
            return irrecoverable_error(anyhow!("Missing request ID"));
        };
        let inner_client = self.choose_client(request_id.clone());

        let (error_count, grpc_result) = poll_for_result(
            self.grpc_request_retries,
            || {
                let mut client = inner_client.clone();
                let request = Request::new(request_id.clone());
                async move { client.get_crs_gen_result(request).await }
            },
            EventType::CrsgenRequest,
        )
        .await;

        match grpc_result {
            Err(status) => (
                error_count,
                Err(ProcessingError::from_response_status(status)),
            ),
            Ok(grpc_response) => (
                error_count,
                Ok(KmsGrpcResponse::Crsgen(grpc_response.into_inner())),
            ),
        }
    }

    async fn request_new_mpc_context(
        &self,
        request: &NewMpcContextRequest,
    ) -> (i16, Result<SendResponse, ProcessingError>) {
        let Some(context_id) = request
            .new_context
            .as_ref()
            .and_then(|c| c.context_id.clone())
        else {
            return irrecoverable_error(anyhow!("Missing context_id in NewMpcContextRequest"));
        };

        let inner_client = self.choose_client(context_id);

        send_request_with_retries(
            self.grpc_request_retries,
            || {
                let mut client = inner_client.clone();
                let request = request.clone();
                async move { client.new_mpc_context(request).await }
            },
            EventType::NewKmsContext,
            |_| Ok(SendResponse::Empty),
        )
        .await
    }

    async fn request_destroy_mpc_context(
        &self,
        request: &DestroyMpcContextRequest,
    ) -> (i16, Result<SendResponse, ProcessingError>) {
        let Some(context_id) = request.context_id.clone() else {
            return irrecoverable_error(anyhow!("Missing context_id in DestroyMpcContextRequest"));
        };
        let inner_client = self.choose_client(context_id);

        send_request_with_retries(
            self.grpc_request_retries,
            || {
                let mut client = inner_client.clone();
                let request = request.clone();
                async move { client.destroy_mpc_context(request).await }
            },
            EventType::KmsContextDestroyed,
            map_destroyed_epochs,
        )
        .await
    }

    async fn request_destroy_mpc_epoch(
        &self,
        request: &DestroyMpcEpochRequest,
    ) -> (i16, Result<SendResponse, ProcessingError>) {
        let Some(epoch_id) = request.epoch_id.clone() else {
            return irrecoverable_error(anyhow!("Missing epoch_id in DestroyMpcEpochRequest"));
        };
        let inner_client = self.choose_client(epoch_id);

        send_request_with_retries(
            self.grpc_request_retries,
            || {
                let mut client = inner_client.clone();
                let request = request.clone();
                async move { client.destroy_mpc_epoch(request).await }
            },
            EventType::KmsEpochDestroyed,
            |_| Ok(SendResponse::Empty),
        )
        .await
    }

    async fn request_new_mpc_epoch(
        &self,
        request: &NewMpcEpochRequest,
    ) -> (i16, Result<SendResponse, ProcessingError>) {
        let Some(epoch_id) = request.epoch_id.clone() else {
            return irrecoverable_error(anyhow!("Missing epoch_id in NewMpcEpochRequest"));
        };
        let inner_client = self.choose_client(epoch_id);

        send_request_with_retries(
            self.grpc_request_retries,
            || {
                let mut client = inner_client.clone();
                let request = request.clone();
                async move { client.new_mpc_epoch(request).await }
            },
            EventType::NewKmsEpoch,
            |_| Ok(SendResponse::Empty),
        )
        .await
    }

    /// Polls `GetEpochResult` and wraps the reply in a `KmsGrpcResponse::EpochResult`. The Core
    /// keys epochs by `epoch_id`, so every `NewKmsEpoch` event flows through here.
    async fn poll_epoch_result_response(
        &self,
        request: NewMpcEpochRequest,
    ) -> (i16, Result<KmsGrpcResponse, ProcessingError>) {
        let Some(context_id_proto) = request.context_id.clone() else {
            return irrecoverable_error(anyhow!("Missing context_id in NewMpcEpochRequest"));
        };
        let context_id = match request_id_to_u256(context_id_proto.clone()) {
            Ok(id) => id,
            Err(e) => {
                return irrecoverable_error(anyhow!(
                    "Invalid context_id in NewMpcEpochRequest: {e}"
                ));
            }
        };
        let Some(epoch_id_proto) = request.epoch_id.clone() else {
            return irrecoverable_error(anyhow!("Missing epoch_id in NewMpcEpochRequest"));
        };
        let epoch_id = match request_id_to_u256(epoch_id_proto.clone()) {
            Ok(id) => id,
            Err(e) => {
                return irrecoverable_error(anyhow!("Invalid epoch_id in NewMpcEpochRequest: {e}"));
            }
        };

        let inner_client = self.choose_client(epoch_id_proto.clone());
        let (error_count, grpc_result) = poll_for_result(
            self.grpc_request_retries,
            || {
                let mut client = inner_client.clone();
                let request = Request::new(epoch_id_proto.clone());
                async move { client.get_epoch_result(request).await }
            },
            EventType::NewKmsEpoch,
        )
        .await;

        match grpc_result.map_err(ProcessingError::from_response_status) {
            Err(e) => (error_count, Err(e)),
            Ok(grpc_response) => (
                error_count,
                Ok(KmsGrpcResponse::EpochResult {
                    context_id,
                    epoch_id,
                    grpc_response: grpc_response.into_inner(),
                }),
            ),
        }
    }

    fn choose_client(&self, request_id: RequestId) -> CoreServiceEndpointClient<Channel> {
        let request_id = request_id_to_u256(request_id).unwrap_or_else(|e| {
            warn!("Failed to parse request ID: {e}. Sending request to shard 0 by default");
            U256::ZERO
        });
        let client_index = u256_to_u32(request_id % U256::from(self.inners.len())).unwrap_or_else(|e| {
            warn!("Failed to convert request ID from U256 to u32: {e}. Sending request to shard 0 by default");
            0
        });
        info!("Sending GRPC request to KMS shard #{client_index}");
        self.inners[client_index as usize].clone()
    }
}

/// Reads the `context_id` U256 out of a `NewMpcContextRequest`.
fn extract_new_mpc_context_id(request: &NewMpcContextRequest) -> Result<U256, ProcessingError> {
    request
        .new_context
        .as_ref()
        .and_then(|c| c.context_id.clone())
        .ok_or_else(|| {
            ProcessingError::Irrecoverable(anyhow!("Missing context_id in NewMpcContextRequest"))
        })
        .map(request_id_to_u256)?
        .map_err(|e| {
            ProcessingError::Irrecoverable(anyhow!(
                "Invalid context_id in NewMpcContextRequest: {e}"
            ))
        })
}

fn irrecoverable_error<T>(err: anyhow::Error) -> (i16, Result<T, ProcessingError>) {
    (0, Err(ProcessingError::Irrecoverable(err)))
}

/// Converts a `DestroyMpcContextResponse` into the list of destroyed epoch IDs to invalidate.
fn map_destroyed_epochs(
    response: DestroyMpcContextResponse,
) -> Result<SendResponse, ProcessingError> {
    response
        .epoch_ids
        .into_iter()
        .map(request_id_to_u256)
        .collect::<Result<Vec<_>, _>>()
        .map(SendResponse::DestroyedEpochs)
        .map_err(|e| {
            ProcessingError::Irrecoverable(anyhow!(
                "Invalid epoch_id in DestroyMpcContextResponse: {e}"
            ))
        })
}

const RETRYABLE_GRPC_CODE: [Code; 4] = [
    Code::DeadlineExceeded,
    Code::ResourceExhausted,
    Code::Unavailable,
    Code::Unknown,
];

/// Sends a given GRPC request to the KMS with retries.
///
/// Returns the number of errors and the result of the request.
async fn send_request_with_retries<F, Fut, R, M>(
    retries: u8,
    mut request_fn: F,
    event_type: EventType,
    map_response: M,
) -> (i16, Result<SendResponse, ProcessingError>)
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<Response<R>, Status>>,
    M: Fn(R) -> Result<SendResponse, ProcessingError>,
{
    for i in 1..=retries as i16 {
        // Don't count last successful attempt
        let error = match request_fn().await {
            Ok(response) => {
                GRPC_REQUEST_SENT_COUNTER
                    .with_label_values(&[event_type.as_str()])
                    .inc();
                info!("GRPC request successfully sent to the KMS!");
                return (i - 1, map_response(response.into_inner()));
            }
            Err(e) if e.code() == Code::AlreadyExists => {
                info!("GRPC already sent to the KMS!");
                return (i - 1, Ok(SendResponse::Empty));
            }
            Err(e) => e,
        };

        GRPC_REQUEST_SENT_ERRORS
            .with_label_values(&[event_type.as_str()])
            .inc();
        if RETRYABLE_GRPC_CODE.contains(&error.code()) {
            warn!("#{i}/{retries} GRPC request attempt failed: {error}");
        } else {
            return (i, Err(ProcessingError::Irrecoverable(error.into())));
        }
    }
    (
        retries as i16,
        Err(ProcessingError::Recoverable(anyhow!(
            "All GRPC requests failed!"
        ))),
    )
}

/// Polls the KMS for the result of a request previously sent, with retries.
///
/// Returns the number of errors and the result of the polling.
async fn poll_for_result<T, F, Fut>(
    retries: u8,
    mut poll_fn: F,
    event_type: EventType,
) -> (i16, Result<Response<T>, Status>)
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<Response<T>, Status>>,
{
    for i in 1..=retries as i16 {
        info!("#{i}/{retries} Trying to retrieve result from KMS Core...");
        match poll_fn().await {
            Ok(response) => {
                GRPC_RESPONSE_POLLED_COUNTER
                    .with_label_values(&[event_type.as_str()])
                    .inc();
                info!("Result successfully retrieved from KMS Core!");
                return (i - 1, Ok(response)); // Don't count last successful attempt
            }
            Err(status) => {
                if RETRYABLE_GRPC_CODE.contains(&status.code()) {
                    info!("#{i}/{retries} Failed to poll result from KMS: {status}");
                } else {
                    // Any other error is returned immediately
                    GRPC_RESPONSE_POLLED_ERRORS
                        .with_label_values(&[event_type.as_str()])
                        .inc();
                    return (i, Err(status));
                }
            }
        }
    }
    GRPC_RESPONSE_POLLED_ERRORS
        .with_label_values(&[event_type.as_str()])
        .inc();
    (
        retries as i16,
        Err(Status::unavailable("all result polling attempts failed")),
    )
}
