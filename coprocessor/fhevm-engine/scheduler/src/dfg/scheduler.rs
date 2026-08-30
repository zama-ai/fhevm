use crate::{
    dfg::{
        partition_components, partition_preserving_parallelism, types::*, ComponentEdge, ExecNode,
    },
    FHE_BATCH_LATENCY_HISTOGRAM, RERAND_LATENCY_BATCH_HISTOGRAM,
};
use anyhow::Result;
use daggy::{
    petgraph::{
        visit::{EdgeRef, IntoEdgesDirected, IntoNodeIdentifiers, IntoNodeReferences},
        Direction::{self, Incoming},
    },
    Dag, NodeIndex,
};
use fhevm_engine_common::common::FheOperation;
use fhevm_engine_common::telemetry;
use fhevm_engine_common::tfhe_ops::perform_fhe_operation;
use fhevm_engine_common::types::{get_ct_type, Handle, SupportedFheCiphertexts};
use fhevm_engine_common::utils::HeartBeat;
use std::collections::{HashMap, HashSet};
use tfhe::ReRandomizationContext;
use tokio::task::JoinSet;
use tracing::{debug, error, info, warn};

use super::{DFComponentGraph, DFGraph, OpEdge, OpNode};

const OPERATION_RERANDOMISATION_DOMAIN_SEPARATOR: [u8; 8] = *b"TFHE_Rrd";
const COMPACT_PUBLIC_ENCRYPTION_DOMAIN_SEPARATOR: [u8; 8] = *b"TFHE_Enc";
/// Separates a rooted-function transcript from a per-operation one, so the two
/// re-randomization scopes can never derive the same seeds for the same handle.
const ROOTED_FUNCTION_TRANSCRIPT_TAG: &[u8] = b"rooted-subdag-v1";

/// Which function a re-randomization seed is bound to (RFC 019).
///
/// CONSENSUS PARAMETER, NOT A PER-NODE TUNABLE. The scope and its cutoff
/// select which function description enters the seed transcript, so they
/// change computed ciphertext bytes. Every coprocessor in a deployment must
/// run the same setting; a fleet that disagrees splits consensus. The default
/// is the normative per-operation rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RerandScope {
    /// Normative RFC 019 rule: each operation re-randomizes all of its own
    /// encrypted operands, bound to that operation's output handle.
    PerOperation,
    /// Optional output-rooted rule: for each materialized output, the function
    /// is its backward dependency closure within the transaction.
    ///
    /// Its distinct boundary leaves are re-randomized once under one transcript
    /// bound to the root handle; internal edges carry raw wires.
    ///
    /// Applied to a transaction only when its per-root closures share at most
    /// `max_shared_intermediates` nodes; above the cutoff the transaction falls
    /// back to `PerOperation`, because overlapping closures must recompute the
    /// shared nodes once per root. `0` means "only when the closures are
    /// disjoint", i.e. never recompute. The cutoff is evaluated on the
    /// transaction's own dataflow graph, which is consensus data, so the
    /// decision is identical on every node running the same setting.
    OutputRootedSubDag { max_shared_intermediates: usize },
}

pub enum PartitionStrategy {
    MaxParallelism,
    MaxLocality,
}

enum DeviceSelection {
    #[allow(dead_code)]
    Index(usize),
    RoundRobin,
    #[allow(dead_code)]
    NA,
}

pub struct Scheduler<'a> {
    graph: &'a mut DFComponentGraph,
    edges: Dag<(), ComponentEdge>,
    #[cfg(not(feature = "gpu"))]
    sks: tfhe::ServerKey,
    cpk: tfhe::CompactPublicKey,
    #[cfg(feature = "gpu")]
    csks: Vec<tfhe::CudaServerKey>,
    activity_heartbeat: HeartBeat,
    rerand_scope: RerandScope,
}

type PartitionResult = (HashMap<Handle, Result<TaskResult>>, NodeIndex);
impl<'a> Scheduler<'a> {
    fn is_ready_task(&self, node: &ExecNode) -> bool {
        node.dependence_counter
            .load(std::sync::atomic::Ordering::SeqCst)
            == 0
    }
    pub fn new(
        graph: &'a mut DFComponentGraph,
        #[cfg(not(feature = "gpu"))] sks: tfhe::ServerKey,
        cpk: tfhe::CompactPublicKey,
        #[cfg(feature = "gpu")] csks: Vec<tfhe::CudaServerKey>,
        activity_heartbeat: HeartBeat,
    ) -> Self {
        let edges = graph.graph.map(|_, _| (), |_, edge| *edge);
        Self {
            graph,
            edges,
            #[cfg(not(feature = "gpu"))]
            sks,
            cpk,
            #[cfg(feature = "gpu")]
            csks,
            activity_heartbeat,
            rerand_scope: RerandScope::PerOperation,
        }
    }

    /// Overrides the re-randomization scope. See [`RerandScope`]: this changes
    /// computed bytes and must be identical across the fleet.
    pub fn with_rerand_scope(mut self, rerand_scope: RerandScope) -> Self {
        self.rerand_scope = rerand_scope;
        self
    }

    pub async fn schedule(&mut self) -> Result<()> {
        let schedule_type = std::env::var("FHEVM_DF_SCHEDULE");
        match schedule_type {
            Ok(val) => match val.as_str() {
                "MAX_PARALLELISM" => {
                    self.schedule_coarse_grain(PartitionStrategy::MaxParallelism)
                        .await
                }
                "MAX_LOCALITY" => {
                    self.schedule_coarse_grain(PartitionStrategy::MaxLocality)
                        .await
                }
                unhandled => {
                    error!(target: "scheduler", { strategy = ?unhandled },
			   "Scheduling strategy does not exist");
                    info!(target: "scheduler", { },
			  "Reverting to default (generally best performance) strategy MAX_PARALLELISM");
                    self.schedule_coarse_grain(PartitionStrategy::MaxParallelism)
                        .await
                }
            },
            // Use overall best strategy as default
            #[cfg(not(feature = "gpu"))]
            _ => {
                self.schedule_coarse_grain(PartitionStrategy::MaxParallelism)
                    .await
            }
            #[cfg(feature = "gpu")]
            _ => {
                self.schedule_coarse_grain(PartitionStrategy::MaxParallelism)
                    .await
            }
        }
    }

    #[cfg(not(feature = "gpu"))]
    fn get_keys(
        &self,
        _target: DeviceSelection,
    ) -> Result<(tfhe::ServerKey, tfhe::CompactPublicKey)> {
        Ok((self.sks.clone(), self.cpk.clone()))
    }
    #[cfg(feature = "gpu")]
    fn get_keys(
        &self,
        target: DeviceSelection,
    ) -> Result<(tfhe::CudaServerKey, tfhe::CompactPublicKey)> {
        match target {
            DeviceSelection::Index(i) => {
                if i < self.csks.len() {
                    Ok((self.csks[i].clone(), self.cpk.clone()))
                } else {
                    error!(target: "scheduler", {index = ?i },
			   "Wrong device index");
                    // Instead of giving up, we'll use device 0 (which
                    // should always be safe to use) and keep making
                    // progress even if suboptimally
                    Ok((self.csks[0].clone(), self.cpk.clone()))
                }
            }
            DeviceSelection::RoundRobin => {
                static LAST: std::sync::atomic::AtomicUsize =
                    std::sync::atomic::AtomicUsize::new(0);
                // Use fetch_add to increment atomically
                let i = LAST.fetch_add(1, std::sync::atomic::Ordering::Relaxed) % self.csks.len();
                Ok((self.csks[i].clone(), self.cpk.clone()))
            }
            DeviceSelection::NA => Ok((self.csks[0].clone(), self.cpk.clone())),
        }
    }

    async fn schedule_coarse_grain(&mut self, strategy: PartitionStrategy) -> Result<()> {
        let mut execution_graph: Dag<ExecNode, ()> = Dag::default();
        match strategy {
            PartitionStrategy::MaxLocality => {
                partition_components(&self.graph.graph, &mut execution_graph)?
            }
            PartitionStrategy::MaxParallelism => {
                partition_preserving_parallelism(&self.graph.graph, &mut execution_graph)?
            }
        };
        let task_dependences = execution_graph.map(|_, _| (), |_, edge| *edge);
        // Prime the scheduler with all nodes without dependences
        let mut set: JoinSet<PartitionResult> = JoinSet::new();
        for idx in 0..execution_graph.node_count() {
            let index = NodeIndex::new(idx);
            let node = execution_graph
                .node_weight_mut(index)
                .ok_or(SchedulerError::DataflowGraphError)?;
            if self.is_ready_task(node) {
                let mut args = Vec::with_capacity(node.df_nodes.len());
                for nidx in node.df_nodes.iter() {
                    let tx = self
                        .graph
                        .graph
                        .node_weight_mut(*nidx)
                        .ok_or(SchedulerError::DataflowGraphError)?;
                    // Skip transactions that cannot complete because of
                    // missing dependences — same skip as the dependent
                    // loop below; pre-poisoned nodes are ready by
                    // construction and would otherwise execute here.
                    if tx.is_uncomputable {
                        continue;
                    }
                    args.push((
                        std::mem::take(&mut tx.graph),
                        std::mem::take(&mut tx.inputs),
                        tx.transaction_id.clone(),
                        tx.component_id,
                    ));
                }
                let (sks, cpk) = self.get_keys(DeviceSelection::RoundRobin)?;
                let parent_span = tracing::Span::current();
                let heartbeat = self.activity_heartbeat.clone();
                let rerand_scope = self.rerand_scope;
                set.spawn_blocking(move || {
                    let span_guard = parent_span.enter();
                    let result =
                        execute_partition(args, index, 0, sks, cpk, heartbeat, rerand_scope);
                    drop(span_guard);
                    result
                });
            }
        }
        while let Some(result) = set.join_next().await {
            self.activity_heartbeat.update();
            // The result contains all outputs (allowed handles)
            // computed within the finished partition. Now check the
            // outputs and update the trnsaction inputs of downstream
            // transactions
            let (sks, _cpk) = self.get_keys(DeviceSelection::RoundRobin)?;
            tfhe::set_server_key(sks);
            let result = result?;
            let task_index = result.1;
            for (handle, node_result) in result.0.into_iter() {
                // Add computed allowed handles to the graph. These
                // can be used as inputs and forwarded to subsequent,
                // dependent transactions
                self.graph.add_output(&handle, node_result, &self.edges)?;
            }
            for edge in task_dependences.edges_directed(task_index, Direction::Outgoing) {
                let dependent_task_index = edge.target();
                let dependent_task = execution_graph
                    .node_weight_mut(dependent_task_index)
                    .ok_or(SchedulerError::DataflowGraphError)?;
                dependent_task
                    .dependence_counter
                    .fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                if self.is_ready_task(dependent_task) {
                    let mut args = Vec::with_capacity(dependent_task.df_nodes.len());
                    for nidx in dependent_task.df_nodes.iter() {
                        let tx = self
                            .graph
                            .graph
                            .node_weight_mut(*nidx)
                            .ok_or(SchedulerError::DataflowGraphError)?;
                        // Skip transactions that cannot complete
                        // because of missing dependences.
                        if tx.is_uncomputable {
                            continue;
                        }
                        args.push((
                            std::mem::take(&mut tx.graph),
                            std::mem::take(&mut tx.inputs),
                            tx.transaction_id.clone(),
                            tx.component_id,
                        ));
                    }
                    let (sks, cpk) = self.get_keys(DeviceSelection::RoundRobin)?;
                    let parent_span = tracing::Span::current();
                    let heartbeat = self.activity_heartbeat.clone();
                    let rerand_scope = self.rerand_scope;
                    set.spawn_blocking(move || {
                        let span_guard = parent_span.enter();
                        let result = execute_partition(
                            args,
                            dependent_task_index,
                            0,
                            sks,
                            cpk,
                            heartbeat,
                            rerand_scope,
                        );
                        drop(span_guard);
                        result
                    });
                }
            }
        }
        Ok(())
    }
}

/// Re-randomizes an operation's encrypted operands (RFC 019). The seed
/// transcript's function description binds the operation's OUTPUT HANDLE
/// ahead of its opcode: the handle's preimage commits to the opcode, every
/// operand handle and each operand's origin, so it is the collision-resistant
/// commitment to the function being evaluated. The opcode is kept alongside
/// it, redundantly, so the function binding stays visible in the transcript.
///
/// Binding the output handle rather than any chain coordinate is what keeps
/// dynamic single assignment: two sites minting the same handle — a same-block
/// alias, a replay on a competing fork — derive the same transcript from the
/// same operands and assign that handle the same bytes, while different
/// computations mint different handles and randomize independently.
fn re_randomise_operation_inputs(
    cts: &mut [SupportedFheCiphertexts],
    result_handle: &[u8],
    opcode: i32,
    cpk: &tfhe::CompactPublicKey,
) -> Result<()> {
    let opcode_bytes = opcode.to_be_bytes();
    let mut re_rand_context = ReRandomizationContext::new(
        OPERATION_RERANDOMISATION_DOMAIN_SEPARATOR,
        [result_handle, opcode_bytes.as_slice()],
        COMPACT_PUBLIC_ENCRYPTION_DOMAIN_SEPARATOR,
    );
    for ct in cts.iter() {
        ct.add_to_re_randomization_context(&mut re_rand_context);
    }
    let mut seed_gen = re_rand_context.finalize();
    for ct in cts.iter_mut() {
        if !matches!(ct, SupportedFheCiphertexts::Scalar(_)) {
            ct.re_randomise(cpk, seed_gen.next_seed()?)?;
        }
    }
    Ok(())
}

/// A transaction's evaluation plan under [`RerandScope::OutputRootedSubDag`].
///
/// One closure per materialized (allowed) output, each the backward dependency
/// closure of that root within the transaction, stopping at boundary operands.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ClosurePlan {
    /// Roots in canonical order (by result handle), each paired with its
    /// closure in topological order.
    pub(crate) closures: Vec<(NodeIndex, Vec<NodeIndex>)>,
    /// Nodes appearing in more than one closure: they must be evaluated once
    /// per root, so they are the recomputation the cutoff bounds.
    pub(crate) shared_intermediates: usize,
    /// Extra node evaluations the plan costs versus one evaluation each.
    pub(crate) extra_evaluations: usize,
    /// Whether every node of the transaction belongs to some closure. A node
    /// outside every closure would silently never execute.
    pub(crate) covers_all_nodes: bool,
}

impl ClosurePlan {
    /// Whether this plan may be used under `scope`.
    pub(crate) fn is_admissible(&self, scope: RerandScope) -> bool {
        match scope {
            RerandScope::PerOperation => false,
            RerandScope::OutputRootedSubDag {
                max_shared_intermediates,
            } => {
                !self.closures.is_empty()
                    && self.covers_all_nodes
                    && self.shared_intermediates <= max_shared_intermediates
            }
        }
    }
}

/// Builds the output-rooted closure plan for one transaction.
///
/// `topological_order` is the transaction's own topological order; each closure
/// preserves it, so evaluating a closure in sequence is always executable.
pub(crate) fn plan_output_rooted_closures(
    graph: &Dag<OpNode, OpEdge>,
    topological_order: &[NodeIndex],
) -> ClosurePlan {
    let mut roots: Vec<NodeIndex> = graph
        .node_references()
        .filter(|(_, node)| node.is_allowed)
        .map(|(index, _)| index)
        .collect();
    // Canonical root order: by result handle. Roots are evaluated
    // independently, so this only fixes logging and evaluation sequence, but
    // determinism costs nothing here.
    roots.sort_by(|left, right| {
        let left = graph.node_weight(*left).map(|node| &node.result_handle);
        let right = graph.node_weight(*right).map(|node| &node.result_handle);
        left.cmp(&right)
    });

    let mut membership: HashMap<NodeIndex, usize> = HashMap::new();
    let mut closures: Vec<(NodeIndex, Vec<NodeIndex>)> = Vec::with_capacity(roots.len());
    for root in roots {
        let mut in_closure: HashSet<NodeIndex> = HashSet::new();
        let mut stack = vec![root];
        while let Some(current) = stack.pop() {
            if !in_closure.insert(current) {
                continue;
            }
            for edge in graph.edges_directed(current, Incoming) {
                stack.push(edge.source());
            }
        }
        for node in in_closure.iter() {
            *membership.entry(*node).or_insert(0) += 1;
        }
        // Topological order within the closure, inherited from the
        // transaction's order.
        let ordered: Vec<NodeIndex> = topological_order
            .iter()
            .copied()
            .filter(|node| in_closure.contains(node))
            .collect();
        closures.push((root, ordered));
    }

    let shared_intermediates = membership.values().filter(|count| **count > 1).count();
    let evaluations: usize = closures.iter().map(|(_, nodes)| nodes.len()).sum();
    ClosurePlan {
        shared_intermediates,
        extra_evaluations: evaluations.saturating_sub(membership.len()),
        covers_all_nodes: membership.len() == graph.node_count(),
        closures,
    }
}

type ComponentSet = Vec<(DFGraph, HashMap<Handle, Option<DFGTxInput>>, Handle, usize)>;
/// Executes a partition of whole transactions in topological order.
///
/// The transaction is the materialization boundary. Every value produced in
/// this transaction is forwarded raw to its same-transaction consumers,
/// including values which are also compressed for persistence. Values which
/// enter from another transaction are reconstructed from their canonical
/// persisted representation. The consuming handle commits to that origin, so
/// the raw and canonical forms cannot alias even when they represent the same
/// plaintext operation.
fn execute_partition(
    transactions: ComponentSet,
    task_id: NodeIndex,
    gpu_idx: usize,
    #[cfg(not(feature = "gpu"))] sks: tfhe::ServerKey,
    #[cfg(feature = "gpu")] sks: tfhe::CudaServerKey,
    cpk: tfhe::CompactPublicKey,
    activity_heartbeat: HeartBeat,
    rerand_scope: RerandScope,
) -> PartitionResult {
    tfhe::set_server_key(sks);
    let mut res: HashMap<Handle, Result<TaskResult>> = HashMap::with_capacity(transactions.len());
    // Traverse transactions within the partition. The transactions
    // are topologically sorted so the order is executable
    'tx: for (ref mut dfg, ref mut tx_inputs, tid, _cid) in transactions {
        let txn_id_short = telemetry::short_hex_id(&tid);

        // Update the transaction inputs based on allowed handles so
        // far. If any input is still missing, and we cannot fill it
        // (e.g., error in the producer transaction) we cannot execute
        // this transaction and possibly more downstream.
        for (h, i) in tx_inputs.iter_mut() {
            if i.is_none() {
                let Some(Ok(ct)) = res.get(h) else {
                    warn!(target: "scheduler", {transaction_id = ?hex::encode(tid) },
		       "Missing input to compute transaction - skipping");
                    for nidx in dfg.graph.node_identifiers() {
                        let Some(node) = dfg.graph.node_weight_mut(nidx) else {
                            error!(target: "scheduler", {index = ?nidx.index() }, "Wrong dataflow graph index");
                            continue;
                        };
                        if node.is_allowed {
                            res.insert(
                                node.result_handle.clone(),
                                Err(SchedulerError::MissingInputs.into()),
                            );
                        }
                    }
                    continue 'tx;
                };
                *i = Some(DFGTxInput::Compressed((
                    ct.compressed_ct.clone(),
                    ct.is_allowed,
                )));
            }
        }

        // Prime the scheduler with ready ops from the transaction's subgraph
        let _exec_guard = tracing::info_span!(
            "execute_transaction",
            txn_id = %txn_id_short,
        )
        .entered();
        let started_at = std::time::Instant::now();

        let Ok(ts) = daggy::petgraph::algo::toposort(&dfg.graph, None) else {
            error!(target: "scheduler", {transaction_id = ?tid },
		       "Cyclical dependence error in transaction");
            for nidx in dfg.graph.node_identifiers() {
                let Some(node) = dfg.graph.node_weight_mut(nidx) else {
                    error!(target: "scheduler", {index = ?nidx.index() }, "Wrong dataflow graph index");
                    continue;
                };
                if node.is_allowed {
                    res.insert(
                        node.result_handle.clone(),
                        Err(SchedulerError::CyclicDependence.into()),
                    );
                }
            }
            continue 'tx;
        };
        // Optional rooted scope (RFC 019): admissible only when the
        // transaction's own closures stay within the configured recomputation
        // cutoff. The plan is derived from the dataflow graph, which is
        // consensus data, so every node running the same setting decides
        // identically.
        if !matches!(rerand_scope, RerandScope::PerOperation) {
            let plan = plan_output_rooted_closures(&dfg.graph, &ts);
            if plan.is_admissible(rerand_scope) {
                info!(target: "scheduler",
                    { txn_id = %txn_id_short, roots = plan.closures.len(),
                      shared_intermediates = plan.shared_intermediates,
                      extra_evaluations = plan.extra_evaluations },
                    "Evaluating transaction under the output-rooted re-randomization scope");
                execute_transaction_rooted(
                    dfg,
                    &plan,
                    tx_inputs,
                    &mut res,
                    gpu_idx,
                    &tid,
                    &cpk,
                    &activity_heartbeat,
                );
                drop(_exec_guard);
                let elapsed = started_at.elapsed();
                FHE_BATCH_LATENCY_HISTOGRAM.observe(elapsed.as_secs_f64());
                continue 'tx;
            }
            debug!(target: "scheduler",
                { txn_id = %txn_id_short, roots = plan.closures.len(),
                  shared_intermediates = plan.shared_intermediates,
                  covers_all_nodes = plan.covers_all_nodes },
                "Rooted re-randomization scope not admissible; using the per-operation rule");
        }
        let edges = dfg.graph.map(|_, _| (), |_, edge| *edge);
        for nidx in ts.iter() {
            let Some(node) = dfg.graph.node_weight_mut(*nidx) else {
                error!(target: "scheduler", {index = ?nidx.index() }, "Wrong dataflow graph index");
                continue;
            };
            let result = try_execute_node(node, nidx.index(), tx_inputs, gpu_idx, &tid, &cpk);
            // Per-op progress tick: a partition can legitimately run longer
            // than both the heartbeat freshness window and the in-flight
            // batch TTL; liveness must track op completions, not partition
            // completions, so only a genuinely wedged op exhausts the TTL.
            activity_heartbeat.update();
            match result {
                Ok((node_index, op_result)) => {
                    let nidx = NodeIndex::new(node_index);
                    let Some(node) = dfg.graph.node_weight(nidx) else {
                        error!(target: "scheduler", {index = ?nidx.index() }, "Wrong dataflow graph index");
                        continue;
                    };
                    let handle = node.result_handle.clone();
                    let is_allowed = node.is_allowed;
                    let opcode = node.opcode;
                    match op_result {
                        Ok(working) => {
                            // Each consumer's representation of this output
                            // is pinned on chain: the executor folded a
                            // boundary bit per operand into the consuming
                            // handle, zero for operands minted in the
                            // consuming transaction. Every in-graph edge here
                            // is by definition that case, so all of them
                            // forward the raw working value — no
                            // compress/decompress round-trip for
                            // same-transaction consumers, and no byte-equality
                            // obligation against differently-sourced aliases,
                            // which now mint different handles.
                            //
                            // An output is compressed iff it is allowed:
                            // persistence needs the bytes, and any
                            // cross-transaction consumer must have been
                            // granted a persistent allowance first (transient
                            // allowances are transaction-scoped), so
                            // cross-transaction consumers need no separate
                            // tracking. `computations.is_allowed` is always
                            // stamped by the compute block's own ACL events
                            // at ingest: ACL.allow requires an already
                            // allowed sender, and before a handle's first
                            // persistent grant the only access is transient
                            // (transaction-scoped), seeded unguarded only by
                            // the executor at mint/verifyInput and by the
                            // bridge at delivery. So the first persistent
                            // allow always lands in a transaction that
                            // minted the handle — the original mint, a
                            // re-mint of the same spelling (which inserts
                            // and stamps its own row), or a bridge delivery
                            // (no listener-stamped computation rows; the
                            // ingest allow set covers bridge dst handles) —
                            // and an allow can never arrive later for a
                            // handle that was not already persisted.
                            let forwarded = if is_allowed {
                                match compress_output(&working, &tid, opcode) {
                                    Ok(compressed_ct) => {
                                        res.insert(
                                            handle,
                                            Ok(TaskResult {
                                                compressed_ct,
                                                is_allowed,
                                                transaction_id: tid.clone(),
                                            }),
                                        );
                                        Some(working)
                                    }
                                    Err(e) => {
                                        // The block fails on this allowed
                                        // handle anyway; forward nothing so
                                        // downstream ops fail as missing
                                        // inputs instead of computing results
                                        // destined to be discarded.
                                        res.insert(handle, Err(e));
                                        None
                                    }
                                }
                            } else {
                                Some(working)
                            };
                            if let Some(forwarded) = forwarded {
                                for edge in edges.edges_directed(nidx, Direction::Outgoing) {
                                    let child_index = edge.target();
                                    let Some(child_node) = dfg.graph.node_weight_mut(child_index)
                                    else {
                                        error!(target: "scheduler", {index = ?child_index.index() }, "Wrong dataflow graph index");
                                        continue;
                                    };
                                    child_node.inputs[*edge.weight() as usize] =
                                        DFGTaskInput::Value(forwarded.clone());
                                }
                            }
                        }
                        Err(e) => {
                            res.insert(handle, Err(e));
                        }
                    }
                }
                Err(e) => {
                    let Some(node) = dfg.graph.node_weight(*nidx) else {
                        error!(target: "scheduler", {index = ?nidx.index() }, "Wrong dataflow graph index");
                        continue;
                    };
                    if node.is_allowed {
                        res.insert(node.result_handle.clone(), Err(e));
                    }
                }
            }
        }
        drop(_exec_guard);
        let elapsed = started_at.elapsed();
        FHE_BATCH_LATENCY_HISTOGRAM.observe(elapsed.as_secs_f64());
    }
    (res, task_id)
}

/// Evaluates one transaction under [`RerandScope::OutputRootedSubDag`].
///
/// For each root, the closure's DISTINCT boundary leaves are re-randomized
/// once from a single transcript bound to the root handle, then the closure is
/// evaluated with raw internal wires. A leaf feeding several closure nodes is
/// one input of the rooted function, so it contributes one re-randomization —
/// that, plus raw internal edges, is what the scope saves over the
/// per-operation rule.
///
/// Working wires are context-scoped: the value computed for an intermediate
/// handle inside this root's closure is NOT that handle's canonical value and
/// is never published. An intermediate that is itself materialized is a root
/// of its own closure and is evaluated (and persisted) there.
#[allow(clippy::too_many_arguments)]
fn execute_transaction_rooted(
    dfg: &DFGraph,
    plan: &ClosurePlan,
    tx_inputs: &HashMap<Handle, Option<DFGTxInput>>,
    res: &mut HashMap<Handle, Result<TaskResult>>,
    gpu_idx: usize,
    transaction_id: &Handle,
    cpk: &tfhe::CompactPublicKey,
    activity_heartbeat: &HeartBeat,
) {
    for (root, closure) in plan.closures.iter() {
        let Some(root_node) = dfg.graph.node_weight(*root) else {
            error!(target: "scheduler", { index = ?root.index() }, "Wrong dataflow graph index");
            continue;
        };
        let root_handle = root_node.result_handle.clone();
        let root_opcode = root_node.opcode;

        match evaluate_closure(
            dfg,
            *root,
            closure,
            tx_inputs,
            gpu_idx,
            transaction_id,
            &root_handle,
            cpk,
            activity_heartbeat,
        ) {
            Ok(working) => match compress_output(&working, transaction_id, root_opcode) {
                Ok(compressed_ct) => {
                    res.insert(
                        root_handle,
                        Ok(TaskResult {
                            compressed_ct,
                            is_allowed: true,
                            transaction_id: transaction_id.clone(),
                        }),
                    );
                }
                Err(e) => {
                    res.insert(root_handle, Err(e));
                }
            },
            Err(e) => {
                res.insert(root_handle, Err(e));
            }
        }
    }
}

/// Re-randomizes a closure's distinct boundary leaves under one root-bound
/// transcript, then evaluates the closure and returns the root's raw value.
#[allow(clippy::too_many_arguments)]
fn evaluate_closure(
    dfg: &DFGraph,
    root: NodeIndex,
    closure: &[NodeIndex],
    tx_inputs: &HashMap<Handle, Option<DFGTxInput>>,
    gpu_idx: usize,
    transaction_id: &Handle,
    root_handle: &Handle,
    cpk: &tfhe::CompactPublicKey,
    activity_heartbeat: &HeartBeat,
) -> Result<SupportedFheCiphertexts> {
    // Operand slots of the closure, split into internal edges and leaves. A
    // slot is internal exactly when an in-closure producer feeds it.
    let mut internal: HashMap<(NodeIndex, usize), NodeIndex> = HashMap::new();
    let in_closure: HashSet<NodeIndex> = closure.iter().copied().collect();
    for node in closure.iter() {
        for edge in dfg.graph.edges_directed(*node, Incoming) {
            if in_closure.contains(&edge.source()) {
                internal.insert((*node, *edge.weight() as usize), edge.source());
            }
        }
    }

    // Distinct encrypted boundary handles of the closure, in canonical handle
    // order. Ordering fixes which counter-indexed seed each leaf receives, so
    // it is consensus-critical and must not depend on graph traversal.
    //
    // A leaf is ONE input of the rooted function however many closure nodes
    // consume it, so it is re-randomized once and that value feeds every use.
    // Together with raw internal wires, this is what the scope saves over the
    // per-operation rule. Note the noise consequence: two uses of a leaf now
    // carry the same sample rather than independent ones, so noise growth for
    // repeated operands differs from the per-operation rule.
    let mut leaf_handles: Vec<Handle> = Vec::new();
    for node in closure.iter() {
        let Some(node_ref) = dfg.graph.node_weight(*node) else {
            return Err(SchedulerError::DataflowGraphError.into());
        };
        for (position, input) in node_ref.inputs.iter().enumerate() {
            if internal.contains_key(&(*node, position)) {
                continue;
            }
            match input {
                DFGTaskInput::BoundaryDependence(handle) => {
                    if !leaf_handles.contains(handle) {
                        leaf_handles.push(handle.clone());
                    }
                }
                // Scalars are plaintext: never re-randomized, and the handle
                // preimage commits to them.
                DFGTaskInput::Value(SupportedFheCiphertexts::Scalar(_)) => {}
                // A leaf can only be an unresolved boundary dependence or a
                // scalar constant at this point; anything else means the graph
                // was already partly executed, which this scope cannot honour.
                _ => {
                    error!(target: "scheduler",
                        { handle = ?hex::encode(root_handle), position },
                        "Unexpected pre-resolved leaf operand in rooted closure");
                    return Err(SchedulerError::DataflowGraphError.into());
                }
            }
        }
    }
    leaf_handles.sort();

    let mut leaves: Vec<SupportedFheCiphertexts> = Vec::with_capacity(leaf_handles.len());
    for handle in leaf_handles.iter() {
        let Some(Some(input)) = tx_inputs.get(handle) else {
            error!(target: "scheduler", { handle = ?hex::encode(handle) },
                "Missing transaction input for rooted closure leaf");
            return Err(SchedulerError::MissingInputs.into());
        };
        // Same transaction-level invariant as the per-operation path: a
        // `Value` here is the decompressed canonical form (GPU boundary
        // materialization), never a raw working value from another node.
        let base = match input {
            DFGTxInput::Value((value, _)) => DFGTaskInput::Value(value.clone()),
            DFGTxInput::Compressed((compressed, _)) => DFGTaskInput::Compressed(compressed.clone()),
        };
        leaves.push(resolve_base_ciphertext(base, root_handle, gpu_idx)?);
    }

    {
        let _guard = tracing::info_span!("rerandomise_rooted_leaves").entered();
        let started_at = std::time::Instant::now();
        if let Err(e) = re_randomise_rooted_function_inputs(&mut leaves, root_handle, cpk) {
            error!(target: "scheduler", { handle = ?hex::encode(root_handle), error = ?e },
                   "Error while re-randomising rooted function inputs");
            telemetry::set_current_span_error(&e);
            return Err(SchedulerError::ReRandomisationError.into());
        }
        let elapsed = started_at.elapsed();
        RERAND_LATENCY_BATCH_HISTOGRAM.observe(elapsed.as_secs_f64());
    }
    let leaf_values: HashMap<&Handle, &SupportedFheCiphertexts> =
        leaf_handles.iter().zip(leaves.iter()).collect();

    // Evaluate the closure in topological order with raw internal wires.
    let mut wires: HashMap<NodeIndex, SupportedFheCiphertexts> = HashMap::new();
    for node in closure.iter() {
        let Some(node_ref) = dfg.graph.node_weight(*node) else {
            return Err(SchedulerError::DataflowGraphError.into());
        };
        let opcode = node_ref.opcode;
        let result_handle = node_ref.result_handle.clone();
        let mut cts = Vec::with_capacity(node_ref.inputs.len());
        for (position, input) in node_ref.inputs.iter().enumerate() {
            if let Some(producer) = internal.get(&(*node, position)) {
                let Some(wire) = wires.get(producer) else {
                    return Err(SchedulerError::DataflowGraphError.into());
                };
                cts.push(wire.clone());
                continue;
            }
            match input {
                DFGTaskInput::BoundaryDependence(handle) => {
                    let Some(value) = leaf_values.get(handle) else {
                        return Err(SchedulerError::DataflowGraphError.into());
                    };
                    cts.push((*value).clone());
                }
                DFGTaskInput::Value(scalar @ SupportedFheCiphertexts::Scalar(_)) => {
                    cts.push(scalar.clone());
                }
                _ => return Err(SchedulerError::DataflowGraphError.into()),
            }
        }
        let (_, op_result) = execute_operation(
            opcode,
            cts,
            node.index(),
            gpu_idx,
            transaction_id,
            &result_handle,
        )?;
        // Per-op progress tick, as in the per-operation path: liveness must
        // track op completions, not whole-closure completions.
        activity_heartbeat.update();
        wires.insert(*node, op_result?);
    }

    wires
        .remove(&root)
        .ok_or_else(|| SchedulerError::DataflowGraphError.into())
}

/// Re-randomizes the distinct boundary leaves of a rooted function. The
/// transcript's function description is the rooted-scope tag followed by the
/// root handle, which commits to the complete rooted function — its
/// operations, operands, constants and operand origins — so changing anything
/// downstream changes every leaf seed and every internal wire. The tag keeps
/// this transcript disjoint from the per-operation one.
fn re_randomise_rooted_function_inputs(
    leaves: &mut [SupportedFheCiphertexts],
    root_handle: &[u8],
    cpk: &tfhe::CompactPublicKey,
) -> Result<()> {
    if leaves.is_empty() {
        return Ok(());
    }
    let mut re_rand_context = ReRandomizationContext::new(
        OPERATION_RERANDOMISATION_DOMAIN_SEPARATOR,
        [ROOTED_FUNCTION_TRANSCRIPT_TAG, root_handle],
        COMPACT_PUBLIC_ENCRYPTION_DOMAIN_SEPARATOR,
    );
    for ct in leaves.iter() {
        ct.add_to_re_randomization_context(&mut re_rand_context);
    }
    let mut seed_gen = re_rand_context.finalize();
    for ct in leaves.iter_mut() {
        if !matches!(ct, SupportedFheCiphertexts::Scalar(_)) {
            ct.re_randomise(cpk, seed_gen.next_seed()?)?;
        }
    }
    Ok(())
}

/// Turns one resolved operand into its concrete base ciphertext: a raw
/// same-transaction value or a scalar is taken as is, a canonical boundary
/// value is decompressed. Unresolved dependences are a scheduling bug.
fn resolve_base_ciphertext(
    input: DFGTaskInput,
    result_handle: &Handle,
    gpu_idx: usize,
) -> Result<SupportedFheCiphertexts> {
    match input {
        // Scalars, or raw working values forwarded from a producer in the SAME
        // transaction (the materialization boundary). A raw value crossing a
        // transaction boundary is flagged where transaction-level inputs are
        // resolved (check_ready_inputs).
        DFGTaskInput::Value(v) => Ok(v),
        DFGTaskInput::Compressed(cct) => {
            SupportedFheCiphertexts::decompress(cct.ct_type, &cct.ct_bytes, gpu_idx).map_err(|e| {
                error!(
                    target: "scheduler",
                    { handle = ?hex::encode(result_handle), ct_type = cct.ct_type, error = ?e },
                    "Error while decompressing op input"
                );
                telemetry::set_current_span_error(&e);
                SchedulerError::DecompressionError.into()
            })
        }
        DFGTaskInput::LocalDependence(_) | DFGTaskInput::BoundaryDependence(_) => {
            error!(target: "scheduler", { handle = ?hex::encode(result_handle) }, "Computation missing inputs");
            Err(SchedulerError::MissingInputs.into())
        }
    }
}

/// Runs one FHE operation with its operands already re-randomized, containing
/// a panic in the operation as an `ExecutionPanic` result for this node alone.
fn execute_operation(
    opcode: i32,
    cts: Vec<SupportedFheCiphertexts>,
    node_index: usize,
    gpu_idx: usize,
    transaction_id: &Handle,
    result_handle: &Handle,
) -> Result<(usize, OpResult)> {
    let output_type = get_ct_type(result_handle).map_err(|e| {
        error!(target: "scheduler", { handle = ?hex::encode(result_handle), error = ?e },
               "Invalid result handle: cannot read type byte");
        telemetry::set_current_span_error(&e);
        SchedulerError::SchedulerError
    })?;
    let result = std::panic::catch_unwind(|| {
        run_computation(
            opcode,
            cts,
            node_index,
            gpu_idx,
            transaction_id,
            output_type,
        )
    });
    match result {
        Err(e) => {
            let msg = panic_message(e);
            eprintln!("Panic while executing operation: {msg}");
            error!(target: "scheduler", { handle = ?hex::encode(result_handle), msg },
               "Panic while executing operation");
            telemetry::set_current_span_error(&msg);
            Err(SchedulerError::ExecutionPanic(msg).into())
        }
        Ok(r) => Ok(r),
    }
}

fn try_execute_node(
    node: &mut OpNode,
    node_index: usize,
    tx_inputs: &mut HashMap<Handle, Option<DFGTxInput>>,
    gpu_idx: usize,
    transaction_id: &Handle,
    cpk: &tfhe::CompactPublicKey,
) -> Result<(usize, OpResult)> {
    if !node.check_ready_inputs(tx_inputs) {
        return Err(SchedulerError::SchedulerError.into());
    }
    let mut cts = Vec::with_capacity(node.inputs.len());
    for i in std::mem::take(&mut node.inputs) {
        cts.push(resolve_base_ciphertext(i, &node.result_handle, gpu_idx)?);
    }
    // Re-randomize inputs for this operation
    {
        let _guard = tracing::info_span!("rerandomise_op_inputs").entered();
        let started_at = std::time::Instant::now();
        if let Err(e) =
            re_randomise_operation_inputs(&mut cts, &node.result_handle, node.opcode, cpk)
        {
            error!(target: "scheduler", { handle = ?hex::encode(&node.result_handle), error = ?e },
                   "Error while re-randomising operation inputs");
            telemetry::set_current_span_error(&e);
            return Err(SchedulerError::ReRandomisationError.into());
        }
        let elapsed = started_at.elapsed();
        RERAND_LATENCY_BATCH_HISTOGRAM.observe(elapsed.as_secs_f64());
    }
    execute_operation(
        node.opcode,
        cts,
        node_index,
        gpu_idx,
        transaction_id,
        &node.result_handle,
    )
}

fn panic_message(e: Box<dyn std::any::Any + Send>) -> String {
    e.downcast_ref::<&str>()
        .map(|s| s.to_string())
        .or_else(|| e.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "unknown panic payload".to_string())
}

type OpResult = Result<SupportedFheCiphertexts>;

/// Materializes an operation output that leaves its transaction: allowed
/// handles (persisted) and inputs of other transactions both read this
/// canonical compressed form — as do the producer's own same-transaction
/// consumers, so an aliased producer elsewhere converges byte-identically.
fn compress_output(
    working: &SupportedFheCiphertexts,
    transaction_id: &Handle,
    operation: i32,
) -> Result<CompressedCiphertext> {
    let _guard = tracing::info_span!(
        "compress_ciphertext",
        txn_id = %telemetry::short_hex_id(transaction_id),
        ct_type = working.type_name(),
        operation = FheOperation::try_from(operation)
            .map(|op| op.as_str_name())
            .unwrap_or("unknown"),
        compressed_size = tracing::field::Empty,
    )
    .entered();
    let ct_type = working.type_num();
    // Compression panics get the same per-op containment as op execution
    // (on main, compression ran inside run_computation's catch_unwind; it
    // must not regress into a whole-partition abort now that it lives
    // here): the panic becomes an ExecutionPanic result for this handle
    // alone. AssertUnwindSafe is sound because the caller's error path
    // forwards nothing and drops `working`, so no state that crossed the
    // unwind boundary is observed afterwards.
    let compressed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| working.compress()));
    let ct_bytes = match compressed {
        Ok(compress_result) => compress_result.inspect_err(|error| {
            telemetry::set_current_span_error(error);
        })?,
        Err(e) => {
            let msg = panic_message(e);
            error!(target: "scheduler", { txn_id = %telemetry::short_hex_id(transaction_id), msg },
                "Panic while compressing operation output");
            telemetry::set_current_span_error(&msg);
            return Err(SchedulerError::ExecutionPanic(msg).into());
        }
    };
    tracing::Span::current().record("compressed_size", ct_bytes.len() as i64);
    Ok(CompressedCiphertext { ct_type, ct_bytes })
}

fn run_computation(
    operation: i32,
    inputs: Vec<SupportedFheCiphertexts>,
    graph_node_index: usize,
    gpu_idx: usize,
    transaction_id: &Handle,
    output_type: i16,
) -> (usize, OpResult) {
    let txn_id_short = telemetry::short_hex_id(transaction_id);
    let op = FheOperation::try_from(operation);
    match op {
        Ok(FheOperation::FheGetCiphertext) => match inputs.into_iter().next() {
            Some(ct) => (graph_node_index, Ok(ct)),
            None => (graph_node_index, Err(SchedulerError::MissingInputs.into())),
        },
        Ok(fhe_op) => {
            let op_name = fhe_op.as_str_name();

            // FHE operation span
            let _fhe_guard = tracing::info_span!(
                "fhe_operation",
                txn_id = %txn_id_short,
                operation = op_name,
                operation_code = operation as i64,
                input_type = tracing::field::Empty,
            )
            .entered();
            if !inputs.is_empty() {
                tracing::Span::current().record("input_type", inputs[0].type_name());
            }

            let result = perform_fhe_operation(operation as i16, &inputs, gpu_idx, output_type);

            match result {
                Ok(result) => (graph_node_index, Ok(result)),
                Err(e) => {
                    telemetry::set_current_span_error(&e);
                    (graph_node_index, Err(e.into()))
                }
            }
        }
        Err(e) => (graph_node_index, Err(e.into())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dfg::{build_component_nodes, DFGOp};
    use fhevm_engine_common::types::SupportedFheOperations;

    fn handle(byte: u8) -> Handle {
        vec![byte; 32]
    }

    fn op(output: u8, input: DFGTaskInput, is_allowed: bool) -> DFGOp {
        DFGOp {
            output_handle: handle(output),
            fhe_op: SupportedFheOperations::FheNot,
            inputs: vec![input],
            is_allowed,
        }
    }

    fn plan_for(operations: Vec<DFGOp>) -> ClosurePlan {
        let (components, _) =
            build_component_nodes(operations, &handle(0xFF)).expect("valid transaction graph");
        let graph = &components[0].graph.graph;
        let order = daggy::petgraph::algo::toposort(graph, None).expect("acyclic");
        plan_output_rooted_closures(graph, &order)
    }

    fn disjoint_operations() -> Vec<DFGOp> {
        // Two independent chains, each ending in a materialized output:
        // 0x01 -> 0x02 (allowed) and 0x03 -> 0x04 (allowed).
        vec![
            op(0x01, DFGTaskInput::BoundaryDependence(handle(0xA0)), false),
            op(0x02, DFGTaskInput::LocalDependence(handle(0x01)), true),
            op(0x03, DFGTaskInput::BoundaryDependence(handle(0xA1)), false),
            op(0x04, DFGTaskInput::LocalDependence(handle(0x03)), true),
        ]
    }

    /// Disjoint closures cost no recomputation, so the rooted scope applies
    /// even at the strictest cutoff.
    #[test]
    fn disjoint_closures_need_no_recomputation() {
        let plan = plan_for(disjoint_operations());
        assert_eq!(
            plan.closures.len(),
            2,
            "one closure per materialized output"
        );
        assert_eq!(plan.shared_intermediates, 0);
        assert_eq!(plan.extra_evaluations, 0);
        assert!(plan.covers_all_nodes);
        assert!(plan.is_admissible(RerandScope::OutputRootedSubDag {
            max_shared_intermediates: 0
        }));
    }

    /// A fan-out intermediate lands in both roots' closures, so it is
    /// evaluated once per root. The cutoff is what decides whether that
    /// recomputation is worth taking.
    #[test]
    fn shared_intermediate_is_counted_and_gated_by_the_cutoff() {
        let plan = plan_for(vec![
            op(0x01, DFGTaskInput::BoundaryDependence(handle(0xA0)), false),
            op(0x02, DFGTaskInput::LocalDependence(handle(0x01)), true),
            op(0x03, DFGTaskInput::LocalDependence(handle(0x01)), true),
        ]);
        assert_eq!(plan.closures.len(), 2);
        assert_eq!(plan.shared_intermediates, 1, "0x01 feeds both roots");
        assert_eq!(plan.extra_evaluations, 1);
        assert!(!plan.is_admissible(RerandScope::OutputRootedSubDag {
            max_shared_intermediates: 0
        }));
        assert!(plan.is_admissible(RerandScope::OutputRootedSubDag {
            max_shared_intermediates: 1
        }));
    }

    /// A materialized handle consumed by a later materialized handle is a root
    /// of its own closure AND an internal wire of the consumer's, so it is
    /// recomputed there — the canonical value and the working wire are
    /// different ciphertexts.
    #[test]
    fn materialized_intermediate_is_recomputed_inside_its_consumer() {
        let plan = plan_for(vec![
            op(0x01, DFGTaskInput::BoundaryDependence(handle(0xA0)), true),
            op(0x02, DFGTaskInput::LocalDependence(handle(0x01)), true),
        ]);
        assert_eq!(plan.closures.len(), 2);
        assert_eq!(plan.shared_intermediates, 1);
        assert_eq!(plan.extra_evaluations, 1);
        let (_, consumer_closure) = plan
            .closures
            .iter()
            .max_by_key(|(_, closure)| closure.len())
            .expect("two closures");
        assert_eq!(
            consumer_closure.len(),
            2,
            "the consumer's closure re-evaluates its materialized producer"
        );
    }

    /// Closures are listed in canonical root order, and each closure ends at
    /// its root, so evaluation order and leaf indexing are deterministic.
    #[test]
    fn closures_are_canonically_ordered() {
        let (components, _) = build_component_nodes(disjoint_operations(), &handle(0xFF))
            .expect("valid transaction graph");
        let graph = &components[0].graph.graph;
        let order = daggy::petgraph::algo::toposort(graph, None).expect("acyclic");
        let plan = plan_output_rooted_closures(graph, &order);

        let root_handles: Vec<Handle> = plan
            .closures
            .iter()
            .map(|(root, _)| {
                graph
                    .node_weight(*root)
                    .expect("root node")
                    .result_handle
                    .clone()
            })
            .collect();
        let mut canonical = root_handles.clone();
        canonical.sort();
        assert_eq!(
            root_handles, canonical,
            "roots must be visited in canonical handle order"
        );
        for (root, closure) in plan.closures.iter() {
            assert_eq!(
                closure.last(),
                Some(root),
                "a closure is topologically ordered, so it ends at its root"
            );
        }
    }

    /// The normative scope never takes the rooted path.
    #[test]
    fn per_operation_scope_is_never_admissible() {
        let plan = plan_for(disjoint_operations());
        assert!(!plan.is_admissible(RerandScope::PerOperation));
    }
}
