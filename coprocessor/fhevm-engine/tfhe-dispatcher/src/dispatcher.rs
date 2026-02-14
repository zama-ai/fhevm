use crate::scheduler::messages as msg;
use crate::scheduler::{
    computation_scheduler::ComputationScheduler,
    traits::{Commands, Events},
};
use std::collections::HashMap;
use tracing::warn;

type PartitionId = [u8; 32];

#[derive(Default)]
pub struct Dispatcher {
    computation_scheduler: ComputationScheduler,

    /// Set of partitions that are currently being executed by workers
    running_partitions: HashMap<PartitionId, msg::ExecutablePartition>,
}

impl Dispatcher {
    pub fn dispatch_fhe_partitions(
        &mut self,
        batch: &[msg::FheLog],
        sender_channel: lapin::Channel,
    ) {
        if !batch.is_empty() {
            self.computation_scheduler.on_fhe_log_batch(batch);
        }

        let exec_partitions = self.computation_scheduler.retrieve_executable_partitions();

        let new_exec_partitions = &exec_partitions
            .into_iter()
            .filter(|p| !self.running_partitions.contains_key(&p.hash))
            .collect::<Vec<_>>();

        for p in new_exec_partitions {
            // Mark these partitions as running
            self.running_partitions.insert(p.hash, p.clone());

            tokio::spawn(Self::send_partition_to_workers(
                p.clone(),
                sender_channel.clone(),
            ));
        }
    }

    async fn send_partition_to_workers(
        partition: msg::ExecutablePartition,
        sender_channel: lapin::Channel,
    ) {
        let payload = &serde_json::to_vec(&partition).expect("Failed to serialize partition");

        let confirm = sender_channel
            .basic_publish(
                "", // TODO:
                "shared_tfhe_queue",
                lapin::options::BasicPublishOptions::default(),
                payload,
                lapin::BasicProperties::default(),
            )
            .await
            .expect("Failed to publish partition to RabbitMQ");

        confirm
            .await
            .expect("Could not receive ack for published partition");
    }

    pub fn on_partition_execution_complete(&mut self, partition_id: &PartitionId) {
        // TODO: Pass partition here instead of just the id
        let executed_partition = self.running_partitions.get(partition_id);
        match executed_partition {
            Some(p) => {
                // Inform the scheduler about the completed partition so it can update the DFG and potentially unlock dependent partitions
                self.computation_scheduler.on_partition_completed(p);
                self.running_partitions.remove(partition_id);
            }
            None => {
                // This should not happen - log a warning
                warn!(partition_id = %hex::encode(partition_id), "Received execution complete for unknown partition");
            }
        }
    }
}
