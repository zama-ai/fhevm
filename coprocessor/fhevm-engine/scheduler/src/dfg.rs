pub mod scheduler;
pub mod types;

use std::{
    collections::{HashMap, HashSet},
    sync::atomic::AtomicUsize,
};
use tracing::{error, info, warn};

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

pub fn finalize(graph: &mut Dag<(bool, usize), OpEdge>) -> Vec<usize> {
    // Mark the complete ancestor closure of every allowed result in one
    // reverse traversal. The former implementation ran a descendant DFS from
    // every node, which was quadratic for long dependence chains.
    let mut needed = graph.visit_map();
    let mut stack = graph
        .node_references()
        .filter_map(|(index, node)| node.0.then_some(index))
        .collect::<Vec<_>>();
    while let Some(index) = stack.pop() {
        if needed.is_visited(&index) {
            continue;
        }
        needed.visit(index);
        for edge in graph.edges_directed(index, Incoming) {
            let source = edge.source();
            if !needed.is_visited(&source) {
                stack.push(source);
            }
        }
    }

    // Prune graph of all unneeded nodes and edges
    let mut unneeded_nodes = Vec::new();
    for index in 0..graph.node_count() {
        let node_index = NodeIndex::new(index);
        if needed.is_visited(&node_index) {
            if let Some(node) = graph.node_weight_mut(node_index) {
                node.0 = true;
            }
        } else {
            unneeded_nodes.push(index);
        }
    }
    unneeded_nodes.sort();
    // Remove unneeded nodes and their edges
    for index in unneeded_nodes.iter().rev() {
        let node_index = NodeIndex::new(*index);
        graph.remove_node(node_index);
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
                DFGTaskInput::Value(_)
                | DFGTaskInput::SharedValue(_)
                | DFGTaskInput::Compressed(_) => {}
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
                    DFGTaskInput::Value(_)
                    | DFGTaskInput::SharedValue(_)
                    | DFGTaskInput::Compressed(_) => {}
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
    /// The graph was deliberately left unexecuted because a cross-block
    /// boundary input is still pending. Workers use this distinct outcome to
    /// requeue the lane fairly instead of treating it as ordinary no-progress.
    pub deferred: bool,
    deferred_dependences: Vec<(NodeIndex, NodeIndex, Handle)>,
    /// Handles that are produced by more than one ComponentNode in this
    /// graph. Key is the output handle; value is the list of non-canonical
    /// producer `transaction_id`s — i.e. every `transaction_id` other than
    /// the one the scheduler picks as canonical for that handle. Two
    /// transactions in the same block can deterministically derive the
    /// same output handle (handle = keccak256 of op, operands, ACL,
    /// chain_id, block context; all constant within a block), and the
    /// legacy `computations` PK `(tenant_id, output_handle,
    /// transaction_id)` lets both rows coexist. `get_results()` uses this
    /// map to synthesize a `DFGTxResult` for any producer `transaction_id`
    /// missing from the partition output, so every producer row reaches
    /// `is_completed = true` in the downstream DB UPDATE independent of
    /// how partitioning distributes the producers.
    aliased_tids: HashMap<Handle, Vec<Handle>>,
}
impl DFComponentGraph {
    pub fn deferred() -> Self {
        Self {
            deferred: true,
            ..Self::default()
        }
    }

    pub fn build(&mut self, nodes: &mut Vec<ComponentNode>) -> Result<()> {
        while let Some(tx) = nodes.pop() {
            self.graph.add_node(tx);
        }
        // Gather handles produced within the graph. When the same
        // handle is produced by multiple ComponentNodes (two transactions
        // in the same block deriving the same output handle — see
        // `aliased_tids` on DFComponentGraph), we sort producers by
        // `transaction_id` lexicographically so the canonical choice is
        // deterministic and reproducible across coprocessors regardless
        // of Vec insertion order: the lowest `transaction_id` is
        // canonical; the rest go into `aliased_tids` for
        // completion-broadcast at `get_results()` time.
        for (producer, tx) in self.graph.node_references() {
            for r in tx.results.iter() {
                self.produced
                    .entry(r.clone())
                    .or_default()
                    .push((producer, tx.transaction_id.clone()));
            }
        }
        for (handle, producers) in self.produced.iter_mut() {
            if producers.len() <= 1 {
                continue;
            }
            producers.sort_by(|a, b| a.1.cmp(&b.1));
            let aliased: Vec<Handle> = producers
                .iter()
                .skip(1)
                .map(|(_, tid)| tid.clone())
                .collect();
            let canonical_tid = producers[0].1.clone();
            info!(
                target: "scheduler",
                output_handle = %hex::encode(handle),
                canonical_transaction_id = %hex::encode(&canonical_tid),
                aliased_transaction_ids = ?aliased
                    .iter()
                    .map(hex::encode)
                    .collect::<Vec<_>>(),
                "Multi-producer handle detected; completion will broadcast \
                 to all producer tids"
            );
            self.aliased_tids.insert(handle.clone(), aliased);
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
                        // Multi-producer handle with a cross-transaction
                        // consumer: route the consumer from the canonical
                        // producer (producer[0]). Same-block selected
                        // producers are never materialized through DB fetch,
                        // so the canonical in-memory value defines the
                        // consumer input and the persisted canonical bytes.
                        self.deferred_dependences
                            .push((producer[0].0, consumer, i.clone()));
                        self.needed_map
                            .entry(i.clone())
                            .and_modify(|uses| uses.push(consumer))
                            .or_insert(vec![consumer]);
                    } else if producer.is_empty() {
                        error!(target: "scheduler", { output_handle = ?hex::encode(i.clone()) },
				   "Missing producer for handle");
                    } else {
                        // Cross-transaction dependence: defer until
                        // after DB fetch. If the handle is found in
                        // DB, we use the fetched value and skip the
                        // dependence edge.
                        self.deferred_dependences
                            .push((producer[0].0, consumer, i.clone()));
                        self.needed_map
                            .entry(i.clone())
                            .and_modify(|uses| uses.push(consumer))
                            .or_insert(vec![consumer]);
                    }
                } else {
                    self.needed_map
                        .entry(i.clone())
                        .and_modify(|uses| uses.push(consumer))
                        .or_insert(vec![consumer]);
                }
            }
        }

        // Same-transaction dependences are always acyclic (they
        // derive from the transaction's internal DAG). Add them
        // directly; cycle detection runs once in
        // resolve_dependences() over the full edge set.
        for (producer, consumer) in dependence_pairs.iter() {
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
		       "Unexpected cycle in same-transaction dependence");
                return Err(SchedulerError::CyclicDependence.into());
            }
        }
        Ok(())
    }

    // Resolve deferred cross-transaction dependences after DB fetch.
    // Dependences whose handle was successfully fetched are dropped
    // (the consumer already has the data). Remaining dependences are
    // added as graph edges after cycle detection.
    pub fn resolve_dependences(&mut self, fetched_handles: &HashSet<Handle>) -> Result<()> {
        let remaining: Vec<(NodeIndex, NodeIndex)> = self
            .deferred_dependences
            .drain(..)
            .filter(|(_, _, handle)| !fetched_handles.contains(handle))
            .map(|(prod, cons, _)| (prod, cons))
            .collect();
        if remaining.is_empty() {
            return Ok(());
        }
        // Build a digraph replica including existing edges +
        // remaining deferred edges and check for cycles
        let mut digraph = self.graph.map(|idx, _| idx, |_, _| ()).graph().clone();
        for (producer, consumer) in remaining.iter() {
            digraph.add_edge(*producer, *consumer, ());
        }
        let mut tarjan = daggy::petgraph::algo::TarjanScc::new();
        let mut sccs = Vec::new();
        tarjan.run(&digraph, |scc| {
            if scc.len() > 1 {
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
        }
        for (producer, consumer) in remaining.iter() {
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
        transaction_id: &Handle,
        result: Result<TaskResult>,
        edges: &Dag<(), ComponentEdge>,
    ) -> Result<()> {
        if let Some(producer) = self.produced.get(handle).cloned() {
            if producer.is_empty() {
                error!(target: "scheduler", { output_handle = ?hex::encode(handle) },
		       "Missing producer for handle");
                return Err(SchedulerError::DataflowGraphError.into());
            } else {
                let Some((prod_idx, _)) = producer.iter().find(|(_, tid)| tid == transaction_id)
                else {
                    error!(target: "scheduler", { output_handle = ?hex::encode(handle), transaction_id = ?hex::encode(transaction_id) },
                        "Producer transaction id not found for output");
                    return Err(SchedulerError::DataflowGraphError.into());
                };
                let prod_idx = *prod_idx;
                let mut save_result = true;
                if let Ok(ref result) = result {
                    save_result = result.is_allowed;
                    let dependent_tx_indices: Vec<_> = edges
                        .edges_directed(prod_idx, Direction::Outgoing)
                        .map(|edge| edge.target())
                        .collect();
                    // Traverse immediate dependents and add this result as an input
                    for dependent_tx_index in dependent_tx_indices {
                        let working = result.working_ct.as_ref().ok_or_else(|| {
                            error!(
                                target: "scheduler",
                                output_handle = ?hex::encode(handle),
                                transaction_id = ?hex::encode(transaction_id),
                                "same-block propagation invariant violation: successful output missing working ciphertext"
                            );
                            SchedulerError::DataflowGraphError
                        })?;
                        let dependent_tx = self
                            .graph
                            .node_weight_mut(dependent_tx_index)
                            .ok_or(SchedulerError::DataflowGraphError)?;
                        dependent_tx.inputs.entry(handle.to_vec()).and_modify(|v| {
                            *v = Some(DFGTxInput::Value((working.clone(), result.is_allowed)))
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
                    let compressed_ct = match result {
                        Ok(rok) => Ok(rok.compressed_ct.ok_or_else(|| {
                            error!(
                                target: "scheduler",
                                output_handle = ?hex::encode(handle),
                                transaction_id = ?hex::encode(transaction_id),
                                "persisted output is missing its compressed ciphertext"
                            );
                            SchedulerError::DataflowGraphError
                        })?),
                        Err(error) => Err(error),
                    };
                    self.results.push(DFGTxResult {
                        transaction_id: producer_tx.transaction_id.clone(),
                        handle: handle.to_vec(),
                        compressed_ct,
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
    pub fn get_results(&mut self) -> Result<Vec<DFGTxResult>> {
        let mut results = std::mem::take(&mut self.results);
        if self.aliased_tids.is_empty() {
            return Ok(results);
        }
        // Completion broadcast for multi-producer handles. Transaction IDs do
        // not participate in either output-handle derivation or block-scoped
        // input re-randomization, so every successful producer of one handle
        // must yield byte-identical ciphertexts. Use the lexicographically
        // smallest *available* successful producer as the source, verify every
        // other successful result against it, then publish those bytes for all
        // producer rows. Requiring the lexicographically smallest producer to
        // have executed turns a scheduler/partition omission into a permanent
        // liveness failure even though an equivalent result is available.
        //
        // Divergent successful results violate the deterministic-ciphertext
        // consensus invariant. Fail the whole batch instead of selecting one
        // partition-dependent image or hiding the violation behind canonical
        // transaction ordering.
        let aliased_handles = self.aliased_tids.keys().cloned().collect::<Vec<_>>();
        for handle in &aliased_handles {
            let Some(producers) = self.produced.get(handle) else {
                continue;
            };
            let mut successful = results
                .iter()
                .filter_map(|result| {
                    if &result.handle != handle {
                        return None;
                    }
                    result
                        .compressed_ct
                        .as_ref()
                        .ok()
                        .map(|ciphertext| (&result.transaction_id, ciphertext))
                })
                .collect::<Vec<_>>();
            if successful.is_empty() {
                continue;
            }
            successful.sort_by(|a, b| a.0.cmp(b.0));
            let (source_tid, source_cct) = successful[0];
            for (other_tid, other_cct) in successful.iter().skip(1) {
                if source_cct.ct_type != other_cct.ct_type
                    || source_cct.ct_bytes != other_cct.ct_bytes
                {
                    anyhow::bail!(
                        "multi-producer ciphertext divergence for handle {}: source tid {} type {}, divergent tid {} type {}",
                        hex::encode(handle),
                        hex::encode(source_tid),
                        source_cct.ct_type,
                        hex::encode(other_tid),
                        other_cct.ct_type,
                    );
                }
            }

            let broadcast_cct = source_cct.clone();
            results.retain(|result| &result.handle != handle);
            results.extend(producers.iter().map(|(_, tid)| DFGTxResult {
                transaction_id: tid.clone(),
                handle: handle.clone(),
                compressed_ct: Ok(broadcast_cct.clone()),
            }));
        }
        Ok(results)
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
    pub result: Result<Option<CompressedCiphertext>>,
    pub work_index: usize,
}
pub type OpEdge = u8;
pub struct OpNode {
    opcode: i32,
    result_handle: Handle,
    inputs: Vec<DFGTaskInput>,
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
    fn check_ready_inputs(
        &mut self,
        ct_map: &mut HashMap<Handle, Option<DFGTxInput>>,
        remaining_uses: &mut HashMap<Handle, usize>,
    ) -> bool {
        for i in self.inputs.iter_mut() {
            match i {
                DFGTaskInput::Value(_)
                | DFGTaskInput::SharedValue(_)
                | DFGTaskInput::Compressed(_) => continue,
                DFGTaskInput::Dependence(d) => {
                    if !matches!(ct_map.get(d), Some(Some(_))) {
                        return false;
                    }
                    let Some(uses) = remaining_uses.get(d).copied() else {
                        return false;
                    };
                    let resolved_input = if uses == 1 {
                        remaining_uses.remove(d);
                        ct_map.get_mut(d).and_then(Option::take)
                    } else {
                        remaining_uses.insert(d.clone(), uses - 1);
                        ct_map.get(d).cloned().flatten()
                    };
                    let resolved = match resolved_input {
                        Some(DFGTxInput::Value((val, _))) => DFGTaskInput::Value(val),
                        Some(DFGTxInput::Compressed((cct, _))) => DFGTaskInput::Compressed(cct),
                        None => return false,
                    };
                    *i = resolved;
                }
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

pub fn add_execution_dependences<TNode, TEdge>(
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
            });
            for n in df_nodes.iter() {
                node_map.insert(*n, ex_node);
            }
            execution_graph[ex_node].df_nodes = df_nodes;
        }
    }
    add_execution_dependences(graph, execution_graph, node_map)
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
#[allow(clippy::field_reassign_with_default, clippy::redundant_clone)]
mod tests {
    use super::*;
    use fhevm_engine_common::types::SupportedFheCiphertexts;
    use std::sync::Arc;

    fn handle(byte: u8) -> Handle {
        vec![byte; 32]
    }

    fn make_cnode(tid: Handle, result: Handle) -> ComponentNode {
        let mut node = ComponentNode::default();
        node.transaction_id = tid;
        node.results = vec![result];
        node
    }

    fn make_cnode_with_allowed_op(tid: Handle, result: Handle) -> ComponentNode {
        let mut node = make_cnode(tid, result.clone());
        node.graph.add_node(
            result,
            (SupportedFheOperations::FheTrivialEncrypt as i16).into(),
            vec![],
            true,
        );
        node
    }

    fn trivial_encrypt_op(output: Handle, transaction_id: &Handle) -> ComponentNode {
        let ops = vec![DFGOp {
            output_handle: output,
            fhe_op: SupportedFheOperations::FheTrivialEncrypt,
            inputs: vec![
                DFGTaskInput::Value(Arc::new(SupportedFheCiphertexts::Scalar(vec![0; 32]))),
                DFGTaskInput::Value(Arc::new(SupportedFheCiphertexts::Scalar(vec![5]))),
            ],
            is_allowed: true,
        }];
        build_component_nodes(ops, transaction_id)
            .expect("component build")
            .0
            .pop()
            .expect("one component")
    }

    #[test]
    fn finalize_keeps_only_allowed_results_and_their_ancestors() {
        let mut graph = Dag::<(bool, usize), OpEdge>::default();
        let kept_root = graph.add_node((false, 0));
        let kept_mid = graph.add_node((false, 1));
        let kept_result = graph.add_node((true, 2));
        let pruned_root = graph.add_node((false, 3));
        let pruned_leaf = graph.add_node((false, 4));
        graph.add_edge(kept_root, kept_mid, 0).unwrap();
        graph.add_edge(kept_mid, kept_result, 0).unwrap();
        graph.add_edge(pruned_root, pruned_leaf, 0).unwrap();

        assert_eq!(finalize(&mut graph), vec![3, 4]);
        assert_eq!(graph.node_count(), 3);
        assert!(graph.node_references().all(|(_, node)| node.0));
    }

    #[test]
    fn finalize_handles_deep_chains_in_one_iterative_pass() {
        const NODE_COUNT: usize = 10_000;
        let mut graph = Dag::<(bool, usize), OpEdge>::default();
        let mut previous = graph.add_node((false, 0));
        for index in 1..NODE_COUNT {
            let current = graph.add_node((index == NODE_COUNT - 1, index));
            graph.add_edge(previous, current, 0).unwrap();
            previous = current;
        }

        assert!(finalize(&mut graph).is_empty());
        assert_eq!(graph.node_count(), NODE_COUNT);
    }

    fn ct(bytes: &[u8]) -> CompressedCiphertext {
        CompressedCiphertext {
            ct_type: 4,
            ct_bytes: bytes.to_vec(),
        }
    }

    /// Two ComponentNodes produce the same output handle. `build()` must
    /// pick the lexicographically smallest `transaction_id` as canonical
    /// and record the other in `aliased_tids` regardless of Vec
    /// insertion order.
    #[test]
    fn build_sorts_producers_and_records_aliased_tids() {
        let out = handle(0xAA);
        let tid_lo = handle(0x01);
        let tid_hi = handle(0x02);

        for (label, order) in [
            ("lo-then-hi", vec![tid_lo.clone(), tid_hi.clone()]),
            ("hi-then-lo", vec![tid_hi.clone(), tid_lo.clone()]),
        ] {
            let mut nodes: Vec<ComponentNode> = order
                .iter()
                .map(|tid| make_cnode(tid.clone(), out.clone()))
                .collect();
            let mut g = DFComponentGraph::default();
            g.build(&mut nodes).expect("build");

            let producers = g.produced.get(&out).expect("produced entry");
            assert_eq!(producers.len(), 2, "{label}");
            assert_eq!(
                &producers[0].1, &tid_lo,
                "{label}: canonical must be the lexicographically smallest tid"
            );
            assert_eq!(&producers[1].1, &tid_hi, "{label}");

            let aliased = g.aliased_tids.get(&out).expect("aliased_tids entry");
            assert_eq!(aliased, &vec![tid_hi.clone()], "{label}");
        }
    }

    /// Single-producer handles do not appear in `aliased_tids`.
    #[test]
    fn build_single_producer_leaves_aliased_tids_empty() {
        let mut nodes = vec![make_cnode(handle(0x01), handle(0xAA))];
        let mut g = DFComponentGraph::default();
        g.build(&mut nodes).expect("build");
        assert!(g.aliased_tids.is_empty());
    }

    #[test]
    fn build_does_not_require_scalar_trivial_encrypt_literals() {
        let mut nodes = vec![
            trivial_encrypt_op(handle(0xAA), &handle(0x01)),
            trivial_encrypt_op(handle(0xAA), &handle(0x02)),
        ];
        let mut g = DFComponentGraph::default();
        g.build(&mut nodes).expect("build");

        assert!(
            !g.needed_map.contains_key(&vec![0; 32]),
            "zero literal must not be fetched as ciphertext"
        );
        assert!(
            !g.needed_map.contains_key(&vec![5]),
            "type literal must not be fetched as ciphertext"
        );
    }

    /// `get_results()` synthesizes a `DFGTxResult` for every producer tid.
    /// When the lexicographically smallest producer succeeded, its ciphertext
    /// is the deterministic broadcast source.
    #[test]
    fn get_results_broadcasts_to_missing_aliased_tids() {
        let out = handle(0xAA);
        let tid_canonical = handle(0x01);
        let tid_aliased_1 = handle(0x02);
        let tid_aliased_2 = handle(0x03);

        let mut nodes = vec![
            make_cnode(tid_canonical.clone(), out.clone()),
            make_cnode(tid_aliased_1.clone(), out.clone()),
            make_cnode(tid_aliased_2.clone(), out.clone()),
        ];
        let mut g = DFComponentGraph::default();
        g.build(&mut nodes).expect("build");

        // Simulate a same-partition scenario: only the canonical tid
        // received a `DFGTxResult` via `add_output`; the aliased tids
        // were overwritten in `res` and never routed through
        // `add_output` at all.
        let canonical_bytes = b"canonical-bytes".to_vec();
        g.results.push(DFGTxResult {
            transaction_id: tid_canonical.clone(),
            handle: out.clone(),
            compressed_ct: Ok(ct(&canonical_bytes)),
        });

        let out_results = g.get_results().expect("coalesce results");
        assert_eq!(out_results.len(), 3, "canonical + both aliased must appear");
        for tid in [&tid_canonical, &tid_aliased_1, &tid_aliased_2] {
            let entry = out_results
                .iter()
                .find(|r| &r.transaction_id == tid)
                .unwrap_or_else(|| panic!("missing result for tid {tid:?}"));
            let cct = entry.compressed_ct.as_ref().expect("Ok result");
            assert_eq!(
                cct.ct_bytes, canonical_bytes,
                "bytes must be cloned from canonical"
            );
            assert_eq!(cct.ct_type, 4);
        }
    }

    /// If a partition-layout regression leaves only an aliased producer
    /// result, promote its deterministic ciphertext to every producer row.
    /// Transaction IDs do not participate in ciphertext derivation, so making
    /// progress must not depend on which equivalent producer was scheduled.
    #[test]
    fn get_results_promotes_aliased_ok_when_canonical_missing() {
        let out = handle(0xAA);
        let tid_canonical = handle(0x01);
        let tid_aliased = handle(0x02);

        let mut nodes = vec![
            make_cnode(tid_canonical.clone(), out.clone()),
            make_cnode(tid_aliased.clone(), out.clone()),
        ];
        let mut g = DFComponentGraph::default();
        g.build(&mut nodes).expect("build");

        let aliased_bytes = b"aliased-bytes".to_vec();
        g.results.push(DFGTxResult {
            transaction_id: tid_aliased.clone(),
            handle: out.clone(),
            compressed_ct: Ok(ct(&aliased_bytes)),
        });

        let out_results = g.get_results().expect("promote aliased result");
        assert_eq!(out_results.len(), 2, "both producer rows must complete");
        for tid in [&tid_canonical, &tid_aliased] {
            let result = out_results
                .iter()
                .find(|result| &result.transaction_id == tid)
                .unwrap_or_else(|| panic!("missing result for tid {tid:?}"));
            let ciphertext = result.compressed_ct.as_ref().expect("Ok result");
            assert_eq!(ciphertext.ct_type, 4);
            assert_eq!(ciphertext.ct_bytes, aliased_bytes);
        }
    }

    /// Independently computed successful producer results must agree exactly.
    /// Divergence is a consensus violation, not a value that canonical
    /// transaction ordering can safely hide.
    #[test]
    fn get_results_rejects_divergent_aliased_ciphertext() {
        let out = handle(0xAA);
        let tid_canonical = handle(0x01);
        let tid_aliased = handle(0x02);

        let mut nodes = vec![
            make_cnode(tid_canonical.clone(), out.clone()),
            make_cnode(tid_aliased.clone(), out.clone()),
        ];
        let mut g = DFComponentGraph::default();
        g.build(&mut nodes).expect("build");

        g.results.push(DFGTxResult {
            transaction_id: tid_canonical.clone(),
            handle: out.clone(),
            compressed_ct: Ok(ct(b"canonical")),
        });
        g.results.push(DFGTxResult {
            transaction_id: tid_aliased.clone(),
            handle: out.clone(),
            compressed_ct: Ok(ct(b"aliased-independently-computed")),
        });

        let error = g
            .get_results()
            .expect_err("divergent ciphertext bytes must fail the batch");
        assert!(error
            .to_string()
            .contains("multi-producer ciphertext divergence"));
    }

    /// If every producer failed (no Ok result for the handle), the
    /// broadcast must not invent a synthetic Ok entry — the aliased
    /// tids are left alone so their own error rows remain the source
    /// of truth.
    #[test]
    fn get_results_skips_broadcast_when_canonical_errored() {
        let out = handle(0xAA);
        let tid_canonical = handle(0x01);
        let tid_aliased = handle(0x02);

        let mut nodes = vec![
            make_cnode(tid_canonical.clone(), out.clone()),
            make_cnode(tid_aliased.clone(), out.clone()),
        ];
        let mut g = DFComponentGraph::default();
        g.build(&mut nodes).expect("build");

        g.results.push(DFGTxResult {
            transaction_id: tid_canonical.clone(),
            handle: out.clone(),
            compressed_ct: Err(SchedulerError::MissingInputs.into()),
        });

        let out_results = g.get_results().expect("retain producer error");
        assert_eq!(out_results.len(), 1, "no Ok ciphertext to broadcast");
        assert_eq!(out_results[0].transaction_id, tid_canonical);
    }

    /// Duplicate-handle errors must be attributed to the producer tid that
    /// emitted the error. Falling back to the canonical producer poisons the
    /// wrong dependency path and can leave the failed aliased row pending.
    #[test]
    fn add_output_error_uses_result_transaction_id_for_duplicate_handles() {
        let out = handle(0xAA);
        let tid_canonical = handle(0x01);
        let tid_aliased = handle(0x02);

        let mut nodes = vec![
            make_cnode_with_allowed_op(tid_canonical.clone(), out.clone()),
            make_cnode_with_allowed_op(tid_aliased.clone(), out.clone()),
        ];
        let mut g = DFComponentGraph::default();
        g.build(&mut nodes).expect("build");

        let edges = g.graph.map(|_, _| (), |_, edge| *edge);
        g.add_output(
            &out,
            &tid_aliased,
            Err(SchedulerError::MissingInputs.into()),
            &edges,
        )
        .expect("add output");

        let producers = g.produced.get(&out).expect("produced entry");
        let canonical_idx = producers
            .iter()
            .find(|(_, tid)| tid == &tid_canonical)
            .expect("canonical producer")
            .0;
        let aliased_idx = producers
            .iter()
            .find(|(_, tid)| tid == &tid_aliased)
            .expect("aliased producer")
            .0;

        assert!(
            !g.graph
                .node_weight(canonical_idx)
                .expect("canonical node")
                .is_uncomputable,
            "canonical producer must not be marked uncomputable"
        );
        assert!(
            g.graph
                .node_weight(aliased_idx)
                .expect("aliased node")
                .is_uncomputable,
            "aliased producer must be marked uncomputable"
        );

        let out_results = g.get_results().expect("retain producer error");
        assert!(
            !out_results.is_empty(),
            "the aliased failure should emit at least one error result"
        );
        assert!(
            out_results
                .iter()
                .all(|result| result.transaction_id == tid_aliased),
            "all emitted error results must belong to the failed aliased producer"
        );
    }

    #[test]
    fn add_output_errors_when_transaction_id_is_not_a_producer() {
        let out = handle(0xAA);
        let tid_canonical = handle(0x01);
        let tid_aliased = handle(0x02);
        let tid_unknown = handle(0x03);

        let mut nodes = vec![
            make_cnode_with_allowed_op(tid_canonical.clone(), out.clone()),
            make_cnode_with_allowed_op(tid_aliased.clone(), out.clone()),
        ];
        let mut g = DFComponentGraph::default();
        g.build(&mut nodes).expect("build");

        let edges = g.graph.map(|_, _| (), |_, edge| *edge);
        let error = g
            .add_output(
                &out,
                &tid_unknown,
                Err(SchedulerError::MissingInputs.into()),
                &edges,
            )
            .expect_err("unknown producer transaction id should fail closed");

        assert_eq!(
            error.to_string(),
            SchedulerError::DataflowGraphError.to_string()
        );
        assert!(
            g.get_results().expect("collect results").is_empty(),
            "unknown producer output must not be saved under another producer"
        );
    }

    #[test]
    fn add_output_rejects_uncompressed_persisted_ciphertext() {
        let out = handle(0xAA);
        let tid = handle(0x01);
        let mut nodes = vec![make_cnode_with_allowed_op(tid.clone(), out.clone())];
        let mut g = DFComponentGraph::default();
        g.build(&mut nodes).expect("build");

        let edges = g.graph.map(|_, _| (), |_, edge| *edge);
        let error = g
            .add_output(
                &out,
                &tid,
                Ok(TaskResult {
                    compressed_ct: None,
                    working_ct: Some(Arc::new(SupportedFheCiphertexts::Scalar(vec![1]))),
                    is_allowed: true,
                    transaction_id: tid.clone(),
                }),
                &edges,
            )
            .expect_err("block-exit ciphertexts must be compressed");

        assert_eq!(
            error.to_string(),
            SchedulerError::DataflowGraphError.to_string()
        );
        assert!(g.get_results().expect("collect results").is_empty());
    }
}
