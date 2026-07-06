use crate::{
    dfg::{
        partition_components, partition_preserving_parallelism, types::*, ComponentEdge, ExecNode,
    },
    FHE_BATCH_LATENCY_HISTOGRAM,
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
use tfhe::ReRandomizationContext;
use tokio::task::JoinSet;
use tracing::{error, info, warn};

use super::{DFComponentGraph, DFGraph, OpNode};

const COMPACT_PUBLIC_ENCRYPTION_DOMAIN_SEPARATOR: [u8; 8] = *b"TFHE_Enc";
const BLOCK_RERANDOMISATION_DOMAIN_SEPARATOR: [u8; 8] = *b"TFHE_Brr";

/// Re-randomize a single boundary ciphertext using a block-scoped seed
/// derived from the current block hash and the ciphertext itself.
/// This is the RFC 019 materialization step: each boundary input entering
/// a block is re-randomized exactly once with `ReRand(Hash(H_B, ct))`.
pub fn re_randomise_boundary_input(
    ct: &mut SupportedFheCiphertexts,
    block_hash: &[u8],
    cpk: &tfhe::CompactPublicKey,
) -> Result<()> {
    if matches!(ct, SupportedFheCiphertexts::Scalar(_)) {
        return Ok(());
    }
    let mut re_rand_context = ReRandomizationContext::new(
        BLOCK_RERANDOMISATION_DOMAIN_SEPARATOR,
        [block_hash],
        COMPACT_PUBLIC_ENCRYPTION_DOMAIN_SEPARATOR,
    );
    ct.add_to_re_randomization_context(&mut re_rand_context);
    let mut seed_gen = re_rand_context.finalize();
    ct.re_randomise(cpk, seed_gen.next_seed()?)?;
    Ok(())
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
}

struct SchedulerKeys {
    #[cfg(not(feature = "gpu"))]
    sks: tfhe::ServerKey,
    #[cfg(feature = "gpu")]
    sks: tfhe::CudaServerKey,
    cpk: tfhe::CompactPublicKey,
    gpu_idx: usize,
}

type PartitionResult = (Vec<(Handle, TaskOutput)>, NodeIndex);
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
    fn get_keys(&self, _target: DeviceSelection) -> Result<SchedulerKeys> {
        Ok(SchedulerKeys {
            sks: self.sks.clone(),
            cpk: self.cpk.clone(),
            gpu_idx: 0,
        })
    }
    #[cfg(feature = "gpu")]
    fn get_keys(&self, target: DeviceSelection) -> Result<SchedulerKeys> {
        if self.csks.is_empty() {
            anyhow::bail!("No GPU server keys available");
        }
        match target {
            DeviceSelection::Index(i) => {
                let gpu_idx = if i < self.csks.len() {
                    i
                } else {
                    error!(target: "scheduler", {index = ?i },
			   "Wrong device index");
                    // Instead of giving up, we'll use device 0 (which
                    // should always be safe to use) and keep making
                    // progress even if suboptimally
                    0
                };
                Ok(SchedulerKeys {
                    sks: self.csks[gpu_idx].clone(),
                    cpk: self.cpk.clone(),
                    gpu_idx,
                })
            }
            DeviceSelection::RoundRobin => {
                static LAST: std::sync::atomic::AtomicUsize =
                    std::sync::atomic::AtomicUsize::new(0);
                // Use fetch_add to increment atomically
                let i = LAST.fetch_add(1, std::sync::atomic::Ordering::Relaxed) % self.csks.len();
                Ok(SchedulerKeys {
                    sks: self.csks[i].clone(),
                    cpk: self.cpk.clone(),
                    gpu_idx: i,
                })
            }
            DeviceSelection::NA => Ok(SchedulerKeys {
                sks: self.csks[0].clone(),
                cpk: self.cpk.clone(),
                gpu_idx: 0,
            }),
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
                let keys = self.get_keys(DeviceSelection::RoundRobin)?;
                let parent_span = tracing::Span::current();
                set.spawn_blocking(move || {
                    let span_guard = parent_span.enter();
                    let result = execute_partition(args, index, keys.gpu_idx, keys.sks, keys.cpk);
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
            let task_index = result.1;
            for (handle, output) in result.0.into_iter() {
                // Add computed allowed handles to the graph. These
                // can be used as inputs and forwarded to subsequent,
                // dependent transactions
                self.graph.add_output(
                    &handle,
                    &output.transaction_id,
                    output.result,
                    &self.edges,
                )?;
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
                    let keys = self.get_keys(DeviceSelection::RoundRobin)?;
                    let parent_span = tracing::Span::current();
                    set.spawn_blocking(move || {
                        let span_guard = parent_span.enter();
                        let result = execute_partition(
                            args,
                            dependent_task_index,
                            keys.gpu_idx,
                            keys.sks,
                            keys.cpk,
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
    let mut outputs = Vec::with_capacity(transactions.len());
    let mut available_outputs: HashMap<Handle, DFGTxInput> = HashMap::new();
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
                let Some(ct) = available_outputs.get(h).cloned() else {
                    warn!(target: "scheduler", {transaction_id = ?hex::encode(&tid) },
		       "Missing input to compute transaction - skipping");
                    for nidx in dfg.graph.node_identifiers() {
                        let Some(node) = dfg.graph.node_weight_mut(nidx) else {
                            error!(target: "scheduler", {index = ?nidx.index() }, "Wrong dataflow graph index");
                            continue;
                        };
                        if node.is_allowed {
                            outputs.push((
                                node.result_handle.clone(),
                                TaskOutput {
                                    transaction_id: tid.clone(),
                                    result: Err(SchedulerError::MissingInputs.into()),
                                },
                            ));
                        }
                    }
                    continue 'tx;
                };
                *i = Some(ct);
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
                    outputs.push((
                        node.result_handle.clone(),
                        TaskOutput {
                            transaction_id: tid.clone(),
                            result: Err(SchedulerError::CyclicDependence.into()),
                        },
                    ));
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
            let result = try_execute_node(node, nidx.index(), tx_inputs, gpu_idx, &tid, &cpk);
            match result {
                Ok(result) => {
                    let nidx = NodeIndex::new(result.0);
                    if result.1.is_ok() {
                        for edge in edges.edges_directed(nidx, Direction::Outgoing) {
                            let child_index = edge.target();
                            let Some(child_node) = dfg.graph.node_weight_mut(child_index) else {
                                error!(target: "scheduler", {index = ?child_index.index() }, "Wrong dataflow graph index");
                                continue;
                            };
                            // Update input of consumers
                            if let Ok(ref res) = result.1 {
                                child_node.inputs[*edge.weight() as usize] =
                                    DFGTaskInput::Value(res.working.clone());
                            }
                        }
                    }
                    // Update partition's outputs (allowed handles only)
                    let Some(node) = dfg.graph.node_weight_mut(nidx) else {
                        error!(target: "scheduler", {index = ?nidx.index() }, "Wrong dataflow graph index");
                        continue;
                    };
                    let handle = node.result_handle.clone();
                    let result = match result.1 {
                        Ok(v) => {
                            let task_result = TaskResult {
                                compressed_ct: v.compressed,
                                working_ct: Some(v.working),
                                is_allowed: node.is_allowed,
                                transaction_id: tid.clone(),
                            };
                            let working = task_result.working_ct.as_ref().expect(
                                "same-block propagation invariant violation: successful output missing working ciphertext",
                            );
                            let input =
                                DFGTxInput::Value((working.clone(), task_result.is_allowed));
                            available_outputs.insert(handle.clone(), input);
                            Ok(task_result)
                        }
                        Err(e) => Err(e),
                    };
                    outputs.push((
                        handle,
                        TaskOutput {
                            transaction_id: tid.clone(),
                            result,
                        },
                    ));
                }
                Err(e) => {
                    let Some(node) = dfg.graph.node_weight(*nidx) else {
                        error!(target: "scheduler", {index = ?nidx.index() }, "Wrong dataflow graph index");
                        continue;
                    };
                    if node.is_allowed {
                        outputs.push((
                            node.result_handle.clone(),
                            TaskOutput {
                                transaction_id: tid.clone(),
                                result: Err(e),
                            },
                        ));
                    }
                }
            }
        }
        drop(_exec_guard);
        let elapsed = started_at.elapsed();
        FHE_BATCH_LATENCY_HISTOGRAM.observe(elapsed.as_secs_f64());
    }
    (outputs, task_id)
}

fn try_execute_node(
    node: &mut OpNode,
    node_index: usize,
    tx_inputs: &mut HashMap<Handle, Option<DFGTxInput>>,
    gpu_idx: usize,
    transaction_id: &Handle,
    cpk: &tfhe::CompactPublicKey,
) -> Result<(usize, OpResult)> {
    let _ = cpk;
    if !node.check_ready_inputs(tx_inputs) {
        return Err(SchedulerError::SchedulerError.into());
    }
    let mut cts = Vec::with_capacity(node.inputs.len());
    for i in std::mem::take(&mut node.inputs) {
        match i {
            DFGTaskInput::Value(v) => {
                // Boundary inputs arrive pre-materialized from the worker (D5)
                // and same-block intermediates propagate as Values (D2/D3).
                #[cfg(feature = "gpu")]
                let mut v = v;
                #[cfg(feature = "gpu")]
                v.move_to_current_device();
                cts.push(v);
            }
            DFGTaskInput::Compressed(cct) => {
                let decompressed = SupportedFheCiphertexts::decompress(
		    cct.ct_type,
		    &cct.ct_bytes,
		    gpu_idx,
		)
		    .map_err(|e| {
			error!(
			    target: "scheduler",
			    { handle = ?hex::encode(&node.result_handle), ct_type = cct.ct_type, error = ?e },
			    "Error while decompressing op input"
			);
			telemetry::set_current_span_error(&e);
			SchedulerError::DecompressionError
		    })?;
                cts.push(decompressed);
            }
            DFGTaskInput::Dependence(_) => {
                error!(target: "scheduler", { handle = ?hex::encode(&node.result_handle) }, "Computation missing inputs");
                return Err(SchedulerError::MissingInputs.into());
            }
        }
    }
    let opcode = node.opcode;
    let output_type = get_ct_type(&node.result_handle).map_err(|e| {
        error!(target: "scheduler", { handle = ?hex::encode(&node.result_handle), error = ?e },
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
            error!(target: "scheduler", { handle = ?hex::encode(&node.result_handle), msg },
               "Panic while executing operation");
            telemetry::set_current_span_error(&msg);
            Err(SchedulerError::ExecutionPanic(msg).into())
        }
        Ok(r) => Ok(r),
    }
}

/// Compresses an operation result; when the compression keys reject carry
/// residue (left by levelled no-PBS paths, e.g. ops taking tfhe's trivial
/// fast path on raw-forwarded operands), the ciphertext is re-encoded
/// through the deterministic, value-preserving `clean_carries` and
/// compressed again. The cleaned ciphertext is returned so it also replaces
/// the forwarded working value — persisted bytes and in-memory propagation
/// must stay identical.
fn compress_or_clean_carries(
    working: SupportedFheCiphertexts,
) -> (
    SupportedFheCiphertexts,
    std::result::Result<Vec<u8>, fhevm_engine_common::types::FhevmError>,
) {
    match working.compress() {
        Err(fhevm_engine_common::types::FhevmError::CiphertextCompressionRequiresEmptyCarries) => {
            match working.clean_carries() {
                Ok(cleaned) => {
                    info!(target: "scheduler", { ct_type = cleaned.type_name() },
                          "Cleaned carry residue before compressing operation result");
                    let compressed = cleaned.compress();
                    (cleaned, compressed)
                }
                Err(error) => (working, Err(error)),
            }
        }
        other => (working, other),
    }
}

type OpResult = Result<ComputationOutput>;
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
        Ok(FheOperation::FheGetCiphertext) => {
            // Compression span (no FHE here)
            let _guard = tracing::info_span!(
                "compress_ciphertext",
                txn_id = %txn_id_short,
                ct_type = inputs[0].type_name(),
                operation = "FheGetCiphertext",
                compressed_size = tracing::field::Empty,
            )
            .entered();

            let working = inputs
                .into_iter()
                .next()
                .expect("FheGetCiphertext requires one input");
            let ct_type = working.type_num();
            let (working, compressed) = compress_or_clean_carries(working);
            match compressed {
                Ok(ct_bytes) => {
                    tracing::Span::current().record("compressed_size", ct_bytes.len() as i64);
                    (
                        graph_node_index,
                        Ok(ComputationOutput {
                            compressed: CompressedCiphertext { ct_type, ct_bytes },
                            working,
                        }),
                    )
                }
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
                Ok(working) => {
                    // Compression span
                    let _guard = tracing::info_span!(
                        "compress_ciphertext",
                        txn_id = %txn_id_short,
                        ct_type = working.type_name(),
                        operation = op_name,
                        compressed_size = tracing::field::Empty,
                    )
                    .entered();
                    let ct_type = working.type_num();
                    let (working, compressed) = compress_or_clean_carries(working);
                    match compressed {
                        Ok(ct_bytes) => {
                            tracing::Span::current()
                                .record("compressed_size", ct_bytes.len() as i64);
                            (
                                graph_node_index,
                                Ok(ComputationOutput {
                                    compressed: CompressedCiphertext { ct_type, ct_bytes },
                                    working,
                                }),
                            )
                        }
                        Err(error) => {
                            telemetry::set_current_span_error(&error);
                            (graph_node_index, Err(error.into()))
                        }
                    }
                }
                Err(e) => {
                    telemetry::set_current_span_error(&e);
                    (graph_node_index, Err(e.into()))
                }
            }
        }
        Err(e) => (graph_node_index, Err(e.into())),
    }
}
