use crate::scheduler::{
    traits::{Commands, Events},
    types::ExecNode,
};
use daggy::{
    petgraph::{
        visit::{
            EdgeRef, IntoEdgeReferences, IntoEdgesDirected, IntoNeighbors, VisitMap, Visitable,
        },
        Direction::{self, Incoming},
    },
    Dag, NodeIndex,
};
use fhevm_engine_common::protocol::messages as msg;
use fhevm_engine_common::protocol::messages::{OpNode, Status};
use fhevm_engine_common::types::Handle;
use std::collections::{HashMap, HashSet};
use std::time::SystemTime;
use tracing::{info, warn};
/// The ComputationScheduler is responsible for maintaining the Dataflow Graph (DFG) of computations
/// and the Execution Graph (ExecGraph) that represents executable partitions of the DFG.
///
/// It provides methods to process incoming FHE log messages, update the graphs incrementally,
/// and determine which partitions of computations are ready for execution based on their dependencies.
pub struct ComputationScheduler {
    key_id: u64,
    dataflow_graph: Dag<OpNode, ()>,
    exec_graph: Dag<ExecNode, ()>,

    // Mapping between DFG nodes and execution nodes
    dfg_to_exec: HashMap<NodeIndex, NodeIndex>,

    // Mapping output_handle to the DFG node that produces it
    handle_to_node_idx: HashMap<Handle, NodeIndex>,

    // Set of handles that are dependencies but whose producers have not been seen yet
    missing_producers: HashSet<Handle>,
}

impl Commands for ComputationScheduler {
    /// Retrieve executable partitions based on the current state of the ExecGraph and DFG.
    fn retrieve_executable_partitions(&self) -> Vec<msg::ExecutablePartition> {
        let ready_exec_nodes: Vec<NodeIndex> = self
            .exec_graph
            .graph()
            .node_indices()
            .filter(|nidx| self.exec_graph[*nidx].is_ready())
            .collect();

        let mut partition_set = Vec::new();
        for exec_idx in ready_exec_nodes.iter() {
            let computations = self
                .exec_graph
                .node_weight(*exec_idx)
                .expect("Exec node should exist")
                .chain
                .iter()
                .filter(|dfg_idx| {
                    matches!(
                        self.dataflow_graph[**dfg_idx].status,
                        Status::Pending { .. }
                    )
                })
                .map(|dfg_idx| {
                    (
                        self.dataflow_graph[*dfg_idx].clone(),
                        *dfg_idx,
                        self.get_dependencies(*dfg_idx),
                    )
                })
                .collect::<Vec<_>>();

            if computations.is_empty() {
                continue;
            }

            partition_set.push(msg::ExecutablePartition::new(
                self.key_id,
                *exec_idx,
                computations,
            ));
        }

        partition_set
    }
}

impl ComputationScheduler {
    pub fn new(key_id: u64) -> Self {
        Self {
            key_id,
            dataflow_graph: Dag::new(),
            exec_graph: Dag::new(),
            dfg_to_exec: HashMap::new(),
            handle_to_node_idx: HashMap::new(),
            missing_producers: HashSet::new(),
        }
    }

    fn mark_partition_executed(&mut self, partition: &msg::ExecutablePartition) {
        if partition.is_empty() {
            warn!("Attempting to mark an empty partition as executed");
            return;
        }

        // Mark all computations in the partition as computed
        for (comp, dfg_idx, _) in &partition.computations {
            if let Status::Pending { .. } = self.dataflow_graph[*dfg_idx].status {
                self.dataflow_graph[*dfg_idx].status = Status::Computed {
                    finished_at: SystemTime::now(),
                };
            } else {
                warn!(cid = ?comp.id(), "Computation is not in Pending state, cannot mark as executed");
            }
        }

        // Update dependence counters for dependent Exec nodes
        let dependents: Vec<NodeIndex> = self
            .exec_graph
            .edges_directed(partition.exec_node_idx, Direction::Outgoing)
            .map(|edge| edge.target())
            .collect();

        for dep in dependents {
            if let Some(exec_node) = self.exec_graph.node_weight_mut(dep) {
                if exec_node.dependence_counter > 0 {
                    exec_node.dependence_counter -= 1;
                } else {
                    warn!("Exec node {:?} has already satisfied all dependences", dep);
                }
            }
        }
    }

    fn add_computation_node(&mut self, log: &msg::FheLog) -> NodeIndex {
        self.missing_producers.remove(&log.output_handle);

        let uncomputed_deps = log
            .dependence_handles()
            .iter()
            .filter(|handle| match self.handle_to_node_idx.get(*handle) {
                Some(&producer) => !matches!(
                    self.dataflow_graph.node_weight(producer).unwrap().status,
                    Status::Computed { .. }
                ),
                None => true,
            })
            .count();

        let node_idx = self.add_dfg_node(log, uncomputed_deps);

        if self
            .handle_to_node_idx
            .insert(log.output_handle.clone(), node_idx)
            .is_some()
        {
            panic!("Handle has multiple producers");
        }

        let dependence_handles = log.dependence_handles();

        for dep in dependence_handles.iter() {
            if let Some(&producer) = self.handle_to_node_idx.get(dep) {
                self.dataflow_graph
                    .add_edge(producer, node_idx, ())
                    .expect("Cycle detected");
            } else {
                self.missing_producers.insert(dep.clone());
                warn!("Missing producer for dependency {:?}", hex::encode(dep));
            }
        }

        node_idx
    }

    fn add_dfg_node(&mut self, log: &msg::FheLog, remaining_deps: usize) -> NodeIndex {
        let scalar_operands = log
            .dependencies
            .iter()
            .filter_map(|dep| match dep {
                msg::Dependence::Scalar(s) => Some(s.clone()),
                _ => None,
            })
            .collect::<Vec<_>>();

        self.dataflow_graph.add_node(OpNode {
            key_id: self.key_id,
            output_handle: log.output_handle.clone(),
            fhe_operation: log.fhe_operation,
            status: Status::Pending { remaining_deps },
            is_scalar: log.is_scalar,
            created_at: log.created_at,
            block_info: log.block_info.clone(),
            scalar_operands,
        })
    }

    /// Update incrementally the execution graph as new nodes are added to the DFG,
    /// without reconstructing the entire graph.
    ///
    /// Preserve existing execution nodes and only create new ones for newly added DFG nodes.
    /// Preserve parallelism by only merging DFG nodes that form a linear chain without siblings
    fn update_exec_graph_with_max_parallelism(&mut self) {
        let dfg_graph = &self.dataflow_graph;
        let exec_graph = &mut self.exec_graph;

        // First sort the dfg_graph in a schedulable order
        let mut visited = dfg_graph.visit_map();

        // Mark already partitioned nodes as visited
        for n in self.dfg_to_exec.keys() {
            visited.visit(*n);
        }

        let mut newly_created_exec_nodes = Vec::new();

        let tsorted = daggy::petgraph::algo::toposort(dfg_graph, None).unwrap();
        for nidx in tsorted.iter() {
            if visited.is_visited(nidx) {
                continue;
            }

            visited.visit(*nidx);
            let mut chain = vec![*nidx];
            let mut stack = vec![*nidx];

            while let Some(n) = stack.pop() {
                if dfg_graph.edges_directed(n, Direction::Outgoing).count() == 1 {
                    for dependent in dfg_graph.neighbors(n) {
                        let not_pending = !matches!(
                            dfg_graph.node_weight(dependent).unwrap().status,
                            Status::Pending { .. }
                        );

                        if visited.is_visited(&dependent) || not_pending {
                            continue;
                        }

                        if dfg_graph
                            .edges_directed(dependent, Direction::Incoming)
                            .count()
                            == 1
                        {
                            visited.visit(dependent);

                            stack.push(dependent);
                            chain.push(dependent);
                        }
                    }
                }
            }

            // Create execution node only for new chain
            let exec_node = exec_graph.add_node(ExecNode {
                chain: chain.clone(),
                dependence_counter: 0,
            });

            for dfg_node_idx in chain.iter() {
                self.dfg_to_exec.insert(*dfg_node_idx, exec_node);
            }

            newly_created_exec_nodes.push(exec_node);
        }

        self.add_exec_dependences(newly_created_exec_nodes);
    }

    fn add_exec_dependences(&mut self, new_exec_nodes: Vec<NodeIndex>) {
        let dfg = &self.dataflow_graph;
        let exec_graph = &mut self.exec_graph;

        let mut touched = HashSet::new();

        for edge in dfg.edge_references() {
            let (src, dst) = (edge.source(), edge.target());

            let (Some(&xsrc), Some(&xdst)) =
                (self.dfg_to_exec.get(&src), self.dfg_to_exec.get(&dst))
            else {
                warn!(
                    "Missing exec nodes for DFG edge from {:?} to {:?}",
                    dfg[src].id(),
                    dfg[dst].id()
                );
                continue;
            };

            if xsrc != xdst && exec_graph.find_edge(xsrc, xdst).is_none() {
                exec_graph
                    .add_edge(xsrc, xdst, ())
                    .expect("ExecGraph should not have cycles");
                touched.insert(xdst);
            }
        }

        // Update dependence counters for touched nodes
        touched.extend(new_exec_nodes);

        for node in touched {
            exec_graph[node].dependence_counter = exec_graph.edges_directed(node, Incoming).count();
        }
    }

    /// Gets dependencies for a given DFG node, which are the inputs of the corresponding computation.
    fn get_dependencies(&self, dfg_idx: NodeIndex) -> Vec<Handle> {
        self.dataflow_graph
            .edges_directed(dfg_idx, Incoming)
            .map(|edge| {
                let src_idx = edge.source();
                self.dataflow_graph[src_idx].output_handle()
            })
            .collect()
    }

    pub fn prune(&mut self) {
        // TODO: implement
    }

    /// Generate DOT files and PNG images for both the DFG and ExecGraph for visualization and debugging.
    #[cfg(feature = "export-graphs")]
    pub fn export_graphs(&self, folder: &str) {
        crate::scheduler::utils::gen_dot_from_dag(&self.dataflow_graph, folder, "dfg-").unwrap();
        crate::scheduler::utils::gen_dot_from_dag(&self.exec_graph, folder, "execgraph-").unwrap();
    }
}

impl Events for ComputationScheduler {
    /// Returns the index of the newly created DFG node corresponding to the log message.
    fn on_fhe_log_msg(&mut self, log: &msg::FheLog, update_exec_graph: bool) -> NodeIndex {
        info!(log = ?log, "Adding log");
        let node_idx = self.add_computation_node(log);

        if update_exec_graph {
            self.update_exec_graph_with_max_parallelism();
        }

        node_idx
    }

    /// Returns the indices of the newly created DFG nodes corresponding to the batch of log messages.
    fn on_fhe_log_batch(&mut self, logs: &[msg::FheLog]) -> Vec<NodeIndex> {
        let node_indices = logs
            .iter()
            .map(|log| self.on_fhe_log_msg(log, false))
            .collect::<Vec<_>>();

        info!(
            count = node_indices.len(),
            "Added batch of FHE log messages to the DFG"
        );
        self.update_exec_graph_with_max_parallelism();

        info!(
            count = node_indices.len(),
            "Updated ExecGraph with new computations from the batch"
        );

        node_indices
    }

    fn on_partition_completed(&mut self, partition: &msg::ExecutablePartition) {
        self.mark_partition_executed(partition);

        self.prune();
    }
}
