pub mod scheduler;
pub mod types;

use std::{collections::HashMap, sync::atomic::AtomicUsize};
use tracing::error;

use crate::dfg::types::*;
use anyhow::Result;
use daggy::{
    petgraph::{
        graph::node_index,
        visit::{
            EdgeRef, IntoEdgeReferences, IntoEdgesDirected, IntoNeighbors, IntoNodeReferences,
            VisitMap, Visitable,
        },
        Direction::{self, Incoming},
    },
    Dag, NodeIndex,
};
use fhevm_engine_common::types::{Handle, SupportedFheOperations};

pub struct ExecNode {
    df_nodes: Vec<NodeIndex>,
    dependence_counter: AtomicUsize,
    #[cfg(feature = "gpu")]
    locality: i32,
}
impl std::fmt::Debug for ExecNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.df_nodes.is_empty() {
            write!(f, "Vec [ ]")
        } else {
            let _ = write!(f, "Vec [ ");
            for i in self.df_nodes.iter() {
                let _ = write!(f, "{}, ", i.index());
            }
            write!(f, "] - dependences: {:?}", self.dependence_counter)
        }
    }
}

#[derive(Debug)]
pub struct DFGOp {
    pub output_handle: Handle,
    pub fhe_op: SupportedFheOperations,
    pub inputs: Vec<DFGTaskInput>,
    pub is_allowed: bool,
}
impl Default for DFGOp {
    fn default() -> Self {
        DFGOp {
            output_handle: vec![],
            fhe_op: SupportedFheOperations::FheTrivialEncrypt,
            inputs: vec![],
            is_allowed: false,
        }
    }
}
pub type ComponentEdge = ();
#[derive(Default)]
pub struct ComponentNode {
    // Inner dataflow graph
    pub graph: DFGraph,
    pub ops: Vec<DFGOp>,
    // Allowed handles or verified input handles, with a map of
    // internal DFG node indexes to input positions in the
    // corresponding FHE op
    pub inputs: HashMap<Handle, Option<DFGTxInput>>,
    pub results: Vec<Handle>,
    pub unneeded: Vec<Handle>,
    pub transaction_id: Handle,
    pub is_uncomputable: bool,
    pub component_id: usize,
}

fn is_needed(graph: &Dag<(bool, usize), OpEdge>, index: usize) -> bool {
    let node_index = NodeIndex::new(index);
    let node = match graph.node_weight(node_index) {
        Some(n) => n,
        None => {
            error!(target: "scheduler", "Missing node for index in DFG finalization");
            return false;
        }
    };
    if node.0 {
        true
    } else {
        for edge in graph.edges_directed(node_index, Direction::Outgoing) {
            // If any outgoing dependence is needed, so is this node
            if is_needed(graph, edge.target().index()) {
                return true;
            }
        }
        false
    }
}

pub fn finalize(graph: &mut Dag<(bool, usize), OpEdge>) -> Vec<usize> {
    // Traverse in reverse order and mark nodes as needed as the
    // graph order is roughly computable, so allowed nodes should
    // generally be later in the graph.
    for index in (0..graph.node_count()).rev() {
        if is_needed(graph, index) {
            let node = match graph.node_weight_mut(NodeIndex::new(index)) {
                Some(n) => n,
                None => {
                    // Shouldn't happen - if this fails we don't prune and execute all the graph
                    error!(target: "scheduler", "Missing node for index in DFG finalization");
                    return vec![];
                }
            };
            node.0 = true;
        }
    }
    // Prune graph of all unneeded nodes and edges
    let mut unneeded_nodes = Vec::new();
    for index in 0..graph.node_count() {
        let node_index = NodeIndex::new(index);
        let Some(node) = graph.node_weight(node_index) else {
            continue;
        };
        if !node.0 {
            unneeded_nodes.push(index);
        }
    }
    unneeded_nodes.sort();
    // Remove unneeded nodes and their edges
    for index in unneeded_nodes.iter().rev() {
        let node_index = NodeIndex::new(*index);
        let Some(node) = graph.node_weight(node_index) else {
            continue;
        };
        if !node.0 {
            graph.remove_node(node_index);
        }
    }
    unneeded_nodes
}

type ComponentNodes = Result<(Vec<ComponentNode>, Vec<(Handle, Handle)>)>;
pub fn build_component_nodes(
    mut operations: Vec<DFGOp>,
    transaction_id: &Handle,
) -> ComponentNodes {
    operations.sort_by_key(|o| o.output_handle.clone());
    let mut graph: Dag<(bool, usize), OpEdge> = Dag::default();
    let mut produced_handles: HashMap<Handle, usize> = HashMap::new();
    let mut components: Vec<ComponentNode> = vec![];
    for (index, op) in operations.iter().enumerate() {
        produced_handles.insert(op.output_handle.clone(), index);
    }
    let mut dependence_pairs = vec![];
    // Determine dependences within this graph
    for (index, op) in operations.iter().enumerate() {
        for (pos, i) in op.inputs.iter().enumerate() {
            match i {
                DFGTaskInput::Dependence(dh) => {
                    let producer = produced_handles.get(dh);
                    if let Some(producer) = producer {
                        dependence_pairs.push((*producer, index, pos));
                    }
                }
                DFGTaskInput::Value(_) | DFGTaskInput::Compressed(_) => {}
            }
        }
        let node_idx = graph.add_node((op.is_allowed, index)).index();
        if index != node_idx {
            return Err(SchedulerError::DataflowGraphError.into());
        }
    }
    for (source, destination, pos) in dependence_pairs {
        // This returns an error in case of circular
        // dependences. This should not be possible.
        graph
            .add_edge(node_index(source), node_index(destination), pos as u8)
            .map_err(|_| SchedulerError::CyclicDependence)?;
    }
    // Prune unneeded branches from the graph
    let unneeded: Vec<(Handle, Handle)> = finalize(&mut graph)
        .into_iter()
        .map(|i| (operations[i].output_handle.clone(), transaction_id.clone()))
        .collect();
    // Partition the graph and extract sequential components
    let mut execution_graph: Dag<ExecNode, ()> = Dag::default();
    partition_preserving_parallelism(&graph, &mut execution_graph)?;
    for idx in 0..execution_graph.node_count() {
        let index = NodeIndex::new(idx);
        let node = execution_graph
            .node_weight_mut(index)
            .ok_or(SchedulerError::DataflowGraphError)?;
        let mut component = ComponentNode::default();
        let mut component_ops = vec![];
        for i in node.df_nodes.iter() {
            let op_node = graph
                .node_weight(*i)
                .ok_or(SchedulerError::DataflowGraphError)?;
            component_ops.push(std::mem::take(&mut operations[op_node.1]));
        }
        component.build(component_ops, transaction_id, idx)?;
        components.push(component);
    }
    Ok((components, unneeded))
}

impl ComponentNode {
    pub fn build(
        &mut self,
        mut operations: Vec<DFGOp>,
        transaction_id: &Handle,
        component_id: usize,
    ) -> Result<()> {
        self.transaction_id = transaction_id.clone();
        self.component_id = component_id;
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
            self.results.push(op.output_handle.clone());
            let node_idx = self
                .graph
                .add_node(
                    op.output_handle.clone(),
                    (op.fhe_op as i16).into(),
                    std::mem::take(&mut op.inputs),
                    op.is_allowed,
                )
                .index();
            if index != node_idx {
                return Err(SchedulerError::DataflowGraphError.into());
            }
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
impl std::fmt::Debug for ComponentNode {
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
pub struct DFComponentGraph {
    pub graph: Dag<ComponentNode, ComponentEdge>,
    pub needed_map: HashMap<Handle, Vec<NodeIndex>>,
    pub allowed_map: HashMap<Handle, NodeIndex>,
    pub results: Vec<DFGTxResult>,
}
impl DFComponentGraph {
    pub fn build(&mut self, nodes: &mut Vec<ComponentNode>) -> Result<()> {
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
                    dependence_pairs.push((producer, consumer));
                } else {
                    self.needed_map
                        .entry(i.clone())
                        .and_modify(|uses| uses.push(consumer))
                        .or_insert(vec![consumer]);
                }
            }
        }
        // Add transaction dependence edges
        for (producer, consumer) in dependence_pairs {
            // Error only occurs in case of cyclic dependence which
            // shoud not be possible between transactions. In that
            // case, the whole cycle should be put in an error state.
            self.graph
                .add_edge(*producer, consumer, ())
                .map_err(|_| SchedulerError::CyclicDependence)?;
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
        result: Result<TaskResult>,
        edges: &Dag<(), ComponentEdge>,
    ) -> Result<()> {
        if let Some(producer) = self.allowed_map.get(handle).cloned() {
            let mut save_result = true;
            if let Ok(ref result) = result {
                save_result = result.is_allowed;
                // Traverse immediate dependents and add this result as an input
                for edge in edges.edges_directed(producer, Direction::Outgoing) {
                    let dependent_tx_index = edge.target();
                    let dependent_tx = self
                        .graph
                        .node_weight_mut(dependent_tx_index)
                        .ok_or(SchedulerError::DataflowGraphError)?;
                    dependent_tx.inputs.entry(handle.to_vec()).and_modify(|v| {
                        *v = Some(DFGTxInput::Value((result.ct.clone(), result.is_allowed)))
                    });
                }
            } else {
                // If this result was an error, mark this transaction
                // and all its dependents as uncomputable, we will
                // skip them during scheduling
                self.set_uncomputable(producer, edges)?;
            }
            // Finally add the output (either error or compressed
            // ciphertext) to the graph's outputs
            if save_result {
                let producer_tx = self
                    .graph
                    .node_weight_mut(producer)
                    .ok_or(SchedulerError::DataflowGraphError)?;
                if let Ok(ref r) = result {
                    if r.compressed_ct.is_none() {
                        error!(target: "scheduler", {handle = ?hex::encode(handle) }, "Missing compressed ciphertext in task result");
                        return Err(SchedulerError::SchedulerError.into());
                    }
                }
                self.results.push(DFGTxResult {
                    transaction_id: producer_tx.transaction_id.clone(),
                    handle: handle.to_vec(),
                    compressed_ct: result.map(|rok| {
                        // Safe to unwrap as this is checked above
                        let cct = rok.compressed_ct.unwrap();
                        (cct.0, cct.1)
                    }),
                });
            }
        }
        Ok(())
    }
    // Set a node as uncomputable and recursively traverse graph to
    // set its dependents as uncomputable as well
    fn set_uncomputable(
        &mut self,
        tx_node_index: NodeIndex,
        edges: &Dag<(), ComponentEdge>,
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
    pub fn get_handles(&mut self) -> Vec<(Handle, Handle)> {
        let mut res = vec![];
        for tx in self.graph.node_weights_mut() {
            if !tx.is_uncomputable {
                res.append(
                    &mut (std::mem::take(&mut tx.results))
                        .into_iter()
                        .map(|h| (h, tx.transaction_id.clone()))
                        .collect::<Vec<_>>(),
                );
            }
        }
        res
    }
}
impl std::fmt::Debug for DFComponentGraph {
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
                let Some(Some(DFGTxInput::Value((val, _)))) = ct_map.get(d) else {
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

pub fn add_execution_depedences<TNode, TEdge>(
    graph: &Dag<TNode, TEdge>,
    execution_graph: &mut Dag<ExecNode, ()>,
    node_map: HashMap<NodeIndex, NodeIndex>,
) -> Result<()> {
    // Once the DFG is partitioned, we need to add dependences as
    // edges in the execution graph
    for edge in graph.edge_references() {
        let (xsrc, xdst) = (
            node_map
                .get(&edge.source())
                .ok_or(SchedulerError::DataflowGraphError)?,
            node_map
                .get(&edge.target())
                .ok_or(SchedulerError::DataflowGraphError)?,
        );
        if xsrc != xdst && execution_graph.find_edge(*xsrc, *xdst).is_none() {
            let _ = execution_graph.add_edge(*xsrc, *xdst, ());
        }
    }
    for node in 0..execution_graph.node_count() {
        let deps = execution_graph
            .edges_directed(node_index(node), Incoming)
            .count();
        execution_graph[node_index(node)]
            .dependence_counter
            .store(deps, std::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn partition_preserving_parallelism<TNode, TEdge>(
    graph: &Dag<TNode, TEdge>,
    execution_graph: &mut Dag<ExecNode, ()>,
) -> Result<()> {
    // First sort the DAG in a schedulable order
    let ts = daggy::petgraph::algo::toposort(graph, None)
        .map_err(|_| SchedulerError::CyclicDependence)?;
    let mut vis = graph.visit_map();
    let mut node_map = HashMap::new();
    // Traverse the DAG and build a graph of connected components
    // without siblings (i.e. without parallelism)
    for nidx in ts.iter() {
        if !vis.is_visited(nidx) {
            vis.visit(*nidx);
            let mut df_nodes = vec![*nidx];
            let mut stack = vec![*nidx];
            while let Some(n) = stack.pop() {
                if graph.edges_directed(n, Direction::Outgoing).count() == 1 {
                    for child in graph.neighbors(n) {
                        if !vis.is_visited(&child.index())
                            && graph.edges_directed(child, Direction::Incoming).count() == 1
                        {
                            df_nodes.push(child);
                            stack.push(child);
                            vis.visit(child.index());
                        }
                    }
                }
            }
            let ex_node = execution_graph.add_node(ExecNode {
                df_nodes: vec![],
                dependence_counter: AtomicUsize::new(usize::MAX),
                #[cfg(feature = "gpu")]
                locality: -1,
            });
            for n in df_nodes.iter() {
                node_map.insert(*n, ex_node);
            }
            execution_graph[ex_node].df_nodes = df_nodes;
        }
    }
    add_execution_depedences(graph, execution_graph, node_map)
}

pub fn partition_components<TNode, TEdge>(
    graph: &Dag<TNode, TEdge>,
    execution_graph: &mut Dag<ExecNode, ()>,
) -> Result<()> {
    // First sort the DAG in a schedulable order
    let ts = daggy::petgraph::algo::toposort(graph, None)
        .map_err(|_| SchedulerError::CyclicDependence)?;
    let tsmap: HashMap<&NodeIndex, usize> = ts.iter().enumerate().map(|(c, x)| (x, c)).collect();
    let mut vis = graph.visit_map();
    // Traverse the DAG and build a graph of the connected components
    for nidx in ts.iter() {
        if !vis.is_visited(nidx) {
            vis.visit(*nidx);
            let mut df_nodes = vec![*nidx];
            let mut stack = vec![*nidx];
            // DFS from the entry point undirected to gather all nodes
            // in the component
            while let Some(n) = stack.pop() {
                for neighbor in graph.graph().neighbors_undirected(n) {
                    if !vis.is_visited(&neighbor) {
                        df_nodes.push(neighbor);
                        stack.push(neighbor);
                        vis.visit(neighbor);
                    }
                }
            }
            // Apply toposort to component nodes
            df_nodes.sort_by_key(|x| tsmap.get(x).unwrap());
            execution_graph
                .add_node(ExecNode {
                    df_nodes,
                    dependence_counter: AtomicUsize::new(0),
                    #[cfg(feature = "gpu")]
                    locality: -1,
                })
                .index();
        }
    }
    // As this partition is made by coalescing all connected
    // components within the DFG, there are no dependences (edges) to
    // add to the execution graph.
    Ok(())
}
