use crate::core::{errors::EventProcessingError, job_id::JobId};
use crate::gateway::arbitrum::transaction::helper::TransactionType;
use crate::gateway::arbitrum::transaction::TxLifecycleHooks;
use crate::metrics;
use alloy::primitives::{Address, Bytes};
use governor::{Quota, RateLimiter};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::future::Future;
use std::num::NonZeroU32;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tracing::{error, info, instrument, warn};

pub struct TxThrottlers {
    pub input_proof_tx_throttler: TxThrottlingSender<GatewayTxTask>,
    pub input_proof_tx_worker: TxThrottlingWorker<GatewayTxTask>,
    pub user_decrypt_tx_throttler: TxThrottlingSender<GatewayTxTask>,
    pub user_decrypt_tx_worker: TxThrottlingWorker<GatewayTxTask>,
    pub public_decrypt_tx_throttler: TxThrottlingSender<GatewayTxTask>,
    pub public_decrypt_tx_worker: TxThrottlingWorker<GatewayTxTask>,
}

impl TxThrottlers {
    pub fn new(
        input_proof_tx_throttler: TxThrottlingSender<GatewayTxTask>,
        input_proof_tx_worker: TxThrottlingWorker<GatewayTxTask>,
        user_decrypt_tx_throttler: TxThrottlingSender<GatewayTxTask>,
        user_decrypt_tx_worker: TxThrottlingWorker<GatewayTxTask>,
        public_decrypt_tx_throttler: TxThrottlingSender<GatewayTxTask>,
        public_decrypt_tx_worker: TxThrottlingWorker<GatewayTxTask>,
    ) -> Self {
        Self {
            input_proof_tx_throttler,
            input_proof_tx_worker,
            user_decrypt_tx_throttler,
            user_decrypt_tx_worker,
            public_decrypt_tx_throttler,
            public_decrypt_tx_worker,
        }
    }
}

pub struct TxSenders {
    pub input_proof_tx_throttler: TxThrottlingSender<GatewayTxTask>,
    pub user_decrypt_tx_throttler: TxThrottlingSender<GatewayTxTask>,
    pub public_decrypt_tx_throttler: TxThrottlingSender<GatewayTxTask>,
}

impl TxSenders {
    pub fn new(
        input_proof_tx_throttler: TxThrottlingSender<GatewayTxTask>,
        user_decrypt_tx_throttler: TxThrottlingSender<GatewayTxTask>,
        public_decrypt_tx_throttler: TxThrottlingSender<GatewayTxTask>,
    ) -> Self {
        Self {
            input_proof_tx_throttler,
            user_decrypt_tx_throttler,
            public_decrypt_tx_throttler,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TxThrottlingType {
    InputProof,
    UserDecrypt,
    PublicDecrypt,
}

impl TxThrottlingType {
    fn as_metrics_type(&self) -> metrics::QueueType {
        match self {
            TxThrottlingType::InputProof => metrics::QueueType::InputProofTxThrottler,
            TxThrottlingType::UserDecrypt => metrics::QueueType::UserDecryptTxThrottler,
            TxThrottlingType::PublicDecrypt => metrics::QueueType::PublicDecryptTxThrottler,
        }
    }
}

impl fmt::Display for TxThrottlingType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TxThrottlingType::InputProof => write!(f, "input_proof_tx_throttler"),
            TxThrottlingType::UserDecrypt => write!(f, "user_decrypt_tx_throttler"),
            TxThrottlingType::PublicDecrypt => write!(f, "public_decrypt_tx_throttler"),
        }
    }
}

// DOMAIN TYPES
#[derive(Clone)]
pub struct DynTxHook(pub Arc<dyn TxLifecycleHooks>);

impl fmt::Debug for DynTxHook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TxLifecycleHooks(dyn)")
    }
}

impl DynTxHook {
    /// Helper to easily access the inner trait object reference
    pub fn as_inner(&self) -> &dyn TxLifecycleHooks {
        self.0.as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct GatewayTxTask {
    pub id: String,
    pub job_id: JobId,
    pub transaction_type: TransactionType,
    pub target: Address,
    pub calldata: Bytes,
    pub hook: DynTxHook,
}

impl MemoryThrottlerItem for GatewayTxTask {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

// THROTTLER ABSTRACTIONS
pub trait MemoryThrottlerItem: Send + Sync + 'static + std::fmt::Debug {
    fn get_id(&self) -> String;
}

/// The Producer Handle.
#[derive(Clone)]
pub struct TxThrottlingSender<T> {
    tx_type: TxThrottlingType,
    // Bounded Channel carrying the Payload T
    sender: mpsc::Sender<T>,
    tracker: Arc<RwLock<IndexMap<String, ()>>>,
    soft_capacity: usize,
}

// NOTE: There must be at least one instance of this and hence the reciever for the queue channel to stay alive.
/// The Background Worker.
pub struct TxThrottlingWorker<T> {
    tx_type: TxThrottlingType,
    receiver: mpsc::Receiver<T>,
    // Shared reference to the same tracker to remove items when processed
    tracker: Arc<RwLock<IndexMap<String, ()>>>,
    tps: u32,
    // Control channel for dynamic TPS updates (optional)
    control_rx: Option<mpsc::Receiver<u32>>,
}

impl<T> TxThrottlingSender<T>
where
    T: MemoryThrottlerItem + Send + Sync + 'static,
{
    pub fn new(
        tx_type: TxThrottlingType,
        capacity: usize,
        capacity_safety_margin: usize,
        tps: u32,
        enable_dynamic_rate_limiting: bool,
    ) -> (Self, TxThrottlingWorker<T>, Option<mpsc::Sender<u32>>) {
        let (sender, receiver) = mpsc::channel(capacity);
        let tracker = Arc::new(RwLock::new(IndexMap::new()));
        // The safety margin (Headroom). is_queue_full returns true when (capacity - capacity_safety_margin) is reached.
        // Useful to avoid the TOCTOU race condition when bouncing from the api routes.
        let soft_capacity = capacity.saturating_sub(capacity_safety_margin);

        // Create control channel if dynamic rate limiting is enabled
        let (control_tx, control_rx) = if enable_dynamic_rate_limiting {
            let (tx, rx) = mpsc::channel(1);
            (Some(tx), Some(rx))
        } else {
            (None, None)
        };

        let throttler = Self {
            tx_type,
            sender,
            tracker: tracker.clone(),
            soft_capacity,
        };

        let worker = TxThrottlingWorker {
            tx_type,
            receiver,
            tracker,
            tps,
            control_rx,
        };

        (throttler, worker, control_tx)
    }

    /// Try to push an item.
    /// 1. Locks Map.
    /// 2. Checks Deduplication.
    /// 3. Tries to push to Channel (Bounded).
    /// 4. If success, inserts ID into Map.
    #[instrument(skip(self, item), fields(id = %item.get_id()))]
    pub async fn push(&self, item: T) -> anyhow::Result<(), EventProcessingError> {
        let id = item.get_id();

        // Acquire Write Lock
        // We hold this lock during the send to ensure consistency between Map and Channel.
        // If the worker is fast, it waits for this lock before it can remove the item.
        let mut tracker = self.tracker.write().await;

        // Try Send (Bounded check)
        // Dedup not necessary.
        match self.sender.try_send(item) {
            Ok(_) => {
                // Success: Add ID to tracker
                tracker.insert(id, ());
                metrics::queue::increment_queue_size(self.tx_type.as_metrics_type());
                Ok(())
            }
            Err(mpsc::error::TrySendError::Full(_)) => {
                warn!("Throttler queue is full! Bouncing request.");
                Err(EventProcessingError::QueueFull)
            }
            Err(mpsc::error::TrySendError::Closed(_)) => {
                // This flow is supposed to be not reachable.
                // Note: Self healing infinite loop or auto restart within kubernetes should be implemented.
                // Following die-fast principle, a restart should restart as soon as possible.
                error!(
                    "CRITICAL: Throttler channel is closed! Worker is dead, Relayer must restart."
                );
                Err(EventProcessingError::ChannelClosed)
            }
        }
    }

    // OBSERVABILITY

    pub async fn len(&self) -> usize {
        self.tracker.read().await.len()
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

    /// SOFT CHECK: Used by API to bounce requests early.
    /// Returns TRUE if we have reached the "High Water Mark".
    ///
    /// Logic:
    /// If Capacity = 100, Buffer = 10.
    /// We say "Full" when Len >= 90.
    /// The remaining 10 slots are reserved for Concurrent post requests (Race conditions).
    pub async fn is_queue_full(&self) -> bool {
        let len = self.len().await;
        len >= self.soft_capacity
    }
}

impl<T> TxThrottlingWorker<T>
where
    T: MemoryThrottlerItem + Send + Sync + 'static,
{
    /// Helper to create a rate limiter with given TPS
    fn create_limiter(
        tps: u32,
    ) -> Arc<
        RateLimiter<
            governor::state::direct::NotKeyed,
            governor::state::InMemoryState,
            governor::clock::DefaultClock,
        >,
    > {
        let burst = NonZeroU32::new(tps).unwrap_or(NonZeroU32::MIN);
        let quota = Quota::per_second(burst).allow_burst(burst);
        Arc::new(RateLimiter::direct(quota))
    }

    pub async fn run_consumer<F, Fut>(mut self, processor: F)
    where
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let dynamic_rate_limiting_enabled = self.control_rx.is_some();
        info!(
            "Throttler Worker started. TPS: {}, Capacity: Bounded, Dynamic rate limiting: {}",
            self.tps, dynamic_rate_limiting_enabled
        );

        let mut limiter = Self::create_limiter(self.tps);
        let processor = Arc::new(processor);

        loop {
            // Handle control channel conditionally
            let control_fut = async {
                match &mut self.control_rx {
                    Some(rx) => rx.recv().await,
                    None => std::future::pending().await, // Never resolves if disabled
                }
            };

            tokio::select! {
                Some(item) = self.receiver.recv() => {
                    let id = item.get_id();

                    // 1. Rate Limit
                    limiter.until_ready().await;

                    // 2. Remove from Tracker
                    // We remove it *before* processing so the position list is accurate
                    // (it is no longer "waiting" in queue, it is "processing")
                    {
                        let mut tracker = self.tracker.write().await;
                        tracker.shift_remove(&id);
                        metrics::queue::decrement_queue_size(self.tx_type.as_metrics_type());
                    }

                    // 3. Process (Isolated)
                    let proc_clone = processor.clone();

                    tokio::spawn(async move {
                        // If this panics, the worker loop survives.
                        proc_clone(item).await;
                    });
                }

                Some(new_tps) = control_fut => {
                    info!("Throttler rate limit updated: {} TPS -> {} TPS", self.tps, new_tps);
                    self.tps = new_tps;
                    limiter = Self::create_limiter(new_tps);
                }

                else => {
                    info!("Throttler Worker stopping (All producers dropped).");
                    break;
                }
            }
        }
    }
}

// UNIT TESTS
#[cfg(test)]
mod tests {
    use super::*;
    use prometheus::Registry;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex, Once,
    };
    use tokio::time::{sleep, Duration};

    // This static ensures the closure inside call_once runs only one time per process
    static INIT: Once = Once::new();

    /// Call this at the start of every test
    fn init_metrics_once() {
        INIT.call_once(|| {
            let registry = Registry::new();
            // This registers the global static metrics.
            // Doing this more than once would panic.
            crate::metrics::init_queue_metrics(&registry);
        });
    }

    #[derive(Debug, Clone)]
    struct MockTask {
        id: String,
    }

    impl MemoryThrottlerItem for MockTask {
        fn get_id(&self) -> String {
            self.id.clone()
        }
    }

    // --- Test 1: FIFO Ordering & Tracking ---
    #[tokio::test]
    async fn test_fifo_ordering() {
        init_metrics_once();
        let (throttler, worker, _control_tx) =
            TxThrottlingSender::<MockTask>::new(TxThrottlingType::InputProof, 10, 0, 100, false);
        let processed = Arc::new(Mutex::new(Vec::new()));
        let processed_clone = processed.clone();

        tokio::spawn(async move {
            worker
                .run_consumer(move |task| {
                    let p = processed_clone.clone();
                    async move {
                        p.lock().unwrap().push(task.id);
                    }
                })
                .await;
        });

        throttler.push(MockTask { id: "1".into() }).await.unwrap();
        throttler.push(MockTask { id: "2".into() }).await.unwrap();

        sleep(Duration::from_millis(100)).await;

        assert_eq!(*processed.lock().unwrap(), vec!["1", "2"]);
        assert_eq!(throttler.len().await, 0);
    }

    // --- Test 2: Deduplication ---
    #[tokio::test]
    async fn test_deduplication() {
        init_metrics_once();
        let (throttler, _worker, _control_tx) =
            TxThrottlingSender::<MockTask>::new(TxThrottlingType::InputProof, 10, 0, 100, false);

        throttler.push(MockTask { id: "A".into() }).await.unwrap();

        // This should be ignored
        throttler.push(MockTask { id: "A".into() }).await.unwrap();

        assert_eq!(throttler.len().await, 1);
    }

    // --- Test 3: Queue Full (Backpressure) ---
    #[tokio::test]
    async fn test_queue_full() {
        init_metrics_once();
        // Capacity = 1
        let (throttler, _worker, _control_tx) =
            TxThrottlingSender::<MockTask>::new(TxThrottlingType::InputProof, 1, 0, 100, false);

        // 1. Push OK
        assert!(throttler.push(MockTask { id: "1".into() }).await.is_ok());

        // 2. Push Fail (Full)
        let err = throttler.push(MockTask { id: "2".into() }).await;
        assert!(err.is_err());
        assert_eq!(
            err.unwrap_err().to_string(),
            "Relayer internal queue is full"
        );

        // Map should only have "1"
        assert!(throttler.contains("1").await);
        assert!(!throttler.contains("2").await);
    }

    // --- Test 4: Position Tracking ---
    #[tokio::test]
    async fn test_position_tracking() {
        init_metrics_once();
        // Very slow rate (1 TPS) to ensure items stay in queue
        let (throttler, worker, _control_tx) =
            TxThrottlingSender::<MockTask>::new(TxThrottlingType::InputProof, 10, 0, 1, false);

        throttler.push(MockTask { id: "A".into() }).await.unwrap();
        throttler.push(MockTask { id: "B".into() }).await.unwrap();
        throttler.push(MockTask { id: "C".into() }).await.unwrap();

        // A should be picked up instantly or very soon.
        // B and C wait.

        // Spawn worker
        tokio::spawn(async move {
            worker.run_consumer(|_| async {}).await;
        });

        // Small wait to let A start processing and be removed from map
        sleep(Duration::from_millis(50)).await;

        // A is likely gone (processed). B is at 0 (next). C is at 1.
        // Or if rate limiter held A, A=0, B=1, C=2.
        // Let's rely on IDs list.
        let contains = throttler.contains("C").await;
        let len = throttler.len().await;
        assert!(contains);
        assert!(len >= 2);
    }

    // --- Test 4 - 2: Position Tracking (Deterministic) ---
    #[tokio::test]
    async fn test_position_tracking_deterministic() {
        init_metrics_once();
        // 1. Setup Throttler (Worker NOT spawned yet)
        // Capacity 10 allows us to push multiple items without blocking/failing
        let (throttler, worker, _control_tx) =
            TxThrottlingSender::<MockTask>::new(TxThrottlingType::InputProof, 10, 0, 50, false);

        // 2. Push items
        // Since the worker isn't running, these sit in the Channel and the Tracker.
        throttler.push(MockTask { id: "A".into() }).await.unwrap();
        throttler.push(MockTask { id: "B".into() }).await.unwrap();
        throttler.push(MockTask { id: "C".into() }).await.unwrap();

        // 3. Verify Positions (Queue state: [A, B, C])
        assert_eq!(throttler.len().await, 3);

        // IndexMap preserves insertion order:
        assert_eq!(throttler.get_position("A").await, Some(0)); // Head of queue
        assert_eq!(throttler.get_position("B").await, Some(1));
        assert_eq!(throttler.get_position("C").await, Some(2)); // Tail of queue

        // Verify random access failure
        assert_eq!(throttler.get_position("Z").await, None);

        // 4. Start Worker to drain
        // Now we verify that they are processed and removed correctly
        tokio::spawn(async move {
            worker.run_consumer(|_| async {}).await;
        });

        // Wait for drain (50 TPS is fast, 100ms is plenty)
        sleep(Duration::from_millis(100)).await;

        // 5. Verify Empty
        assert_eq!(throttler.len().await, 0);
        assert_eq!(throttler.get_position("A").await, None);
    }

    // --- Test 5: Rate Limit (Burst + Tail) ---
    #[tokio::test]
    async fn test_rate_limit_simple() {
        init_metrics_once();
        // Setup: 10 TPS.
        // Governor behavior with allow_burst(10):
        // - Items 1-10: Processed Instantly (Burst).
        // - Items 11-20: Processed 1 every 100ms.
        // Total expected time: ~1.0 second.
        let tps = 10;
        let (throttler, worker, _control_tx) =
            TxThrottlingSender::<MockTask>::new(TxThrottlingType::InputProof, 100, 0, tps, false);
        let processed_count = Arc::new(AtomicUsize::new(0));
        let counter = processed_count.clone();

        tokio::spawn(async move {
            worker
                .run_consumer(move |_| {
                    let c = counter.clone();
                    async move {
                        c.fetch_add(1, Ordering::Relaxed);
                    }
                })
                .await;
        });

        let total_items = 20;
        let start = std::time::Instant::now();

        // Push 20 items
        for i in 0..total_items {
            throttler
                .push(MockTask {
                    id: format!("{}", i),
                })
                .await
                .unwrap();
        }

        // Wait for drain
        while processed_count.load(Ordering::Relaxed) < total_items {
            sleep(Duration::from_millis(10)).await;
        }

        let elapsed = start.elapsed();
        println!(
            "Simple Rate Test: Processed {} items in {:?}",
            total_items, elapsed
        );

        // Assertions:
        // Must take at least ~900ms (10 * 100ms delays)
        assert!(
            elapsed.as_millis() >= 900,
            "Too fast! Rate limit not enforced."
        );
        // Shouldn't take forever
        assert!(elapsed.as_millis() < 2000, "Too slow! Performance issue.");
    }

    // --- Test 6: Sustained Rate (Second-by-Second) ---
    #[tokio::test]
    async fn test_rate_limit_sustained() {
        init_metrics_once();
        let tps = 20;
        // Capacity large enough to hold the blast so we don't get "Queue Full" errors during setup
        let (throttler, worker, _control_tx) =
            TxThrottlingSender::<MockTask>::new(TxThrottlingType::InputProof, 200, 0, tps, false);
        let processed_count = Arc::new(AtomicUsize::new(0));
        let counter = processed_count.clone();

        tokio::spawn(async move {
            worker
                .run_consumer(move |_| {
                    let c = counter.clone();
                    async move {
                        c.fetch_add(1, Ordering::Relaxed);
                    }
                })
                .await;
        });

        // 1. Blast queue with 100 items (Instant because queue capacity is 200)
        for i in 0..100 {
            throttler
                .push(MockTask {
                    id: format!("sus_{}", i),
                })
                .await
                .unwrap();
        }

        let mut prev_count = 0;
        println!("Starting Sustained Monitor (Target: 20/sec)...");

        // Monitor for 3 seconds
        for s in 1..=3 {
            sleep(Duration::from_secs(1)).await;

            let curr = processed_count.load(Ordering::Relaxed);
            let delta = curr - prev_count;
            println!("T={}s | Total: {} | Delta: {}", s, curr, delta);

            if s == 1 {
                // First Second: Burst (20) + Flow (20) = ~40
                assert!(
                    (35..=45).contains(&delta),
                    "First second should handle Burst + Flow"
                );
            } else {
                // Subsequent Seconds: Strict Flow (20)
                assert!((18..=22).contains(&delta), "Steady state should match TPS");
            }
            prev_count = curr;
        }
    }

    // --- Test: Cloning Safety ---
    #[tokio::test]
    async fn test_cloning_shares_state() {
        init_metrics_once();
        let (throttler_1, worker, _control_tx) =
            TxThrottlingSender::<MockTask>::new(TxThrottlingType::InputProof, 10, 0, 100, false);

        // Clone it (Creating a second producer handle)
        let throttler_2 = throttler_1.clone();

        // Spawn worker to drain queue
        tokio::spawn(async move {
            worker.run_consumer(|_| async {}).await;
        });

        // 1. Push via Clone 1
        throttler_1.push(MockTask { id: "A".into() }).await.unwrap();

        // 2. Check via Clone 2 (Should see "A")
        assert!(throttler_2.contains("A").await);
        assert_eq!(throttler_2.len().await, 1);

        // 3. Push via Clone 2
        throttler_2.push(MockTask { id: "B".into() }).await.unwrap();

        // 4. Check via Clone 1 (Should see "B")
        assert!(throttler_1.contains("B").await);
    }

    // --- Test: Buffer / Headroom Logic ---
    #[tokio::test]
    async fn test_throttler_buffer_logic() {
        init_metrics_once();
        // Capacity = 10
        // Buffer = 2
        // Soft Limit = 8
        // TPS = 100 (irrelevant as we don't start worker immediately)
        let (throttler, worker, _control_tx) =
            TxThrottlingSender::<MockTask>::new(TxThrottlingType::InputProof, 10, 2, 100, false);

        // 1. Fill up to Soft Limit (8 items)
        for i in 0..8 {
            throttler
                .push(MockTask {
                    id: format!("{}", i),
                })
                .await
                .unwrap();
        }

        assert_eq!(throttler.len().await, 8);

        // 2. CHECK: Queue should report FULL (Soft Limit reached)
        // Even though physical capacity is 10, we stop accepting API calls here.
        assert!(throttler.is_queue_full().await);

        // 3. PROOF OF SAFETY (The "Race Condition" Simulation)
        // Imagine 2 requests passed the check concurrently right before step 2.
        // They try to push NOW.

        // Pushing 9th item -> Should SUCCEED (consumed 1 buffer slot)
        assert!(throttler.push(MockTask { id: "9".into() }).await.is_ok());

        // Pushing 10th item -> Should SUCCEED (consumed 2nd buffer slot)
        assert!(throttler.push(MockTask { id: "10".into() }).await.is_ok());

        assert_eq!(throttler.len().await, 10);

        // 4. HARD LIMIT
        // Now the buffer is truly gone. Next push fails hard.
        let err = throttler.push(MockTask { id: "11".into() }).await;
        assert!(err.is_err());

        // Drain to clean up
        tokio::spawn(async move {
            worker.run_consumer(|_| async {}).await;
        });
        sleep(Duration::from_millis(100)).await;
    }
}
