use crate::core::job_id::JobId;
use crate::gateway::arbitrum::transaction::helper::TransactionType;
use crate::gateway::arbitrum::transaction::TxLifecycleHooks;
use alloy::primitives::{Address, Bytes};
use anyhow::anyhow;
use governor::{Quota, RateLimiter};
use indexmap::IndexMap;
use std::fmt;
use std::future::Future;
use std::num::NonZeroU32;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::time::Duration;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::sync::{Mutex, RwLock};
use tracing::{error, info, instrument, warn};

// DOMAIN TYPES (Gateway Specifics)

/// Wrapper to make the generic Hook Debug-compatible and Cloneable via Arc.
#[derive(Clone)]
pub struct DynTxHook(pub Arc<dyn TxLifecycleHooks>);

impl fmt::Debug for DynTxHook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TxLifecycleHooks(dyn)")
    }
}

/// The actual unit of work submitted to the Mempool.
#[derive(Debug, Clone)]
pub struct GatewayTask {
    pub id: String,
    pub job_id: JobId,
    pub transaction_type: TransactionType,
    pub target: Address,
    pub calldata: Bytes,
    pub hook: DynTxHook,
}

impl MempoolItem for GatewayTask {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

// MEMPOOL ABSTRACTIONS

/// Items in the Mempool must implement this to be tracked by ID.
pub trait MempoolItem: Send + Sync + 'static + std::fmt::Debug {
    fn get_id(&self) -> String;
}

/// A throttled, self-healing, in-memory queue with observability.
#[derive(Clone)]
pub struct Mempool<T> {
    // The transport layer. Wrapped in RwLock to allow hot-swapping on failure.
    sender: Arc<RwLock<UnboundedSender<T>>>,

    // The receiving end. Protected by Mutex to ensure single-consumer exclusive access.
    // Wrapped in Option so we can "take" it during the run loop.
    receiver: Arc<Mutex<Option<UnboundedReceiver<T>>>>,

    // The "Sidecar" state. Tracks which IDs are pending and their insertion order.
    // IndexMap allows O(1) lookup of "Position in Queue".
    // Use of RwLock for making thread access safe, cannot use Dashmap here for O(1) lookups.
    tracker: Arc<RwLock<IndexMap<String, ()>>>,

    // Shared metric for total enqueued items (redundant with tracker, but faster for simple len)
    queue_len: Arc<AtomicUsize>,

    // Throughput Per Second configuration
    tps: u32,
}

impl<T> Mempool<T>
where
    T: MempoolItem,
{
    pub fn new(tps: u32) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();

        Self {
            sender: Arc::new(RwLock::new(sender)),
            receiver: Arc::new(Mutex::new(Some(receiver))),
            tracker: Arc::new(RwLock::new(IndexMap::new())),
            queue_len: Arc::new(AtomicUsize::new(0)),
            tps,
        }
    }

    // PRODUCER SIDE

    /// Robust Push:
    /// - Adds to tracking map (deduplication check).
    /// - Tries to send to the channel.
    /// - If worker/channel is dead, loops and waits for the worker to self-heal.
    #[instrument(skip(self, item), fields(id = %item.get_id()))]
    pub async fn push(&self, mut item: T) {
        let id = item.get_id();

        // Register in Tracker (Preserves Order & Deduplicates)
        {
            let mut tracker = self.tracker.write().await;
            if tracker.contains_key(&id) {
                warn!("Item already exists in mempool, ignoring duplicate push");
                return;
            }
            tracker.insert(id.clone(), ());
        }

        self.queue_len.fetch_add(1, Ordering::Relaxed);

        // Reliable Transport Loop
        loop {
            // Acquire current sender (Read lock is cheap)
            let sender = self.sender.read().await.clone();

            // Try to send
            match sender.send(item) {
                Ok(_) => {
                    return;
                }
                Err(return_err) => {
                    // Channel is closed (Worker died)
                    // We recover the item so it is not lost.
                    item = return_err.0;

                    error!("Mempool disconnected (worker down). Retrying in 1s...");

                    // Wait for Consumer to call refresh_channel()
                    // The item stays safely in memory in this variable 'item'
                    // until a new worker/channel is established.
                    tokio::time::sleep(Duration::from_millis(1000)).await;
                }
            }
        }
    }

    // CONSUMER SIDE (Self-Healing)

    /// Main entry point for the consumer.
    /// Runs indefinitely. If the internal logic crashes or the channel closes,
    /// it repairs the channel and restarts the logic automatically.
    pub async fn run_consumer<F, Fut>(&self, processor: F)
    where
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        info!(
            "Mempool consumer starting with rate limit: {} TPS",
            self.tps
        );

        let processor = Arc::new(processor);

        loop {
            // Run the worker logic
            let result = self.run_internal_logic(processor.clone()).await;

            // Handle Exit
            match result {
                Ok(_) => {
                    info!("Mempool consumer stopped gracefully.");
                    break;
                }
                Err(e) => {
                    error!(
                        "Mempool consumer crashed or channel closed: {:#}. Restarting...",
                        e
                    );

                    // 3. Self-Healing: Refresh the channel
                    self.refresh_channel().await;

                    // Backoff to prevent tight loop spinning
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
            }
        }
    }

    /// Internal logic with Rate Limiting and Isolation.
    /// Returns Err if the channel closes or supervisor fails.
    async fn run_internal_logic<F, Fut>(&self, processor: Arc<F>) -> anyhow::Result<()>
    where
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        // Claim Receiver
        let mut rx_guard = self.receiver.lock().await;
        let mut receiver = rx_guard
            .take()
            .ok_or_else(|| anyhow!("Could not claim receiver (already running?)"))?;
        drop(rx_guard);

        // Setup Rate Limiter
        let burst = NonZeroU32::new(self.tps).unwrap_or(NonZeroU32::MIN);
        let quota = Quota::per_second(burst).allow_burst(burst);
        let limiter = Arc::new(RateLimiter::direct(quota));

        info!("Mempool worker loop active.");

        // Supervisor Loop
        while let Some(item) = receiver.recv().await {
            let id = item.get_id();

            // Rate Limit
            limiter.until_ready().await;

            // Update State (Remove from tracker as we are about to process)
            {
                let mut tracker = self.tracker.write().await;
                tracker.shift_remove(&id);
            }
            self.queue_len.fetch_sub(1, Ordering::Relaxed);

            // Spawn Isolated Task (Crash Protection)
            let proc_clone = processor.clone();
            let id_clone = id.clone();

            // If a specific item processing crashes the business logic, the queue survives.
            let handle = tokio::spawn(async move {
                proc_clone(item).await;
            });

            // Monitor Result
            // We catch the panic here.
            // The main 'receiver' loop continues running.
            // The channel is NOT closed. The buffer is NOT dropped.
            if let Err(e) = handle.await {
                if e.is_panic() {
                    error!(id = %id_clone, "Worker PANIC during processing. Queue safe.");
                } else {
                    error!(id = %id_clone, "Worker task cancelled: {}", e);
                }
            }
        }

        Err(anyhow!("Internal channel closed unexpectedly"))
    }

    /// Self-Healing Utility: Replaces the broken channel with a new one.
    async fn refresh_channel(&self) {
        let (new_tx, new_rx) = mpsc::unbounded_channel();

        // Update Sender (Unblocks Producers)
        let mut tx_guard = self.sender.write().await;
        *tx_guard = new_tx;
        drop(tx_guard);

        // Update Receiver (Ready for next run loop)
        let mut rx_guard = self.receiver.lock().await;
        *rx_guard = Some(new_rx);
        drop(rx_guard);

        info!("Mempool channel refreshed and ready.");
    }

    // OBSERVABILITY
    pub fn len(&self) -> usize {
        self.queue_len.load(Ordering::Relaxed)
    }

    pub async fn is_empty(&self) -> bool {
        self.tracker.read().await.is_empty()
    }

    pub async fn contains(&self, id: &str) -> bool {
        self.tracker.read().await.contains_key(id)
    }

    pub async fn get_position(&self, id: &str) -> Option<usize> {
        self.tracker.read().await.get_index_of(id)
    }

    pub async fn get_queued_ids(&self) -> Vec<String> {
        self.tracker.read().await.keys().cloned().collect()
    }
}

// UNIT TESTS
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;
    use tokio::time::{sleep, Duration};

    // --- Mock Item ---
    #[derive(Debug, Clone)]
    struct MockTask {
        id: String,
        should_panic: bool,
    }

    impl MempoolItem for MockTask {
        fn get_id(&self) -> String {
            self.id.clone()
        }
    }

    // --- Test 1: Basic Push and FIFO Order ---
    #[tokio::test]
    async fn test_fifo_ordering() {
        let mempool = Mempool::<MockTask>::new(100);
        let processed_ids = Arc::new(Mutex::new(Vec::new()));

        // Spawn Consumer
        let mp_cons = mempool.clone();
        let ids_cons = processed_ids.clone();
        tokio::spawn(async move {
            mp_cons
                .run_consumer(move |task| {
                    let ids = ids_cons.clone();
                    async move {
                        ids.lock().await.push(task.id);
                    }
                })
                .await;
        });

        // Push Items
        mempool
            .push(MockTask {
                id: "1".into(),
                should_panic: false,
            })
            .await;
        mempool
            .push(MockTask {
                id: "2".into(),
                should_panic: false,
            })
            .await;
        mempool
            .push(MockTask {
                id: "3".into(),
                should_panic: false,
            })
            .await;

        // Wait for processing
        sleep(Duration::from_millis(100)).await;

        let results = processed_ids.lock().await.clone();
        assert_eq!(results, vec!["1", "2", "3"]);
        assert_eq!(mempool.len(), 0);
    }

    // --- Test 2: Deduplication ---
    #[tokio::test]
    async fn test_deduplication() {
        let mempool = Mempool::<MockTask>::new(100);

        // Push same ID twice
        mempool
            .push(MockTask {
                id: "A".into(),
                should_panic: false,
            })
            .await;
        mempool
            .push(MockTask {
                id: "A".into(),
                should_panic: false,
            })
            .await;

        assert_eq!(mempool.len(), 1);
        assert!(mempool.contains("A").await);
    }

    // --- Test 3: Observability & Positioning ---
    #[tokio::test]
    async fn test_observability() {
        let mempool = Mempool::<MockTask>::new(1); // Very slow consumer to queue items up

        // We do NOT spawn a consumer, so items sit in the queue
        mempool
            .push(MockTask {
                id: "A".into(),
                should_panic: false,
            })
            .await;
        mempool
            .push(MockTask {
                id: "B".into(),
                should_panic: false,
            })
            .await;
        mempool
            .push(MockTask {
                id: "C".into(),
                should_panic: false,
            })
            .await;

        assert_eq!(mempool.len(), 3);
        assert_eq!(mempool.get_position("A").await, Some(0));
        assert_eq!(mempool.get_position("B").await, Some(1));
        assert_eq!(mempool.get_position("C").await, Some(2));
        assert_eq!(mempool.get_position("Z").await, None);

        let ids = mempool.get_queued_ids().await;
        assert_eq!(ids, vec!["A", "B", "C"]);
    }

    // --- Test 4: Task Isolation (Panic Recovery) ---
    #[tokio::test]
    async fn test_panic_resilience() {
        let mempool = Mempool::<MockTask>::new(50);
        let processed_count = Arc::new(AtomicUsize::new(0));

        let mp_cons = mempool.clone();
        let counter = processed_count.clone();

        tokio::spawn(async move {
            mp_cons
                .run_consumer(move |task| {
                    let c = counter.clone();
                    async move {
                        if task.should_panic {
                            panic!("Simulated Crash");
                        }
                        c.fetch_add(1, Ordering::Relaxed);
                    }
                })
                .await;
        });

        mempool
            .push(MockTask {
                id: "good_1".into(),
                should_panic: false,
            })
            .await;
        mempool
            .push(MockTask {
                id: "bad".into(),
                should_panic: true,
            })
            .await; // Should crash sub-task
        mempool
            .push(MockTask {
                id: "good_2".into(),
                should_panic: false,
            })
            .await;

        sleep(Duration::from_millis(200)).await;

        // "bad" crashed, but "good_2" should still process
        assert_eq!(processed_count.load(Ordering::Relaxed), 2);
        assert_eq!(mempool.len(), 0);
    }

    // --- Test 5: Self-Healing Consumer Restart ---
    #[tokio::test]
    async fn test_consumer_restart() {
        // This test simulates a condition where the inner logic fails
        // (simulated by manually aborting the channel, though harder to test exactly without mocks).
        // Instead, we verify the refresh_channel mechanism works by manually triggering it
        // and ensuring flow continues.

        let mempool = Mempool::<MockTask>::new(50);
        let processed = Arc::new(AtomicBool::new(false));

        // Manually break the channel to simulate a dead state
        mempool.refresh_channel().await;

        // Spawn consumer (should pick up the new channel)
        let mp_cons = mempool.clone();
        let p = processed.clone();
        tokio::spawn(async move {
            mp_cons
                .run_consumer(move |_task| {
                    let p_inner = p.clone();
                    async move {
                        p_inner.store(true, Ordering::Relaxed);
                    }
                })
                .await;
        });

        sleep(Duration::from_millis(50)).await;

        // Push item
        mempool
            .push(MockTask {
                id: "recover".into(),
                should_panic: false,
            })
            .await;

        sleep(Duration::from_millis(100)).await;

        assert!(
            processed.load(Ordering::Relaxed),
            "Consumer should have processed item after manual refresh"
        );
    }

    #[tokio::test]
    async fn test_tps_rate_limiting() {
        // Setup: 10 TPS.
        // Implementation uses allow_burst(10), so:
        // - First 10 items: Instant (Burst)
        // - Next 10 items: Throttled (1 every 100ms)
        // Total expected time for 20 items: ~1.0 seconds (plus small overhead)
        let tps = 10;
        let mempool = Mempool::<MockTask>::new(tps);
        let processed_count = Arc::new(AtomicUsize::new(0));

        let mp_cons = mempool.clone();
        let counter = processed_count.clone();

        tokio::spawn(async move {
            mp_cons
                .run_consumer(move |_task| {
                    let c = counter.clone();
                    async move {
                        c.fetch_add(1, Ordering::Relaxed);
                    }
                })
                .await;
        });

        // Push 20 items
        let total_items = 20;
        let start_time = std::time::Instant::now();

        for i in 0..total_items {
            mempool
                .push(MockTask {
                    id: format!("{}", i),
                    should_panic: false,
                })
                .await;
        }

        // Wait until all are processed
        while processed_count.load(Ordering::Relaxed) < total_items {
            sleep(Duration::from_millis(10)).await;
        }

        let elapsed = start_time.elapsed();

        // Assertions
        println!("Processed {} items in {:?}", total_items, elapsed);

        // We expect it to be at least 900ms.
        // Logic:
        // - Items 0-9: Instant.
        // - Item 10: Wait 100ms
        // ...
        // - Item 19: Wait 100ms
        // Total wait ~10 * 100ms = 1000ms.
        // We use 900ms to account for slight timing variances.
        assert!(
            elapsed.as_millis() >= 900,
            "Processing was too fast! Rate limiter failed."
        );

        // It shouldn't be insanely slow either (e.g., < 2000ms)
        assert!(elapsed.as_millis() < 2500, "Processing was too slow!");
    }

    // --- Test 7: Sustained Rate Limiting (Constant Flow) ---
    #[tokio::test]
    async fn test_sustained_rate_limiting() {
        // Configuration
        let tps = 20;
        let test_duration_seconds = 5;
        // We push enough items to keep the queue busy for the whole test + buffer
        // (20 tps * 5s) + 20 burst + buffer = ~150 items
        let total_items_to_push = 150;

        // 1. Setup Mempool
        let mempool = Mempool::<MockTask>::new(tps);
        let processed_count = Arc::new(AtomicUsize::new(0));

        let mp_cons = mempool.clone();
        let counter = processed_count.clone();

        // 2. Spawn Consumer
        tokio::spawn(async move {
            mp_cons
                .run_consumer(move |_task| {
                    let c = counter.clone();
                    async move {
                        c.fetch_add(1, Ordering::Relaxed);
                    }
                })
                .await;
        });

        // Blast the queue with items (Instant because it's unbounded)
        let start_push = std::time::Instant::now();
        for i in 0..total_items_to_push {
            mempool
                .push(MockTask {
                    id: format!("load_{}", i),
                    should_panic: false,
                })
                .await;
        }
        println!(
            "Filled queue with {} items in {:?}",
            total_items_to_push,
            start_push.elapsed()
        );

        // Monitor Loop (Check every second)
        let mut previous_count = 0;

        println!("Starting monitor loop (Target: ~{} items/sec)...", tps);

        for s in 1..=test_duration_seconds {
            // Wait 1 second
            sleep(Duration::from_secs(1)).await;

            let current_count = processed_count.load(Ordering::Relaxed);
            let delta = current_count - previous_count;

            println!("T={}s | Total: {} | Delta: {}", s, current_count, delta);

            if s == 1 {
                // FIRST SECOND BEHAVIOR:
                // Because of .allow_burst(tps), we expect:
                // Burst (20) + Refill for 1 second (20) = ~40 items total.
                // We allow a margin of error (+/- 5).
                assert!(
                    current_count >= 35 && current_count <= 45,
                    "T=1s expected ~40 items (Burst+Flow), got {}",
                    current_count
                );
            } else {
                // SUBSEQUENT SECONDS BEHAVIOR:
                // The burst is exhausted. We should see STRICTLY the TPS rate.
                // Target: 20. Margin: +/- 2 (very stable).
                assert!(
                    delta >= 18 && delta <= 22,
                    "T={}s unstable rate! Expected ~{}, got {}",
                    s,
                    tps,
                    delta
                );
            }

            previous_count = current_count;
        }
    }

    // --- Test 8: Full Crash & Recovery Cycle ---
    #[tokio::test]
    async fn test_consumer_crash_and_recovery() {
        let mempool = Mempool::<MockTask>::new(50);
        let processed_ids = Arc::new(Mutex::new(Vec::new()));

        // Spawn the Long-Running Consumer
        let mp_cons = mempool.clone();
        let ids_clone = processed_ids.clone();

        // We spawn this and expect it to live forever, even through crashes
        tokio::spawn(async move {
            mp_cons
                .run_consumer(move |task| {
                    let ids = ids_clone.clone();
                    async move {
                        info!("Processing: {}", task.id);
                        ids.lock().await.push(task.id);
                    }
                })
                .await;
        });

        // Phase 1: Normal Operation
        mempool
            .push(MockTask {
                id: "tx_1".into(),
                should_panic: false,
            })
            .await;

        // Wait for tx_1
        sleep(Duration::from_millis(50)).await;
        assert_eq!(processed_ids.lock().await.as_slice(), &["tx_1"]);

        // Phase 2: SIMULATE CRASH
        // Calling refresh_channel() replaces the sender/receiver in the struct.
        // Crucially, this drops the old sender.
        // The *running* consumer (holding the old receiver) will see the channel close (return None).
        // It should catch this, log "Restarting...", and loop back to claim the NEW receiver.
        info!("--- SIMULATING CHANNEL CRASH ---");
        mempool.refresh_channel().await;

        // -------------------
        // When we call `mempool.refresh_channel()` manually in the test, we create "Channel B".
        // The Consumer (background task) sees the old "Channel A" die, wakes up, and
        // triggers its own self-healing logic: it calls `refresh_channel()` again to create "Channel C".
        //
        // This creates a Race Condition specific to this test:
        // 1. Test creates Channel B.
        // 2. Test pushes item into Channel B.
        // 3. Consumer finishes its reaction and creates Channel C.
        // 4. Consumer replaces Channel B with Channel C, DROPPING Channel B's receiver.
        //
        // Result: The item sitting in Channel B is deleted from memory before it can be read.
        //
        // We must wait (sleep) to ensure the Consumer has finished its "Double Refresh" cycle
        // (Gen A -> Gen B -> Gen C) before we push data.
        // In production, this race does not exist because only the Consumer initiates refreshes.
        sleep(Duration::from_millis(1000)).await;

        // Phase 3: Push During/After Recovery
        // This push goes to the NEW channel.
        // If the consumer successfully restarted, it should have claimed the NEW receiver.
        info!("--- PUSHING POST-CRASH TASK ---");
        mempool
            .push(MockTask {
                id: "tx_after_crash".into(),
                should_panic: false,
            })
            .await;

        // Wait for processing
        sleep(Duration::from_millis(200)).await;

        // Assert: Both tasks processed
        let final_results = processed_ids.lock().await.clone();
        assert_eq!(final_results, vec!["tx_1", "tx_after_crash"]);

        info!("Test Passed: Consumer survived channel death and resumed processing.");
    }

    #[tokio::test]
    async fn test_consumer_crash_recovery_deterministic() {
        // Setup
        let mempool = Mempool::<MockTask>::new(50);
        let processed_ids = Arc::new(Mutex::new(Vec::new()));

        let mp_cons = mempool.clone();
        let ids_clone = processed_ids.clone();

        tokio::spawn(async move {
            mp_cons
                .run_consumer(move |task| {
                    let ids = ids_clone.clone();
                    async move {
                        info!("Processing: {}", task.id);
                        ids.lock().await.push(task.id);
                    }
                })
                .await;
        });

        // Initial Push
        mempool
            .push(MockTask {
                id: "tx_init".into(),
                should_panic: false,
            })
            .await;

        // Wait for initial processing (basic sanity check)
        loop {
            if processed_ids.lock().await.contains(&"tx_init".to_string()) {
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        // TRIGGER CRASH
        info!("--- SIMULATING CRASH ---");
        mempool.refresh_channel().await;

        // THE PROBE LOOP (The "No Wait" Solution)
        // Instead of sleeping 1s, we keep pushing "Probes" until the system heals.
        // This handles the "Double Refresh" race: if a probe is lost, we just send another.
        let mut probe_idx = 0;
        loop {
            probe_idx += 1;
            let probe_id = format!("probe_{}", probe_idx);

            info!("Sending {}", probe_id);
            mempool
                .push(MockTask {
                    id: probe_id.clone(),
                    should_panic: false,
                })
                .await;

            // Wait a tiny bit to see if this probe lands
            tokio::time::sleep(Duration::from_millis(50)).await;

            if processed_ids.lock().await.contains(&probe_id) {
                info!("System recovered! {} was processed.", probe_id);
                break;
            }
        }

        // CRITICAL PUSH
        // Now that we PROVED the system is stable (probe got through),
        // we push the transaction we actually care about.
        info!("--- PUSHING CRITICAL TASK ---");
        mempool
            .push(MockTask {
                id: "tx_critical".into(),
                should_panic: false,
            })
            .await;

        // Verify Critical Task
        // We use a poll loop here too to avoid arbitrary sleeps
        let start = std::time::Instant::now();
        loop {
            let ids = processed_ids.lock().await;
            if ids.contains(&"tx_critical".to_string()) {
                break;
            }
            drop(ids);

            if start.elapsed() > Duration::from_secs(2) {
                panic!("Timeout waiting for critical task");
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        let final_results = processed_ids.lock().await.clone();
        assert!(final_results.contains(&"tx_init".to_string()));
        assert!(final_results.contains(&"tx_critical".to_string()));

        info!("Test Passed: System self-healed and processed critical transaction.");
    }
}
