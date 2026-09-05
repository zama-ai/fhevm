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
    /// Whether this worker OWNS the row: its chain is the one under lease.
    ///
    /// Distinct from `is_allowed`, and not derivable from it. `is_allowed` says
    /// the output is persisted and visible to consumers, so it is already
    /// `owned && row.is_allowed` by the time it reaches here -- an internal
    /// producer we own is `is_owned` without being `is_allowed`, and a foreign
    /// row loaded as a recompute-only producer is neither. Error retention has
    /// to key on ownership: an owned internal producer has a row of ours to
    /// stamp, a foreign one does not.
    pub is_owned: bool,
}
impl Default for DFGOp {
    fn default() -> Self {
        DFGOp {
            output_handle: vec![],
            fhe_op: SupportedFheOperations::FheTrivialEncrypt,
            inputs: vec![],
            is_allowed: false,
            is_owned: false,
        }
    }
}
pub type ComponentEdge = ();
/// A transaction's inner dataflow graph reduced to `(result handle,
/// is_allowed)` per node; see `DFComponentGraph::blocked_dependents`.
type ReducedGraph = Dag<(Handle, bool), OpEdge>;
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
                DFGTaskInput::LocalDependence(dh) => {
                    let producer = produced_handles
                        .get(dh)
                        .ok_or(SchedulerError::MissingLocalProducer)?;
                    dependence_pairs.push((*producer, index, pos));
                }
                // A boundary operand must not become local merely because a
                // stale row with the same transaction id was loaded. It is
                // intentionally left without an operation edge and exposed
                // to the canonical ciphertext fetch below.
                DFGTaskInput::BoundaryDependence(_)
                | DFGTaskInput::Value(_)
                | DFGTaskInput::Compressed(..) => {}
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
    // The transaction is the materialization boundary: all remaining
    // operations execute as one unit so intra-transaction intermediates are
    // forwarded in memory and never pay a compress/decompress round-trip.
    // The consuming handle's boundary bit pins that choice on chain, so an
    // operand minted in the consuming transaction is always read raw, while
    // values crossing a transaction boundary are read in their canonical
    // compressed form.
    let mut kept: Vec<usize> = graph
        .node_references()
        .map(|(_, op_node)| op_node.1)
        .collect();
    // Deterministic op order (original list is sorted by output handle);
    // execution order is derived from the dependence edges by toposort.
    kept.sort_unstable();
    if !kept.is_empty() {
        let mut component_ops = Vec::with_capacity(kept.len());
        for i in kept {
            component_ops.push(std::mem::take(&mut operations[i]));
        }
        let mut component = ComponentNode::default();
        component.build(component_ops, transaction_id, 0)?;
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
                    DFGTaskInput::LocalDependence(dh) => {
                        let producer = produced_handles
                            .get(dh)
                            .ok_or(SchedulerError::MissingLocalProducer)?;
                        dependence_pairs.push((*producer, index, pos));
                    }
                    DFGTaskInput::BoundaryDependence(dh) => {
                        // The listener derived this bit from executor logs,
                        // so it is the sole authority for representation
                        // choice. Always expose it for canonical sourcing.
                        self.inputs.entry(dh.clone()).or_insert(None);
                    }
                    DFGTaskInput::Value(_) | DFGTaskInput::Compressed(..) => {}
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
                    op.is_owned,
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
    deferred_dependences: Vec<(NodeIndex, NodeIndex, Handle)>,
    /// Per transaction, the inner dataflow graph reduced to
    /// `(result handle, is_allowed)` per node. Taken by
    /// [`Self::snapshot_blocked_dependents`] BEFORE scheduling, because the
    /// scheduler moves each transaction's inner graph out of its node at
    /// dispatch (`std::mem::take`) and never puts it back, so after
    /// `schedule()` the edges [`Self::allowed_dependents`] walks no longer
    /// exist here. A reduced copy of the graph is O(V+E) per transaction and
    /// answers for EVERY producer, unlike a per-producer list which is
    /// quadratic on hub-shaped transactions and has to guess which producers
    /// will need it.
    blocked_dependents: Option<HashMap<Handle, ReducedGraph>>,
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
        // Every transaction-level input is an authoritative boundary input.
        // Fetch its canonical persisted representation first. A concurrently
        // scheduled foreign producer may supply the same *compressed* form if
        // that fetch is not ready yet, but a same-transaction producer is
        // deliberately ignored: it may be an orphan/stale row whose mere
        // presence must never override the listener-derived boundary bit.
        for (consumer, tx) in self.graph.node_references() {
            for i in tx.inputs.keys() {
                self.needed_map
                    .entry(i.clone())
                    .and_modify(|uses| uses.push(consumer))
                    .or_insert(vec![consumer]);

                if let Some(producers) = self.produced.get(i) {
                    let mut foreign_producers = producers
                        .iter()
                        .filter(|(_, tid)| *tid != tx.transaction_id);
                    if let Some((producer, _)) = foreign_producers.next() {
                        if foreign_producers.next().is_none() {
                            self.deferred_dependences
                                .push((*producer, consumer, i.clone()));
                        } else {
                            // Several in-batch candidates are not an excuse
                            // to choose arbitrary raw bytes. Leave this as a
                            // DB-only boundary; absent ciphertext defers the
                            // consumer until canonical material is present.
                            warn!(target: "scheduler", output_handle = ?hex::encode(i),
                                  "multiple foreign producers for boundary handle; requiring canonical DB ciphertext");
                        }
                    }
                }
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
        // A cross-transaction cycle is an artifact of BATCH COMPOSITION, never
        // a fact about the chain: the on-chain dependence graph is acyclic,
        // and these edges only exist between the transactions that happen to
        // share this graph. The reproducible shape is a same-block alias --
        // t0 mints h1, t1 consumes h1, t2 consumes t1's output and re-mints
        // h1 (same op, operands and boundary bits, so the same handle). With
        // t0 present, h1 has two in-batch producers and draws no edge. With
        // t0 absent -- demoted, deferred, or simply not selected -- t2 is
        // h1's only in-batch producer, and t1 <-> t2 closes a cycle that
        // exists on no other coprocessor.
        //
        // So the members are DEFERRED, exactly as a consumer whose producer
        // is merely absent is: marked uncomputable, reported as
        // MissingInputs (which upload leaves unstamped), and their edges
        // dropped so the rest of the batch still executes. They come back
        // once the missing producer's bytes are in `ciphertexts`, at which
        // point the edge is never drawn. Stamping them -- and CyclicDependence
        // used to be a TERMINAL stamp -- condemned two transactions on one
        // coprocessor that every other one computed.
        let mut in_cycle: HashSet<NodeIndex> = HashSet::new();
        for scc in sccs {
            warn!(target: "scheduler", { cycle_size = ?scc.len() },
                  "cross-transaction dependence cycle from batch composition; deferring its transactions");
            for idx in scc {
                let idx = digraph
                    .node_weight(idx)
                    .ok_or(SchedulerError::DataflowGraphError)?;
                in_cycle.insert(*idx);
                let tx = self
                    .graph
                    .node_weight_mut(*idx)
                    .ok_or(SchedulerError::DataflowGraphError)?;
                tx.is_uncomputable = true;
                warn!(target: "scheduler", { transaction_id = ?hex::encode(tx.transaction_id.clone()) },
                      "transaction deferred: part of a batch-composition dependence cycle");
                for (_, op) in tx.graph.graph.node_references() {
                    self.results.push(DFGTxResult {
                        transaction_id: tx.transaction_id.clone(),
                        handle: op.result_handle.to_vec(),
                        compressed_ct: Err(SchedulerError::MissingInputs.into()),
                    });
                }
            }
        }
        for (producer, consumer) in remaining.iter().filter(|(producer, consumer)| {
            !in_cycle.contains(producer) && !in_cycle.contains(consumer)
        }) {
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
        transaction_id: &[u8],
        result: Result<TaskResult>,
        edges: &Dag<(), ComponentEdge>,
    ) -> Result<()> {
        if let Some(producer) = self.produced.get(handle).cloned() {
            if producer.is_empty() {
                error!(target: "scheduler", { output_handle = ?hex::encode(handle) },
		       "Missing producer for handle");
            } else {
                // Attribute to the producer whose TRANSACTION this outcome
                // belongs to, success or failure alike. Only the success arm
                // used to do this, from `TaskResult.transaction_id`; an error
                // carried no identity, so it fell through to `producer[0]` --
                // an arbitrary one of the colliding transactions. That stamped
                // a row belonging to another transaction (possibly one on a
                // chain this worker does not even own) and left the row that
                // actually failed unstamped, so it never accrued a retry count
                // and never reached demotion.
                //
                // The fallback remains for the single-producer case, which is
                // every handle that is not a same-block collision.
                let mut prod_idx = producer[0].0;
                if let Some((pid, _)) = producer
                    .iter()
                    .find(|(_, tid)| tid.as_slice() == transaction_id)
                {
                    prod_idx = *pid;
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
                            *v = Some(DFGTxInput::Compressed((
                                result.compressed_ct.clone(),
                                result.is_allowed,
                            )))
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
                        compressed_ct: result.map(|rok| rok.compressed_ct),
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
    /// The allowed handles of `transaction_id` that transitively depend on
    /// `handle`, read from the dataflow graph rather than inferred from stored
    /// operand bytes.
    ///
    /// The distinction is not academic. `computations.dependencies` holds
    /// encrypted operand handles AND plain scalar values in one array, so a
    /// byte-equality search over it cannot tell a dependency from a scalar whose
    /// value happens to equal a handle -- and an operand's boundary bit cannot
    /// separate them either, because only ENCRYPTED positions are visited when
    /// the mask is derived, leaving a scalar position indistinguishable from an
    /// operand minted in this transaction. Here the edges are typed: they exist
    /// because one op consumes another's output.
    ///
    /// Returns allowed handles only, since those are the rows a verdict can be
    /// recorded on, and skips the starting handle itself.
    ///
    /// After [`Self::snapshot_blocked_dependents`] this answers from the
    /// snapshot, which is the only form that survives scheduling: the
    /// scheduler takes each transaction's inner graph out of its node when it
    /// dispatches the partition, so a walk over `tx.graph` after `schedule()`
    /// sees an empty graph and finds nothing. Before a snapshot it walks the
    /// live graph, which is what the graph-construction tests exercise.
    pub fn allowed_dependents(&self, transaction_id: &[u8], handle: &[u8]) -> Vec<Handle> {
        if let Some(snapshot) = &self.blocked_dependents {
            return snapshot
                .get(transaction_id)
                .map(|reduced| Self::allowed_dependents_in(reduced, handle))
                .unwrap_or_default();
        }
        let Some((_, tx)) = self
            .graph
            .node_references()
            .find(|(_, tx)| tx.transaction_id.as_slice() == transaction_id)
        else {
            return vec![];
        };
        Self::allowed_dependents_in(&Self::reduce(&tx.graph), handle)
    }

    /// Keep, for every transaction, what [`Self::allowed_dependents`] needs
    /// to answer after scheduling: the inner graph's edges and each node's
    /// `(result handle, is_allowed)`. Call this once the graph is fully built
    /// and BEFORE handing it to the scheduler.
    ///
    /// Every producer is covered, whatever its `is_allowed`. The fallback
    /// this serves fires whenever a failed handle's own row cannot carry the
    /// verdict, and that is not only the internal producer the listener
    /// stores `is_completed = TRUE`: an ALLOWED producer already persisted in
    /// an earlier cycle is re-executed as a raw forward for its pending
    /// consumers, and its direct stamp (`WHERE is_completed = false`) then
    /// matches nothing either.
    pub fn snapshot_blocked_dependents(&mut self) {
        let snapshot = self
            .graph
            .node_references()
            .map(|(_, tx)| (tx.transaction_id.clone(), Self::reduce(&tx.graph)))
            .collect();
        self.blocked_dependents = Some(snapshot);
    }

    fn reduce(graph: &DFGraph) -> ReducedGraph {
        graph.graph.map(
            |_, node| (node.result_handle.clone(), node.is_allowed),
            |_, edge| *edge,
        )
    }

    /// The allowed handles transitively downstream of `handle` in a reduced
    /// graph, excluding `handle` itself.
    fn allowed_dependents_in(reduced: &ReducedGraph, handle: &[u8]) -> Vec<Handle> {
        let Some(start) = reduced
            .node_references()
            .find(|(_, (result_handle, _))| result_handle.as_slice() == handle)
            .map(|(index, _)| index)
        else {
            return vec![];
        };
        let mut seen = std::collections::HashSet::new();
        let mut stack = vec![start];
        let mut out = vec![];
        while let Some(current) = stack.pop() {
            for edge in reduced.edges_directed(current, Direction::Outgoing) {
                let next = edge.target();
                if !seen.insert(next) {
                    continue;
                }
                if let Some((result_handle, is_allowed)) = reduced.node_weight(next) {
                    if *is_allowed {
                        out.push(result_handle.clone());
                    }
                }
                stack.push(next);
            }
        }
        out
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
    pub result: Result<Option<CompressedCiphertext>>,
    pub work_index: usize,
}
pub type OpEdge = u8;
pub struct OpNode {
    opcode: i32,
    result_handle: Handle,
    inputs: Vec<DFGTaskInput>,
    is_allowed: bool,
    is_owned: bool,
}
impl std::fmt::Debug for OpNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpNode")
            .field("OP", &self.opcode)
            .field("Result handle", &format_args!("{:?}", self.result_handle))
            .finish()
    }
}
impl OpNode {
    fn check_ready_inputs(&mut self, ct_map: &mut HashMap<Handle, Option<DFGTxInput>>) -> bool {
        for i in self.inputs.iter_mut() {
            match i {
                DFGTaskInput::Value(_) | DFGTaskInput::Compressed(..) => continue,
                DFGTaskInput::LocalDependence(d) => {
                    error!(target: "scheduler", handle = ?hex::encode(d),
                           "transaction-local dependence reached execution without local producer");
                    return false;
                }
                DFGTaskInput::BoundaryDependence(d) => {
                    let resolved = match ct_map.get(d) {
                        Some(Some(DFGTxInput::Value((val, _)))) => {
                            // CONSENSUS INVARIANT: a transaction-level value
                            // input must be the DECOMPRESSED CANONICAL FORM of
                            // the handle's persisted bytes — byte-identical to
                            // what any consumer would reconstruct itself. A raw
                            // working value injected here would make the
                            // consumer's bytes depend on which node or pass
                            // produced it, which is a consensus divergence.
                            //
                            // This variant has NO constructor in either build
                            // configuration: boundary operands enter the graph
                            // compressed and are decompressed in the executor,
                            // which memoizes ct(h) per partition rather than
                            // materializing raw values here. The arm is
                            // therefore unreachable by construction and the
                            // check is free — it exists so that if a future
                            // change starts injecting transaction-level raw
                            // values, on either backend, this says so instead
                            // of silently changing consumers' bytes.
                            if !matches!(
                                val,
                                fhevm_engine_common::types::SupportedFheCiphertexts::Scalar(_)
                            ) {
                                error!(target: "scheduler", { handle = ?hex::encode(&self.result_handle) },
                                       "Consensus risk: non-scalar raw ciphertext crossing a transaction boundary");
                            }
                            DFGTaskInput::Value(val.clone())
                        }
                        Some(Some(DFGTxInput::Compressed((cct, _)))) => {
                            DFGTaskInput::Compressed(d.clone(), cct.clone())
                        }
                        _ => return false,
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
    #[allow(clippy::too_many_arguments)]
    pub fn add_node(
        &mut self,
        rh: Handle,
        opcode: i32,
        inputs: Vec<DFGTaskInput>,
        is_allowed: bool,
        is_owned: bool,
    ) -> NodeIndex {
        self.graph.add_node(OpNode {
            opcode,
            result_handle: rh,
            inputs,
            is_allowed,
            is_owned,
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
mod tests {
    use super::*;

    fn handle(byte: u8) -> Handle {
        vec![byte; 32]
    }

    fn op(output: u8, input: DFGTaskInput, is_allowed: bool) -> DFGOp {
        // Owned: these fixtures stand in for rows under this worker's lease.
        DFGOp {
            output_handle: handle(output),
            fhe_op: SupportedFheOperations::FheNot,
            inputs: vec![input],
            is_allowed,
            is_owned: true,
        }
    }

    /// An error must be attributed to the transaction that produced it, even
    /// when another transaction mints the same handle.
    ///
    /// Two transactions in one block performing the same operation on the same
    /// operands share a handle preimage, so they mint the SAME handle -- the
    /// reason `produced` maps a handle to a LIST of producers. Successes were
    /// already disambiguated through `TaskResult.transaction_id`; errors
    /// carried no identity and fell through to `producer[0]`, an arbitrary one
    /// of the two. That stamped a row belonging to the other transaction and
    /// left the one that actually failed unstamped, so it never accrued a retry
    /// count and never reached demotion.
    #[test]
    fn an_error_is_attributed_to_the_transaction_that_raised_it() {
        let tx_one = handle(0xA1);
        let tx_two = handle(0xB1);
        let colliding = handle(0x01);
        let boundary = handle(0xA0);

        let mut nodes = vec![];
        for tid in [&tx_one, &tx_two] {
            let (mut components, _) = build_component_nodes(
                vec![op(
                    0x01,
                    DFGTaskInput::BoundaryDependence(boundary.clone()),
                    true,
                )],
                tid,
            )
            .expect("valid transaction graph");
            nodes.append(&mut components);
        }
        let mut graph = DFComponentGraph::default();
        graph.build(&mut nodes).expect("component graph");
        assert_eq!(
            graph.produced.get(&colliding).map(Vec::len),
            Some(2),
            "the fixture must actually produce one handle from two \
             transactions, or this proves nothing"
        );

        // Blame whichever producer is NOT first in the list, because
        // `producer[0]` is exactly the old fallback: attributing to the first
        // one would pass under the bug as well as under the fix. `build` pops
        // its input, so the list order is not the fixture's order -- read it
        // rather than assume it.
        let producers = graph.produced.get(&colliding).expect("producers").clone();
        let first_tid = producers[0].1.clone();
        let victim = producers
            .iter()
            .map(|(_, tid)| tid.clone())
            .find(|tid| *tid != first_tid)
            .expect("two distinct producing transactions");

        // An edge-only view with one node per component, as the scheduler
        // passes; no edges, since neither transaction depends on the other.
        let mut edges: Dag<(), ComponentEdge> = Dag::new();
        for _ in 0..graph.graph.node_count() {
            edges.add_node(());
        }

        graph
            .add_output(
                &colliding,
                &victim,
                Err(SchedulerError::ExecutionPanic("device fault".into()).into()),
                &edges,
            )
            .expect("add_output");

        let attributed: Vec<_> = graph
            .get_results()
            .into_iter()
            .filter(|r| r.handle == colliding && r.compressed_ct.is_err())
            .map(|r| r.transaction_id)
            .collect();
        assert!(
            attributed.contains(&victim),
            "the failing transaction must be stamped; got {attributed:?}"
        );
        assert!(
            !attributed.contains(&first_tid),
            "the other transaction's row must not be stamped for a failure it \
             did not have; got {attributed:?}"
        );
    }

    /// The blocked-dependents answer must survive scheduling.
    ///
    /// The scheduler `std::mem::take`s every dispatched transaction's inner
    /// graph and never restores it, so a walk over the live graph after
    /// `schedule()` returns nothing -- which silently disabled the fallback
    /// that records an internal producer's verdict on its allowed consumers.
    /// The snapshot is taken before dispatch; this simulates the take and
    /// checks the answer is unchanged.
    #[test]
    fn blocked_dependents_survive_the_scheduler_taking_the_graph() {
        let transaction_id = handle(0x11);
        let boundary = handle(0xA0);
        let operations = vec![
            // Internal producer P = f(boundary); allowed C1 = f(P); allowed
            // C2 = f(C1) (transitive); allowed U = f(boundary), unrelated.
            op(
                0x01,
                DFGTaskInput::BoundaryDependence(boundary.clone()),
                false,
            ),
            op(0x02, DFGTaskInput::LocalDependence(handle(0x01)), true),
            op(0x03, DFGTaskInput::LocalDependence(handle(0x02)), true),
            op(0x04, DFGTaskInput::BoundaryDependence(boundary), true),
        ];
        let (mut components, _) =
            build_component_nodes(operations, &transaction_id).expect("valid transaction graph");
        let mut graph = DFComponentGraph::default();
        graph.build(&mut components).expect("component graph");

        let expected = {
            let mut live = graph.allowed_dependents(&transaction_id, &handle(0x01));
            live.sort();
            live
        };
        assert_eq!(
            expected,
            vec![handle(0x02), handle(0x03)],
            "the live walk is the reference: both transitive allowed \
             consumers, not the unrelated allowed op, not the producer"
        );

        graph.snapshot_blocked_dependents();

        // What `schedule_coarse_grain` does to every dispatched transaction.
        for tx in graph.graph.node_weights_mut() {
            let _ = std::mem::take(&mut tx.graph);
            let _ = std::mem::take(&mut tx.inputs);
        }

        let mut after = graph.allowed_dependents(&transaction_id, &handle(0x01));
        after.sort();
        assert_eq!(
            after, expected,
            "the answer after dispatch must equal the pre-dispatch walk"
        );
        assert_eq!(
            graph.allowed_dependents(&transaction_id, &handle(0x02)),
            vec![handle(0x03)],
            "an allowed producer is covered too: re-executed after an earlier \
             completion, its own row cannot carry a verdict either"
        );
        assert!(
            graph
                .allowed_dependents(&handle(0x99), &handle(0x01))
                .is_empty(),
            "an unknown transaction has no blocked dependents"
        );
    }

    fn op_multi(output: Handle, inputs: Vec<DFGTaskInput>, is_allowed: bool) -> DFGOp {
        DFGOp {
            output_handle: output,
            fhe_op: SupportedFheOperations::FheNot,
            inputs,
            is_allowed,
            is_owned: true,
        }
    }

    fn component(ops: Vec<DFGOp>, tid: &Handle) -> ComponentNode {
        let (mut components, _) = build_component_nodes(ops, tid).expect("valid transaction graph");
        assert_eq!(components.len(), 1, "one execution unit per transaction");
        components.pop().expect("component")
    }

    fn bd(h: &Handle) -> DFGTaskInput {
        DFGTaskInput::BoundaryDependence(h.clone())
    }

    /// A cross-transaction cycle is a batch-composition artifact and must
    /// DEFER its members, never condemn them.
    ///
    /// Same-block alias: t0 mints h1; t1 consumes h1 -> h3; t2 consumes h3
    /// and re-mints h1. With t0 in the graph h1 has two in-batch producers
    /// and draws no edge. With t0 absent (demoted, deferred, not selected)
    /// t2 is h1's only in-batch producer and t1 <-> t2 is a cycle on THIS
    /// coprocessor only. The members must come back as MissingInputs, which
    /// upload leaves unstamped, and an unrelated transaction in the same
    /// batch must still be schedulable.
    #[test]
    fn cross_transaction_cycle_defers_its_members_instead_of_condemning_them() {
        let (t0, t1, t2, t9) = (handle(0x00), handle(0x11), handle(0x22), handle(0x99));
        let (x, h1, h3, h4, h9) = (
            handle(0xF0),
            handle(0x01),
            handle(0x03),
            handle(0x04),
            handle(0x09),
        );
        let t1_ops = || vec![op_multi(h3.clone(), vec![bd(&h1)], true)];
        let t2_ops = || {
            vec![
                op_multi(h4.clone(), vec![bd(&h3)], true),
                op_multi(h1.clone(), vec![bd(&x)], true),
            ]
        };
        let t9_ops = || vec![op_multi(h9.clone(), vec![bd(&x)], true)];

        // With the original minter present: two foreign producers of h1, no
        // edge, no cycle, nothing reported.
        let mut nodes = vec![
            component(vec![op_multi(h1.clone(), vec![bd(&x)], true)], &t0),
            component(t1_ops(), &t1),
            component(t2_ops(), &t2),
        ];
        let mut graph = DFComponentGraph::default();
        graph.build(&mut nodes).expect("component graph");
        graph
            .resolve_dependences(&HashSet::new())
            .expect("two foreign producers of h1 draw no edge");
        assert!(graph.get_results().is_empty());

        // Without it: the false cycle defers t1 and t2, and t9 still runs.
        let mut nodes = vec![
            component(t1_ops(), &t1),
            component(t2_ops(), &t2),
            component(t9_ops(), &t9),
        ];
        let mut graph = DFComponentGraph::default();
        graph.build(&mut nodes).expect("component graph");
        graph
            .resolve_dependences(&HashSet::new())
            .expect("a batch-composition cycle is not an error of the batch");

        let mut deferred: Vec<(Handle, Handle)> = vec![];
        for result in graph.get_results() {
            let Err(error) = result.compressed_ct else {
                panic!("cycle members carry no bytes");
            };
            assert!(
                matches!(
                    error.downcast_ref::<SchedulerError>(),
                    Some(SchedulerError::MissingInputs)
                ),
                "cycle members must be DEFERRED (MissingInputs), never stamped: {error}"
            );
            deferred.push((result.handle, result.transaction_id));
        }
        deferred.sort();
        let mut expected = vec![(h3, t1.clone()), (h4, t2.clone()), (h1, t2.clone())];
        expected.sort();
        assert_eq!(deferred, expected, "every op of both cycle members defers");

        for (_, tx) in graph.graph.node_references() {
            let in_cycle = tx.transaction_id == t1 || tx.transaction_id == t2;
            assert_eq!(
                tx.is_uncomputable,
                in_cycle,
                "only the cycle members are skipped; {:?} must {} run",
                hex::encode(&tx.transaction_id),
                if in_cycle { "not" } else { "still" }
            );
        }
        assert_eq!(
            graph.graph.edge_count(),
            0,
            "edges touching a deferred member are dropped, not added"
        );
    }

    /// The transaction is the materialization boundary: a fan-out graph that
    /// partitioning would previously split into several execution segments
    /// (with compressed hand-offs between them) must build as a single
    /// ComponentNode so every intra-transaction edge stays an in-memory
    /// forward of the raw working value.
    #[test]
    fn transaction_builds_as_a_single_execution_unit() {
        let boundary = handle(0xA0);
        let operations = vec![
            // X = f(boundary); Y = f(X); Z = f(X) — a fan-out at X.
            op(
                0x01,
                DFGTaskInput::BoundaryDependence(boundary.clone()),
                false,
            ),
            op(0x02, DFGTaskInput::LocalDependence(handle(0x01)), true),
            op(0x03, DFGTaskInput::LocalDependence(handle(0x01)), true),
        ];
        let (components, unneeded) =
            build_component_nodes(operations, &handle(0x11)).expect("valid transaction graph");
        assert!(unneeded.is_empty());
        assert_eq!(
            components.len(),
            1,
            "all of a transaction's operations must execute as one unit"
        );
        let component = &components[0];
        assert_eq!(component.results.len(), 3);
        assert_eq!(component.intermediate_handles, vec![handle(0x01)]);
        // Only the true cross-transaction boundary is exposed as an input;
        // the intra-transaction handles are resolved as graph edges.
        assert_eq!(component.inputs.len(), 1);
        assert!(component.inputs.contains_key(&boundary));
        assert_eq!(component.graph.graph.edge_count(), 2);
    }

    /// Pruning of operations that no allowed output depends on is unchanged.
    #[test]
    fn unneeded_operations_are_still_pruned() {
        let operations = vec![
            op(0x01, DFGTaskInput::BoundaryDependence(handle(0xA0)), true),
            // A dangling non-allowed op nothing depends on.
            op(0x02, DFGTaskInput::BoundaryDependence(handle(0xA1)), false),
        ];
        let transaction_id = handle(0x11);
        let (components, unneeded) =
            build_component_nodes(operations, &transaction_id).expect("valid transaction graph");
        assert_eq!(unneeded, vec![(handle(0x02), transaction_id)]);
        assert_eq!(components.len(), 1);
        assert_eq!(components[0].results, vec![handle(0x01)]);
    }

    /// A transaction whose operations are all pruned yields no execution unit.
    #[test]
    fn fully_pruned_transaction_yields_no_component() {
        let operations = vec![op(
            0x01,
            DFGTaskInput::BoundaryDependence(handle(0xA0)),
            false,
        )];
        let (components, unneeded) =
            build_component_nodes(operations, &handle(0x11)).expect("valid transaction graph");
        assert!(components.is_empty());
        assert_eq!(unneeded.len(), 1);
    }

    #[test]
    fn local_dependence_requires_a_producer_in_its_transaction() {
        let operations = vec![op(0x01, DFGTaskInput::LocalDependence(handle(0xA0)), true)];

        let err = build_component_nodes(operations, &handle(0x11))
            .expect_err("a locally minted operand without its producer is unsafe");
        assert!(matches!(
            err.downcast_ref::<SchedulerError>(),
            Some(SchedulerError::MissingLocalProducer)
        ));
    }

    #[test]
    fn boundary_dependence_ignores_same_transaction_stale_producer() {
        let stale_handle = handle(0xA0);
        let operations = vec![
            // This can be an orphan row retained after a reorg. It shares a
            // transaction id with the consumer but is NOT authority to turn
            // the consumer's boundary-marked operand into a raw edge.
            op(0x01, DFGTaskInput::BoundaryDependence(handle(0xB0)), false),
            op(
                0x02,
                DFGTaskInput::BoundaryDependence(stale_handle.clone()),
                true,
            ),
            DFGOp {
                output_handle: stale_handle.clone(),
                fhe_op: SupportedFheOperations::FheNot,
                inputs: vec![DFGTaskInput::BoundaryDependence(handle(0xC0))],
                is_allowed: false,
                is_owned: true,
            },
        ];

        let (components, _) = build_component_nodes(operations, &handle(0x11))
            .expect("boundary source must not use the stale producer");
        let component = components
            .iter()
            .find(|component| component.results.contains(&handle(0x02)))
            .expect("consumer remains in the component");
        assert!(component.inputs.contains_key(&stale_handle));
        assert_eq!(
            component.graph.graph.edge_count(),
            0,
            "boundary operand has no raw in-transaction edge"
        );
    }
}
