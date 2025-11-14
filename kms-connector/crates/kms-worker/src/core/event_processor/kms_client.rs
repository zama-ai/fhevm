use crate::{
    core::{Config, event_processor::processor::ProcessingError},
    monitoring::metrics::{
        DECRYPTION_REQUEST_SENT_COUNTER, DECRYPTION_REQUEST_SENT_ERRORS,
        DECRYPTION_RESPONSE_COUNTER, DECRYPTION_RESPONSE_ERRORS,
        KEY_MANAGEMENT_REQUEST_SENT_COUNTER, KEY_MANAGEMENT_REQUEST_SENT_ERRORS,
        KEY_MANAGEMENT_RESPONSE_COUNTER, KEY_MANAGEMENT_RESPONSE_ERRORS,
    },
};
use alloy::{hex, primitives::U256};
use anyhow::anyhow;
use connector_utils::{
    conn::{CONNECTION_RETRY_DELAY, CONNECTION_RETRY_NUMBER},
    types::{
        KmsGrpcRequest, KmsGrpcResponse, decode_request_id, gw_event::PRSS_INIT_ID, u256_to_u32,
    },
};
use kms_grpc::{
    kms::v1::{
        CrsGenRequest, InitRequest, InitiateResharingRequest, KeyGenPreprocRequest, KeyGenRequest,
        PublicDecryptionRequest, RequestId, UserDecryptionRequest,
    },
    kms_service::v1::core_service_endpoint_client::CoreServiceEndpointClient,
};
use prometheus::IntCounter;
use std::sync::LazyLock;
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
    ) -> (i16, Result<(), ProcessingError>) {
        match request {
            KmsGrpcRequest::PublicDecryption(req) => self.request_public_decryption(req).await,
            KmsGrpcRequest::UserDecryption(req) => self.request_user_decryption(req).await,
            KmsGrpcRequest::PrepKeygen(req) => self.request_prep_keygen(req).await,
            KmsGrpcRequest::Keygen(req) => self.request_keygen(req).await,
            KmsGrpcRequest::Crsgen(req) => self.request_crsgen(req).await,
            KmsGrpcRequest::PrssInit(req) => self.request_prss_init(req).await,
            KmsGrpcRequest::KeyReshareSameSet(req) => self.request_initiate_resharing(req).await,
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
            KmsGrpcRequest::PrssInit(_) => (0, Ok(KmsGrpcResponse::NoResponseExpected)),
            KmsGrpcRequest::KeyReshareSameSet(_) => (0, Ok(KmsGrpcResponse::NoResponseExpected)),
        }
    }

    async fn request_public_decryption(
        &self,
        request: &PublicDecryptionRequest,
    ) -> (i16, Result<(), ProcessingError>) {
        let Some(request_id) = request.request_id.clone() else {
            return (
                0,
                Err(ProcessingError::Irrecoverable(anyhow!(
                    "Missing request ID"
                ))),
            );
        };
        let inner_client = self.choose_client(request_id.clone());

        send_request_with_retries(
            self.grpc_request_retries,
            || {
                let mut client = inner_client.clone();
                let request = request.clone();
                async move { client.public_decrypt(request).await }
            },
            &DECRYPTION_REQUEST_SENT_COUNTER,
            &DECRYPTION_REQUEST_SENT_ERRORS,
        )
        .await
    }

    async fn request_user_decryption(
        &self,
        request: &UserDecryptionRequest,
    ) -> (i16, Result<(), ProcessingError>) {
        let Some(request_id) = request.request_id.clone() else {
            return (
                0,
                Err(ProcessingError::Irrecoverable(anyhow!(
                    "Missing request ID"
                ))),
            );
        };
        let inner_client = self.choose_client(request_id.clone());
        send_request_with_retries(
            self.grpc_request_retries,
            || {
                let mut client = inner_client.clone();
                let request = request.clone();
                async move { client.user_decrypt(request).await }
            },
            &DECRYPTION_REQUEST_SENT_COUNTER,
            &DECRYPTION_REQUEST_SENT_ERRORS,
        )
        .await
    }

    async fn request_prep_keygen(
        &self,
        request: &KeyGenPreprocRequest,
    ) -> (i16, Result<(), ProcessingError>) {
        let Some(request_id) = request.request_id.clone() else {
            return (
                0,
                Err(ProcessingError::Irrecoverable(anyhow!(
                    "Missing request ID"
                ))),
            );
        };
        let inner_client = self.choose_client(request_id.clone());

        send_request_with_retries(
            self.grpc_request_retries,
            || {
                let mut client = inner_client.clone();
                let request = request.clone();
                async move { client.key_gen_preproc(request).await }
            },
            &KEY_MANAGEMENT_REQUEST_SENT_COUNTER,
            &KEY_MANAGEMENT_REQUEST_SENT_ERRORS,
        )
        .await
    }

    async fn request_keygen(&self, request: &KeyGenRequest) -> (i16, Result<(), ProcessingError>) {
        let Some(request_id) = request.request_id.clone() else {
            return (
                0,
                Err(ProcessingError::Irrecoverable(anyhow!(
                    "Missing request ID"
                ))),
            );
        };
        let inner_client = self.choose_client(request_id.clone());

        send_request_with_retries(
            self.grpc_request_retries,
            || {
                let mut client = inner_client.clone();
                let request = request.clone();
                async move { client.key_gen(request).await }
            },
            &KEY_MANAGEMENT_REQUEST_SENT_COUNTER,
            &KEY_MANAGEMENT_REQUEST_SENT_ERRORS,
        )
        .await
    }

    async fn request_crsgen(&self, request: &CrsGenRequest) -> (i16, Result<(), ProcessingError>) {
        let Some(request_id) = request.request_id.clone() else {
            return (
                0,
                Err(ProcessingError::Irrecoverable(anyhow!(
                    "Missing request ID"
                ))),
            );
        };
        let inner_client = self.choose_client(request_id.clone());

        send_request_with_retries(
            self.grpc_request_retries,
            || {
                let mut client = inner_client.clone();
                let request = request.clone();
                async move { client.crs_gen(request).await }
            },
            &KEY_MANAGEMENT_REQUEST_SENT_COUNTER,
            &KEY_MANAGEMENT_REQUEST_SENT_ERRORS,
        )
        .await
    }

    async fn request_prss_init(&self, request: &InitRequest) -> (i16, Result<(), ProcessingError>) {
        let inner_client = self.choose_client(RequestId {
            request_id: hex::encode(PRSS_INIT_ID.to_be_bytes::<32>()),
        });
        send_request_with_retries(
            self.grpc_request_retries,
            || {
                let mut client = inner_client.clone();
                let request = request.clone();
                async move { client.init(request).await }
            },
            &KEY_MANAGEMENT_REQUEST_SENT_COUNTER,
            &KEY_MANAGEMENT_REQUEST_SENT_ERRORS,
        )
        .await
    }

    async fn request_initiate_resharing(
        &self,
        request: &InitiateResharingRequest,
    ) -> (i16, Result<(), ProcessingError>) {
        let Some(request_id) = request.request_id.clone() else {
            return (
                0,
                Err(ProcessingError::Irrecoverable(anyhow!(
                    "Missing request ID"
                ))),
            );
        };
        let inner_client = self.choose_client(request_id.clone());

        send_request_with_retries(
            self.grpc_request_retries,
            || {
                let mut client = inner_client.clone();
                let request = request.clone();
                async move { client.initiate_resharing(request).await }
            },
            &KEY_MANAGEMENT_REQUEST_SENT_COUNTER,
            &KEY_MANAGEMENT_REQUEST_SENT_ERRORS,
        )
        .await
    }

    async fn poll_public_decryption_result(
        &self,
        request: PublicDecryptionRequest,
    ) -> (i16, Result<KmsGrpcResponse, ProcessingError>) {
        let Some(request_id) = request.request_id.clone() else {
            return (
                0,
                Err(ProcessingError::Irrecoverable(anyhow!(
                    "Missing request ID"
                ))),
            );
        };
        let inner_client = self.choose_client(request_id.clone());

        let (error_count, grpc_result) = poll_for_result(
            self.grpc_request_retries,
            || {
                let request = Request::new(request_id.clone());
                let mut client = inner_client.clone();
                async move { client.get_public_decryption_result(request).await }
            },
            &DECRYPTION_RESPONSE_COUNTER,
            &DECRYPTION_RESPONSE_ERRORS,
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
            return (
                0,
                Err(ProcessingError::Irrecoverable(anyhow!(
                    "Missing request ID"
                ))),
            );
        };
        let inner_client = self.choose_client(request_id.clone());

        let (error_count, grpc_result) = poll_for_result(
            self.grpc_request_retries,
            || {
                let mut client = inner_client.clone();
                let request = Request::new(request_id.clone());
                async move { client.get_user_decryption_result(request).await }
            },
            &DECRYPTION_RESPONSE_COUNTER,
            &DECRYPTION_RESPONSE_ERRORS,
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
            return (
                0,
                Err(ProcessingError::Irrecoverable(anyhow!(
                    "Missing request ID"
                ))),
            );
        };
        let inner_client = self.choose_client(request_id.clone());

        let (error_count, grpc_result) = poll_for_result(
            self.grpc_request_retries,
            || {
                let mut client = inner_client.clone();
                let request = Request::new(request_id.clone());
                async move { client.get_key_gen_preproc_result(request).await }
            },
            &KEY_MANAGEMENT_RESPONSE_COUNTER,
            &KEY_MANAGEMENT_RESPONSE_ERRORS,
        )
        .await;

        match grpc_result.map_err(ProcessingError::from_response_status) {
            Err(e) => (error_count, Err(e)),
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
            return (
                0,
                Err(ProcessingError::Irrecoverable(anyhow!(
                    "Missing request ID"
                ))),
            );
        };
        let inner_client = self.choose_client(request_id.clone());

        let (error_count, grpc_result) = poll_for_result(
            self.grpc_request_retries,
            || {
                let mut client = inner_client.clone();
                let request = Request::new(request_id.clone());
                async move { client.get_key_gen_result(request).await }
            },
            &KEY_MANAGEMENT_RESPONSE_COUNTER,
            &KEY_MANAGEMENT_RESPONSE_ERRORS,
        )
        .await;

        match grpc_result.map_err(ProcessingError::from_response_status) {
            Err(e) => (error_count, Err(e)),
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
            return (
                0,
                Err(ProcessingError::Irrecoverable(anyhow!(
                    "Missing request ID"
                ))),
            );
        };
        let inner_client = self.choose_client(request_id.clone());

        let (error_count, grpc_result) = poll_for_result(
            self.grpc_request_retries,
            || {
                let mut client = inner_client.clone();
                let request = Request::new(request_id.clone());
                async move { client.get_crs_gen_result(request).await }
            },
            &KEY_MANAGEMENT_RESPONSE_COUNTER,
            &KEY_MANAGEMENT_RESPONSE_ERRORS,
        )
        .await;

        match grpc_result.map_err(ProcessingError::from_response_status) {
            Err(e) => (error_count, Err(e)),
            Ok(grpc_response) => (
                error_count,
                Ok(KmsGrpcResponse::Crsgen(grpc_response.into_inner())),
            ),
        }
    }

    fn choose_client(&self, request_id: RequestId) -> CoreServiceEndpointClient<Channel> {
        let request_id = decode_request_id(request_id).unwrap_or_else(|e| {
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

const RETRYABLE_GRPC_CODE: [Code; 4] = [
    Code::DeadlineExceeded,
    Code::ResourceExhausted,
    Code::Unavailable,
    Code::Unknown,
];

/// Sends a given GRPC request to the KMS with retries.
///
/// Returns the number of errors and the result of the request.
#[tracing::instrument(skip_all)]
async fn send_request_with_retries<F, Fut, R>(
    retries: u8,
    mut request_fn: F,
    success_counter: &LazyLock<IntCounter>,
    error_counter: &LazyLock<IntCounter>,
) -> (i16, Result<(), ProcessingError>)
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<Response<R>, Status>>,
{
    for i in 1..=retries as i16 {
        match request_fn().await {
            Ok(_) => {
                success_counter.inc();
                info!("GRPC request successfully sent to the KMS!");
                return (i - 1, Ok(())); // Don't count last successful attempt
            }
            Err(e) if e.code() == Code::AlreadyExists => {
                info!("GRPC already sent to the KMS!");
                return (i - 1, Ok(())); // Don't count last successful attempt
            }
            Err(e) if RETRYABLE_GRPC_CODE.contains(&e.code()) => {
                error_counter.inc();
                warn!("#{i}/{retries} GRPC request attempt failed: {e}");
            }
            Err(e) => {
                error_counter.inc();
                return (i, Err(ProcessingError::Irrecoverable(e.into())));
            }
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
#[tracing::instrument(skip_all)]
async fn poll_for_result<T, F, Fut>(
    retries: u8,
    mut poll_fn: F,
    success_counter: &LazyLock<IntCounter>,
    error_counter: &LazyLock<IntCounter>,
) -> (i16, Result<Response<T>, Status>)
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<Response<T>, Status>>,
{
    for i in 1..=retries as i16 {
        info!("#{i}/{retries} Trying to retrieve result from KMS Core...");
        match poll_fn().await {
            Ok(response) => {
                success_counter.inc();
                info!("Result successfully retrieved from KMS Core!");
                return (i - 1, Ok(response)); // Don't count last successful attempt
            }
            Err(status) => {
                if RETRYABLE_GRPC_CODE.contains(&status.code()) {
                    info!("#{i}/{retries} Failed to poll result from KMS: {status}");
                } else {
                    // Any other error is returned immediately
                    error_counter.inc();
                    return (i, Err(status));
                }
            }
        }
    }
    error_counter.inc();
    (
        retries as i16,
        Err(Status::unavailable("all result polling attempts failed")),
    )
}
