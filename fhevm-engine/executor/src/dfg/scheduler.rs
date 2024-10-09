use std::borrow::Borrow;
use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;

use crate::dfg::types::*;
use crate::dfg::{OpEdge, OpNode};
use crate::server::{self, run_computation, InMemoryCiphertext, SyncComputeError};
use anyhow::Result;
use daggy::petgraph::csr::IndexType;
use daggy::petgraph::graph::node_index;
use daggy::petgraph::visit::{IntoEdgeReferences, IntoNeighbors, VisitMap, Visitable};
use daggy::petgraph::Direction::Incoming;
use fhevm_engine_common::types::SupportedFheCiphertexts;

use rayon::prelude::*;
use std::sync::mpsc::channel;

use daggy::{
    petgraph::{
        visit::{EdgeRef, IntoEdgesDirected},
        Direction,
    },
    Dag, NodeIndex,
};
use tokio::task::JoinSet;

struct ExecNode {
    df_nodes: Vec<NodeIndex>,
    dependence_counter: AtomicUsize,
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
}

impl<'a> Scheduler<'a> {
    fn is_ready(node: &OpNode) -> bool {
        let mut ready = true;
        for i in node.inputs.iter() {
            if let DFGTaskInput::Dep(_) = i {
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
    pub fn new(graph: &'a mut Dag<OpNode, OpEdge>) -> Self {
        let edges = graph.map(|_, _| (), |_, edge| *edge);
        Self { graph, edges }
    }

    pub async fn schedule(&mut self) -> Result<(), SyncComputeError> {
        let schedule_type = std::env::var("FHEVM_DF_SCHEDULE");
        match schedule_type {
            Ok(val) if val == "MAX_PARALLELISM" => {
                self.schedule_coarse_grain(PartitionStrategy::MaxParallelism)
                    .await
            }
            Ok(val) if val == "MAX_LOCALITY" => {
                self.schedule_coarse_grain(PartitionStrategy::MaxLocality)
                    .await
            }
            Ok(val) if val == "LOOP" => self.schedule_component_loop().await,
            Ok(val) if val == "FINE_GRAIN" => self.schedule_fine_grain().await,
            Ok(unhandled) => panic!("Scheduling strategy {:?} does not exist", unhandled),

            _ => self.schedule_component_loop().await,
        }
    }

    async fn schedule_fine_grain(&mut self) -> Result<(), SyncComputeError> {
        let mut set: JoinSet<Result<(usize, InMemoryCiphertext), SyncComputeError>> =
            JoinSet::new();
        // Prime the scheduler with all nodes without dependences
        for idx in 0..self.graph.node_count() {
            let index = NodeIndex::new(idx);
            let node = self.graph.node_weight_mut(index).unwrap();
            if Self::is_ready(node) {
                let opcode = node.opcode;
                let inputs: Result<Vec<SupportedFheCiphertexts>, SyncComputeError> = node
                    .inputs
                    .iter()
                    .map(|i| match i {
                        DFGTaskInput::Val(i) => Ok(i.clone()),
                        _ => Err(SyncComputeError::ComputationFailed),
                    })
                    .collect();
                set.spawn_blocking(move || run_computation(opcode, inputs, idx));
            }
        }
        // Get results from computations and update dependences of remaining computations
        while let Some(result) = set.join_next().await {
            let output = result.map_err(|_| SyncComputeError::ComputationFailed)??;
            let index = output.0;
            let node_index = NodeIndex::new(index);
            // Satisfy deps from the executed task
            for edge in self.edges.edges_directed(node_index, Direction::Outgoing) {
                let child_index = edge.target();
                let child_node = self.graph.node_weight_mut(child_index).unwrap();
                child_node.inputs[*edge.weight() as usize] =
                    DFGTaskInput::Val(output.1.expanded.clone());
                if Self::is_ready(child_node) {
                    let opcode = child_node.opcode;
                    let inputs: Result<Vec<SupportedFheCiphertexts>, SyncComputeError> = child_node
                        .inputs
                        .iter()
                        .map(|i| match i {
                            DFGTaskInput::Val(i) => Ok(i.clone()),
                            _ => Err(SyncComputeError::ComputationFailed),
                        })
                        .collect();
                    set.spawn_blocking(move || {
                        run_computation(opcode, inputs, child_index.index())
                    });
                }
            }
            self.graph[node_index].result = Some(output.1);
        }
        Ok(())
    }

    async fn schedule_coarse_grain(
        &mut self,
        strategy: PartitionStrategy,
    ) -> Result<(), SyncComputeError> {
        let mut set: JoinSet<
            Result<(Vec<(usize, InMemoryCiphertext)>, NodeIndex), SyncComputeError>,
        > = JoinSet::new();
        let mut execution_graph: Dag<ExecNode, ()> = Dag::default();
        match strategy {
            PartitionStrategy::MaxLocality => {
                let _ = partition_components(self.graph, &mut execution_graph);
            }
            PartitionStrategy::MaxParallelism => {
                let _ = partition_preserving_parallelism(self.graph, &mut execution_graph);
            }
        }
        let task_dependences = execution_graph.map(|_, _| (), |_, edge| *edge);

        // Prime the scheduler with all nodes without dependences
        for idx in 0..execution_graph.node_count() {
            let index = NodeIndex::new(idx);
            let node = execution_graph.node_weight_mut(index).unwrap();
            if self.is_ready_task(node) {
                let mut args = Vec::with_capacity(node.df_nodes.len());
                for nidx in node.df_nodes.iter() {
                    let n = self.graph.node_weight_mut(*nidx).unwrap();
                    let opcode = n.opcode;
                    args.push((opcode, std::mem::take(&mut n.inputs), *nidx));
                }
                set.spawn_blocking(move || execute_partition(args, index, false));
            }
        }
        // Get results from computations and update dependences of remaining computations
        while let Some(result) = set.join_next().await {
            let mut output = result.map_err(|_| SyncComputeError::ComputationFailed)??;
            let task_index = output.1;
            while let Some(o) = output.0.pop() {
                let index = o.0;
                let node_index = NodeIndex::new(index);
                // Satisfy deps from the executed computation in the DFG
                for edge in self.edges.edges_directed(node_index, Direction::Outgoing) {
                    let child_index = edge.target();
                    let child_node = self.graph.node_weight_mut(child_index).unwrap();
                    if !child_node.inputs.is_empty() {
                        child_node.inputs[*edge.weight() as usize] =
                            DFGTaskInput::Val(o.1.expanded.clone());
                    }
                }
                self.graph[node_index].result = Some(o.1);
            }
            for edge in task_dependences.edges_directed(task_index, Direction::Outgoing) {
                let dependent_task_index = edge.target();
                let dependent_task = execution_graph
                    .node_weight_mut(dependent_task_index)
                    .unwrap();
                dependent_task
                    .dependence_counter
                    .fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                if self.is_ready_task(dependent_task) {
                    let mut args = Vec::with_capacity(dependent_task.df_nodes.len());
                    for nidx in dependent_task.df_nodes.iter() {
                        let n = self.graph.node_weight_mut(*nidx).unwrap();
                        let opcode = n.opcode;
                        args.push((opcode, std::mem::take(&mut n.inputs), *nidx));
                    }
                    set.spawn_blocking(move || {
                        execute_partition(args, dependent_task_index, false)
                    });
                }
            }
        }
        Ok(())
    }

    async fn schedule_component_loop(&mut self) -> Result<(), SyncComputeError> {
        let mut execution_graph: Dag<ExecNode, ()> = Dag::default();
        let _ = partition_components(self.graph, &mut execution_graph);
        let mut comps = vec![];

        // Prime the scheduler with all nodes without dependences
        for idx in 0..execution_graph.node_count() {
            let index = NodeIndex::new(idx);
            let node = execution_graph.node_weight_mut(index).unwrap();
            if self.is_ready_task(node) {
                let mut args = Vec::with_capacity(node.df_nodes.len());
                for nidx in node.df_nodes.iter() {
                    let n = self.graph.node_weight_mut(*nidx).unwrap();
                    let opcode = n.opcode;
                    args.push((opcode, std::mem::take(&mut n.inputs), *nidx));
                }
                comps.push((std::mem::take(&mut args), index));
            }
        }

        let (src, dest) = channel();
        comps.par_iter().for_each_with(src, |src, (args, index)| {
            src.send(execute_partition(args.to_vec(), *index, true))
                .unwrap();
        });
        let results: Vec<_> = dest.iter().collect();
        for result in results {
            let mut output = result.map_err(|_| SyncComputeError::ComputationFailed)?;
            while let Some(o) = output.0.pop() {
                let index = o.0;
                let node_index = NodeIndex::new(index);
                self.graph[node_index].result = Some(o.1);
            }
        }
        Ok(())
    }
}

fn add_execution_depedences(
    graph: &Dag<OpNode, OpEdge>,
    execution_graph: &mut Dag<ExecNode, ()>,
    node_map: HashMap<NodeIndex, NodeIndex>,
) -> Result<(), SyncComputeError> {
    // Once the DFG is partitioned, we need to add dependences as
    // edges in the execution graph
    for edge in graph.edge_references() {
        let (xsrc, xdst) = (
            node_map
                .get(&edge.source())
                .ok_or(SyncComputeError::BadInputs)?,
            node_map
                .get(&edge.target())
                .ok_or(SyncComputeError::BadInputs)?,
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
) -> Result<(), SyncComputeError> {
    // First sort the DAG in a schedulable order
    let ts = daggy::petgraph::algo::toposort(graph, None)
        .map_err(|_| SyncComputeError::UnsatisfiedDependence)?;
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
) -> Result<(), SyncComputeError> {
    // First sort the DAG in a schedulable order
    let ts = daggy::petgraph::algo::toposort(graph, None)
        .map_err(|_| SyncComputeError::UnsatisfiedDependence)?;
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
            df_nodes.sort_by_key(|x| {
                tsmap
                    .get(x)
                    .ok_or(SyncComputeError::UnsatisfiedDependence)
                    .unwrap()
            });
            execution_graph
                .add_node(ExecNode {
                    df_nodes,
                    dependence_counter: AtomicUsize::new(0),
                })
                .index();
        }
    }
    // As this partition is made by coalescing all connected
    // components within the DFG, there are no dependences (edges) to
    // add to the execution graph.
    Ok(())
}

pub fn execute_partition(
    computations: Vec<(i32, Vec<DFGTaskInput>, NodeIndex)>,
    task_id: NodeIndex,
    use_global_threadpool: bool,
) -> Result<(Vec<(usize, InMemoryCiphertext)>, NodeIndex), SyncComputeError> {
    let mut res: HashMap<usize, InMemoryCiphertext> = HashMap::with_capacity(computations.len());
    for (opcode, inputs, nidx) in computations {
        let mut cts = Vec::with_capacity(inputs.len());
        for i in inputs.iter() {
            match i {
                DFGTaskInput::Dep(d) => {
                    if let Some(d) = d {
                        if let Some(ct) = res.get(d) {
                            cts.push(ct.expanded.clone());
                        }
                    } else {
                        return Err(SyncComputeError::ComputationFailed);
                    }
                }
                DFGTaskInput::Val(v) => {
                    cts.push(v.clone());
                }
            }
        }
        if use_global_threadpool {
            let (node_index, result) = run_computation(opcode, Ok(cts), nidx.index())?;
            res.insert(node_index, result);
        } else {
            let thread_pool = server::THREAD_POOL
                .borrow()
                .take()
                .ok_or(SyncComputeError::ComputationFailed)?;
            thread_pool.install(|| -> Result<(), SyncComputeError> {
                let (node_index, result) = run_computation(opcode, Ok(cts), nidx.index())?;
                res.insert(node_index, result);
                Ok(())
            })?;
            server::THREAD_POOL.set(Some(thread_pool));
        }
    }
    Ok((Vec::from_iter(res), task_id))
}
