use alloy::{hex, providers::Provider};
use chrono::Utc;
use fhevm_gateway_rust_bindings::decryption::Decryption::SnsCiphertextMaterial;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};
use tracing::{error, info, warn};

use crate::{
    core::{
        config::Config,
        coordination::{MessageScheduler, scheduler::BackpressureSignal},
        decryption::handler::DecryptionHandler,
        polling::{get_block_timestamp, remove_block_timestamp},
        utils::s3,
    },
    error::Result,
    gw_adapters::events::KmsCoreEvent,
};

/// Process events from the Gateway
pub struct EventProcessor<P> {
    decryption_handler: DecryptionHandler<P>,
    config: Config,
    provider: Arc<P>,
    shutdown: Option<broadcast::Receiver<()>>,
    message_scheduler: Option<Arc<MessageScheduler<P>>>,
    backpressure_rx: Option<broadcast::Receiver<BackpressureSignal>>,
}

impl<P: Provider + Clone + 'static> EventProcessor<P> {
    /// Create a new event processor
    pub async fn new(
        decryption_handler: DecryptionHandler<P>,
        config: Config,
        provider: Arc<P>,
        shutdown: broadcast::Receiver<()>,
    ) -> Result<Self> {
        // Create MessageScheduler if coordinated sending is enabled
        let (message_scheduler, backpressure_rx) = if config.enable_coordinated_sending {
            info!("🔂 Coordinated sending enabled - starting MessageScheduler");

            // Use the same shutdown receiver as the EventProcessor to coordinate shutdown
            let scheduler_shutdown_rx = shutdown.resubscribe();

            let (scheduler, backpressure_rx) = MessageScheduler::new(
                decryption_handler.clone(),
                config.clone(),
                provider.clone(),
                scheduler_shutdown_rx,
            );

            // IMPORTANT: Start the scheduler background task
            scheduler.start_scheduler().await?;

            (Some(Arc::new(scheduler)), Some(backpressure_rx))
        } else {
            (None, None)
        };

        Ok(Self {
            decryption_handler,
            config,
            provider,
            shutdown: Some(shutdown),
            message_scheduler,
            backpressure_rx,
        })
    }

    /// Get a backpressure receiver for polling system integration
    pub fn get_backpressure_receiver(&self) -> Option<broadcast::Receiver<BackpressureSignal>> {
        self.backpressure_rx.as_ref().map(|rx| rx.resubscribe())
    }

    /// Helper method to retrieve ciphertext materials from S3
    async fn retrieve_sns_ciphertext_materials(
        &self,
        sns_materials: Vec<SnsCiphertextMaterial>,
    ) -> Vec<(Vec<u8>, Vec<u8>)> {
        let s3_config = self.config.s3_config.clone();

        // Process all SNS ciphertext materials
        let mut sns_ciphertext_materials = Vec::new();
        for sns_material in sns_materials {
            let extracted_ct_handle = sns_material.ctHandle.to_vec();
            let extracted_sns_ciphertext_digest = sns_material.snsCiphertextDigest.to_vec();
            let coprocessor_addresses = sns_material.coprocessorTxSenderAddresses;

            // Get S3 URL and retrieve ciphertext
            // 1. For each SNS material, we try to retrieve its ciphertext from multiple possible S3 URLs
            // 2. Once we successfully retrieve a ciphertext from any of those URLs, we break out of the S3 URLs loop
            // 3. Then we continue processing the next SNS material in the outer loop
            let s3_urls = s3::prefetch_coprocessor_buckets(
                coprocessor_addresses,
                self.config.gateway_config_address,
                self.provider.clone(),
                s3_config.as_ref(),
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
                        break; // We want to stop as soon as ciphertext corresponding to extracted_sns_ciphertext_digest is retrieved
                    }
                    Err(error) => {
                        // Log warning but continue trying other URLs
                        warn!(
                            "Failed to retrieve ciphertext for digest {} from S3 URL {}: {}",
                            alloy::hex::encode(&extracted_sns_ciphertext_digest),
                            s3_url,
                            error
                        );
                        // Continue to the next URL
                    }
                }
            }

            if !ciphertext_retrieved {
                warn!(
                    "Failed to retrieve ciphertext for digest {} from any S3 URL",
                    alloy::hex::encode(&extracted_sns_ciphertext_digest)
                );
                // Continue to the next SNS material
            }
        }

        sns_ciphertext_materials
    }

    /// Process events from Gateway
    pub async fn process_gateway_events(
        &self,
        mut event_rx: mpsc::Receiver<KmsCoreEvent>,
    ) -> Result<()> {
        info!("Starting EVENTS processing...");

        let mut shutdown = self.shutdown.as_ref().unwrap().resubscribe();

        loop {
            tokio::select! {
                Some(event) = event_rx.recv() => {
                    let result = match event {
                        KmsCoreEvent::PublicDecryptionRequest(req) => {
                            info!(
                                "Processing PublicDecryptionRequest-{}",
                                req.decryptionId
                            );

                            // Extract keyId from the first SNS ciphertext material if available
                            let key_id = if !req.snsCtMaterials.is_empty() {
                                let extracted_key_id = req.snsCtMaterials.first().unwrap().keyId;
                                let key_id_hex = alloy::hex::encode(extracted_key_id.to_be_bytes::<32>());
                                info!(
                                    "Extracted key_id {} from snsCtMaterials[0] for PublicDecryptionRequest-{}",
                                    key_id_hex, req.decryptionId
                                );
                                key_id_hex
                            } else {
                                // Fail the request if no materials available
                                error!(
                                    "No snsCtMaterials found for PublicDecryptionRequest-{}, cannot proceed without a valid key_id",
                                    req.decryptionId
                                );
                                continue;
                            };

                            // Clone the request first to avoid borrow checker issues
                            let req_clone = req.clone();

                            // Retrieve ciphertext materials from S3
                            let sns_ciphertext_materials = self.retrieve_sns_ciphertext_materials(req.snsCtMaterials).await;

                            // If we couldn't retrieve any materials, fail the request
                            if sns_ciphertext_materials.is_empty() {
                                error!(
                                    "Failed to retrieve any ciphertext materials for PublicDecryptionRequest-{}",
                                    req_clone.decryptionId
                                );
                                continue;
                            }

                            // Use MessageScheduler if coordinated sending is enabled
                            if let Some(scheduler) = &self.message_scheduler {
                                info!("Scheduling PublicDecryptionRequest-{} for coordinated sending", req_clone.decryptionId);

                                // Get block timestamp for coordinated sending - this fixes the critical timing bug
                                let event_id = req_clone.decryptionId.to_string();
                                let block_timestamp = get_block_timestamp(&event_id)
                                    .unwrap_or_else(|| {
                                        warn!("No block timestamp found for event {}, using current UTC time", event_id);
                                        Utc::now().timestamp() as u64
                                    });

                                // Clean up timestamp to prevent memory leaks
                                remove_block_timestamp(&event_id);

                                scheduler.schedule_message(
                                    KmsCoreEvent::PublicDecryptionRequest(req_clone.clone()),
                                    block_timestamp,
                                ).await
                            } else {
                                // Immediate processing for non-coordinated mode
                                self.decryption_handler.handle_decryption_request_response(
                                    req.decryptionId,
                                    key_id,
                                    sns_ciphertext_materials,
                                    None,
                                    None,
                                )
                                .await
                            }
                        }

                        KmsCoreEvent::UserDecryptionRequest(req) => {
                            info!(
                                "Processing UserDecryptionRequest-{}",
                                req.decryptionId
                            );

                            // Extract keyId from the first SNS ciphertext material if available
                            let key_id = if !req.snsCtMaterials.is_empty() {
                                let extracted_key_id = req.snsCtMaterials.first().unwrap().keyId;
                                let key_id_hex = alloy::hex::encode(extracted_key_id.to_be_bytes::<32>());
                                info!(
                                    "Extracted key_id {} from snsCtMaterials[0] for UserDecryptionRequest-{}",
                                    key_id_hex, req.decryptionId
                                );
                                key_id_hex
                            } else {
                                // Fail the request if no materials available
                                error!(
                                    "No snsCtMaterials found for UserDecryptionRequest-{} (contract: {}), cannot proceed without a valid key_id",
                                    req.decryptionId, req.publicKey
                                );
                                continue;
                            };

                            // Clone the request first to avoid borrow checker issues
                            let req_clone = req.clone();

                            // Retrieve ciphertext materials from S3
                            let sns_ciphertext_materials = self.retrieve_sns_ciphertext_materials(req.snsCtMaterials).await;

                            // If we couldn't retrieve any materials, fail the request
                            if sns_ciphertext_materials.is_empty() {
                                error!(
                                    "Failed to retrieve any ciphertext materials for UserDecryptionRequest {}",
                                    req_clone.decryptionId
                                );
                                continue;
                            }

                            let user_key_prefixed = hex::encode_prefixed(req_clone.userAddress);

                            info!(
                                "UserDecryptionRequest-{} was received with userAddress: {}",
                                req_clone.decryptionId,
                                user_key_prefixed,
                            );

                            // Use MessageScheduler if coordinated sending is enabled
                            if let Some(scheduler) = &self.message_scheduler {
                                info!("Scheduling UserDecryptionRequest-{} for coordinated sending", req_clone.decryptionId);

                                // Get block timestamp for coordinated sending - this fixes the critical timing bug
                                let event_id = req_clone.decryptionId.to_string();
                                let block_timestamp = get_block_timestamp(&event_id)
                                    .unwrap_or_else(|| {
                                        warn!("No block timestamp found for event {}, using current UTC time", event_id);
                                        Utc::now().timestamp() as u64
                                    });

                                // Clean up timestamp to prevent memory leaks
                                remove_block_timestamp(&event_id);

                                scheduler.schedule_message(
                                    KmsCoreEvent::UserDecryptionRequest(req_clone.clone()),
                                    block_timestamp,
                                ).await
                            } else {
                                // Immediate processing for non-coordinated mode
                                match self.decryption_handler.handle_decryption_request_response(
                                    req.decryptionId,
                                    key_id,
                                    sns_ciphertext_materials,
                                    Some(req.userAddress),
                                    Some(req.publicKey)
                                ).await {
                                    Ok(_) => Ok(()),
                                    Err(e) => {
                                        error!(
                                            "Error processing UserDecryptionRequest-{}: {}",
                                            req.decryptionId, e
                                        );
                                        // Log error but continue processing other events
                                        Ok(())
                                    }
                                }
                            }
                        }
                        _ => Ok(()), // Ignore other events for now
                    };

                    if let Err(e) = result {
                        error!("Failed to process event: {}", e);
                        // Continue processing other events
                    }
                }
                _ = shutdown.recv() => {
                    info!("Received shutdown signal in event processor");
                    break;
                }
            }
        }

        info!("Event processing stopped");
        Ok(())
    }
}
