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
use fhevm_engine_common::types::{
    get_ct_type, Handle, SupportedFheCiphertexts, SupportedFheOperations,
};
use fhevm_engine_common::utils::HeartBeat;
use std::collections::HashMap;
use std::time::Duration;
use tfhe::ReRandomizationContext;
use tokio::task::JoinSet;
use tracing::{error, info};

use super::{DFComponentGraph, DFGOutput, DFGraph, OpNode};

mod parallel;
use parallel::{execute_partition, CanonicalInputs};

const OPERATION_RERANDOMISATION_DOMAIN_SEPARATOR: [u8; 8] = *b"TFHE_Rrd";
const COMPACT_PUBLIC_ENCRYPTION_DOMAIN_SEPARATOR: [u8; 8] = *b"TFHE_Enc";

#[cfg(feature = "gpu")]
pub use crate::gpu_execution::GpuExecutionLimiter;

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

/// What a partition hands back: one entry per (output handle, transaction)
/// pair, in production order.
///
/// A `Vec` keyed by the pair rather than a `HashMap` keyed by the handle, for
/// two reasons that only appear when two transactions in one partition mint the
/// SAME handle -- same block, same operation, same operands, so the same
/// preimage. A handle-keyed map made those two collide: the second insert
/// overwrote the first, so one transaction's result vanished and its row never
/// completed. And an error carried no transaction id at all, so `add_output`
/// fell back to the first producer and could stamp the wrong row, leaving the
/// one that actually failed unstamped -- outside the retry and demotion path
/// entirely.
type PartitionOutcome = (Vec<Handle>, Handle, Result<TaskResult>);
type PartitionResult = (Vec<PartitionOutcome>, NodeIndex, usize);
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
        gpu_reservation_timeout: Duration,
    ) -> Self {
        let edges = graph.graph.map(|_, _| (), |_, edge| *edge);
        Self {
            graph,
            edges,
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
                #[cfg(feature = "gpu")]
                let limiter = self.gpu_execution_limiter.clone();
                let gpu_reservation_timeout = self.gpu_reservation_timeout;
                let parent_span = tracing::Span::current();
                let heartbeat = self.activity_heartbeat.clone();
                let dispatched_at = std::time::Instant::now();
                set.spawn(async move {
                    use tracing::Instrument;
                    execute_partition(
                        args,
                        index,
                        dispatched_at,
                        gpu_idx,
                        sks,
                        cpk,
                        gpu_reservation_timeout,
                        heartbeat,
                        #[cfg(feature = "gpu")]
                        limiter,
                    )
                    .instrument(parent_span)
                    .await
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
            for (handles, transaction_id, node_result) in result.0.into_iter() {
                // Add computed allowed handles to the graph. These
                // can be used as inputs and forwarded to subsequent,
                // dependent transactions. The transaction travels with the
                // handle: two transactions can mint the same one, and the
                // graph has to attribute this outcome to the right producer.
                self.graph
                    .add_output(&handles, &transaction_id, node_result, &self.edges)?;
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
                    let limiter = self.gpu_execution_limiter.clone();
                    let gpu_reservation_timeout = self.gpu_reservation_timeout;
                    let parent_span = tracing::Span::current();
                    let heartbeat = self.activity_heartbeat.clone();
                    let dispatched_at = std::time::Instant::now();
                    set.spawn(async move {
                        use tracing::Instrument;
                        execute_partition(
                            args,
                            dependent_task_index,
                            dispatched_at,
                            gpu_idx,
                            sks,
                            cpk,
                            gpu_reservation_timeout,
                            heartbeat,
                            #[cfg(feature = "gpu")]
                            limiter,
                        )
                        .instrument(parent_span)
                        .await
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
#[allow(clippy::too_many_arguments)]
fn try_execute_node(
    node: &mut OpNode,
    node_index: usize,
    tx_inputs: &HashMap<Handle, Option<DFGTxInput>>,
    gpu_idx: usize,
    transaction_id: &Handle,
    cpk: &tfhe::CompactPublicKey,
    gpu_reservation_timeout: Duration,
    boundary_cache: &CanonicalInputs,
) -> Result<(usize, OpResult)> {
    if !node.check_ready_inputs(tx_inputs) {
        return Err(SchedulerError::SchedulerError.into());
    }
    let handle = hex::encode(&node.outputs[0].handle);
    let outputs = node.outputs.len();
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
                // declaration in parallel::CanonicalInputs for the RFC-020 basis.
                if let Some(cached) = boundary_cache.get(&handle) {
                    cts.push(cached.as_ref().clone());
                    continue;
                }
                // Decompression is inside catch_unwind for the same reason
                // the operation is: an allocation failure that surfaces as a
                // PANIC rather than as a reservation error would otherwise
                // escape this function, kill the spawn_blocking task, and fail
                // the WHOLE batch with nothing stamped.
                let decompressed =
                    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        SupportedFheCiphertexts::decompress(
                            cct.ct_type,
                            &cct.ct_bytes,
                            gpu_idx,
                            gpu_reservation_timeout,
                        )
                    })) {
                        Ok(result) => result,
                        Err(panic) => {
                            let msg = panic_message(panic);
                            error!(
                                target: "scheduler",
                                { handle = ?handle, ct_type = cct.ct_type, panic = %msg },
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
                            { handle = ?handle, ct_type = cct.ct_type, error = ?e },
                            "Error while decompressing op input"
                        );
                        telemetry::set_current_span_error(&e);
                        #[cfg(feature = "gpu")]
                        if matches!(
                            e.downcast_ref::<fhevm_engine_common::types::FhevmError>(),
                            Some(
                                fhevm_engine_common::types::FhevmError::GpuMemoryReservationError(
                                    _
                                )
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
                error!(target: "scheduler",
                    { handle = ?handle, outputs },
                    "Computation missing inputs");
                return Err(SchedulerError::MissingInputs.into());
            }
        }
    }
    // Re-randomize inputs for this operation
    {
        let _guard = tracing::info_span!("rerandomise_op_inputs").entered();
        let started_at = std::time::Instant::now();
        // Every handle of a group derives from one base, so output 0 binds the
        // operation for the transcript.
        if let Err(e) =
            re_randomise_operation_inputs(&mut cts, &node.outputs[0].handle, node.opcode, cpk)
        {
            error!(target: "scheduler",
                { handle = ?handle, outputs, error = ?e },
                "Error while re-randomising operation inputs");
            telemetry::set_current_span_error(&e);
            return Err(SchedulerError::ReRandomisationError.into());
        }
        let elapsed = started_at.elapsed();
        RERAND_LATENCY_BATCH_HISTOGRAM.observe(elapsed.as_secs_f64());
    }
    let opcode = node.opcode;
    // One type per declared output: outputs of one operation can differ in type.
    // Terminal: the generic SchedulerError is left unstamped and retried forever.
    let output_types = node
        .outputs
        .iter()
        .enumerate()
        .map(|(index, o)| {
            get_ct_type(&o.handle).map_err(|e| {
                error!(target: "scheduler",
                    { handle = ?handle, outputs, error = ?e },
                    "Invalid result handle: cannot read type byte");
                telemetry::set_current_span_error(&e);
                SchedulerError::MultiOutputFailure(format!(
                    "output {index} has an invalid handle: {e}"
                ))
            })
        })
        .collect::<std::result::Result<Vec<i16>, _>>()?;

    // AssertUnwindSafe: the closure only reads shared state and
    // owns everything else it touches; a panic cannot leave observable
    // broken state behind.
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        run_computation(
            opcode,
            cts,
            node_index,
            gpu_idx,
            transaction_id,
            &output_types,
            gpu_reservation_timeout,
        )
    }));
    match result {
        Err(e) => {
            let msg = panic_message(e);
            eprintln!("Panic while executing operation: {msg}");
            error!(target: "scheduler",
                { handle = ?handle, outputs, msg },
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

/// Ok payload length matches the op's output handle count (1 or N). Values are
/// the raw working representation; the scheduler compresses the ones it must
/// persist.
type OpResult = Result<Vec<SupportedFheCiphertexts>>;

/// Checks that the operation produced what its handles asked for. Runs before
/// anything is routed, so a bad result fails the whole group rather than
/// half-updating the graph.
/// Checks the dispatch result against what the handles asked for, before any
/// output is compressed, published or forwarded. Runs on the raw working
/// values, so a bad result fails the whole group rather than half-updating it.
fn validate_results(
    outputs: &[DFGOutput],
    produced_types: &[i16],
) -> std::result::Result<(), SchedulerError> {
    if produced_types.len() != outputs.len() {
        return Err(SchedulerError::MultiOutputFailure(format!(
            "produced {} ciphertexts for {} handles",
            produced_types.len(),
            outputs.len()
        )));
    }
    for (index, (output, &produced)) in outputs.iter().zip(produced_types).enumerate() {
        let asked_for = get_ct_type(&output.handle).map_err(|_| {
            SchedulerError::MultiOutputFailure(format!("output {index} has an invalid handle"))
        })?;
        if produced != asked_for {
            return Err(SchedulerError::MultiOutputFailure(format!(
                "output {index} has type {produced} but was asked for {asked_for}"
            )));
        }
    }
    Ok(())
}

/// Materializes an operation output that leaves its transaction: allowed
/// handles (persisted) and inputs of other transactions both read this
/// canonical compressed form. Same-transaction consumers retain the raw
/// working value, as committed by their operand-origin bits.
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
    output_types: &[i16],
    gpu_reservation_timeout: Duration,
) -> (usize, OpResult) {
    let txn_id_short = telemetry::short_hex_id(transaction_id);

    // Multi-output ops dispatch through a separate impl that returns Vec.
    if let Ok(sup_op) = SupportedFheOperations::try_from(operation as i16) {
        if sup_op.is_multi_output() {
            let op_name = format!("{:?}", sup_op);
            let _fhe_guard = tracing::info_span!(
                "fhe_operation_multi_output",
                txn_id = %txn_id_short,
                operation = %op_name,
                operation_code = operation as i64,
            )
            .entered();

            let result = fhevm_engine_common::tfhe_ops::perform_multi_output_fhe_operation(
                operation as i16,
                &inputs,
                output_types,
                gpu_idx,
                gpu_reservation_timeout,
            );

            return match result {
                Ok(results) => (graph_node_index, Ok(results)),
                Err(e) => {
                    telemetry::set_current_span_error(&e);
                    (graph_node_index, Err(e.into()))
                }
            };
        }
    }

    let op = FheOperation::try_from(operation);
    match op {
        Ok(FheOperation::FheGetCiphertext) => match inputs.into_iter().next() {
            Some(ct) => (graph_node_index, Ok(vec![ct])),
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

            // A single-output op has exactly one declared output.
            let result = perform_fhe_operation(
                operation as i16,
                &inputs,
                gpu_idx,
                output_types[0],
                gpu_reservation_timeout,
            );

            match result {
                Ok(result) => (graph_node_index, Ok(vec![result])),
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

    /// Uses the actual production rerandomizer with fresh keys; no developer
    /// fixture or historical serialization is substituted for this build.
    #[cfg(not(feature = "gpu"))]
    #[test]
    fn rerandomization_binds_handle_opcode_and_ordered_ciphertexts() {
        use fhevm_engine_common::keys::{
            TFHE_COMPACT_PK_PARAMS, TFHE_PARAMS, TFHE_PKS_RERANDOMIZATION_PARAMS,
        };
        use tfhe::prelude::FheEncrypt;
        let config = tfhe::ConfigBuilder::with_custom_parameters(TFHE_PARAMS)
            .use_dedicated_compact_public_key_parameters((
                TFHE_COMPACT_PK_PARAMS.pke_params,
                TFHE_COMPACT_PK_PARAMS.ksk_params,
            ))
            .enable_ciphertext_re_randomization(TFHE_PKS_RERANDOMIZATION_PARAMS)
            .build();
        let client = tfhe::ClientKey::generate(config);
        let public = tfhe::CompactPublicKey::new(&client);
        tfhe::set_server_key(tfhe::ServerKey::new(&client));
        let encrypted =
            |value: u8| SupportedFheCiphertexts::FheUint8(tfhe::FheUint8::encrypt(value, &client));
        let inputs = vec![encrypted(17), encrypted(29), encrypted(43)];
        let evaluate = |operands: &[SupportedFheCiphertexts], handle: &[u8], opcode: i32| {
            let mut result = operands.to_vec();
            re_randomise_operation_inputs(&mut result, handle, opcode, &public).unwrap();
            let bytes: Vec<_> = result
                .iter()
                .map(SupportedFheCiphertexts::serialize)
                .collect();
            let values: Vec<_> = result.iter().map(|ct| ct.decrypt(&client)).collect();
            (bytes, values)
        };
        let handle = [1u8; 32];
        let baseline = evaluate(&inputs, &handle, 1);
        assert_eq!(
            baseline,
            evaluate(&inputs, &handle, 1),
            "alias/replay uses the same transcript"
        );
        assert_eq!(baseline.1, ["17", "29", "43"]);
        for changed in [
            evaluate(&inputs, &[2u8; 32], 1),
            evaluate(&inputs, &handle, 2),
        ] {
            assert_ne!(
                baseline.0, changed.0,
                "the output handle and opcode must each affect bytes"
            );
        }
        // Keep the observed ciphertext and its seed-stream position unchanged:
        // comparing the swapped outputs would differ just from their plaintexts.
        let reordered = evaluate(
            &[inputs[1].clone(), inputs[0].clone(), inputs[2].clone()],
            &handle,
            1,
        );
        assert_eq!(reordered.1, ["29", "17", "43"]);
        assert_ne!(
            baseline.0[2], reordered.0[2],
            "operand order must affect the unchanged third operand's rerandomization"
        );

        // A fresh encryption can change its own output without changing the
        // transcript seed, so observe the unchanged first operand instead.
        let replaced = evaluate(
            &[inputs[0].clone(), encrypted(29), inputs[2].clone()],
            &handle,
            1,
        );
        assert_eq!(replaced.1, baseline.1);
        assert_ne!(
            baseline.0[0], replaced.0[0],
            "operand ciphertext bytes must affect the unchanged first operand's rerandomization"
        );
        // Scalars are bound through the output handle; they are not ciphertexts
        // and must not consume a seed or be changed by rerandomization.
        let mut mixed = vec![inputs[0].clone(), SupportedFheCiphertexts::Scalar(vec![9])];
        re_randomise_operation_inputs(&mut mixed, &handle, 1, &public).unwrap();
        assert_eq!(mixed[0].decrypt(&client), "17");
        assert!(matches!(&mixed[1], SupportedFheCiphertexts::Scalar(bytes) if bytes == &[9]));
    }

    fn output(type_byte: u8) -> DFGOutput {
        let mut handle = vec![0u8; 32];
        handle[30] = type_byte;
        DFGOutput {
            handle,
            is_allowed: true,
        }
    }

    #[test]
    fn accepts_matching_count_and_types() {
        let outputs = [output(4), output(5), output(4)];
        let results = [4i16, 5, 4];
        assert!(validate_results(&outputs, &results).is_ok());
    }

    #[test]
    fn rejects_a_count_mismatch() {
        let outputs = [output(4), output(4)];
        assert!(validate_results(&outputs, &[4]).is_err());
        assert!(validate_results(&outputs[..1], &[4, 4]).is_err());
    }

    #[test]
    fn rejects_a_wrong_type_at_the_last_output() {
        let outputs = [output(4), output(5)];
        assert!(validate_results(&outputs, &[4, 4]).is_err());
    }

    #[test]
    fn rejects_an_invalid_handle() {
        let outputs = [DFGOutput {
            handle: vec![0u8; 8],
            is_allowed: true,
        }];
        assert!(validate_results(&outputs, &[4]).is_err());
    }
}
