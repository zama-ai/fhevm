use kms_grpc::{
    kms::v1::{DecryptionRequest, DecryptionResponse, ReencryptionRequest, ReencryptionResponse},
    kms_service::v1::core_service_endpoint_client::CoreServiceEndpointClient,
};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tonic::{transport::Channel, Code, Request, Response, Status};
use tracing::{error, info, warn};

use crate::core::config::Config;
use crate::core::utils::eip712::verify_reencryption_eip712;
use crate::error::Result;

#[tonic::async_trait]
pub trait KmsService {
    async fn request_decryption(
        &self,
        request: Request<DecryptionRequest>,
    ) -> std::result::Result<Response<DecryptionResponse>, Status>;

    async fn request_reencryption(
        &self,
        request: Request<ReencryptionRequest>,
    ) -> std::result::Result<Response<ReencryptionResponse>, Status>;
}

#[derive(Debug)]
pub struct KmsServiceImpl {
    kms_core_endpoint: String,
    running: Arc<AtomicBool>,
    client: Arc<tokio::sync::Mutex<Option<CoreServiceEndpointClient<Channel>>>>,
    config: Config,
}

impl KmsServiceImpl {
    /// Create a new KMS service instance
    pub fn new(kms_core_endpoint: &str, config: Config) -> Self {
        Self {
            kms_core_endpoint: kms_core_endpoint.to_string(),
            running: Arc::new(AtomicBool::new(true)),
            client: Arc::new(tokio::sync::Mutex::new(None)),
            config,
        }
    }

    /// Initialize the KMS client connection
    pub async fn initialize(&self) -> Result<()> {
        let channel = Channel::from_shared(self.kms_core_endpoint.clone())
            .map_err(|e| crate::error::Error::Transport(e.to_string()))?
            .connect()
            .await
            .map_err(|e| crate::error::Error::Transport(e.to_string()))?;

        let mut client_guard = self.client.lock().await;
        *client_guard = Some(CoreServiceEndpointClient::new(channel));
        info!("Connected to KMS-core at {}", self.kms_core_endpoint);
        Ok(())
    }

    /// Get a client, attempting to reconnect if necessary
    async fn get_client(&self) -> Result<CoreServiceEndpointClient<Channel>> {
        loop {
            {
                let client_guard = self.client.lock().await;
                if let Some(client) = client_guard.clone() {
                    return Ok(client);
                }
            }

            // No client available, try to connect
            match self.initialize().await {
                Ok(_) => continue, // Client is now initialized, try to get it
                Err(e) => {
                    error!("Failed to connect to KMS-core: {}, retrying...", e);
                    sleep(self.config.retry_interval()).await;
                }
            }
        }
    }

    /// Poll for result with timeout
    async fn poll_for_result<T, F, Fut>(
        &self,
        timeout: Duration,
        mut poll_fn: F,
    ) -> std::result::Result<Response<T>, Status>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = std::result::Result<Response<T>, Status>>,
    {
        let start = Instant::now();
        let retry_interval = self.config.retry_interval();

        loop {
            match poll_fn().await {
                Ok(response) => return Ok(response),
                Err(status) => {
                    if status.code() == Code::NotFound {
                        // Check if we've exceeded the timeout
                        if start.elapsed() >= timeout {
                            return Err(Status::deadline_exceeded(format!(
                                "Operation timed out after {:?}",
                                timeout
                            )));
                        }
                        // Result not ready yet, wait and retry
                        sleep(retry_interval).await;
                        continue;
                    }
                    // Any other error is returned immediately
                    return Err(status);
                }
            }
        }
    }
}

#[tonic::async_trait]
impl KmsService for KmsServiceImpl {
    async fn request_decryption(
        &self,
        request: Request<DecryptionRequest>,
    ) -> std::result::Result<Response<DecryptionResponse>, Status> {
        if !self.running.load(Ordering::SeqCst) {
            return Err(Status::cancelled("Service is shutting down"));
        }

        let request_id = request
            .get_ref()
            .request_id
            .clone()
            .ok_or_else(|| Status::invalid_argument("Missing request ID"))?;

        // Log the FHE types being processed in this request
        if let Some(ciphertexts) = request.get_ref().ciphertexts.as_slice().first() {
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

        let mut client = self
            .get_client()
            .await
            .map_err(|e| Status::unavailable(format!("Failed to get KMS client: {}", e)))?;

        // Send initial request
        client.decrypt(request).await?;

        // Poll for result with timeout
        let timeout = self.config.decryption_timeout();

        self.poll_for_result(timeout, || {
            let request = Request::new(request_id.clone());
            async move {
                let mut client = self
                    .get_client()
                    .await
                    .map_err(|e| Status::unavailable(format!("Failed to get KMS client: {}", e)))?;
                client.get_decrypt_result(request).await
            }
        })
        .await
    }

    async fn request_reencryption(
        &self,
        request: Request<ReencryptionRequest>,
    ) -> std::result::Result<Response<ReencryptionResponse>, Status> {
        if !self.running.load(Ordering::SeqCst) {
            return Err(Status::cancelled("Service is shutting down"));
        }

        let request_id = request
            .get_ref()
            .request_id
            .clone()
            .ok_or_else(|| Status::invalid_argument("Missing request ID"))?;

        // Verify the EIP-712 signature for the reencryption request
        if let Err(e) = verify_reencryption_eip712(request.get_ref()) {
            error!("Failed to verify reencryption request: {}", e);
            warn!(
                "Proceeding with reencryption despite verification failure: {}",
                e
            );
        }

        // Log the client address and FHE types being processed
        let fhe_types = request
            .get_ref()
            .typed_ciphertexts
            .iter()
            .map(|ct| ct.fhe_type.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        info!(
            "[OUT] 🔑 Sending ReencryptionRequest({}) for client {} with FHE types: [{}]",
            request_id.request_id,
            request.get_ref().client_address,
            fhe_types
        );

        let mut client = self
            .get_client()
            .await
            .map_err(|e| Status::unavailable(format!("Failed to get KMS client: {}", e)))?;

        // Send initial request
        client.reencrypt(request).await?;

        // Poll for result with timeout
        let timeout = self.config.reencryption_timeout();

        self.poll_for_result(timeout, || {
            let request = Request::new(request_id.clone());
            async move {
                let mut client = self
                    .get_client()
                    .await
                    .map_err(|e| Status::unavailable(format!("Failed to get KMS client: {}", e)))?;
                client.get_reencrypt_result(request).await
            }
        })
        .await
    }
}

impl KmsServiceImpl {
    /// Stop the KMS service
    pub fn stop(&self) {
        info!("Stopping KMS service...");
        self.running.store(false, Ordering::SeqCst);
    }

    /// Get the config object
    pub fn config(&self) -> &Config {
        &self.config
    }
}
