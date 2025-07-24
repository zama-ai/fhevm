use alloy::primitives::{Address, TxHash};
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use dashmap::DashMap;
use futures::lock::Mutex;
use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::sync::{broadcast, oneshot};
use tracing::{debug, error, info, warn};

use crate::core::backpressure::BackpressureSignal;
use crate::core::config::Config;
use crate::error::{Error, Result};

// Queue limits and thresholds are now configurable via Config
// No more hardcoded constants - everything comes from config

/// A transaction pending in the queue with its result channel
#[derive(Debug)]
struct PendingTransaction {
    tx_request: TransactionRequest,
    result_sender: oneshot::Sender<Result<TxHash>>,
}

/// Sequential nonce manager that processes transactions per wallet in order
/// while keeping block polling completely independent
#[derive(Clone)]
pub struct SequentialNonceManager<P> {
    /// Cached nonces per address (u64::MAX = uninitialized)
    nonces: Arc<DashMap<Address, Arc<Mutex<u64>>>>,

    /// Transaction queues per address for sequential processing
    transaction_queues: Arc<DashMap<Address, Arc<Mutex<VecDeque<PendingTransaction>>>>>,

    /// Processing flags to ensure only one processor per address
    processing_flags: Arc<DashMap<Address, Arc<AtomicBool>>>,

    /// Provider for blockchain calls
    provider: Arc<P>,

    /// Optional backpressure sender for queue status signaling
    backpressure_tx: Option<broadcast::Sender<BackpressureSignal>>,

    /// Configuration for queue limits and thresholds
    config: Arc<Config>,

    /// Wallet address from config
    wallet_address: Address,
}

impl<P> SequentialNonceManager<P>
where
    P: Provider + Clone + Send + Sync + 'static,
{
    /// Create a new sequential nonce manager with config-based queue limits
    pub fn new(provider: Arc<P>, config: Arc<Config>) -> Self {
        let wallet_address = config.wallet.address();
        Self {
            nonces: Arc::new(DashMap::new()),
            transaction_queues: Arc::new(DashMap::new()),
            processing_flags: Arc::new(DashMap::new()),
            provider,
            backpressure_tx: None,
            config,
            wallet_address,
        }
    }

    /// Create a new sequential nonce manager with backpressure signaling and config-based queue limits
    pub fn new_with_backpressure(
        provider: Arc<P>,
        backpressure_tx: broadcast::Sender<BackpressureSignal>,
        config: Arc<Config>,
    ) -> Self {
        let wallet_address = config.wallet.address();
        Self {
            nonces: Arc::new(DashMap::new()),
            transaction_queues: Arc::new(DashMap::new()),
            processing_flags: Arc::new(DashMap::new()),
            provider,
            backpressure_tx: Some(backpressure_tx),
            config,
            wallet_address,
        }
    }

    /// Queue a transac
    ///
    ///  sequential processing
    /// This method is NON-BLOCKING and returns immediately
    pub async fn send_transaction_queued(&self, mut tx: TransactionRequest) -> Result<TxHash> {
        let address = if let Some(addr) = tx.from {
            addr
        } else {
            let wallet_address = self.wallet_address;
            tx.from = Some(wallet_address); // Set it for queue processing
            info!(
                "Using config wallet address for sequential processing: {}",
                wallet_address
            );
            wallet_address
        };

        let (result_sender, result_receiver) = oneshot::channel();
        let pending_tx = PendingTransaction {
            tx_request: tx,
            result_sender,
        };

        let queue = self
            .transaction_queues
            .entry(address)
            .or_insert_with(|| Arc::new(Mutex::new(VecDeque::new())))
            .clone();

        // Check transaction queue utilization for backpressure signaling
        // For single wallet setup, we measure actual transaction queue vs reasonable limit
        let total_queued = self.get_total_queued().await;

        // Use a fraction of channel_size as transaction queue limit (transactions are slower than events)
        let max_transaction_queue_size = (self.config.channel_size as f32 * 0.3) as usize; // 30% of event capacity
        let utilization = total_queued as f32 / max_transaction_queue_size as f32;

        // Use config threshold for backpressure signaling
        let backpressure_threshold = self.config.pending_events_queue_slowdown_threshold;
        let critical_threshold = 0.95; // 95% for critical (close to full)

        // Send backpressure signals based on queue utilization
        if let Some(ref tx) = self.backpressure_tx {
            if utilization >= critical_threshold {
                let _ = tx.send(BackpressureSignal::QueueCritical);
                warn!(
                    "Sent QueueCritical signal: {:.1}% utilization",
                    utilization * 100.0
                );
            } else if utilization >= backpressure_threshold {
                let _ = tx.send(BackpressureSignal::QueueFull);
                warn!(
                    "Sent QueueFull signal: {:.1}% utilization",
                    utilization * 100.0
                );
            } else if utilization < backpressure_threshold {
                let _ = tx.send(BackpressureSignal::QueueAvailable);
                debug!(
                    "Sent QueueAvailable signal: {:.1}% utilization",
                    utilization * 100.0
                );
            }
        }

        // Only reject at 100% capacity (hard limit)
        if total_queued >= max_transaction_queue_size {
            warn!(
                "Transaction queue at capacity: {} total transactions (max: {})",
                total_queued, max_transaction_queue_size
            );
            return Err(Error::Transport(format!(
                "Transaction queue temporarily full: {total_queued} transactions pending. Polling will adapt automatically."
            )));
        }

        // Check per-wallet queue and add transaction
        {
            let mut queue_guard = queue.lock().await;

            // For single wallet setup, per-wallet limit equals total transaction queue limit
            let max_per_wallet = max_transaction_queue_size;

            // Check per-wallet queue limit
            if queue_guard.len() >= max_per_wallet {
                warn!(
                    "Queue overflow: wallet {} has {} transactions (max: {})",
                    address,
                    queue_guard.len(),
                    max_per_wallet
                );
                return Err(Error::Transport(format!(
                    "Transaction queue full for wallet {}: {} transactions pending",
                    address,
                    queue_guard.len()
                )));
            }

            queue_guard.push_back(pending_tx);
            debug!(
                "Queued transaction for wallet {}: {} in queue, {} total system-wide",
                address,
                queue_guard.len(),
                total_queued + 1
            );
        }

        // Start processing if not already running (non-blocking spawn)
        self.start_processing_if_needed(address).await;

        // Wait for result from background processor
        result_receiver
            .await
            .map_err(|_| Error::Channel("Transaction cancelled".to_string()))?
    }

    /// Start background processing for an address if not already running
    /// This spawns a tokio task that runs independently
    async fn start_processing_if_needed(&self, address: Address) {
        let processing_flag = self
            .processing_flags
            .entry(address)
            .or_insert_with(|| Arc::new(AtomicBool::new(false)))
            .clone();

        // Only start if not already processing
        if !processing_flag.swap(true, Ordering::SeqCst) {
            let manager = self.clone();

            tokio::spawn(async move {
                debug!("Started transaction processor for address {}", address);
                manager.process_transaction_queue(address).await;
                debug!("Stopped transaction processor for address {}", address);
            });
        }
    }

    /// Background task that processes transactions sequentially per address
    async fn process_transaction_queue(&self, address: Address) {
        let queue = self.transaction_queues.get(&address).unwrap().clone();
        let processing_flag = self.processing_flags.get(&address).unwrap().clone();

        loop {
            // Get next transaction from queue
            let pending_tx = {
                let mut queue_guard = queue.lock().await;
                queue_guard.pop_front()
            };

            match pending_tx {
                Some(pending) => {
                    // Process single transaction with proper nonce management
                    let result = self.process_single_transaction(pending.tx_request).await;

                    // Send result back to caller (non-blocking)
                    let _ = pending.result_sender.send(result);
                }
                None => {
                    // Queue is empty, stop processing
                    processing_flag.store(false, Ordering::SeqCst);
                    break;
                }
            }
        }
    }

    /// Process a single transaction with aggressive parallel nonce management for maximum throughput
    async fn process_single_transaction(&self, mut tx: TransactionRequest) -> Result<TxHash> {
        let address = tx.from.unwrap();

        // Check if we're at the parallel transaction limit
        let in_flight_count = self.get_in_flight_transaction_count(address).await;
        if in_flight_count >= self.config.max_parallel_transactions {
            // Minimal wait
            warn!(
                "[MILD] Hit mild limit in parallel transactions sending ({} >= {}), delaying by 25ms",
                in_flight_count, self.config.max_parallel_transactions
            );
            tokio::time::sleep(Duration::from_millis(25)).await;

            // Recheck after brief wait
            let updated_count = self.get_in_flight_transaction_count(address).await;
            if updated_count >= self.config.max_parallel_transactions {
                warn!(
                    "[PARALLEL] Hit parallel transactions sending limit ({} >= {}), backing off",
                    updated_count, self.config.max_parallel_transactions
                );
                // Send backpressure signal to slow down event processing
                if let Some(ref tx) = self.backpressure_tx {
                    let _ = tx.send(BackpressureSignal::QueueFull);
                }
                // Shorter backoff with WebSocket - faster confirmations
                tokio::time::sleep(Duration::from_millis(50)).await;
            }
        }

        let nonce = self.get_next_nonce_internal(address).await?;
        tx.nonce = Some(nonce);

        match self.send_transaction_immediately(tx, nonce).await {
            Ok(tx_hash) => {
                info!(
                    "[PARALLEL] Transaction sent: {} (nonce: {}, in-flight: {})",
                    tx_hash,
                    nonce,
                    in_flight_count + 1
                );
                self.track_transaction_confirmation_async(tx_hash, nonce);
                Ok(tx_hash)
            }
            Err(e) => {
                error!("[PARALLEL] Transaction failed: {} (nonce: {})", e, nonce);
                if let Err(refresh_err) = self.refresh_nonce_from_chain(address).await {
                    error!("Failed to refresh nonce: {}", refresh_err);
                }
                Err(e)
            }
        }
    }

    /// Get the count of in-flight transactions for aggressive parallel management
    async fn get_in_flight_transaction_count(&self, address: Address) -> usize {
        if let Some(queue_ref) = self.transaction_queues.get(&address) {
            let queue = queue_ref.lock().await;
            queue.len()
        } else {
            0
        }
    }

    /// Get the next nonce for the given address, incrementing the stored value
    async fn get_next_nonce_internal(&self, address: Address) -> Result<u64> {
        const NONE: u64 = u64::MAX;

        let nonce_mutex = self
            .nonces
            .entry(address)
            .or_insert_with(|| Arc::new(Mutex::new(NONE)))
            .clone();

        let mut nonce_guard = nonce_mutex.lock().await;

        if *nonce_guard == NONE {
            // First time - get from chain
            let chain_nonce = self
                .provider
                .get_transaction_count(address)
                .await
                .map_err(|e| Error::Transport(format!("Failed to get nonce: {e}")))?;
            *nonce_guard = chain_nonce;
            debug!("Initialized nonce for {} to {}", address, chain_nonce);
        }

        let current_nonce = *nonce_guard;
        *nonce_guard += 1;

        Ok(current_nonce)
    }

    /// Send transaction immediately with retry logic for parallel processing
    async fn send_transaction_immediately(
        &self,
        mut tx: TransactionRequest,
        expected_nonce: u64,
    ) -> Result<TxHash> {
        const MAX_RETRIES: usize = 3;
        let mut attempt = 0;

        while attempt < MAX_RETRIES {
            // Set nonce and aggressive gas price for immediate sending
            tx.nonce = Some(expected_nonce);

            // Use aggressive gas price from first attempt
            if attempt == 0 {
                if let Some(gas_price) = tx.gas_price {
                    tx.gas_price = Some(gas_price * 150 / 100); // 50% boost for fast processing
                    debug!(
                        "Set aggressive gas price {} for immediate send (nonce: {}, attempt: {})",
                        tx.gas_price.unwrap(),
                        expected_nonce,
                        attempt + 1
                    );
                }
            }

            // Send transaction immediately
            match self.provider.send_transaction(tx.clone()).await {
                Ok(pending_tx) => {
                    let tx_hash = *pending_tx.tx_hash();
                    info!(
                        "Transaction sent: {} (nonce: {}, attempt: {})",
                        tx_hash,
                        expected_nonce,
                        attempt + 1
                    );
                    return Ok(tx_hash);
                }
                Err(e) => {
                    let error_msg = e.to_string().to_lowercase();

                    // Check if it's a gas-related error that we can retry
                    if error_msg.contains("gas")
                        || error_msg.contains("underpriced")
                        || error_msg.contains("insufficient")
                    {
                        warn!(
                            "Gas-related error on immediate send attempt {} (nonce: {}): {}",
                            attempt + 1,
                            expected_nonce,
                            e
                        );

                        if attempt < MAX_RETRIES - 1 {
                            // Bump gas price by 10% for retry
                            if let Some(gas_price) = tx.gas_price {
                                tx.gas_price = Some(gas_price * 110 / 100);
                                info!(
                                    "Bumped gas price to {} for immediate retry",
                                    tx.gas_price.unwrap()
                                );
                            }

                            // Short retry delays for immediate sending: 50ms, then 100ms
                            let delay_ms = if attempt == 0 { 50 } else { 100 };
                            tokio::time::sleep(Duration::from_millis(delay_ms)).await;

                            attempt += 1;
                            continue;
                        }
                    }

                    // Non-retryable error or max retries exceeded
                    error!(
                        "Transaction failed permanently on immediate send (nonce: {}): {}",
                        expected_nonce, e
                    );
                    return Err(Error::Transport(e.to_string()));
                }
            }
        }

        Err(Error::Transport(
            "Max retries exceeded on immediate send".to_string(),
        ))
    }

    /// Track transaction confirmation asynchronously (fire-and-forget)
    /// This enables monitoring of parallel transactions without blocking the main flow
    fn track_transaction_confirmation_async(&self, tx_hash: TxHash, nonce: u64) {
        let provider = self.provider.clone();

        tokio::spawn(async move {
            let mut attempts = 0;
            const MAX_CONFIRMATION_ATTEMPTS: usize = 30;
            const CONFIRMATION_POLL_INTERVAL: Duration = Duration::from_millis(50);

            while attempts < MAX_CONFIRMATION_ATTEMPTS {
                tokio::time::sleep(CONFIRMATION_POLL_INTERVAL).await;
                attempts += 1;

                match provider.get_transaction_receipt(tx_hash).await {
                    Ok(Some(receipt)) => {
                        info!(
                            "[TRX SUCCESS] Transaction confirmed: {} (nonce: {}, attempt: {})",
                            tx_hash, nonce, attempts
                        );
                        info!(
                            "[GAS] consumed by transaction {}: {}",
                            tx_hash, receipt.gas_used
                        );
                        return; // Success - exit tracking
                    }
                    Ok(None) => {
                        debug!(
                            "Transaction receipt not yet available: {} (nonce: {}, attempt: {})",
                            tx_hash, nonce, attempts
                        );
                        // Continue polling
                    }
                    Err(e) => {
                        warn!(
                            "Error checking transaction receipt: {} (nonce: {}, attempt: {}): {}",
                            tx_hash, nonce, attempts, e
                        );
                        // Continue polling despite errors
                    }
                }
            }

            // Max attempts reached - log warning but don't fail
            warn!(
                "Transaction confirmation tracking timed out: {} (nonce: {}) after {} attempts",
                tx_hash, nonce, MAX_CONFIRMATION_ATTEMPTS
            );
        });
    }

    /// Refresh nonce from chain (used for recovery)
    async fn refresh_nonce_from_chain(&self, address: Address) -> Result<()> {
        let chain_nonce = self
            .provider
            .get_transaction_count(address)
            .await
            .map_err(|e| Error::Transport(format!("Failed to get chain nonce: {e}")))?;

        if let Some(nonce_mutex) = self.nonces.get(&address) {
            let mut nonce_guard = nonce_mutex.lock().await;
            *nonce_guard = chain_nonce;
            debug!("Refreshed nonce for {} to {}", address, chain_nonce);
        }

        Ok(())
    }

    /// Get total queued transactions (optimized for single wallet)
    pub async fn get_total_queued(&self) -> usize {
        // For single wallet deployment, we only have one queue
        if let Some(entry) = self.transaction_queues.iter().next() {
            let queue_guard = entry.value().lock().await;
            queue_guard.len()
        } else {
            0
        }
    }

    /// Get current queue utilization as a percentage (0.0 to 1.0)
    pub async fn get_queue_utilization(&self) -> f32 {
        let total_queued = self.get_total_queued().await;
        let max_transaction_queue_size = (self.config.channel_size as f32 * 0.2) as usize;
        total_queued as f32 / max_transaction_queue_size as f32
    }

    /// Get queue status for monitoring
    pub async fn get_queue_status(&self) -> (usize, usize, f32) {
        let total_queued = self.get_total_queued().await;
        let max_transaction_queue_size = (self.config.channel_size as f32 * 0.2) as usize;
        let utilization = total_queued as f32 / max_transaction_queue_size as f32;
        (total_queued, max_transaction_queue_size, utilization)
    }
}
