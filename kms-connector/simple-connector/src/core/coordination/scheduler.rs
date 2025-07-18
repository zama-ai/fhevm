use crate::{
    core::{config::Config, decryption::handler::DecryptionHandler},
    error::Result,
    gw_adapters::events::KmsCoreEvent,
};
use alloy::{primitives::U256, providers::Provider};
use chrono::Utc;
use dashmap::DashMap;
use std::{
    collections::BinaryHeap,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering as AtomicOrdering},
    },
    time::Duration,
};
use tokio::{
    sync::{Mutex, Semaphore, broadcast},
    time::sleep,
};
use tracing::{debug, error, info, warn};

/// Backpressure signals to control polling speed based on queue capacity
#[derive(Debug, Clone)]
pub enum BackpressureSignal {
    /// Queue is full - polling should slow down or pause
    QueueFull,
    /// Queue has space available - polling can resume normal speed
    QueueAvailable,
    /// Queue is critically full - polling should pause completely
    QueueCritical,
}

/// A message scheduled for coordinated sending using natural event IDs
#[derive(Debug, Clone)]
struct CoordinatedMessage {
    event: KmsCoreEvent,
    send_time: u64,  // UNIX timestamp in milliseconds when to send
    created_at: u64, // UNIX timestamp in milliseconds when created
}

impl CoordinatedMessage {
    /// Create a new coordinated message
    fn new(event: KmsCoreEvent, block_timestamp: u64, send_delta_ms: u64) -> Self {
        // IMPORTANT: Use UTC time consistently to match blockchain block timestamps
        // Block timestamps are always in UTC, so we must use UTC for all time comparisons
        let now_utc = Utc::now();
        let now = now_utc.timestamp_millis() as u64;

        // IMPORTANT: Convert block_timestamp from seconds to milliseconds
        // block_timestamp is in seconds, but we need milliseconds for comparison with now
        let block_timestamp_ms = block_timestamp * 1000;
        let send_time = block_timestamp_ms + send_delta_ms;

        // Extract request ID and type for logging
        let (request_id, event_type) = match &event {
            KmsCoreEvent::PublicDecryptionRequest(req) => {
                (req.decryptionId, "PublicDecryptionRequest")
            }
            KmsCoreEvent::UserDecryptionRequest(req) => (req.decryptionId, "UserDecryptionRequest"),
            _ => (U256::ZERO, "Not tracked event"), // TODO: Properly implement fallback for other event types
        };

        // Log concise scheduling info with message type
        let delay_ms = send_time as i64 - now as i64;

        if delay_ms < 0 {
            // Only calculate when needed - delayed case
            info!(
                "[SCHEDULED] {}-{} in 0ms (delta: {}ms, delayed by {}ms)",
                event_type, request_id, send_delta_ms, -delay_ms
            );
        } else {
            // Normal case - use delay_ms directly (already positive)
            info!(
                "[SCHEDULED] {}-{} in {}ms (delta: {}ms)",
                event_type, request_id, delay_ms, send_delta_ms
            );
        }

        Self {
            event,
            send_time,
            created_at: now,
        }
    }

    /// Extract the natural unique ID from the event (U256 for efficiency)
    fn get_event_id(&self) -> U256 {
        match &self.event {
            KmsCoreEvent::PublicDecryptionRequest(req) => req.decryptionId,
            KmsCoreEvent::UserDecryptionRequest(req) => req.decryptionId,
            // For other event types, we'll use a timestamp-based ID for now
            // TODO: Add proper ID extraction for management events when their structure is known
            _ => {
                let nanos = Utc::now().timestamp_nanos_opt().unwrap_or(0) as u128;
                U256::from(nanos)
            }
        }
    }
}

// Implement ordering for priority queue (min-heap by send_time)
impl Ord for CoordinatedMessage {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse ordering for min-heap behavior (earliest time first)
        other.send_time.cmp(&self.send_time)
    }
}

impl PartialOrd for CoordinatedMessage {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CoordinatedMessage {
    fn eq(&self, other: &Self) -> bool {
        self.get_event_id() == other.get_event_id()
    }
}

impl Eq for CoordinatedMessage {}

/// MessageScheduler handles coordinated message sending with timing synchronization
/// Uses natural event IDs for deduplication and a single priority queue for efficiency
pub struct MessageScheduler<P> {
    // Single priority queue ordered by send_time, with deduplication by event ID
    schedule_queue: Arc<Mutex<BinaryHeap<CoordinatedMessage>>>,
    // Track processed event IDs to prevent duplicates (U256 for efficiency)
    processed_events: Arc<DashMap<U256, u64>>, // event_id -> processed_timestamp
    decryption_handler: DecryptionHandler<P>,
    config: Config,
    provider: Arc<P>,
    task_semaphore: Arc<Semaphore>,
    queue_size: Arc<AtomicUsize>,
    shutdown_tx: broadcast::Receiver<()>,
    // Backpressure signaling: notify when queue is full/available
    backpressure_tx: Arc<broadcast::Sender<BackpressureSignal>>,
}

impl<P: Provider + Clone + 'static> MessageScheduler<P> {
    pub fn new(
        decryption_handler: DecryptionHandler<P>,
        config: Config,
        provider: Arc<P>,
        shutdown_tx: broadcast::Receiver<()>,
    ) -> (Self, broadcast::Receiver<BackpressureSignal>) {
        let task_semaphore = Arc::new(Semaphore::new(config.max_concurrent_tasks));
        let (backpressure_tx, backpressure_rx) = broadcast::channel(100);

        let scheduler = Self {
            schedule_queue: Arc::new(Mutex::new(BinaryHeap::new())),
            processed_events: Arc::new(DashMap::new()),
            decryption_handler,
            config,
            provider,
            task_semaphore,
            queue_size: Arc::new(AtomicUsize::new(0)),
            shutdown_tx,
            backpressure_tx: Arc::new(backpressure_tx),
        };

        (scheduler, backpressure_rx)
    }

    /// Schedule a message for coordinated sending with deduplication
    pub async fn schedule_message(&self, event: KmsCoreEvent, block_timestamp: u64) -> Result<()> {
        // Check queue capacity and apply backpressure instead of dropping
        let current_size = self.queue_size.load(AtomicOrdering::Relaxed);
        let queue_threshold = (self.config.pending_events_max as f32 * 0.9) as usize; // 90% threshold
        let critical_threshold = (self.config.pending_events_max as f32 * 0.95) as usize; // 95% threshold

        if current_size >= self.config.pending_events_max {
            // Queue is completely full - signal critical backpressure and wait
            warn!(
                "Message queue at capacity ({}), signaling critical backpressure",
                current_size
            );
            let _ = self.backpressure_tx.send(BackpressureSignal::QueueCritical);

            // Wait for queue to have space (with timeout to prevent infinite blocking)
            let mut retry_count = 0;
            while self.queue_size.load(AtomicOrdering::Relaxed) >= self.config.pending_events_max
                && retry_count < 50
            {
                sleep(Duration::from_millis(100)).await; // Wait 100ms
                retry_count += 1;
            }

            if retry_count >= 50 {
                error!("Queue remained full after 5 seconds, dropping event to prevent deadlock");
                return Ok(());
            }
        } else if current_size >= critical_threshold {
            // Queue is critically full - signal strong backpressure
            warn!(
                "Message queue critically full ({}/{}), signaling backpressure",
                current_size, self.config.pending_events_max
            );
            let _ = self.backpressure_tx.send(BackpressureSignal::QueueCritical);
        } else if current_size >= queue_threshold {
            // Queue is getting full - signal moderate backpressure
            warn!(
                "Message queue filling up ({}/{}), signaling backpressure",
                current_size, self.config.pending_events_max
            );
            let _ = self.backpressure_tx.send(BackpressureSignal::QueueFull);
        }

        let coordinated_message =
            CoordinatedMessage::new(event, block_timestamp, self.config.message_send_delta_ms);

        let event_id = coordinated_message.get_event_id();

        // Check for duplicates using natural event ID
        if self.processed_events.contains_key(&event_id) {
            debug!("Skipping duplicate event: {}", event_id);
            return Ok(());
        }

        // Add to schedule queue (single priority queue)
        {
            let mut queue = self.schedule_queue.lock().await;
            queue.push(coordinated_message.clone());
        }

        // Track this event ID to prevent duplicates
        self.processed_events
            .insert(event_id, coordinated_message.created_at);

        // IMPORTANT: Use saturating arithmetic to prevent overflow
        let old_size = self.queue_size.fetch_add(1, AtomicOrdering::Relaxed);
        let new_size = old_size.saturating_add(1);

        // Safety check: Log if we detect potential overflow conditions
        if old_size > self.config.pending_events_max * 2 {
            error!(
                "Queue size anomaly detected: old_size={}, max_size={}, new_size={}",
                old_size, self.config.pending_events_max, new_size
            );
        }

        // Signal queue available if we were previously at capacity and now have space
        let queue_threshold = (self.config.pending_events_max as f32 * 0.8) as usize; // 80% threshold for recovery
        if new_size == queue_threshold {
            info!(
                "Queue size reduced to {}, signaling queue available",
                new_size
            );
            let _ = self
                .backpressure_tx
                .send(BackpressureSignal::QueueAvailable);
        }

        debug!(
            "Scheduled message {} for sending at timestamp {} (queue size: {})",
            event_id, coordinated_message.send_time, new_size
        );
        Ok(())
    }

    /// Start the background scheduler task
    pub async fn start_scheduler(&self) -> Result<()> {
        info!("Starting MessageScheduler background task");

        let pending_messages = Arc::clone(&self.processed_events);
        let schedule_queue = Arc::clone(&self.schedule_queue);
        let decryption_handler = self.decryption_handler.clone();
        let config = self.config.clone();
        let provider = Arc::clone(&self.provider);
        let task_semaphore = Arc::clone(&self.task_semaphore);
        let queue_size = Arc::clone(&self.queue_size);
        let backpressure_tx = Arc::clone(&self.backpressure_tx);
        let mut shutdown_rx = self.shutdown_tx.resubscribe();

        tokio::spawn(async move {
            let mut cleanup_interval = tokio::time::interval(Duration::from_secs(3600)); // Cleanup every hour

            // Dynamic processing interval: message_spacing_ms / 2, capped at 100ms for efficiency
            let dynamic_interval_ms = std::cmp::min(100, config.message_spacing_ms / 2);
            let mut processing_interval =
                tokio::time::interval(Duration::from_millis(dynamic_interval_ms));

            loop {
                tokio::select! {
                    _ = cleanup_interval.tick() => {
                        Self::cleanup_expired_events(&pending_messages, &queue_size).await;
                    }
                    _ = processing_interval.tick() => {
                        // IMPORTANT: Continuously process scheduled messages
                        Self::process_scheduled_messages(
                            &schedule_queue,
                            &decryption_handler,
                            &config,
                            provider.clone(),
                            Arc::clone(&task_semaphore),
                            &queue_size,
                            &backpressure_tx,
                        ).await;
                    }
                    _ = shutdown_rx.recv() => {
                        info!("MessageScheduler received shutdown signal");
                        break;
                    }
                }
            }

            info!("MessageScheduler background task stopped");
        });

        Ok(())
    }

    /// Process messages that are ready to be sent
    async fn process_scheduled_messages(
        schedule_queue: &Arc<Mutex<BinaryHeap<CoordinatedMessage>>>,
        decryption_handler: &DecryptionHandler<P>,
        config: &Config,
        provider: Arc<P>,
        task_semaphore: Arc<Semaphore>,
        queue_size: &Arc<AtomicUsize>,
        backpressure_tx: &Arc<broadcast::Sender<BackpressureSignal>>,
    ) {
        // IMPORTANT: Use UTC time consistently to match blockchain block timestamps
        // Block timestamps are always in UTC, so we must use UTC for all time comparisons
        let now = Utc::now().timestamp_millis() as u64;

        let mut messages_to_send = Vec::new();
        let initial_queue_size = queue_size.load(AtomicOrdering::Relaxed);

        // Collect messages ready to send from single priority queue
        {
            let mut queue = schedule_queue.lock().await;
            while let Some(message) = queue.peek() {
                if message.send_time <= now {
                    let message = queue.pop().unwrap();
                    messages_to_send.push(message);
                    // IMPORTANT: Use saturating subtraction to prevent underflow
                    let old_size = queue_size.load(AtomicOrdering::Relaxed);
                    if old_size > 0 {
                        queue_size.fetch_sub(1, AtomicOrdering::Relaxed);
                    }
                } else {
                    break;
                }
            }
        }

        // Signal queue available if we freed up significant space
        let final_queue_size = queue_size.load(AtomicOrdering::Relaxed);
        let queue_threshold = (config.pending_events_max as f32 * 0.8) as usize; // 80% threshold

        if !messages_to_send.is_empty()
            && initial_queue_size > queue_threshold
            && final_queue_size <= queue_threshold
        {
            info!(
                "Queue size reduced from {} to {}, signaling queue available",
                initial_queue_size, final_queue_size
            );
            let _ = backpressure_tx.send(BackpressureSignal::QueueAvailable);
        }

        if messages_to_send.is_empty() {
            // No messages ready, sleep briefly
            sleep(Duration::from_millis(10)).await;
            return;
        }

        info!("Processing {} scheduled messages", messages_to_send.len());

        // Send messages sequentially with proper spacing
        for (index, message) in messages_to_send.into_iter().enumerate() {
            let handler = decryption_handler.clone();
            let task_semaphore_clone = Arc::clone(&task_semaphore);
            let config_clone = config.clone();
            let provider_clone = provider.clone();
            let message_spacing = Duration::from_millis(config.message_spacing_ms);
            let max_retries = config.max_retries;

            // Add spacing delay BEFORE spawning each task (except the first)
            if index > 0 {
                sleep(message_spacing).await;
            }

            tokio::spawn(async move {
                // Acquire semaphore permit for backpressure control inside the task
                let _permit = task_semaphore_clone.acquire().await.unwrap();

                let mut retry_count = 0;
                let mut success = false;

                while retry_count <= max_retries && !success {
                    match Self::send_message(
                        &handler,
                        &message.event,
                        &config_clone,
                        provider_clone.clone(),
                    )
                    .await
                    {
                        Ok(_) => {
                            debug!("Successfully sent message after {} retries", retry_count);
                            success = true;
                        }
                        Err(e) => {
                            retry_count += 1;
                            if retry_count <= max_retries {
                                warn!("Failed to send message (attempt {}): {}", retry_count, e);
                                sleep(Duration::from_millis(1000 * retry_count as u64)).await;
                            } else {
                                error!(
                                    "Failed to send message after {} retries: {}",
                                    max_retries, e
                                );
                            }
                        }
                    }
                }
            });

            // Enforce message spacing
            if config.message_spacing_ms > 0 {
                sleep(message_spacing).await;
            }
        }
    }

    /// Helper method to retrieve ciphertext materials from S3
    /// This reuses the same logic as EventProcessor to ensure consistency
    async fn retrieve_sns_ciphertext_materials(
        sns_materials: Vec<
            fhevm_gateway_rust_bindings::decryption::Decryption::SnsCiphertextMaterial,
        >,
        config: &Config,
        provider: Arc<P>,
    ) -> Vec<(Vec<u8>, Vec<u8>)> {
        let s3_config = config.s3_config.clone();

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
            let s3_urls = crate::core::utils::s3::prefetch_coprocessor_buckets(
                coprocessor_addresses,
                config.gateway_config_address,
                provider.clone(),
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
                match crate::core::utils::s3::retrieve_s3_ciphertext(
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

    /// Send a single message through the decryption handler
    /// This method processes the event using the same pattern as EventProcessor
    async fn send_message(
        handler: &DecryptionHandler<P>,
        event: &KmsCoreEvent,
        config: &Config,
        provider: Arc<P>,
    ) -> Result<()> {
        debug!("MessageScheduler: Processing scheduled event: {:?}", event);

        match event {
            KmsCoreEvent::PublicDecryptionRequest(request) => {
                info!(
                    "[DEQUEUING] PublicDecryptionRequest-{}",
                    request.decryptionId
                );

                // Extract key_id from the first snsCtMaterials entry (same as EventProcessor)
                let key_id = if !request.snsCtMaterials.is_empty() {
                    // IMPORTANT: Convert U256 keyId to 32-byte lowercase hex string (no 0x prefix)
                    // This matches the protobuf requirement for 64-character hex string
                    let extracted_key_id = request.snsCtMaterials.first().unwrap().keyId;
                    alloy::hex::encode(extracted_key_id.to_be_bytes::<32>())
                } else {
                    error!(
                        "No snsCtMaterials found for PublicDecryptionRequest-{}",
                        request.decryptionId
                    );
                    return Ok(()); // Continue processing other messages
                };

                // Retrieve ciphertext materials from S3 using the same logic as EventProcessor
                let sns_ciphertext_materials = Self::retrieve_sns_ciphertext_materials(
                    request.snsCtMaterials.clone(),
                    config,
                    provider.clone(),
                )
                .await;

                // If we couldn't retrieve any materials, fail the request
                if sns_ciphertext_materials.is_empty() {
                    error!(
                        "Failed to retrieve any ciphertext materials for PublicDecryptionRequest-{}",
                        request.decryptionId
                    );
                    return Ok(()); // Continue processing other messages
                }

                // Call the decryption handler with the request data
                match handler
                    .handle_decryption_request_response(
                        request.decryptionId,
                        key_id,
                        sns_ciphertext_materials,
                        None, // client_addr is None for public requests
                        None, // public_key is None for public requests
                    )
                    .await
                {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        error!(
                            "Error processing scheduled PublicDecryptionRequest-{}: {}",
                            request.decryptionId, e
                        );
                        Ok(()) // Continue processing other messages
                    }
                }
            }
            KmsCoreEvent::UserDecryptionRequest(request) => {
                info!("[DEQUEUING] UserDecryptionRequest-{}", request.decryptionId);

                // Extract key_id from the first snsCtMaterials entry (same as EventProcessor)
                let key_id = if !request.snsCtMaterials.is_empty() {
                    // IMPORTANT: Convert U256 keyId to 32-byte lowercase hex string (no 0x prefix)
                    // This matches the protobuf requirement for 64-character hex string
                    let extracted_key_id = request.snsCtMaterials.first().unwrap().keyId;
                    alloy::hex::encode(extracted_key_id.to_be_bytes::<32>())
                } else {
                    error!(
                        "No snsCtMaterials found for UserDecryptionRequest-{}",
                        request.decryptionId
                    );
                    return Ok(()); // Continue processing other messages
                };

                // Retrieve ciphertext materials from S3 using the same logic as EventProcessor
                let sns_ciphertext_materials = Self::retrieve_sns_ciphertext_materials(
                    request.snsCtMaterials.clone(),
                    config,
                    provider.clone(),
                )
                .await;

                // If we couldn't retrieve any materials, fail the request
                if sns_ciphertext_materials.is_empty() {
                    error!(
                        "Failed to retrieve any ciphertext materials for UserDecryptionRequest-{}",
                        request.decryptionId
                    );
                    return Ok(()); // Continue processing other messages
                }

                // Call the decryption handler with the request data
                match handler
                    .handle_decryption_request_response(
                        request.decryptionId,
                        key_id,
                        sns_ciphertext_materials,
                        Some(request.userAddress), // client_addr for user requests
                        Some(request.publicKey.clone()), // public_key for user requests
                    )
                    .await
                {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        error!(
                            "Error processing scheduled UserDecryptionRequest-{}: {}",
                            request.decryptionId, e
                        );
                        Ok(()) // Continue processing other messages
                    }
                }
            }
            _ => {
                warn!(
                    "MessageScheduler: Unsupported event type for scheduled sending: {:?}",
                    event
                );
                Ok(()) // Continue processing other messages
            }
        }
    }

    /// Clean up expired event IDs (TTL-based cleanup)
    async fn cleanup_expired_events(
        processed_events: &Arc<DashMap<U256, u64>>,
        _queue_size: &Arc<AtomicUsize>,
    ) {
        // IMPORTANT: Use UTC time consistently to match blockchain block timestamps
        let now = Utc::now().timestamp_millis() as u64;

        let ttl_ms = 24 * 60 * 60 * 1000; // 24 hours
        let mut expired_count = 0;

        // Remove expired event IDs
        processed_events.retain(|_event_id, &mut timestamp| {
            let is_expired = now.saturating_sub(timestamp) > ttl_ms;
            if is_expired {
                expired_count += 1;
            }
            !is_expired
        });

        if expired_count > 0 {
            info!(
                "Cleaned up {} expired event IDs from scheduler",
                expired_count
            );
        }
    }

    /// Check if queue is approaching capacity (for backpressure)
    pub fn should_slow_down(&self) -> bool {
        let current_size = self.queue_size.load(std::sync::atomic::Ordering::Relaxed);
        let threshold = (self.config.pending_events_max as f32
            * self.config.pending_events_queue_slowdown_threshold) as usize;
        current_size >= threshold
    }

    /// Get current queue statistics
    pub fn get_queue_stats(&self) -> (usize, usize) {
        let current_size = self.queue_size.load(std::sync::atomic::Ordering::Relaxed);
        (current_size, self.config.pending_events_max)
    }
}
