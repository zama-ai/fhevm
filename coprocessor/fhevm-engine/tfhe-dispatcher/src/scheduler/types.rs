use daggy::NodeIndex;

#[cfg(feature = "tfhe-rs-compute-graph")]
pub type ComputeGraph = daggy::Dag<NodeIndex, ()>;

/// Topologically sorted, linear subgraph of DFG nodes that can be executed together as a partition.
#[cfg(not(feature = "tfhe-rs-compute-graph"))]
pub type ComputeGraph = Vec<NodeIndex>;

#[derive(Debug, Clone)]
pub struct ExecNode {
    /// A partiton of DFG nodes that can be executed
    /// This is a subset of the DFG, but with the same structure (i.e. edges between nodes are preserved).
    /// NB: Eventually this should be passed directly to tfhe-rs when `graph api` in tfhe-rs is fully implemented
    pub compute_graph: ComputeGraph,

    /// Number of ExecNode dependences that must be completed before this node can be executed.
    pub dependence_counter: usize,

    /// Shortened hash of the partition that this ExecNode corresponds to.
    /// For debugging and tracibility purposes.
    pub pid: String,
}

impl ExecNode {
    /// Check if all dependences have been satisfied
    pub fn is_ready(&self) -> bool {
        self.dependence_counter == 0
    }
}
