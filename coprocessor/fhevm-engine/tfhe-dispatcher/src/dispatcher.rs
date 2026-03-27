use crate::scheduler::{
    computation_scheduler::ComputationScheduler,
    traits::{Commands, Events},
};
use fhevm_engine_common::protocol::messages::{self as msg};
use message_broker::Sender;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

type Payload = Vec<u8>;
const KEY_ID: u64 = 1u64;

pub struct Dispatcher<S: Sender<Payload>> {
    scheduler: ComputationScheduler,

    /// Set of partitions that are currently being executed by workers
    in_progress: Arc<RwLock<HashMap<msg::PartitionHash, msg::ExecutablePartition>>>,

    /// Default sender to use
    sender: Arc<S>,
}

impl<S: Sender<Payload>> Dispatcher<S> {
    pub fn new(sender: S) -> Self {
        Self {
            scheduler: ComputationScheduler::new(KEY_ID),
            in_progress: Arc::new(RwLock::new(HashMap::new())),
            sender: Arc::new(sender),
        }
    }

    /// Main entry point for processing incoming FHE log batches.
    /// This will update the scheduler's DFG and determine which partitions are now executable.
    /// It will then dispatch those partitions to workers via Message Broker.
    pub(crate) async fn dispatch(&mut self, batch: &[msg::FheLog]) -> usize {
        if !batch.is_empty() {
            let _ = self.scheduler.on_fhe_log_batch(batch);

            // For debugging purposes
            #[cfg(feature = "export-graphs")]
            self.scheduler.export_graphs("./viz");
        }

        let in_progress = self.in_progress.read().await.keys().cloned().collect();

        // Retrieve executable partitions from the scheduler, excluding those that are already in progress.
        let exec_partitions = self.scheduler.retrieve_executable_partitions(in_progress);

        debug!(partitions =  ?exec_partitions, "New executable partitions");
        for (index, partition) in exec_partitions.iter().enumerate() {
            info!(
                index = index,
                pid = %partition.id(),
                "Dispatch exec partition"
            );

            self.spawn_publish(partition);
        }
        exec_partitions.len()
    }

    /// This should be called when a worker reports that it has completed executing a partition.
    pub async fn on_partition_execution_complete(&mut self, partition: &msg::ExecutablePartition) {
        // Inform the scheduler about the completed partition so it can update the DFG
        // and potentially unlock dependent partitions
        self.scheduler.on_partition_completed(partition);

        let hash = partition.hash;

        if !self.in_progress.read().await.contains_key(&hash) {
            // This can happen if the same partition completion message is received multiple times due to retries,
            // or the dispatcher was restarted and lost its in-memory state of in-progress tasks.
            warn!(
                pid = %partition.id(),
                "Received completion for unknown partition"
            );
        }

        self.in_progress.write().await.remove(&hash);

        // Prune unneeded nodes from the scheduler to prevent unbounded memory growth
        self.scheduler.prune();

        info!(
            pid = %partition.id(),
            "Partition completed"
        );
    }

    /// Publishes the given partition to the global default sender.
    /// This dispatch does not enforce worker affinity,
    /// so any worker that is available may pick up the partition for execution.
    pub fn spawn_publish(&self, partition: &msg::ExecutablePartition) {
        let partition = partition.clone();
        let sender = self.sender.clone();
        let in_progress = self.in_progress.clone();

        // In case of rabbitmq, we should have an fire-and-forget publish to avoid blocking
        // the dispatcher loop while waiting for a Confirm from the broker.
        tokio::spawn(async move {
            let pid = partition.id();
            let payload = match postcard::to_allocvec(&partition) {
                Ok(payload) => payload,
                Err(err) => {
                    error!(error = ?err, pid = %pid, "Failed to serialize partition");
                    return;
                }
            };

            if let Err(err) = sender.send(payload).await {
                error!(error = ?err, pid = %pid, "Failed to send partition to channel");
                return;
            }

            in_progress.write().await.insert(partition.hash, partition);
            info!(pid = %pid, "Partition published successfully");
        });
    }

    pub async fn report(&self) {
        let in_progress = self.in_progress.read().await;
        if !in_progress.is_empty() {
            info!(
                tag = "stats",
                in_progress = in_progress.len(),
                "In-progress tasks"
            );
        }
        drop(in_progress);

        self.scheduler.stats();
    }
}
