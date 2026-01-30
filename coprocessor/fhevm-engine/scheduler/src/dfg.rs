pub mod scheduler;
pub mod types;

use std::{
    collections::{HashMap, HashSet},
    sync::atomic::AtomicUsize,
};
use tracing::{error, warn};

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
    pub intermediate_handles: Vec<Handle>,
    pub transaction_id: Handle,
    pub is_uncomputable: bool,
    pub component_id: usize,
}

/// Check if a node is needed by traversing its outgoing edges iteratively.
/// Uses an explicit stack to avoid stack overflow on deep computation graphs.
fn is_needed(graph: &Dag<(bool, usize), OpEdge>, index: usize) -> bool {
    let mut stack = vec![index];
    let mut visited = graph.visit_map();

    while let Some(current_index) = stack.pop() {
        let node_index = NodeIndex::new(current_index);

        // Skip if already visited to avoid cycles and redundant work
        if visited.is_visited(&node_index) {
            continue;
        }
        visited.visit(node_index);

        let node = match graph.node_weight(node_index) {
            Some(n) => n,
            None => {
                error!(target: "scheduler", "Missing node for index in DFG finalization");
                continue;
            }
        };

        // If this node is marked as needed, the original node is needed
        if node.0 {
            return true;
        }

        // Push all outgoing neighbors onto the stack for exploration
        for edge in graph.edges_directed(node_index, Direction::Outgoing) {
            let target = edge.target();
            if !visited.is_visited(&target) {
                stack.push(target.index());
            }
        }
    }

    false
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
            if !op.is_allowed {
                self.intermediate_handles.push(op.output_handle.clone());
            }
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
    pub produced: HashMap<Handle, Vec<(NodeIndex, Handle)>>,
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
                self.produced
                    .entry(r.clone())
                    .and_modify(|p| p.push((producer, tx.transaction_id.clone())))
                    .or_insert(vec![(producer, tx.transaction_id.clone())]);
            }
        }
        // Identify all dependence pairs (producer, consumer)
        let mut dependence_pairs = vec![];
        for (consumer, tx) in self.graph.node_references() {
            for i in tx.inputs.keys() {
                if let Some(producer) = self.produced.get(i) {
                    // If this handle is produced within this same transaction
                    if let Some((prod_idx, _)) =
                        producer.iter().find(|(_, tid)| *tid == tx.transaction_id)
                    {
                        if *prod_idx == consumer {
                            warn!(target: "scheduler", { },
			       "Self-dependence on node");
                        } else {
                            dependence_pairs.push((*prod_idx, consumer));
                        }
                    } else if producer.len() > 1 {
                        error!(target: "scheduler", { output_handle = ?hex::encode(i.clone()),
							  count =  ?producer.len() },
				   "Handle collision for computation output");
                    } else if producer.is_empty() {
                        error!(target: "scheduler", { output_handle = ?hex::encode(i.clone()) },
				   "Missing producer for handle");
                    } else {
                        dependence_pairs.push((producer[0].0, consumer));
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
                if self.graph.add_edge(*producer, *consumer, ()).is_err() {
                    let prod = self
                        .graph
                        .node_weight(*producer)
                        .ok_or(SchedulerError::DataflowGraphError)?;
                    let cons = self
                        .graph
                        .node_weight(*consumer)
                        .ok_or(SchedulerError::DataflowGraphError)?;
                    error!(target: "scheduler", { producer_id = ?hex::encode(prod.transaction_id.clone()), consumer_id = ?hex::encode(cons.transaction_id.clone()) },
		       "Dependence cycle when adding dependence - initial cycle detection failed");
                    return Err(SchedulerError::CyclicDependence.into());
                }
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
        result: Result<TaskResult>,
        edges: &Dag<(), ComponentEdge>,
    ) -> Result<()> {
        if let Some(producer) = self.produced.get(handle).cloned() {
            if producer.is_empty() {
                error!(target: "scheduler", { output_handle = ?hex::encode(handle) },
		       "Missing producer for handle");
            } else {
                let mut prod_idx = producer[0].0;
                if let Ok(ref result) = result {
                    if let Some((pid, _)) = producer
                        .iter()
                        .find(|(_, tid)| *tid == result.transaction_id)
                    {
                        prod_idx = *pid;
                    }
                }
                let mut save_result = true;
                if let Ok(ref result) = result {
                    save_result = result.is_allowed;
                    // Traverse immediate dependents and add this result as an input
                    for edge in edges.edges_directed(prod_idx, Direction::Outgoing) {
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
                    self.set_uncomputable(prod_idx, edges)?;
                }
                // Finally add the output (either error or compressed
                // ciphertext) to the graph's outputs
                if save_result {
                    let producer_tx = self
                        .graph
                        .node_weight_mut(prod_idx)
                        .ok_or(SchedulerError::DataflowGraphError)?;
                    self.results.push(DFGTxResult {
                        transaction_id: producer_tx.transaction_id.clone(),
                        handle: handle.to_vec(),
                        compressed_ct: result.and_then(|rok| {
                            rok.compressed_ct
                                .map(|cct| (cct.0, cct.1))
                                .ok_or_else(|| {
                                    error!(target: "scheduler", {handle = ?hex::encode(handle) }, "Missing compressed ciphertext in task result");
                                    SchedulerError::SchedulerError.into()
                                })
                        }),
                    });
                }
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
        let mut stack = vec![tx_node_index];

        while let Some(current_index) = stack.pop() {
            let tx_node = self
                .graph
                .node_weight_mut(current_index)
                .ok_or(SchedulerError::DataflowGraphError)?;

            // Skip if already marked as uncomputable (handles diamond dependencies)
            if tx_node.is_uncomputable {
                continue;
            }
            tx_node.is_uncomputable = true;

            // Add error results for all operations in this transaction
            for (_idx, op) in tx_node.graph.graph.node_references() {
                self.results.push(DFGTxResult {
                    transaction_id: tx_node.transaction_id.clone(),
                    handle: op.result_handle.to_vec(),
                    compressed_ct: Err(SchedulerError::MissingInputs.into()),
                });
            }

            // Push all dependent transactions onto the stack
            for edge in edges.edges_directed(current_index, Direction::Outgoing) {
                stack.push(edge.target());
            }
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
    // edges in the execution graph. We use a HashSet to track added
    // edges for O(1) deduplication.
    let mut added_edges: HashSet<(NodeIndex, NodeIndex)> = HashSet::new();
    for edge in graph.edge_references() {
        let (xsrc, xdst) = (
            node_map
                .get(&edge.source())
                .ok_or(SchedulerError::DataflowGraphError)?,
            node_map
                .get(&edge.target())
                .ok_or(SchedulerError::DataflowGraphError)?,
        );
        if xsrc != xdst && added_edges.insert((*xsrc, *xdst)) {
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
            // All nodes should be in the toposort map; use MAX as fallback for corrupt state
            df_nodes.sort_by_key(|x| {
                tsmap.get(x).copied().unwrap_or_else(|| {
                    error!(target: "scheduler", {index = ?x.index()}, "Node missing from topological sort");
                    usize::MAX
                })
            });
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

#[cfg(test)]
mod tests {
    use super::*;
    use daggy::petgraph::graph::node_index;

    // ============================================================
    // Helper functions
    // ============================================================

    /// Create a simple DFGOp for testing
    fn make_test_op(output: &[u8], inputs: Vec<DFGTaskInput>, is_allowed: bool) -> DFGOp {
        DFGOp {
            output_handle: output.to_vec(),
            fhe_op: SupportedFheOperations::FheTrivialEncrypt,
            inputs,
            is_allowed,
        }
    }

    /// Create a dependence input reference
    fn dep(handle: &[u8]) -> DFGTaskInput {
        DFGTaskInput::Dependence(handle.to_vec())
    }

    /// Helper to build a simple test DAG with given edges.
    /// Panics if edge addition fails (e.g., due to invalid indices or cycles),
    /// which helps catch test setup bugs early.
    fn build_test_dag(node_count: usize, edges: &[(usize, usize)]) -> Dag<usize, ()> {
        let mut graph: Dag<usize, ()> = Dag::default();
        for i in 0..node_count {
            graph.add_node(i);
        }
        for (src, dst) in edges {
            graph
                .add_edge(node_index(*src), node_index(*dst), ())
                .expect(
                    "test DAG edge addition should succeed - check for cycles or invalid indices",
                );
        }
        graph
    }

    // ============================================================
    // Partition Strategy Tests - partition_preserving_parallelism
    // ============================================================

    #[test]
    fn test_partition_parallelism_empty_graph() {
        let graph: Dag<usize, ()> = Dag::default();
        let mut exec_graph: Dag<ExecNode, ()> = Dag::default();

        let result = partition_preserving_parallelism(&graph, &mut exec_graph);
        assert!(result.is_ok(), "partitioning empty graph should succeed");
        assert_eq!(exec_graph.node_count(), 0);
    }

    #[test]
    fn test_partition_parallelism_single_node() {
        let graph = build_test_dag(1, &[]);
        let mut exec_graph: Dag<ExecNode, ()> = Dag::default();

        let result = partition_preserving_parallelism(&graph, &mut exec_graph);
        assert!(
            result.is_ok(),
            "partitioning single node graph should succeed"
        );
        assert_eq!(exec_graph.node_count(), 1);
        assert_eq!(exec_graph[node_index(0)].df_nodes.len(), 1);
    }

    #[test]
    fn test_partition_parallelism_linear_chain() {
        // A -> B -> C should become a single partition (no siblings)
        let graph = build_test_dag(3, &[(0, 1), (1, 2)]);
        let mut exec_graph: Dag<ExecNode, ()> = Dag::default();

        let result = partition_preserving_parallelism(&graph, &mut exec_graph);
        assert!(result.is_ok(), "partitioning linear chain should succeed");
        // Linear chain with no forks coalesces into 1 partition
        assert_eq!(exec_graph.node_count(), 1);
        assert_eq!(exec_graph[node_index(0)].df_nodes.len(), 3);
    }

    #[test]
    fn test_partition_parallelism_fork() {
        // A -> B and A -> C (fork at A)
        // A has 2 children, so B and C should be in separate partitions
        let graph = build_test_dag(3, &[(0, 1), (0, 2)]);
        let mut exec_graph: Dag<ExecNode, ()> = Dag::default();

        let result = partition_preserving_parallelism(&graph, &mut exec_graph);
        assert!(result.is_ok(), "partitioning fork graph should succeed");
        // A has multiple children, so it stays alone
        // B and C are separate (they each have a sibling)
        assert_eq!(exec_graph.node_count(), 3);
    }

    #[test]
    fn test_partition_parallelism_join() {
        // A -> C and B -> C (join at C)
        // C has 2 parents (siblings B and A from C's perspective)
        let graph = build_test_dag(3, &[(0, 2), (1, 2)]);
        let mut exec_graph: Dag<ExecNode, ()> = Dag::default();

        let result = partition_preserving_parallelism(&graph, &mut exec_graph);
        assert!(result.is_ok(), "partitioning join graph should succeed");
        // A, B, C all separate since C has multiple incoming edges
        assert_eq!(exec_graph.node_count(), 3);
    }

    #[test]
    fn test_partition_parallelism_diamond() {
        // Diamond: A -> B -> D, A -> C -> D
        //     A
        //    / \
        //   B   C
        //    \ /
        //     D
        let graph = build_test_dag(4, &[(0, 1), (0, 2), (1, 3), (2, 3)]);
        let mut exec_graph: Dag<ExecNode, ()> = Dag::default();

        let result = partition_preserving_parallelism(&graph, &mut exec_graph);
        assert!(result.is_ok(), "partitioning diamond graph should succeed");
        // A forks (2 children), D joins (2 parents)
        // All 4 nodes should be separate partitions
        assert_eq!(exec_graph.node_count(), 4);
    }

    #[test]
    fn test_partition_parallelism_chain_with_branch() {
        // A -> B -> C -> D, but also B -> E
        //   A -> B -> C -> D
        //        |
        //        v
        //        E
        let graph = build_test_dag(5, &[(0, 1), (1, 2), (2, 3), (1, 4)]);
        let mut exec_graph: Dag<ExecNode, ()> = Dag::default();

        let result = partition_preserving_parallelism(&graph, &mut exec_graph);
        assert!(
            result.is_ok(),
            "partitioning chain with branch should succeed"
        );
        // A has 1 outgoing edge, B has 1 incoming edge -> A-B chain together
        // B has 2 outgoing edges, so chaining stops at B
        // C has 1 incoming edge (from B) and 1 outgoing edge -> C-D chain together
        // E has 1 incoming edge (from B) and 0 outgoing edges -> E alone
        // Result: {A,B}, {C,D}, {E} = 3 partitions
        assert_eq!(exec_graph.node_count(), 3);
    }

    // ============================================================
    // Partition Strategy Tests - partition_components
    // ============================================================

    #[test]
    fn test_partition_components_empty_graph() {
        let graph: Dag<usize, ()> = Dag::default();
        let mut exec_graph: Dag<ExecNode, ()> = Dag::default();

        let result = partition_components(&graph, &mut exec_graph);
        assert!(
            result.is_ok(),
            "partition_components on empty graph should succeed"
        );
        assert_eq!(exec_graph.node_count(), 0);
    }

    #[test]
    fn test_partition_components_single_node() {
        let graph = build_test_dag(1, &[]);
        let mut exec_graph: Dag<ExecNode, ()> = Dag::default();

        let result = partition_components(&graph, &mut exec_graph);
        assert!(
            result.is_ok(),
            "partition_components on single node should succeed"
        );
        assert_eq!(exec_graph.node_count(), 1);
        assert_eq!(exec_graph[node_index(0)].df_nodes.len(), 1);
    }

    #[test]
    fn test_partition_components_connected_graph() {
        // All connected nodes should be in single component
        let graph = build_test_dag(4, &[(0, 1), (0, 2), (1, 3), (2, 3)]);
        let mut exec_graph: Dag<ExecNode, ()> = Dag::default();

        let result = partition_components(&graph, &mut exec_graph);
        assert!(
            result.is_ok(),
            "partition_components on connected graph should succeed"
        );
        // All nodes connected -> single component
        assert_eq!(exec_graph.node_count(), 1);
        assert_eq!(exec_graph[node_index(0)].df_nodes.len(), 4);
    }

    #[test]
    fn test_partition_components_disconnected_graph() {
        // Two disconnected components: {0, 1} and {2, 3}
        let graph = build_test_dag(4, &[(0, 1), (2, 3)]);
        let mut exec_graph: Dag<ExecNode, ()> = Dag::default();

        let result = partition_components(&graph, &mut exec_graph);
        assert!(
            result.is_ok(),
            "partition_components on disconnected graph should succeed"
        );
        // Two separate components
        assert_eq!(exec_graph.node_count(), 2);
    }

    #[test]
    fn test_partition_components_preserves_topo_order() {
        // Chain A -> B -> C -> D
        let graph = build_test_dag(4, &[(0, 1), (1, 2), (2, 3)]);
        let mut exec_graph: Dag<ExecNode, ()> = Dag::default();

        let result = partition_components(&graph, &mut exec_graph);
        assert!(
            result.is_ok(),
            "partition_components on chain should succeed"
        );
        assert_eq!(exec_graph.node_count(), 1);
        // Nodes should be in topological order within the component
        let df_nodes = &exec_graph[node_index(0)].df_nodes;
        assert_eq!(df_nodes.len(), 4);
        // Verify order: indices should be 0, 1, 2, 3
        for (i, n) in df_nodes.iter().enumerate() {
            assert_eq!(n.index(), i);
        }
    }

    // ============================================================
    // Graph Construction Tests - build_component_nodes
    // ============================================================

    #[test]
    fn test_build_empty_operations() {
        let ops: Vec<DFGOp> = vec![];
        let tx_id = vec![1, 2, 3];

        let result = build_component_nodes(ops, &tx_id);
        assert!(
            result.is_ok(),
            "build_component_nodes with empty ops should succeed"
        );
        let (components, unneeded) = result.unwrap();
        assert!(
            components.is_empty(),
            "no components expected for empty ops"
        );
        assert!(unneeded.is_empty(), "no unneeded handles for empty ops");
    }

    #[test]
    fn test_build_single_allowed_operation() {
        let ops = vec![make_test_op(b"out1", vec![], true)];
        let tx_id = vec![1, 2, 3];

        let result = build_component_nodes(ops, &tx_id);
        assert!(
            result.is_ok(),
            "build_component_nodes with single allowed op should succeed"
        );
        let (components, unneeded) = result.unwrap();
        assert_eq!(components.len(), 1);
        assert!(
            unneeded.is_empty(),
            "allowed ops should not produce unneeded handles"
        );
        assert_eq!(components[0].results.len(), 1);
    }

    #[test]
    fn test_build_single_unallowed_operation() {
        // Single op that's not allowed -> gets pruned
        let ops = vec![make_test_op(b"out1", vec![], false)];
        let tx_id = vec![1, 2, 3];

        let result = build_component_nodes(ops, &tx_id);
        assert!(
            result.is_ok(),
            "build_component_nodes with unallowed op should succeed"
        );
        let (components, unneeded) = result.unwrap();
        // Unallowed with no dependents gets pruned
        assert!(components.is_empty());
        assert_eq!(unneeded.len(), 1);
    }

    #[test]
    fn test_build_with_internal_dependence() {
        // A produces output, B depends on A's output
        let ops = vec![
            make_test_op(b"out_a", vec![], false),
            make_test_op(b"out_b", vec![dep(b"out_a")], true),
        ];
        let tx_id = vec![1, 2, 3];

        let result = build_component_nodes(ops, &tx_id);
        assert!(
            result.is_ok(),
            "build with internal dependence should succeed"
        );
        let (components, unneeded) = result.unwrap();
        // Both should be kept since B is allowed and depends on A
        assert!(
            !components.is_empty(),
            "components should not be empty when allowed op exists"
        );
        assert!(
            unneeded.is_empty(),
            "no unneeded handles when dependence chain leads to allowed op"
        );
    }

    #[test]
    fn test_build_with_external_dependence() {
        // Op depends on handle not produced internally
        let ops = vec![make_test_op(b"out1", vec![dep(b"external")], true)];
        let tx_id = vec![1, 2, 3];

        let result = build_component_nodes(ops, &tx_id);
        assert!(
            result.is_ok(),
            "build with external dependence should succeed"
        );
        let (components, _) = result.unwrap();
        assert_eq!(components.len(), 1);
        // External dependence should appear in inputs map
        assert!(
            components[0].inputs.contains_key(b"external".as_slice()),
            "external dependence should be recorded in inputs map"
        );
    }

    #[test]
    fn test_build_independent_operations() {
        // Two unrelated allowed operations
        let ops = vec![
            make_test_op(b"out_a", vec![], true),
            make_test_op(b"out_b", vec![], true),
        ];
        let tx_id = vec![1, 2, 3];

        let result = build_component_nodes(ops, &tx_id);
        assert!(result.is_ok(), "build with independent ops should succeed");
        let (components, unneeded) = result.unwrap();
        // Both are independent and allowed
        // With partition_preserving_parallelism, they stay separate
        assert_eq!(components.len(), 2);
        assert!(
            unneeded.is_empty(),
            "allowed ops should not produce unneeded handles"
        );
    }

    #[test]
    fn test_build_cyclic_dependence_error() {
        // Create cycle: A -> B, B -> A
        let ops = vec![
            make_test_op(b"out_a", vec![dep(b"out_b")], true),
            make_test_op(b"out_b", vec![dep(b"out_a")], true),
        ];
        let tx_id = vec![1, 2, 3];

        let result = build_component_nodes(ops, &tx_id);
        assert!(
            result.is_err(),
            "cyclic dependence should cause build to fail"
        );
    }

    // ============================================================
    // Finalization/Pruning Tests
    // ============================================================

    #[test]
    fn test_finalize_all_allowed() {
        // All nodes marked as allowed -> none pruned
        let mut graph: Dag<(bool, usize), OpEdge> = Dag::default();
        graph.add_node((true, 0));
        graph.add_node((true, 1));
        graph
            .add_edge(node_index(0), node_index(1), 0)
            .expect("edge addition should succeed");

        let pruned = finalize(&mut graph);
        assert!(pruned.is_empty(), "all allowed nodes should not be pruned");
        assert_eq!(graph.node_count(), 2);
    }

    #[test]
    fn test_finalize_prune_orphan() {
        // Node not allowed and no path to allowed node -> pruned
        let mut graph: Dag<(bool, usize), OpEdge> = Dag::default();
        graph.add_node((false, 0)); // Not allowed, no dependents
        graph.add_node((true, 1)); // Allowed, no connection to 0

        let pruned = finalize(&mut graph);
        assert_eq!(pruned.len(), 1);
        assert_eq!(pruned[0], 0);
    }

    #[test]
    fn test_finalize_keep_chain_to_allowed() {
        // A (not allowed) -> B (not allowed) -> C (allowed)
        // A and B should be kept since they lead to allowed C
        let mut graph: Dag<(bool, usize), OpEdge> = Dag::default();
        graph.add_node((false, 0));
        graph.add_node((false, 1));
        graph.add_node((true, 2));
        graph
            .add_edge(node_index(0), node_index(1), 0)
            .expect("edge addition should succeed");
        graph
            .add_edge(node_index(1), node_index(2), 0)
            .expect("edge addition should succeed");

        let pruned = finalize(&mut graph);
        assert!(
            pruned.is_empty(),
            "nodes leading to allowed node should not be pruned"
        );
        // All nodes should still exist (marked as needed)
        assert_eq!(graph.node_count(), 3);
    }

    #[test]
    fn test_finalize_prune_dead_branch() {
        // A -> B (allowed)
        // A -> C (not allowed, no dependents)
        // C should be pruned
        let mut graph: Dag<(bool, usize), OpEdge> = Dag::default();
        graph.add_node((false, 0)); // A - kept (leads to B)
        graph.add_node((true, 1)); // B - allowed
        graph.add_node((false, 2)); // C - not allowed, dead end
        graph
            .add_edge(node_index(0), node_index(1), 0)
            .expect("edge addition should succeed");
        graph
            .add_edge(node_index(0), node_index(2), 0)
            .expect("edge addition should succeed");

        let pruned = finalize(&mut graph);
        assert_eq!(pruned.len(), 1);
        // Note: `pruned` contains the *original* node indices before removal.
        // After finalize() removes nodes, graph indices may shift, but the
        // returned Vec records which original indices were pruned.
        assert_eq!(pruned[0], 2);
    }

    #[test]
    fn test_is_needed_allowed_node() {
        let mut graph: Dag<(bool, usize), OpEdge> = Dag::default();
        graph.add_node((true, 0)); // Allowed node

        assert!(
            is_needed(&graph, 0),
            "allowed node should be marked as needed"
        );
    }

    #[test]
    fn test_is_needed_has_allowed_descendant() {
        let mut graph: Dag<(bool, usize), OpEdge> = Dag::default();
        graph.add_node((false, 0));
        graph.add_node((true, 1));
        graph
            .add_edge(node_index(0), node_index(1), 0)
            .expect("edge addition should succeed");

        // Node 0 is needed because it leads to allowed node 1
        assert!(
            is_needed(&graph, 0),
            "node with allowed descendant should be needed"
        );
    }

    #[test]
    fn test_is_needed_no_allowed_descendant() {
        let mut graph: Dag<(bool, usize), OpEdge> = Dag::default();
        graph.add_node((false, 0));
        graph.add_node((false, 1));
        graph
            .add_edge(node_index(0), node_index(1), 0)
            .expect("edge addition should succeed");

        // Neither node is allowed, no path to allowed
        assert!(
            !is_needed(&graph, 0),
            "node without allowed descendant should not be needed"
        );
        assert!(
            !is_needed(&graph, 1),
            "node without allowed descendant should not be needed"
        );
    }

    // ============================================================
    // Cycle Detection Tests - DFComponentGraph
    // ============================================================

    #[test]
    fn test_component_graph_no_cycle() {
        // Simple DAG: A -> B -> C
        let mut nodes = vec![
            ComponentNode {
                transaction_id: b"tx_a".to_vec(),
                results: vec![b"out_a".to_vec()],
                inputs: HashMap::new(),
                ..Default::default()
            },
            ComponentNode {
                transaction_id: b"tx_b".to_vec(),
                results: vec![b"out_b".to_vec()],
                inputs: [(b"out_a".to_vec(), None)].into_iter().collect(),
                ..Default::default()
            },
            ComponentNode {
                transaction_id: b"tx_c".to_vec(),
                results: vec![b"out_c".to_vec()],
                inputs: [(b"out_b".to_vec(), None)].into_iter().collect(),
                ..Default::default()
            },
        ];

        let mut graph = DFComponentGraph::default();
        let result = graph.build(&mut nodes);
        assert!(
            result.is_ok(),
            "building acyclic component graph should succeed"
        );
        assert_eq!(graph.graph.node_count(), 3);
    }

    #[test]
    fn test_component_graph_two_node_cycle() {
        // Cycle: A depends on B, B depends on A
        let mut nodes = vec![
            ComponentNode {
                transaction_id: b"tx_a".to_vec(),
                results: vec![b"out_a".to_vec()],
                inputs: [(b"out_b".to_vec(), None)].into_iter().collect(),
                ..Default::default()
            },
            ComponentNode {
                transaction_id: b"tx_b".to_vec(),
                results: vec![b"out_b".to_vec()],
                inputs: [(b"out_a".to_vec(), None)].into_iter().collect(),
                ..Default::default()
            },
        ];

        let mut graph = DFComponentGraph::default();
        let result = graph.build(&mut nodes);
        assert!(result.is_err(), "two-node cycle should be detected");
    }

    #[test]
    fn test_component_graph_three_node_cycle() {
        // Cycle: A -> B -> C -> A
        let mut nodes = vec![
            ComponentNode {
                transaction_id: b"tx_a".to_vec(),
                results: vec![b"out_a".to_vec()],
                inputs: [(b"out_c".to_vec(), None)].into_iter().collect(),
                ..Default::default()
            },
            ComponentNode {
                transaction_id: b"tx_b".to_vec(),
                results: vec![b"out_b".to_vec()],
                inputs: [(b"out_a".to_vec(), None)].into_iter().collect(),
                ..Default::default()
            },
            ComponentNode {
                transaction_id: b"tx_c".to_vec(),
                results: vec![b"out_c".to_vec()],
                inputs: [(b"out_b".to_vec(), None)].into_iter().collect(),
                ..Default::default()
            },
        ];

        let mut graph = DFComponentGraph::default();
        let result = graph.build(&mut nodes);
        assert!(result.is_err(), "three-node cycle should be detected");
    }

    #[test]
    fn test_component_graph_records_cycle_errors() {
        // When cycle detected, affected transactions should have error results
        let mut nodes = vec![
            ComponentNode {
                transaction_id: b"tx_a".to_vec(),
                results: vec![b"out_a".to_vec()],
                inputs: [(b"out_b".to_vec(), None)].into_iter().collect(),
                graph: {
                    let mut g = DFGraph::default();
                    g.add_node(b"out_a".to_vec(), 0, vec![], true);
                    g
                },
                ..Default::default()
            },
            ComponentNode {
                transaction_id: b"tx_b".to_vec(),
                results: vec![b"out_b".to_vec()],
                inputs: [(b"out_a".to_vec(), None)].into_iter().collect(),
                graph: {
                    let mut g = DFGraph::default();
                    g.add_node(b"out_b".to_vec(), 0, vec![], true);
                    g
                },
                ..Default::default()
            },
        ];

        let mut graph = DFComponentGraph::default();
        let _ = graph.build(&mut nodes);
        // Results should contain errors for cycle nodes
        assert!(
            !graph.results.is_empty(),
            "cycle detection should produce error results"
        );
        for result in &graph.results {
            assert!(
                result.compressed_ct.is_err(),
                "cycle nodes should have error results"
            );
        }
    }

    #[test]
    fn test_component_graph_external_dependence() {
        // Node depends on handle not produced by any node
        let mut nodes = vec![ComponentNode {
            transaction_id: b"tx_a".to_vec(),
            results: vec![b"out_a".to_vec()],
            inputs: [(b"external_input".to_vec(), None)].into_iter().collect(),
            ..Default::default()
        }];

        let mut graph = DFComponentGraph::default();
        let result = graph.build(&mut nodes);
        assert!(
            result.is_ok(),
            "building graph with external dependence should succeed"
        );
        // External dependence should be in needed_map
        assert!(
            graph.needed_map.contains_key(b"external_input".as_slice()),
            "external dependence should be recorded in needed_map"
        );
    }

    // ============================================================
    // Output Propagation Tests - set_uncomputable
    // ============================================================

    #[test]
    fn test_set_uncomputable_single_node() {
        let mut nodes = vec![ComponentNode {
            transaction_id: b"tx_a".to_vec(),
            results: vec![b"out_a".to_vec()],
            graph: {
                let mut g = DFGraph::default();
                g.add_node(b"out_a".to_vec(), 0, vec![], true);
                g
            },
            ..Default::default()
        }];

        let mut graph = DFComponentGraph::default();
        graph.build(&mut nodes).unwrap();

        let edges = graph.graph.map(|_, _| (), |_, e| *e);
        graph.set_uncomputable(node_index(0), &edges).unwrap();

        assert!(
            graph.graph[node_index(0)].is_uncomputable,
            "node should be marked uncomputable"
        );
        assert!(
            !graph.results.is_empty(),
            "uncomputable node should produce error results"
        );
    }

    #[test]
    fn test_set_uncomputable_cascades() {
        // A -> B -> C, mark A uncomputable, B and C should follow
        //
        // Implementation note: This test intentionally relies on the current behavior
        // of build() which uses pop() to process nodes, resulting in reverse insertion
        // order. We define nodes in reverse (C, B, A) so that after build() processes
        // them, they end up as A=0, B=1, C=2. If build()'s iteration order changes,
        // this test may need adjustment - the assertions verify cascade behavior
        // regardless of specific indices.
        let mut nodes = vec![
            ComponentNode {
                transaction_id: b"tx_c".to_vec(),
                results: vec![b"out_c".to_vec()],
                inputs: [(b"out_b".to_vec(), None)].into_iter().collect(),
                graph: {
                    let mut g = DFGraph::default();
                    g.add_node(b"out_c".to_vec(), 0, vec![], true);
                    g
                },
                ..Default::default()
            },
            ComponentNode {
                transaction_id: b"tx_b".to_vec(),
                results: vec![b"out_b".to_vec()],
                inputs: [(b"out_a".to_vec(), None)].into_iter().collect(),
                graph: {
                    let mut g = DFGraph::default();
                    g.add_node(b"out_b".to_vec(), 0, vec![], true);
                    g
                },
                ..Default::default()
            },
            ComponentNode {
                transaction_id: b"tx_a".to_vec(),
                results: vec![b"out_a".to_vec()],
                inputs: HashMap::new(),
                graph: {
                    let mut g = DFGraph::default();
                    g.add_node(b"out_a".to_vec(), 0, vec![], true);
                    g
                },
                ..Default::default()
            },
        ];

        let mut graph = DFComponentGraph::default();
        graph.build(&mut nodes).unwrap();

        // After build: node 0 = tx_a, node 1 = tx_b, node 2 = tx_c
        // Dependencies: tx_a -> tx_b -> tx_c
        let edges = graph.graph.map(|_, _| (), |_, e| *e);

        // Mark tx_a (node 0) as uncomputable
        graph.set_uncomputable(node_index(0), &edges).unwrap();

        // All three should be marked uncomputable since B depends on A, C depends on B
        assert!(
            graph.graph[node_index(0)].is_uncomputable,
            "node A should be marked uncomputable"
        );
        assert!(
            graph.graph[node_index(1)].is_uncomputable,
            "node B should cascade to uncomputable (depends on A)"
        );
        assert!(
            graph.graph[node_index(2)].is_uncomputable,
            "node C should cascade to uncomputable (depends on B)"
        );
    }

    #[test]
    fn test_set_uncomputable_idempotent() {
        // Tests that `set_uncomputable` is idempotent - calling it multiple times
        // on the same node should not produce duplicate error results.
        //
        // The function checks `is_uncomputable` at the start and returns early
        // if already marked, preventing duplicate entries in the results vector.
        // This is important for correctness when error cascades could potentially
        // visit the same node multiple times through different dependence paths.
        let mut nodes = vec![ComponentNode {
            transaction_id: b"tx_a".to_vec(),
            results: vec![b"out_a".to_vec()],
            graph: {
                let mut g = DFGraph::default();
                g.add_node(b"out_a".to_vec(), 0, vec![], true);
                g
            },
            ..Default::default()
        }];

        let mut graph = DFComponentGraph::default();
        graph.build(&mut nodes).unwrap();

        let edges = graph.graph.map(|_, _| (), |_, e| *e);

        // Call twice - should not error or add duplicate results
        graph.set_uncomputable(node_index(0), &edges).unwrap();
        let results_after_first = graph.results.len();

        graph.set_uncomputable(node_index(0), &edges).unwrap();
        let results_after_second = graph.results.len();

        // Second call should not add more results (early return)
        assert_eq!(results_after_first, results_after_second);
    }

    // ============================================================
    // Task Dependence Tests - add_execution_depedences
    // ============================================================

    #[test]
    fn test_execution_deps_counter_initialization() {
        // Build a simple graph: 0 -> 1 -> 2
        let graph = build_test_dag(3, &[(0, 1), (1, 2)]);
        let mut exec_graph: Dag<ExecNode, ()> = Dag::default();

        partition_preserving_parallelism(&graph, &mut exec_graph).unwrap();

        // In a linear chain, there's 1 exec node with counter 0
        // (all nodes coalesce into 1)
        assert_eq!(exec_graph.node_count(), 1);
        assert_eq!(
            exec_graph[node_index(0)]
                .dependence_counter
                .load(std::sync::atomic::Ordering::SeqCst),
            0
        );
    }

    #[test]
    fn test_execution_deps_with_dependencies() {
        // Fork: 0 -> 1, 0 -> 2 (separate partitions with deps)
        let graph = build_test_dag(3, &[(0, 1), (0, 2)]);
        let mut exec_graph: Dag<ExecNode, ()> = Dag::default();

        partition_preserving_parallelism(&graph, &mut exec_graph).unwrap();

        // Should have 3 exec nodes (fork creates separate partitions)
        assert_eq!(exec_graph.node_count(), 3);

        // Find the partition containing node 0 (should have 0 deps)
        let mut found_root = false;
        for idx in 0..exec_graph.node_count() {
            let node = &exec_graph[node_index(idx)];
            let deps = node
                .dependence_counter
                .load(std::sync::atomic::Ordering::SeqCst);
            if node.df_nodes.contains(&node_index(0)) {
                assert_eq!(deps, 0, "root partition should have 0 dependencies");
                found_root = true;
            }
        }
        assert!(found_root, "should find partition containing root node");
    }

    #[test]
    fn test_execution_deps_no_duplicate_edges() {
        // Graph where same partition pair could have multiple deps
        // 0 -> 2, 1 -> 2 with 0,1 in same partition would create 2 edges to 2
        // But with fork, 0 and 1 would be separate anyway
        // Test with join: 0 -> 2, 1 -> 2
        let graph = build_test_dag(3, &[(0, 2), (1, 2)]);
        let mut exec_graph: Dag<ExecNode, ()> = Dag::default();

        partition_preserving_parallelism(&graph, &mut exec_graph).unwrap();

        // Node 2 should have exactly 2 incoming deps
        for idx in 0..exec_graph.node_count() {
            let node = &exec_graph[node_index(idx)];
            if node.df_nodes.contains(&node_index(2)) {
                let deps = node
                    .dependence_counter
                    .load(std::sync::atomic::Ordering::SeqCst);
                assert_eq!(
                    deps, 2,
                    "join node should have exactly 2 incoming dependencies"
                );
            }
        }
    }

    // ============================================================
    // ComponentNode Tests
    // ============================================================

    #[test]
    fn test_component_node_build() {
        let ops = vec![
            make_test_op(b"out_a", vec![], true),
            make_test_op(b"out_b", vec![dep(b"out_a")], true),
        ];
        let tx_id = b"tx1".to_vec();

        let mut node = ComponentNode::default();
        let result = node.build(ops, &tx_id, 0);
        assert!(
            result.is_ok(),
            "ComponentNode::build should succeed for valid ops"
        );

        assert_eq!(node.transaction_id, tx_id);
        assert_eq!(node.component_id, 0);
        assert!(
            !node.is_uncomputable,
            "newly built node should not be marked uncomputable"
        );
        assert_eq!(node.results.len(), 2);
    }

    #[test]
    fn test_component_node_add_input() {
        let mut node = ComponentNode::default();
        // Pre-populate the inputs map with keys (as build() would do)
        node.inputs.insert(b"handle1".to_vec(), None);
        node.inputs.insert(b"handle2".to_vec(), None);

        // Verify initial state - both inputs are None
        assert!(
            node.inputs.get(b"handle1".as_slice()).unwrap().is_none(),
            "handle1 should initially be None"
        );
        assert!(
            node.inputs.get(b"handle2".as_slice()).unwrap().is_none(),
            "handle2 should initially be None"
        );

        // Test add_input: it uses entry().and_modify() so it only updates existing keys.
        // Use Compressed variant since it doesn't require TFHE keys.
        let test_input = DFGTxInput::Compressed(((1i16, vec![1, 2, 3]), true));
        node.add_input(b"handle1", test_input.clone());

        // Verify add_input updated the existing key
        assert!(
            node.inputs.get(b"handle1".as_slice()).unwrap().is_some(),
            "handle1 should be Some after add_input"
        );
        assert!(
            node.inputs.get(b"handle2".as_slice()).unwrap().is_none(),
            "handle2 should still be None (not modified)"
        );

        // Verify add_input does NOT insert new keys (and_modify behavior)
        node.add_input(b"nonexistent", test_input);
        assert!(
            !node.inputs.contains_key(b"nonexistent".as_slice()),
            "add_input should not insert new keys"
        );
        assert_eq!(node.inputs.len(), 2);
    }

    // ============================================================
    // DFComponentGraph::add_input Tests
    // ============================================================

    #[test]
    fn test_add_input_propagates_to_single_consumer() {
        // Create a node that needs an external input
        let mut nodes = vec![ComponentNode {
            transaction_id: b"tx_a".to_vec(),
            results: vec![b"out_a".to_vec()],
            inputs: [(b"external".to_vec(), None)].into_iter().collect(),
            ..Default::default()
        }];

        let mut graph = DFComponentGraph::default();
        graph.build(&mut nodes).unwrap();

        // Verify external input is in needed_map
        assert!(graph.needed_map.contains_key(b"external".as_slice()));

        // Add input
        let input = DFGTxInput::Compressed(((1i16, vec![1, 2, 3]), true));
        graph.add_input(b"external", &input).unwrap();

        // Verify input was propagated to the node
        let node = &graph.graph[node_index(0)];
        assert!(
            node.inputs.get(b"external".as_slice()).unwrap().is_some(),
            "input should be propagated to consumer"
        );
    }

    #[test]
    fn test_add_input_propagates_to_multiple_consumers() {
        // Create two nodes that need the same external input
        let mut nodes = vec![
            ComponentNode {
                transaction_id: b"tx_a".to_vec(),
                results: vec![b"out_a".to_vec()],
                inputs: [(b"shared_input".to_vec(), None)].into_iter().collect(),
                ..Default::default()
            },
            ComponentNode {
                transaction_id: b"tx_b".to_vec(),
                results: vec![b"out_b".to_vec()],
                inputs: [(b"shared_input".to_vec(), None)].into_iter().collect(),
                ..Default::default()
            },
        ];

        let mut graph = DFComponentGraph::default();
        graph.build(&mut nodes).unwrap();

        // Verify external input has two consumers in needed_map
        let consumers = graph.needed_map.get(b"shared_input".as_slice()).unwrap();
        assert_eq!(consumers.len(), 2);

        // Add input
        let input = DFGTxInput::Compressed(((1i16, vec![1, 2, 3]), true));
        graph.add_input(b"shared_input", &input).unwrap();

        // Verify input was propagated to both nodes
        for idx in 0..graph.graph.node_count() {
            let node = &graph.graph[node_index(idx)];
            assert!(
                node.inputs
                    .get(b"shared_input".as_slice())
                    .unwrap()
                    .is_some(),
                "input should be propagated to all consumers"
            );
        }
    }

    // ============================================================
    // DFComponentGraph::get_results Tests
    // ============================================================

    #[test]
    fn test_get_results_returns_all_results() {
        let mut nodes = vec![ComponentNode {
            transaction_id: b"tx_a".to_vec(),
            results: vec![b"out_a".to_vec()],
            graph: {
                let mut g = DFGraph::default();
                g.add_node(b"out_a".to_vec(), 0, vec![], true);
                g
            },
            ..Default::default()
        }];

        let mut graph = DFComponentGraph::default();
        graph.build(&mut nodes).unwrap();

        // Mark as uncomputable to populate results
        let edges = graph.graph.map(|_, _| (), |_, e| *e);
        graph.set_uncomputable(node_index(0), &edges).unwrap();

        let results = graph.get_results();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].handle, b"out_a".to_vec());
    }

    // ============================================================
    // DFComponentGraph::get_intermediate_handles Tests
    // ============================================================

    #[test]
    fn test_get_intermediate_handles_returns_non_allowed() {
        let ops = vec![
            make_test_op(b"intermediate", vec![], false), // not allowed
            make_test_op(b"allowed", vec![dep(b"intermediate")], true), // allowed
        ];
        let tx_id = b"tx1".to_vec();

        let (components, _) = build_component_nodes(ops, &tx_id).unwrap();
        let mut nodes = components;

        let mut graph = DFComponentGraph::default();
        graph.build(&mut nodes).unwrap();

        let intermediate = graph.get_intermediate_handles();
        // Should contain the intermediate handle
        assert!(
            intermediate
                .iter()
                .any(|(h, _)| h == b"intermediate".as_slice()),
            "should return intermediate handles"
        );
        // Should not contain allowed handle
        assert!(
            !intermediate.iter().any(|(h, _)| h == b"allowed".as_slice()),
            "should not return allowed handles"
        );
    }

    #[test]
    fn test_get_intermediate_handles_skips_uncomputable() {
        let mut nodes = vec![ComponentNode {
            transaction_id: b"tx_a".to_vec(),
            results: vec![b"out_a".to_vec()],
            intermediate_handles: vec![b"intermediate".to_vec()],
            is_uncomputable: true, // Mark as uncomputable
            ..Default::default()
        }];

        let mut graph = DFComponentGraph::default();
        graph.build(&mut nodes).unwrap();
        // Manually set uncomputable since build resets it
        graph.graph[node_index(0)].is_uncomputable = true;

        let intermediate = graph.get_intermediate_handles();
        assert!(
            intermediate.is_empty(),
            "should skip intermediate handles from uncomputable transactions"
        );
    }

    #[test]
    fn test_get_intermediate_handles_empty_when_all_allowed() {
        let ops = vec![
            make_test_op(b"out_a", vec![], true),
            make_test_op(b"out_b", vec![], true),
        ];
        let tx_id = b"tx1".to_vec();

        let (components, _) = build_component_nodes(ops, &tx_id).unwrap();
        let mut nodes = components;

        let mut graph = DFComponentGraph::default();
        graph.build(&mut nodes).unwrap();

        let intermediate = graph.get_intermediate_handles();
        assert!(
            intermediate.is_empty(),
            "should return empty when all handles are allowed"
        );
    }

    // ============================================================
    // OpNode::check_ready_inputs Tests
    // ============================================================

    #[test]
    fn test_check_ready_inputs_all_values_ready() {
        use fhevm_engine_common::types::SupportedFheCiphertexts;

        let mut graph = DFGraph::default();
        // Create node with Value inputs (simulating already-resolved inputs)
        // Use Scalar variant which doesn't require TFHE server key
        let ct = SupportedFheCiphertexts::Scalar(vec![1, 2, 3, 4]);
        graph.add_node(b"out".to_vec(), 0, vec![DFGTaskInput::Value(ct)], true);

        let mut ct_map: HashMap<Handle, Option<DFGTxInput>> = HashMap::new();
        let node = graph.graph.node_weight_mut(node_index(0)).unwrap();
        let result = node.check_ready_inputs(&mut ct_map);
        assert!(result, "should return true when all inputs are Values");
    }

    #[test]
    fn test_check_ready_inputs_empty_inputs_returns_true() {
        let mut graph = DFGraph::default();
        graph.add_node(b"out".to_vec(), 0, vec![], true);

        let mut ct_map: HashMap<Handle, Option<DFGTxInput>> = HashMap::new();
        let node = graph.graph.node_weight_mut(node_index(0)).unwrap();
        let result = node.check_ready_inputs(&mut ct_map);
        assert!(result, "should return true for empty inputs");
    }

    #[test]
    fn test_check_ready_inputs_missing_dependence_returns_false() {
        let mut graph = DFGraph::default();
        graph.add_node(
            b"out".to_vec(),
            0,
            vec![DFGTaskInput::Dependence(b"missing".to_vec())],
            true,
        );

        let mut ct_map: HashMap<Handle, Option<DFGTxInput>> = HashMap::new();
        // Don't add the missing handle to ct_map

        let node = graph.graph.node_weight_mut(node_index(0)).unwrap();
        let result = node.check_ready_inputs(&mut ct_map);
        assert!(
            !result,
            "should return false when dependence handle is missing from ct_map"
        );
    }

    #[test]
    fn test_check_ready_inputs_converts_dependence_to_value() {
        use fhevm_engine_common::types::SupportedFheCiphertexts;

        let mut graph = DFGraph::default();
        graph.add_node(
            b"out".to_vec(),
            0,
            vec![DFGTaskInput::Dependence(b"dep_handle".to_vec())],
            true,
        );

        // Use Scalar variant which doesn't require TFHE server key
        let ct = SupportedFheCiphertexts::Scalar(vec![1, 2, 3, 4]);
        let mut ct_map: HashMap<Handle, Option<DFGTxInput>> = HashMap::new();
        ct_map.insert(
            b"dep_handle".to_vec(),
            Some(DFGTxInput::Value((ct.clone(), true))),
        );

        let node = graph.graph.node_weight_mut(node_index(0)).unwrap();
        let result = node.check_ready_inputs(&mut ct_map);
        assert!(result, "should return true when dependence is available");

        // Verify the input was converted from Dependence to Value
        assert!(
            matches!(node.inputs[0], DFGTaskInput::Value(_)),
            "dependence should be converted to value"
        );
    }

    #[test]
    fn test_check_ready_inputs_compressed_returns_false() {
        let mut graph = DFGraph::default();
        // Using Compressed variant directly as input
        graph.add_node(
            b"out".to_vec(),
            0,
            vec![DFGTaskInput::Compressed((1i16, vec![1, 2, 3]))],
            true,
        );

        let mut ct_map: HashMap<Handle, Option<DFGTxInput>> = HashMap::new();
        let node = graph.graph.node_weight_mut(node_index(0)).unwrap();
        let result = node.check_ready_inputs(&mut ct_map);
        assert!(
            !result,
            "should return false for Compressed inputs (not yet decompressed)"
        );
    }

    // ============================================================
    // DFComponentGraph::add_output Tests
    // ============================================================

    /// Helper to create a TaskResult for testing
    fn make_task_result(
        ct_data: Vec<u8>,
        compressed_ct: Option<(i16, Vec<u8>)>,
        is_allowed: bool,
        transaction_id: &[u8],
    ) -> TaskResult {
        use fhevm_engine_common::types::SupportedFheCiphertexts;
        TaskResult {
            ct: SupportedFheCiphertexts::Scalar(ct_data),
            compressed_ct,
            is_allowed,
            transaction_id: transaction_id.to_vec(),
        }
    }

    #[test]
    fn test_add_output_saves_allowed_result() {
        let mut nodes = vec![ComponentNode {
            transaction_id: b"tx_a".to_vec(),
            results: vec![b"out_a".to_vec()],
            inputs: HashMap::new(),
            graph: {
                let mut g = DFGraph::default();
                g.add_node(b"out_a".to_vec(), 0, vec![], true);
                g
            },
            ..Default::default()
        }];

        let mut graph = DFComponentGraph::default();
        graph.build(&mut nodes).unwrap();

        let edges = graph.graph.map(|_, _| (), |_, e| *e);
        let task_result = make_task_result(
            vec![1, 2, 3],
            Some((1i16, vec![4, 5, 6])),
            true, // allowed
            b"tx_a",
        );

        graph.add_output(b"out_a", Ok(task_result), &edges).unwrap();

        // Result should be saved
        assert_eq!(graph.results.len(), 1);
        assert_eq!(graph.results[0].handle, b"out_a".to_vec());
        assert!(graph.results[0].compressed_ct.is_ok());
    }

    #[test]
    fn test_add_output_skips_unallowed_result() {
        let mut nodes = vec![ComponentNode {
            transaction_id: b"tx_a".to_vec(),
            results: vec![b"out_a".to_vec()],
            inputs: HashMap::new(),
            graph: {
                let mut g = DFGraph::default();
                g.add_node(b"out_a".to_vec(), 0, vec![], false); // not allowed
                g
            },
            ..Default::default()
        }];

        let mut graph = DFComponentGraph::default();
        graph.build(&mut nodes).unwrap();

        let edges = graph.graph.map(|_, _| (), |_, e| *e);
        let task_result = make_task_result(
            vec![1, 2, 3],
            Some((1i16, vec![4, 5, 6])),
            false, // not allowed
            b"tx_a",
        );

        let initial_results = graph.results.len();
        graph.add_output(b"out_a", Ok(task_result), &edges).unwrap();

        // Result should NOT be saved (not allowed)
        assert_eq!(graph.results.len(), initial_results);
    }

    #[test]
    fn test_add_output_error_triggers_uncomputable() {
        let mut nodes = vec![ComponentNode {
            transaction_id: b"tx_a".to_vec(),
            results: vec![b"out_a".to_vec()],
            inputs: HashMap::new(),
            graph: {
                let mut g = DFGraph::default();
                g.add_node(b"out_a".to_vec(), 0, vec![], true);
                g
            },
            ..Default::default()
        }];

        let mut graph = DFComponentGraph::default();
        graph.build(&mut nodes).unwrap();

        let edges = graph.graph.map(|_, _| (), |_, e| *e);

        // Add error result
        let error_result: Result<TaskResult> = Err(SchedulerError::MissingInputs.into());
        graph.add_output(b"out_a", error_result, &edges).unwrap();

        // Node should be marked uncomputable
        assert!(graph.graph[node_index(0)].is_uncomputable);
        // Results should contain error
        assert!(!graph.results.is_empty());
    }

    #[test]
    fn test_add_output_propagates_to_dependents() {
        // A -> B: A produces out_a, B needs it as input
        let mut nodes = vec![
            ComponentNode {
                transaction_id: b"tx_b".to_vec(),
                results: vec![b"out_b".to_vec()],
                inputs: [(b"out_a".to_vec(), None)].into_iter().collect(),
                graph: {
                    let mut g = DFGraph::default();
                    g.add_node(b"out_b".to_vec(), 0, vec![], true);
                    g
                },
                ..Default::default()
            },
            ComponentNode {
                transaction_id: b"tx_a".to_vec(),
                results: vec![b"out_a".to_vec()],
                inputs: HashMap::new(),
                graph: {
                    let mut g = DFGraph::default();
                    g.add_node(b"out_a".to_vec(), 0, vec![], true);
                    g
                },
                ..Default::default()
            },
        ];

        let mut graph = DFComponentGraph::default();
        graph.build(&mut nodes).unwrap();

        let edges = graph.graph.map(|_, _| (), |_, e| *e);
        let task_result =
            make_task_result(vec![1, 2, 3], Some((1i16, vec![4, 5, 6])), true, b"tx_a");

        graph.add_output(b"out_a", Ok(task_result), &edges).unwrap();

        // Find tx_b and verify its input was populated
        for idx in 0..graph.graph.node_count() {
            let node = &graph.graph[node_index(idx)];
            if node.transaction_id == b"tx_b".to_vec() {
                let input = node.inputs.get(b"out_a".as_slice());
                assert!(
                    input.is_some() && input.unwrap().is_some(),
                    "dependent transaction should have input populated"
                );
            }
        }
    }
}
