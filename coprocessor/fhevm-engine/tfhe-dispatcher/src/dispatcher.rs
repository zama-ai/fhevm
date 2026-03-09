use crate::scheduler::{
    computation_scheduler::ComputationScheduler,
    traits::{Commands, Events},
};
use fhevm_engine_common::protocol::messages as msg;
use message_broker::Sender;
use std::{collections::HashMap, sync::Arc};
use tracing::{debug, error, info};

type PartitionId = [u8; 32];
type Payload = Vec<u8>;

pub struct Dispatcher<S: Sender<Payload>> {
    computation_scheduler: ComputationScheduler,

    /// Set of partitions that are currently being executed by workers
    running_partitions: HashMap<PartitionId, msg::ExecutablePartition>,

    /// Default sender to use
    sender: Arc<S>,
}

impl<S: Sender<Payload>> Dispatcher<S> {
    pub fn new(sender: S) -> Self {
        Self {
            computation_scheduler: ComputationScheduler::new(1),
            running_partitions: HashMap::new(),
            sender: Arc::new(sender),
        }
    }

    /// Main entry point for processing incoming FHE log batches.
    /// This will update the scheduler's DFG and determine which partitions are now executable.
    /// It will then dispatch those partitions to workers via Message Broker.
    pub(crate) fn dispatch(&mut self, batch: &[msg::FheLog]) {
        if !batch.is_empty() {
            self.computation_scheduler.on_fhe_log_batch(batch);
        }

        let exec_partitions = self.computation_scheduler.retrieve_executable_partitions();

        let new_exec_partitions = &exec_partitions
            .into_iter()
            .filter(|p| !self.running_partitions.contains_key(&p.hash))
            .collect::<Vec<_>>();

        // For debugging and visualization purposes
        #[cfg(feature = "export-graphs")]
        self.computation_scheduler.export_graphs("./viz");

        let running = self.running_partitions.len();
        info!(
            count = new_exec_partitions.len(),
            running, "Dispatching executable partitions"
        );

        debug!(partitions =  ?new_exec_partitions, "New executable partitions");

        for p in new_exec_partitions {
            // Mark these partitions as running
            self.running_partitions.insert(p.hash, p.clone());
            self.publish(p);
        }
    }

    /// This should be called when a worker reports that it has completed executing a partition.
    pub fn on_partition_execution_complete(&mut self, partition: &msg::ExecutablePartition) {
        // Inform the scheduler about the completed partition so it can update the DFG
        // and potentially unlock dependent partitions
        self.computation_scheduler.on_partition_completed(partition);

        let hash = partition.hash;
        let known = self.running_partitions.contains_key(&hash);
        self.running_partitions.remove(&hash);

        // Prune unneeded nodes from the scheduler to prevent unbounded memory growth
        self.computation_scheduler.prune();

        info!(
            pid = %partition.id(),
            known,
            "Partition execution complete"
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
}
