pub mod scheduler;
mod types;

use crate::dfg::types::*;
use crate::server::{
    CompressedCiphertext, ComputationState, Input, SyncComputation, SyncComputeError,
    SyncComputeRequest,
};
use anyhow::Result;
use fhevm_engine_common::types::{
    FhevmError, Handle, SupportedFheCiphertexts, HANDLE_LEN, SCALAR_LEN,
};

use daggy::{Dag, NodeIndex};
use std::collections::HashMap;

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
        computation: &'a SyncComputation,
        inputs: Vec<DFGTaskInput>,
    ) -> Result<NodeIndex, SyncComputeError> {
        let rh = computation
            .result_handles
            .first()
            .filter(|h| h.len() == HANDLE_LEN)
            .ok_or(SyncComputeError::BadResultHandles)?;
        Ok(self.graph.add_node(OpNode {
            opcode: computation.operation,
            result: None,
            result_handle: rh.clone(),
            inputs,
        }))
    }

    pub fn add_dependence(
        &mut self,
        source: NodeIndex,
        destination: NodeIndex,
        consumer_input: OpEdge,
    ) -> Result<(), SyncComputeError> {
        let _edge = self
            .graph
            .add_edge(source, destination, consumer_input)
            .map_err(|_| SyncComputeError::UnsatisfiedDependence)?;
        Ok(())
    }
    pub fn build_from_request(
        &mut self,
        req: &'a SyncComputeRequest,
        state: &ComputationState,
    ) -> Result<(), SyncComputeError> {
        // Add all computations as nodes in the graph.
        for computation in &req.computations {
            let inputs: Result<Vec<DFGTaskInput>> = computation
                .inputs
                .iter()
                .map(|input| match &input.input {
                    Some(input) => match input {
                        Input::Handle(h) => {
                            if let Some(ct) = state.ciphertexts.get(h) {
                                Ok(DFGTaskInput::Val(ct.expanded.clone()))
                            } else {
                                Ok(DFGTaskInput::Dep(None))
                            }
                        }
                        Input::Scalar(s) if s.len() == SCALAR_LEN => Ok(DFGTaskInput::Val(
                            SupportedFheCiphertexts::Scalar(s.clone()),
                        )),
                        _ => Err(FhevmError::BadInputs.into()),
                    },
                    None => Err(FhevmError::BadInputs.into()),
                })
                .collect();
            if let Ok(mut inputs) = inputs {
                let n = self.add_node(computation, std::mem::take(&mut inputs))?;
                self.produced_handles.insert(
                    computation
                        .result_handles
                        .first()
                        .filter(|h| h.len() == HANDLE_LEN)
                        .ok_or(SyncComputeError::BadResultHandles)?,
                    n,
                );
            }
        }
        // Traverse computations and add dependences/edges as required
        for (index, computation) in req.computations.iter().enumerate() {
            for (input_idx, input) in computation.inputs.iter().enumerate() {
                if let Some(Input::Handle(input)) = &input.input {
                    if !state.ciphertexts.contains_key(input) {
                        if let Some(producer_index) = self.produced_handles.get(input) {
                            let consumer_index = NodeIndex::new(index);
                            self.graph[consumer_index].inputs[input_idx] =
                                DFGTaskInput::Dep(Some((*producer_index).index()));
                            self.add_dependence(*producer_index, consumer_index, input_idx as u8)?;
                        } else {
                            return Err(SyncComputeError::UnsatisfiedDependence);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn get_results(&mut self) -> Result<Vec<CompressedCiphertext>, SyncComputeError> {
        let mut res = Vec::with_capacity(self.graph.node_count());
        for index in 0..self.graph.node_count() {
            let node = self.graph.node_weight_mut(NodeIndex::new(index)).unwrap();
            if let Some(imc) = &node.result {
                res.push(CompressedCiphertext {
                    handle: node.result_handle.clone(),
                    serialization: imc.compressed.clone(),
                });
            } else {
                return Err(SyncComputeError::ComputationFailed);
            }
        }
        Ok(res)
    }
}
