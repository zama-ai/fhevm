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
use tfhe::integer::U256;

use daggy::{Dag, NodeIndex};
use std::collections::HashMap;

//TODO#[derive(Debug)]
pub struct Node<'a> {
    computation: &'a SyncComputation,
    result: DFGTaskResult,
    result_handle: Handle,
    inputs: Vec<DFGTaskInput>,
}
pub type Edge = u8;

//TODO#[derive(Debug)]
#[derive(Default)]
pub struct DFGraph<'a> {
    pub graph: Dag<Node<'a>, Edge>,
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
        Ok(self.graph.add_node(Node {
            computation,
            result: None,
            result_handle: rh.clone(),
            inputs,
        }))
    }

    pub fn add_dependence(
        &mut self,
        source: NodeIndex,
        destination: NodeIndex,
        consumer_input: Edge,
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
                                Ok(DFGTaskInput::Handle(h.clone()))
                            }
                        }
                        Input::Scalar(s) if s.len() == SCALAR_LEN => {
                            let mut scalar = U256::default();
                            scalar.copy_from_be_byte_slice(s);
                            Ok(DFGTaskInput::Val(SupportedFheCiphertexts::Scalar(scalar)))
                        }
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
        // Traverse nodes and add dependences/edges as required
        for index in 0..self.graph.node_count() {
            let take_inputs = std::mem::take(
                &mut self
                    .graph
                    .node_weight_mut(NodeIndex::new(index))
                    .unwrap()
                    .inputs,
            );
            for (idx, input) in take_inputs.iter().enumerate() {
                match input {
                    DFGTaskInput::Handle(input) => {
                        if let Some(producer_index) = self.produced_handles.get(input) {
                            self.add_dependence(*producer_index, NodeIndex::new(index), idx as u8)?;
                        }
                    }
                    DFGTaskInput::Val(_) => {}
                };
            }
            self.graph
                .node_weight_mut(NodeIndex::new(index))
                .unwrap()
                .inputs = take_inputs;
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
