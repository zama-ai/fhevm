use crate::scheduler::{types::ComputationNode, BlockContext, Handle};
use daggy::NodeIndex;
use fhevm_engine_common::types::SupportedFheOperations;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use xxhash_rust::xxh3::xxh3_128;

#[derive(Clone, Serialize, Deserialize)]
pub struct FheLog {
    pub output_handle: Handle,
    pub dependencies: Vec<Handle>,
    pub fhe_operation: SupportedFheOperations,
    pub is_scalar: bool,
    pub is_allowed: bool,
    pub created_at: SystemTime,
    pub block_info: BlockContext,
}

impl std::fmt::Debug for FheLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FheLogMsg {{ output: {:?}, op: {:?}, deps: {:?} }}",
            hex::encode(&self.output_handle[0..4]),
            self.fhe_operation,
            self.dependencies
                .iter()
                .map(|h| hex::encode(&h[0..4]))
                .collect::<Vec<_>>()
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutablePartition {
    pub hash: [u8; 32],
    pub exec_node_idx: NodeIndex,
    pub computations: Vec<(ComputationNode, NodeIndex /* DFG node index */)>,
}

impl ExecutablePartition {
    pub fn new(exec_node_idx: NodeIndex, computations: Vec<(ComputationNode, NodeIndex)>) -> Self {
        let mut partition = Self {
            exec_node_idx,
            computations,
            hash: [0u8; 32],
        };
        partition.hash = partition.calc_hash();
        partition
    }

    pub fn get_dfg_idx(&self, i: usize) -> Option<NodeIndex> {
        self.computations.get(i).map(|(_, idx)| *idx)
    }

    pub fn is_empty(&self) -> bool {
        self.computations.is_empty()
    }

    /// Calculate a hash for the partition based on the output handles of its computations.
    fn calc_hash(&self) -> [u8; 32] {
        let mut buffer = Vec::with_capacity(self.computations.len() * 32);

        for (comp, _) in &self.computations {
            buffer.extend_from_slice(&comp.output_handle);
        }

        let h1 = xxh3_128(&buffer).to_le_bytes();
        let h2 = xxh3_128(&h1).to_le_bytes(); // expand to 32 bytes

        let mut out = [0u8; 32];
        out[..16].copy_from_slice(&h1);
        out[16..].copy_from_slice(&h2);

        // Overwrite the first 2 bytes with the length of the partition for easier debugging
        let len_bytes = (self.computations.len() as u16).to_le_bytes();
        out[0..2].copy_from_slice(&len_bytes);

        out
    }
}

impl std::fmt::Display for ExecutablePartition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let handles = self
            .computations
            .iter()
            .map(|(comp, _)| hex::encode(&comp.output_handle[0..4]))
            .collect::<Vec<_>>()
            .join(", ");
        write!(
            f,
            "ExecutablePartition {{ hash: {:?}, computations: [{}] }}",
            hex::encode(&self.hash[0..4]),
            handles
        )
    }
}
