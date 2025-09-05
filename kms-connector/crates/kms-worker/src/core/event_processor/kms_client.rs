use crate::{
    core::{
        Config,
        event_processor::{eip712::verify_user_decryption_eip712, processor::ProcessingError},
    },
    monitoring::metrics::{
        DECRYPTION_REQUEST_SENT_COUNTER, DECRYPTION_REQUEST_SENT_ERRORS,
        DECRYPTION_RESPONSE_COUNTER, DECRYPTION_RESPONSE_ERRORS,
        KEY_MANAGEMENT_REQUEST_SENT_COUNTER, KEY_MANAGEMENT_REQUEST_SENT_ERRORS,
        KEY_MANAGEMENT_RESPONSE_COUNTER, KEY_MANAGEMENT_RESPONSE_ERRORS,
    },
};
use anyhow::anyhow;
use connector_utils::{
    conn::{CONNECTION_RETRY_DELAY, CONNECTION_RETRY_NUMBER},
    types::{KmsGrpcRequest, KmsGrpcResponse},
};
use kms_grpc::{
    kms::v1::{
        CrsGenRequest, Empty, KeyGenPreprocRequest, KeyGenRequest, PublicDecryptionRequest,
        RequestId, UserDecryptionRequest,
    },
    kms_service::v1::core_service_endpoint_client::CoreServiceEndpointClient,
};
use prometheus::IntCounter;
use std::{
    sync::LazyLock,
    time::{Duration, Instant},
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

    /// The timeout to retrieve the response of a public decryption operation.
    public_decryption_timeout: Duration,

    /// The timeout to retrieve the response of a user decryption operation.
    user_decryption_timeout: Duration,

    /// The interval between GRPC response collection retries.
    grpc_poll_interval: Duration,
}

impl KmsClient {
    pub fn new(
        channels: Vec<Channel>,
        grpc_request_retries: u8,
        public_decryption_timeout: Duration,
        user_decryption_timeout: Duration,
        grpc_poll_interval: Duration,
    ) -> Self {
        let inners = channels
            .into_iter()
            .map(CoreServiceEndpointClient::new)
            .collect();

        Self {
            inners,
            grpc_request_retries,
            public_decryption_timeout,
            user_decryption_timeout,
            grpc_poll_interval,
        }
    }

    /// Connects to all the KMS Core shards.
    pub async fn connect(config: &Config) -> anyhow::Result<Self> {
        let mut channels = vec![];
        for (i, kms_shard_endpoint) in config.kms_core_endpoints.iter().enumerate() {
            channels.push(KmsClient::connect_single_shard(i, kms_shard_endpoint).await?);
        }

        Ok(Self::new(
            channels,
            config.grpc_request_retries,
            config.public_decryption_timeout,
            config.user_decryption_timeout,
            config.grpc_poll_interval,
        ))
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
                Err(e) => warn!("KMS Core shard #{shard_id} connection attempt #{i} failed: {e}"),
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
    #[tracing::instrument(skip_all)]
    pub async fn send_request(
        &mut self,
        request: KmsGrpcRequest,
    ) -> Result<KmsGrpcResponse, ProcessingError> {
        match request {
            KmsGrpcRequest::PublicDecryption(req) => self.request_public_decryption(req).await,
            KmsGrpcRequest::UserDecryption(req) => self.request_user_decryption(req).await,
            KmsGrpcRequest::PrepKeygen(req) => self.request_prep_keygen(req).await,
            KmsGrpcRequest::Keygen(req) => self.request_keygen(req).await,
            KmsGrpcRequest::Crsgen(req) => self.request_crsgen(req).await,
        }
    }

    async fn request_public_decryption(
        &mut self,
        request: PublicDecryptionRequest,
    ) -> Result<KmsGrpcResponse, ProcessingError> {
        let request_id = request
            .request_id
            .clone()
            .ok_or_else(|| ProcessingError::Irrecoverable(anyhow!("Missing request ID")))?;

        let inner_client = self.choose_client(request_id.clone());
        send_request_with_retry(
            self.grpc_request_retries,
            || {
                let mut client = inner_client.clone();
                let request = request.clone();
                async move { client.public_decrypt(request).await }
            },
            &DECRYPTION_REQUEST_SENT_COUNTER,
            &DECRYPTION_REQUEST_SENT_ERRORS,
        )
        .await?;

        let grpc_response = poll_for_result(
            self.public_decryption_timeout,
            self.grpc_poll_interval,
            || {
                let request = Request::new(request_id.clone());
                let mut client = inner_client.clone();
                async move { client.get_public_decryption_result(request).await }
            },
            &DECRYPTION_RESPONSE_COUNTER,
            &DECRYPTION_RESPONSE_ERRORS,
        )
        .await
        .map_err(ProcessingError::from_response_status)?;

        KmsGrpcResponse::try_from((request_id, grpc_response))
            .map_err(ProcessingError::Irrecoverable)
    }

    async fn request_user_decryption(
        &mut self,
        request: UserDecryptionRequest,
    ) -> Result<KmsGrpcResponse, ProcessingError> {
        let request_id = request
            .request_id
            .clone()
            .ok_or_else(|| ProcessingError::Irrecoverable(anyhow!("Missing request ID")))?;

        // Verify the EIP-712 signature for the user decryption request
        if let Err(e) = verify_user_decryption_eip712(&request) {
            warn!("Failed to verify request: {e}. Proceeding despite failure...");
        }

        let inner_client = self.choose_client(request_id.clone());
        send_request_with_retry(
            self.grpc_request_retries,
            || {
                let mut client = inner_client.clone();
                let request = request.clone();
                async move { client.user_decrypt(request).await }
            },
            &DECRYPTION_REQUEST_SENT_COUNTER,
            &DECRYPTION_REQUEST_SENT_ERRORS,
        )
        .await?;

        let grpc_response = poll_for_result(
            self.user_decryption_timeout,
            self.grpc_poll_interval,
            || {
                let mut client = inner_client.clone();
                let request = Request::new(request_id.clone());
                async move { client.get_user_decryption_result(request).await }
            },
            &DECRYPTION_RESPONSE_COUNTER,
            &DECRYPTION_RESPONSE_ERRORS,
        )
        .await
        .map_err(ProcessingError::from_response_status)?;

        KmsGrpcResponse::try_from((request_id, grpc_response))
            .map_err(ProcessingError::Irrecoverable)
    }

    async fn request_prep_keygen(
        &self,
        request: KeyGenPreprocRequest,
    ) -> Result<KmsGrpcResponse, ProcessingError> {
        let request_id = request
            .request_id
            .clone()
            .ok_or_else(|| ProcessingError::Irrecoverable(anyhow!("Missing request ID")))?;

        let inner_client = self.choose_client(request_id.clone());
        send_request_with_retry(
            self.grpc_request_retries,
            || {
                let mut client = inner_client.clone();
                let request = request.clone();
                async move { client.key_gen_preproc(request).await }
            },
            &KEY_MANAGEMENT_REQUEST_SENT_COUNTER,
            &KEY_MANAGEMENT_REQUEST_SENT_ERRORS,
        )
        .await?;

        let grpc_response = poll_for_result(
            self.user_decryption_timeout,
            self.grpc_poll_interval,
            || {
                let mut client = inner_client.clone();
                let request = Request::new(request_id.clone());
                async move { client.get_key_gen_preproc_result(request).await }
            },
            &KEY_MANAGEMENT_RESPONSE_COUNTER,
            &KEY_MANAGEMENT_RESPONSE_ERRORS,
        )
        .await
        .map_err(ProcessingError::from_response_status)?;

        Ok(KmsGrpcResponse::PrepKeygen(grpc_response.into_inner()))
    }

    async fn request_keygen(
        &self,
        request: KeyGenRequest,
    ) -> Result<KmsGrpcResponse, ProcessingError> {
        let request_id = request
            .request_id
            .clone()
            .ok_or_else(|| ProcessingError::Irrecoverable(anyhow!("Missing request ID")))?;

        let inner_client = self.choose_client(request_id.clone());
        send_request_with_retry(
            self.grpc_request_retries,
            || {
                let mut client = inner_client.clone();
                let request = request.clone();
                async move { client.key_gen(request).await }
            },
            &KEY_MANAGEMENT_REQUEST_SENT_COUNTER,
            &KEY_MANAGEMENT_REQUEST_SENT_ERRORS,
        )
        .await?;

        let grpc_response = poll_for_result(
            self.user_decryption_timeout,
            self.grpc_poll_interval,
            || {
                let mut client = inner_client.clone();
                let request = Request::new(request_id.clone());
                async move { client.get_key_gen_result(request).await }
            },
            &KEY_MANAGEMENT_RESPONSE_COUNTER,
            &KEY_MANAGEMENT_RESPONSE_ERRORS,
        )
        .await
        .map_err(ProcessingError::from_response_status)?;

        Ok(KmsGrpcResponse::Keygen(grpc_response.into_inner()))
    }

    async fn request_crsgen(
        &self,
        request: CrsGenRequest,
    ) -> Result<KmsGrpcResponse, ProcessingError> {
        let request_id = request
            .request_id
            .clone()
            .ok_or_else(|| ProcessingError::Irrecoverable(anyhow!("Missing request ID")))?;

        let inner_client = self.choose_client(request_id.clone());
        send_request_with_retry(
            self.grpc_request_retries,
            || {
                let mut client = inner_client.clone();
                let request = request.clone();
                async move { client.crs_gen(request).await }
            },
            &KEY_MANAGEMENT_REQUEST_SENT_COUNTER,
            &KEY_MANAGEMENT_REQUEST_SENT_ERRORS,
        )
        .await?;

        let grpc_response = poll_for_result(
            self.user_decryption_timeout,
            self.grpc_poll_interval,
            || {
                let mut client = inner_client.clone();
                let request = Request::new(request_id.clone());
                async move { client.get_crs_gen_result(request).await }
            },
            &KEY_MANAGEMENT_RESPONSE_COUNTER,
            &KEY_MANAGEMENT_RESPONSE_ERRORS,
        )
        .await
        .map_err(ProcessingError::from_response_status)?;

        Ok(KmsGrpcResponse::Crsgen(grpc_response.into_inner()))
    }

    fn choose_client(&self, request_id: RequestId) -> CoreServiceEndpointClient<Channel> {
        let request_id = request_id.request_id.parse::<usize>().unwrap_or_else(|_| {
            warn!("Failed to parse request ID. Sending request to shard 0 by default");
            0
        });
        let client_index = request_id % self.inners.len();
        info!("Sending GRPC request to KMS shard #{client_index}...");
        self.inners[client_index].clone()
    }
}

#[tracing::instrument(skip_all)]
async fn send_request_with_retry<F, Fut>(
    retries: u8,
    mut request_fn: F,
    success_counter: &LazyLock<IntCounter>,
    error_counter: &LazyLock<IntCounter>,
) -> Result<(), ProcessingError>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<Response<Empty>, Status>>,
{
    for i in 1..=retries {
        match request_fn()
            .await
            .map_err(ProcessingError::from_request_status)
        {
            Ok(_) => break,
            Err(ProcessingError::Irrecoverable(e)) => {
                error_counter.inc();
                return Err(ProcessingError::Irrecoverable(e));
            }
            Err(ProcessingError::Recoverable(e)) => {
                error_counter.inc();
                warn!("#{}/{} GRPC request attempt failed: {}", i, retries, e);
                if i == retries {
                    return Err(ProcessingError::Recoverable(anyhow!(
                        "All GRPC requests failed!"
                    )));
                }
            }
        }
    }
    success_counter.inc();
    info!("GRPC request successfully sent to the KMS!");
    Ok(())
}

/// Poll for result with timeout.
#[tracing::instrument(skip_all)]
async fn poll_for_result<T, F, Fut>(
    timeout: Duration,
    retry_interval: Duration,
    mut poll_fn: F,
    success_counter: &LazyLock<IntCounter>,
    error_counter: &LazyLock<IntCounter>,
) -> Result<Response<T>, Status>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<Response<T>, Status>>,
{
    let start = Instant::now();
    loop {
        info!("Trying to retrieve result from KMS Core...");
        match poll_fn().await {
            Ok(response) => {
                success_counter.inc();
                info!("Result successfully retrieved from KMS Core!");
                return Ok(response);
            }
            Err(status) => {
                if status.code() == Code::Unavailable {
                    // Check if we've exceeded the timeout
                    if start.elapsed() >= timeout {
                        error_counter.inc();
                        return Err(Status::deadline_exceeded(format!(
                            "Operation timed out after {timeout:?}"
                        )));
                    }
                    info!(
                        "Result was not ready, retrying in {}s...",
                        retry_interval.as_secs()
                    );
                    tokio::time::sleep(retry_interval).await;
                    continue;
                }
                // Any other error is returned immediately
                error_counter.inc();
                return Err(status);
            }
        }
    }
}
