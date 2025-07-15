//! Efficient blockchain polling implementation leveraging existing WebSocket provider.
//!
//! This module provides a clean, performant polling strategy that can be used as an
//! alternative to WebSocket-based event streaming.

use crate::core::coordination::scheduler::BackpressureSignal;
use crate::gw_adapters::events::KmsCoreEvent;
use crate::{Error, Result, core::config::Config};
use alloy::providers::Provider;
use chrono::Utc;
use dashmap::DashMap;
use fhevm_gateway_rust_bindings::{decryption::Decryption, kmsmanagement::KmsManagement};
use std::{
    collections::BTreeMap,
    sync::{
        Arc, OnceLock,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    time::Duration,
};
use tokio::{
    sync::{Mutex, Semaphore, broadcast, mpsc},
    time::{Instant, sleep},
};

// Timestamp entry with creation time for TTL cleanup
#[derive(Debug, Clone)]
struct TimestampEntry {
    block_timestamp: u64,
    created_at: i64, // UTC timestamp in seconds (chrono::Utc)
}

/// Global storage for block timestamps indexed by event ID
/// This enables the event processor to access block timestamps for coordinated sending
static BLOCK_TIMESTAMPS: OnceLock<DashMap<String, TimestampEntry>> = OnceLock::new();

// Flag to ensure cleanup task is started only once
static TIMESTAMP_CLEANUP_STARTED: OnceLock<()> = OnceLock::new();

use tracing::{debug, error, info, warn};

/// Non-blocking TTL cleanup for block timestamps
/// Runs in background task to prevent flow jamming
/// Uses chrono::Utc for consistent UTC timing with blockchain timestamps
async fn cleanup_expired_block_timestamps_once() -> std::result::Result<usize, String> {
    let ttl_seconds = 2 * 60 * 60; // 2 hours TTL (shorter than S3 cache)
    let now = Utc::now().timestamp(); // UTC timestamp in seconds
    let mut expired_count = 0;

    // Non-blocking cleanup using DashMap's retain method
    if let Some(timestamps) = BLOCK_TIMESTAMPS.get() {
        timestamps.retain(|_event_id, entry| {
            let is_expired = now.saturating_sub(entry.created_at) > ttl_seconds;
            if is_expired {
                expired_count += 1;
            }
            !is_expired
        });

        // Log state periodically
        if !timestamps.is_empty() {
            debug!(
                "BLOCK_TIMESTAMPS: {} active entries after cleanup",
                timestamps.len()
            );
        }
    }

    Ok(expired_count)
}

/// Start the non-blocking timestamp cleanup task (called once)
fn start_cleanup_task_for_expired_block_timestamps() {
    TIMESTAMP_CLEANUP_STARTED.get_or_init(|| {
        let handle = tokio::spawn(async {
            info!("Block timestamp cleanup task starting...");

            // Run cleanup with proper error handling
            loop {
                match tokio::time::timeout(
                    Duration::from_secs(15), // 15 second timeout for cleanup operation
                    cleanup_expired_block_timestamps_once(),
                )
                .await
                {
                    Ok(Ok(cleaned_count)) => {
                        if cleaned_count > 0 {
                            info!(
                                "Block timestamp cleanup completed: {} entries removed",
                                cleaned_count
                            );
                        } else {
                            debug!("Block timestamp cleanup completed: no expired entries");
                        }
                    }
                    Ok(Err(e)) => {
                        warn!("Block timestamp cleanup failed: {}", e);
                    }
                    Err(_) => {
                        warn!("Block timestamp cleanup timed out after 15 seconds");
                    }
                }

                // Wait 5 minutes before next cleanup
                tokio::time::sleep(Duration::from_secs(5 * 60)).await;
            }
        });

        info!(
            "Started non-blocking block timestamp cleanup task (JoinHandle: {:?})",
            handle.id()
        );
    });
}

/// High-performance blockchain poller optimized for sub-second block times
pub struct BlockPoller<P> {
    /// Ethereum provider for blockchain access
    provider: Arc<P>,
    /// Configuration
    config: Arc<Config>,
    /// Event channel sender
    event_tx: mpsc::Sender<KmsCoreEvent>,
    /// Current block number being processed
    current_block: Arc<AtomicU64>,
    /// Latest known block number
    latest_block: Arc<AtomicU64>,
    /// Shutdown signal sender
    shutdown_tx: broadcast::Sender<()>,
    /// Running state
    is_running: Arc<AtomicBool>,
    /// Lock-free event deduplication cache (block_num -> processed)
    processed_blocks: Arc<DashMap<u64, bool>>,
    /// Semaphore to limit concurrent event processing
    event_semaphore: Arc<Semaphore>,
    /// Last poll time for adaptive intervals
    last_poll_time: Arc<Mutex<Instant>>,
    /// Backpressure receiver for queue management
    backpressure_rx: Option<broadcast::Receiver<BackpressureSignal>>,
    /// Backpressure state tracking
    is_backpressure_active: Arc<AtomicBool>,
}

impl<P> Clone for BlockPoller<P> {
    fn clone(&self) -> Self {
        Self {
            provider: Arc::clone(&self.provider),
            config: Arc::clone(&self.config),
            event_tx: self.event_tx.clone(),
            current_block: Arc::clone(&self.current_block),
            latest_block: Arc::clone(&self.latest_block),
            shutdown_tx: self.shutdown_tx.clone(),
            is_running: Arc::clone(&self.is_running),
            processed_blocks: Arc::clone(&self.processed_blocks),
            event_semaphore: Arc::clone(&self.event_semaphore),
            last_poll_time: Arc::clone(&self.last_poll_time),
            backpressure_rx: self.backpressure_rx.as_ref().map(|rx| rx.resubscribe()),
            is_backpressure_active: Arc::clone(&self.is_backpressure_active),
        }
    }
}

impl<P> BlockPoller<P>
where
    P: Provider + Clone + Send + Sync + 'static,
{
    /// Create a new block poller
    pub fn new(
        provider: Arc<P>,
        config: Arc<Config>,
        event_tx: mpsc::Sender<KmsCoreEvent>,
        shutdown_tx: broadcast::Sender<()>,
        backpressure_rx: Option<broadcast::Receiver<BackpressureSignal>>,
    ) -> Self {
        let starting_block = config.starting_block_number.unwrap_or(0);

        Self {
            provider,
            config,
            event_tx,
            current_block: Arc::new(AtomicU64::new(starting_block)),
            latest_block: Arc::new(AtomicU64::new(0)),
            shutdown_tx,
            is_running: Arc::new(AtomicBool::new(false)),
            processed_blocks: Arc::new(DashMap::new()),
            event_semaphore: Arc::new(Semaphore::new(100)), // Limit concurrent event processing
            last_poll_time: Arc::new(Mutex::new(Instant::now())),
            backpressure_rx,
            is_backpressure_active: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Shutdown the poller
    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down BlockPoller...");

        // Signal shutdown
        if let Err(e) = self.shutdown_tx.send(()) {
            warn!("Failed to send shutdown signal: {}", e);
        }

        // Set running to false
        self.is_running.store(false, Ordering::SeqCst);

        info!("BlockPoller shutdown complete");
        Ok(())
    }

    /// Backpressure monitoring with graceful shutdown
    async fn run_backpressure_monitor(
        mut backpressure_rx: broadcast::Receiver<BackpressureSignal>,
        backpressure_state: Arc<AtomicBool>,
        mut shutdown: broadcast::Receiver<()>,
    ) {
        info!("Starting backpressure monitoring task");

        loop {
            tokio::select! {
                signal = backpressure_rx.recv() => {
                    match signal {
                        Ok(BackpressureSignal::QueueFull) => {
                            debug!("Received QueueFull signal, activating moderate backpressure");
                            backpressure_state.store(true, Ordering::Relaxed);
                        }
                        Ok(BackpressureSignal::QueueCritical) => {
                            warn!("Received QueueCritical signal, activating strong backpressure");
                            backpressure_state.store(true, Ordering::Relaxed);
                        }
                        Ok(BackpressureSignal::QueueAvailable) => {
                            info!("Received QueueAvailable signal, releasing backpressure");
                            backpressure_state.store(false, Ordering::Relaxed);
                        }
                        Err(e) => {
                            debug!("Backpressure channel error: {}, stopping monitor", e);
                            break;
                        }
                    }
                }
                _ = shutdown.recv() => {
                    info!("Backpressure monitoring task received shutdown signal");
                    break;
                }
            }
        }

        info!("Backpressure monitoring task stopped");
    }

    /// Start the polling loop
    pub async fn start(&self) -> Result<()> {
        if self.is_running.swap(true, Ordering::SeqCst) {
            warn!("BlockPoller is already running");
            return Ok(());
        }

        info!(
            "Starting blockchain polling with interval: {}s",
            self.config.base_poll_interval_secs
        );

        // Initialize current block if starting from latest
        if self.config.starting_block_number.is_none() {
            match self.get_latest_block_number().await {
                Ok(latest) => {
                    self.current_block.store(latest, Ordering::SeqCst);
                    self.latest_block.store(latest, Ordering::SeqCst);
                    info!("Starting from latest block: {}", latest);
                }
                Err(e) => {
                    error!("Failed to get latest block number: {}", e);
                    return Err(e);
                }
            }
        }

        // Start backpressure monitoring task if backpressure receiver is available
        if let Some(backpressure_rx) = self.backpressure_rx.as_ref().map(|rx| rx.resubscribe()) {
            let backpressure_state = Arc::clone(&self.is_backpressure_active);
            let shutdown = self.shutdown_tx.subscribe();

            tokio::spawn(Self::run_backpressure_monitor(
                backpressure_rx,
                backpressure_state,
                shutdown,
            ));
        }

        // Adaptive polling for sub-second block times
        let mut shutdown_rx = self.shutdown_tx.subscribe();
        let base_interval_ms = (self.config.base_poll_interval_secs * 1000).max(100); // Min 100ms
        let mut current_interval_ms = base_interval_ms;

        info!(
            "Starting adaptive polling with base interval: {}ms",
            base_interval_ms
        );

        loop {
            let poll_start = Instant::now();

            tokio::select! {
                _ = sleep(Duration::from_millis(current_interval_ms)) => {
                    match self.poll_and_process_fast().await {
                        Ok(blocks_processed) => {
                            // Check if we're truly behind (catch-up mode) or just processing normal blockchain progression
                            let current = self.current_block.load(Ordering::SeqCst);
                            // Get fresh latest block number for accurate catch-up detection
                            let latest = match self.get_latest_block_number().await {
                                Ok(latest_block) => {
                                    self.latest_block.store(latest_block, Ordering::SeqCst);
                                    latest_block
                                },
                                Err(_) => self.latest_block.load(Ordering::SeqCst) // Fallback to cached value
                            };
                            let blocks_behind = latest.saturating_sub(current);

                            if blocks_processed > 0 {
                                // Use hardcoded threshold of 3 blocks
                                let catch_up_threshold = 3u64;
                                if blocks_behind > catch_up_threshold {
                                    // True catch-up mode: we're significantly behind
                                    current_interval_ms = (base_interval_ms / 4).max(50); // Min 50ms for catch-up
                                    info!("Catch-up mode: {}ms (processed {} blocks, {} blocks behind)", current_interval_ms, blocks_processed, blocks_behind);
                                } else {
                                    // Normal blockchain progression: use base interval
                                    current_interval_ms = base_interval_ms;
                                    debug!("Normal polling: {}ms (processed {} blocks, {} blocks behind)", current_interval_ms, blocks_processed, blocks_behind);
                                }
                            } else {
                                // No new blocks: gradually slow down but don't go below base interval
                                current_interval_ms = (current_interval_ms * 11 / 10).min(base_interval_ms * 2).max(base_interval_ms);
                                debug!("Idle polling: {}ms (no new blocks)", current_interval_ms);
                            }
                        }
                        Err(e) => {
                            error!("Polling error: {}", e);
                            return Err(e);
                        }
                    }

                    // Update last poll time
                    if let Ok(mut last_time) = self.last_poll_time.try_lock() {
                        *last_time = poll_start;
                    }
                }
                _ = shutdown_rx.recv() => {
                    info!("Received shutdown signal, stopping poller");
                    break;
                }
            }
        }

        self.is_running.store(false, Ordering::SeqCst);
        info!("BlockPoller stopped");
        Ok(())
    }

    /// High-performance polling optimized for sub-second block times
    async fn poll_and_process_fast(&self) -> Result<u64> {
        let current = self.current_block.load(Ordering::SeqCst);
        let latest = self.get_latest_block_number().await?;

        if latest <= current {
            debug!(
                "No new blocks to process (current: {}, latest: {})",
                current, latest
            );
            return Ok(0); // Return number of blocks processed
        }

        self.latest_block.store(latest, Ordering::SeqCst);

        let blocks_behind = latest - current;
        // Use a simple threshold for catch-up mode (5 blocks behind)
        let catch_up_threshold = 5u64;
        let max_blocks_per_batch = 10u64; // Default batch size

        let batch_size = if blocks_behind > catch_up_threshold {
            debug!(
                "Catch-up mode: {} blocks behind, using batch size {}",
                blocks_behind, max_blocks_per_batch
            );
            max_blocks_per_batch
        } else {
            // For sub-second blocks, process small batches even when caught up
            (blocks_behind).min(3) // Max 3 blocks per batch for fast processing
        };

        let end_block = (current + batch_size).min(latest);
        let blocks_to_process = end_block - current;

        if blocks_to_process == 0 {
            return Ok(0);
        }

        debug!(
            "Processing blocks {} to {} (latest: {}, batch_size: {})",
            current + 1,
            end_block,
            latest,
            blocks_to_process
        );

        // Process blocks concurrently for better performance with sub-second blocks
        let mut tasks = Vec::new();

        for block_num in (current + 1)..=end_block {
            // Check if already processed using lock-free DashMap
            if self.processed_blocks.contains_key(&block_num) {
                debug!("Block {} already processed, skipping", block_num);
                continue;
            }

            // Mark as being processed
            self.processed_blocks.insert(block_num, false);

            let poller = self.clone();
            let task = tokio::spawn(async move {
                match poller.process_single_block_fast(block_num).await {
                    Ok(_) => {
                        // Mark as successfully processed
                        poller.processed_blocks.insert(block_num, true);
                        Ok(block_num)
                    }
                    Err(e) => {
                        error!("Failed to process block {}: {}", block_num, e);
                        // Remove from processed map on failure
                        poller.processed_blocks.remove(&block_num);
                        Err(e)
                    }
                }
            });
            tasks.push(task);
        }

        // Wait for all block processing tasks to complete
        let mut successful_blocks = Vec::new();
        for task in tasks {
            match task.await {
                Ok(Ok(block_num)) => successful_blocks.push(block_num),
                Ok(Err(e)) => error!("Block processing error: {}", e),
                Err(e) => error!("Task join error: {}", e),
            }
        }

        // Update current block to the highest successfully processed block
        if let Some(&max_block) = successful_blocks.iter().max() {
            self.current_block.store(max_block, Ordering::SeqCst);
            debug!("Updated current block to {}", max_block);
        }

        // Cleanup old processed blocks to prevent memory growth
        self.cleanup_processed_blocks(current).await;

        Ok(successful_blocks.len() as u64)
    }

    /// Process a single block with concurrent event processing for maximum throughput
    async fn process_single_block_fast(&self, block_number: u64) -> Result<()> {
        debug!("Fast-processing block {}", block_number);

        // Get block information including timestamp
        let block = self
            .provider
            .get_block_by_number(block_number.into())
            .await
            .map_err(|e| Error::Provider(format!("Failed to get block {block_number}: {e}")))?
            .ok_or_else(|| Error::Provider(format!("Block {block_number} not found")))?;

        let block_timestamp = block.header.timestamp;
        debug!("Block {} timestamp: {}", block_number, block_timestamp);

        // Process decryption and gateway events concurrently with block timestamp
        let decryption_task = self.process_decryption_events(block_number, block_timestamp);
        let gateway_task = self.process_gateway_events(block_number, block_timestamp);

        // Wait for both to complete
        let (decryption_result, gateway_result) = tokio::join!(decryption_task, gateway_task);

        // Check results
        decryption_result?;
        gateway_result?;

        debug!(
            "Successfully fast-processed block {} with timestamp {}",
            block_number, block_timestamp
        );
        Ok(())
    }

    /// Cleanup old processed blocks to prevent memory growth
    async fn cleanup_processed_blocks(&self, current_block: u64) {
        // Keep only recent blocks in memory (last 1000 blocks)
        let cleanup_threshold = current_block.saturating_sub(1000);

        // Remove old entries from DashMap
        self.processed_blocks
            .retain(|&block_num, _| block_num > cleanup_threshold);

        debug!(
            "Cleaned up processed blocks older than {}",
            cleanup_threshold
        );
    }

    /// Process decryption events for a specific block with proper ordering
    async fn process_decryption_events(
        &self,
        block_number: u64,
        block_timestamp: u64,
    ) -> Result<()> {
        // Create contract instance
        let contract = Decryption::new(self.config.decryption_address, Arc::clone(&self.provider));

        // Create filters for the specific block
        let public_filter = contract
            .PublicDecryptionRequest_filter()
            .from_block(block_number)
            .to_block(block_number);

        let user_filter = contract
            .UserDecryptionRequest_filter()
            .from_block(block_number)
            .to_block(block_number);

        // Query events for this block
        let public_events = public_filter.query().await.map_err(|e| {
            Error::Provider(format!("Failed to query public decryption events: {e}"))
        })?;

        let user_events = user_filter
            .query()
            .await
            .map_err(|e| Error::Provider(format!("Failed to query user decryption events: {e}")))?;

        // Collect all events with their transaction indices for proper ordering
        let mut ordered_events = BTreeMap::new();

        // Add public events with their transaction indices
        for (event, log) in public_events {
            let tx_index = log.transaction_index.unwrap_or(0);
            let log_index = log.log_index.unwrap_or(0);
            let sort_key = (tx_index, log_index);
            ordered_events.insert(sort_key, KmsCoreEvent::PublicDecryptionRequest(event));
        }

        // Add user events with their transaction indices
        for (event, log) in user_events {
            let tx_index = log.transaction_index.unwrap_or(0);
            let log_index = log.log_index.unwrap_or(0);
            let sort_key = (tx_index, log_index);
            ordered_events.insert(sort_key, KmsCoreEvent::UserDecryptionRequest(event));
        }

        // Send events in transaction order (BTreeMap maintains sorted order)
        for (_, event) in ordered_events {
            self.send_event_with_backpressure(event, block_timestamp)
                .await?;
        }

        Ok(())
    }

    /// Process gateway config events for a specific block with proper ordering
    async fn process_gateway_events(&self, block_number: u64, block_timestamp: u64) -> Result<()> {
        // Create contract instance
        let contract = KmsManagement::new(
            self.config.gateway_config_address,
            Arc::clone(&self.provider),
        );

        // Create filters for the specific block
        let keygen_filter = contract
            .KeygenRequest_filter()
            .from_block(block_number)
            .to_block(block_number);

        let crsgen_filter = contract
            .CrsgenRequest_filter()
            .from_block(block_number)
            .to_block(block_number);

        // Query events for this block
        let keygen_events = keygen_filter
            .query()
            .await
            .map_err(|e| Error::Provider(format!("Failed to query keygen events: {e}")))?;

        let crsgen_events = crsgen_filter
            .query()
            .await
            .map_err(|e| Error::Provider(format!("Failed to query crsgen events: {e}")))?;

        // Collect all events with their transaction indices for proper ordering
        let mut ordered_events = BTreeMap::new();

        // Add keygen events with their transaction indices
        for (event, log) in keygen_events {
            let tx_index = log.transaction_index.unwrap_or(0);
            let log_index = log.log_index.unwrap_or(0);
            let sort_key = (tx_index, log_index);
            ordered_events.insert(sort_key, KmsCoreEvent::KeygenRequest(event));
        }

        // Add crsgen events with their transaction indices
        for (event, log) in crsgen_events {
            let tx_index = log.transaction_index.unwrap_or(0);
            let log_index = log.log_index.unwrap_or(0);
            let sort_key = (tx_index, log_index);
            ordered_events.insert(sort_key, KmsCoreEvent::CrsgenRequest(event));
        }

        // Send events in transaction order (BTreeMap maintains sorted order)
        for (_, event) in ordered_events {
            self.send_event_with_backpressure(event, block_timestamp)
                .await?;
        }

        Ok(())
    }

    /// Send event to the processing channel with backpressure control
    async fn send_event_with_backpressure(
        &self,
        event: KmsCoreEvent,
        block_timestamp: u64,
    ) -> Result<()> {
        // Check if backpressure is active and wait if needed
        if self.is_backpressure_active.load(Ordering::Relaxed) {
            debug!("Backpressure active, waiting before sending event");

            // Wait for backpressure to be released (with timeout to prevent infinite blocking)
            let mut retry_count = 0;
            while self.is_backpressure_active.load(Ordering::Relaxed) && retry_count < 100 {
                sleep(Duration::from_millis(100)).await; // Wait 100ms
                retry_count += 1;
            }

            if retry_count >= 100 {
                warn!("Backpressure remained active for 10 seconds, proceeding with event send");
            } else {
                debug!("Backpressure released, resuming event processing");
            }
        }

        // Acquire semaphore permit to limit concurrent event processing
        let _permit = self
            .event_semaphore
            .acquire()
            .await
            .map_err(|e| Error::Channel(format!("Failed to acquire event semaphore: {e}")))?;

        // Start cleanup task if not already started (non-blocking)
        start_cleanup_task_for_expired_block_timestamps();

        // Store block timestamp in a global map for the event processor to access
        // This fixes the critical timing bug by making block timestamps available for coordinated sending
        if let Some(event_id) = Self::get_event_id(&event) {
            let now = Utc::now().timestamp(); // UTC timestamp in seconds

            let timestamp_entry = TimestampEntry {
                block_timestamp,
                created_at: now,
            };

            let timestamps = BLOCK_TIMESTAMPS.get_or_init(DashMap::new);
            timestamps.insert(event_id.clone(), timestamp_entry);
            debug!(
                "Stored block timestamp {} for event ID {} for coordinated processing",
                block_timestamp, event_id
            );
        }

        self.event_tx
            .send(event)
            .await
            .map_err(|e| Error::Channel(format!("Failed to send event: {e}")))
        // Permit is automatically released when _permit goes out of scope
    }

    /// Get the latest block number from the blockchain
    async fn get_latest_block_number(&self) -> Result<u64> {
        self.provider
            .get_block_number()
            .await
            .map_err(|e| Error::Provider(format!("Failed to get latest block number: {e}")))
    }

    /// Get current processing status
    pub fn get_status(&self) -> PollingStatus {
        PollingStatus {
            is_running: self.is_running.load(Ordering::SeqCst),
            current_block: self.current_block.load(Ordering::SeqCst),
            latest_block: self.latest_block.load(Ordering::SeqCst),
        }
    }

    /// Extract unique event ID for timestamp storage
    /// This enables coordinated sending by linking events to their block timestamps
    fn get_event_id(event: &KmsCoreEvent) -> Option<String> {
        match event {
            KmsCoreEvent::PublicDecryptionRequest(req) => Some(req.decryptionId.to_string()),
            KmsCoreEvent::UserDecryptionRequest(req) => Some(req.decryptionId.to_string()),
            KmsCoreEvent::KeygenRequest(req) => Some(req.preKeyId.to_string()),
            KmsCoreEvent::CrsgenRequest(req) => Some(req.crsgenRequestId.to_string()),
            _ => None, // Management events don't need coordinated sending
        }
    }
}

/// Public function to get block timestamp for an event ID
/// This enables the event processor to access block timestamps for coordinated sending
pub fn get_block_timestamp(event_id: &str) -> Option<u64> {
    BLOCK_TIMESTAMPS
        .get()?
        .get(event_id)
        .map(|entry| entry.value().block_timestamp)
}

/// Public function to remove block timestamp after processing
/// This prevents memory leaks by cleaning up processed timestamps
pub fn remove_block_timestamp(event_id: &str) -> Option<u64> {
    BLOCK_TIMESTAMPS
        .get()?
        .remove(event_id)
        .map(|(_, entry)| entry.block_timestamp)
}

/// Polling status information
#[derive(Debug, Clone)]
pub struct PollingStatus {
    pub is_running: bool,
    pub current_block: u64,
    pub latest_block: u64,
}

impl PollingStatus {
    /// Get the number of blocks behind
    pub fn blocks_behind(&self) -> u64 {
        self.latest_block.saturating_sub(self.current_block)
    }

    /// Check if caught up (within 1 block)
    pub fn is_caught_up(&self) -> bool {
        self.blocks_behind() <= 1
    }
}
