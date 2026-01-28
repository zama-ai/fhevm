use crate::{
    core::{
        errors::EventProcessingError,
        event::{DelegatedUserDecryptRequest, PublicDecryptRequest, UserDecryptRequest},
        job_id::JobId,
    },
    metrics,
};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{fmt, future::Future};
use tokio::sync::{mpsc, RwLock, Semaphore};
use tracing::{error, info, instrument, warn};

/// Queue information for ETA computation (concurrency-based throttler)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ReadinessQueueInfo {
    /// Current queue size (number of items waiting)
    pub size: usize,
    /// Maximum concurrency (semaphore permits)
    pub max_concurrency: usize,
    /// Position in queue (0-indexed, None if not in queue or request will join at end)
    pub position: Option<usize>,
}

pub struct ReadinessThrottlers {
    pub user_decrypt_readiness_throttler: ReadinessSender<UserDecryptReadinessTask>,
    pub user_decrypt_readiness_worker: ReadinessWorker<UserDecryptReadinessTask>,
    pub delegated_user_decrypt_readiness_throttler:
        ReadinessSender<DelegatedUserDecryptReadinessTask>,
    pub delegated_user_decrypt_readiness_worker: ReadinessWorker<DelegatedUserDecryptReadinessTask>,
    pub public_decrypt_readiness_throttler: ReadinessSender<PublicDecryptReadinessTask>,
    pub public_decrypt_readiness_worker: ReadinessWorker<PublicDecryptReadinessTask>,
}

impl ReadinessThrottlers {
    pub fn new(
        user_decrypt_readiness_throttler: ReadinessSender<UserDecryptReadinessTask>,
        user_decrypt_readiness_worker: ReadinessWorker<UserDecryptReadinessTask>,
        delegated_user_decrypt_readiness_throttler: ReadinessSender<
            DelegatedUserDecryptReadinessTask,
        >,
        delegated_user_decrypt_readiness_worker: ReadinessWorker<DelegatedUserDecryptReadinessTask>,
        public_decrypt_readiness_throttler: ReadinessSender<PublicDecryptReadinessTask>,
        public_decrypt_readiness_worker: ReadinessWorker<PublicDecryptReadinessTask>,
    ) -> Self {
        Self {
            user_decrypt_readiness_throttler,
            user_decrypt_readiness_worker,
            delegated_user_decrypt_readiness_throttler,
            delegated_user_decrypt_readiness_worker,
            public_decrypt_readiness_throttler,
            public_decrypt_readiness_worker,
        }
    }
}

pub struct ReadinessSenders {
    pub user_decrypt_readiness_throttler: ReadinessSender<UserDecryptReadinessTask>,
    pub delegated_user_decrypt_readiness_throttler:
        ReadinessSender<DelegatedUserDecryptReadinessTask>,
    pub public_decrypt_readiness_throttler: ReadinessSender<PublicDecryptReadinessTask>,
}

impl ReadinessSenders {
    pub fn new(
        user_decrypt_readiness_throttler: ReadinessSender<UserDecryptReadinessTask>,
        delegated_user_decrypt_readiness_throttler: ReadinessSender<
            DelegatedUserDecryptReadinessTask,
        >,
        public_decrypt_readiness_throttler: ReadinessSender<PublicDecryptReadinessTask>,
    ) -> Self {
        Self {
            user_decrypt_readiness_throttler,
            delegated_user_decrypt_readiness_throttler,
            public_decrypt_readiness_throttler,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ReadinessThrottlingType {
    UserDecrypt,
    PublicDecrypt,
}

impl ReadinessThrottlingType {
    fn as_metrics_type(&self) -> metrics::QueueType {
        match self {
            ReadinessThrottlingType::UserDecrypt => {
                metrics::QueueType::UserDecryptReadinessThrottler
            }
            ReadinessThrottlingType::PublicDecrypt => {
                metrics::QueueType::PublicDecryptReadinessThrottler
            }
        }
    }
}

impl fmt::Display for ReadinessThrottlingType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReadinessThrottlingType::UserDecrypt => write!(f, "user_decrypt_readiness_throttler"),
            ReadinessThrottlingType::PublicDecrypt => {
                write!(f, "public_decrypt_readiness_throttler")
            }
        }
    }
}

pub trait ReadinessItem: Send + Sync + 'static + std::fmt::Debug {
    fn get_id(&self) -> String;
}

/// The unit of work for the Readiness Throttler.
#[derive(Debug, Clone)]
pub struct PublicDecryptReadinessTask {
    pub id: String,
    pub job_id: JobId,
    pub request: PublicDecryptRequest,
}

impl ReadinessItem for PublicDecryptReadinessTask {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// The unit of work for the Readiness Throttler.
#[derive(Debug, Clone)]
pub struct UserDecryptReadinessTask {
    pub id: String,
    pub job_id: JobId,
    pub request: UserDecryptRequest,
}

impl ReadinessItem for UserDecryptReadinessTask {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// The unit of work for the Readiness Throttler.
#[derive(Debug, Clone)]
pub struct DelegatedUserDecryptReadinessTask {
    pub id: String,
    pub job_id: JobId,
    pub request: DelegatedUserDecryptRequest,
}

impl ReadinessItem for DelegatedUserDecryptReadinessTask {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

// READINESS SENDER (Producer)

/// The Producer Handle.
#[derive(Clone)]
pub struct ReadinessSender<T> {
    readiness_type: ReadinessThrottlingType,
    // Bounded Channel carrying the Payload T
    sender: mpsc::Sender<T>,
    tracker: Arc<RwLock<IndexMap<String, ()>>>,
    soft_capacity: usize,
    // Maximum parallelism (stored for queue info)
    max_parallelism: usize,
}

// READINESS WORKER (Consumer)

/// The Background Worker.
pub struct ReadinessWorker<T> {
    readiness_type: ReadinessThrottlingType,
    receiver: mpsc::Receiver<T>,
    // Shared reference to the same tracker to remove items when processed
    tracker: Arc<RwLock<IndexMap<String, ()>>>,
    // Replaces TPS: Limits how many tasks can run concurrently
    max_parallelism: usize,
}

impl<T> ReadinessSender<T>
where
    T: ReadinessItem + Send + Sync + 'static,
{
    pub fn new(
        readiness_type: ReadinessThrottlingType,
        capacity: usize,
        capacity_safety_margin: usize,
        max_parallelism: usize,
    ) -> (Self, ReadinessWorker<T>) {
        let (sender, receiver) = mpsc::channel(capacity);
        let tracker = Arc::new(RwLock::new(IndexMap::new()));

        // The safety margin (Headroom).
        let soft_capacity = capacity.saturating_sub(capacity_safety_margin);

        let sender = Self {
            readiness_type,
            sender,
            tracker: tracker.clone(),
            soft_capacity,
            max_parallelism,
        };

        let worker = ReadinessWorker {
            readiness_type,
            receiver,
            tracker,
            max_parallelism,
        };

        (sender, worker)
    }

    /// Try to push an item.
    #[instrument(skip(self, item), fields(id = %item.get_id()))]
    pub async fn push(&self, item: T) -> anyhow::Result<(), EventProcessingError> {
        let id = item.get_id();

        // Acquire Write Lock
        let mut tracker = self.tracker.write().await;

        // Note: We can add at this step a deduplication step.

        // Try Send (Bounded check)
        match self.sender.try_send(item) {
            Ok(_) => {
                // Success: Add ID to tracker
                tracker.insert(id, ());
                metrics::queue::increment_queue_size(self.readiness_type.as_metrics_type());
                Ok(())
            }
            Err(mpsc::error::TrySendError::Full(_)) => {
                warn!("Readiness queue is full! Bouncing request.");
                Err(EventProcessingError::QueueFull)
            }
            Err(mpsc::error::TrySendError::Closed(_)) => {
                error!("CRITICAL: Readiness channel is closed! Worker is dead.");
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

    pub async fn get_queued_ids(&self) -> Vec<String> {
        self.tracker.read().await.keys().cloned().collect()
    }

    /// SOFT CHECK: Used to bounce requests early based on headroom configuration.
    pub async fn is_queue_full(&self) -> bool {
        let len = self.len().await;
        len >= self.soft_capacity
    }

    /// Get the maximum concurrency (semaphore permits).
    pub fn max_concurrency(&self) -> usize {
        self.max_parallelism
    }

    /// Get queue info for ETA computation.
    /// Returns current queue size and max concurrency.
    /// Position is None since we don't know which specific request this is for.
    pub async fn get_queue_info(&self) -> ReadinessQueueInfo {
        ReadinessQueueInfo {
            size: self.len().await,
            max_concurrency: self.max_parallelism,
            position: None,
        }
    }

    /// Get queue info for a specific request ID.
    /// Returns current queue size, max concurrency, and position if request is in queue.
    pub async fn get_queue_info_for_request(&self, request_id: &str) -> ReadinessQueueInfo {
        ReadinessQueueInfo {
            size: self.len().await,
            max_concurrency: self.max_parallelism,
            position: self.get_position(request_id).await,
        }
    }
}

impl<T> ReadinessWorker<T>
where
    T: ReadinessItem + Send + Sync + 'static,
{
    pub async fn run_consumer<F, Fut>(mut self, processor: F)
    where
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        info!(
            "Readiness Worker started. Max Parallelism: {}, Capacity: Bounded",
            self.max_parallelism
        );

        // Semaphore controls concurrency.
        // We use an Arc so we can clone it into the tasks.
        let semaphore = Arc::new(Semaphore::new(self.max_parallelism));
        let processor = Arc::new(processor);

        while let Some(item) = self.receiver.recv().await {
            let id = item.get_id();

            // Acquire Permit (Limits Concurrency)
            // This will Wait (Async) if we have reached max_parallelism active tasks.
            // We use acquire_owned to move the permit into the spawned task.
            let permit = match semaphore.clone().acquire_owned().await {
                Ok(p) => p,
                Err(_) => {
                    // Semaphore closed, shouldn't happen unless we shutdown explicitly.
                    error!("CRITICAL: Readiness Semaphore closed. Should never happens if not shutting down.");
                    break;
                }
            };

            // Remove from Tracker
            // The item leaves the "Waiting Queue" and enters "Processing"
            {
                let mut tracker = self.tracker.write().await;
                tracker.shift_remove(&id);
                metrics::queue::decrement_queue_size(self.readiness_type.as_metrics_type());
            }

            // Process (Isolated)
            let proc_clone = processor.clone();

            tokio::spawn(async move {
                // Do the work
                proc_clone(item).await;

                // CRITICAL: Permit is dropped here automatically.
                // This releases the slot for the next task in the queue.
                drop(permit);
            });
        }

        info!("Readiness Worker stopping (All producers dropped).");
    }
}

// =============================================================================
// UNIT TESTS
// =============================================================================

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

    impl ReadinessItem for MockTask {
        fn get_id(&self) -> String {
            self.id.clone()
        }
    }

    // --- Test 1: FIFO Ordering ---
    #[tokio::test]
    async fn test_fifo_ordering() {
        init_metrics_once();
        let (sender, worker) =
            ReadinessSender::<MockTask>::new(ReadinessThrottlingType::UserDecrypt, 10, 0, 100);
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

        sender.push(MockTask { id: "1".into() }).await.unwrap();
        sender.push(MockTask { id: "2".into() }).await.unwrap();

        sleep(Duration::from_millis(100)).await;

        assert_eq!(*processed.lock().unwrap(), vec!["1", "2"]);
        assert_eq!(sender.len().await, 0);
    }

    // --- Test 2: Deduplication ---
    #[tokio::test]
    async fn test_deduplication() {
        init_metrics_once();
        let (sender, _worker) =
            ReadinessSender::<MockTask>::new(ReadinessThrottlingType::UserDecrypt, 10, 0, 100);

        sender.push(MockTask { id: "A".into() }).await.unwrap();
        sender.push(MockTask { id: "A".into() }).await.unwrap();

        assert_eq!(sender.len().await, 1);
    }

    // --- Test 3: Queue Full ---
    #[tokio::test]
    async fn test_queue_full() {
        init_metrics_once();
        let (sender, _worker) =
            ReadinessSender::<MockTask>::new(ReadinessThrottlingType::UserDecrypt, 1, 0, 100);

        assert!(sender.push(MockTask { id: "1".into() }).await.is_ok());

        let err = sender.push(MockTask { id: "2".into() }).await;
        assert!(matches!(err, Err(EventProcessingError::QueueFull)));
    }

    // --- Test 4: Concurrency Limit (Semaphore Logic) ---
    #[tokio::test]
    async fn test_concurrency_limit() {
        init_metrics_once();
        // Max Parallelism = 3
        let max_parallelism = 3;
        let (sender, worker) = ReadinessSender::<MockTask>::new(
            ReadinessThrottlingType::UserDecrypt,
            100,
            0,
            max_parallelism,
        );

        // Counter tracks currently ACTIVE tasks
        let active_tasks = Arc::new(AtomicUsize::new(0));
        let max_seen = Arc::new(AtomicUsize::new(0));

        let ac = active_tasks.clone();
        let ms = max_seen.clone();

        tokio::spawn(async move {
            worker
                .run_consumer(move |_| {
                    let ac = ac.clone();
                    let ms = ms.clone();
                    async move {
                        // Increment active count
                        let current = ac.fetch_add(1, Ordering::Relaxed) + 1;

                        // Track peak parallelism
                        ms.fetch_max(current, Ordering::Relaxed);

                        // Simulate work (Hold the semaphore permit)
                        sleep(Duration::from_millis(100)).await;

                        // Decrement active count
                        ac.fetch_sub(1, Ordering::Relaxed);
                    }
                })
                .await;
        });

        // Push 10 items rapidly
        for i in 0..10 {
            sender
                .push(MockTask {
                    id: format!("{}", i),
                })
                .await
                .unwrap();
        }

        // Wait for all to finish
        sleep(Duration::from_millis(1200)).await;

        // Verify:
        // 1. The peak active tasks should never exceed max_parallelism (3)
        // 2. All tasks should have finished (active = 0)
        assert!(
            max_seen.load(Ordering::Relaxed) <= 3,
            "Parallelism exceeded limit!"
        );
        assert_eq!(
            active_tasks.load(Ordering::Relaxed),
            0,
            "Not all tasks finished"
        );

        // Sanity check: We should have seen at least 3 running to prove it wasn't just serial
        assert_eq!(
            max_seen.load(Ordering::Relaxed),
            3,
            "Did not reach max parallelism"
        );
    }

    // --- Test 5: Headroom / Soft Limit Logic ---
    #[tokio::test]
    async fn test_readiness_headroom() {
        init_metrics_once();
        // Capacity = 10, Safety = 2 -> Soft Limit = 8
        let (sender, worker) =
            ReadinessSender::<MockTask>::new(ReadinessThrottlingType::UserDecrypt, 10, 2, 100);

        for i in 0..8 {
            sender
                .push(MockTask {
                    id: format!("{}", i),
                })
                .await
                .unwrap();
        }

        assert_eq!(sender.len().await, 8);
        assert!(sender.is_queue_full().await); // Soft limit reached

        // Hard push should still work (Safety Zone)
        assert!(sender.push(MockTask { id: "9".into() }).await.is_ok());

        // Drain
        tokio::spawn(async move {
            worker.run_consumer(|_| async {}).await;
        });
        sleep(Duration::from_millis(100)).await;
    }
}
