pub mod scheduler;
pub mod types;

use crate::dfg::types::*;
use anyhow::Result;
use daggy::{petgraph::graph::node_index, Dag, NodeIndex};
use fhevm_engine_common::types::Handle;

pub struct OpNode {
    opcode: i32,
    result: DFGTaskResult,
    result_handle: Handle,
    inputs: Vec<DFGTaskInput>,
    #[cfg(feature = "gpu")]
    locality: i32,
}
pub type OpEdge = u8;

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
    ) -> Result<NodeIndex> {
        Ok(self.graph.add_node(OpNode {
            opcode,
            result: None,
            result_handle: rh,
            inputs,
            #[cfg(feature = "gpu")]
            locality: -1,
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

    #[expect(clippy::type_complexity)]
    pub fn get_results(&mut self) -> Vec<(Handle, Result<(i16, Vec<u8>)>)> {
        let mut res = Vec::with_capacity(self.graph.node_count());
        for index in 0..self.graph.node_count() {
            let node = self.graph.node_weight_mut(NodeIndex::new(index)).unwrap();
            if let Some(ct) = std::mem::take(&mut node.result) {
                if let Ok(ct) = ct {
                    res.push((node.result_handle.clone(), Ok((ct.1, ct.2))));
                } else {
                    res.push((node.result_handle.clone(), Err(ct.err().unwrap())));
                }
            } else {
                res.push((
                    node.result_handle.clone(),
                    Err(SchedulerError::DataflowGraphError.into()),
                ));
            }
        }
        res
    }
}
