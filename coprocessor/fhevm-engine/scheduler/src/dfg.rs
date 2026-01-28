//! # Dataflow Graph Module
//!
//! This module implements the core dataflow graph (DFG) infrastructure for scheduling FHE operations.
//! It provides data structures and algorithms for representing computation graphs, partitioning them
//! for parallel execution, and managing dependences between operations.
//!
//! ## Key Components
//!
//! - [`DFGraph`]: Low-level dataflow graph representing individual FHE operations
//! - [`DFComponentGraph`]: Higher-level graph of components of transactions and their dependences
//! - [`ComponentNode`]: Represents a component containing a subgraph of operations
//! - [`ExecNode`]: Execution node in the partitioned graph for scheduling
//!
//! ## Partitioning Strategies
//!
//! The module provides two partitioning strategies:
//! - [`partition_preserving_parallelism`]: Maximizes parallel execution by keeping independent
//!   operations in separate partitions, but groups dependence chains for locality
//! - [`partition_components`]: Groups connected components together for better data locality

pub mod scheduler;
pub mod types;

use std::{collections::HashMap, sync::atomic::AtomicUsize};
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

/// Represents an execution node in the partitioned dataflow graph.
///
/// An `ExecNode` groups together multiple dataflow graph nodes that should be executed
/// as a single unit. This grouping is determined by the partitioning strategy used
/// (either maximizing parallelism or maximizing locality).
///
/// # Fields
///
/// * `df_nodes` - Vector of node indices from the original dataflow graph that belong
///   to this execution partition
/// * `dependence_counter` - Atomic counter tracking the number of unsatisfied dependences;
///   when this reaches zero, the node is ready for execution
/// * `locality` (GPU only) - GPU device affinity hint for scheduling on multi-GPU systems
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

/// Represents a single FHE operation in the dataflow graph.
///
/// A `DFGOp` captures all the information needed to execute an FHE operation,
/// including its output handle, the operation type, input dependences, and
/// whether the result should be persisted (allowed).
///
/// # Fields
///
/// * `output_handle` - The unique handle identifying the output ciphertext of this operation
/// * `fhe_op` - The type of FHE operation to perform
/// * `inputs` - Vector of inputs to the operation, which can be values, compressed
///   ciphertexts, or dependences on other operations
/// * `is_allowed` - Whether this operation's result is "allowed"
///   (i.e., should be persisted and made available for subsequent
///   transactions - and outside the scope of the scheduler, whether
///   to squash noise to make ready for decryption)
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

/// Type alias for edges between component nodes.
pub type ComponentEdge = ();

/// Represents a transaction component containing a subgraph of FHE operations.
///
/// A `ComponentNode` encapsulates all operations belonging to a single transaction
/// or a connected component within a transaction. It maintains the internal dataflow
/// graph, tracks required inputs, and collects the results of computations.
///
/// # Fields
///
/// * `graph` - The inner dataflow graph containing the operations for this component
/// * `ops` - Vector of FHE operations (used during construction)
/// * `inputs` - Map of required input handles to their values; keys are handles that
///   this component needs from external sources, values are `None` until provided
/// * `results` - Handles of all outputs produced by this component
/// * `intermediate_handles` - Handles of intermediate results (non-allowed) that should
///   be cleaned up after execution
/// * `transaction_id` - Unique identifier for the transaction this component belongs to
/// * `is_uncomputable` - Flag indicating whether this component cannot be computed
///   (e.g., due to missing dependences or dependence cycles)
/// * `component_id` - Index of this component within its parent transaction
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

/// Checks if a node is needed by traversing its outgoing edges iteratively.
///
/// A node is considered "needed" if it is marked as allowed or if any of its
/// transitive successors in the graph is marked as needed. This function uses
/// an explicit stack-based traversal instead of recursion to avoid stack overflow
/// on deep computation graphs.
///
/// # Arguments
///
/// * `graph` - The dataflow graph where nodes are tuples of (is_needed, operation_index)
/// * `index` - The index of the node to check
///
/// # Returns
///
/// `true` if the node or any of its transitive successors is marked as needed,
/// `false` otherwise.
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

/// Finalizes the dataflow graph by pruning nodes that are not needed.
///
/// This function performs dead code elimination on the dataflow graph. It traverses
/// the graph in reverse order (since allowed nodes are typically later in the graph)
/// and marks nodes as needed if they contribute to any allowed output. Nodes that
/// don't contribute to any allowed output are pruned from the graph.
///
/// # Arguments
///
/// * `graph` - Mutable reference to the dataflow graph to finalize. Nodes are tuples
///   of (is_needed_flag, operation_index).
///
/// # Returns
///
/// A vector of indices of the nodes that were removed (unneeded nodes). These indices
/// correspond to the original operation indices before pruning.
///
/// # Note
///
/// If a node cannot be found during finalization (which shouldn't happen in a valid
/// graph), an error is logged and an empty vector is returned to avoid partial pruning.
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

/// Result type for building component nodes, containing the components and unneeded handles.
type ComponentNodes = Result<(Vec<ComponentNode>, Vec<(Handle, Handle)>)>;

/// Builds component nodes from a list of FHE operations.
///
/// This function takes a list of FHE operations and constructs a partitioned execution
/// graph. It performs the following steps:
///
/// 1. Sorts operations by output handle for deterministic processing
/// - required for consensus, particularly under re-randomisation
/// 2. Builds a dependence graph based on operation inputs
/// 3. Adds edges for data dependences between operations
/// 4. Prunes unneeded branches (operations that don't contribute to allowed outputs)
/// 5. Partitions the graph using the parallelism-preserving strategy
/// 6. Creates `ComponentNode` instances for each partition
///
/// # Arguments
///
/// * `operations` - Vector of FHE operations to process
/// * `transaction_id` - The unique identifier for the transaction these operations belong to
///
/// # Returns
///
/// On success, returns a tuple containing:
/// * A vector of `ComponentNode` instances representing the partitioned operations
/// * A vector of (handle, transaction_id) tuples for handles that were pruned as unneeded
///
/// # Errors
///
/// Returns an error if:
/// * There is a cyclic dependence between operations
/// * The dataflow graph construction fails due to inconsistent state
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
    /// Builds the component node from a list of FHE operations.
    ///
    /// This method initializes the component node by:
    /// 1. Setting up the transaction and component identifiers
    /// 2. Building the internal dataflow graph from the operations
    /// 3. Identifying external inputs (dependences not satisfied within this component)
    /// 4. Collecting result handles and intermediate handles
    /// 5. Adding dependence edges between operations
    ///
    /// # Arguments
    ///
    /// * `operations` - Vector of FHE operations to include in this component
    /// * `transaction_id` - The unique identifier for the parent transaction
    /// * `component_id` - The index of this component within the transaction
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or an error if:
    /// * There is a cyclic dependence between operations
    /// * The graph structure is inconsistent
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

    /// Adds an input value to the component node.
    ///
    /// This method provides an external input value for a handle that this component
    /// depends on. If the handle exists in the inputs map, its value is updated.
    ///
    /// # Arguments
    ///
    /// * `handle` - The handle identifying the input
    /// * `cct` - The transaction input value (either a decompressed value or compressed ciphertext)
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

/// High-level dataflow graph representing transactions and their dependences.
///
/// `DFComponentGraph` manages the execution of multiple transaction components,
/// tracking which handles are needed as inputs, which handles are produced as outputs,
/// and the dependences between transactions. It detects cyclic dependences and
/// coordinates result propagation between dependent transactions.
///
/// # Fields
///
/// * `graph` - The DAG of component nodes representing transactions
/// * `needed_map` - Maps input handles to the component nodes that require them;
///   these are external inputs that must be provided before execution
/// * `produced` - Maps output handles to the (node_index, transaction_id) pairs
///   that produce them
/// * `results` - Collection of computed results ready to be returned
#[derive(Default)]
pub struct DFComponentGraph {
    pub graph: Dag<ComponentNode, ComponentEdge>,
    pub needed_map: HashMap<Handle, Vec<NodeIndex>>,
    pub produced: HashMap<Handle, Vec<(NodeIndex, Handle)>>,
    pub results: Vec<DFGTxResult>,
}
impl DFComponentGraph {
    /// Builds the component graph from a vector of component nodes.
    ///
    /// This method constructs the inter-transaction dependence graph by:
    /// 1. Adding all component nodes to the graph
    /// 2. Recording which handles are produced by which nodes
    /// 3. Identifying dependences between transactions based on input/output handles
    /// 4. Detecting and reporting cyclic dependences using Tarjan's SCC algorithm
    /// 5. Adding dependence edges to the graph
    ///
    /// # Arguments
    ///
    /// * `nodes` - Mutable reference to a vector of component nodes to add (consumed)
    ///
    /// # Returns
    ///
    /// `Ok(())` on success. If cyclic dependences are detected, the affected nodes
    /// are marked as uncomputable and their results are set to error before returning
    /// `Err(SchedulerError::CyclicDependence)`.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * Cyclic dependences are detected between transactions
    /// * Graph operations fail due to inconsistent state
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

    /// Adds an external input to all component nodes that require it.
    ///
    /// This method distributes an input value to all transactions that declared
    /// a dependence on the given handle. It's used to provide initial inputs
    /// from the database or from completed transactions.
    ///
    /// # Arguments
    ///
    /// * `handle` - The handle identifying the input
    /// * `input` - The transaction input value to distribute
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or an error if a node index is invalid.
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

    /// Records an output result and propagates it to dependent transactions.
    ///
    /// This method handles the completion of an operation by:
    /// 1. Finding the producer of the given handle
    /// 2. If successful, propagating the result to dependent transactions as an input
    /// 3. If failed, marking dependent transactions as uncomputable
    /// 4. Storing allowed results for later retrieval
    ///
    /// # Arguments
    ///
    /// * `handle` - The handle identifying the output
    /// * `result` - The computation result (success with `TaskResult` or error)
    /// * `edges` - Reference to the dependence edges graph for traversing dependents
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or an error if:
    /// * The producer node cannot be found
    /// * A compressed ciphertext is missing when expected
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
        }
        Ok(())
    }

    /// Marks a node and all its dependents as uncomputable.
    ///
    /// When a transaction fails (e.g., due to missing inputs or errors), this method
    /// recursively marks it and all downstream dependent transactions as uncomputable.
    /// Error results are generated for all operations in the affected transactions.
    ///
    /// # Arguments
    ///
    /// * `tx_node_index` - The index of the transaction node to mark as uncomputable
    /// * `edges` - Reference to the dependence edges graph for traversing dependents
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or an error if a node index is invalid.
    fn set_uncomputable(
        &mut self,
        tx_node_index: NodeIndex,
        edges: &Dag<(), ComponentEdge>,
    ) -> Result<()> {
        let tx_node = self
            .graph
            .node_weight_mut(tx_node_index)
            .ok_or(SchedulerError::DataflowGraphError)?;
        if tx_node.is_uncomputable {
            return Ok(());
        }
        tx_node.is_uncomputable = true;
        for (_idx, op) in tx_node.graph.graph.node_references() {
            self.results.push(DFGTxResult {
                transaction_id: tx_node.transaction_id.clone(),
                handle: op.result_handle.to_vec(),
                compressed_ct: Err(SchedulerError::MissingInputs.into()),
            });
        }
        for edge in edges.edges_directed(tx_node_index, Direction::Outgoing) {
            let dependent_tx_index = edge.target();
            self.set_uncomputable(dependent_tx_index, edges)?;
        }
        Ok(())
    }

    /// Retrieves and consumes all computed results.
    ///
    /// This method returns all results that have been accumulated in the graph
    /// and clears the internal results vector. It should be called after all
    /// scheduling is complete to retrieve the final outputs.
    ///
    /// # Returns
    ///
    /// A vector of `DFGTxResult` containing all computed results (both successful
    /// and failed operations).
    pub fn get_results(&mut self) -> Vec<DFGTxResult> {
        std::mem::take(&mut self.results)
    }

    /// Retrieves and consumes all intermediate handles for cleanup.
    ///
    /// Intermediate handles are outputs of operations that are not "allowed"
    /// (i.e., they were only needed as inputs to other operations within the
    /// transaction). These handles can be cleaned up after execution since
    /// they don't need to be persisted.
    ///
    /// # Returns
    ///
    /// A vector of (handle, transaction_id) tuples identifying intermediate
    /// results that can be cleaned up.
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

/// Represents the result of a dataflow graph operation.
///
/// # Fields
///
/// * `handle` - The handle identifying the output of the operation
/// * `result` - The computation result: `Ok(Some((type, bytes)))` for allowed outputs,
///   `Ok(None)` for intermediate results, or `Err` for failures
/// * `work_index` - The index of this operation in the work queue
pub struct DFGResult {
    pub handle: Handle,
    pub result: Result<Option<(i16, Vec<u8>)>>,
    pub work_index: usize,
}
/// Type alias for operation edges (stores the input position as a u8).
pub type OpEdge = u8;

/// Represents a single operation node in the dataflow graph.
///
/// An `OpNode` contains all the information needed to execute a single FHE
/// operation, including the operation type, inputs, and metadata about the
/// result.
///
/// # Fields
///
/// * `opcode` - The FHE operation code (e.g., add, multiply, compare)
/// * `result_handle` - The handle that will identify the output of this operation
/// * `inputs` - Vector of inputs to the operation (values, compressed, or dependences)
/// * `locality` (GPU only) - GPU device affinity hint for multi-GPU scheduling
/// * `is_allowed` - Whether this operation's result should be persisted
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
    /// Checks if all inputs are ready and resolves dependences.
    ///
    /// This method verifies that all inputs to this operation are available as
    /// concrete values. For dependence inputs, it looks up the value in the
    /// provided ciphertext map and converts the input to a value if found.
    ///
    /// # Arguments
    ///
    /// * `ct_map` - Map of handles to their computed values
    ///
    /// # Returns
    ///
    /// `true` if all inputs are ready (either already values or successfully
    /// resolved from the map), `false` if any input is still missing.
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

/// Low-level dataflow graph for FHE operations within a single component.
///
/// `DFGraph` is a directed acyclic graph where nodes represent FHE operations
/// and edges represent data dependences. The edge weight indicates which input
/// position of the destination node receives the output from the source node.
///
/// # Fields
///
/// * `graph` - The underlying DAG structure from the `daggy` crate
#[derive(Default, Debug)]
pub struct DFGraph {
    pub graph: Dag<OpNode, OpEdge>,
}
impl DFGraph {
    /// Adds a new operation node to the dataflow graph.
    ///
    /// Creates a new node representing an FHE operation and adds it to the graph.
    /// The node is not connected to any other nodes initially; dependences must
    /// be added separately using [`add_dependence`](Self::add_dependence).
    ///
    /// # Arguments
    ///
    /// * `rh` - The result handle that will identify this operation's output
    /// * `opcode` - The FHE operation code
    /// * `inputs` - Vector of inputs to the operation
    /// * `is_allowed` - Whether this operation's result should be persisted
    ///
    /// # Returns
    ///
    /// The `NodeIndex` of the newly created node in the graph.
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

    /// Adds a data dependence edge between two nodes.
    ///
    /// Creates an edge from the source node to the destination node, indicating
    /// that the output of the source operation is used as an input to the destination
    /// operation. The edge weight specifies which input position receives the value.
    ///
    /// # Arguments
    ///
    /// * `source` - Index of the producer node
    /// * `destination` - Index of the consumer node
    /// * `consumer_input` - The input position in the consumer that receives the value
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or `Err(SchedulerError::CyclicDependence)` if adding
    /// this edge would create a cycle in the graph.
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

/// Adds execution dependence edges to the partitioned execution graph.
///
/// After partitioning the original dataflow graph, this function creates edges
/// in the execution graph based on the original data dependences. It translates
/// edges from the original graph to the execution graph using the provided node
/// mapping, and initializes the dependence counter for each execution node.
///
/// # Type Parameters
///
/// * `TNode` - The node type in the original graph
/// * `TEdge` - The edge type in the original graph
///
/// # Arguments
///
/// * `graph` - Reference to the original dataflow graph
/// * `execution_graph` - Mutable reference to the execution graph being built
/// * `node_map` - Mapping from original graph node indices to execution graph node indices
///
/// # Returns
///
/// `Ok(())` on success, or an error if the node mapping is inconsistent.
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

/// Partitions a dataflow graph while preserving opportunities for parallel execution.
///
/// This partitioning strategy creates execution nodes that group only sequential
/// chains of operations together, preserving parallelism at branch points. An
/// operation is grouped with its successor only if it has exactly one outgoing
/// edge and the successor has exactly one incoming edge.
///
/// The algorithm:
/// 1. Performs a topological sort of the input graph
/// 2. Traverses nodes in topological order
/// 3. Groups nodes into chains where each node has exactly one successor/predecessor
/// 4. Creates execution nodes for each chain
/// 5. Adds dependence edges between execution nodes
///
/// # Type Parameters
///
/// * `TNode` - The node type in the original graph
/// * `TEdge` - The edge type in the original graph
///
/// # Arguments
///
/// * `graph` - Reference to the dataflow graph to partition
/// * `execution_graph` - Mutable reference to the execution graph being built
///
/// # Returns
///
/// `Ok(())` on success, or `Err(SchedulerError::CyclicDependence)` if the
/// graph contains cycles.
///
/// # Example
///
/// Given a graph like:
/// ```text
///     A
///    / \
///   B   C
///    \ /
///     D
/// ```
///
/// This strategy creates 4 execution nodes (A, B, C, D) rather than grouping
/// them, preserving the opportunity to execute B and C in parallel.
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

/// Partitions a dataflow graph into connected components for maximum locality.
///
/// This partitioning strategy groups all nodes in each connected component
/// together into a single execution node. This maximizes data locality at the
/// expense of parallelism, as nodes that could potentially execute in parallel
/// are grouped together.
///
/// The algorithm:
/// 1. Performs a topological sort of the input graph
/// 2. Traverses nodes using undirected DFS to find connected components
/// 3. Creates one execution node per connected component
/// 4. Sorts nodes within each component by topological order
///
/// # Type Parameters
///
/// * `TNode` - The node type in the original graph
/// * `TEdge` - The edge type in the original graph
///
/// # Arguments
///
/// * `graph` - Reference to the dataflow graph to partition
/// * `execution_graph` - Mutable reference to the execution graph being built
///
/// # Returns
///
/// `Ok(())` on success, or `Err(SchedulerError::CyclicDependence)` if the
/// graph contains cycles.
///
/// # Note
///
/// Since all connected components are independent, the resulting execution
/// graph has no edges between nodes.
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
