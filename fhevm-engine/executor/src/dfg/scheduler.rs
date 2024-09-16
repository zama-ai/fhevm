use crate::dfg::{types::DFGTaskInput, Edge, Node};
use crate::server::{run_computation, InMemoryCiphertext, SyncComputeError};
use anyhow::Result;
use fhevm_engine_common::types::SupportedFheCiphertexts;

use daggy::{
    petgraph::{
        visit::{EdgeRef, IntoEdgesDirected},
        Direction,
    },
    Dag, NodeIndex,
};
use tokio::task::JoinSet;

pub struct Scheduler<'a, 'b> {
    graph: &'b mut Dag<Node<'a>, Edge>,
    edges: Dag<(), Edge>,
    set: JoinSet<Result<(usize, InMemoryCiphertext), SyncComputeError>>,
}

impl<'a, 'b> Scheduler<'a, 'b> {
    fn is_ready(node: &Node<'a>) -> bool {
        let mut ready = true;
        for i in node.inputs.iter() {
            if let DFGTaskInput::Handle(_) = i {
                ready = false;
            }
        }
        ready
    }
    pub fn new(graph: &'b mut Dag<Node<'a>, Edge>) -> Self {
        let mut set = JoinSet::new();
        for idx in 0..graph.node_count() {
            let index = NodeIndex::new(idx);
            let node = graph.node_weight_mut(index).unwrap();
            if Self::is_ready(node) {
                let opc = node.computation.operation;
                let inputs: Result<Vec<SupportedFheCiphertexts>, SyncComputeError> = node
                    .inputs
                    .iter()
                    .map(|i| match i {
                        DFGTaskInput::Val(i) => Ok(i.clone()),
                        DFGTaskInput::Handle(_) => Err(SyncComputeError::ComputationFailed),
                    })
                    .collect();
                set.spawn_blocking(move || run_computation(opc, inputs, idx));
            }
        }

        let edges = graph.map(|_, _| (), |_, edge| *edge);

        Self { graph, edges, set }
    }
    pub async fn schedule(&mut self) -> Result<(), SyncComputeError> {
        while let Some(result) = self.set.join_next().await {
            let output = result.map_err(|_| SyncComputeError::ComputationFailed)??;
            let index = output.0;
            let node_index = NodeIndex::new(index);
            // Satisfy deps from the executed task
            for edge in self.edges.edges_directed(node_index, Direction::Outgoing) {
                let child_index = edge.target();
                let child_node = self.graph.node_weight_mut(child_index).unwrap();
                child_node.inputs[*edge.weight() as usize] =
                    DFGTaskInput::Val(output.1.expanded.clone());
                if Self::is_ready(child_node) {
                    let opc = child_node.computation.operation;
                    let inputs: Result<Vec<SupportedFheCiphertexts>, SyncComputeError> = child_node
                        .inputs
                        .iter()
                        .map(|i| match i {
                            DFGTaskInput::Val(i) => Ok(i.clone()),
                            DFGTaskInput::Handle(_) => Err(SyncComputeError::ComputationFailed),
                        })
                        .collect();
                    self.set
                        .spawn_blocking(move || run_computation(opc, inputs, child_index.index()));
                }
            }
            self.graph.node_weight_mut(node_index).unwrap().result = Some(output.1);
        }
        Ok(())
    }
}
