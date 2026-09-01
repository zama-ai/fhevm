use crate::{
    dfg::{
        partition_components, partition_preserving_parallelism, types::*, ComponentEdge, ExecNode,
    },
    FHE_BATCH_LATENCY_HISTOGRAM, RERAND_LATENCY_BATCH_HISTOGRAM,
};
use anyhow::Result;
use daggy::{
    petgraph::{
        visit::{EdgeRef, IntoEdgesDirected, IntoNodeIdentifiers},
        Direction::{self},
    },
    Dag, NodeIndex,
};
use fhevm_engine_common::common::FheOperation;
use fhevm_engine_common::telemetry;
use fhevm_engine_common::tfhe_ops::perform_fhe_operation;
use fhevm_engine_common::types::{get_ct_type, Handle, SupportedFheCiphertexts};
use fhevm_engine_common::utils::HeartBeat;
use std::collections::HashMap;
#[cfg(feature = "gpu")]
use std::sync::Arc;
use std::time::Duration;
use tfhe::ReRandomizationContext;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

use super::{DFComponentGraph, DFGraph, OpNode};

const OPERATION_RERANDOMISATION_DOMAIN_SEPARATOR: [u8; 8] = *b"TFHE_Rrd";
const COMPACT_PUBLIC_ENCRYPTION_DOMAIN_SEPARATOR: [u8; 8] = *b"TFHE_Enc";

/// Process-wide GPU stream capacity shared by every concurrently scheduled
/// batch. A scheduler-local bound alone would be insufficient if several
/// schedulers ever run at once, and partitions must not oversubscribe a
/// device's streams.
#[cfg(feature = "gpu")]
#[derive(Clone)]
pub struct GpuExecutionLimiter {
    devices: Arc<Vec<Arc<tokio::sync::Semaphore>>>,
    streams_per_device: usize,
}

#[cfg(feature = "gpu")]
impl GpuExecutionLimiter {
    pub fn new(device_count: usize, streams_per_device: usize) -> Result<Self> {
        if device_count == 0 || streams_per_device == 0 {
            anyhow::bail!("GPU execution requires at least one device and stream");
        }
        Ok(Self {
            devices: Arc::new(
                (0..device_count)
                    .map(|_| Arc::new(tokio::sync::Semaphore::new(streams_per_device)))
                    .collect(),
            ),
            streams_per_device,
        })
    }

    pub fn total_capacity(&self) -> usize {
        self.devices.len().saturating_mul(self.streams_per_device)
    }

    pub async fn acquire(&self, device: usize) -> Result<tokio::sync::OwnedSemaphorePermit> {
        self.devices
            .get(device)
            .ok_or_else(|| anyhow::anyhow!("GPU device {device} has no execution limiter"))?
            .clone()
            .acquire_owned()
            .await
            .map_err(|_| anyhow::anyhow!("GPU execution limiter closed"))
    }
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
    /// Observed inside GPU memory reservation waits so a shutting-down
    /// worker does not keep spinning for memory it will never use.
    cancellation: CancellationToken,
    /// Upper bound on any single GPU memory reservation wait; exceeding it
    /// fails the operation instead of spinning forever while holding
    /// resources.
    gpu_reservation_timeout: Duration,
    #[cfg(not(feature = "gpu"))]
    sks: tfhe::ServerKey,
    cpk: tfhe::CompactPublicKey,
    #[cfg(feature = "gpu")]
    csks: Vec<tfhe::CudaServerKey>,
    #[cfg(feature = "gpu")]
    gpu_execution_limiter: GpuExecutionLimiter,
    activity_heartbeat: HeartBeat,
}

type PartitionResult = (HashMap<Handle, Result<TaskResult>>, NodeIndex, usize);
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
        #[cfg(feature = "gpu")] gpu_execution_limiter: GpuExecutionLimiter,
        activity_heartbeat: HeartBeat,
        cancellation: CancellationToken,
        gpu_reservation_timeout: Duration,
    ) -> Self {
        let edges = graph.graph.map(|_, _| (), |_, edge| *edge);
        Self {
            graph,
            edges,
            cancellation,
            gpu_reservation_timeout,
            #[cfg(not(feature = "gpu"))]
            sks,
            cpk,
            #[cfg(feature = "gpu")]
            csks,
            #[cfg(feature = "gpu")]
            gpu_execution_limiter,
            activity_heartbeat,
        }
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
    ) -> Result<(tfhe::ServerKey, tfhe::CompactPublicKey, usize)> {
        Ok((self.sks.clone(), self.cpk.clone(), 0))
    }
    #[cfg(feature = "gpu")]
    fn get_keys(
        &self,
        target: DeviceSelection,
    ) -> Result<(tfhe::CudaServerKey, tfhe::CompactPublicKey, usize)> {
        if self.csks.is_empty() {
            anyhow::bail!("No GPU server keys available");
        }
        match target {
            DeviceSelection::Index(i) => {
                if i < self.csks.len() {
                    Ok((self.csks[i].clone(), self.cpk.clone(), i))
                } else {
                    error!(target: "scheduler", {index = ?i },
			   "Wrong device index");
                    // Instead of giving up, we'll use device 0 (which
                    // should always be safe to use) and keep making
                    // progress even if suboptimally
                    Ok((self.csks[0].clone(), self.cpk.clone(), 0))
                }
            }
            DeviceSelection::RoundRobin => {
                static LAST: std::sync::atomic::AtomicUsize =
                    std::sync::atomic::AtomicUsize::new(0);
                // Use fetch_add to increment atomically
                let i = LAST.fetch_add(1, std::sync::atomic::Ordering::Relaxed) % self.csks.len();
                Ok((self.csks[i].clone(), self.cpk.clone(), i))
            }
            DeviceSelection::NA => Ok((self.csks[0].clone(), self.cpk.clone(), 0)),
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
                let (sks, cpk, gpu_idx) = self.get_keys(DeviceSelection::RoundRobin)?;
                // Bound concurrent partitions per device. Permits are
                // released by the blocking task itself, never by this
                // coordinator, so waiting here cannot deadlock; it only
                // paces dispatch to the configured stream capacity.
                #[cfg(feature = "gpu")]
                let gpu_permit = self.gpu_execution_limiter.acquire(gpu_idx).await?;
                let cancellation = self.cancellation.clone();
                let gpu_reservation_timeout = self.gpu_reservation_timeout;
                let parent_span = tracing::Span::current();
                let heartbeat = self.activity_heartbeat.clone();
                let dispatched_at = std::time::Instant::now();
                set.spawn_blocking(move || {
                    #[cfg(feature = "gpu")]
                    let _gpu_permit = gpu_permit;
                    let span_guard = parent_span.enter();
                    let result = execute_partition(
                        args,
                        index,
                        dispatched_at,
                        gpu_idx,
                        sks,
                        cpk,
                        &cancellation,
                        gpu_reservation_timeout,
                        heartbeat,
                    );
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
            let result = result?;
            // Install the key of the device the partition ran on: forwarded
            // results referenced below live there.
            let (sks, _cpk, _) = self.get_keys(DeviceSelection::Index(result.2))?;
            tfhe::set_server_key(sks);
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
                    let (sks, cpk, gpu_idx) = self.get_keys(DeviceSelection::RoundRobin)?;
                    #[cfg(feature = "gpu")]
                    let gpu_permit = self.gpu_execution_limiter.acquire(gpu_idx).await?;
                    let cancellation = self.cancellation.clone();
                    let gpu_reservation_timeout = self.gpu_reservation_timeout;
                    let parent_span = tracing::Span::current();
                    let heartbeat = self.activity_heartbeat.clone();
                    let dispatched_at = std::time::Instant::now();
                    set.spawn_blocking(move || {
                        #[cfg(feature = "gpu")]
                        let _gpu_permit = gpu_permit;
                        let span_guard = parent_span.enter();
                        let result = execute_partition(
                            args,
                            dependent_task_index,
                            dispatched_at,
                            gpu_idx,
                            sks,
                            cpk,
                            &cancellation,
                            gpu_reservation_timeout,
                            heartbeat,
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
#[allow(clippy::too_many_arguments)]
fn execute_partition(
    transactions: ComponentSet,
    task_id: NodeIndex,
    dispatched_at: std::time::Instant,
    gpu_idx: usize,
    #[cfg(not(feature = "gpu"))] sks: tfhe::ServerKey,
    #[cfg(feature = "gpu")] sks: tfhe::CudaServerKey,
    cpk: tfhe::CompactPublicKey,
    cancellation: &CancellationToken,
    gpu_reservation_timeout: Duration,
    activity_heartbeat: HeartBeat,
) -> PartitionResult {
    let spawned_at = std::time::Instant::now();
    tfhe::set_server_key(sks);
    let key_installed_at = std::time::Instant::now();
    let partition_tx_count = transactions.len();
    // Per-partition memo of the canonical decompressed form ct(h), permitted
    // by RFC-020 ("the worker may cache the canonical decompressed form ct(h)
    // for the duration of the transaction batch"). Without it a boundary
    // handle consumed by K ops is decompressed K times from identical bytes.
    //
    // Scoped to the PARTITION, not the batch, on purpose: a partition runs on
    // one thread with one server key installed, so entries are created on the
    // device that consumes them and need no synchronisation. A batch-wide
    // cache would have to be shared across partition threads and would pin
    // every boundary value to whichever device populated it.
    //
    // Memoization is observationally transparent — Decompress(cmp(h)) is
    // deterministic, so a hit and a miss yield the same value.
    let mut boundary_cache: HashMap<Handle, SupportedFheCiphertexts> = HashMap::new();
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
        let edges = dfg.graph.map(|_, _| (), |_, edge| *edge);
        for nidx in ts.iter() {
            let Some(node) = dfg.graph.node_weight_mut(*nidx) else {
                error!(target: "scheduler", {index = ?nidx.index() }, "Wrong dataflow graph index");
                continue;
            };
            let result = try_execute_node(
                node,
                nidx.index(),
                tx_inputs,
                gpu_idx,
                &tid,
                &cpk,
                cancellation,
                gpu_reservation_timeout,
                &mut boundary_cache,
            );
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
    tracing::info!(
        target: "scheduler",
        dispatch_us = spawned_at.duration_since(dispatched_at).as_micros() as u64,
        key_install_us = key_installed_at.duration_since(spawned_at).as_micros() as u64,
        exec_us = key_installed_at.elapsed().as_micros() as u64,
        total_us = dispatched_at.elapsed().as_micros() as u64,
        tx_count = partition_tx_count,
        "partition_hop"
    );
    // No trailing device synchronization: it would be a DEVICE-WIDE barrier
    // that couples this partition to every other in-flight partition's
    // queued kernels — on a dependency-deep workload each link then waits
    // for all sibling chains before its successor can spawn. It is also not
    // needed for correctness: (1) every value that escapes the partition is
    // host bytes produced by compress, which synchronizes the partition's
    // own streams; (2) raw and canonical forwards are consumed inside the
    // partition on the same streams, in order; (3) buffer frees are
    // stream-ordered; (4) the GPU memory reservations already release at op
    // return, so the sync never extended that accounting.
    (res, task_id, gpu_idx)
}

#[allow(clippy::too_many_arguments)]
fn try_execute_node(
    node: &mut OpNode,
    node_index: usize,
    tx_inputs: &mut HashMap<Handle, Option<DFGTxInput>>,
    gpu_idx: usize,
    transaction_id: &Handle,
    cpk: &tfhe::CompactPublicKey,
    cancellation: &CancellationToken,
    gpu_reservation_timeout: Duration,
    boundary_cache: &mut HashMap<Handle, SupportedFheCiphertexts>,
) -> Result<(usize, OpResult)> {
    if !node.check_ready_inputs(tx_inputs) {
        return Err(SchedulerError::SchedulerError.into());
    }
    let mut cts = Vec::with_capacity(node.inputs.len());
    for i in std::mem::take(&mut node.inputs) {
        match i {
            // Scalars, or raw working values forwarded from a producer in
            // the SAME transaction (the materialization boundary). A raw
            // value crossing a transaction boundary is flagged where
            // transaction-level inputs are resolved (check_ready_inputs).
            DFGTaskInput::Value(v) => {
                cts.push(v);
            }
            DFGTaskInput::Compressed(handle, cct) => {
                // ct(h) is the same value for every consumer, so a hit and a
                // miss are indistinguishable to the operation. See the memo's
                // declaration in execute_partition for the RFC-020 basis.
                if let Some(cached) = boundary_cache.get(&handle) {
                    cts.push(cached.clone());
                    continue;
                }
                // Decompression is inside catch_unwind for the same reason
                // the operation is: an allocation failure that surfaces as a
                // PANIC rather than as a reservation error would otherwise
                // escape this function, kill the spawn_blocking task, and fail
                // the WHOLE batch with nothing stamped — retried only at batch
                // granularity, unattributable, and rotating forever if it is
                // deterministic. Caught, it becomes an ExecutionPanic like any
                // other, and takes the per-computation retry path.
                let decompressed = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| {
                        SupportedFheCiphertexts::decompress(
                            cct.ct_type,
                            &cct.ct_bytes,
                            gpu_idx,
                            cancellation,
                            gpu_reservation_timeout,
                        )
                    }),
                ) {
                    Ok(result) => result,
                    Err(panic) => {
                        let msg = panic_message(panic);
                        error!(
                            target: "scheduler",
                            { handle = ?hex::encode(&node.result_handle), ct_type = cct.ct_type, panic = %msg },
                            "Panic while decompressing op input"
                        );
                        return Err(SchedulerError::ExecutionPanic(format!(
                            "decompressing boundary input: {msg}"
                        ))
                        .into());
                    }
                }
		    .map_err(|e| {
				error!(
			    target: "scheduler",
			    { handle = ?hex::encode(&node.result_handle), ct_type = cct.ct_type, error = ?e },
			    "Error while decompressing op input"
				);
				telemetry::set_current_span_error(&e);
				#[cfg(feature = "gpu")]
				if matches!(
				    e.downcast_ref::<fhevm_engine_common::types::FhevmError>(),
				    Some(
					fhevm_engine_common::types::FhevmError::GpuMemoryReservationError(_)
				    )
				) {
				    return e;
				}
				anyhow::Error::new(SchedulerError::DecompressionError)
			    })?;
                boundary_cache.insert(handle, decompressed.clone());
                cts.push(decompressed);
            }
            DFGTaskInput::LocalDependence(_) | DFGTaskInput::BoundaryDependence(_) => {
                error!(target: "scheduler", { handle = ?hex::encode(&node.result_handle) }, "Computation missing inputs");
                return Err(SchedulerError::MissingInputs.into());
            }
        }
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
    let opcode = node.opcode;
    let output_type = get_ct_type(&node.result_handle).map_err(|e| {
        error!(target: "scheduler", { handle = ?hex::encode(&node.result_handle), error = ?e },
               "Invalid result handle: cannot read type byte");
        telemetry::set_current_span_error(&e);
        SchedulerError::SchedulerError
    })?;

    // AssertUnwindSafe: the closure only reads the cancellation token and
    // owns everything else it touches; a panic cannot leave observable
    // broken state behind.
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        run_computation(
            opcode,
            cts,
            node_index,
            gpu_idx,
            transaction_id,
            output_type,
            cancellation,
            gpu_reservation_timeout,
        )
    }));
    match result {
        Err(e) => {
            let msg = panic_message(e);
            eprintln!("Panic while executing operation: {msg}");
            error!(target: "scheduler", { handle = ?hex::encode(&node.result_handle), msg },
               "Panic while executing operation");
            telemetry::set_current_span_error(&msg);
            Err(SchedulerError::ExecutionPanic(msg).into())
        }
        Ok(r) => Ok(r),
    }
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

#[allow(clippy::too_many_arguments)]
fn run_computation(
    operation: i32,
    inputs: Vec<SupportedFheCiphertexts>,
    graph_node_index: usize,
    gpu_idx: usize,
    transaction_id: &Handle,
    output_type: i16,
    cancellation: &CancellationToken,
    gpu_reservation_timeout: Duration,
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

            let result = perform_fhe_operation(
                operation as i16,
                &inputs,
                gpu_idx,
                output_type,
                cancellation,
                gpu_reservation_timeout,
            );

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
