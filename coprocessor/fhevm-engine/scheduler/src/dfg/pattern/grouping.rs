use std::collections::{HashMap, HashSet};

use daggy::petgraph::algo::toposort;
use daggy::Dag;

use super::encoding::{
    build_hash_pattern, encode_subgraph, encode_subgraph_hashable, finalize_pattern,
    PATTERN_HASH_THRESHOLD,
};
use crate::dfg::types::DFGTaskInput;
use crate::dfg::{DFGOp, OpEdge};
use fhevm_engine_common::types::SupportedFheOperations;

/// A simple union-find (disjoint-set) data structure.
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }
    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]]; // path splitting
            x = self.parent[x];
        }
        x
    }
    fn union(&mut self, a: usize, b: usize) {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return;
        }
        if self.rank[ra] < self.rank[rb] {
            self.parent[ra] = rb;
        } else if self.rank[ra] > self.rank[rb] {
            self.parent[rb] = ra;
        } else {
            self.parent[rb] = ra;
            self.rank[ra] += 1;
        }
    }
}

/// Returns true if the operation is a "source" node — input provisioning that
/// should be excluded from logical-operation cones.
fn is_source_op(op: &DFGOp) -> bool {
    matches!(
        op.fhe_op,
        SupportedFheOperations::FheTrivialEncrypt
            | SupportedFheOperations::FheRand
            | SupportedFheOperations::FheRandBounded
    ) && op
        .inputs
        .iter()
        .all(|i| !matches!(i, DFGTaskInput::Dependence(_)))
}

/// Compute logical-operation pattern IDs on the **pre-partition** graph.
///
/// Returns a map from op_index → pattern_id (compact binary encoding).
/// Every op in the same logical operation gets the same pattern_id.
///
/// # Algorithm
///
/// 1. **Classify nodes** — `TrivialEncrypt`, `FheRand`, `FheRandBounded`
///    that have no FHE dependencies are **source nodes** (input provisioning).
///    They are excluded from cone tracing but inherit their group's
///    `pattern_id` afterward if they feed exactly one group.
///
/// 2. **Backward cones** — From each `is_allowed=true` computation node,
///    walk backward collecting every non-source computation node. Stop at
///    other `is_allowed=true` nodes (they mark the boundary of another cone).
///
/// 3. **Union-find merge** — Cones sharing any node get merged.
///
/// 4. **Encode each group** — Compact binary encoding over structure
///    (see [`encode_subgraph`](super::encoding::encode_subgraph)).
///
/// # Example: grouping walkthrough
///
/// Suppose a computation has one intermediate node (`Le`) feeding two
/// published outputs (`✓`):
///
/// ```text
///   [Le]  ← takes ext_a
///    │
///    ├──► [Select] ✓   ← also takes ext_b   → out_a
///    │
///    └──► [Add] ✓      ← also takes ext_c   → out_b
/// ```
///
/// **Step 1 — trace backward from each `✓` node** (its "cone"):
///
/// ```text
///   From Select✓:  trace back → finds Le      cone = {Le, Select}
///   From Add✓:     trace back → finds Le      cone = {Le, Add}
///                                      ^^
///                                Le is in both cones
/// ```
///
/// **Step 2 — merge overlapping cones** (union-find):
///
/// Both cones contain `Le`, so they merge into one group:
/// `{Le, Select, Add}` → **one** `pattern_id`.
///
/// # Example: ERC20 `transferFrom`
///
/// A real `transferFrom` compiles to 9 FHE operations with three published
/// outputs. The intermediate nodes `And` and `Select` are shared across
/// all three output cones:
///
/// ```text
///   [0]Le ──┐
///   [1]Le ──┼──► [2]And ──┬──► [4]Select ✓   newAllowance
///   [3]Sub ─┘             │
///                         └──► [6]Select ──┬──► [7]Add ✓  newBalanceTo
///                                          │
///   [5]TrivialEncrypt(0)                   └──► [8]Sub ✓  newBalanceFrom
///         ↑ source node (excluded)
/// ```
///
/// Tracing backward from the three `✓` nodes, all cones overlap at `And`,
/// `Le`, etc. — so union-find merges everything into **one group** of 8
/// computation nodes (the `TrivialEncrypt` is a source node and excluded).
///
/// Three `transferFrom` calls in one transaction — even with chained
/// dependencies — all get the **same** `pattern_id` because their
/// *structure* is identical.
pub fn compute_logical_pattern_ids(
    graph: &Dag<(bool, usize), OpEdge>,
    operations: &[DFGOp],
    produced_handles: &HashMap<Vec<u8>, usize>,
) -> HashMap<usize, Vec<u8>> {
    let n = operations.len();
    if n == 0 {
        return HashMap::new();
    }

    // Classify nodes in a single pass
    let mut source_nodes: HashSet<usize> = HashSet::new();
    let mut allowed_computation: HashSet<usize> = HashSet::new();
    for (i, op) in operations.iter().enumerate() {
        if is_source_op(op) {
            source_nodes.insert(i);
        } else if op.is_allowed {
            allowed_computation.insert(i);
        }
    }

    if allowed_computation.is_empty() {
        // No allowed nodes → no meaningful pattern_ids
        return HashMap::new();
    }

    // Build successor map: op_index → [consumer indices] (for cone propagation
    // and source-node inheritance).
    let mut successors: HashMap<usize, Vec<usize>> = HashMap::new();
    for (op_idx, op) in operations.iter().enumerate() {
        for input in &op.inputs {
            if let DFGTaskInput::Dependence(h) = input {
                if let Some(&prod_idx) = produced_handles.get(h) {
                    successors.entry(prod_idx).or_default().push(op_idx);
                }
            }
        }
    }

    // Single reverse-topo pass to assign groups via union-find.
    // Each non-source, non-allowed node inherits the group of its successors.
    // Allowed nodes start their own group. Shared intermediates cause merges.
    let topo = match toposort(graph, None) {
        Ok(t) => t,
        Err(_) => return HashMap::new(),
    };

    let mut uf = UnionFind::new(n);
    let mut node_in_any_cone: HashSet<usize> = HashSet::new();

    // Walk in reverse topo order (sinks first → sources last).
    for &nidx in topo.iter().rev() {
        let op_idx = match graph.node_weight(nidx) {
            Some(w) => w.1,
            None => continue,
        };

        if source_nodes.contains(&op_idx) {
            continue;
        }

        if allowed_computation.contains(&op_idx) {
            // Allowed node: starts its own group (it's already its own UF set).
            node_in_any_cone.insert(op_idx);
            continue;
        }

        // Non-allowed computation node: union with each non-source,
        // non-boundary successor that is already in a cone.
        if let Some(succs) = successors.get(&op_idx) {
            for &succ in succs {
                if node_in_any_cone.contains(&succ) {
                    uf.union(op_idx, succ);
                    node_in_any_cone.insert(op_idx);
                }
            }
        }
    }

    // Collect groups by union-find representative
    let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
    for &node in &node_in_any_cone {
        let rep = uf.find(node);
        groups.entry(rep).or_default().push(node);
    }

    // Sort each group for deterministic encoding
    for group in groups.values_mut() {
        group.sort_unstable();
    }

    // Encode each group and assign pattern_ids.
    //
    // Try the compact inline encoding first. If it succeeds and is above the
    // threshold, we hash and log the full encoding once. If inline encoding
    // fails (group too large), hash the wide encoding without logging the full
    // payload.
    let threshold = *PATTERN_HASH_THRESHOLD;
    let mut result: HashMap<usize, Vec<u8>> = HashMap::new();
    for group in groups.values() {
        let pattern_id = match encode_subgraph(operations, group, produced_handles, &topo, graph) {
            Some(encoding) => Some(finalize_pattern(encoding, threshold)),
            None => encode_subgraph_hashable(operations, group, produced_handles, &topo, graph)
                .map(|wide| build_hash_pattern(&wide, group.len(), false)),
        };
        if let Some(pid) = pattern_id {
            for &node in group {
                result.insert(node, pid.clone());
            }
        }
    }

    // Source nodes that feed into a single group inherit that group's pattern_id.
    for &src_idx in &source_nodes {
        let mut unique_pattern: Option<&Vec<u8>> = None;
        let mut ambiguous = false;
        if let Some(succs) = successors.get(&src_idx) {
            for &succ in succs {
                if let Some(pid) = result.get(&succ) {
                    match unique_pattern {
                        None => unique_pattern = Some(pid),
                        Some(prev) if prev == pid => {}
                        Some(_) => {
                            ambiguous = true;
                            break;
                        }
                    }
                }
            }
        }
        if !ambiguous {
            if let Some(pid) = unique_pattern {
                result.insert(src_idx, pid.clone());
            }
        }
    }

    result
}

/// Encode the entire pre-partition transaction graph.
///
/// Like `compute_logical_pattern_ids` this operates on the pre-partition graph,
/// but instead of grouping by cones it encodes ALL computation nodes as one group.
/// Source nodes (TrivialEncrypt, FheRand, FheRandBounded) are excluded from the
/// group but their consumers see them as external inputs (byte 0x00).
pub fn compute_transaction_pattern_id(
    graph: &Dag<(bool, usize), OpEdge>,
    operations: &[DFGOp],
    produced_handles: &HashMap<Vec<u8>, usize>,
) -> Vec<u8> {
    let n = operations.len();
    if n == 0 {
        return Vec::new();
    }

    let all_computation: Vec<usize> = (0..n).filter(|&i| !is_source_op(&operations[i])).collect();

    if all_computation.is_empty() {
        return Vec::new();
    }

    let topo = match toposort(graph, None) {
        Ok(t) => t,
        Err(_) => return Vec::new(),
    };

    let threshold = *PATTERN_HASH_THRESHOLD;
    match encode_subgraph(operations, &all_computation, produced_handles, &topo, graph) {
        Some(encoding) => finalize_pattern(encoding, threshold),
        None => {
            encode_subgraph_hashable(operations, &all_computation, produced_handles, &topo, graph)
                .map(|wide| build_hash_pattern(&wide, all_computation.len(), false))
                .unwrap_or_default()
        }
    }
}
