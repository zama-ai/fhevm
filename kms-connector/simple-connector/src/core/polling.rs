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
        let base_interval_ms = (self.config.base_poll_interval_secs * 1000).max(500);
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
                            // Adaptive polling
                            let current = self.current_block.load(Ordering::SeqCst);
                            let latest = self.latest_block.load(Ordering::SeqCst); // Use cached value
                            let blocks_behind = latest.saturating_sub(current);

                            if blocks_processed > 0 {
                                // Use hardcoded threshold of 3 blocks
                                if blocks_behind > 3 {
                                    // Catch-up mode: faster polling
                                    current_interval_ms = (base_interval_ms / 2).max(100); // Min 100ms
                                    debug!("Catch-up mode: {}ms (processed {} blocks, {} behind)", current_interval_ms, blocks_processed, blocks_behind);
                                } else {
                                    // Normal mode: base interval
                                    current_interval_ms = base_interval_ms;
                                    debug!("Normal polling: {}ms (processed {} blocks)", current_interval_ms, blocks_processed);
                                }
                            } else {
                                // No new blocks: base interval
                                current_interval_ms = base_interval_ms;
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

    /// Sequential polling
    async fn poll_and_process_fast(&self) -> Result<u64> {
        let current = self.current_block.load(Ordering::SeqCst);
        let latest = self.get_latest_block_number().await?;

        if latest <= current {
            debug!(
                "No new blocks to process (current: {}, latest: {})",
                current, latest
            );
            return Ok(0);
        }

        self.latest_block.store(latest, Ordering::SeqCst);
        let blocks_behind = latest - current;

        let mut blocks_processed = 0u64;
        let mut next_block = current + 1;

        // Process up to 5 blocks per poll cycle to maintain responsiveness
        let max_blocks_per_cycle = 5u64;
        let blocks_to_process = blocks_behind.min(max_blocks_per_cycle);

        debug!(
            "Sequential processing: blocks {} to {} (latest: {}, {} behind)",
            next_block,
            next_block + blocks_to_process - 1,
            latest,
            blocks_behind
        );

        // Sequential block processing
        for _ in 0..blocks_to_process {
            // Check if already processed
            if self.processed_blocks.contains_key(&next_block) {
                debug!("Block {} already processed, skipping", next_block);
                next_block += 1;
                continue;
            }

            // Process a single
            match self.process_single_block_sequential(next_block).await {
                Ok(_) => {
                    // Mark as successfully processed
                    self.processed_blocks.insert(next_block, true);
                    self.current_block.store(next_block, Ordering::SeqCst);
                    blocks_processed += 1;
                    debug!("Successfully processed block {}", next_block);
                }
                Err(e) => {
                    error!(
                        "Failed to process block {}: {} - continuing with next block",
                        next_block, e
                    );
                    self.processed_blocks.insert(next_block, false);
                }
            }

            next_block += 1;
        }

        // Cleanup old processed blocks to prevent memory growth
        if self.processed_blocks.len() > 1000 {
            self.cleanup_processed_blocks(current).await;
        }

        Ok(blocks_processed)
    }

    /// Process a single block sequentially for reliability and memory efficiency with monitoring
    async fn process_single_block_sequential(&self, block_number: u64) -> Result<()> {
        let block_start = Instant::now();
        debug!("Processing block {}", block_number);

        // Monitor block fetch time
        let fetch_start = Instant::now();
        let block = self
            .provider
            .get_block_by_number(block_number.into())
            .await
            .map_err(|e| Error::Provider(format!("Failed to get block {block_number}: {e}")))?
            .ok_or_else(|| Error::Provider(format!("Block {block_number} not found")))?;

        let fetch_duration = fetch_start.elapsed();
        // Arbitrum-optimized thresholds: 250ms block time, ~100-200ms RPC response expected
        if fetch_duration > Duration::from_secs(2) {
            error!(
                "[CRITICAL SLOW BLOCK FETCH]: Block {} fetch took {:?} - provider severely degraded (Gateway expected: <500ms)",
                block_number, fetch_duration
            );
        } else if fetch_duration > Duration::from_millis(1000) {
            warn!(
                "[SLOW BLOCK FETCH]: Block {} fetch took {:?} - provider performance degraded (Gateway expected: <500ms)",
                block_number, fetch_duration
            );
        } else if fetch_duration > Duration::from_millis(500) {
            debug!(
                "Moderate block fetch time: {:?} for block {} (Gateway baseline: ~200ms)",
                fetch_duration, block_number
            );
        }

        let block_timestamp = block.header.timestamp;
        debug!(
            "Retrieved timestamp {} for block {} in {:?}",
            block_timestamp, block_number, fetch_duration
        );

        // Monitor event processing time
        let events_start = Instant::now();
        self.process_all_events_unified(block_number, block_timestamp)
            .await?;

        let events_duration = events_start.elapsed();
        let total_duration = block_start.elapsed();

        // Alert on slow block processing
        if total_duration > Duration::from_secs(5) {
            error!(
                "[CRITICAL SLOW BLOCK]: Block {} processing took {:?} (fetch: {:?}, events: {:?}) - missing multiple Gateway blocks!",
                block_number, total_duration, fetch_duration, events_duration
            );
        } else if total_duration > Duration::from_secs(2) {
            warn!(
                "[SLOW BLOCK PROCESSING]: Block {} took {:?} (fetch: {:?}, events: {:?}) - slower than Gateway block time",
                block_number, total_duration, fetch_duration, events_duration
            );
        } else if total_duration > Duration::from_millis(500) {
            debug!(
                "Block {} processing: {:?} (fetch: {:?}, events: {:?}) - within Gateway tolerance",
                block_number, total_duration, fetch_duration, events_duration
            );
        } else {
            debug!(
                "Successfully processed block {} with timestamp {} in {:?} (fetch: {:?}, events: {:?})",
                block_number, block_timestamp, total_duration, fetch_duration, events_duration
            );
        }

        Ok(())
    }

    /// Cleanup old processed blocks to prevent memory growth
    async fn cleanup_processed_blocks(&self, current_block: u64) {
        // More aggressive cleanup to prevent memory leaks (last 1000 blocks only)
        let cleanup_threshold = current_block.saturating_sub(1000);
        let initial_size = self.processed_blocks.len();

        // Remove old entries from DashMap
        self.processed_blocks
            .retain(|&block_num, _| block_num > cleanup_threshold);

        let final_size = self.processed_blocks.len();
        let cleaned_count = initial_size.saturating_sub(final_size);

        if cleaned_count > 0 {
            debug!(
                "Cleaned up {} processed blocks older than {} (size: {} -> {})",
                cleaned_count, cleanup_threshold, initial_size, final_size
            );
        }
    }

    /// Unified event processing
    async fn process_all_events_unified(
        &self,
        block_number: u64,
        block_timestamp: u64,
    ) -> Result<()> {
        // Collect all events with their transaction indices for proper ordering
        let mut ordered_events = BTreeMap::new();

        // Process decryption events
        let decryption_contract =
            Decryption::new(self.config.decryption_address, Arc::clone(&self.provider));

        // Query public decryption events
        let public_filter = decryption_contract
            .PublicDecryptionRequest_filter()
            .from_block(block_number)
            .to_block(block_number);

        let public_events = public_filter.query().await.map_err(|e| {
            Error::Provider(format!("Failed to query public decryption events: {e}"))
        })?;

        // Query user decryption events
        let user_filter = decryption_contract
            .UserDecryptionRequest_filter()
            .from_block(block_number)
            .to_block(block_number);

        let user_events = user_filter
            .query()
            .await
            .map_err(|e| Error::Provider(format!("Failed to query user decryption events: {e}")))?;

        // Process gateway management events
        let gateway_contract = KmsManagement::new(
            self.config.gateway_config_address,
            Arc::clone(&self.provider),
        );

        // Query keygen events
        let keygen_filter = gateway_contract
            .KeygenRequest_filter()
            .from_block(block_number)
            .to_block(block_number);

        let keygen_events = keygen_filter
            .query()
            .await
            .map_err(|e| Error::Provider(format!("Failed to query keygen events: {e}")))?;

        // Query crsgen events
        let crsgen_filter = gateway_contract
            .CrsgenRequest_filter()
            .from_block(block_number)
            .to_block(block_number);

        let crsgen_events = crsgen_filter
            .query()
            .await
            .map_err(|e| Error::Provider(format!("Failed to query crsgen events: {e}")))?;

        // Unified event ordering logic
        self.add_events_to_ordered_map(&mut ordered_events, public_events, |event| {
            KmsCoreEvent::PublicDecryptionRequest(event)
        });

        self.add_events_to_ordered_map(&mut ordered_events, user_events, |event| {
            KmsCoreEvent::UserDecryptionRequest(event)
        });

        self.add_events_to_ordered_map(&mut ordered_events, keygen_events, |event| {
            KmsCoreEvent::KeygenRequest(event)
        });

        self.add_events_to_ordered_map(&mut ordered_events, crsgen_events, |event| {
            KmsCoreEvent::CrsgenRequest(event)
        });

        // Send all events in transaction order (BTreeMap maintains sorted order)
        for (_, event) in ordered_events {
            self.send_event_with_backpressure(event, block_timestamp)
                .await?;
        }

        Ok(())
    }

    /// Helper method to add events to ordered map
    fn add_events_to_ordered_map<T, F>(
        &self,
        ordered_events: &mut BTreeMap<(u64, u64), KmsCoreEvent>,
        events: Vec<(T, alloy::rpc::types::Log)>,
        event_converter: F,
    ) where
        F: Fn(T) -> KmsCoreEvent,
    {
        for (event, log) in events {
            let tx_index = log.transaction_index.unwrap_or(0);
            let log_index = log.log_index.unwrap_or(0);
            let sort_key = (tx_index, log_index);
            ordered_events.insert(sort_key, event_converter(event));
        }
    }

    /// Send event to the processing channel with async backpressure control and monitoring
    async fn send_event_with_backpressure(
        &self,
        event: KmsCoreEvent,
        block_timestamp: u64,
    ) -> Result<()> {
        let operation_start = Instant::now();

        // Monitor backpressure wait time
        if self.is_backpressure_active.load(Ordering::Relaxed) {
            let backpressure_start = Instant::now();
            debug!("Backpressure active - waiting for release");

            // Use async timeout instead of blocking polling loop
            match tokio::time::timeout(
                Duration::from_secs(2), // 2 second max wait
                async {
                    while self.is_backpressure_active.load(Ordering::Relaxed) {
                        tokio::time::sleep(Duration::from_millis(50)).await;
                    }
                },
            )
            .await
            {
                Ok(_) => {
                    let backpressure_duration = backpressure_start.elapsed();
                    debug!(
                        "Backpressure released after {:?} - resuming event processing",
                        backpressure_duration
                    );

                    // Alert on long backpressure waits
                    if backpressure_duration > Duration::from_secs(1) {
                        warn!(
                            "[SLOW BACKPRESSURE]: Waited {:?} for backpressure release - check downstream processing capacity",
                            backpressure_duration
                        );
                    }
                }
                Err(_) => warn!(
                    "[BACKPRESSURE TIMEOUT]: 2+ seconds waiting for release - resuming with degraded performance"
                ),
            }
        }

        // Monitor semaphore acquisition time
        let semaphore_start = Instant::now();
        let _permit = match self.event_semaphore.try_acquire() {
            Ok(permit) => {
                let acquire_duration = semaphore_start.elapsed();
                if acquire_duration > Duration::from_millis(1) {
                    debug!("Fast semaphore acquire: {:?}", acquire_duration);
                }
                permit
            }
            Err(_) => {
                warn!(
                    "[SEMAPHORE FULL]: Event processing semaphore exhausted - acquiring with blocking wait"
                );

                // Monitor blocking semaphore acquire time
                let blocking_start = Instant::now();
                let permit = self.event_semaphore.acquire().await.map_err(|e| {
                    Error::Channel(format!("Failed to acquire event semaphore: {e}"))
                })?;

                let blocking_duration = blocking_start.elapsed();
                if blocking_duration > Duration::from_millis(100) {
                    warn!(
                        "[SLOW SEMAPHORE]: Blocking acquire took {:?} - system under high load",
                        blocking_duration
                    );
                } else {
                    debug!(
                        "Semaphore acquired after blocking wait: {:?}",
                        blocking_duration
                    );
                }

                permit
            }
        };

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

        // Monitor channel send time
        let send_start = Instant::now();
        let result = self
            .event_tx
            .send(event)
            .await
            .map_err(|e| Error::Channel(format!("Failed to send event: {e}")));

        let send_duration = send_start.elapsed();

        // Alert on slow channel sends
        if send_duration > Duration::from_secs(3) {
            error!(
                "[CRITICAL CHANNEL SLOW SEND]: Channel send took {:?} - downstream processing severely degraded! (blocking multiple Arbitrum blocks)",
                send_duration
            );
        } else if send_duration > Duration::from_millis(500) {
            warn!(
                "[SLOW CHANNEL SEND]: Event send took {:?} - check downstream event processor performance (Arbitrum block time: 250ms)",
                send_duration
            );
        } else if send_duration > Duration::from_millis(100) {
            debug!(
                "Moderate channel send time: {:?} (within Arbitrum block tolerance)",
                send_duration
            );
        }

        // Monitor total operation time
        let total_duration = operation_start.elapsed();
        if total_duration > Duration::from_secs(2) {
            warn!(
                "[SLOW EVENT PROCESSING]: Total send_event_with_backpressure took {:?}",
                total_duration
            );
        }

        result
        // Permit is automatically released when _permit goes out of scope
    }

    /// Get the latest block number from the blockchain with monitoring
    async fn get_latest_block_number(&self) -> Result<u64> {
        let rpc_start = Instant::now();

        let result = self
            .provider
            .get_block_number()
            .await
            .map_err(|e| Error::Provider(format!("Failed to get latest block number: {e}")));

        let rpc_duration = rpc_start.elapsed();

        // Monitor RPC call performance
        match &result {
            Ok(block_number) => {
                if rpc_duration > Duration::from_secs(3) {
                    error!(
                        "[CRITICAL SLOW RPC]: get_block_number took {:?} - provider severely degraded! Block: {} (Arbitrum expected: <200ms)",
                        rpc_duration, block_number
                    );
                } else if rpc_duration > Duration::from_millis(1000) {
                    warn!(
                        "[SLOW RPC]: get_block_number took {:?} - provider performance degraded. Block: {} (Arbitrum expected: <200ms)",
                        rpc_duration, block_number
                    );
                } else if rpc_duration > Duration::from_millis(500) {
                    debug!(
                        "Moderate RPC time: {:?} for block {} (Arbitrum baseline: ~100-200ms)",
                        rpc_duration, block_number
                    );
                }
            }
            Err(e) => {
                error!(
                    "[RPC FAILURE]: get_block_number failed after {:?} - {}",
                    rpc_duration, e
                );
            }
        }

        result
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
