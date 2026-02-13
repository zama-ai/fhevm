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
use fhevm_engine_common::tfhe_ops::perform_fhe_operation;
use fhevm_engine_common::types::{FhevmError, Handle, SupportedFheCiphertexts};
use fhevm_engine_common::utils::HeartBeat;
use opentelemetry::trace::{Status, TraceContextExt};
use std::collections::HashMap;
use std::fmt::Display;
use tfhe::ReRandomizationContext;
use tokio::task::JoinSet;
use tracing::{error, info, warn};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use super::{DFComponentGraph, DFGraph, OpNode};

const OPERATION_RERANDOMISATION_DOMAIN_SEPARATOR: [u8; 8] = *b"TFHE_Rrd";
const COMPACT_PUBLIC_ENCRYPTION_DOMAIN_SEPARATOR: [u8; 8] = *b"TFHE_Enc";

#[derive(Clone, Copy, Debug)]
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
            sks: sks.clone(),
            cpk: cpk.clone(),
            #[cfg(feature = "gpu")]
            csks: csks.clone(),
            activity_heartbeat,
        }
    }

    fn spawn_partition_task(
        &self,
        set: &mut JoinSet<PartitionResult>,
        args: ComponentSet,
        partition_index: NodeIndex,
    ) -> Result<()> {
        let (sks, cpk) = self.get_keys(DeviceSelection::RoundRobin)?;
        let parent_span = tracing::Span::current();
        set.spawn_blocking(move || {
            parent_span.in_scope(|| execute_partition(args, partition_index, 0, sks, cpk))
        });
        Ok(())
    }

    fn collect_partition_components(
        &mut self,
        component_nodes: &[NodeIndex],
        skip_uncomputable: bool,
    ) -> Result<ComponentSet> {
        let mut args = Vec::with_capacity(component_nodes.len());
        for nidx in component_nodes.iter() {
            let tx = self
                .graph
                .graph
                .node_weight_mut(*nidx)
                .ok_or(SchedulerError::DataflowGraphError)?;
            if skip_uncomputable && tx.is_uncomputable {
                continue;
            }
            args.push((
                std::mem::take(&mut tx.graph),
                std::mem::take(&mut tx.inputs),
                tx.transaction_id.clone(),
                tx.component_id,
            ));
        }
        Ok(args)
    }

    #[tracing::instrument(skip_all, fields(operation = "schedule"))]
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

    #[tracing::instrument(
        skip_all,
        fields(operation = "schedule_coarse_grain", strategy = ?strategy)
    )]
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
                let component_nodes = node.df_nodes.clone();
                let args = self.collect_partition_components(&component_nodes, false)?;
                self.spawn_partition_task(&mut set, args, index)?;
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
                    let component_nodes = dependent_task.df_nodes.clone();
                    let args = self.collect_partition_components(&component_nodes, true)?;
                    self.spawn_partition_task(&mut set, args, dependent_task_index)?;
                }
            }
        }
        Ok(())
    }
}

fn decompress_transaction_inputs(
    inputs: &mut HashMap<Handle, Option<DFGTxInput>>,
    gpu_idx: usize,
) -> Result<usize> {
    let mut count = 0;
    for txinput in inputs.values_mut() {
        if let Some(DFGTxInput::Compressed(((t, c), allowed))) = txinput {
            let decomp = SupportedFheCiphertexts::decompress(*t, c, gpu_idx)?;
            *txinput = Some(DFGTxInput::Value((decomp, *allowed)));
            count += 1;
        }
    }
    Ok(count)
}

fn short_transaction_id(transaction_id: &Handle) -> String {
    hex::encode(transaction_id)
        .get(0..10)
        .unwrap_or_default()
        .to_owned()
}

fn mark_current_span_error(err: impl Display) {
    tracing::Span::current()
        .context()
        .span()
        .set_status(Status::error(err.to_string()));
}

fn mark_allowed_nodes_failed(
    dfg: &mut DFGraph,
    output: &mut HashMap<Handle, Result<TaskResult>>,
    scheduler_error: SchedulerError,
) {
    for nidx in dfg.graph.node_identifiers() {
        let Some(node) = dfg.graph.node_weight_mut(nidx) else {
            error!(target: "scheduler", { index = ?nidx.index() }, "Wrong dataflow graph index");
            continue;
        };
        if node.is_allowed {
            output.insert(node.result_handle.clone(), Err(scheduler_error.into()));
        }
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
#[tracing::instrument(
    skip_all,
    fields(
        operation = "execute_partition",
        task_id = task_id.index() as i64,
        gpu_idx = gpu_idx as i64,
        transaction_count = transactions.len() as i64
    )
)]
fn execute_partition(
    transactions: ComponentSet,
    task_id: NodeIndex,
    gpu_idx: usize,
    #[cfg(not(feature = "gpu"))] sks: tfhe::ServerKey,
    #[cfg(feature = "gpu")] sks: tfhe::CudaServerKey,
    cpk: tfhe::CompactPublicKey,
) -> PartitionResult {
    tfhe::set_server_key(sks);
    let mut output: HashMap<Handle, Result<TaskResult>> =
        HashMap::with_capacity(transactions.len());
    for (mut dfg, mut tx_inputs, tid, _cid) in transactions {
        execute_transaction(&mut dfg, &mut tx_inputs, &tid, gpu_idx, &cpk, &mut output);
    }
    (output, task_id)
}

#[tracing::instrument(
    skip_all,
    fields(
        operation = "execute_transaction",
        txn_id = %short_transaction_id(transaction_id)
    )
)]
fn execute_transaction(
    dfg: &mut DFGraph,
    tx_inputs: &mut HashMap<Handle, Option<DFGTxInput>>,
    transaction_id: &Handle,
    gpu_idx: usize,
    cpk: &tfhe::CompactPublicKey,
    output: &mut HashMap<Handle, Result<TaskResult>>,
) {
    for (h, tx_input) in tx_inputs.iter_mut() {
        if tx_input.is_some() {
            continue;
        }
        let Some(Ok(ct)) = output.get(h) else {
            warn!(target: "scheduler", { transaction_id = ?hex::encode(transaction_id) }, "Missing input to compute transaction - skipping");
            mark_allowed_nodes_failed(dfg, output, SchedulerError::MissingInputs);
            return;
        };
        *tx_input = Some(DFGTxInput::Value((ct.ct.clone(), ct.is_allowed)));
    }

    if let Err(err) = decompress_inputs_for_transaction(tx_inputs, transaction_id, gpu_idx) {
        error!(target: "scheduler", { transaction_id = ?hex::encode(transaction_id), error = ?err }, "Error while decompressing inputs");
        mark_allowed_nodes_failed(dfg, output, SchedulerError::DecompressionError);
        return;
    }

    let started_at = std::time::Instant::now();
    let Ok(topo_sorted_nodes) = daggy::petgraph::algo::toposort(&dfg.graph, None) else {
        error!(target: "scheduler", { transaction_id = ?transaction_id }, "Cyclical dependence error in transaction");
        mark_allowed_nodes_failed(dfg, output, SchedulerError::CyclicDependence);
        return;
    };
    let edges = dfg.graph.map(|_, _| (), |_, edge| *edge);

    for nidx in topo_sorted_nodes.iter() {
        let Some(node) = dfg.graph.node_weight_mut(*nidx) else {
            error!(target: "scheduler", { index = ?nidx.index() }, "Wrong dataflow graph index");
            continue;
        };
        let result = try_execute_node(node, nidx.index(), tx_inputs, gpu_idx, transaction_id, cpk);
        match result {
            Ok(result) => {
                let nidx = NodeIndex::new(result.0);
                if result.1.is_ok() {
                    for edge in edges.edges_directed(nidx, Direction::Outgoing) {
                        let child_index = edge.target();
                        let Some(child_node) = dfg.graph.node_weight_mut(child_index) else {
                            error!(target: "scheduler", { index = ?child_index.index() }, "Wrong dataflow graph index");
                            continue;
                        };
                        if let Ok(ref res) = result.1 {
                            child_node.inputs[*edge.weight() as usize] =
                                DFGTaskInput::Value(res.0.clone());
                        }
                    }
                }
                let Some(node) = dfg.graph.node_weight_mut(nidx) else {
                    error!(target: "scheduler", { index = ?nidx.index() }, "Wrong dataflow graph index");
                    continue;
                };
                output.insert(
                    node.result_handle.clone(),
                    result.1.map(|v| TaskResult {
                        ct: v.0,
                        compressed_ct: if node.is_allowed { v.1 } else { None },
                        is_allowed: node.is_allowed,
                        transaction_id: transaction_id.clone(),
                    }),
                );
            }
            Err(e) => {
                let Some(node) = dfg.graph.node_weight(*nidx) else {
                    error!(target: "scheduler", { index = ?nidx.index() }, "Wrong dataflow graph index");
                    continue;
                };
                if node.is_allowed {
                    output.insert(node.result_handle.clone(), Err(e));
                }
            }
        }
    }

    FHE_BATCH_LATENCY_HISTOGRAM.observe(started_at.elapsed().as_secs_f64());
}

#[tracing::instrument(
    skip_all,
    fields(
        operation = "decompress_ciphertexts",
        txn_id = %short_transaction_id(transaction_id),
        count = tracing::field::Empty
    )
)]
fn decompress_inputs_for_transaction(
    tx_inputs: &mut HashMap<Handle, Option<DFGTxInput>>,
    transaction_id: &Handle,
    gpu_idx: usize,
) -> Result<()> {
    match decompress_transaction_inputs(tx_inputs, gpu_idx) {
        Ok(count) => {
            tracing::Span::current().record("count", count as i64);
            Ok(())
        }
        Err(err) => {
            mark_current_span_error(&err);
            error!(target: "scheduler", { transaction_id = ?hex::encode(transaction_id), error = ?err }, "Decompression failed");
            Err(err)
        }
    }
}

#[tracing::instrument(
    skip_all,
    fields(
        operation = "try_execute_node",
        txn_id = %short_transaction_id(transaction_id),
        node_index = node_index as i64,
        operation_code = node.opcode as i64,
        is_allowed = node.is_allowed
    )
)]
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
        if let DFGTaskInput::Value(i) = i {
            cts.push(i);
        } else {
            // That should not be possible as we called the checker.
            error!(target: "scheduler", { handle = ?hex::encode(&node.result_handle) }, "Computation missing inputs");
            return Err(SchedulerError::MissingInputs.into());
        }
    }

    if let Err(err) = rerandomise_node_inputs(&mut cts, node.opcode, cpk, transaction_id) {
        error!(target: "scheduler", { handle = ?hex::encode(&node.result_handle), error = ?err }, "Error while re-randomising operation inputs");
        return Err(SchedulerError::ReRandomisationError.into());
    }

    let opcode = node.opcode;
    let is_allowed = node.is_allowed;
    Ok(run_computation(
        opcode,
        cts,
        node_index,
        is_allowed,
        gpu_idx,
        transaction_id,
    ))
}

#[tracing::instrument(
    skip_all,
    fields(
        operation = "rerandomise_op_inputs",
        txn_id = %short_transaction_id(transaction_id),
        operation_code = opcode as i64
    )
)]
fn rerandomise_node_inputs(
    cts: &mut [SupportedFheCiphertexts],
    opcode: i32,
    cpk: &tfhe::CompactPublicKey,
    transaction_id: &Handle,
) -> Result<()> {
    let started_at = std::time::Instant::now();
    match re_randomise_operation_inputs(cts, opcode, cpk) {
        Ok(()) => {
            RERAND_LATENCY_BATCH_HISTOGRAM.observe(started_at.elapsed().as_secs_f64());
            Ok(())
        }
        Err(err) => {
            mark_current_span_error(&err);
            error!(target: "scheduler", { transaction_id = ?hex::encode(transaction_id), error = ?err }, "Re-randomisation failed");
            Err(err)
        }
    }
}

type OpResult = Result<(SupportedFheCiphertexts, Option<(i16, Vec<u8>)>)>;

#[tracing::instrument(
    skip_all,
    fields(
        operation = "compress_ciphertext",
        txn_id = %short_transaction_id(transaction_id),
        logical_operation = logical_operation,
        ct_type = ciphertext.type_name(),
        compressed_size = tracing::field::Empty
    )
)]
fn compress_ciphertext_result(
    ciphertext: SupportedFheCiphertexts,
    logical_operation: &str,
    graph_node_index: usize,
    transaction_id: &Handle,
) -> (usize, OpResult) {
    let ct_type = ciphertext.type_num();
    match ciphertext.compress() {
        Ok(ct_bytes) => {
            tracing::Span::current().record("compressed_size", ct_bytes.len() as i64);
            (
                graph_node_index,
                Ok((ciphertext, Some((ct_type, ct_bytes)))),
            )
        }
        Err(error) => {
            mark_current_span_error(&error);
            (graph_node_index, Err(error.into()))
        }
    }
}

#[tracing::instrument(
    skip_all,
    fields(
        operation = "fhe_operation",
        txn_id = %short_transaction_id(transaction_id),
        logical_operation = op_name,
        operation_code = operation as i64,
        input_type = tracing::field::Empty
    )
)]
fn execute_fhe_operation(
    operation: i32,
    inputs: &[SupportedFheCiphertexts],
    gpu_idx: usize,
    transaction_id: &Handle,
    op_name: &str,
) -> Result<SupportedFheCiphertexts, FhevmError> {
    if let Some(first_input) = inputs.first() {
        tracing::Span::current().record("input_type", first_input.type_name());
    }

    let result = perform_fhe_operation(operation as i16, inputs, gpu_idx);
    if let Err(err) = &result {
        mark_current_span_error(err);
    }
    result
}

#[tracing::instrument(
    skip_all,
    fields(
        operation = "run_computation",
        txn_id = %short_transaction_id(transaction_id),
        operation_code = operation as i64,
        is_allowed
    )
)]
fn run_computation(
    operation: i32,
    inputs: Vec<SupportedFheCiphertexts>,
    graph_node_index: usize,
    is_allowed: bool,
    gpu_idx: usize,
    transaction_id: &Handle,
) -> (usize, OpResult) {
    let op = FheOperation::try_from(operation);
    match op {
        Ok(FheOperation::FheGetCiphertext) => compress_ciphertext_result(
            inputs[0].clone(),
            "FheGetCiphertext",
            graph_node_index,
            transaction_id,
        ),
        Ok(fhe_op) => {
            let op_name = fhe_op.as_str_name();
            match execute_fhe_operation(operation, &inputs, gpu_idx, transaction_id, op_name) {
                Ok(result) => {
                    if is_allowed {
                        compress_ciphertext_result(
                            result,
                            op_name,
                            graph_node_index,
                            transaction_id,
                        )
                    } else {
                        (graph_node_index, Ok((result, None)))
                    }
                }
                Err(err) => (graph_node_index, Err(err.into())),
            }
        }
        Err(e) => {
            mark_current_span_error(e);
            (graph_node_index, Err(e.into()))
        }
    }
}
