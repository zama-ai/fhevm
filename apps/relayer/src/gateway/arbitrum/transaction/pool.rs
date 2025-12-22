use governor::{Quota, RateLimiter};
use std::future::Future;
use std::num::NonZeroU32;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::time::Duration;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::sync::{Mutex, RwLock};
use tracing::{error, info};

// =============================================================================
// 1. THE MEMPOOL STRUCT
// =============================================================================

// TODO: first and last entry getters helpers.
#[derive(Clone)]
pub struct Mempool<T> {
    // RwLock allows us to "Hot Swap" the sender if the worker restarts.
    // Read access (for pushing) is fast and concurrent.
    sender: Arc<RwLock<UnboundedSender<T>>>,
    // Mutex<Option<...>> allows us to "Claim" the receiver exactly once per run.
    receiver: Arc<Mutex<Option<UnboundedReceiver<T>>>>,
    // Shared metric
    queue_len: Arc<AtomicUsize>,
    // Configuration
    tps: u32,
}

impl<T> Mempool<T>
where
    T: Send + Sync + 'static + std::fmt::Debug,
{
    /// Initialize the Mempool. Does NOT start the worker.
    pub fn new(tps: u32) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();

        Self {
            sender: Arc::new(RwLock::new(sender)),
            receiver: Arc::new(Mutex::new(Some(receiver))),
            queue_len: Arc::new(AtomicUsize::new(0)),
            tps,
        }
    }

    /// Robust Push:
    /// - If the system is healthy, sends immediately.
    /// - If the worker/channel is dead, it WAITS and RETRIES until fixed.
    /// - This ensures NO data loss during restarts of a failed worker.
    pub async fn push(&self, mut item: T) {
        // Increment metric immediately (it is effectively queued, just waiting for the lock)
        self.queue_len.fetch_add(1, Ordering::Relaxed);
        loop {
            // Acquire current sender
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

                    error!("Could not send the transaction to the worker, retrying. Transaction item: {:?}", item);

                    // Wait for Orchestrator to call refresh_channel()
                    tokio::time::sleep(Duration::from_millis(1000)).await;
                }
            }
        }
    }

    /// Call this if the worker crashed and you need to restart it.
    /// It creates a new channel and updates all 'push' loops automatically.
    pub async fn refresh_channel(&self) {
        let (new_tx, new_rx) = mpsc::unbounded_channel();

        // 1. Update the Sender (Wakes up all stuck 'push' loops)
        let mut tx_guard = self.sender.write().await;
        *tx_guard = new_tx;
        drop(tx_guard);

        // 2. Reset the Receiver (So .run() can claim it)
        let mut rx_guard = self.receiver.lock().await;
        *rx_guard = Some(new_rx);
        drop(rx_guard);

        println!("♻️  Mempool Channel Refreshed.");
    }

    /// Current queue depth
    pub fn len(&self) -> usize {
        self.queue_len.load(Ordering::Relaxed)
    }

    // =========================================================================
    // 2. THE WORKER LOGIC
    // =========================================================================

    /// Runs the processing loop.
    /// - `processor`: The async function to execute for each item.
    /// - Returns an error if called twice simultaneously.
    pub async fn run<F, Fut>(&self, processor: F) -> Result<(), String>
    where
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        // Claim the Receiver
        let mut rx_guard = self.receiver.lock().await;
        let mut receiver = rx_guard
            .take()
            .ok_or("Worker already running or not refreshed!")?;
        drop(rx_guard);

        // Setup Rate Limiter
        let burst = NonZeroU32::new(self.tps).expect("TPS cannot be zero");
        let quota = Quota::per_second(burst).allow_burst(burst);
        let limiter = Arc::new(RateLimiter::direct(quota));

        // Prepare Processor
        let processor = Arc::new(processor);

        info!("Transaction Throttling queue as started: {} TPS", self.tps);

        // Supervisor Loop
        while let Some(item) = receiver.recv().await {
            // Rate Limit
            limiter.until_ready().await;

            // Metric
            self.queue_len.fetch_sub(1, Ordering::Relaxed);

            // Isolation Spawn
            let proc_clone = processor.clone();
            let item_debug = format!("{:?}", item);

            let handle = tokio::spawn(async move {
                proc_clone(item).await;
            });

            // Error Monitoring
            if let Err(e) = handle.await {
                if e.is_panic() {
                    eprintln!(
                        "🔥 WORKER PANIC on item: {} | Queue safe. Continuing...",
                        item_debug
                    );
                } else {
                    eprintln!("⚠️ Task cancelled: {}", e);
                }
            }
        }

        println!("🛑 Worker stopped (Channel closed internally).");
        Ok(())
    }
}

// =============================================================================
// 3. USAGE DEMONSTRATION
// =============================================================================

/*
#[derive(Debug, Clone)]
struct Transaction {
    id: String,
}

#[tokio::main]
async fn main() {
    // 1. Create Mempool
    let mempool = Mempool::<Transaction>::new(10); // 10 TPS

    // 2. Define our Processor Logic
    let processor_logic = |tx: Transaction| async move {
        if tx.id == "POISON" {
            panic!("Boom!");
        }
        println!("✅ Broadcast: {}", tx.id);
    };

    // 3. Start Initial Worker (In background)
    let pool_handle = mempool.clone();
    let mut worker_task = tokio::spawn(async move {
        let _ = pool_handle.run(processor_logic).await;
    });

    // 4. Push Normal Transactions
    println!("--- Phase 1: Normal Operation ---");
    mempool.push(Transaction { id: "tx_1".into() }).await;
    mempool.push(Transaction { id: "tx_2".into() }).await;
    mempool
        .push(Transaction {
            id: "POISON".into(),
        })
        .await; // This will crash SUB-TASK, not worker
    mempool.push(Transaction { id: "tx_3".into() }).await;

    tokio::time::sleep(Duration::from_secs(1)).await;

    // 5. SIMULATE CATASTROPHIC WORKER FAILURE
    // We manually kill the worker task to simulate a crash/restart scenario.
    println!("\n--- Phase 2: Simulating Worker Death ---");
    worker_task.abort();
    tokio::time::sleep(Duration::from_millis(500)).await;
    println!("💀 Worker is DEAD.");

    // 6. PUSH WHILE DEAD (The Robust Push Test)
    // This call will BLOCK and HANG until we fix the system.
    println!("✋ Pushing 'tx_waiting' (This should hang momentarily)...");

    let pool_clone_for_push = mempool.clone();
    let push_task = tokio::spawn(async move {
        // This will enter the retry loop
        pool_clone_for_push
            .push(Transaction {
                id: "tx_waiting".into(),
            })
            .await;
        println!("🎉 'tx_waiting' successfully pushed!");
    });

    tokio::time::sleep(Duration::from_secs(2)).await;

    // 7. REPAIR SYSTEM
    println!("\n--- Phase 3: System Repair ---");

    // A. Refresh Channel (This will unblock the push_task shortly after)
    mempool.refresh_channel().await;

    // B. Restart Worker
    let pool_handle_2 = mempool.clone();
    tokio::spawn(async move {
        // We redefine logic or use the same one
        let _ = pool_handle_2
            .run(|tx| async move {
                println!("✅ Broadcast (New Worker): {}", tx.id);
            })
            .await;
    });

    // 8. VERIFY RECOVERY
    // Wait for the hanging push to succeed
    let _ = push_task.await;

    // Push one more to be sure
    mempool
        .push(Transaction {
            id: "tx_after_restart".into(),
        })
        .await;

    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("\nTest Complete. Queue len: {}", mempool.len());
}
*/
