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
use tfhe::ReRandomizationContext;
use tokio::task::JoinSet;
use tracing::{error, info, warn};

use super::{DFComponentGraph, DFGraph, OpNode};

const OPERATION_RERANDOMISATION_DOMAIN_SEPARATOR: [u8; 8] = *b"TFHE_Rrd";
const COMPACT_PUBLIC_ENCRYPTION_DOMAIN_SEPARATOR: [u8; 8] = *b"TFHE_Enc";

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
                    args.push((
                        std::mem::take(&mut tx.graph),
                        std::mem::take(&mut tx.inputs),
                        tx.transaction_id.clone(),
                        tx.component_id,
                    ));
                }
                let (sks, cpk) = self.get_keys(DeviceSelection::RoundRobin)?;
                let parent_span = tracing::Span::current();
                set.spawn_blocking(move || {
                    let span_guard = parent_span.enter();
                    let result = execute_partition(args, index, 0, sks, cpk);
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
                    set.spawn_blocking(move || {
                        let span_guard = parent_span.enter();
                        let result = execute_partition(args, dependent_task_index, 0, sks, cpk);
                        drop(span_guard);
                        result
                    });
                }
            }
        }
        Ok(())
    }
}

fn re_randomise_operation_inputs(
    cts: &mut [SupportedFheCiphertexts],
    opcode: i32,
    cpk: &tfhe::CompactPublicKey,
) -> Result<()> {
    let mut re_rand_context = ReRandomizationContext::new(
        OPERATION_RERANDOMISATION_DOMAIN_SEPARATOR,
        [opcode.to_be_bytes().as_slice()],
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
fn execute_partition(
    transactions: ComponentSet,
    task_id: NodeIndex,
    gpu_idx: usize,
    #[cfg(not(feature = "gpu"))] sks: tfhe::ServerKey,
    #[cfg(feature = "gpu")] sks: tfhe::CudaServerKey,
    cpk: tfhe::CompactPublicKey,
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
                            for h in node.result_handles.iter() {
                                res.insert(h.clone(), Err(SchedulerError::MissingInputs.into()));
                            }
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
                    for h in node.result_handles.iter() {
                        res.insert(h.clone(), Err(SchedulerError::CyclicDependence.into()));
                    }
                }
            }
            continue 'tx;
        };
        let edges = dfg.graph.map(|_, _| (), |_, edge| *edge);
        for nidx in ts.iter() {
            let producer_handles: Vec<Handle> = {
                let Some(node) = dfg.graph.node_weight(*nidx) else {
                    error!(target: "scheduler", {index = ?nidx.index() }, "Wrong dataflow graph index");
                    continue;
                };
                node.result_handles.clone()
            };

            let Some(node) = dfg.graph.node_weight_mut(*nidx) else {
                error!(target: "scheduler", {index = ?nidx.index() }, "Wrong dataflow graph index");
                continue;
            };
            let result = try_execute_node(node, nidx.index(), tx_inputs, gpu_idx, &tid, &cpk);
            match result {
                Ok(result) => {
                    let nidx = NodeIndex::new(result.0);
                    if let Ok(ref vec_res) = result.1 {
                        // Match consumer dep handle against producer_handles to
                        // forward the right ciphertext from a multi-output op.
                        for edge in edges.edges_directed(nidx, Direction::Outgoing) {
                            let child_index = edge.target();
                            let input_idx = *edge.weight() as usize;
                            let Some(child_node) = dfg.graph.node_weight_mut(child_index) else {
                                error!(target: "scheduler", {index = ?child_index.index() }, "Wrong dataflow graph index");
                                continue;
                            };
                            let dep_handle = match child_node.inputs.get(input_idx) {
                                Some(DFGTaskInput::Dependence(dh)) => dh.clone(),
                                _ => continue,
                            };
                            let Some(out_idx) =
                                producer_handles.iter().position(|h| h == &dep_handle)
                            else {
                                error!(target: "scheduler",
                                    { handle = ?hex::encode(&dep_handle) },
                                    "Consumer dependence handle not found in producer outputs - graph inconsistency");
                                continue;
                            };
                            let Some(ct) = vec_res.get(out_idx) else {
                                error!(target: "scheduler",
                                    { out_idx, vec_len = vec_res.len() },
                                    "Result vector shorter than expected output index");
                                continue;
                            };
                            child_node.inputs[input_idx] = DFGTaskInput::Compressed(ct.clone());
                        }
                    }
                    // Update partition's outputs (allowed handles only)
                    let Some(node) = dfg.graph.node_weight_mut(nidx) else {
                        error!(target: "scheduler", {index = ?nidx.index() }, "Wrong dataflow graph index");
                        continue;
                    };
                    let is_allowed = node.is_allowed;
                    match result.1 {
                        Ok(vec_res) if vec_res.len() == producer_handles.len() => {
                            for (h, ct) in producer_handles.iter().zip(vec_res.iter()) {
                                res.insert(
                                    h.clone(),
                                    Ok(TaskResult {
                                        compressed_ct: ct.clone(),
                                        is_allowed,
                                        transaction_id: tid.clone(),
                                    }),
                                );
                            }
                        }
                        Ok(vec_res) => {
                            // Should not happen; fail every handle loudly rather than stall.
                            error!(target: "scheduler",
                                { produced = vec_res.len(), expected = producer_handles.len() },
                                "Multi-output arity mismatch: dispatch returned wrong number of ciphertexts");
                            let msg = format!(
                                "multi-output arity mismatch: produced {} ciphertexts for {} handles",
                                vec_res.len(),
                                producer_handles.len()
                            );
                            fan_out_error(
                                &producer_handles,
                                SchedulerError::MultiOutputFailure(msg).into(),
                                &mut res,
                            );
                        }
                        Err(e) => fan_out_error(&producer_handles, e, &mut res),
                    }
                }
                Err(e) => {
                    let Some(node) = dfg.graph.node_weight(*nidx) else {
                        error!(target: "scheduler", {index = ?nidx.index() }, "Wrong dataflow graph index");
                        continue;
                    };
                    if node.is_allowed {
                        fan_out_error(&producer_handles, e, &mut res);
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
    let handle = hex::encode(&node.result_handles[0]);
    let outputs = node.result_handles.len();
    let mut cts = Vec::with_capacity(node.inputs.len());
    for i in std::mem::take(&mut node.inputs) {
        match i {
            DFGTaskInput::Value(v) => {
                if !matches!(v, SupportedFheCiphertexts::Scalar(_)) {
                    error!(target: "scheduler",
                        { handle = ?handle, outputs },
                        "Consensus risk: non-scalar uncompressed ciphertext");
                }
                cts.push(v);
            }
            DFGTaskInput::Compressed(cct) => {
                let decompressed =
                    SupportedFheCiphertexts::decompress(cct.ct_type, &cct.ct_bytes, gpu_idx)
                        .map_err(|e| {
                            error!(
                                target: "scheduler",
                                { handle = ?handle, outputs, ct_type = cct.ct_type, error = ?e },
                                "Error while decompressing op input"
                            );
                            telemetry::set_current_span_error(&e);
                            SchedulerError::DecompressionError
                        })?;
                cts.push(decompressed);
            }
            DFGTaskInput::Dependence(_) => {
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
        if let Err(e) = re_randomise_operation_inputs(&mut cts, node.opcode, cpk) {
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
    let output_type = get_ct_type(&node.result_handles[0]).map_err(|e| {
        error!(target: "scheduler",
            { handle = ?handle, outputs, error = ?e },
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
            let msg = e
                .downcast_ref::<&str>()
                .map(|s| s.to_string())
                .or_else(|| e.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "unknown panic payload".to_string());
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

/// Ok payload length matches the op's output handle count (1 or N).
type OpResult = Result<Vec<CompressedCiphertext>>;

/// Fan one error out to every output handle so siblings of a multi-output op share
/// the same worker classification. Typed `SchedulerError` is cloned; anything else
/// is wrapped in `MultiOutputFailure`.
fn fan_out_error(
    handles: &[Handle],
    err: anyhow::Error,
    res: &mut HashMap<Handle, Result<TaskResult>>,
) {
    if handles.is_empty() {
        return;
    }
    if handles.len() == 1 {
        res.insert(handles[0].clone(), Err(err));
        return;
    }
    if let Some(sched) = err.downcast_ref::<SchedulerError>() {
        let cloned = sched.clone();
        for h in handles {
            res.insert(h.clone(), Err(cloned.clone().into()));
        }
        return;
    }
    let err_msg = format!("{}", err);
    for h in handles {
        res.insert(
            h.clone(),
            Err(SchedulerError::MultiOutputFailure(err_msg.clone()).into()),
        );
    }
}

fn compress_single(
    ct: SupportedFheCiphertexts,
    txn_id_short: &str,
    op_name: &str,
) -> Result<CompressedCiphertext> {
    let _guard = tracing::info_span!(
        "compress_ciphertext",
        txn_id = %txn_id_short,
        ct_type = ct.type_name(),
        operation = op_name,
        compressed_size = tracing::field::Empty,
    )
    .entered();
    let ct_type = ct.type_num();
    let ct_bytes = ct.compress().map_err(|e| {
        telemetry::set_current_span_error(&e);
        anyhow::Error::from(e)
    })?;
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
                gpu_idx,
            );

            return match result {
                Ok(results) => {
                    let compressed: Result<Vec<CompressedCiphertext>> = results
                        .into_iter()
                        .map(|r| compress_single(r, &txn_id_short, &op_name))
                        .collect();
                    (graph_node_index, compressed)
                }
                Err(e) => {
                    telemetry::set_current_span_error(&e);
                    (graph_node_index, Err(e.into()))
                }
            };
        }
    }

    let op = FheOperation::try_from(operation);
    match op {
        Ok(FheOperation::FheGetCiphertext) => {
            let ct_type = inputs[0].type_num();
            let compressed = inputs[0].compress();
            match compressed {
                Ok(ct_bytes) => (
                    graph_node_index,
                    Ok(vec![CompressedCiphertext { ct_type, ct_bytes }]),
                ),
                Err(error) => {
                    telemetry::set_current_span_error(&error);
                    (graph_node_index, Err(error.into()))
                }
            }
        }
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
                Ok(result) => match compress_single(result, &txn_id_short, op_name) {
                    Ok(cc) => (graph_node_index, Ok(vec![cc])),
                    Err(e) => (graph_node_index, Err(e)),
                },
                Err(e) => {
                    telemetry::set_current_span_error(&e);
                    (graph_node_index, Err(e.into()))
                }
            }
        }
        Err(e) => (graph_node_index, Err(e.into())),
    }
}
