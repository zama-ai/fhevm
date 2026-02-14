use crate::scheduler::messages as msg;
use daggy::NodeIndex;

pub trait Commands {
    fn retrieve_executable_partitions(&self) -> Vec<msg::ExecutablePartition>;
}

pub trait Events {
    /// Process a single FHE log message, update the DFG, and return the corresponding node index.
    fn on_fhe_log_msg(&mut self, log: &msg::FheLog, update_exec_graph: bool) -> NodeIndex;
    /// Process a batch of FHE log messages
    fn on_fhe_log_batch(&mut self, logs: &[msg::FheLog]) -> Vec<NodeIndex>;

    /// Trigger when a partition is completed
    fn on_partition_completed(&mut self, partition: &msg::ExecutablePartition);
}
