use crate::scheduler::{
    computation_scheduler::ComputationScheduler,
    traits::{Commands, Events},
};
use fhevm_engine_common::protocol::messages as msg;
use message_broker::Sender;
use std::{collections::HashMap, sync::Arc};
use tracing::{debug, error, info};

type Payload = Vec<u8>;

pub struct Dispatcher<S: Sender<Payload>> {
    scheduler: ComputationScheduler,

    /// Set of partitions that are currently being executed by workers
    in_progress: HashMap<msg::PartitionHash, msg::ExecutablePartition>,

    /// Default sender to use
    sender: Arc<S>,
}

impl<S: Sender<Payload>> Dispatcher<S> {
    pub fn new(sender: S) -> Self {
        Self {
            scheduler: ComputationScheduler::new(1),
            in_progress: HashMap::new(),
            sender: Arc::new(sender),
        }
    }

    /// Main entry point for processing incoming FHE log batches.
    /// This will update the scheduler's DFG and determine which partitions are now executable.
    /// It will then dispatch those partitions to workers via Message Broker.
    pub(crate) fn dispatch(&mut self, batch: &[msg::FheLog]) -> usize {
        if !batch.is_empty() {
            let _ = self.scheduler.on_fhe_log_batch(batch);

            // For debugging purposes
            #[cfg(feature = "export-graphs")]
            self.scheduler.export_graphs("./viz");
        }

        let exec_partitions = self
            .scheduler
            .retrieve_executable_partitions(self.in_progress.keys().cloned().collect());

        debug!(partitions =  ?exec_partitions, "New executable partitions");

        for (index, partition) in exec_partitions.iter().enumerate() {
            info!(
                index = index,
                pid = %partition.id(),
                "Dispatch exec partition"
            );

            // Mark these partitions as running
            self.in_progress.insert(partition.hash, partition.clone());
            self.publish(partition);
        }
        exec_partitions.len()
    }

    /// This should be called when a worker reports that it has completed executing a partition.
    pub fn on_partition_execution_complete(&mut self, partition: &msg::ExecutablePartition) {
        // Inform the scheduler about the completed partition so it can update the DFG
        // and potentially unlock dependent partitions
        self.scheduler.on_partition_completed(partition);

        let hash = partition.hash;
        let known = self.in_progress.contains_key(&hash);
        self.in_progress.remove(&hash);

        // Prune unneeded nodes from the scheduler to prevent unbounded memory growth
        self.scheduler.prune();

        info!(
            pid = %partition.id(),
            known,
            "Partition completed"
        );
    }

    /// Publishes the given partition to the global default sender.
    /// This dispatch does not enforce worker affinity,
    /// so any worker that is available may pick up the partition for execution.
    pub fn publish(&self, partition: &msg::ExecutablePartition) {
        info!(
            partition_id = %partition.id(),
            "Sending partition to channel"
        );

        let partition = partition.clone();
        let sender = self.sender.clone();
        tokio::spawn(async move {
            let payload = postcard::to_allocvec(&partition).expect("Failed to serialize partition");
            let res = sender.send(payload).await;

            match res {
                Ok(_) => {
                    info!(partition_id = %partition.id(), "Partition sent to channel successfully")
                }
                Err(err) => {
                    error!(error = ?err, partition_id = %partition.id(), "Failed to send partition to channel")
                }
            }
        });
    }

    pub fn report(&self) {
        if !self.in_progress.is_empty() {
            info!(
                in_progress = self.in_progress.len(),
                "Current in-progress partitions"
            );
        }

        self.scheduler.stats();
    }
}
