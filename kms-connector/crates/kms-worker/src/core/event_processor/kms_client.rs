use crate::core::{
    Config, config::KmsClientConfig, event_processor::eip712::verify_user_decryption_eip712,
};
use alloy::{hex, primitives::U256};
use anyhow::anyhow;
use connector_utils::{
    conn::{CONNECTION_RETRY_DELAY, CONNECTION_RETRY_NUMBER},
    types::{KmsGrpcRequest, KmsGrpcResponse, KmsResponse},
};
use kms_grpc::{
    kms::v1::{PublicDecryptionRequest, RequestId, UserDecryptionRequest},
    kms_service::v1::core_service_endpoint_client::CoreServiceEndpointClient,
    metastore_status::v1::{
        GetRequestStatusesRequest, ListRequestsRequest, MetaStoreType, RequestProcessingStatus,
        RequestStatusInfo, meta_store_status_service_client::MetaStoreStatusServiceClient,
    },
};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{
    select,
    sync::{RwLock, mpsc::Sender},
};
use tokio_util::sync::CancellationToken;
use tonic::{Request, Status, transport::Channel};
use tracing::{debug, error, info, warn};

/// The struct handling the communication with the KMS Core.
#[derive(Clone, Debug)]
pub struct KmsClient {
    /// The internal GRPC client used to query the core API of the KMS Core.
    core_client: CoreServiceEndpointClient<Channel>,

    /// The internal GRPC client used to query the meta store API of the KMS Core.
    meta_store_client: MetaStoreStatusServiceClient<Channel>,

    /// `Sender` channel used to send KMS Core's responses to the `KmsResponsePublisher`.
    sender: Sender<KmsResponse>,

    /// The configuraton of the `KmsClient`.
    config: KmsClientConfig,

    /// `HashMap` storing pending requests and the time the processing of the requests started.
    pending_requests: Arc<RwLock<HashMap<String, Instant>>>,
}

const META_STORE_TYPES: [MetaStoreType; 5] = [
    MetaStoreType::PublicDecryption,
    MetaStoreType::UserDecryption,
    MetaStoreType::Preprocessing,
    MetaStoreType::KeyGeneration,
    MetaStoreType::CrsGeneration,
];

impl KmsClient {
    /// Creates a new instance of `KmsClient`.
    pub fn new(channel: Channel, sender: Sender<KmsResponse>, config: KmsClientConfig) -> Self {
        let meta_store_client = MetaStoreStatusServiceClient::new(channel.clone());
        let core_client = CoreServiceEndpointClient::new(channel);
        Self {
            core_client,
            meta_store_client,
            sender,
            config,
            pending_requests: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Connects to the KMS Core.
    pub async fn connect(config: &Config, sender: Sender<KmsResponse>) -> anyhow::Result<Self> {
        let kms_core_endpoint = &config.kms_core_endpoint;
        let grpc_endpoint = Channel::from_shared(kms_core_endpoint.to_string())
            .map_err(|e| anyhow!("Invalid KMS Core endpoint {kms_core_endpoint}: {e}"))?;

        for i in 1..=CONNECTION_RETRY_NUMBER {
            info!("Attempting connection to KMS Core... ({i}/{CONNECTION_RETRY_NUMBER})");

            match grpc_endpoint.connect().await {
                Ok(channel) => {
                    info!("Connected to KMS Core at {kms_core_endpoint}");
                    let mut kms_client = Self::new(channel, sender, config.kms_client.clone());
                    kms_client.get_pending_requests().await;
                    return Ok(kms_client);
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

    /// Gets the list of pending requests from the KMS Core.
    pub async fn get_pending_requests(&mut self) {
        // Get pending requests via `ListRequests` API
        info!("Fetching list of pending requests from KMS Core...");

        for meta_store_type in META_STORE_TYPES {
            debug!("Fetching pending requests for meta store {meta_store_type:?}");
            let grpc_request = ListRequestsRequest {
                meta_store_type: meta_store_type as i32,
                status_filter: Some(RequestProcessingStatus::Any as i32),
                ..Default::default()
            };
            match self.meta_store_client.list_requests(grpc_request).await {
                Err(e) => warn!(
                    "Failed to fetch pending requests for meta store {:?}: {}",
                    meta_store_type, e
                ),
                Ok(response) => {
                    // Save time of requests collection
                    let now = Instant::now();
                    let requests = response.into_inner().requests;
                    debug!(
                        "Fetched {} pending requests from meta store {:?}!",
                        requests.len(),
                        meta_store_type
                    );
                    self.pending_requests
                        .write()
                        .await
                        .extend(requests.into_iter().map(|r| (r.request_id, now)));
                }
            };
        }
        info!(
            "Fetched {} pending requests!",
            self.pending_requests.read().await.len()
        );
    }

    /// Sends the GRPC request to the KMS Core.
    pub async fn send_request(&mut self, id: U256, request: KmsGrpcRequest) -> anyhow::Result<()> {
        match request {
            KmsGrpcRequest::PublicDecryption(request) => {
                self.request_public_decryption(request).await
            }
            KmsGrpcRequest::UserDecryption(request) => self.request_user_decryption(request).await,
        }?;

        self.pending_requests
            .write()
            .await
            .insert(hex::encode(id.to_be_bytes::<32>()), Instant::now());
        Ok(())
    }

    /// Sends a public decryption request to the KMS Core.
    async fn request_public_decryption(
        &mut self,
        request: PublicDecryptionRequest,
    ) -> anyhow::Result<()> {
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
        for i in 1..=self.config.request_retries {
            match self.core_client.public_decrypt(request.clone()).await {
                Ok(_) => break,
                Err(e) => {
                    warn!("GRPC PublicDecryptionRequest attempt #{i} failed: {e}");
                    if i == self.config.request_retries {
                        return Err(anyhow!("All GRPC PublicDecryptionRequest attempts failed!"));
                    }
                }
            }
        }
        Ok(())
    }

    /// Sends a user decryption request to the KMS Core.
    async fn request_user_decryption(
        &mut self,
        request: UserDecryptionRequest,
    ) -> anyhow::Result<()> {
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
        for i in 1..=self.config.request_retries {
            match self.core_client.user_decrypt(request.clone()).await {
                Ok(_) => break,
                Err(e) => {
                    warn!("GRPC UserDecryptionRequest attempt #{i} failed: {e}");
                    if i == self.config.request_retries {
                        return Err(anyhow!("All GRPC UserDecryptionRequest attempts failed!"));
                    }
                }
            }
        }
        Ok(())
    }

    /// Spawns a tasks dedicated to collection of requests' results.
    ///
    /// Results are collected every `self.config.poll_interval`.
    pub fn spawn_requests_results_collection(&self, cancel_token: CancellationToken) {
        let mut kms_client = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(kms_client.config.poll_interval);
            loop {
                select! {
                    _ = interval.tick() => (),
                    _ = cancel_token.cancelled() => break,
                }

                select! {
                    _ = kms_client.collect_completed_requests_results() => (),
                    _ = cancel_token.cancelled() => break,
                }
            }
        });
    }

    /// Collects the results of completed requests.
    ///
    /// Requests are considered as completed if their status is either `Completed` of `Failed`.
    async fn collect_completed_requests_results(&mut self) {
        if self.pending_requests.read().await.is_empty() {
            return debug!("No pending requests!");
        }

        // Get pending requests statuses via `GetRequestStatuses` API
        let request = GetRequestStatusesRequest {
            request_ids: self
                .pending_requests
                .read()
                .await
                .keys()
                .map(String::clone)
                .collect(),
            meta_store_type: None,
        };
        let response = match self.meta_store_client.get_request_statuses(request).await {
            Ok(r) => r.into_inner(),
            Err(e) => return error!("Failed to get pending requests statuses: {e}"),
        };

        // Filter completed or failed requests
        let completed_requests = response.statuses.into_iter().filter(|s| {
            s.status == RequestProcessingStatus::Completed as i32
                || s.status == RequestProcessingStatus::Failed as i32
        });

        // Fetch results for these requests
        for req in completed_requests {
            let mut kms_client = self.clone();
            tokio::spawn(async move { kms_client.handle_completed_request(req).await });
        }
    }

    /// Handles a completed request of the `KmsClient`.
    async fn handle_completed_request(&mut self, req: RequestStatusInfo) {
        info!("Trying to get result for request: {req:?}");
        match self.get_request_result(&req).await {
            // Sending response to `KmsResponsePublisher` via channel
            Ok(response) => match self.sender.send(response).await {
                Ok(()) => _ = self.pending_requests.write().await.remove(&req.request_id),
                Err(e) => error!("Failed to send response in channel: {e}"),
            },
            Err(e) => {
                error!("Failed to get request {} result: {}", req.request_id, e);

                // Remove response from hashmap if timeout has been reached
                if let Some(req_start) = self.pending_requests.read().await.get(&req.request_id) {
                    if req_start.elapsed() >= self.get_timeout(req.meta_store_type) {
                        error!("Timeout reached for {req:?}. Dropping the request");
                        self.pending_requests.write().await.remove(&req.request_id);
                    }
                }
            }
        }
    }

    /// Fetches the result of a completed request from the KMS Core.
    async fn get_request_result(
        &mut self,
        request_status: &RequestStatusInfo,
    ) -> anyhow::Result<KmsResponse> {
        // Prepare the GRPC request
        let request_id = request_status.request_id.clone();
        let grpc_request = Request::new(RequestId { request_id });

        // Send the request and gets the GRPC response
        let grpc_response = match MetaStoreType::try_from(request_status.meta_store_type) {
            Ok(MetaStoreType::PublicDecryption) => {
                let grpc_response = self
                    .core_client
                    .get_public_decryption_result(grpc_request)
                    .await?;
                KmsGrpcResponse::parse(request_status.request_id.as_str(), grpc_response)
            }
            Ok(MetaStoreType::UserDecryption) => {
                let grpc_response = self
                    .core_client
                    .get_user_decryption_result(grpc_request)
                    .await?;
                KmsGrpcResponse::parse(request_status.request_id.as_str(), grpc_response)
            }
            _ => unimplemented!(),
        }?;

        // Process the response accordingly
        if request_status.status == RequestProcessingStatus::Failed as i32 {
            let request_id = &request_status.request_id;
            return Err(anyhow!(
                "KMS Core failed to process request {request_id}: {grpc_response:?}"
            ));
        }
        KmsResponse::process(grpc_response)
    }

    /// Gets the `KmsClient` timeout associated to the meta store type.
    fn get_timeout(&self, meta_store_type: i32) -> Duration {
        match MetaStoreType::try_from(meta_store_type) {
            Ok(MetaStoreType::UserDecryption) => self.config.user_decryption_timeout,

            // Default value
            _ => self.config.public_decryption_timeout,
        }
    }
}
