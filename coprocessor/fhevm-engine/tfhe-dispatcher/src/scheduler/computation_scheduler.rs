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
use fhevm_engine_common::protocol::messages::{ExecutablePartition, OpNode, PartitionHash, Status};
use fhevm_engine_common::types::Handle;
use fhevm_engine_common::{protocol::messages as msg, types::SupportedFheOperations};

use std::collections::{HashMap, HashSet};
use std::time::SystemTime;
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
        filter: HashSet<PartitionHash>,
    ) -> Vec<msg::ExecutablePartition> {
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

            let partition = msg::ExecutablePartition::new(self.key_id, *exec_idx, computations);

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

                // decrement their dependents' dependence counters in DFG
                for dependent_idx in dependents {
                    Self::dec_dependence_counter(&mut self.dataflow_graph[dependent_idx].status);
                }
            } else {
                warn!(op_type = ?comp.fhe_operation, "Computation is not in Pending state, cannot mark as executed");
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
            "Marking partition as executed and updating dependent Exec nodes"
        );

        for dep in dependents {
            if let Some(exec_node) = self.exec_graph.node_weight_mut(dep) {
                if exec_node.dependence_counter > 0 {
                    exec_node.dependence_counter -= 1;
                    info!(
                        pid = %partition.id(),
                        dep_exec_node_idx = ?dep,
                        remaining_deps = exec_node.dependence_counter,
                        "Decremented dependence counter for dependent Exec node"
                    );
                } else {
                    warn!("Exec node {:?} has already satisfied all dependencies", dep);
                }
            }
        }
    }

    fn add_computation_node(&mut self, log: &msg::FheLog) -> Result<NodeIndex, String> {
        if let Some(index) = self.handle_to_node_idx.get(&log.output_handle) {
            return Ok(*index);
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
            return Err(format!(
                "handle {} has multiple producers",
                hex::encode(&log.output_handle),
            ));
        }

        let dependence_handles = log.dependence_handles();

        for handle_dep in dependence_handles.iter() {
            if let Some(&producer_idx) = self.handle_to_node_idx.get(handle_dep) {
                self.dataflow_graph
                    .add_edge(producer_idx, node_idx, ())
                    .map_err(|_| format!("Cycle detected when adding edge from producer {:?} to consumer {:?} for handle {:?}", producer_idx, node_idx, hex::encode(handle_dep)))?;
            } else {
                // TODO: what if the missing producer is actually not a FheGetInputCiphertext

                // A missing producer means that this computation depends on an input ciphertext
                // that has to be queried from DataLayer
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
                    true,
                );

                self.handle_to_node_idx
                    .insert(handle_dep.clone(), producer_idx);

                self.dataflow_graph
                    .add_edge(producer_idx, node_idx, ())
                    .map_err(|_| format!("Cycle detected when adding edge from synthetic producer {:?} to consumer {:?} for handle {:?}", producer_idx, node_idx, hex::encode(handle_dep)))?;

                // This synthetic FheGetInputCiphertext producer has no dependencies of its own,
                // so the dependency it represents for node_idx is already satisfied.
                // Decrement the uncomputed dependency count for node_idx.
                if let Some(node) = self.dataflow_graph.node_weight_mut(node_idx) {
                    Self::dec_dependence_counter(&mut node.status);
                }

                info!(
                    "FheGetInputCiphertext producer for dependency {:?}",
                    hex::encode(handle_dep)
                );
            }
        }

        Ok(node_idx)
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
    pub fn update_exec_graph_with_max_parallelism(&mut self) -> Result<(), String> {
        let dfg_graph = &self.dataflow_graph;
        let exec_graph = &mut self.exec_graph;

        // First sort the dfg_graph in a schedulable order
        let mut visited = dfg_graph.visit_map();

        // Mark already partitioned nodes as visited
        for n in self.dfg_to_exec.keys() {
            visited.visit(*n);
        }

        let tsorted = match daggy::petgraph::algo::toposort(dfg_graph, None) {
            Ok(tsorted) => tsorted,
            Err(cycle) => {
                return Err(format!(
                    "Cycle detected in dataflow graph while updating execution graph: \
                     cycle involves node index {:?}",
                    cycle.node_id()
                ));
            }
        };

        let mut newly_created_exec_nodes = Vec::new();

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

            let pid = ExecutablePartition::compute_id(
                &chain
                    .iter()
                    .map(|idx| dfg_graph[*idx].output_handle())
                    .collect::<Vec<&Handle>>(),
            );

            // Create execution node only for new chain
            let exec_node = exec_graph.add_node(ExecNode {
                chain: chain.clone(),
                dependence_counter: 0,
                pid,
            });

            for dfg_node_idx in chain.iter() {
                self.dfg_to_exec.insert(*dfg_node_idx, exec_node);
            }

            newly_created_exec_nodes.push(exec_node);
        }

        self.add_exec_dependencies(newly_created_exec_nodes);
        Ok(())
    }

    fn add_exec_dependencies(&mut self, new_exec_nodes: Vec<NodeIndex>) {
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
            let dependencies = exec_graph
                .edges_directed(node, Incoming)
                .map(|idx| idx.source())
                .collect::<Vec<_>>();

            // Check if corresponding DFG nodes for dependencies are computed; if not computed, increment the dependence counter
            let mut dep_count = 0;
            for dep in dependencies {
                let dfg_nodes = &exec_graph.node_weight(dep).unwrap().chain;
                for dfg_node in dfg_nodes.iter() {
                    if !matches!(dfg[*dfg_node].status, Status::Computed { .. }) {
                        dep_count += 1;
                        break;
                    }
                }
            }
            exec_graph[node].dependence_counter = dep_count;
        }
    }

    /// Gets dependencies for a given DFG node, which are the inputs of the corresponding computation.
    fn get_dependencies(&self, dfg_idx: NodeIndex) -> Vec<&Handle> {
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

    pub fn stats(&self) {
        info!(
            tag = "stats",
            dfg_nodes = self.dataflow_graph.node_count(),
            dfg_edges = self.dataflow_graph.edge_count(),
            exec_nodes = self.exec_graph.node_count(),
            exec_edges = self.exec_graph.edge_count(),
        );
    }

    /// Decrement the dependence counter stored in the given `Status` value for a DFG node.
    fn dec_dependence_counter(status: &mut Status) {
        match status {
            Status::Pending { remaining_deps } if *remaining_deps > 0 => {
                *remaining_deps -= 1;
            }
            Status::Pending { .. } => {
                warn!("dependency underflow");
            }
            Status::Computed { .. } => {
                warn!("Attempting to decrement dependence counter for a node that is already computed");
            }
            Status::Malformed { error_code } => {
                warn!("Attempting to decrement dependence counter for a node that is malformed with error code {}", error_code);
            }
        }
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
    fn on_fhe_log_msg(
        &mut self,
        log: &msg::FheLog,
        update_exec_graph: bool,
    ) -> Result<NodeIndex, String> {
        info!(log = ?log, "Adding log");
        let res = self.add_computation_node(log);

        if update_exec_graph {
            if let Err(err) = self.update_exec_graph_with_max_parallelism() {
                error!(error = ?err, "Failed to update ExecGraph");
            } else {
                info!("ExecGraph updated");
            }
        }

        // TODO: serialize the graph after every N nodes or when the missing producers set exceeds a certain threshold,
        // to allow recovery in case of crashes and to prevent unbounded memory growth.
        // The serialization should include the mapping from handles to node indices to allow reconstructing the graph state accurately.
        // Store in Redis
        res
    }

    fn on_fhe_log_batch(&mut self, logs: &[msg::FheLog]) -> Result<(), String> {
        let results = logs
            .iter()
            .map(|log| self.on_fhe_log_msg(log, false))
            .collect::<Vec<_>>();

        // Count Ok and Err results and log them
        let ok_count = results.iter().filter(|a| a.is_ok()).count();
        let err_count = results.iter().filter(|a| a.is_err()).count();

        if ok_count == logs.len() {
            info!(ops_count = ok_count, "Successfully added all FHE ops");
        } else {
            error!(
                err_count = err_count,
                total = logs.len(),
                "Failed to add some FHE ops"
            );
        }

        if let Err(err) = self.update_exec_graph_with_max_parallelism() {
            error!(error = ?err, "Failed to update ExecGraph");
        } else {
            info!("ExecGraph updated");
        }

        Ok(())
    }

    fn on_partition_completed(&mut self, partition: &msg::ExecutablePartition) {
        self.commit_partition_execution(partition);

        self.prune();
    }
}
