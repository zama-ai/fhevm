use daggy::{Dag, NodeIndex};
use fhevm_engine_common::protocol::messages::OpNode;
use std::collections::HashMap;

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

    pub dfg_ref: &'a Dag<OpNode, ()>,
}
