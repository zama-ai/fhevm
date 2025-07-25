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
    request_id: Option<String>,
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

    /// Queue a transaction for sequential processing with optional request ID for logging
    /// This method is NON-BLOCKING and returns immediately
    pub async fn send_transaction_queued(
        &self,
        mut tx: TransactionRequest,
        request_id: Option<String>,
    ) -> Result<TxHash> {
        let request_context = request_id.as_deref().unwrap_or("unknown");
        let request_id_clone = request_id.clone();

        let address = if let Some(addr) = tx.from {
            debug!(
                "[TRX QUEUE] {}: Using provided wallet address: {}",
                request_context, addr
            );
            addr
        } else {
            let wallet_address = self.wallet_address;
            tx.from = Some(wallet_address); // Set it for queue processing
            info!(
                "[TRX QUEUE] {}: Using config wallet address for sequential processing: {}",
                request_context, wallet_address
            );
            wallet_address
        };

        let (result_sender, result_receiver) = oneshot::channel();
        let pending_tx = PendingTransaction {
            tx_request: tx,
            result_sender,
            request_id: request_id_clone,
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

        debug!(
            "[TRX QUEUE] {}: Queue status - total: {}/{} ({:.1}% utilization)",
            request_context,
            total_queued,
            max_transaction_queue_size,
            utilization * 100.0
        );

        // Use config threshold for backpressure signaling
        let backpressure_threshold = self.config.pending_events_queue_slowdown_threshold;
        let critical_threshold = 0.95; // 95% for critical (close to full)

        // Send backpressure signals based on queue utilization
        if let Some(ref tx) = self.backpressure_tx {
            if utilization >= critical_threshold {
                let _ = tx.send(BackpressureSignal::QueueCritical);
                warn!(
                    "[BACKPRESSURE] {}: Sent QueueCritical signal - {:.1}% utilization (threshold: {:.1}%)",
                    request_context,
                    utilization * 100.0,
                    critical_threshold * 100.0
                );
            } else if utilization >= backpressure_threshold {
                let _ = tx.send(BackpressureSignal::QueueFull);
                warn!(
                    "[BACKPRESSURE] {}: Sent QueueFull signal - {:.1}% utilization (threshold: {:.1}%)",
                    request_context,
                    utilization * 100.0,
                    backpressure_threshold * 100.0
                );
            } else if utilization < backpressure_threshold {
                let _ = tx.send(BackpressureSignal::QueueAvailable);
                debug!(
                    "[BACKPRESSURE] {}: Sent QueueAvailable signal - {:.1}% utilization",
                    request_context,
                    utilization * 100.0
                );
            }
        }

        // Only reject at 100% capacity (hard limit)
        if total_queued >= max_transaction_queue_size {
            warn!(
                "[TRX QUEUE] {}: REJECTED - Queue at capacity: {} total transactions (max: {})",
                request_context, total_queued, max_transaction_queue_size
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
                    "[TRX QUEUE] {}: REJECTED - Wallet {} queue overflow: {} transactions (max: {})",
                    request_context,
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

            let queue_position = queue_guard.len() + 1; // Position after adding
            queue_guard.push_back(pending_tx);
            info!(
                "[TRX QUEUED] {}: Position #{} in wallet {} queue ({} total system-wide)",
                request_context,
                queue_position,
                address,
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
                    let result = self
                        .process_single_transaction(pending.tx_request, pending.request_id)
                        .await;

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
    async fn process_single_transaction(
        &self,
        mut tx: TransactionRequest,
        request_id: Option<String>,
    ) -> Result<TxHash> {
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

        match self
            .send_transaction_immediately(tx, nonce, request_id)
            .await
        {
            Ok(tx_hash) => {
                info!(
                    "[PARALLEL] Transaction sent: {} (nonce: {}, in-flight: {})",
                    tx_hash,
                    nonce,
                    in_flight_count + 1
                );
                // Receipt confirmation is handled by DecryptionAdapter's analyze_receipt_with_retry_logic
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

    /// Send transaction immediately with optimized gas settings for parallel processing
    /// Handles gas estimation and 30% boost centrally for all transaction types
    async fn send_transaction_immediately(
        &self,
        mut tx: TransactionRequest,
        expected_nonce: u64,
        request_id: Option<String>,
    ) -> Result<TxHash> {
        let request_context = request_id.as_deref().unwrap_or("unknown");

        // Set nonce for immediate sending
        tx.nonce = Some(expected_nonce);
        debug!(
            "[TRX SEND] {}: Preparing immediate send with nonce: {}",
            request_context, expected_nonce
        );

        // Always estimate gas fresh with 100ms retry for resilience
        let gas_estimation = match self.provider.estimate_gas(tx.clone()).await {
            Ok(estimation) => {
                debug!(
                    "[GAS ESTIMATE] {}: Fresh estimation successful (nonce: {}): {} gas",
                    request_context, expected_nonce, estimation
                );
                estimation
            }
            Err(first_error) => {
                warn!(
                    "[GAS ESTIMATE] {}: Failed for nonce {}: {} - retrying in 100ms",
                    request_context, expected_nonce, first_error
                );

                // Wait 100ms and retry once
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;

                match self.provider.estimate_gas(tx.clone()).await {
                    Ok(estimation) => {
                        info!(
                            "[GAS ESTIMATE] {}: Retry succeeded for nonce {}: {} gas",
                            request_context, expected_nonce, estimation
                        );
                        estimation
                    }
                    Err(retry_error) => {
                        error!(
                            "[GAS ESTIMATE] {}: Failed twice for nonce {}: first={}, retry={} - using fallback",
                            request_context, expected_nonce, first_error, retry_error
                        );

                        // Fallback strategies
                        if let Some(existing_gas) = tx.gas {
                            info!(
                                "[GAS FALLBACK] {}: Using existing gas limit: {} for nonce: {}",
                                request_context, existing_gas, expected_nonce
                            );
                            existing_gas
                        } else {
                            let fallback_gas = 2_000_000u64; // High fallback for complex decryption contract calls (observed: ~1.8M gas)
                            warn!(
                                "[GAS FALLBACK] {}: Using fallback gas limit: {} for nonce: {} due to estimation failures",
                                request_context, fallback_gas, expected_nonce
                            );
                            fallback_gas
                        }
                    }
                }
            }
        };

        // Apply configurable gas boost to estimated or fallback gas
        let gas_boost_percent = self.config.gas_boost_percent;
        let boosted_gas = gas_estimation * (100 + gas_boost_percent as u64) / 100;
        tx.gas = Some(boosted_gas);
        info!(
            "[GAS BOOST] {}: Applied {}% boost (nonce: {}): {} → {} gas",
            request_context, gas_boost_percent, expected_nonce, gas_estimation, boosted_gas
        );

        // Single send attempt - let receipt analysis handle any failures
        match self.provider.send_transaction(tx).await {
            Ok(pending_tx) => {
                let tx_hash = *pending_tx.tx_hash();
                info!(
                    "[TRX SENT] {}: Transaction sent successfully - hash: {} (nonce: {})",
                    request_context, tx_hash, expected_nonce
                );
                Ok(tx_hash)
            }
            Err(e) => {
                warn!(
                    "[TRX FAILED] {}: Immediate send failed (nonce: {}): {} - will be handled by receipt analysis",
                    request_context, expected_nonce, e
                );
                Err(Error::Transport(e.to_string()))
            }
        }
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

    /// Retry a transaction with increased gas limit (called by DecryptionAdapter on out-of-gas)
    /// This maintains decoupling: adapter detects issue, nonce manager handles retry
    /// Uses async queue processing to maintain proper sequential nonce management
    pub async fn retry_transaction_with_gas_bump(
        &self,
        mut tx: TransactionRequest,
        gas_bump_percent: u32,
    ) -> Result<TxHash> {
        // Increase gas limit by specified percentage
        if let Some(gas_limit) = tx.gas {
            tx.gas = Some(gas_limit * (100 + gas_bump_percent) as u64 / 100);
            info!(
                "Retrying transaction with {}% gas bump: {} -> {} (async queue)",
                gas_bump_percent,
                gas_limit,
                tx.gas.unwrap()
            );
        }

        // Queue the retry transaction (maintains proper async architecture)
        // This ensures retries follow the same sequential nonce management
        // and backpressure handling as initial transactions
        self.send_transaction_queued(tx, None).await
    }

    /// Get queue status for monitoring
    pub async fn get_queue_status(&self) -> (usize, usize, f32) {
        let total_queued = self.get_total_queued().await;
        let max_transaction_queue_size = (self.config.channel_size as f32 * 0.2) as usize;
        let utilization = total_queued as f32 / max_transaction_queue_size as f32;
        (total_queued, max_transaction_queue_size, utilization)
    }
}
