use alloy::{hex, providers::Provider};
use fhevm_gateway_rust_bindings::decryption::Decryption::SnsCiphertextMaterial;
use std::sync::Arc;
use std::sync::OnceLock;
use tokio::sync::{Semaphore, broadcast};
use tokio::task::JoinSet;
use tracing::{error, info, warn};

use crate::{
    core::{
        config::Config, coordination::scheduler::MessageScheduler,
        decryption::handler::DecryptionHandler, utils::s3,
    },
    error::Result,
    gw_adapters::events::KmsCoreEvent,
};

/// Context for processing individual events in parallel tasks
struct TaskContext<P: Provider + Clone + 'static> {
    decryption_handler: DecryptionHandler<P>,
    message_scheduler: Option<Arc<MessageScheduler<P>>>,
    config: Config,
    provider: Arc<P>,
}

impl<P: Provider + Clone + 'static> TaskContext<P> {
    /// Process a single event (for parallel execution)
    async fn process_single_event(self, event: KmsCoreEvent) -> Result<()> {
        match event {
            KmsCoreEvent::PublicDecryptionRequest(req, block_timestamp) => {
                info!("Processing PublicDecryptionRequest: {}", req.decryptionId);

                // Extract keyId from the first SNS ciphertext material if available
                let key_id = if !req.snsCtMaterials.is_empty() {
                    let extracted_key_id = req.snsCtMaterials.first().unwrap().keyId;
                    let key_id_hex = alloy::hex::encode(extracted_key_id.to_be_bytes::<32>());
                    info!(
                        "Extracted key_id {} from snsCtMaterials[0] for public decryption request {}",
                        key_id_hex, req.decryptionId
                    );
                    key_id_hex
                } else {
                    // Fail the request if no materials available
                    error!(
                        "No snsCtMaterials found for public decryption request {}, cannot proceed without a valid key_id",
                        req.decryptionId
                    );
                    return Ok(()); // Return early
                };

                // Retrieve ciphertext materials from S3
                let sns_ciphertext_materials = self
                    .retrieve_sns_ciphertext_materials(req.snsCtMaterials)
                    .await;

                // If we couldn't retrieve any materials, fail the request
                if sns_ciphertext_materials.is_empty() {
                    error!(
                        "Failed to retrieve any ciphertext materials for public decryption request {}",
                        req.decryptionId
                    );
                    return Ok(()); // Return early
                }

                // Use message scheduler for coordinated sending if enabled
                if let Some(ref scheduler) = self.message_scheduler {
                    info!(
                        "Scheduling PublicDecryptionRequest {} for coordinated sending at block timestamp {}",
                        req.decryptionId, block_timestamp
                    );
                    match scheduler
                        .schedule_message(
                            req.decryptionId,
                            key_id,
                            sns_ciphertext_materials,
                            None,
                            None,
                            block_timestamp,
                        )
                        .await
                    {
                        Ok(_) => Ok(()),
                        Err(e) => {
                            error!(
                                "Error scheduling public decryption request {}: {}",
                                req.decryptionId, e
                            );
                            Ok(())
                        }
                    }
                } else {
                    // Send immediately if coordination is disabled
                    match self
                        .decryption_handler
                        .handle_decryption_request_response(
                            req.decryptionId,
                            key_id,
                            sns_ciphertext_materials,
                            None,
                            None,
                        )
                        .await
                    {
                        Ok(_) => Ok(()),
                        Err(e) => {
                            error!(
                                "Error processing public decryption request {}: {}",
                                req.decryptionId, e
                            );
                            Ok(())
                        }
                    }
                }
            }

            KmsCoreEvent::UserDecryptionRequest(req, block_timestamp) => {
                info!("Processing UserDecryptionRequest: {}", req.decryptionId);

                // Extract keyId from the first SNS ciphertext material if available
                let key_id = if !req.snsCtMaterials.is_empty() {
                    let extracted_key_id = req.snsCtMaterials.first().unwrap().keyId;
                    let key_id_hex = alloy::hex::encode(extracted_key_id.to_be_bytes::<32>());
                    info!(
                        "Extracted key_id {} from snsCtMaterials[0] for user decryption request {} (contract: {})",
                        key_id_hex, req.decryptionId, req.publicKey
                    );
                    key_id_hex
                } else {
                    // Fail the request if no materials available
                    error!(
                        "No snsCtMaterials found for user decryption request {} (contract: {}), cannot proceed without a valid key_id",
                        req.decryptionId, req.publicKey
                    );
                    return Ok(()); // Return early
                };

                // Retrieve ciphertext materials from S3
                let sns_ciphertext_materials = self
                    .retrieve_sns_ciphertext_materials(req.snsCtMaterials)
                    .await;

                // If we couldn't retrieve any materials, fail the request
                if sns_ciphertext_materials.is_empty() {
                    error!(
                        "Failed to retrieve any ciphertext materials for user decryption request {}",
                        req.decryptionId
                    );
                    return Ok(()); // Return early
                }

                let user_key_prefixed = hex::encode_prefixed(req.userAddress);
                let public_key_string = hex::encode_prefixed(&req.publicKey);

                info!(
                    "UserDecryptionRequest {} was received with:\nuserAddress: {}\npublicKey: {}\nkeyId: {}",
                    req.decryptionId, user_key_prefixed, public_key_string, key_id
                );

                // Use message scheduler for coordinated sending if enabled
                if let Some(ref scheduler) = self.message_scheduler {
                    info!(
                        "Scheduling UserDecryptionRequest {} for coordinated sending at block timestamp {}",
                        req.decryptionId, block_timestamp
                    );
                    match scheduler
                        .schedule_message(
                            req.decryptionId,
                            key_id,
                            sns_ciphertext_materials,
                            Some(req.userAddress),
                            Some(req.publicKey),
                            block_timestamp,
                        )
                        .await
                    {
                        Ok(_) => Ok(()),
                        Err(e) => {
                            error!(
                                "Error scheduling user decryption request {}: {}",
                                req.decryptionId, e
                            );
                            Ok(())
                        }
                    }
                } else {
                    // Send immediately if coordination is disabled
                    match self
                        .decryption_handler
                        .handle_decryption_request_response(
                            req.decryptionId,
                            key_id,
                            sns_ciphertext_materials,
                            Some(req.userAddress),
                            Some(req.publicKey),
                        )
                        .await
                    {
                        Ok(_) => Ok(()),
                        Err(e) => {
                            error!(
                                "Error processing user decryption request {}: {}",
                                req.decryptionId, e
                            );
                            Ok(())
                        }
                    }
                }
            }
            _ => Ok(()), // Ignore other events for now
        }
    }

    /// Get or create a shared S3 client (connection pooling)
    fn get_s3_client() -> &'static aws_sdk_s3::Client {
        static S3_CLIENT: OnceLock<aws_sdk_s3::Client> = OnceLock::new();
        S3_CLIENT.get_or_init(|| {
            // Initialize S3 client once and reuse across all requests
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    aws_sdk_s3::Client::new(
                        &aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await,
                    )
                })
            })
        })
    }

    /// Retrieve ciphertext materials from S3 with connection pooling
    async fn retrieve_sns_ciphertext_materials(
        &self,
        sns_materials: Vec<SnsCiphertextMaterial>,
    ) -> Vec<(Vec<u8>, Vec<u8>)> {
        let mut sns_ciphertext_materials = Vec::new();
        let _s3_client = Self::get_s3_client(); // Reuse pooled connection

        for sns_material in sns_materials {
            let extracted_ct_handle = sns_material.ctHandle.to_vec();
            let extracted_sns_ciphertext_digest = sns_material.snsCiphertextDigest.to_vec();
            let coprocessor_addresses = sns_material.coprocessorTxSenderAddresses;

            // Get S3 URL and retrieve ciphertext
            let s3_urls = s3::prefetch_coprocessor_buckets(
                coprocessor_addresses,
                self.config.gateway_config_address,
                self.provider.clone(),
                self.config.s3_config.as_ref(),
            )
            .await;

            if s3_urls.is_empty() {
                warn!(
                    "No S3 URLs found for ciphertext digest {}",
                    alloy::hex::encode(&extracted_sns_ciphertext_digest)
                );
                continue;
            }

            let mut ciphertext_retrieved = false;
            for s3_url in s3_urls {
                match s3::retrieve_s3_ciphertext(
                    s3_url.clone(),
                    extracted_sns_ciphertext_digest.clone(),
                )
                .await
                {
                    Ok(ciphertext) => {
                        info!(
                            "Successfully retrieved ciphertext for digest {} from S3 URL {}",
                            alloy::hex::encode(&extracted_sns_ciphertext_digest),
                            s3_url
                        );
                        sns_ciphertext_materials.push((extracted_ct_handle.clone(), ciphertext));
                        ciphertext_retrieved = true;
                        break;
                    }
                    Err(error) => {
                        warn!(
                            "Failed to retrieve ciphertext for digest {} from S3 URL {}: {}",
                            alloy::hex::encode(&extracted_sns_ciphertext_digest),
                            s3_url,
                            error
                        );
                    }
                }
            }

            if !ciphertext_retrieved {
                warn!(
                    "Failed to retrieve ciphertext for digest {} from any S3 URL",
                    alloy::hex::encode(&extracted_sns_ciphertext_digest)
                );
            }
        }

        sns_ciphertext_materials
    }
}

/// Process events from the Gateway
pub struct EventProcessor<P> {
    decryption_handler: DecryptionHandler<P>,
    message_scheduler: Option<Arc<MessageScheduler<P>>>,
    config: Config,
    provider: Arc<P>,
    shutdown: Option<broadcast::Receiver<()>>,
    /// Semaphore to limit concurrent task execution and prevent memory exhaustion
    task_semaphore: Arc<Semaphore>,
    /// Active tasks for graceful shutdown and monitoring
    active_tasks: JoinSet<()>,
}

impl<P: Provider + Clone + 'static> EventProcessor<P> {
    /// Create a new event processor
    pub fn new(
        decryption_handler: DecryptionHandler<P>,
        config: Config,
        provider: Arc<P>,
        shutdown: broadcast::Receiver<()>,
    ) -> Self {
        // Initialize message scheduler if coordinated sending is enabled
        let message_scheduler = if config.enable_coordinated_sending {
            Some(Arc::new(MessageScheduler::new(
                config.clone(),
                decryption_handler.clone(),
                shutdown.resubscribe(),
            )))
        } else {
            None
        };

        const MAX_CONCURRENT_TASKS: usize = 100; // Prevent task explosion

        Self {
            decryption_handler,
            message_scheduler,
            config,
            provider,
            shutdown: Some(shutdown),
            task_semaphore: Arc::new(Semaphore::new(MAX_CONCURRENT_TASKS)),
            active_tasks: JoinSet::new(),
        }
    }

    /// Process events from Gateway using broadcast channel for parallel processing
    pub async fn process_gateway_events(
        &mut self,
        mut event_rx: broadcast::Receiver<KmsCoreEvent>,
    ) -> Result<()> {
        info!("Starting parallel event processing...");

        let mut shutdown = self.shutdown.as_ref().unwrap().resubscribe();
        let active_tasks = &mut self.active_tasks;

        loop {
            tokio::select! {
                Ok(event) = event_rx.recv() => {
                    // Acquire semaphore permit to limit concurrent tasks (backpressure)
                    let permit = match self.task_semaphore.clone().try_acquire_owned() {
                        Ok(permit) => permit,
                        Err(_) => {
                            warn!("Task queue at capacity, waiting for available slot");
                            self.task_semaphore.clone().acquire_owned().await.unwrap()
                        }
                    };

                    // Spawn a task to process this event in parallel
                    let task_context = TaskContext {
                        decryption_handler: self.decryption_handler.clone(),
                        message_scheduler: self.message_scheduler.clone(),
                        config: self.config.clone(),
                        provider: self.provider.clone(),
                    };

                    active_tasks.spawn(async move {
                        let _permit = permit; // Hold permit until task completes
                        if let Err(e) = task_context.process_single_event(event).await {
                            error!("Failed to process event: {}", e);
                        }
                        // Permit automatically released when _permit is dropped
                    });
                }
                Some(task_result) = active_tasks.join_next() => {
                    // Handle completed tasks
                    match task_result {
                        Ok(()) => {
                            // Task completed successfully
                        }
                        Err(e) => {
                            error!("Event processing task panicked: {}", e);
                        }
                    }
                }
                _ = shutdown.recv() => {
                    info!("Received shutdown signal in event processor");
                    break;
                }
            }
        }

        // Wait for all active tasks to complete during shutdown
        info!(
            "Waiting for {} active event processing tasks to complete...",
            active_tasks.len()
        );
        while let Some(task_result) = active_tasks.join_next().await {
            match task_result {
                Ok(()) => {}
                Err(e) => error!("Event processing task panicked during shutdown: {}", e),
            }
        }

        info!("Event processing stopped");
        Ok(())
    }
}
