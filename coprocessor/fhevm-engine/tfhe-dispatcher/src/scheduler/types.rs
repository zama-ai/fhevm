use crate::scheduler::{BlockContext, Handle};
use daggy::{Dag, NodeIndex};
use fhevm_engine_common::types::SupportedFheOperations;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Clone, Serialize, Deserialize)]
pub struct ComputationNode {
    pub key_id: u64,
    pub output_handle: Handle,
    pub fhe_operation: SupportedFheOperations,
    pub is_scalar: bool,
    pub created_at: SystemTime,
    pub status: Status,
    pub block_info: BlockContext,
}

impl ComputationNode {
    pub fn id(&self) -> String {
        self.output_handle
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect()
    }
}

impl std::fmt::Debug for ComputationNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Computation {{ output: {:?}, op: {:?}, status: {:?} }}",
            hex::encode(&self.output_handle[0..4]),
            self.fhe_operation,
            self.status
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    Pending { remaining_deps: usize },
    Computing { started_at: SystemTime },
    Computed { finished_at: SystemTime },
    Malformed { error_code: u8 },
}

#[derive(Debug, Clone)]
pub struct ExecNode {
    /// Chain of DFG nodes that can be executed sequentially without parallelism
    pub chain: Vec<NodeIndex>,

    /// Number of ExecNode dependences that must be completed before this node can be executed.
    pub dependence_counter: usize,
}

impl ExecNode {
    /// Check if all dependences have been satisfied
    pub fn is_ready(&self) -> bool {
        self.dependence_counter == 0
    }
}

pub struct ExecGraph<'a> {
    pub inner: Dag<ExecNode, ()>,
    pub dfg_ind_to_exec_ind: HashMap<NodeIndex, NodeIndex>,

    pub dfg_ref: &'a Dag<ComputationNode, ()>,
}
