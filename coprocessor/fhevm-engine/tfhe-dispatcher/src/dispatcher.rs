use crate::scheduler::{
    computation_scheduler::ComputationScheduler,
    traits::{Commands, Events},
};
use fhevm_engine_common::protocol::messages as msg;
use lapin::options::BasicPublishOptions;
use lapin::BasicProperties;
use std::collections::HashMap;
use std::future::Future;
use tracing::{debug, info};

type PartitionId = [u8; 32];

#[derive(Clone)]
pub struct LapinChannel {
    pub exchange_name: String,
    pub routing_key: String,
    sender_channel: lapin::Channel,
}

impl LapinChannel {
    pub fn new(sender_channel: lapin::Channel, exchange_name: String, routing_key: String) -> Self {
        Self {
            sender_channel,
            exchange_name,
            routing_key,
        }
    }
}

impl Channel for LapinChannel {
    fn exchange_name(&self) -> &str {
        &self.exchange_name
    }

    fn routing_key(&self) -> &str {
        &self.routing_key
    }

    async fn send_partition(&self, partition: &msg::ExecutablePartition) {
        let payload = &postcard::to_allocvec(partition).expect("Failed to serialize partition");

        debug!(
            partition_id = %partition.id(),
            routing_key = %self.routing_key,
            "Sending partition to channel"
        );

        let confirm = self
            .sender_channel
            .basic_publish(
                &self.exchange_name,
                &self.routing_key,
                BasicPublishOptions::default(),
                payload,
                BasicProperties::default(),
            )
            .await
            .expect("Failed to publish partition");

        let confirm = confirm
            .await
            .expect("Could not receive ack for published partition");

        debug!(
            partition_id = %partition.id(),
            routing_key = %self.routing_key,
            confirm = ?confirm,
            "Published partition to channel"
        );
    }
}

pub trait Channel: Clone + Send + Sync + 'static {
    fn exchange_name(&self) -> &str;
    fn routing_key(&self) -> &str;
    fn send_partition(
        &self,
        partition: &msg::ExecutablePartition,
    ) -> impl Future<Output = ()> + Send;
}

pub struct Dispatcher<C: Channel> {
    computation_scheduler: ComputationScheduler,

    /// Set of partitions that are currently being executed by workers
    running_partitions: HashMap<PartitionId, msg::ExecutablePartition>,

    /// Default channel to use
    default_channel: C,

    /// Configuration for the publisher
    channel_pool: HashMap<String, C>,
}

impl<C: Channel> Dispatcher<C> {
    pub fn new(default_channel: C) -> Self {
        Self {
            computation_scheduler: ComputationScheduler::new(1),
            running_partitions: HashMap::new(),
            default_channel,
            channel_pool: HashMap::new(),
        }
    }

    pub fn add_channel(&mut self, id: String, channel: C) {
        self.channel_pool.insert(id, channel);
    }

    /// Main entry point for processing incoming FHE log batches.
    /// This will update the scheduler's DFG and determine which partitions are now executable.
    /// It will then dispatch those partitions to workers via Message Broker.
    pub fn dispatch(&mut self, batch: &[msg::FheLog]) {
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
            self.publish_to_default_channel(p);
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

    /// Publishes the given partition to the global default channel.
    /// This dispatch does not enforce worker affinity or locality
    /// the partition may be processed by any available worker.
    pub fn publish_to_default_channel(&self, partition: &msg::ExecutablePartition) {
        let sender_channel = self.default_channel.clone();
        let p: msg::ExecutablePartition = partition.clone();

        tokio::spawn(async move {
            sender_channel.send_partition(&p).await;
        });
    }
}
