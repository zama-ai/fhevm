use crate::dfg::{types::*, OpEdge, OpNode};
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
use fhevm_engine_common::types::SupportedFheCiphertexts;

use fhevm_engine_common::utils::HeartBeat;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    sync::{atomic::AtomicUsize, mpsc::channel},
};
use tokio::task::JoinSet;

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

pub struct Scheduler<'a> {
    graph: &'a mut Dag<OpNode, OpEdge>,
    edges: Dag<(), OpEdge>,
    sks: tfhe::ServerKey,
    #[cfg(feature = "gpu")]
    csks: Vec<tfhe::CudaServerKey>,
    activity_heartbeat: HeartBeat,
}

impl<'a> Scheduler<'a> {
    fn is_ready(node: &OpNode) -> bool {
        let mut ready = true;
        for i in node.inputs.iter() {
            if let DFGTaskInput::Dependence(_) = i {
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
        graph: &'a mut Dag<OpNode, OpEdge>,
        sks: tfhe::ServerKey,
        #[cfg(feature = "gpu")] csks: Vec<tfhe::CudaServerKey>,
        activity_heartbeat: HeartBeat,
    ) -> Self {
        let edges = graph.map(|_, _| (), |_, edge| *edge);
        Self {
            graph,
            edges,
            sks: sks.clone(),
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
                "LOOP" => self.schedule_component_loop().await,
                "FINE_GRAIN" => self.schedule_fine_grain().await,
                unhandled => panic!("Scheduling strategy {:?} does not exist", unhandled),
            },
            // Use overall best strategy as default
            #[cfg(not(feature = "gpu"))]
            _ => self.schedule_component_loop().await,
            #[cfg(feature = "gpu")]
            _ => {
                self.schedule_coarse_grain(PartitionStrategy::MaxLocality)
                    .await
            }
        }
    }

    #[cfg(not(feature = "gpu"))]
    async fn schedule_fine_grain(&mut self) -> Result<()> {
        let mut set: JoinSet<(usize, TaskResult)> = JoinSet::new();
        let sks = self.sks.clone();
        tfhe::set_server_key(sks.clone());
        // Prime the scheduler with all nodes without dependences
        for idx in 0..self.graph.node_count() {
            let sks = sks.clone();
            let index = NodeIndex::new(idx);
            let node = self
                .graph
                .node_weight_mut(index)
                .ok_or(SchedulerError::DataflowGraphError)?;
            if Self::is_ready(node) {
                let opcode = node.opcode;
                let is_allowed = node.is_allowed;
                let inputs: Vec<SupportedFheCiphertexts> = node
                    .inputs
                    .iter()
                    .map(|i| match i {
                        DFGTaskInput::Value(i) => Ok(i.clone()),
                        DFGTaskInput::Compressed((t, c)) => {
                            SupportedFheCiphertexts::decompress_no_memcheck(*t, c)
                        }
                        _ => Err(SchedulerError::UnsatisfiedDependence.into()),
                    })
                    .collect::<Result<Vec<_>>>()?;
                set.spawn_blocking(move || {
                    tfhe::set_server_key(sks.clone());
                    run_computation(opcode, inputs, idx, is_allowed, 0)
                });
            }
        }
        // Get results from computations and update dependences of remaining computations
        while let Some(result) = set.join_next().await {
            self.activity_heartbeat.update();
            let result = result?;
            let index = result.0;
            let node_index = NodeIndex::new(index);
            if let Ok(output) = &result.1 {
                // Satisfy deps from the executed task
                for edge in self.edges.edges_directed(node_index, Direction::Outgoing) {
                    let sks = sks.clone();
                    let child_index = edge.target();
                    let child_node = self
                        .graph
                        .node_weight_mut(child_index)
                        .ok_or(SchedulerError::DataflowGraphError)?;
                    child_node.inputs[*edge.weight() as usize] =
                        DFGTaskInput::Value(output.0.clone());
                    if Self::is_ready(child_node) {
                        let opcode = child_node.opcode;
                        let is_allowed = child_node.is_allowed;
                        let inputs: Vec<SupportedFheCiphertexts> = child_node
                            .inputs
                            .iter()
                            .map(|i| match i {
                                DFGTaskInput::Value(i) => Ok(i.clone()),
                                DFGTaskInput::Compressed((t, c)) => {
                                    SupportedFheCiphertexts::decompress_no_memcheck(*t, c)
                                }
                                _ => Err(SchedulerError::UnsatisfiedDependence.into()),
                            })
                            .collect::<Result<Vec<_>>>()?;
                        set.spawn_blocking(move || {
                            tfhe::set_server_key(sks.clone());
                            run_computation(opcode, inputs, child_index.index(), is_allowed, 0)
                        });
                    }
                }
            }
            let node_index = NodeIndex::new(result.0);
            self.graph[node_index].result = Some(result.1);
        }
        Ok(())
    }

    #[cfg(not(feature = "gpu"))]
    async fn schedule_coarse_grain(&mut self, strategy: PartitionStrategy) -> Result<()> {
        let sks = self.sks.clone();
        tfhe::set_server_key(sks.clone());
        let mut set: JoinSet<(Vec<(usize, TaskResult)>, NodeIndex)> = JoinSet::new();
        let mut execution_graph: Dag<ExecNode, ()> = Dag::default();
        let _ = match strategy {
            PartitionStrategy::MaxLocality => {
                partition_components(self.graph, &mut execution_graph)
            }
            PartitionStrategy::MaxParallelism => {
                partition_preserving_parallelism(self.graph, &mut execution_graph)
            }
        };
        let task_dependences = execution_graph.map(|_, _| (), |_, edge| *edge);

        // Prime the scheduler with all nodes without dependences
        for idx in 0..execution_graph.node_count() {
            let sks = sks.clone();
            let index = NodeIndex::new(idx);
            let node = execution_graph
                .node_weight_mut(index)
                .ok_or(SchedulerError::DataflowGraphError)?;
            if self.is_ready_task(node) {
                let mut args = Vec::with_capacity(node.df_nodes.len());
                for nidx in node.df_nodes.iter() {
                    let n = self
                        .graph
                        .node_weight_mut(*nidx)
                        .ok_or(SchedulerError::DataflowGraphError)?;
                    let opcode = n.opcode;
                    let is_allowed = n.is_allowed;
                    args.push((opcode, std::mem::take(&mut n.inputs), *nidx, is_allowed));
                }
                set.spawn_blocking(move || {
                    tfhe::set_server_key(sks.clone());
                    execute_partition(args, index, 0)
                });
            }
        }
        // Get results from computations and update dependences of remaining computations
        while let Some(result) = set.join_next().await {
            self.activity_heartbeat.update();
            let mut result = result?;
            let task_index = result.1;
            while let Some((node_index, node_result)) = result.0.pop() {
                let node_index = NodeIndex::new(node_index);
                // If this node result is an error, we can't satisfy
                // any dependences with it, so skip - all dependences
                // on this will remain unsatisfied and result in
                // further errors.
                if let Ok(ref node_result) = node_result {
                    // Satisfy deps from the executed computation in the DFG
                    for edge in self.edges.edges_directed(node_index, Direction::Outgoing) {
                        let child_index = edge.target();
                        let child_node = self
                            .graph
                            .node_weight_mut(child_index)
                            .ok_or(SchedulerError::DataflowGraphError)?;
                        if !child_node.inputs.is_empty() {
                            // Here cannot be an error
                            child_node.inputs[*edge.weight() as usize] =
                                DFGTaskInput::Value(node_result.0.clone());
                        }
                    }
                }
                self.graph[node_index].result = Some(node_result);
            }
            for edge in task_dependences.edges_directed(task_index, Direction::Outgoing) {
                let sks = sks.clone();
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
                        let n = self
                            .graph
                            .node_weight_mut(*nidx)
                            .ok_or(SchedulerError::DataflowGraphError)?;
                        let opcode = n.opcode;
                        let is_allowed = n.is_allowed;
                        args.push((opcode, std::mem::take(&mut n.inputs), *nidx, is_allowed));
                    }
                    set.spawn_blocking(move || {
                        tfhe::set_server_key(sks.clone());
                        execute_partition(args, dependent_task_index, 0)
                    });
                }
            }
        }
        Ok(())
    }

    #[cfg(not(feature = "gpu"))]
    async fn schedule_component_loop(&mut self) -> Result<()> {
        let mut execution_graph: Dag<ExecNode, ()> = Dag::default();
        let _ = partition_components(self.graph, &mut execution_graph);
        let mut comps = vec![];
        let sks = self.sks.clone();
        tfhe::set_server_key(sks.clone());
        rayon::broadcast(|_| {
            tfhe::set_server_key(sks.clone());
        });

        // Prime the scheduler with all nodes without dependences
        for idx in 0..execution_graph.node_count() {
            let index = NodeIndex::new(idx);
            let node = execution_graph
                .node_weight_mut(index)
                .ok_or(SchedulerError::DataflowGraphError)?;
            if self.is_ready_task(node) {
                let mut args = Vec::with_capacity(node.df_nodes.len());
                for nidx in node.df_nodes.iter() {
                    let n = self
                        .graph
                        .node_weight_mut(*nidx)
                        .ok_or(SchedulerError::DataflowGraphError)?;
                    let opcode = n.opcode;
                    let is_allowed = n.is_allowed;
                    args.push((opcode, std::mem::take(&mut n.inputs), *nidx, is_allowed));
                }
                comps.push((std::mem::take(&mut args), index));
            }
        }

        let (src, dest) = channel();
        tokio::task::spawn_blocking(move || {
            tfhe::set_server_key(sks.clone());
            comps.par_iter().for_each_with(src, |src, (args, index)| {
                src.send(execute_partition(args.to_vec(), *index, 0))
                    .unwrap();
            });
        })
        .await?;
        let mut results = vec![];
        for v in dest.iter() {
            self.activity_heartbeat.update();
            results.push(v);
        }
        for mut result in results {
            while let Some(o) = result.0.pop() {
                let index = o.0;
                let node_index = NodeIndex::new(index);
                self.graph[node_index].result = Some(o.1);
            }
        }
        Ok(())
    }

    #[cfg(feature = "gpu")]
    async fn schedule_fine_grain(&mut self) -> Result<()> {
        let now = std::time::SystemTime::now();
        let mut set: JoinSet<(usize, TaskResult)> = JoinSet::new();
        let keys = self.csks.clone();
        let mut rr = 0;
        // Prime the scheduler with all nodes without dependences
        for idx in 0..self.graph.node_count() {
            let index = NodeIndex::new(idx);
            let node = self
                .graph
                .node_weight_mut(index)
                .ok_or(SchedulerError::DataflowGraphError)?;
            if Self::is_ready(node) {
                let gpu_index = rr % keys.len();
                let key = keys[gpu_index].clone();
                node.locality = (gpu_index) as i32;
                rr += 1;
                tfhe::set_server_key(key.clone());
                let opcode = node.opcode;
                let is_allowed = node.is_allowed;
                let inputs: Vec<SupportedFheCiphertexts> = node
                    .inputs
                    .iter()
                    .map(|i| match i {
                        DFGTaskInput::Value(i) => Ok(i.clone()),
                        DFGTaskInput::Compressed((t, c)) => {
                            SupportedFheCiphertexts::decompress(*t, c, gpu_index)
                        }
                        _ => Err(SchedulerError::UnsatisfiedDependence.into()),
                    })
                    .collect::<Result<Vec<_>>>()?;
                set.spawn_blocking(move || {
                    tfhe::set_server_key(key);
                    run_computation(opcode, inputs, idx, is_allowed, gpu_index)
                });
            }
        }
        // Get results from computations and update dependences of remaining computations
        while let Some(result) = set.join_next().await {
            self.activity_heartbeat.update();
            let result = result?;
            let index = result.0;
            let node_index = NodeIndex::new(index);
            let loc = self.graph[node_index].locality;
            if let Ok(output) = &result.1 {
                // Satisfy deps from the executed task
                for edge in self.edges.edges_directed(node_index, Direction::Outgoing) {
                    let child_index = edge.target();
                    let child_node = self
                        .graph
                        .node_weight_mut(child_index)
                        .ok_or(SchedulerError::DataflowGraphError)?;
                    child_node.locality = loc;
                    child_node.inputs[*edge.weight() as usize] =
                        DFGTaskInput::Value(output.0.clone());
                    if Self::is_ready(child_node) {
                        let loc = if child_node.locality == -1 {
                            let loc = rr % keys.len();
                            rr += 1;
                            loc
                        } else {
                            child_node.locality as usize
                        };
                        let key = keys[loc].clone();
                        tfhe::set_server_key(key.clone());
                        let opcode = child_node.opcode;
                        let is_allowed = child_node.is_allowed;
                        let inputs: Vec<SupportedFheCiphertexts> = child_node
                            .inputs
                            .iter()
                            .map(|i| match i {
                                DFGTaskInput::Value(i) => Ok(i.clone()),
                                DFGTaskInput::Compressed((t, c)) => {
                                    SupportedFheCiphertexts::decompress(*t, c, loc)
                                }
                                _ => Err(SchedulerError::UnsatisfiedDependence.into()),
                            })
                            .collect::<Result<Vec<_>>>()?;
                        set.spawn_blocking(move || {
                            tfhe::set_server_key(key);
                            run_computation(opcode, inputs, child_index.index(), is_allowed, loc)
                        });
                    }
                }
            }
            let node_index = NodeIndex::new(result.0);
            self.graph[node_index].result = Some(result.1);
        }
        println!(
            "Scheduler time for block of {}: {}",
            self.graph.node_count(),
            now.elapsed().unwrap().as_millis()
        );
        Ok(())
    }

    #[cfg(feature = "gpu")]
    async fn schedule_coarse_grain(&mut self, strategy: PartitionStrategy) -> Result<()> {
        let keys = self.csks.clone();
        tfhe::set_server_key(keys[0].clone());
        let mut set: JoinSet<(Vec<(usize, TaskResult)>, NodeIndex)> = JoinSet::new();
        let mut execution_graph: Dag<ExecNode, ()> = Dag::default();
        let _ = match strategy {
            PartitionStrategy::MaxLocality => {
                partition_components(self.graph, &mut execution_graph)
            }
            PartitionStrategy::MaxParallelism => {
                partition_preserving_parallelism(self.graph, &mut execution_graph)
            }
        };
        let task_dependences = execution_graph.map(|_, _| (), |_, edge| *edge);
        let now = std::time::SystemTime::now();
        // Prime the scheduler with all nodes without dependences
        let mut rr = 0;
        for idx in 0..execution_graph.node_count() {
            let loc = rr % keys.len();
            let key = keys[loc].clone();
            rr += 1;
            let index = NodeIndex::new(idx);
            let node = execution_graph
                .node_weight_mut(index)
                .ok_or(SchedulerError::DataflowGraphError)?;
            node.locality = loc as i32;
            if self.is_ready_task(node) {
                let mut args = Vec::with_capacity(node.df_nodes.len());
                for nidx in node.df_nodes.iter() {
                    let n = self
                        .graph
                        .node_weight_mut(*nidx)
                        .ok_or(SchedulerError::DataflowGraphError)?;
                    let opcode = n.opcode;
                    let is_allowed = n.is_allowed;
                    args.push((opcode, std::mem::take(&mut n.inputs), *nidx, is_allowed));
                }
                set.spawn_blocking(move || {
                    tfhe::set_server_key(key);
                    execute_partition(args, index, loc)
                });
            }
        }
        // Get results from computations and update dependences of remaining computations
        while let Some(result) = set.join_next().await {
            self.activity_heartbeat.update();
            let mut result = result?;
            let task_index = result.1;
            while let Some((node_index, node_result)) = result.0.pop() {
                let node_index = NodeIndex::new(node_index);
                let loc: usize = if self.graph[node_index].locality < 0 {
                    0
                } else {
                    self.graph[node_index].locality as usize
                };
                // If this node result is an error, we can't satisfy
                // any dependences with it, so skip - all dependences
                // on this will remain unsatisfied and result in
                // further errors.
                if let Ok(ref node_result) = node_result {
                    // Satisfy deps from the executed computation in the DFG
                    for edge in self.edges.edges_directed(node_index, Direction::Outgoing) {
                        let child_index = edge.target();
                        let child_node = self
                            .graph
                            .node_weight_mut(child_index)
                            .ok_or(SchedulerError::DataflowGraphError)?;
                        if !child_node.inputs.is_empty() {
                            tfhe::set_server_key(keys[loc].clone());
                            // Here cannot be an error
                            child_node.inputs[*edge.weight() as usize] =
                                DFGTaskInput::Value(node_result.0.clone());
                        }
                    }
                }
                self.graph[node_index].result = Some(node_result);
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
                    let loc = rr % keys.len();
                    let key = keys[loc].clone();
                    dependent_task.locality = loc as i32;
                    rr += 1;
                    let mut args = Vec::with_capacity(dependent_task.df_nodes.len());
                    for nidx in dependent_task.df_nodes.iter() {
                        let n = self
                            .graph
                            .node_weight_mut(*nidx)
                            .ok_or(SchedulerError::DataflowGraphError)?;
                        let opcode = n.opcode;
                        let is_allowed = n.is_allowed;
                        args.push((opcode, std::mem::take(&mut n.inputs), *nidx, is_allowed));
                    }
                    set.spawn_blocking(move || {
                        tfhe::set_server_key(key);
                        execute_partition(args, dependent_task_index, loc)
                    });
                }
            }
        }
        println!(
            "Scheduler time for block of {}: {}",
            self.graph.node_count(),
            now.elapsed().unwrap().as_millis()
        );
        Ok(())
    }

    #[cfg(feature = "gpu")]
    async fn schedule_component_loop(&mut self) -> Result<()> {
        let mut execution_graph: Dag<ExecNode, ()> = Dag::default();
        let _ = partition_components(self.graph, &mut execution_graph);
        let mut comps = vec![];

        let now = std::time::SystemTime::now();
        // Prime the scheduler with all nodes without dependences
        for idx in 0..execution_graph.node_count() {
            let index = NodeIndex::new(idx);
            let node = execution_graph
                .node_weight_mut(index)
                .ok_or(SchedulerError::DataflowGraphError)?;
            if self.is_ready_task(node) {
                let mut args = Vec::with_capacity(node.df_nodes.len());
                for nidx in node.df_nodes.iter() {
                    let n = self
                        .graph
                        .node_weight_mut(*nidx)
                        .ok_or(SchedulerError::DataflowGraphError)?;
                    let opcode = n.opcode;
                    let is_allowed = n.is_allowed;
                    args.push((opcode, std::mem::take(&mut n.inputs), *nidx, is_allowed));
                }
                comps.push((std::mem::take(&mut args), index));
            }
        }
        if comps.is_empty() {
            return Ok(());
        }

        let keys = self.csks.clone();
        let (src, dest) = channel();
        tokio::task::spawn_blocking(move || {
            let num_streams_per_gpu = 8; // TODO: add config variable for this
            let chunk_size = comps.len() / keys.len() + (comps.len() % keys.len() != 0) as usize;

            comps
                .par_chunks(chunk_size) // Split into as many chunks as GPUs available
                .enumerate() // Get the index for GPU
                .for_each_with(src, |src, (i, comps_i)| {
                    // Process chunks within each GPU
                    let stream_chunk_size = comps_i.len() / num_streams_per_gpu
                        + (comps_i.len() % num_streams_per_gpu != 0) as usize;
                    let src = src.clone();
                    comps_i
                        .par_chunks(stream_chunk_size) // Further split chunks into as many chunks as we allow streams per GPU
                        .for_each_with(src, |src, chunk| {
                            // Set the server key for the current GPU
                            tfhe::set_server_key(keys[i].clone());
                            // Sequential iteration over the chunks of data for each stream
                            chunk.iter().for_each(|(args, index)| {
                                src.send(execute_partition(args.to_vec(), *index, i))
                                    .unwrap();
                            });
                        });
                });
        })
        .await?;
        let mut results = vec![];
        for v in dest.iter() {
            self.activity_heartbeat.update();
            results.push(v);
        }
        for mut result in results {
            while let Some(o) = result.0.pop() {
                let index = o.0;
                let node_index = NodeIndex::new(index);
                self.graph[node_index].result = Some(o.1);
            }
        }
        println!(
            "Scheduler time for block of {}: {}",
            self.graph.node_count(),
            now.elapsed().unwrap().as_millis()
        );
        Ok(())
    }
}

fn add_execution_depedences(
    graph: &Dag<OpNode, OpEdge>,
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

fn partition_preserving_parallelism(
    graph: &Dag<OpNode, OpEdge>,
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

fn partition_components(
    graph: &Dag<OpNode, OpEdge>,
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

type TaskResult = Result<(SupportedFheCiphertexts, Option<(i16, Vec<u8>)>)>;

fn execute_partition(
    computations: Vec<(i32, Vec<DFGTaskInput>, NodeIndex, bool)>,
    task_id: NodeIndex,
    gpu_idx: usize,
) -> (Vec<(usize, TaskResult)>, NodeIndex) {
    let mut res: HashMap<usize, TaskResult> = HashMap::with_capacity(computations.len());
    'comps: for (opcode, inputs, nidx, is_allowed) in computations {
        let mut cts = Vec::with_capacity(inputs.len());
        for i in inputs.iter() {
            match i {
                DFGTaskInput::Dependence(d) => {
                    if let Some(d) = d {
                        if let Some(Ok(ct)) = res.get(d) {
                            cts.push(ct.0.clone());
                        } else {
                            res.insert(
                                nidx.index(),
                                Err(SchedulerError::UnsatisfiedDependence.into()),
                            );
                            continue 'comps;
                        }
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
                        res.insert(nidx.index(), Err(decomp.err().unwrap()));
                        continue 'comps;
                    }
                }
            }
        }
        let (node_index, result) = run_computation(opcode, cts, nidx.index(), is_allowed, gpu_idx);
        res.insert(node_index, result);
    }
    (Vec::from_iter(res), task_id)
}

fn run_computation(
    operation: i32,
    inputs: Vec<SupportedFheCiphertexts>,
    graph_node_index: usize,
    is_allowed: bool,
    gpu_idx: usize,
) -> (usize, TaskResult) {
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
        _ => (
            graph_node_index,
            Err(SchedulerError::UnknownOperation(operation).into()),
        ),
    }
}
