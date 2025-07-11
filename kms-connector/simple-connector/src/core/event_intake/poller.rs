use alloy::{
    primitives::{Address, U64},
    providers::Provider,
    rpc::types::{Filter, Log},
    sol_types::SolEventInterface,
};
use fhevm_gateway_rust_bindings::{
    decryption::Decryption, idecryption::IDecryption::IDecryptionEvents,
    ikmsmanagement::IKmsManagement::IKmsManagementEvents, kmsmanagement::KmsManagement,
};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::Duration;
use tokio::{
    sync::broadcast,
    time::{Instant, sleep},
};
use tracing::{debug, error, info, warn};

use crate::{core::config::Config, error::Result, gw_adapters::events::KmsCoreEvent};

/// Event statistics for monitoring block polling
#[derive(Debug, Default)]
struct EventStats {
    events_sent: AtomicU64,
    events_dropped: AtomicU64,
    blocks_processed: AtomicU64,
    last_sent_timestamp: AtomicU64,
}

impl EventStats {
    fn increment_sent(&self) {
        self.events_sent.fetch_add(1, Ordering::Relaxed);
        self.last_sent_timestamp.store(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            Ordering::Relaxed,
        );
    }

    fn increment_dropped(&self) {
        self.events_dropped.fetch_add(1, Ordering::Relaxed);
    }

    fn increment_blocks_processed(&self) {
        self.blocks_processed.fetch_add(1, Ordering::Relaxed);
    }

    fn get_stats(&self) -> (u64, u64, u64, u64) {
        (
            self.events_sent.load(Ordering::Relaxed),
            self.events_dropped.load(Ordering::Relaxed),
            self.blocks_processed.load(Ordering::Relaxed),
            self.last_sent_timestamp.load(Ordering::Relaxed),
        )
    }
}

/// Configuration for block polling behavior
#[derive(Debug, Clone)]
pub struct PollingConfig {
    /// Base polling interval when caught up to latest block
    pub base_poll_interval: Duration,
    /// Fast polling interval when catching up on historical blocks
    pub catch_up_poll_interval: Duration,
    /// Maximum number of blocks to process in a single batch
    pub max_blocks_per_batch: u64,
    /// How far behind latest block to consider "caught up"
    pub catch_up_threshold: u64,
}

impl Default for PollingConfig {
    fn default() -> Self {
        Self {
            base_poll_interval: Duration::from_secs(2),
            catch_up_poll_interval: Duration::from_millis(100),
            max_blocks_per_batch: 10,
            catch_up_threshold: 5,
        }
    }
}

/// Block-based event poller with backpressure support
pub struct BlockPoller<P> {
    // Provider for blockchain operations (used for both WebSocket and polling)
    provider: Arc<P>,

    // Configuration
    #[allow(dead_code)] // Used for initialization but not directly accessed in polling logic
    config: Config,
    polling_config: PollingConfig,

    // State tracking
    current_block: AtomicU64,
    latest_block: AtomicU64,
    is_paused: AtomicBool,

    // Event filters
    decryption_address: Address,
    gateway_config_address: Address,

    // Event statistics
    stats: Arc<EventStats>,

    // Shutdown coordination
    shutdown_rx: broadcast::Receiver<()>,
}

impl<P: Provider + Clone + 'static> BlockPoller<P> {
    /// Create a new block poller
    pub fn new(provider: Arc<P>, config: Config, shutdown: broadcast::Receiver<()>) -> Self {
        // Initialize starting block - use 0 as placeholder if None
        // The actual latest block will be fetched in start_polling()
        let starting_block = config.starting_block_number.unwrap_or(0);

        // Extract values before moving config
        let decryption_address = config.decryption_address;
        let gateway_config_address = config.gateway_config_address;

        Self {
            provider,
            config,
            decryption_address,
            gateway_config_address,
            polling_config: PollingConfig::default(),
            current_block: AtomicU64::new(starting_block),
            latest_block: AtomicU64::new(0),
            is_paused: AtomicBool::new(false),
            stats: Arc::new(EventStats::default()),
            shutdown_rx: shutdown,
        }
    }

    /// Schedule periodic event statistics logging
    async fn schedule_log_event_stats(
        stats: Arc<EventStats>,
        event_tx: broadcast::Sender<KmsCoreEvent>,
        mut shutdown_rx: broadcast::Receiver<()>,
    ) {
        info!(
            "📊 Block polling event statistics logging task started - will report every 5 minutes"
        );
        let mut interval = tokio::time::interval(Duration::from_secs(300)); // 5 minutes
        interval.tick().await; // Skip first immediate tick

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    let (sent, dropped, blocks_processed, last_timestamp) = stats.get_stats();
                    let receiver_count = event_tx.receiver_count();

                    let time_since_last = if last_timestamp > 0 {
                        std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs()
                            .saturating_sub(last_timestamp)
                    } else {
                        0
                    };

                    info!(
                        "📊 Block Polling Event Stats - Receivers: {}, Events Sent: {}, Dropped: {}, Blocks Processed: {}, Last event: {}s ago",
                        receiver_count, sent, dropped, blocks_processed, time_since_last
                    );
                }
                _ = shutdown_rx.recv() => {
                    info!("📊 Event statistics logging task shutting down");
                    break;
                }
            }
        }
    }

    /// Start the polling loop
    pub async fn start_polling(&mut self, event_tx: broadcast::Sender<KmsCoreEvent>) -> Result<()> {
        // If starting_block_number was None, fetch the latest block from blockchain
        if self.config.starting_block_number.is_none() {
            match self.fetch_latest_block().await {
                Ok(latest_block) => {
                    info!(
                        "Auto-detected latest block: {}, starting from block {}",
                        latest_block, latest_block
                    );
                    self.current_block.store(latest_block, Ordering::Relaxed);
                    self.latest_block.store(latest_block, Ordering::Relaxed);
                }
                Err(e) => {
                    warn!("Failed to fetch latest block, starting from block 0: {}", e);
                    // Keep the default starting block (0) if we can't fetch latest
                }
            }
        }

        info!(
            "Starting block polling from block {}",
            self.current_block.load(Ordering::Relaxed)
        );

        // Spawn event statistics logging task
        let stats_clone = Arc::clone(&self.stats);
        let event_tx_clone = event_tx.clone();
        let stats_shutdown = self.shutdown_rx.resubscribe();
        tokio::spawn(Self::schedule_log_event_stats(
            stats_clone,
            event_tx_clone,
            stats_shutdown,
        ));

        let mut shutdown = self.shutdown_rx.resubscribe();
        let mut last_poll_time = Instant::now();

        loop {
            tokio::select! {
                _ = shutdown.recv() => {
                    info!("Received shutdown signal in block poller");
                    break;
                }
                _ = self.poll_cycle(&event_tx) => {
                    // Polling cycle completed, continue
                }
            }

            // Note: Backpressure handling is managed by the MessageScheduler internally
            // The poller continues running and the scheduler handles flow control

            // Adaptive polling interval based on catch-up status
            let poll_interval = self.get_adaptive_poll_interval();
            let elapsed = last_poll_time.elapsed();

            if elapsed < poll_interval {
                sleep(poll_interval - elapsed).await;
            }

            last_poll_time = Instant::now();
        }

        info!("Block polling stopped");
        Ok(())
    }

    /// Single polling cycle - fetch and process new blocks
    async fn poll_cycle(&mut self, event_tx: &broadcast::Sender<KmsCoreEvent>) -> Result<()> {
        // Skip if paused
        if self.is_paused.load(Ordering::Relaxed) {
            return Ok(());
        }

        // Get latest block number using WebSocket provider
        let latest_block = match self.provider.get_block_number().await {
            Ok(block_num) => block_num,
            Err(e) => {
                error!("Failed to get latest block number: {}", e);
                return Ok(());
            }
        };

        self.latest_block.store(latest_block, Ordering::Relaxed);
        let current_block = self.current_block.load(Ordering::Relaxed);

        if current_block >= latest_block {
            debug!(
                "No new blocks to process (current: {}, latest: {})",
                current_block, latest_block
            );
            return Ok(());
        }

        // Calculate how many blocks to process in this batch
        let blocks_behind = latest_block - current_block;
        let batch_size = std::cmp::min(blocks_behind, self.polling_config.max_blocks_per_batch);

        debug!(
            "Processing {} blocks starting from {}",
            batch_size,
            current_block + 1
        );

        // Process blocks in batch
        for block_num in (current_block + 1)..=(current_block + batch_size) {
            if let Err(e) = self.process_block(block_num, event_tx).await {
                error!("Failed to process block {}: {}", block_num, e);
                // Continue with next block rather than stopping entirely
                continue;
            }

            // Update current block after successful processing
            self.current_block.store(block_num, Ordering::Relaxed);

            // Check for shutdown between blocks
            if self.shutdown_rx.try_recv().is_ok() {
                break;
            }
        }

        Ok(())
    }

    /// Process a single block for relevant events
    async fn process_block(
        &self,
        block_number: u64,
        event_tx: &broadcast::Sender<KmsCoreEvent>,
    ) -> Result<()> {
        // Get block details for timestamp using WebSocket provider
        let block = match self
            .provider
            .get_block(U64::from(block_number).into())
            .await
        {
            Ok(Some(block)) => block,
            Ok(None) => {
                warn!("Block {} not found", block_number);
                return Ok(());
            }
            Err(e) => {
                error!("Failed to get block {}: {}", block_number, e);
                return Err(crate::error::Error::Provider(format!(
                    "Failed to get block: {e}"
                )));
            }
        };

        let block_timestamp = block.header.timestamp;
        debug!(
            "Processing block {} with timestamp {}",
            block_number, block_timestamp
        );

        // Create filter for decryption events in this block
        let filter = Filter::new()
            .address(vec![self.decryption_address, self.gateway_config_address])
            .from_block(block_number)
            .to_block(block_number);

        // Get logs for this block
        let logs = match self.provider.get_logs(&filter).await {
            Ok(logs) => logs,
            Err(e) => {
                error!("Failed to get logs for block {}: {}", block_number, e);
                return Ok(());
            }
        };

        debug!("Found {} logs in block {}", logs.len(), block_number);

        // Track block processing
        self.stats.increment_blocks_processed();
        let mut events_in_block = 0;

        // Process each log and convert to events
        for log in logs {
            if let Some(event) = self.log_to_event(log, block_timestamp).await {
                // Send event to processing pipeline
                match event_tx.send(event) {
                    Ok(_) => {
                        self.stats.increment_sent();
                        events_in_block += 1;
                    }
                    Err(e) => {
                        self.stats.increment_dropped();
                        warn!("Failed to send event to processing pipeline: {}", e);
                    }
                }
            }
        }

        if events_in_block > 0 {
            debug!(
                "Processed {} events from block {}",
                events_in_block, block_number
            );
        }

        Ok(())
    }

    /// Convert a log entry to a KmsCoreEvent if it matches our patterns
    async fn log_to_event(&self, log: Log, block_timestamp: u64) -> Option<KmsCoreEvent> {
        debug!(
            "Processing log from address {:?} with {} topics",
            log.address(),
            log.topics().len()
        );

        // Check if this log is from the decryption contract
        if log.address() == self.decryption_address {
            // Try to decode as IDecryption events using decode_raw_log
            if let Ok(decoded_event) =
                IDecryptionEvents::decode_raw_log(log.topics(), &log.data().data, true)
            {
                match decoded_event {
                    IDecryptionEvents::PublicDecryptionRequest(event) => {
                        info!("[POLLING] 🔒 Decoded PublicDecryptionRequest event:");
                        info!(
                            "  Block: {:?}, Tx: {:?}, LogIdx: {:?}",
                            log.block_number, log.transaction_hash, log.log_index
                        );
                        info!("  DecryptionId: {}", event.decryptionId);
                        debug!("  Decoded Event: {:#?}", event);

                        // Convert to Decryption contract event type
                        let decryption_event = Decryption::PublicDecryptionRequest {
                            decryptionId: event.decryptionId,
                            snsCtMaterials: event
                                .snsCtMaterials
                                .into_iter()
                                .map(|material| Decryption::SnsCiphertextMaterial {
                                    ctHandle: material.ctHandle,
                                    keyId: material.keyId,
                                    snsCiphertextDigest: material.snsCiphertextDigest,
                                    coprocessorTxSenderAddresses: material
                                        .coprocessorTxSenderAddresses,
                                })
                                .collect(),
                        };
                        return Some(KmsCoreEvent::PublicDecryptionRequest(
                            decryption_event,
                            block_timestamp,
                        ));
                    }
                    IDecryptionEvents::UserDecryptionRequest(event) => {
                        info!("[POLLING] 🔒 Decoded UserDecryptionRequest event:");
                        info!(
                            "  Block: {:?}, Tx: {:?}, LogIdx: {:?}",
                            log.block_number, log.transaction_hash, log.log_index
                        );
                        info!(
                            "  DecryptionId: {}, UserAddress: {:?}",
                            event.decryptionId, event.userAddress
                        );
                        debug!("  Decoded Event: {:#?}", event);

                        // Convert to Decryption contract event type
                        let decryption_event = Decryption::UserDecryptionRequest {
                            decryptionId: event.decryptionId,
                            snsCtMaterials: event
                                .snsCtMaterials
                                .into_iter()
                                .map(|material| Decryption::SnsCiphertextMaterial {
                                    ctHandle: material.ctHandle,
                                    keyId: material.keyId,
                                    snsCiphertextDigest: material.snsCiphertextDigest,
                                    coprocessorTxSenderAddresses: material
                                        .coprocessorTxSenderAddresses,
                                })
                                .collect(),
                            userAddress: event.userAddress,
                            publicKey: event.publicKey,
                        };
                        return Some(KmsCoreEvent::UserDecryptionRequest(
                            decryption_event,
                            block_timestamp,
                        ));
                    }
                    IDecryptionEvents::PublicDecryptionResponse(event) => {
                        info!("[POLLING] 🔒 Decoded PublicDecryptionResponse event:");
                        info!(
                            "  Block: {:?}, Tx: {:?}, LogIdx: {:?}",
                            log.block_number, log.transaction_hash, log.log_index
                        );
                        info!("  DecryptionId: {}", event.decryptionId);
                        debug!("  Decoded Event: {:#?}", event);

                        // Convert to Decryption contract event type
                        let decryption_event = Decryption::PublicDecryptionResponse {
                            decryptionId: event.decryptionId,
                            decryptedResult: event.decryptedResult,
                            signatures: event.signatures,
                        };
                        return Some(KmsCoreEvent::PublicDecryptionResponse(decryption_event));
                    }
                    IDecryptionEvents::UserDecryptionResponse(event) => {
                        info!("[POLLING] 🔒 Decoded UserDecryptionResponse event:");
                        info!(
                            "  Block: {:?}, Tx: {:?}, LogIdx: {:?}",
                            log.block_number, log.transaction_hash, log.log_index
                        );
                        info!("  DecryptionId: {}", event.decryptionId);
                        debug!("  Decoded Event: {:#?}", event);

                        // Convert to Decryption contract event type
                        let decryption_event = Decryption::UserDecryptionResponse {
                            decryptionId: event.decryptionId,
                            userDecryptedShares: event.userDecryptedShares,
                            signatures: event.signatures,
                        };
                        return Some(KmsCoreEvent::UserDecryptionResponse(decryption_event));
                    }
                }
            }

            debug!("Log from decryption contract but failed to decode as IDecryption event");
        }

        // Check if this log is from the gateway config contract
        if log.address() == self.gateway_config_address {
            // Try to decode as IKmsManagement events using decode_raw_log
            if let Ok(decoded_event) =
                IKmsManagementEvents::decode_raw_log(log.topics(), &log.data().data, true)
            {
                match decoded_event {
                    IKmsManagementEvents::PreprocessKeygenRequest(event) => {
                        info!("[POLLING] 🔧 Decoded PreprocessKeygenRequest event");
                        // Convert to KmsManagement contract event type
                        let kms_event = KmsManagement::PreprocessKeygenRequest {
                            preKeygenRequestId: event.preKeygenRequestId,
                            fheParamsDigest: event.fheParamsDigest,
                        };
                        return Some(KmsCoreEvent::PreprocessKeygenRequest(kms_event));
                    }
                    IKmsManagementEvents::PreprocessKeygenResponse(event) => {
                        info!("[POLLING] 🔧 Decoded PreprocessKeygenResponse event");
                        // Convert to KmsManagement contract event type
                        let kms_event = KmsManagement::PreprocessKeygenResponse {
                            preKeygenRequestId: event.preKeygenRequestId,
                            preKeyId: event.preKeyId,
                        };
                        return Some(KmsCoreEvent::PreprocessKeygenResponse(kms_event));
                    }
                    IKmsManagementEvents::PreprocessKskgenRequest(event) => {
                        info!("[POLLING] 🔧 Decoded PreprocessKskgenRequest event");
                        // Convert to KmsManagement contract event type
                        let kms_event = KmsManagement::PreprocessKskgenRequest {
                            preKskgenRequestId: event.preKskgenRequestId,
                            fheParamsDigest: event.fheParamsDigest,
                        };
                        return Some(KmsCoreEvent::PreprocessKskgenRequest(kms_event));
                    }
                    IKmsManagementEvents::PreprocessKskgenResponse(event) => {
                        info!("[POLLING] 🔧 Decoded PreprocessKskgenResponse event");
                        // Convert to KmsManagement contract event type
                        let kms_event = KmsManagement::PreprocessKskgenResponse {
                            preKskgenRequestId: event.preKskgenRequestId,
                            preKskId: event.preKskId,
                        };
                        return Some(KmsCoreEvent::PreprocessKskgenResponse(kms_event));
                    }
                    IKmsManagementEvents::KeygenRequest(event) => {
                        info!("[POLLING] 🔧 Decoded KeygenRequest event");
                        // Convert to KmsManagement contract event type
                        let kms_event = KmsManagement::KeygenRequest {
                            preKeyId: event.preKeyId,
                            fheParamsDigest: event.fheParamsDigest,
                        };
                        return Some(KmsCoreEvent::KeygenRequest(kms_event));
                    }
                    IKmsManagementEvents::KeygenResponse(event) => {
                        info!("[POLLING] 🔧 Decoded KeygenResponse event");
                        // Convert to KmsManagement contract event type
                        let kms_event = KmsManagement::KeygenResponse {
                            preKeyId: event.preKeyId,
                            keygenId: event.keygenId,
                            fheParamsDigest: event.fheParamsDigest,
                        };
                        return Some(KmsCoreEvent::KeygenResponse(kms_event));
                    }
                    IKmsManagementEvents::CrsgenRequest(event) => {
                        info!("[POLLING] 🔧 Decoded CrsgenRequest event");
                        // Convert to KmsManagement contract event type
                        let kms_event = KmsManagement::CrsgenRequest {
                            crsgenRequestId: event.crsgenRequestId,
                            fheParamsDigest: event.fheParamsDigest,
                        };
                        return Some(KmsCoreEvent::CrsgenRequest(kms_event));
                    }
                    IKmsManagementEvents::CrsgenResponse(event) => {
                        info!("[POLLING] 🔧 Decoded CrsgenResponse event");
                        // Convert to KmsManagement contract event type
                        let kms_event = KmsManagement::CrsgenResponse {
                            crsgenRequestId: event.crsgenRequestId,
                            crsId: event.crsId,
                            fheParamsDigest: event.fheParamsDigest,
                        };
                        return Some(KmsCoreEvent::CrsgenResponse(kms_event));
                    }
                    IKmsManagementEvents::KskgenRequest(event) => {
                        info!("[POLLING] 🔧 Decoded KskgenRequest event");
                        // Convert to KmsManagement contract event type
                        let kms_event = KmsManagement::KskgenRequest {
                            preKskId: event.preKskId,
                            sourceKeyId: event.sourceKeyId,
                            destKeyId: event.destKeyId,
                            fheParamsDigest: event.fheParamsDigest,
                        };
                        return Some(KmsCoreEvent::KskgenRequest(kms_event));
                    }
                    IKmsManagementEvents::KskgenResponse(event) => {
                        info!("[POLLING] 🔧 Decoded KskgenResponse event");
                        // Convert to KmsManagement contract event type
                        let kms_event = KmsManagement::KskgenResponse {
                            preKskId: event.preKskId,
                            kskId: event.kskId,
                            fheParamsDigest: event.fheParamsDigest,
                        };
                        return Some(KmsCoreEvent::KskgenResponse(kms_event));
                    }
                    _ => {
                        // Other KMS management events that we don't need to process
                        debug!("Ignoring unhandled KMS management event");
                    }
                }
            }

            debug!("Log from gateway config contract but failed to decode as IKmsManagement event");
        }

        // Log is from an address we don't recognize or doesn't match any known event signatures
        debug!("Log from unrecognized address or no matching event signature");
        None
    }

    /// Get adaptive polling interval based on catch-up status
    fn get_adaptive_poll_interval(&self) -> Duration {
        let current = self.current_block.load(Ordering::Relaxed);
        let latest = self.latest_block.load(Ordering::Relaxed);

        if latest > current && (latest - current) > self.polling_config.catch_up_threshold {
            // We're behind, poll faster
            self.polling_config.catch_up_poll_interval
        } else {
            // We're caught up, poll at normal rate
            self.polling_config.base_poll_interval
        }
    }

    /// Get current polling status for monitoring
    pub fn get_status(&self) -> PollingStatus {
        PollingStatus {
            current_block: self.current_block.load(Ordering::Relaxed),
            latest_block: self.latest_block.load(Ordering::Relaxed),
            is_paused: self.is_paused.load(Ordering::Relaxed),
            is_catching_up: self.is_catching_up(),
        }
    }

    /// Check if we're currently catching up on historical blocks
    pub fn is_catching_up(&self) -> bool {
        let current = self.current_block.load(Ordering::Relaxed);
        let latest = self.latest_block.load(Ordering::Relaxed);
        latest > current && (latest - current) > self.polling_config.catch_up_threshold
    }

    /// Manually set the current block (for recovery scenarios)
    pub fn set_current_block(&self, block_number: u64) {
        self.current_block.store(block_number, Ordering::Relaxed);
        info!("Manually set current block to {}", block_number);
    }

    /// Force resume polling (override backpressure pause)
    pub fn force_resume(&self) {
        self.is_paused.store(false, Ordering::Relaxed);
        info!("Block polling manually resumed");
    }

    /// Fetch the latest block number from the blockchain
    async fn fetch_latest_block(&self) -> Result<u64> {
        match self.provider.get_block_number().await {
            Ok(block_number) => {
                debug!("Fetched latest block number: {}", block_number);
                Ok(block_number)
            }
            Err(e) => {
                error!("Failed to fetch latest block number: {}", e);
                Err(crate::error::Error::Contract(format!(
                    "Failed to get latest block: {e}"
                )))
            }
        }
    }
}

/// Status information for monitoring the poller
#[derive(Debug, Clone)]
pub struct PollingStatus {
    pub current_block: u64,
    pub latest_block: u64,
    pub is_paused: bool,
    pub is_catching_up: bool,
}

impl std::fmt::Display for PollingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Block: {}/{} ({}behind) - {} - {}",
            self.current_block,
            self.latest_block,
            self.latest_block.saturating_sub(self.current_block),
            if self.is_paused { "PAUSED" } else { "ACTIVE" },
            if self.is_catching_up {
                "CATCHING_UP"
            } else {
                "CURRENT"
            }
        )
    }
}
