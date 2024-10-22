pub mod scheduler;
pub mod types;

use crate::dfg::types::*;
use anyhow::Result;
use std::{cell::RefCell, collections::HashMap};
// use executor::server::{
//     CompressedCiphertext, ComputationState, Input, SyncComputation, SyncComputeError,
//     SyncComputeRequest,
// };
use daggy::{petgraph::graph::node_index, Dag, NodeIndex};
use fhevm_engine_common::types::{
    FhevmError, Handle, SupportedFheCiphertexts, HANDLE_LEN, SCALAR_LEN,
};
use tfhe::integer::U256;

thread_local! {
    pub static THREAD_POOL: RefCell<Option<rayon::ThreadPool>> = const {RefCell::new(None)};
}

pub struct OpNode {
    opcode: i32,
    result: DFGTaskResult,
    result_handle: Handle,
    inputs: Vec<DFGTaskInput>,
}
pub type OpEdge = u8;

impl std::fmt::Debug for OpNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpNode")
            .field("OP", &self.opcode)
            .field(
                "Result",
                &format_args!("{0:?} (0x{0:X})", &self.result_handle[0]),
            )
            .finish()
    }
}

#[derive(Default, Debug)]
pub struct DFGraph<'a> {
    pub graph: Dag<OpNode, OpEdge>,
    produced_handles: HashMap<&'a Handle, NodeIndex>,
}

impl<'a> DFGraph<'a> {
    pub fn add_node(
        &mut self,
        rh: &'a Handle,
        opcode: i32,
        inputs: Vec<DFGTaskInput>,
    ) -> Result<NodeIndex, SchedulerError> {
        Ok(self.graph.add_node(OpNode {
            opcode,
            result: None,
            result_handle: rh.clone(),
            inputs,
        }))
    }
    pub fn add_dependence(
        &mut self,
        source: usize,
        destination: usize,
        consumer_input: usize,
    ) -> Result<(), SchedulerError> {
        let consumer_index = node_index(destination);
        self.graph[consumer_index].inputs[consumer_input] = DFGTaskInput::Dep(Some(source));
        let _edge = self
            .graph
            .add_edge(
                node_index(source),
                node_index(destination),
                consumer_input as u8,
            )
            .map_err(|_| SchedulerError::SchedulerError)?;
        Ok(())
    }

    pub fn get_results(
        &mut self,
    ) -> Result<Vec<(Handle, SupportedFheCiphertexts)>, SchedulerError> {
        let mut res = Vec::with_capacity(self.graph.node_count());
        for index in 0..self.graph.node_count() {
            let node = self.graph.node_weight_mut(NodeIndex::new(index)).unwrap();
            if let Some(ct) = &node.result {
                res.push((node.result_handle.clone(), ct.clone()));
            } else {
                return Err(SchedulerError::SchedulerError);
            }
        }
        Ok(res)
    }
}
