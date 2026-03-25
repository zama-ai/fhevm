use crate::types::{Handle, SupportedFheOperations};
use daggy::NodeIndex;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use xxhash_rust::xxh3::xxh3_128;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockContext {
    pub txn_hash: [u8; 32],
    pub block_number: u64,
    pub block_hash: [u8; 32],
}

type ScalarBytes = Vec<u8>;
pub type PartitionHash = [u8; 16];

#[derive(Clone, Serialize, Deserialize)]
pub struct OpNode {
    pub key_id: u64,
    pub output_handle: Handle,
    pub fhe_operation: SupportedFheOperations,
    pub is_scalar: bool,
    pub scalar_operands: Vec<ScalarBytes>,
    pub created_at: SystemTime,
    pub status: Status,
    pub block_info: BlockContext,
}

impl OpNode {
    pub fn id(&self) -> String {
        self.output_handle
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect()
    }

    pub fn output_handle(&self) -> Handle {
        self.output_handle.clone()
    }
}

impl std::fmt::Debug for OpNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "OpNode{{ output: {:?}, op: {:?}, status: {:?}, scalar_operands: {} }}",
            hex::encode(&self.output_handle[0..4]),
            self.fhe_operation,
            self.status,
            self.scalar_operands.len(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    Pending { remaining_deps: usize },
    Computed { finished_at: SystemTime },
    Malformed { error_code: u8 },
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Dependence {
    Reference(Handle),
    Scalar(ScalarBytes),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FheLog {
    pub output_handle: Handle,
    pub dependencies: Vec<Dependence>,
    pub fhe_operation: SupportedFheOperations,
    pub is_scalar: bool,
    pub is_allowed: bool,
    pub created_at: SystemTime,
    pub block_info: BlockContext,
}

impl FheLog {
    /// Extracts the handles of the dependencies that are references (i.e. not scalars)
    pub fn dependence_handles(&self) -> Vec<Handle> {
        self.dependencies
            .iter()
            .filter_map(|dep| match dep {
                Dependence::Reference(handle) => Some(handle.clone()),
                _ => None,
            })
            .collect()
    }
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
                .filter(|dep| matches!(dep, Dependence::Reference(_)))
                .map(|h| match h {
                    Dependence::Reference(handle) => hex::encode(&handle[0..4]),
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        )
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ExecutablePartition {
    /// Key ID associated with the computations in this partition.
    /// (formerly tenant_id)
    pub key_id: u64,

    /// Unique identifier for the partition, derived from the output handles of its computations
    pub hash: PartitionHash,

    /// Index of the corresponding ExecNode in the scheduler's ExecGraph.
    /// This is used for bookkeeping and updating the DFG when the partition is completed.
    pub exec_node_idx: NodeIndex,

    /// Topologically sorted computations that can be executed sequentially without parallelism.
    pub computations: Vec<(
        OpNode,
        NodeIndex,   /* DFG node index */
        Vec<Handle>, /* dependencies/inputs */
    )>,

    pub created_at: Option<SystemTime>,
}

impl ExecutablePartition {
    pub fn new(
        key_id: u64,
        exec_node_idx: NodeIndex,
        computations: Vec<(
            OpNode,
            NodeIndex,
            Vec<Handle>, /* dependencies/inputs */
        )>,
    ) -> Self {
        let mut partition = Self {
            key_id,
            exec_node_idx,
            computations,
            hash: [0u8; 16],
            created_at: Some(SystemTime::now()),
        };
        partition.hash = partition.compute_hash_inner();
        partition
    }

    pub fn get_dfg_idx(&self, i: usize) -> Option<NodeIndex> {
        self.computations.get(i).map(|(_, idx, _)| *idx)
    }

    pub fn is_empty(&self) -> bool {
        self.computations.is_empty()
    }

    /// Calculate a hash for the partition based on the output handles of its computations.
    fn compute_hash_inner(&self) -> PartitionHash {
        let output_handles: Vec<Handle> = self
            .computations
            .iter()
            .map(|(comp, _, _)| comp.output_handle())
            .collect();

        Self::compute_hash(&output_handles)
    }

    fn compute_hash(output_handles: &[Handle]) -> PartitionHash {
        let mut buffer = Vec::with_capacity(output_handles.len() * 32);

        for handle in output_handles {
            buffer.extend_from_slice(handle);
        }

        let h1 = xxh3_128(&buffer).to_le_bytes();

        let mut out = [0u8; 16];
        out[..16].copy_from_slice(&h1);

        // Overwrite the last 1 byte with the length of the partition for debugging
        let len_bytes = (output_handles.len() as u8).to_le_bytes();
        let from = out.len().saturating_sub(1);
        out[from..].copy_from_slice(&len_bytes);

        out
    }

    /// Create ID from first 2 bytes and last 2 bytes of the hash for easier debugging
    pub fn id(&self) -> String {
        let prefix = &self.hash[..2];
        let suffix = &self.hash[15..];
        format!("{}..{}", hex::encode(prefix), hex::encode(suffix))
    }

    pub fn compute_id(output_handles: &[Handle]) -> String {
        let hash = Self::compute_hash(output_handles);
        let prefix = &hash[..2];
        let suffix = &hash[15..];
        format!("{}..{}", hex::encode(prefix), hex::encode(suffix))
    }

    pub fn get_inputs(&self) -> Vec<Handle> {
        let mut inputs = Vec::new();
        for (_, _, deps) in &self.computations {
            inputs.extend_from_slice(deps);
        }
        inputs
    }
}

impl std::fmt::Display for ExecutablePartition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let handles = self
            .computations
            .iter()
            .map(|(comp, _, _)| hex::encode(&comp.output_handle[0..4]))
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
