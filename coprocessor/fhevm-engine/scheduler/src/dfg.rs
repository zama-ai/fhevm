pub mod scheduler;
pub mod types;

use std::collections::HashMap;

use crate::dfg::types::*;
use anyhow::Result;
use daggy::petgraph::{
    visit::{EdgeRef, IntoEdgesDirected, IntoNodeReferences},
    Direction,
};
use daggy::{petgraph::graph::node_index, Dag, NodeIndex};
use fhevm_engine_common::types::{Handle, SupportedFheCiphertexts, SupportedFheOperations};
use tracing::{error, warn};

#[derive(Debug)]
pub struct DFGOp {
    pub output_handle: Handle,
    pub fhe_op: SupportedFheOperations,
    pub inputs: Vec<DFGTaskInput>,
    pub is_allowed: bool,
}
pub type TxEdge = ();
#[derive(Default)]
pub struct TxNode {
    // Inner dataflow graph
    pub graph: DFGraph,
    // Allowed handles or verified input handles, with a map of
    // internal DFG node indexes to input positions in the
    // corresponding FHE op
    pub inputs: HashMap<Handle, Option<DFGTxInput>>,
    // Only allowed handles can be results (used beyond the
    // transaction)
    pub results: Vec<Handle>,
    pub transaction_id: Handle,
    pub is_uncomputable: bool,
    pub intermediate_handles: Vec<Handle>,
}
impl TxNode {
    pub fn build(&mut self, mut operations: Vec<DFGOp>, transaction_id: &Handle) -> Result<()> {
        self.transaction_id = transaction_id.clone();
        self.is_uncomputable = false;
        // Gather all handles produced within the transaction
        let mut produced_handles: HashMap<Handle, usize> = HashMap::new();
        for (index, op) in operations.iter().enumerate() {
            produced_handles.insert(op.output_handle.clone(), index);
        }
        let mut dependence_pairs = vec![];
        for (index, op) in operations.iter_mut().enumerate() {
            for (pos, i) in op.inputs.iter().enumerate() {
                match i {
                    DFGTaskInput::Dependence(dh) => {
                        // Check which dependences are satisfied internally,
                        // all missing ones are exposed as required inputs at
                        // transaction level.
                        let producer = produced_handles.get(dh);
                        if let Some(producer) = producer {
                            dependence_pairs.push((*producer, index, pos));
                        } else {
                            self.inputs.entry(dh.clone()).or_insert(None);
                        }
                    }
                    DFGTaskInput::Value(_) | DFGTaskInput::Compressed(_) => {}
                }
            }
            if op.is_allowed {
                self.results.push(op.output_handle.clone());
            } else {
                self.intermediate_handles.push(op.output_handle.clone());
            }
            assert!(
                index
                    == self
                        .graph
                        .add_node(
                            op.output_handle.clone(),
                            (op.fhe_op as i16).into(),
                            std::mem::take(&mut op.inputs),
                            op.is_allowed,
                        )
                        .index()
            );
        }
        for (source, destination, pos) in dependence_pairs {
            // This returns an error in case of circular
            // dependences. This should not be possible.
            self.graph.add_dependence(source, destination, pos)?;
        }
        Ok(())
    }
    pub fn add_input(&mut self, handle: &[u8], cct: DFGTxInput) {
        self.inputs
            .entry(handle.to_vec())
            .and_modify(|v| *v = Some(cct));
    }
}
impl std::fmt::Debug for TxNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(f, "Transaction: [{:?}]", self.transaction_id);
        let _ = writeln!(
            f,
            "{:?}",
            daggy::petgraph::dot::Dot::with_config(self.graph.graph.graph(), &[])
        );
        let _ = writeln!(f, "Inputs :");
        for i in self.inputs.iter() {
            let _ = writeln!(f, "\t {:?}", i);
        }
        let _ = writeln!(f, "Results :");
        for r in self.results.iter() {
            let _ = writeln!(f, "\t {:?}", r);
        }
        writeln!(f)
    }
}

#[derive(Default)]
pub struct DFTxGraph {
    pub graph: Dag<TxNode, TxEdge>,
    pub needed_map: HashMap<Handle, Vec<NodeIndex>>,
    pub allowed_map: HashMap<Handle, NodeIndex>,
    pub results: Vec<DFGTxResult>,
}
impl DFTxGraph {
    pub fn build(&mut self, nodes: &mut Vec<TxNode>) -> Result<()> {
        while let Some(tx) = nodes.pop() {
            self.graph.add_node(tx);
        }
        // Gather handles produced within the graph
        for (producer, tx) in self.graph.node_references() {
            for r in tx.results.iter() {
                self.allowed_map.insert(r.clone(), producer);
            }
        }
        // Identify all dependence pairs (producer, consumer)
        let mut dependence_pairs = vec![];
        for (consumer, tx) in self.graph.node_references() {
            for i in tx.inputs.keys() {
                if let Some(producer) = self.allowed_map.get(i) {
                    if *producer == consumer {
                        warn!(target: "scheduler", { },
			       "Self-dependence on node");
                    } else {
                        dependence_pairs.push((*producer, consumer));
                    }
                } else {
                    self.needed_map
                        .entry(i.clone())
                        .and_modify(|uses| uses.push(consumer))
                        .or_insert(vec![consumer]);
                }
            }
        }

        // We build a replica of the graph and map it to the
        // underlying DiGraph so we can identify cycles.
        let mut digraph = self.graph.map(|idx, _| idx, |_, _| ()).graph().clone();
        // Add transaction dependence edges
        for (producer, consumer) in dependence_pairs.iter() {
            digraph.add_edge(*producer, *consumer, ());
        }
        let mut tarjan = daggy::petgraph::algo::TarjanScc::new();
        let mut sccs = Vec::new();
        tarjan.run(&digraph, |scc| {
            if scc.len() > 1 {
                // All non-singleton SCCs in a directed graph are
                // dependence cycles
                sccs.push(scc.to_vec());
            }
        });
        if !sccs.is_empty() {
            for scc in sccs {
                error!(target: "scheduler", { cycle_size = ?scc.len() },
		       "Dependence cycle detected");
                for idx in scc {
                    let idx = digraph
                        .node_weight(idx)
                        .ok_or(SchedulerError::DataflowGraphError)?;
                    let tx = self
                        .graph
                        .node_weight_mut(*idx)
                        .ok_or(SchedulerError::DataflowGraphError)?;
                    // Mark the node as uncomputable so we don't go
                    // and mark as completed operations that are in
                    // error.
                    tx.is_uncomputable = true;
                    error!(target: "scheduler", { transaction_id = ?hex::encode(tx.transaction_id.clone()) },
		       "Transaction is part of a dependence cycle");
                    for (_, op) in tx.graph.graph.node_references() {
                        self.results.push(DFGTxResult {
                            transaction_id: tx.transaction_id.clone(),
                            handle: op.result_handle.to_vec(),
                            compressed_ct: Err(SchedulerError::CyclicDependence.into()),
                        });
                    }
                }
            }
            return Err(SchedulerError::CyclicDependence.into());
        } else {
            // If no dependence cycles were found, then we can
            // complete the graph and proceed to execution
            for (producer, consumer) in dependence_pairs.iter() {
                // The error case here should not happen as we've
                // already covered it by testing for SCCs in the graph
                // first
                self.graph
                    .add_edge(*producer, *consumer, ())
                    .map_err(|_| SchedulerError::CyclicDependence)?;
            }
        }
        Ok(())
    }

    pub fn add_input(&mut self, handle: &[u8], input: &DFGTxInput) -> Result<()> {
        if let Some(nodes) = self.needed_map.get(handle) {
            for n in nodes.iter() {
                let node = self
                    .graph
                    .node_weight_mut(*n)
                    .ok_or(SchedulerError::DataflowGraphError)?;
                node.add_input(handle, input.clone());
            }
        }
        Ok(())
    }
    pub fn add_output(
        &mut self,
        handle: &[u8],
        result: Result<(SupportedFheCiphertexts, i16, Vec<u8>)>,
        edges: &Dag<(), TxEdge>,
    ) -> Result<()> {
        if let Some(producer) = self.allowed_map.get(handle).cloned() {
            if let Ok(ref result) = result {
                // Traverse immediate dependents and add this result as an input
                for edge in edges.edges_directed(producer, Direction::Outgoing) {
                    let dependent_tx_index = edge.target();
                    let dependent_tx = self
                        .graph
                        .node_weight_mut(dependent_tx_index)
                        .ok_or(SchedulerError::DataflowGraphError)?;
                    dependent_tx
                        .inputs
                        .entry(handle.to_vec())
                        .and_modify(|v| *v = Some(DFGTxInput::Value(result.0.clone())));
                }
            } else {
                // If this result was an error, mark this transaction
                // and all its dependents as uncomputable, we will
                // skip them during scheduling
                self.set_uncomputable(producer, edges)?;
            }
            // Finally add the output (either error or compressed
            // ciphertext) to the graph's outputs
            let producer_tx = self
                .graph
                .node_weight_mut(producer)
                .ok_or(SchedulerError::DataflowGraphError)?;
            self.results.push(DFGTxResult {
                transaction_id: producer_tx.transaction_id.clone(),
                handle: handle.to_vec(),
                compressed_ct: result.map(|rok| (rok.1, rok.2)),
            });
        }
        Ok(())
    }
    // Set a node as uncomputable and recursively traverse graph to
    // set its dependents as uncomputable as well
    fn set_uncomputable(
        &mut self,
        tx_node_index: NodeIndex,
        edges: &Dag<(), TxEdge>,
    ) -> Result<()> {
        let tx_node = self
            .graph
            .node_weight_mut(tx_node_index)
            .ok_or(SchedulerError::DataflowGraphError)?;
        tx_node.is_uncomputable = true;
        for edge in edges.edges_directed(tx_node_index, Direction::Outgoing) {
            let dependent_tx_index = edge.target();
            self.set_uncomputable(dependent_tx_index, edges)?;
        }
        Ok(())
    }
    pub fn get_results(&mut self) -> Vec<DFGTxResult> {
        std::mem::take(&mut self.results)
    }
    pub fn get_intermediate_handles(&mut self) -> Vec<(Handle, Handle)> {
        let mut res = vec![];
        for tx in self.graph.node_weights_mut() {
            if !tx.is_uncomputable {
                res.append(
                    &mut (std::mem::take(&mut tx.intermediate_handles))
                        .into_iter()
                        .map(|h| (h, tx.transaction_id.clone()))
                        .collect::<Vec<_>>(),
                );
            }
        }
        res
    }
}
impl std::fmt::Debug for DFTxGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(f, "Transaction Graph:",);
        let _ = writeln!(
            f,
            "{:?}",
            daggy::petgraph::dot::Dot::with_config(self.graph.graph(), &[])
        );
        let _ = writeln!(f, "Needed Inputs :");
        for i in self.needed_map.iter() {
            let _ = writeln!(f, "\t {:?}", i);
        }
        let _ = writeln!(f, "Results :");
        for r in self.results.iter() {
            let _ = writeln!(f, "\t {:?}", r);
        }
        writeln!(f)
    }
}

pub struct DFGResult {
    pub handle: Handle,
    pub result: Result<Option<(i16, Vec<u8>)>>,
    pub work_index: usize,
}
pub type OpEdge = u8;
pub struct OpNode {
    opcode: i32,
    result_handle: Handle,
    inputs: Vec<DFGTaskInput>,
    #[cfg(feature = "gpu")]
    locality: i32,
    is_allowed: bool,
}
impl std::fmt::Debug for OpNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpNode")
            .field("OP", &self.opcode)
            .field("Result handle", &format_args!("{:?}", &self.result_handle))
            .finish()
    }
}
impl OpNode {
    fn check_ready_inputs(&mut self, ct_map: &mut HashMap<Handle, Option<DFGTxInput>>) -> bool {
        for i in self.inputs.iter_mut() {
            if !matches!(i, DFGTaskInput::Value(_)) {
                let DFGTaskInput::Dependence(d) = i else {
                    return false;
                };
                let Some(Some(DFGTxInput::Value(val))) = ct_map.get(d) else {
                    return false;
                };
                *i = DFGTaskInput::Value(val.clone());
            }
        }
        true
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
    ) -> NodeIndex {
        self.graph.add_node(OpNode {
            opcode,
            result_handle: rh,
            inputs,
            #[cfg(feature = "gpu")]
            locality: -1,
            is_allowed,
        })
    }
    pub fn add_dependence(
        &mut self,
        source: usize,
        destination: usize,
        consumer_input: usize,
    ) -> Result<()> {
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
}
