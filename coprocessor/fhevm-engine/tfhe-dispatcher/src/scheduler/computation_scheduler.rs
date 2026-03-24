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
use fhevm_engine_common::protocol::messages::{OpNode, Status};
use fhevm_engine_common::types::Handle;
use fhevm_engine_common::{protocol::messages as msg, types::SupportedFheOperations};
use rand::seq::index;
use std::time::SystemTime;
use std::{
    collections::{HashMap, HashSet},
    default,
};
use tracing::{error, info, warn};
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
}

impl Commands for ComputationScheduler {
    /// Retrieve executable partitions based on the current state of the ExecGraph and DFG.
    fn retrieve_executable_partitions(
        &self,
        filter: HashSet<[u8; 32]>,
    ) -> Vec<msg::ExecutablePartition> {
        let ready_exec_nodes: Vec<NodeIndex> = self
            .exec_graph
            .graph()
            .node_indices()
            .filter(|nidx| self.exec_graph[*nidx].is_ready())
            .collect();

        // filter.contains(&self.dataflow_graph[**dfg_idx].output_handle)

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

            let partition =
                msg::ExecutablePartition::new(self.key_id, *exec_idx, computations.clone());

            if filter.contains(&partition.hash) {
                continue;
            }

            partition_set.push(partition);
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
        }
    }

    /// Mark the computations of the partition as executed in the DFG and update the ExecGraph accordingly.
    fn commit_partition_execution(&mut self, partition: &msg::ExecutablePartition) {
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

                let dependents: Vec<_> = self
                    .dataflow_graph
                    .edges_directed(*dfg_idx, Direction::Outgoing)
                    .map(|edge| edge.target())
                    .collect();

                // decrement their dependents dependence counters in DFG
                // TODO: duplicated
                for dependent_idx in dependents {
                    if let Status::Pending { remaining_deps } =
                        self.dataflow_graph[dependent_idx].status
                    {
                        if remaining_deps > 0 {
                            self.dataflow_graph[dependent_idx].status = Status::Pending {
                                remaining_deps: remaining_deps - 1,
                            };
                        } else {
                            warn!(
                                "DFG node {:?} has already satisfied all dependences",
                                dependent_idx
                            );
                        }
                    }
                }
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

        info!(
            pid = %partition.id(),
            exec_node_idx = ?partition.exec_node_idx,
            dependents_count = dependents.len(),
            "msg: Marking partition as executed and updating dependent Exec nodes"
        );

        for dep in dependents {
            if let Some(exec_node) = self.exec_graph.node_weight_mut(dep) {
                if exec_node.dependence_counter > 0 {
                    exec_node.dependence_counter -= 1;
                    info!(
                        pid = %partition.id(),
                        dep_exec_node_idx = ?dep,
                        remaining_deps = exec_node.dependence_counter,
                        "msg: Decremented dependence counter for dependent Exec node"
                    );
                } else {
                    warn!("Exec node {:?} has already satisfied all dependences", dep);
                }
            }
        }
    }

    fn add_computation_node(&mut self, log: &msg::FheLog) -> NodeIndex {
        if let Some(index) = self.handle_to_node_idx.get(&log.output_handle) {
            return index.clone();
        }

        let uncomputed_deps_count = log
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

        let node_idx = self.add_dfg_node(log, uncomputed_deps_count, false);

        if self
            .handle_to_node_idx
            .insert(log.output_handle.clone(), node_idx)
            .is_some()
        {
            error!("Handle has multiple producers");
        }

        let dependence_handles = log.dependence_handles();

        for handle_dep in dependence_handles.iter() {
            if let Some(&producer_idx) = self.handle_to_node_idx.get(handle_dep) {
                self.dataflow_graph
                    .add_edge(producer_idx, node_idx, ())
                    .expect("Cycle detected");
            } else {
                // A missing producer means that this computation depends on an input ciphertext
                // that has to be queried from the DataLayer
                let producer_idx = self.add_dfg_node(
                    &msg::FheLog {
                        output_handle: handle_dep.clone(),
                        fhe_operation: SupportedFheOperations::FheGetInputCiphertext,
                        created_at: log.created_at,
                        block_info: log.block_info.clone(),
                        dependencies: vec![],
                        is_scalar: false,
                        is_allowed: false,
                    },
                    0,
                    false,
                );

                self.handle_to_node_idx
                    .insert(handle_dep.clone(), producer_idx);

                self.dataflow_graph
                    .add_edge(producer_idx, node_idx, ())
                    .expect("Cycle detected");

                // TODO: Mark this as computed and decrement the uncomputed dependencies count for node_idx

                warn!(
                    "FheGetInputCiphertext producer for dependency {:?}",
                    hex::encode(handle_dep)
                );
            }
        }

        // TODO: serialize the graph after every N nodes or when the missing producers set exceeds a certain threshold,
        // to allow recovery in case of crashes and to prevent unbounded memory growth.
        // The serialization should include the mapping from handles to node indices to allow reconstructing the graph state accurately.
        // Store in Redis

        node_idx
    }

    fn add_dfg_node(
        &mut self,
        log: &msg::FheLog,
        remaining_deps: usize,
        is_computed: bool,
    ) -> NodeIndex {
        let scalar_operands = log
            .dependencies
            .iter()
            .filter_map(|dep| match dep {
                msg::Dependence::Scalar(s) => Some(s.clone()),
                _ => None,
            })
            .collect::<Vec<_>>();

        let status = if is_computed {
            Status::Computed {
                finished_at: SystemTime::now(),
            }
        } else {
            Status::Pending { remaining_deps }
        };

        self.dataflow_graph.add_node(OpNode {
            key_id: self.key_id,
            output_handle: log.output_handle.clone(),
            fhe_operation: log.fhe_operation,
            status,
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
    pub fn update_exec_graph_with_max_parallelism(&mut self) {
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
            let dependents = exec_graph
                .edges_directed(node, Incoming)
                .map(|idx| idx.source())
                .collect::<Vec<_>>();

            // Check if corresponding DFG nodes for depenent are computed, if node - increment the dependence counter
            let mut dep_count = 0;
            for dep in dependents {
                let dfg_nodes = &exec_graph.node_weight(dep).unwrap().chain;
                for dfg_node in dfg_nodes.iter() {
                    if !matches!(dfg[*dfg_node].status, Status::Computed { .. }) {
                        dep_count += 1;
                        break;
                    }
                }
                exec_graph[node].dependence_counter = dep_count;
            }
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

    /// Check if all computations corresponding to the given ExecGraph node index have been computed in the DFG.
    fn is_exec_node_computed(&self, exec_node_idx: NodeIndex) -> bool {
        let dfg_nodes = &self.exec_graph.node_weight(exec_node_idx).unwrap().chain;
        for dfg_node in dfg_nodes.iter() {
            if !matches!(
                self.dataflow_graph[*dfg_node].status,
                Status::Computed { .. }
            ) {
                return false;
            }
        }
        true
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
        self.commit_partition_execution(partition);

        self.prune();
    }
}
