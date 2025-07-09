use crate::core::{Config, event_processor::eip712::verify_user_decryption_eip712};
use anyhow::anyhow;
use connector_utils::{
    conn::{CONNECTION_RETRY_DELAY, CONNECTION_RETRY_NUMBER},
    types::{KmsGrpcRequest, KmsGrpcResponse},
};
use kms_grpc::{
    kms::v1::{PublicDecryptionRequest, UserDecryptionRequest},
    kms_service::v1::core_service_endpoint_client::CoreServiceEndpointClient,
};
use std::time::{Duration, Instant};
use tonic::{Code, Request, Response, Status, transport::Channel};
use tracing::{error, info, warn};

/// The struct handling the communication with the KMS Core.
#[derive(Clone, Debug)]
pub struct KmsClient {
    /// The internal KMS Core client from the `kms_grpc` crate.
    inner: CoreServiceEndpointClient<Channel>,

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
    /// Creates a new instance of `KmsClient`.
    pub fn new(
        channel: Channel,
        grpc_request_retries: u8,
        public_decryption_timeout: Duration,
        user_decryption_timeout: Duration,
        grpc_poll_interval: Duration,
    ) -> Self {
        let inner = CoreServiceEndpointClient::new(channel);
        Self {
            inner,
            grpc_request_retries,
            public_decryption_timeout,
            user_decryption_timeout,
            grpc_poll_interval,
        }
    }

    /// Connects to the KMS Core.
    pub async fn connect(config: &Config) -> anyhow::Result<Self> {
        let kms_core_endpoint = &config.kms_core_endpoint;
        let grpc_endpoint = Channel::from_shared(kms_core_endpoint.to_string())
            .map_err(|e| anyhow!("Invalid KMS Core endpoint {kms_core_endpoint}: {e}"))?;

        for i in 1..=CONNECTION_RETRY_NUMBER {
            info!("Attempting connection to KMS Core... ({i}/{CONNECTION_RETRY_NUMBER})");

            match grpc_endpoint.connect().await {
                Ok(channel) => {
                    info!("Connected to KMS Core at {kms_core_endpoint}");
                    return Ok(Self::new(
                        channel,
                        config.grpc_request_retries,
                        config.public_decryption_timeout,
                        config.user_decryption_timeout,
                        config.grpc_poll_interval,
                    ));
                }
                Err(e) => warn!("KMS Core connection attempt #{i} failed: {e}"),
            }

            if i != CONNECTION_RETRY_NUMBER {
                tokio::time::sleep(CONNECTION_RETRY_DELAY).await;
            }
        }

        Err(anyhow!(
            "Could not connect to KMS Core at {kms_core_endpoint}"
        ))
    }

    /// Sends the GRPC request to the KMS Core.
    pub async fn send_request(
        &mut self,
        request: KmsGrpcRequest,
    ) -> anyhow::Result<KmsGrpcResponse> {
        match request {
            KmsGrpcRequest::PublicDecryption(request) => {
                self.request_public_decryption(request).await
            }
            KmsGrpcRequest::UserDecryption(request) => self.request_user_decryption(request).await,
        }
    }

    /// Sends a public decryption request to the KMS Core.
    async fn request_public_decryption(
        &mut self,
        request: PublicDecryptionRequest,
    ) -> anyhow::Result<KmsGrpcResponse> {
        let request_id = request
            .request_id
            .clone()
            .ok_or_else(|| Status::invalid_argument("Missing request ID"))?;

        // Log the FHE types being processed in this request
        if let Some(ciphertexts) = request.ciphertexts.as_slice().first() {
            info!(
                "[OUT] 🔑 Sending PublicDecryptionRequest({}) with FHE type: {}",
                request_id.request_id, ciphertexts.fhe_type
            );
        } else {
            info!(
                "[OUT] Sending PublicDecryptionRequest({}) with no ciphertexts",
                request_id.request_id
            );
        }

        // Send initial request with retries
        for i in 1..=self.grpc_request_retries {
            match self.inner.public_decrypt(request.clone()).await {
                Ok(_) => break,
                Err(e) => {
                    warn!("GRPC PublicDecryptionRequest attempt #{i} failed: {e}");
                    if i == self.grpc_request_retries {
                        return Err(anyhow!("All GRPC PublicDecryptionRequest attempts failed!"));
                    }
                }
            }
        }

        // Poll for result with timeout
        let grpc_response = poll_for_result(
            self.public_decryption_timeout,
            self.grpc_poll_interval,
            || {
                let request = Request::new(request_id.clone());
                let mut inner_client = self.inner.clone();
                async move { inner_client.get_public_decryption_result(request).await }
            },
        )
        .await?;

        KmsGrpcResponse::try_from((request_id, grpc_response))
    }

    /// Sends a user decryption request to the KMS Core.
    async fn request_user_decryption(
        &mut self,
        request: UserDecryptionRequest,
    ) -> anyhow::Result<KmsGrpcResponse> {
        let request_id = request
            .request_id
            .clone()
            .ok_or_else(|| Status::invalid_argument("Missing request ID"))?;

        // Verify the EIP-712 signature for the user decryption request
        if let Err(e) = verify_user_decryption_eip712(&request) {
            error!("Failed to verify user decryption request: {e}");
            warn!("Proceeding with user decryption despite verification failure: {e}");
        }

        // Log the client address and FHE types being processed
        let fhe_types = request
            .typed_ciphertexts
            .iter()
            .map(|ct| ct.fhe_type.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        info!(
            "[OUT] 🔑 Sending UserDecryptionRequest({}) for client {} with FHE types: [{}]",
            request_id.request_id, request.client_address, fhe_types
        );

        // Send initial request with retries
        for i in 1..=self.grpc_request_retries {
            match self.inner.user_decrypt(request.clone()).await {
                Ok(_) => break,
                Err(e) => {
                    warn!("GRPC UserDecryptionRequest attempt #{i} failed: {e}");
                    if i == self.grpc_request_retries {
                        return Err(anyhow!("All GRPC UserDecryptionRequest attempts failed!"));
                    }
                }
            }
        }

        // Poll for result with timeout
        let grpc_response = poll_for_result(
            self.user_decryption_timeout,
            self.grpc_poll_interval,
            || {
                let request = Request::new(request_id.clone());
                let mut inner_client = self.inner.clone();
                async move { inner_client.get_user_decryption_result(request).await }
            },
        )
        .await?;

        KmsGrpcResponse::try_from((request_id, grpc_response))
    }
}

/// Poll for result with timeout
async fn poll_for_result<T, F, Fut>(
    timeout: Duration,
    retry_interval: Duration,
    mut poll_fn: F,
) -> Result<Response<T>, Status>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<Response<T>, Status>>,
{
    let start = Instant::now();
    loop {
        match poll_fn().await {
            Ok(response) => return Ok(response),
            Err(status) => {
                if status.code() == Code::NotFound {
                    // Check if we've exceeded the timeout
                    if start.elapsed() >= timeout {
                        return Err(Status::deadline_exceeded(format!(
                            "Operation timed out after {timeout:?}"
                        )));
                    }
                    // Result not ready yet, wait and retry
                    tokio::time::sleep(retry_interval).await;
                    continue;
                }
                // Any other error is returned immediately
                return Err(status);
            }
        }
    }
}
