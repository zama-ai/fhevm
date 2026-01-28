//! # Scheduler Execution Engine
//!
//! This module implements the main scheduler that orchestrates parallel execution of FHE
//! operations. It manages the lifecycle of computation tasks, handles input preparation
//! (re-randomisation and decompression), and coordinates result propagation between
//! dependent transactions.
//!
//! ## Execution Flow
//!
//! 1. The scheduler partitions the component graph based on the selected strategy
//! 2. Ready partitions (those with no unsatisfied dependences) are spawned as tasks
//! 3. When a task completes, its results are propagated to dependent partitions
//! 4. Newly ready partitions are spawned, continuing until all work is complete
//!
//! ## Re-randomisation
//!
//! On CPU, input ciphertexts are re-randomised before execution to provide sIND-CPAD
//! security. This cryptographic operation refreshes the randomness in ciphertexts while
//! preserving the encrypted values.
//! Reference: https://eprint.iacr.org/2025/2005.pdf
//!
//! ## Configuration
//!
//! The scheduling strategy can be configured via the `FHEVM_DF_SCHEDULE` environment
//! variable:
//! - `MAX_PARALLELISM` (default): Maximize parallel execution
//! - `MAX_LOCALITY`: Maximize data locality by grouping operations in connected components

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
use fhevm_engine_common::tfhe_ops::perform_fhe_operation;
use fhevm_engine_common::types::{Handle, SupportedFheCiphertexts};
use fhevm_engine_common::utils::HeartBeat;
use fhevm_engine_common::{common::FheOperation, telemetry};
use opentelemetry::trace::{Span, Tracer};
use std::collections::HashMap;
use tfhe::ReRandomizationContext;
use tokio::task::JoinSet;
use tracing::{error, info, warn};

use super::{DFComponentGraph, DFGraph, OpNode};

/// Domain separator for transaction re-randomisation context.
const TRANSACTION_RERANDOMISATION_DOMAIN_SEPARATOR: [u8; 8] = *b"TFHE_Rrd";

/// Domain separator for compact public key encryption.
const COMPACT_PUBLIC_ENCRYPTION_DOMAIN_SEPARATOR: [u8; 8] = *b"TFHE_Enc";

/// Scheduling strategy for partitioning the dataflow graph.
///
/// The choice of strategy affects the tradeoff between parallelism
/// and data locality during FHE operation execution. This is often
/// more important when offloading to devices where data movement is
/// expensive.
pub enum PartitionStrategy {
    /// Maximize parallel execution by keeping independent operations
    /// separate. Default strategy, currently best on both CPU and GPU
    /// platforms for the workloads benchmarked.
    MaxParallelism,

    /// Maximize data locality by grouping connected operations together.
    /// Best for workloads where data transfer overhead dominates, or when
    /// operations share significant intermediate results.
    MaxLocality,
}

/// Device selection strategy for multi-GPU execution.
///
/// Controls how tasks are assigned to GPU devices when multiple GPUs are available.
enum DeviceSelection {
    /// Use a specific GPU device by index.
    #[allow(dead_code)]
    Index(usize),

    /// Round-robin distribution across all available GPU devices.
    RoundRobin,

    /// No specific device preference; use the default (device 0).
    #[allow(dead_code)]
    NA,
}

/// The main scheduler that orchestrates parallel execution of FHE operations.
///
/// `Scheduler` manages the execution of a component graph by:
/// - Partitioning the graph according to the selected strategy
/// - Spawning ready tasks for parallel execution
/// - Propagating results between dependent transactions
/// - Managing cryptographic keys for FHE operations
///
/// # Lifetime
///
/// The `'a` lifetime is tied to the component graph being scheduled.
///
/// # Fields
///
/// * `graph` - Mutable reference to the component graph being executed
/// * `edges` - Snapshot of dependence edges for result propagation
/// * `sks` - Server key for CPU FHE operations
/// * `cpk` - Compact public key for re-randomisation
/// * `csks` (GPU only) - Vector of CUDA server keys for GPU execution
/// * `activity_heartbeat` - Heartbeat for activity monitoring
pub struct Scheduler<'a> {
    graph: &'a mut DFComponentGraph,
    edges: Dag<(), ComponentEdge>,
    sks: tfhe::ServerKey,
    cpk: tfhe::CompactPublicKey,
    #[cfg(feature = "gpu")]
    csks: Vec<tfhe::CudaServerKey>,
    activity_heartbeat: HeartBeat,
}

/// Result type for partition execution, containing computed results and the partition index.
type PartitionResult = (HashMap<Handle, Result<TaskResult>>, NodeIndex);

impl<'a> Scheduler<'a> {
    /// Checks if an execution node is ready to execute.
    ///
    /// An execution node is ready when all its dependences have been satisfied,
    /// indicated by the dependence counter reaching zero.
    ///
    /// # Arguments
    ///
    /// * `node` - The execution node to check
    ///
    /// # Returns
    ///
    /// `true` if the node has no unsatisfied dependences, `false` otherwise.
    fn is_ready_task(&self, node: &ExecNode) -> bool {
        node.dependence_counter
            .load(std::sync::atomic::Ordering::SeqCst)
            == 0
    }

    /// Creates a new scheduler for the given component graph.
    ///
    /// Initializes the scheduler with the necessary cryptographic keys and takes
    /// a snapshot of the dependence edges for result propagation.
    ///
    /// # Arguments
    ///
    /// * `graph` - Mutable reference to the component graph to schedule
    /// * `sks` - Server key for FHE operations
    /// * `cpk` - Compact public key for re-randomisation
    /// * `csks` (GPU only) - Vector of CUDA server keys for multi-GPU execution
    /// * `activity_heartbeat` - Heartbeat instance for activity monitoring
    ///
    /// # Returns
    ///
    /// A new `Scheduler` instance ready to execute the component graph.
    pub fn new(
        graph: &'a mut DFComponentGraph,
        sks: tfhe::ServerKey,
        cpk: tfhe::CompactPublicKey,
        #[cfg(feature = "gpu")] csks: Vec<tfhe::CudaServerKey>,
        activity_heartbeat: HeartBeat,
    ) -> Self {
        let edges = graph.graph.map(|_, _| (), |_, edge| *edge);
        Self {
            graph,
            edges,
            sks: sks.clone(),
            cpk: cpk.clone(),
            #[cfg(feature = "gpu")]
            csks: csks.clone(),
            activity_heartbeat,
        }
    }

    /// Executes the scheduled FHE operations.
    ///
    /// This is the main entry point for scheduling. It reads the `FHEVM_DF_SCHEDULE`
    /// environment variable to select the partitioning strategy and delegates to
    /// the coarse-grain scheduler.
    ///
    /// # Arguments
    ///
    /// * `loop_ctx` - OpenTelemetry context for distributed tracing
    ///
    /// # Returns
    ///
    /// `Ok(())` on successful completion of all operations, or an error if
    /// scheduling fails.
    ///
    /// # Environment Variables
    ///
    /// * `FHEVM_DF_SCHEDULE` - Scheduling strategy selection:
    ///   - `"MAX_PARALLELISM"` (default): Use parallelism-preserving partitioning
    ///   - `"MAX_LOCALITY"`: Use locality-maximizing partitioning
    pub async fn schedule(&mut self, loop_ctx: &'a opentelemetry::Context) -> Result<()> {
        let schedule_type = std::env::var("FHEVM_DF_SCHEDULE");
        match schedule_type {
            Ok(val) => match val.as_str() {
                "MAX_PARALLELISM" => {
                    self.schedule_coarse_grain(PartitionStrategy::MaxParallelism, loop_ctx)
                        .await
                }
                "MAX_LOCALITY" => {
                    self.schedule_coarse_grain(PartitionStrategy::MaxLocality, loop_ctx)
                        .await
                }
                unhandled => {
                    error!(target: "scheduler", { strategy = ?unhandled },
			   "Scheduling strategy does not exist");
                    info!(target: "scheduler", { },
			  "Reverting to default (generally best performance) strategy MAX_PARALLELISM");
                    self.schedule_coarse_grain(PartitionStrategy::MaxParallelism, loop_ctx)
                        .await
                }
            },
            // Use overall best strategy as default
            #[cfg(not(feature = "gpu"))]
            _ => {
                self.schedule_coarse_grain(PartitionStrategy::MaxParallelism, loop_ctx)
                    .await
            }
            #[cfg(feature = "gpu")]
            _ => {
                self.schedule_coarse_grain(PartitionStrategy::MaxParallelism, loop_ctx)
                    .await
            }
        }
    }

    /// Retrieves the cryptographic keys for the specified device (CPU version).
    ///
    /// On CPU, this simply returns clones of the server key and compact public key.
    /// The device selection parameter is ignored.
    ///
    /// # Arguments
    ///
    /// * `_target` - Device selection (ignored on CPU)
    ///
    /// # Returns
    ///
    /// A tuple of (ServerKey, CompactPublicKey) for FHE operations.
    #[cfg(not(feature = "gpu"))]
    fn get_keys(
        &self,
        _target: DeviceSelection,
    ) -> Result<(tfhe::ServerKey, tfhe::CompactPublicKey)> {
        Ok((self.sks.clone(), self.cpk.clone()))
    }

    /// Retrieves the cryptographic keys for the specified device (GPU version).
    ///
    /// On GPU, this returns the CUDA server key for the selected device along with
    /// the compact public key. The device selection determines which GPU's key is used.
    ///
    /// # Arguments
    ///
    /// * `target` - Device selection strategy:
    ///   - `Index(i)`: Use GPU device `i`
    ///   - `RoundRobin`: Distribute tasks across GPUs in round-robin fashion
    ///   - `NA`: Use the default device (GPU 0)
    ///
    /// # Returns
    ///
    /// A tuple of (CudaServerKey, CompactPublicKey) for GPU FHE operations.
    ///
    /// # Fallback
    ///
    /// If an invalid device index is specified, falls back to device 0 with a warning.
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
                let i = LAST.load(std::sync::atomic::Ordering::Acquire);
                LAST.store(
                    (i + 1) % self.csks.len(),
                    std::sync::atomic::Ordering::Release,
                );
                Ok((self.csks[i].clone(), self.cpk.clone()))
            }
            DeviceSelection::NA => Ok((self.csks[0].clone(), self.cpk.clone())),
        }
    }

    /// Executes the schedule using coarse-grain partitioning.
    ///
    /// This method implements the main scheduling loop:
    /// 1. Partitions the component graph according to the selected strategy
    /// 2. Spawns all initially ready partitions as blocking tasks
    /// 3. Waits for task completions and processes results
    /// 4. Propagates results to dependent partitions
    /// 5. Spawns newly ready partitions
    /// 6. Continues until all tasks are complete
    ///
    /// # Arguments
    ///
    /// * `strategy` - The partitioning strategy to use
    /// * `loop_ctx` - OpenTelemetry context for distributed tracing
    ///
    /// # Returns
    ///
    /// `Ok(())` on successful completion of all operations, or an error if
    /// scheduling fails.
    ///
    /// # Execution Model
    ///
    /// Tasks are spawned using `spawn_blocking` to avoid blocking the async
    /// runtime during CPU-intensive FHE operations. Results are collected
    /// and processed asynchronously.
    async fn schedule_coarse_grain(
        &mut self,
        strategy: PartitionStrategy,
        loop_ctx: &'a opentelemetry::Context,
    ) -> Result<()> {
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
                let loop_ctx = loop_ctx.clone();
                set.spawn_blocking(move || execute_partition(args, index, 0, sks, cpk, loop_ctx));
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
                    let loop_ctx = loop_ctx.clone();
                    set.spawn_blocking(move || {
                        execute_partition(args, dependent_task_index, 0, sks, cpk, loop_ctx)
                    });
                }
            }
        }
        Ok(())
    }
}

/// Re-randomises transaction inputs to provide sIND-CPAD security.
///
/// This function performs cryptographic re-randomisation on all input ciphertexts
/// for a transaction. Re-randomisation refreshes the randomness in ciphertexts
/// while preserving the encrypted values, which is essential for security in
/// multi-party FHE computations.
///
/// The process:
/// 1. Creates a re-randomisation context with transaction-specific domain separators
/// 2. Adds all transaction component allowed inputs to the context (decompressing if needed)
/// 3. Finalizes the context to generate a seed generator
/// 4. Re-randomises each allowed input using derived seeds
///
/// # Arguments
///
/// * `inputs` - Mutable map of input handles to their values
/// * `transaction_id` - Unique identifier for the transaction
/// * `component_id` - Index of the component within the transaction
/// * `gpu_idx` - GPU device index for decompression operations
/// * `cpk` - Compact public key for re-randomisation
///
/// # Returns
///
/// `Ok(())` on success, or an error if:
/// * An input is missing when re-randomisation is attempted
/// * Re-randomisation fails cryptographically
///
/// # Note
///
/// Only inputs marked as "allowed" are re-randomised. Non-allowed
/// inputs (intermediate results) do not need to be re-randomised as
/// they are never observable outside the coprocessor or able to be
/// reused by an attacker.
fn re_randomise_transaction_inputs(
    inputs: &mut HashMap<Handle, Option<DFGTxInput>>,
    transaction_id: &Handle,
    component_id: usize,
    gpu_idx: usize,
    cpk: tfhe::CompactPublicKey,
) -> Result<()> {
    let mut re_rand_context = ReRandomizationContext::new(
        TRANSACTION_RERANDOMISATION_DOMAIN_SEPARATOR,
        [transaction_id.as_slice(), &component_id.to_be_bytes()],
        COMPACT_PUBLIC_ENCRYPTION_DOMAIN_SEPARATOR,
    );
    for txinput in inputs.values_mut() {
        match txinput {
            Some(DFGTxInput::Value((val, true))) => {
                val.add_to_re_randomization_context(&mut re_rand_context);
            }
            Some(DFGTxInput::Value((_, false))) => {}
            Some(DFGTxInput::Compressed(((t, c), allowed))) => {
                let decomp = SupportedFheCiphertexts::decompress(*t, c, gpu_idx)?;
                decomp.add_to_rerandomisation_context(&mut re_rand_context);
                *txinput = Some(DFGTxInput::Value((decomp, *allowed)));
            }
            None => {
                error!(target: "scheduler", { transaction_id = ?hex::encode(transaction_id) },
		       "Missing transaction input while trying to re-randomise");
                return Err(SchedulerError::MissingInputs.into());
            }
        }
    }
    let mut seed_gen = re_rand_context.finalize();
    for txinput in inputs.values_mut() {
        match txinput {
            Some(DFGTxInput::Value((ref mut val, true))) => {
                val.re_randomise(&cpk, seed_gen.next_seed()?)?;
            }
            Some(DFGTxInput::Value((_, false))) => {}
            Some(DFGTxInput::Compressed(_)) => {
                error!(target: "scheduler", { transaction_id = ?hex::encode(transaction_id) },
		       "Failed to re-randomise inputs for transaction");
                return Err(SchedulerError::ReRandomisationError.into());
            }
            None => {
                error!(target: "scheduler", { transaction_id = ?hex::encode(transaction_id) },
		       "Failed to re-randomise inputs for transaction");
                return Err(SchedulerError::ReRandomisationError.into());
            }
        }
    }
    Ok(())
}

/// Type alias for a set of transaction components to execute in a partition.
/// Each tuple contains: (dataflow graph, input map, transaction ID, component ID).
type ComponentSet = Vec<(DFGraph, HashMap<Handle, Option<DFGTxInput>>, Handle, usize)>;

/// Executes all transactions within a partition.
///
/// This function is the main worker for partition execution. It processes each
/// transaction in the partition sequentially (transactions within a partition
/// are topologically sorted), handling:
/// - Input resolution from completed transactions
/// - Re-randomisation or decompression of inputs
/// - Execution of FHE operations in topological order
/// - Result collection and propagation
///
/// # Arguments
///
/// * `transactions` - Set of transaction components to execute
/// * `task_id` - Node index of this partition in the execution graph
/// * `gpu_idx` - GPU device index for computation
/// * `sks` - Server key (CPU or CUDA depending on feature flag)
/// * `cpk` - Compact public key for re-randomisation
/// * `loop_ctx` - OpenTelemetry context for distributed tracing
///
/// # Returns
///
/// A tuple containing:
/// * HashMap of computed results (handle -> TaskResult or error)
/// * The task_id of this partition (for result propagation)
///
/// # Execution Flow
///
/// For each transaction:
/// 1. Resolve any missing inputs from previously computed results
/// 2. Re-randomise inputs (CPU) or decompress them (GPU)
/// 3. Topologically sort the operations
/// 4. Execute each operation, propagating results to dependents
/// 5. Collect allowed results for output
fn execute_partition(
    transactions: ComponentSet,
    task_id: NodeIndex,
    gpu_idx: usize,
    #[cfg(not(feature = "gpu"))] sks: tfhe::ServerKey,
    #[cfg(feature = "gpu")] sks: tfhe::CudaServerKey,
    cpk: tfhe::CompactPublicKey,
    loop_ctx: opentelemetry::Context,
) -> PartitionResult {
    tfhe::set_server_key(sks);
    let mut res: HashMap<Handle, Result<TaskResult>> = HashMap::with_capacity(transactions.len());
    let tracer = opentelemetry::global::tracer("tfhe_worker");
    // Traverse transactions within the partition. The transactions
    // are topologically sorted so the order is executable
    'tx: for (ref mut dfg, ref mut tx_inputs, tid, cid) in transactions {
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
                *i = Some(DFGTxInput::Value((ct.ct.clone(), ct.is_allowed)));
            }
        }

        {
            let mut s = tracer.start_with_context("rerandomise_inputs", &loop_ctx);
            telemetry::set_txn_id(&mut s, &tid);
            let started_at = std::time::Instant::now();
            // Re-randomise inputs of the transaction - this also
            // decompresses ciphertexts
            if let Err(e) =
                re_randomise_transaction_inputs(tx_inputs, &tid, cid, gpu_idx, cpk.clone())
            {
                error!(target: "scheduler", {transaction_id = ?hex::encode(tid), error = ?e },
		       "Error while re-randomising inputs");
                for nidx in dfg.graph.node_identifiers() {
                    let Some(node) = dfg.graph.node_weight_mut(nidx) else {
                        error!(target: "scheduler", {index = ?nidx.index() }, "Wrong dataflow graph index");
                        continue;
                    };
                    if node.is_allowed {
                        res.insert(
                            node.result_handle.clone(),
                            Err(SchedulerError::ReRandomisationError.into()),
                        );
                    }
                }
                continue 'tx;
            }

            let elapsed = started_at.elapsed();
            RERAND_LATENCY_BATCH_HISTOGRAM.observe(elapsed.as_secs_f64());
            drop(s);
        }

        // Prime the scheduler with ready ops from the transaction's subgraph
        let mut s = tracer.start_with_context("execute_transaction", &loop_ctx);
        telemetry::set_txn_id(&mut s, &tid);
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
            let result = try_execute_node(node, nidx.index(), tx_inputs, gpu_idx);
            if let Ok(result) = result {
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
                                DFGTaskInput::Value(res.0.clone());
                        }
                    }
                }
                // Update partition's outputs (allowed handles only)
                let node = dfg.graph.node_weight_mut(nidx).unwrap();
                res.insert(
                    node.result_handle.clone(),
                    result.1.map(|v| TaskResult {
                        ct: v.0,
                        compressed_ct: if node.is_allowed { v.1 } else { None },
                        is_allowed: node.is_allowed,
                        transaction_id: tid.clone(),
                    }),
                );
            }
        }
        s.end();
        let elapsed = started_at.elapsed();
        FHE_BATCH_LATENCY_HISTOGRAM.observe(elapsed.as_secs_f64());
    }
    (res, task_id)
}

/// Attempts to execute a single operation node.
///
/// This function checks if all inputs are available, extracts them, and
/// delegates to the computation function. It handles input resolution from
/// both transaction-level inputs and dependence outputs.
///
/// # Arguments
///
/// * `node` - Mutable reference to the operation node to execute
/// * `node_index` - Index of this node in the dataflow graph
/// * `tx_inputs` - Map of transaction-level inputs
/// * `gpu_idx` - GPU device index for computation
///
/// # Returns
///
/// On success, returns `(node_index, OpResult)` where `OpResult` contains
/// the computed ciphertext and optionally its compressed form.
///
/// # Errors
///
/// Returns an error if:
/// * Input dependences are not satisfied
/// * Input extraction fails after dependence check (should not happen)
fn try_execute_node(
    node: &mut OpNode,
    node_index: usize,
    tx_inputs: &mut HashMap<Handle, Option<DFGTxInput>>,
    gpu_idx: usize,
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

    let opcode = node.opcode;
    let is_allowed = node.is_allowed;
    Ok(run_computation(
        opcode, cts, node_index, is_allowed, gpu_idx,
    ))
}

/// Result type for a single FHE operation: the computed ciphertext and optionally
/// its compressed form (for allowed outputs).
type OpResult = Result<(SupportedFheCiphertexts, Option<(i16, Vec<u8>)>)>;

/// Executes a single FHE operation.
///
/// This function dispatches FHE operations to the appropriate computation
/// backend. It handles the special case of `FheGetCiphertext` (which just
/// compresses an existing ciphertext) and delegates all other operations
/// to the `perform_fhe_operation` function. TODO: GetCiphertext deprecation.
///
/// # Arguments
///
/// * `operation` - The FHE operation code to execute
/// * `inputs` - Vector of input ciphertexts
/// * `graph_node_index` - Index of this node (returned with results for tracking)
/// * `is_allowed` - Whether this operation's result should be compressed
/// * `gpu_idx` - GPU device index for computation
///
/// # Returns
///
/// A tuple of (node_index, result) where result is:
/// * `Ok((ciphertext, Some((type, bytes))))` for allowed operations
/// * `Ok((ciphertext, None))` for intermediate operations
/// * `Err(...)` if the operation fails
///
/// # Special Cases
///
/// * `FheGetCiphertext`: Returns the input ciphertext compressed, without
///   performing any computation
fn run_computation(
    operation: i32,
    inputs: Vec<SupportedFheCiphertexts>,
    graph_node_index: usize,
    is_allowed: bool,
    gpu_idx: usize,
) -> (usize, OpResult) {
    let op = FheOperation::try_from(operation);
    match op {
        Ok(FheOperation::FheGetCiphertext) => {
            let (ct_type, ct_bytes) = inputs[0].compress();
            (
                graph_node_index,
                Ok((inputs[0].clone(), Some((ct_type, ct_bytes)))),
            )
        }
        Ok(_) => match perform_fhe_operation(operation as i16, &inputs, gpu_idx) {
            Ok(result) => {
                if is_allowed {
                    let (ct_type, ct_bytes) = result.compress();
                    (graph_node_index, Ok((result, Some((ct_type, ct_bytes)))))
                } else {
                    (graph_node_index, Ok((result, None)))
                }
            }
            Err(e) => (graph_node_index, Err(e.into())),
        },
        Err(e) => (graph_node_index, Err(e.into())),
    }
}
