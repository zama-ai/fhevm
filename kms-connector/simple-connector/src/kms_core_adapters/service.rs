use kms_grpc::{
    kms::v1::{
        PublicDecryptionRequest, PublicDecryptionResponse, UserDecryptionRequest,
        UserDecryptionResponse,
    },
    kms_service::v1::core_service_endpoint_client::CoreServiceEndpointClient,
};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tonic::{Code, Request, Response, Status, transport::Channel};
use tracing::{error, info, warn};

use crate::core::config::Config;
use crate::core::types::fhe_types::fhe_type_to_string;
use crate::core::utils::eip712::verify_user_decryption_eip712;
use crate::error::Result;

/// Convert hex RequestId string to decimal for consistent logging
#[inline]
fn request_id_to_decimal(hex_request_id: &str) -> String {
    // Try to parse hex string as U256 and convert to decimal
    if let Ok(bytes) = alloy::hex::decode(hex_request_id) {
        if bytes.len() == 32 {
            let mut array = [0u8; 32];
            array.copy_from_slice(&bytes);
            return alloy::primitives::U256::from_be_bytes(array).to_string();
        }
    }
    // Fallback to original hex string if parsing fails
    hex_request_id.to_string()
}

#[tonic::async_trait]
pub trait KmsService {
    async fn request_public_decryption(
        &self,
        request: Request<PublicDecryptionRequest>,
    ) -> std::result::Result<Response<PublicDecryptionResponse>, Status>;

    async fn request_user_decryption(
        &self,
        request: Request<UserDecryptionRequest>,
    ) -> std::result::Result<Response<UserDecryptionResponse>, Status>;
}

#[derive(Debug)]
pub struct KmsServiceImpl {
    kms_core_endpoints: Vec<String>,
    running: Arc<AtomicBool>,
    clients: Arc<tokio::sync::Mutex<Vec<Option<CoreServiceEndpointClient<Channel>>>>>,
    config: Config,
}

impl KmsServiceImpl {
    /// Create a new KMS service instance
    pub fn new(config: Config) -> Self {
        let num_endpoints = config.kms_core_endpoints.len();
        Self {
            kms_core_endpoints: config.kms_core_endpoints.clone(),
            running: Arc::new(AtomicBool::new(true)),
            clients: Arc::new(tokio::sync::Mutex::new(vec![None; num_endpoints])),
            config,
        }
    }

    /// Initialize all KMS client connections
    pub async fn initialize(&self) -> Result<()> {
        let mut clients_guard = self.clients.lock().await;

        for (i, endpoint) in self.kms_core_endpoints.iter().enumerate() {
            let channel = Channel::from_shared(endpoint.clone())
                .map_err(|e| crate::error::Error::Transport(e.to_string()))?
                .connect()
                .await
                .map_err(|e| crate::error::Error::Transport(e.to_string()))?;

            clients_guard[i] = Some(CoreServiceEndpointClient::new(channel));
            info!("Connected to KMS-core at {}", endpoint);
        }

        Ok(())
    }

    /// Get a client for a specific request ID using modulo sharding
    async fn get_client_for_request(
        &self,
        request_id: &str,
    ) -> Result<CoreServiceEndpointClient<Channel>> {
        // Simple hash of request ID for modulo calculation
        let request_id_hash = request_id.chars().map(|c| c as u64).sum::<u64>();

        // Select endpoint using modulo sharding
        let endpoint_index = (request_id_hash % self.kms_core_endpoints.len() as u64) as usize;

        loop {
            {
                let clients_guard = self.clients.lock().await;
                if let Some(client) = clients_guard[endpoint_index].clone() {
                    return Ok(client);
                }
            }

            // No client available, try to connect
            match self.initialize().await {
                Ok(_) => continue, // Client is now initialized, try to get it
                Err(e) => {
                    error!("Failed to connect to KMS-core: {}, retrying...", e);
                    sleep(self.config.retry_interval).await;
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
        let retry_interval = self.config.retry_interval;

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
    async fn request_public_decryption(
        &self,
        request: Request<PublicDecryptionRequest>,
    ) -> std::result::Result<Response<PublicDecryptionResponse>, Status> {
        if !self.running.load(Ordering::SeqCst) {
            return Err(Status::cancelled("Service is shutting down"));
        }

        let request_id = request
            .get_ref()
            .request_id
            .clone()
            .ok_or_else(|| Status::invalid_argument("Missing request ID"))?;

        // Log the FHE types being processed in this request
        let request_id_decimal = request_id_to_decimal(&request_id.request_id);
        if let Some(ciphertexts) = request.get_ref().ciphertexts.as_slice().first() {
            info!(
                "[SENDING] PublicDecryptionRequest-{} with FHE type: {}",
                request_id_decimal,
                fhe_type_to_string(ciphertexts.fhe_type)
            );
        } else {
            info!(
                "[SENDING] PublicDecryptionRequest-{} with no ciphertexts",
                request_id_decimal
            );
        }

        let mut client = self
            .get_client_for_request(&request_id.request_id)
            .await
            .map_err(|e| Status::unavailable(format!("Failed to get KMS client: {e}")))?;

        // Send initial request
        client.public_decrypt(request).await?;

        // Poll for result with timeout
        self.poll_for_result(self.config.public_decryption_timeout, || {
            let request = Request::new(request_id.clone());
            let request_id_str = request_id.request_id.clone();
            async move {
                let mut client = self
                    .get_client_for_request(&request_id_str)
                    .await
                    .map_err(|e| Status::unavailable(format!("Failed to get KMS client: {e}")))?;
                client.get_public_decryption_result(request).await
            }
        })
        .await
    }

    async fn request_user_decryption(
        &self,
        request: Request<UserDecryptionRequest>,
    ) -> std::result::Result<Response<UserDecryptionResponse>, Status> {
        if !self.running.load(Ordering::SeqCst) {
            return Err(Status::cancelled("Service is shutting down"));
        }

        let request_id = request
            .get_ref()
            .request_id
            .clone()
            .ok_or_else(|| Status::invalid_argument("Missing request ID"))?;

        // Verify the EIP-712 signature for the user decryption request
        if let Err(e) = verify_user_decryption_eip712(request.get_ref()) {
            error!(
                "Failed to verify UserDecryptionRequest-{}: {e}",
                request_id_to_decimal(&request_id.request_id)
            );
            warn!("Proceeding with user decryption despite verification failure: {e}");
        }

        // Log the client address and FHE types being processed
        let fhe_types = request
            .get_ref()
            .typed_ciphertexts
            .iter()
            .map(|ct| fhe_type_to_string(ct.fhe_type))
            .collect::<Vec<_>>()
            .join(", ");

        let request_id_decimal = request_id_to_decimal(&request_id.request_id);
        info!(
            "[SENDING] UserDecryptionRequest-{} for client {} with FHE types: [{}]",
            request_id_decimal,
            request.get_ref().client_address,
            fhe_types
        );

        let mut client = self
            .get_client_for_request(&request_id.request_id)
            .await
            .map_err(|e| Status::unavailable(format!("Failed to get KMS client: {e}")))?;

        // Send initial request
        client.user_decrypt(request).await?;

        // Poll for result with timeout
        self.poll_for_result(self.config.user_decryption_timeout, || {
            let request = Request::new(request_id.clone());
            let request_id_str = request_id.request_id.clone();
            async move {
                let mut client = self
                    .get_client_for_request(&request_id_str)
                    .await
                    .map_err(|e| Status::unavailable(format!("Failed to get KMS client: {e}")))?;
                client.get_user_decryption_result(request).await
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
