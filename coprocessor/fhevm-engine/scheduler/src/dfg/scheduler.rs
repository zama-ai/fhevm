use crate::dfg::{types::*, TxEdge, TxNode};
use anyhow::Result;
use daggy::{
    petgraph::{
        csr::IndexType,
        graph::node_index,
        visit::{
            EdgeRef, IntoEdgeReferences, IntoEdgesDirected, IntoNeighbors, VisitMap, Visitable,
        },
        Direction,
        Direction::Incoming,
    },
    Dag, NodeIndex,
};
use fhevm_engine_common::common::FheOperation;
use fhevm_engine_common::tfhe_ops::perform_fhe_operation;
use fhevm_engine_common::types::{Handle, SupportedFheCiphertexts};
use fhevm_engine_common::utils::HeartBeat;
use std::{collections::HashMap, sync::atomic::AtomicUsize};
use tfhe::ReRandomizationContext;
use tokio::task::JoinSet;

use super::{DFGraph, DFTxGraph};

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
    Index(usize),
    RoundRobin,
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
    fn is_ready(node: &TxNode) -> bool {
        let mut ready = true;
        for (_, i) in node.inputs.iter() {
            if i.is_none() {
                ready = false;
            }
        }
        ready
    }
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
                unhandled => panic!("Scheduling strategy {:?} does not exist", unhandled),
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
                if i > self.csks.len() {
                    return Err(SchedulerError::SchedulerError.into());
                }
                Ok((self.csks[i].clone(), self.cpk.clone()))
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

    async fn schedule_coarse_grain(&mut self, strategy: PartitionStrategy) -> Result<()> {
        let now = std::time::SystemTime::now();
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
                set.spawn_blocking(move || {
                    tfhe::set_server_key(sks.clone());
                    execute_partition(args, index, 0, cpk)
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
            for (handle, node_result) in result.0.into_iter() {
                // If this node result is an error, we can't satisfy
                // any dependences with it, so skip - all dependences
                // on this will remain unsatisfied and result in
                // further errors.
                //if let Ok(ref mut node_result) = node_result {
                //let mut swap_val = SupportedFheCiphertexts::Scalar(vec![]);
                //std::mem::swap(&mut node_result.0, &mut swap_val);
                self.graph.add_output(&handle, node_result, &self.edges)?;
            }
            //}
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
                        args.push((
                            std::mem::take(&mut tx.graph),
                            std::mem::take(&mut tx.inputs),
                            tx.transaction_id.clone(),
                        ));
                    }
                    let (sks, cpk) = self.get_keys(DeviceSelection::RoundRobin)?;
                    set.spawn_blocking(move || {
                        tfhe::set_server_key(sks.clone());
                        execute_partition(args, dependent_task_index, 0, cpk)
                    });
                }
            }
        }
        println!(
            "Scheduler time for block of {}: {}",
            self.graph.graph.node_count(),
            now.elapsed().unwrap().as_millis()
        );
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
            // Apply topsort to component nodes
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

// TODO: support re-randomisation on GPU
#[cfg(not(feature = "gpu"))]
fn re_randomise_transaction_inputs(
    inputs: &mut HashMap<Handle, Option<DFGTxInput>>,
    transaction_id: &Handle,
    gpu_idx: usize,
    cpk: tfhe::CompactPublicKey,
) -> Result<()> {
    let transaction_rerandomisation_domain_separator = *b"TFHE-w.r";
    let compact_public_encryption_domain_separator = *b"TFHE.Enc";

    let now = std::time::SystemTime::now();

    let mut re_rand_context = ReRandomizationContext::new(
        transaction_rerandomisation_domain_separator,
        // First is the function description, second is a nonce
        [transaction_id.as_slice()],
        compact_public_encryption_domain_separator,
    );
    for txinput in inputs.values_mut() {
        match txinput {
            Some(DFGTxInput::Value(val)) => {
                val.add_to_re_randomization_context(&mut re_rand_context);
            }
            Some(DFGTxInput::Compressed((t, c))) => {
                let decomp = SupportedFheCiphertexts::decompress(*t, c, gpu_idx);
                if let Ok(decomp) = decomp {
                    decomp.add_to_rerandomisation_context(&mut re_rand_context);
                    *txinput = Some(DFGTxInput::Value(decomp));
                } else {
                    // If any input fails, we set the input as empty
                    *txinput = None;
                }
            }
            None => {}
        }
    }
    let mut seed_gen = re_rand_context.finalize();
    for txinput in inputs.values_mut() {
        match txinput {
            Some(DFGTxInput::Value(val)) => {
                *txinput = Some(DFGTxInput::Value(
                    val.re_randomise(&cpk, seed_gen.next_seed())?,
                ));
            }
            Some(DFGTxInput::Compressed(_)) => {
                panic!("N compressed inputs should remain here");
            }
            None => {}
        }
    }
    println!(
        "RERAND execution time: {} -- {:?}",
        now.elapsed().unwrap().as_millis(),
        now
    );
    Ok(())
}
#[cfg(feature = "gpu")]
fn re_randomise_transaction_inputs(
    _inputs: &mut HashMap<Handle, Option<DFGTxInput>>,
    _transaction_id: &Handle,
    _gpu_idx: usize,
    _cpk: tfhe::CompactPublicKey,
) -> Result<()> {
    Ok(())
}

type TaskResult = Result<(SupportedFheCiphertexts, i16, Vec<u8>)>;
fn execute_partition(
    transactions: Vec<(DFGraph, HashMap<Handle, Option<DFGTxInput>>, Handle)>,
    task_id: NodeIndex,
    gpu_idx: usize,
    cpk: tfhe::CompactPublicKey,
) -> (HashMap<Handle, TaskResult>, NodeIndex) {
    let mut res: HashMap<Handle, TaskResult> = HashMap::with_capacity(transactions.len());
    // Traverse transactions within the partition. The transactions
    // are topologically sorted so the order is executable
    'tx: for (ref mut dfg, ref mut tx_inputs, tid) in transactions {
        // Update the transaction inputs based on allowed handles so
        // far. If any input is still missing, and we cannot fill it
        // (e.g., error in the producer transaction) we cannot execute
        // this transaction and possibly more downstream.
        for (h, i) in tx_inputs.iter_mut() {
            if i.is_none() {
                let Some(Ok(ct)) = res.get(h) else {
                    continue 'tx;
                };
                *i = Some(DFGTxInput::Value(ct.0.clone()));
            }
        }
        // Re-randomise inputs of the transaction
        if let Err(e) = re_randomise_transaction_inputs(tx_inputs, &tid, gpu_idx, cpk.clone()) {
            res.insert(tid.clone(), Err(e));
            continue 'tx;
        }
        // Sort the transaction's subgraph in an executable order
        let ts = daggy::petgraph::algo::toposort(&dfg.graph, None)
            .map_err(|_| SchedulerError::CyclicDependence)
            .unwrap();
        // Traverse and execute the transaction's subgraph
        // TODO: parallelise this
        'ops: for nidx in ts {
            let node = dfg.graph.node_weight_mut(nidx).unwrap();
            let mut cts = Vec::with_capacity(node.inputs.len());
            for i in node.inputs.iter() {
                match i {
                    DFGTaskInput::Dependence(h) => {
                        if let Some(Some(txinput)) = tx_inputs.get(h) {
                            match txinput {
                                DFGTxInput::Value(val) => cts.push(val.clone()),
                                DFGTxInput::Compressed((t, c)) => {
                                    let decomp =
                                        SupportedFheCiphertexts::decompress(*t, c, gpu_idx);
                                    if let Ok(decomp) = decomp {
                                        cts.push(decomp);
                                    } else {
                                        res.insert(h.clone(), Err(decomp.err().unwrap()));
                                        continue 'ops;
                                    }
                                }
                            }
                        } else {
                            // If any of the operation's inputs is
                            // unavailable, mark it as an error.
                            println!("MISSING INPUT {:?}", h);
                            res.insert(
                                h.clone(),
                                Err(SchedulerError::UnsatisfiedDependence.into()),
                            );
                            continue 'ops;
                        }
                    }
                    DFGTaskInput::Value(v) => {
                        cts.push(v.clone());
                    }
                    DFGTaskInput::Compressed((t, c)) => {
                        let decomp = SupportedFheCiphertexts::decompress(*t, c, gpu_idx);
                        if let Ok(decomp) = decomp {
                            cts.push(decomp);
                        } else {
                            res.insert(Handle::default(), Err(decomp.err().unwrap()));
                            continue 'ops;
                        }
                    }
                }
            }

            let (_, op_res) =
                run_computation(node.opcode, cts, nidx.index(), node.is_allowed, gpu_idx);
            // Update inputs available within the transaction with the
            // output of this operation
            tx_inputs.insert(
                node.result_handle.clone(),
                if let Ok(ref res) = op_res {
                    Some(DFGTxInput::Value(res.0.clone()))
                } else {
                    None
                },
            );
            // Update partition's outputs (allowed handles)
            if node.is_allowed {
                res.insert(
                    node.result_handle.clone(),
                    op_res.map(|v| (v.0, v.1.as_ref().unwrap().0, v.1.unwrap().1)),
                );
            }
        }
    }
    (res, task_id)
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
