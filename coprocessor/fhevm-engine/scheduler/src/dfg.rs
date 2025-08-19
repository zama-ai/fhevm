pub mod scheduler;
pub mod types;

use crate::dfg::types::*;
use anyhow::Result;
use daggy::petgraph::{
    visit::{EdgeRef, IntoEdges, IntoEdgesDirected},
    Direction,
};
use daggy::{petgraph::graph::node_index, Dag, NodeIndex};
use fhevm_engine_common::types::Handle;

pub struct OpNode {
    opcode: i32,
    result: DFGTaskResult,
    result_handle: Handle,
    inputs: Vec<DFGTaskInput>,
    #[cfg(feature = "gpu")]
    locality: i32,
    is_allowed: bool,
    is_needed: bool,
    work_index: usize,
}
pub type OpEdge = u8;

pub struct DFGResult {
    pub handle: Handle,
    pub result: Result<Option<(i16, Vec<u8>)>>,
    pub work_index: usize,
}

impl std::fmt::Debug for OpNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpNode")
            .field("OP", &self.opcode)
            .field(
                "Result handle",
                &format_args!("{:02X?}", &self.result_handle),
            )
            .finish()
    }
}

#[derive(Default, Debug)]
pub struct DFGraph {
    pub graph: Dag<OpNode, OpEdge>,
}

impl DFGraph {
    pub fn add_node(
        &mut self,
        rh: Handle,
        opcode: i32,
        inputs: Vec<DFGTaskInput>,
        is_allowed: bool,
        work_index: usize,
    ) -> Result<NodeIndex> {
        Ok(self.graph.add_node(OpNode {
            opcode,
            result: None,
            result_handle: rh,
            inputs,
            #[cfg(feature = "gpu")]
            locality: -1,
            is_allowed,
            is_needed: is_allowed,
            work_index,
        }))
    }
    pub fn add_dependence(
        &mut self,
        source: usize,
        destination: usize,
        consumer_input: usize,
    ) -> Result<()> {
        let consumer_index = node_index(destination);
        self.graph[consumer_index].inputs[consumer_input] = DFGTaskInput::Dependence(Some(source));
        let _edge = self
            .graph
            .add_edge(
                node_index(source),
                node_index(destination),
                consumer_input as u8,
            )
            .map_err(|_| SchedulerError::CyclicDependence)?;
        Ok(())
    }

    pub fn get_results(&mut self) -> Vec<DFGResult> {
        let mut res = Vec::with_capacity(self.graph.node_count());
        for index in 0..self.graph.node_count() {
            let node = self.graph.node_weight_mut(NodeIndex::new(index)).unwrap();
            if let Some(ct) = std::mem::take(&mut node.result) {
                if let Ok(ct) = ct {
                    if node.is_allowed {
                        res.push(DFGResult {
                            handle: node.result_handle.clone(),
                            result: Ok(ct.1),
                            work_index: node.work_index,
                        });
                    } else {
                        res.push(DFGResult {
                            handle: node.result_handle.clone(),
                            result: Ok(None),
                            work_index: node.work_index,
                        });
                    }
                } else {
                    res.push(DFGResult {
                        handle: node.result_handle.clone(),
                        result: Err(ct.err().unwrap()),
                        work_index: node.work_index,
                    });
                }
            } else {
                res.push(DFGResult {
                    handle: node.result_handle.clone(),
                    result: Err(SchedulerError::DataflowGraphError.into()),
                    work_index: node.work_index,
                });
            }
        }
        res
    }

    fn is_needed(&self, index: usize) -> bool {
        let node_index = NodeIndex::new(index);
        let node = self.graph.node_weight(node_index).unwrap();
        if node.is_allowed || node.is_needed {
            true
        } else {
            for edge in self.graph.edges_directed(node_index, Direction::Outgoing) {
                // If any outgoing dependence is needed, so is this node
                if self.is_needed(edge.target().index()) {
                    return true;
                }
            }
            false
        }
    }

    pub fn finalize(&mut self) {
        // Traverse in reverse order and mark nodes as needed as the
        // graph order is roughly computable, so allowed nodes should
        // generally be later in the graph.
        for index in (0..self.graph.node_count()).rev() {
            if self.is_needed(index) {
                let node = self.graph.node_weight_mut(NodeIndex::new(index)).unwrap();
                node.is_needed = true;
            }
        }
        // Prune graph of all unneeded nodes and edges
        let edges = self.graph.map(|_, _| (), |_, edge| *edge);
        let mut unneeded_nodes = Vec::new();
        for index in 0..self.graph.node_count() {
            let node_index = NodeIndex::new(index);
            let Some(node) = self.graph.node_weight(node_index) else {
                continue;
            };
            if !node.is_needed {
                unneeded_nodes.push(index);
            }
        }
        unneeded_nodes.sort();
        // Remove unneeded nodes and their edges
        for index in unneeded_nodes.iter().rev() {
            let node_index = NodeIndex::new(*index);
            let Some(node) = self.graph.node_weight(node_index) else {
                continue;
            };
            if !node.is_needed {
                for edge in edges.edges(node_index) {
                    self.graph.remove_edge(edge.id());
                }
                self.graph.remove_node(node_index);
            }
        }
    }
}
