use crate::{
    dfg::{types::*, TxEdge},
    FHE_BATCH_LATENCY_HISTOGRAM, RERAND_LATENCY_BATCH_HISTOGRAM,
};
use anyhow::Result;
use daggy::{
    petgraph::{
        csr::IndexType,
        graph::node_index,
        visit::{
            EdgeRef, IntoEdgeReferences, IntoEdgesDirected, IntoNeighbors, IntoNodeIdentifiers,
            VisitMap, Visitable,
        },
        Direction::{self, Incoming},
    },
    Dag, NodeIndex,
};
use fhevm_engine_common::tfhe_ops::perform_fhe_operation;
use fhevm_engine_common::types::{Handle, SupportedFheCiphertexts};
use fhevm_engine_common::utils::HeartBeat;
use fhevm_engine_common::{common::FheOperation, telemetry};
use opentelemetry::trace::{Span, Tracer};
use std::{collections::HashMap, sync::atomic::AtomicUsize};
use tfhe::ReRandomizationContext;
use tokio::task::JoinSet;
use tracing::{error, info, warn};

use super::{DFGraph, DFTxGraph, OpNode};

const TRANSACTION_RERANDOMISATION_DOMAIN_SEPARATOR: [u8; 8] = *b"TFHE_Rrd";
const COMPACT_PUBLIC_ENCRYPTION_DOMAIN_SEPARATOR: [u8; 8] = *b"TFHE_Enc";

struct ExecNode {
    df_nodes: Vec<NodeIndex>,
    dependence_counter: AtomicUsize,
    #[cfg(feature = "gpu")]
    locality: i32,
}

pub enum PartitionStrategy {
    MaxParallelism,
    MaxLocality,
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

enum DeviceSelection {
    #[allow(dead_code)]
    Index(usize),
    RoundRobin,
    #[allow(dead_code)]
    NA,
}

pub struct Scheduler<'a> {
    graph: &'a mut DFTxGraph,
    edges: Dag<(), TxEdge>,
    sks: tfhe::ServerKey,
    cpk: tfhe::CompactPublicKey,
    #[cfg(feature = "gpu")]
    csks: Vec<tfhe::CudaServerKey>,
    activity_heartbeat: HeartBeat,
}

impl<'a> Scheduler<'a> {
    fn is_ready_task(&self, node: &ExecNode) -> bool {
        node.dependence_counter
            .load(std::sync::atomic::Ordering::SeqCst)
            == 0
    }
    pub fn new(
        graph: &'a mut DFTxGraph,
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
                if i > self.csks.len() {
                    error!(target: "scheduler", {index = ?i },
			   "Wrong device index");
                    // Instead of giving up, we'll use device 0 (which
                    // should always be safe to use) and keep making
                    // progress even if suboptimally
                    Ok((self.csks[0].clone(), self.cpk.clone()))
                } else {
                    Ok((self.csks[i].clone(), self.cpk.clone()))
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
        let mut set: JoinSet<(HashMap<Handle, TaskResult>, NodeIndex)> = JoinSet::new();
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
                    ));
                }
                let (sks, cpk) = self.get_keys(DeviceSelection::RoundRobin)?;
                let loop_ctx = loop_ctx.clone();
                set.spawn(
                    async move { execute_partition(args, index, 0, sks, cpk, &loop_ctx).await },
                );
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
                        ));
                    }
                    let (sks, cpk) = self.get_keys(DeviceSelection::RoundRobin)?;
                    let loop_ctx = loop_ctx.clone();
                    set.spawn(async move {
                        execute_partition(args, dependent_task_index, 0, sks, cpk, &loop_ctx).await
                    });
                }
            }
        }
        Ok(())
    }
}

fn add_execution_depedences<TNode, TEdge>(
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

fn partition_preserving_parallelism<TNode, TEdge>(
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

fn partition_components<TNode, TEdge>(
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

fn re_randomise_transaction_inputs(
    inputs: &mut HashMap<Handle, Option<DFGTxInput>>,
    transaction_id: &Handle,
    gpu_idx: usize,
    cpk: tfhe::CompactPublicKey,
) -> Result<()> {
    let mut re_rand_context = ReRandomizationContext::new(
        TRANSACTION_RERANDOMISATION_DOMAIN_SEPARATOR,
        [transaction_id.as_slice()],
        COMPACT_PUBLIC_ENCRYPTION_DOMAIN_SEPARATOR,
    );
    for txinput in inputs.values_mut() {
        match txinput {
            Some(DFGTxInput::Value(val)) => {
                val.add_to_re_randomization_context(&mut re_rand_context);
            }
            Some(DFGTxInput::Compressed((t, c))) => {
                let decomp = SupportedFheCiphertexts::decompress(*t, c, gpu_idx)?;
                decomp.add_to_rerandomisation_context(&mut re_rand_context);
                *txinput = Some(DFGTxInput::Value(decomp));
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
            Some(DFGTxInput::Value(ref mut val)) => {
                val.re_randomise(&cpk, seed_gen.next_seed()?)?;
            }
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
fn decompress_transaction_inputs(
    inputs: &mut HashMap<Handle, Option<DFGTxInput>>,
    transaction_id: &Handle,
    gpu_idx: usize,
    _cpk: tfhe::CompactPublicKey,
) -> Result<()> {
    // TODO: implement re-randomisation on GPU. For now just decompress inputs
    for txinput in inputs.values_mut() {
        match txinput {
            Some(DFGTxInput::Value(_)) => {}
            Some(DFGTxInput::Compressed((t, c))) => {
                let decomp = SupportedFheCiphertexts::decompress(*t, c, gpu_idx)?;
                *txinput = Some(DFGTxInput::Value(decomp));
            }
            None => {
                error!(target: "scheduler", { transaction_id = ?hex::encode(transaction_id) },
		       "Missing transaction input while trying to decompress");
                return Err(SchedulerError::MissingInputs.into());
            }
        }
    }
    Ok(())
}

type TaskResult = Result<(SupportedFheCiphertexts, i16, Vec<u8>)>;
async fn execute_partition(
    transactions: Vec<(DFGraph, HashMap<Handle, Option<DFGTxInput>>, Handle)>,
    task_id: NodeIndex,
    gpu_idx: usize,
    #[cfg(not(feature = "gpu"))] sks: tfhe::ServerKey,
    #[cfg(feature = "gpu")] sks: tfhe::CudaServerKey,
    cpk: tfhe::CompactPublicKey,
    loop_ctx: &opentelemetry::Context,
) -> (HashMap<Handle, TaskResult>, NodeIndex) {
    let mut res: HashMap<Handle, TaskResult> = HashMap::with_capacity(transactions.len());
    let tracer = opentelemetry::global::tracer("tfhe_worker");
    // Traverse transactions within the partition. The transactions
    // are topologically sorted so the order is executable
    'tx: for (ref mut dfg, ref mut tx_inputs, tid) in transactions {
        tfhe::set_server_key(sks.clone());
        // Update the transaction inputs based on allowed handles so
        // far. If any input is still missing, and we cannot fill it
        // (e.g., error in the producer transaction) we cannot execute
        // this transaction and possibly more downstream.
        for (h, i) in tx_inputs.iter_mut() {
            if i.is_none() {
                let Some(Ok(ct)) = res.get(h) else {
                    warn!(target: "scheduler", {transaction_id = ?tid },
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
                *i = Some(DFGTxInput::Value(ct.0.clone()));
            }
        }

        if !cfg!(feature = "gpu") {
            let mut s = tracer.start_with_context("rerandomise_inputs", loop_ctx);
            telemetry::set_txn_id(&mut s, &tid);
            let started_at = std::time::Instant::now();
            // Re-randomise inputs of the transaction - this also
            // decompresses ciphertexts
            if let Err(e) = re_randomise_transaction_inputs(tx_inputs, &tid, gpu_idx, cpk.clone()) {
                error!(target: "scheduler", {transaction_id = ?tid, error = ?e },
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
        } else {
            let mut s = tracer.start_with_context("decompress_transaction_inputs", loop_ctx);
            telemetry::set_txn_id(&mut s, &tid);
            // If re-randomisation is not available (e.g., on GPU),
            // only decompress ciphertexts
            if let Err(e) = decompress_transaction_inputs(tx_inputs, &tid, gpu_idx, cpk.clone()) {
                error!(target: "scheduler", {transaction_id = ?tid, error = ?e },
		       "Error while decompressing inputs");
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
            drop(s);
        }

        // Prime the scheduler with ready ops from the transaction's subgraph
        let mut s = tracer.start_with_context("execute_transaction", loop_ctx);
        telemetry::set_txn_id(&mut s, &tid);
        let started_at = std::time::Instant::now();

        let mut set: JoinSet<(usize, OpResult)> = JoinSet::new();
        for nidx in dfg.graph.node_identifiers() {
            let Some(node) = dfg.graph.node_weight_mut(nidx) else {
                error!(target: "scheduler", {index = ?nidx.index() }, "Wrong dataflow graph index");
                continue;
            };
            try_schedule_node(
                node,
                nidx.index(),
                &mut set,
                tx_inputs,
                gpu_idx,
                sks.clone(),
            );
        }
        let edges = dfg.graph.map(|_, _| (), |_, edge| *edge);
        while let Some(result) = set.join_next().await {
            tfhe::set_server_key(sks.clone());
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
                        try_schedule_node(
                            child_node,
                            child_index.index(),
                            &mut set,
                            tx_inputs,
                            gpu_idx,
                            sks.clone(),
                        );
                    }
                }
                // Update partition's outputs (allowed handles only)
                let node = dfg.graph.node_weight_mut(nidx).unwrap();
                if node.is_allowed {
                    res.insert(
                        node.result_handle.clone(),
                        result
                            .1
                            .map(|v| (v.0, v.1.as_ref().unwrap().0, v.1.unwrap().1)),
                    );
                }
            }
        }
        s.end();
        let elapsed = started_at.elapsed();
        FHE_BATCH_LATENCY_HISTOGRAM.observe(elapsed.as_secs_f64());
    }
    (res, task_id)
}

fn try_schedule_node(
    node: &mut OpNode,
    node_index: usize,
    set: &mut JoinSet<(usize, OpResult)>,
    tx_inputs: &mut HashMap<Handle, Option<DFGTxInput>>,
    gpu_idx: usize,
    #[cfg(not(feature = "gpu"))] sks: tfhe::ServerKey,
    #[cfg(feature = "gpu")] sks: tfhe::CudaServerKey,
) {
    if !node.check_ready_inputs(tx_inputs) {
        return;
    }
    let mut cts = Vec::with_capacity(node.inputs.len());
    for i in std::mem::take(&mut node.inputs) {
        if let DFGTaskInput::Value(i) = i {
            cts.push(i);
        } else {
            // That should not be possible as we called the checker.
            error!(target: "scheduler", { handle = ?node.result_handle }, "Computation missing inputs");
            return;
        }
    }

    let opcode = node.opcode;
    let is_allowed = node.is_allowed;
    let sks = sks.clone();
    set.spawn_blocking(move || {
        tfhe::set_server_key(sks.clone());
        run_computation(opcode, cts, node_index, is_allowed, gpu_idx)
    });
}

type OpResult = Result<(SupportedFheCiphertexts, Option<(i16, Vec<u8>)>)>;
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
