use alloy::{hex, providers::Provider};
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};
use tracing::{debug, error, info};

use crate::{
    core::{
        backpressure::BackpressureSignal, config::Config, decryption::handler::DecryptionHandler,
        polling::remove_block_timestamp, utils::s3_client::S3Client,
    },
    error::Result,
    gw_adapters::events::KmsCoreEvent,
};

/// Process events from the Gateway
pub struct EventProcessor<P> {
    decryption_handler: DecryptionHandler<P>,
    s3_client: S3Client,
    shutdown: Option<broadcast::Receiver<()>>,
    // Backpressure receiver for polling system integration
    backpressure_rx: Option<broadcast::Receiver<BackpressureSignal>>,
}

impl<P: Provider + Clone + 'static> EventProcessor<P> {
    /// Create a new event processor
    pub async fn new(
        decryption_handler: DecryptionHandler<P>,
        config: Config,
        _provider: Arc<P>,
        shutdown: broadcast::Receiver<()>,
        backpressure_rx: Option<broadcast::Receiver<BackpressureSignal>>,
    ) -> Result<Self> {
        let s3_client = S3Client::new(config.s3_config.clone());

        Ok(Self {
            decryption_handler,
            s3_client,
            shutdown: Some(shutdown),
            backpressure_rx,
        })
    }

    /// Get a backpressure receiver for polling system integration
    pub fn get_backpressure_receiver(&self) -> Option<broadcast::Receiver<BackpressureSignal>> {
        self.backpressure_rx.as_ref().map(|rx| rx.resubscribe())
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
                    // Log event reception immediately
                    match &event {
                        KmsCoreEvent::PublicDecryptionRequest(req) => {
                            info!("[RECEIVED] PublicDecryptionRequest-{}", req.decryptionId);
                        }
                        KmsCoreEvent::UserDecryptionRequest(req) => {
                            info!("[RECEIVED] UserDecryptionRequest-{}", req.decryptionId);
                        }
                        _ => {
                            info!("[RECEIVED] Other event type");
                        }
                    }

                    // Spawn concurrent task for processing
                    let decryption_handler = self.decryption_handler.clone();
                    let s3_client = self.s3_client.clone();
                    tokio::spawn(async move {
                        if let Err(e) = Self::process_single_event(decryption_handler, event, s3_client).await {
                            error!("Event processing failed: {}", e);
                        }
                    });

                    // Continue immediately to receive next event
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

    /// Process a single event concurrently (static method to avoid self borrowing issues)
    async fn process_single_event<P2: Provider + Clone + 'static>(
        decryption_handler: DecryptionHandler<P2>,
        event: KmsCoreEvent,
        s3_client: S3Client,
    ) -> Result<()> {
        match event {
            KmsCoreEvent::PublicDecryptionRequest(req) => {
                // Extract keyId from the first SNS ciphertext material if available (CONVENTION)
                let key_id = if !req.snsCtMaterials.is_empty() {
                    let extracted_key_id = req.snsCtMaterials.first().unwrap().keyId;
                    let key_id_hex = alloy::hex::encode(extracted_key_id.to_be_bytes::<32>());
                    debug!(
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
                    return Err(crate::error::Error::InvalidRequestType(
                        "No snsCtMaterials found".to_string(),
                    ));
                };

                let req_clone = req.clone();

                // Retrieve ciphertext materials from S3
                let sns_ciphertext_materials = match s3_client
                    .retrieve_ciphertext_materials(
                        req.snsCtMaterials,
                        decryption_handler.gateway_config_address(),
                        decryption_handler.provider(),
                    )
                    .await
                {
                    Ok(materials) => materials,
                    Err(e) => {
                        error!(
                            "Failed to retrieve ciphertext materials for PublicDecryptionRequest-{}: {}",
                            req_clone.decryptionId, e
                        );
                        return Err(e);
                    }
                };

                // If we couldn't retrieve any materials, log error and skip this request
                if sns_ciphertext_materials.is_empty() {
                    error!(
                        "Failed to retrieve any ciphertext materials for PublicDecryptionRequest-{} - skipping this request",
                        req_clone.decryptionId
                    );
                    return Ok(()); // Continue processing other events
                }

                // Process decryption directly since we already have S3 materials
                info!(
                    "[PROCESSING] PublicDecryptionRequest-{} with pre-fetched S3 materials",
                    req_clone.decryptionId
                );

                // Clean up timestamp to prevent memory leaks
                let event_id = req_clone.decryptionId.to_string();
                remove_block_timestamp(&event_id);

                // Handle decryption directly with the S3 materials we just fetched
                if let Err(e) = decryption_handler
                    .handle_decryption_request_response(
                        req.decryptionId,
                        key_id,
                        sns_ciphertext_materials,
                        None, // client_addr is None for public requests
                        None, // public_key is None for public requests
                    )
                    .await
                {
                    error!(
                        "Error processing PublicDecryptionRequest-{}: {}",
                        req.decryptionId, e
                    );
                }
            }

            KmsCoreEvent::UserDecryptionRequest(req) => {
                // Extract keyId from the first SNS ciphertext material if available (CONVENTION)
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
                    return Err(crate::error::Error::InvalidRequestType(
                        "No snsCtMaterials found".to_string(),
                    ));
                };

                let req_clone = req.clone();

                // Retrieve ciphertext materials from S3
                let sns_ciphertext_materials = match s3_client
                    .retrieve_ciphertext_materials(
                        req.snsCtMaterials,
                        decryption_handler.gateway_config_address(),
                        decryption_handler.provider(),
                    )
                    .await
                {
                    Ok(materials) => materials,
                    Err(e) => {
                        error!(
                            "Failed to retrieve ciphertext materials for UserDecryptionRequest-{}: {}",
                            req_clone.decryptionId, e
                        );
                        return Err(e);
                    }
                };

                // If we couldn't retrieve any materials, log error and skip this request
                if sns_ciphertext_materials.is_empty() {
                    error!(
                        "Failed to retrieve any ciphertext materials for UserDecryptionRequest {} - skipping this request",
                        req_clone.decryptionId
                    );
                    return Ok(()); // Continue processing other events
                }

                let user_key_prefixed = hex::encode_prefixed(req_clone.userAddress);

                debug!(
                    "UserDecryptionRequest-{} was received with userAddress: {}",
                    req_clone.decryptionId, user_key_prefixed,
                );

                // Process decryption directly since we already have S3 materials
                info!(
                    "[PROCESSING] UserDecryptionRequest-{} with pre-fetched S3 materials",
                    req_clone.decryptionId
                );

                // Clean up timestamp to prevent memory leaks
                let event_id = req_clone.decryptionId.to_string();
                remove_block_timestamp(&event_id);

                // Handle decryption directly with the S3 materials we just fetched
                if let Err(e) = decryption_handler
                    .handle_decryption_request_response(
                        req.decryptionId,
                        key_id,
                        sns_ciphertext_materials,
                        Some(req.userAddress), // client_addr for user requests
                        Some(req.publicKey),   // public_key for user requests
                    )
                    .await
                {
                    error!(
                        "Error processing UserDecryptionRequest-{}: {}",
                        req.decryptionId, e
                    );
                }
            }
            _ => {} // Ignore other events for now
        }
        Ok(())
    }
}
