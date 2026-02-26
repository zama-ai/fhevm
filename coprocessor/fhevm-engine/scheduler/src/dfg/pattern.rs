/// # DFG Pattern IDs
///
/// Deterministic structural fingerprints of a Data Flow Graph, used as
/// low-cardinality span attributes for latency segmentation in OpenTelemetry
/// metrics.
///
/// Two transactions performing the same FHE computation (e.g., an ERC20
/// `transferFrom`) produce the **same** `operation_pattern_id` regardless of
/// which ciphertext handles, addresses, or transaction IDs are involved.
/// Additionally, a `transaction_pattern_id` fingerprints the **entire**
/// transaction graph, so dashboards can segment by both "what kind of FHE
/// operation is this?" (operation-level) and "what kind of transaction is
/// this?" (tx-level, e.g., ERC20 transfer vs approval).
///
/// # Why not hash each component individually?
///
/// The scheduler partitions every DFG into single-operation [`ComponentNode`]s
/// for parallel execution. Hashing individual components would label every
/// `FheAdd` the same — useless for distinguishing contract-level operations
/// like "ERC20 transfer" vs "mint".
///
/// Instead, we group FHE ops into **logical operations** on the pre-partition
/// graph and fingerprint each group.
///
/// # Example
///
/// A diamond-shaped computation with one published output (`✓` = `is_allowed`):
///
/// ```text
///      ext_a  ext_b              ← external inputs (from DB)
///        │      │
///        ▼      ▼
///      ┌──────────┐
///      │  FheGe   │              node 0
///      └────┬─────┘
///           │
///      ┌────┴─────┐
///      ▼          ▼
///   ┌────────┐ ┌────────┐
///   │ FheAdd │ │ FheSub │        nodes 1, 2
///   └───┬────┘ └───┬────┘
///       └────┬─────┘
///            ▼
///     ┌──────────────┐
///     │FheIfThenElse │ ✓         node 3 → published result
///     └──────────────┘
/// ```
///
/// All four nodes form **one logical group** → one `pattern_id`.
///
/// A second transaction with the same shape but different ciphertext handles
/// produces the **same** `pattern_id`:
///
/// ```text
///   Tx A: handles 0x1A.. → diamond → pattern_id = a1b2c3...
///   Tx B: handles 0x2B.. → diamond → pattern_id = a1b2c3...  ← identical
///   Tx C: handles 0x1A.. → chain   → pattern_id = f4e5d6...  ← different shape
/// ```
///
/// # Logical-operation grouping
///
/// A transaction may contain multiple FHE operations. The grouping algorithm
/// figures out which ops belong to the same "logical operation" (e.g., one
/// ERC20 transfer) so they all get the same `pattern_id`.
///
/// ## Walkthrough: how grouping works
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
/// If `Select✓` and `Add✓` had *no* shared nodes, they would stay
/// separate groups with *different* `pattern_id`s.
///
/// ## Applied to ERC20 `transferFrom`
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
/// *structure* is identical. External inputs (DB handles, outputs from
/// other groups, source nodes) are all treated as opaque during encoding.
///
/// ## Algorithm summary
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
///    (see `encode_subgraph`).
///
/// # Encoding
///
/// Each group is encoded as a compact self-describing binary structure:
/// opcodes, `is_allowed` flags, input counts, and edges (internal refs vs
/// external) — all in topological order. The encoding is decodable without
/// any external lookup, unlike the previous Keccak-256 hash approach.
///
/// Only graph **shape** contributes to the fingerprint. Runtime identifiers
/// (output handles, transaction IDs, ciphertext data) are excluded.
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::num::NonZeroUsize;
use std::sync::Mutex;

use daggy::petgraph::algo::toposort;
use daggy::{Dag, NodeIndex};
use data_encoding::BASE64URL_NOPAD;
use lru::LruCache;
use sha3::{Digest, Keccak256};

use super::{DFGOp, OpEdge};
use crate::dfg::types::DFGTaskInput;
use fhevm_engine_common::common::FheOperation;
use fhevm_engine_common::types::SupportedFheOperations;

/// Encoding version byte (self-describing, decodable).
const ENCODING_VERSION: u8 = 0x01;

/// Hash version byte (Keccak-256 truncated, not decodable).
const HASH_VERSION: u8 = 0x02;

/// Number of bytes to keep from the Keccak-256 digest.
const HASH_DIGEST_LEN: usize = 20;

/// Default node-count threshold: groups with more nodes than this get hashed.
const DEFAULT_PATTERN_HASH_THRESHOLD: usize = 25;

/// LRU dedup cache so we log the full encoding once per unique hash.
/// Bounded at [`HASH_LOG_CACHE_SIZE`] entries; when full, the least-recently
/// seen pattern is evicted (and will be re-logged if it reappears).
static HASH_LOG_SEEN: Mutex<Option<LruCache<Vec<u8>, ()>>> = Mutex::new(None);

/// Maximum entries in the hash-log dedup LRU cache.
const HASH_LOG_CACHE_SIZE: usize = 10_000;

// ---------------------------------------------------------------------------
// Pattern encoding / decoding types
// ---------------------------------------------------------------------------

/// Decoded description of a pattern encoding.
pub struct PatternDescription {
    pub nodes: Vec<PatternNode>,
}

pub struct PatternNode {
    /// Raw opcode value (SupportedFheOperations repr).
    pub opcode: i32,
    /// Human-readable opcode name via `FheOperation::as_str_name()`.
    pub opcode_name: &'static str,
    pub is_allowed: bool,
    pub inputs: Vec<PatternInput>,
}

pub enum PatternInput {
    /// Internal reference to another node at the given topo position.
    Internal(u8),
    /// External input (DB handle, other group, scalar).
    External,
}

impl fmt::Display for PatternDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, node) in self.nodes.iter().enumerate() {
            if i > 0 {
                write!(f, ",")?;
            }
            write!(f, "{}", node.opcode_name)?;
            if node.is_allowed {
                write!(f, "[a]")?;
            }
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Encoding / decoding
// ---------------------------------------------------------------------------

/// Encode the `pattern_id` bytes as a base64url (no-pad) string for use in
/// OTel span attributes.
pub fn pattern_to_base64url(bytes: &[u8]) -> String {
    BASE64URL_NOPAD.encode(bytes)
}

/// Decode a binary pattern encoding into a [`PatternDescription`].
///
/// Returns `None` if the encoding is malformed (wrong version, truncated, etc.)
/// or if the pattern is a hashed v2 form (hashes are not decodable).
pub fn decode_pattern(bytes: &[u8]) -> Option<PatternDescription> {
    if bytes.len() < 2 {
        return None;
    }
    if bytes[0] == HASH_VERSION {
        return None;
    }
    if bytes[0] != ENCODING_VERSION {
        return None;
    }
    let node_count = bytes[1] as usize;
    let mut pos = 2;
    let mut nodes = Vec::with_capacity(node_count);

    for _ in 0..node_count {
        if pos >= bytes.len() {
            return None;
        }
        let opcode_raw = bytes[pos] as i32;
        pos += 1;

        if pos >= bytes.len() {
            return None;
        }
        let flags_byte = bytes[pos];
        pos += 1;
        let is_allowed = (flags_byte & 0x80) != 0;
        let input_count = (flags_byte & 0x07) as usize;

        let opcode_name = FheOperation::try_from(opcode_raw)
            .ok()
            .map(|op| op.as_str_name())
            .unwrap_or("UNKNOWN");

        let mut inputs = Vec::with_capacity(input_count);
        for _ in 0..input_count {
            if pos >= bytes.len() {
                return None;
            }
            let input_byte = bytes[pos];
            pos += 1;
            if (input_byte & 0x80) != 0 {
                inputs.push(PatternInput::Internal(input_byte & 0x7F));
            } else {
                inputs.push(PatternInput::External);
            }
        }

        nodes.push(PatternNode {
            opcode: opcode_raw,
            opcode_name,
            is_allowed,
            inputs,
        });
    }

    // All bytes should be consumed
    if pos != bytes.len() {
        return None;
    }

    Some(PatternDescription { nodes })
}

// ---------------------------------------------------------------------------
// Two-tier finalization (encoding vs hash)
// ---------------------------------------------------------------------------

/// Returns `true` if the pattern bytes represent a hashed (v2) pattern.
pub fn is_hashed_pattern(bytes: &[u8]) -> bool {
    !bytes.is_empty() && bytes[0] == HASH_VERSION
}

/// Read the hash threshold from `FHEVM_PATTERN_HASH_THRESHOLD` env var,
/// falling back to [`DEFAULT_PATTERN_HASH_THRESHOLD`].
fn pattern_hash_threshold() -> usize {
    std::env::var("FHEVM_PATTERN_HASH_THRESHOLD")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_PATTERN_HASH_THRESHOLD)
}

/// If the v1 encoding has `node_count ≤ threshold`, return it as-is.
/// Otherwise hash it into a compact 23-byte v2 form and log the full
/// encoding once (per unique hash) for operator linkability.
fn finalize_pattern(encoding: Vec<u8>, threshold: usize) -> Vec<u8> {
    debug_assert!(encoding.len() >= 2 && encoding[0] == ENCODING_VERSION);
    let node_count = encoding[1] as usize;

    if node_count <= threshold {
        return encoding;
    }

    build_hash_pattern(&encoding, node_count)
}

/// Build a 23-byte v2 hashed pattern from arbitrary encoding bytes.
///
/// ## Hashed pattern binary layout
///
/// ```text
/// Byte 0:     0x02 (HASH_VERSION)
/// Bytes 1-2:  node_count as u16 big-endian
/// Bytes 3-22: first 20 bytes of Keccak-256(encoding)
/// Total: 23 bytes
/// ```
fn build_hash_pattern(encoding: &[u8], node_count: usize) -> Vec<u8> {
    let digest = Keccak256::digest(encoding);

    let mut buf = Vec::with_capacity(1 + 2 + HASH_DIGEST_LEN);
    buf.push(HASH_VERSION);
    buf.extend_from_slice(&(node_count as u16).to_be_bytes());
    buf.extend_from_slice(&digest[..HASH_DIGEST_LEN]);

    // Log the full encoding once per unique hash for linkability.
    // The LRU cache evicts the least-recently seen pattern when full;
    // if an evicted pattern reappears it simply gets re-logged.
    let mut seen = HASH_LOG_SEEN.lock().unwrap();
    let cache =
        seen.get_or_insert_with(|| LruCache::new(NonZeroUsize::new(HASH_LOG_CACHE_SIZE).unwrap()));
    if cache.put(buf.clone(), ()).is_none() {
        let b64_hash = BASE64URL_NOPAD.encode(&buf);
        let b64_full = BASE64URL_NOPAD.encode(encoding);
        tracing::info!(
            pattern_hash = %b64_hash,
            pattern_full = %b64_full,
            node_count,
            "new hashed pattern: full encoding logged for linkability"
        );
    }

    buf
}

// ---------------------------------------------------------------------------
// Logical-operation pattern IDs
// ---------------------------------------------------------------------------

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

/// Maximum number of nodes that a v1 encoding can represent (node_count is u8).
const V1_MAX_NODES: usize = 255;

/// Maximum number of inputs per node that a v1 encoding can represent
/// (3-bit field in the flags byte).
const V1_MAX_INPUTS_PER_NODE: usize = 7;

/// Maximum topo position that fits in an internal-ref byte (7-bit field).
const V1_MAX_INTERNAL_REF: usize = 127;

/// Encode a subgraph (a set of node indices within the pre-partition graph)
/// as a compact self-describing binary structure.
///
/// Nodes in `group` are the computation nodes belonging to one logical operation.
/// `group` must be sorted. All inputs that come from outside the group (source
/// nodes, allowed nodes from other groups, DB handles) are treated as external
/// (byte 0x00).
///
/// Returns `None` if the group exceeds the v1 encoding limits (\>255 nodes,
/// \>7 inputs per node, or \>127 internal-ref position). Callers should treat
/// `None` the same as an empty pattern (no pattern attribution for this group).
///
/// ## Binary layout (version 1)
///
/// ```text
/// Byte 0:  0x01 (version)
/// Byte 1:  node_count (u8, max 255)
///
/// Per node in canonical topological order:
///   Byte N+0:  opcode (u8, raw SupportedFheOperations repr value: 0-32)
///   Byte N+1:  (is_allowed << 7) | (input_count & 0x07)
///   Per input (input_count times):
///     If internal dep (producer in same group):
///       Byte:  0x80 | (source_node_topo_position & 0x7F)
///     If external (DB handle, other group, scalar):
///       Byte:  0x00
/// ```
fn encode_subgraph(
    operations: &[DFGOp],
    group: &[usize],
    produced_handles: &HashMap<Vec<u8>, usize>,
    parent_topo: &[NodeIndex],
    graph: &Dag<(bool, usize), OpEdge>,
) -> Option<Vec<u8>> {
    if group.is_empty() {
        return None;
    }

    // Filter the parent graph's topo order to group members. The parent topo
    // is a valid topological ordering for any subgraph — filtering preserves
    // the relative order, which is sufficient for deterministic encoding.
    //
    // The ordering is deterministic but is NOT a canonical form for graph
    // isomorphism. In practice, identical structures encode identically because
    // handles are sorted before graph construction (dfg.rs:166), giving all
    // isomorphic instances the same node insertion order.
    let local_topo: Vec<usize> = parent_topo
        .iter()
        .filter_map(|nidx| {
            let op_idx = graph.node_weight(*nidx)?.1;
            if group.binary_search(&op_idx).is_ok() {
                Some(op_idx)
            } else {
                None
            }
        })
        .collect();

    let node_count = local_topo.len();
    if node_count > V1_MAX_NODES {
        return None;
    }

    // Map global op_idx → local topo position for internal refs.
    let mut topo_pos: HashMap<usize, u8> = HashMap::with_capacity(node_count);
    for (pos, &op_idx) in local_topo.iter().enumerate() {
        topo_pos.insert(op_idx, pos as u8);
    }

    // Pre-allocate: version(1) + count(1) + per-node ~4 bytes average
    let mut buf: Vec<u8> = Vec::with_capacity(2 + node_count * 4);

    // Header
    buf.push(ENCODING_VERSION);
    buf.push(node_count as u8);

    // Nodes
    for &global_idx in &local_topo {
        let op = &operations[global_idx];

        if op.inputs.len() > V1_MAX_INPUTS_PER_NODE {
            return None;
        }

        // Opcode byte
        buf.push(op.fhe_op as u8);

        // Flags + input_count byte
        let input_count = op.inputs.len() as u8;
        let flags = if op.is_allowed { 0x80 } else { 0x00 } | (input_count & 0x07);
        buf.push(flags);

        // Per-input bytes
        for input in &op.inputs {
            let byte = match input {
                DFGTaskInput::Dependence(h) => {
                    // Internal dependency only if the producer is in this group
                    if let Some(&producer_idx) = produced_handles.get(h) {
                        if let Some(&src_pos) = topo_pos.get(&producer_idx) {
                            if src_pos as usize > V1_MAX_INTERNAL_REF {
                                return None;
                            }
                            0x80 | src_pos
                        } else {
                            0x00
                        }
                    } else {
                        0x00
                    }
                }
                DFGTaskInput::Value(_) | DFGTaskInput::Compressed(_) => 0x00,
            };
            buf.push(byte);
        }
    }

    Some(buf)
}

/// Wide-format encoding of a subgraph, used only as Keccak-256 input.
///
/// Unlike [`encode_subgraph`] this uses u16 for node counts and internal
/// reference positions, so it handles arbitrarily large groups (up to 65535
/// nodes). The output is NOT decodable as a v1 pattern — it is only fed
/// into the hash to produce a v2 pattern.
///
/// Returns `None` only for truly empty groups.
fn encode_subgraph_hashable(
    operations: &[DFGOp],
    group: &[usize],
    produced_handles: &HashMap<Vec<u8>, usize>,
    parent_topo: &[NodeIndex],
    graph: &Dag<(bool, usize), OpEdge>,
) -> Option<Vec<u8>> {
    if group.is_empty() {
        return None;
    }

    let local_topo: Vec<usize> = parent_topo
        .iter()
        .filter_map(|nidx| {
            let op_idx = graph.node_weight(*nidx)?.1;
            if group.binary_search(&op_idx).is_ok() {
                Some(op_idx)
            } else {
                None
            }
        })
        .collect();

    let node_count = local_topo.len();

    // Map global op_idx → local topo position (u16).
    let mut topo_pos: HashMap<usize, u16> = HashMap::with_capacity(node_count);
    for (pos, &op_idx) in local_topo.iter().enumerate() {
        topo_pos.insert(op_idx, pos as u16);
    }

    // Use a distinct marker byte (0xFE) so the hash never collides with
    // a v1 encoding of a smaller group that happens to have the same bytes.
    let mut buf: Vec<u8> = Vec::with_capacity(3 + node_count * 8);
    buf.push(0xFE); // wide format marker
    buf.extend_from_slice(&(node_count as u16).to_be_bytes());

    for &global_idx in &local_topo {
        let op = &operations[global_idx];

        buf.push(op.fhe_op as u8);
        buf.push(if op.is_allowed { 0x80 } else { 0x00 });
        buf.extend_from_slice(&(op.inputs.len() as u16).to_be_bytes());

        for input in &op.inputs {
            match input {
                DFGTaskInput::Dependence(h) => {
                    if let Some(&producer_idx) = produced_handles.get(h) {
                        if let Some(&src_pos) = topo_pos.get(&producer_idx) {
                            buf.push(0x01); // internal
                            buf.extend_from_slice(&src_pos.to_be_bytes());
                            continue;
                        }
                    }
                    buf.push(0x00); // external
                }
                DFGTaskInput::Value(_) | DFGTaskInput::Compressed(_) => {
                    buf.push(0x00); // external
                }
            }
        }
    }

    Some(buf)
}

/// Compute logical-operation pattern IDs on the **pre-partition** graph.
///
/// Returns a map from op_index → pattern_id (compact binary encoding).
/// Every op in the same logical operation gets the same pattern_id.
///
/// Algorithm:
/// 1. Classify source vs computation nodes.
/// 2. BFS backward from each `is_allowed` computation node, stopping at other
///    `is_allowed` nodes (they're boundaries of a different logical operation).
/// 3. Union-find to merge overlapping cones.
/// 4. For each group, encode using the compact binary structure.
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
    // Try the compact v1 encoding first. If it succeeds and the group is
    // small enough, the encoding is used as-is; otherwise it's hashed.
    // If v1 fails (group too large for u8 node count or u7 internal refs),
    // fall back to the wide encoding which is only used as hash input.
    let threshold = pattern_hash_threshold();
    let mut result: HashMap<usize, Vec<u8>> = HashMap::new();
    for group in groups.values() {
        let pattern_id = match encode_subgraph(operations, group, produced_handles, &topo, graph) {
            Some(encoding) => Some(finalize_pattern(encoding, threshold)),
            None => encode_subgraph_hashable(operations, group, produced_handles, &topo, graph)
                .map(|wide| build_hash_pattern(&wide, group.len())),
        };
        if let Some(pid) = pattern_id {
            for &node in group {
                result.insert(node, pid.clone());
            }
        }
    }

    // Source nodes that feed into a single group inherit that group's pattern_id.
    for &src_idx in &source_nodes {
        let mut target_patterns: HashSet<Vec<u8>> = HashSet::new();
        if let Some(succs) = successors.get(&src_idx) {
            for &succ in succs {
                if let Some(pattern_id) = result.get(&succ) {
                    target_patterns.insert(pattern_id.clone());
                }
            }
        }
        if target_patterns.len() == 1 {
            result.insert(src_idx, target_patterns.into_iter().next().unwrap());
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

    let threshold = pattern_hash_threshold();
    match encode_subgraph(operations, &all_computation, produced_handles, &topo, graph) {
        Some(encoding) => finalize_pattern(encoding, threshold),
        None => {
            // V1 encoding failed (group too large for u8/u7 limits).
            // Fall back to wide encoding → hash.
            encode_subgraph_hashable(operations, &all_computation, produced_handles, &topo, graph)
                .map(|wide| build_hash_pattern(&wide, all_computation.len()))
                .unwrap_or_default()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dfg::{build_component_nodes, types::DFGTaskInput, DFGOp, OpEdge};
    use daggy::{petgraph::graph::node_index, Dag};
    use fhevm_engine_common::types::SupportedFheOperations;
    use std::collections::HashMap;

    use data_encoding::BASE64URL_NOPAD;

    use super::{
        decode_pattern, is_hashed_pattern, pattern_to_base64url, ENCODING_VERSION, HASH_VERSION,
    };

    /// Build a pre-partition graph + produced_handles from `Vec<DFGOp>` and call
    /// `compute_logical_pattern_ids` directly. Avoids going through `build_component_nodes`
    /// for finer-grained assertions on the logical grouping.
    fn compute_logical_ids(mut ops: Vec<DFGOp>) -> HashMap<usize, Vec<u8>> {
        ops.sort_by_key(|o| o.output_handle.clone());
        let mut graph: Dag<(bool, usize), OpEdge> = Dag::default();
        let mut produced_handles: HashMap<Vec<u8>, usize> = HashMap::new();
        for (index, op) in ops.iter().enumerate() {
            produced_handles.insert(op.output_handle.clone(), index);
        }
        let mut dependence_pairs = vec![];
        for (index, op) in ops.iter().enumerate() {
            for (pos, i) in op.inputs.iter().enumerate() {
                if let DFGTaskInput::Dependence(dh) = i {
                    if let Some(&producer) = produced_handles.get(dh) {
                        dependence_pairs.push((producer, index, pos));
                    }
                }
            }
            graph.add_node((op.is_allowed, index));
        }
        for (source, destination, pos) in dependence_pairs {
            graph
                .add_edge(node_index(source), node_index(destination), pos as u8)
                .expect("cycle in test graph");
        }
        super::compute_logical_pattern_ids(&graph, &ops, &produced_handles)
    }

    /// Helper: build a unique 32-byte handle from an id byte and an index.
    fn handle(prefix: u8, idx: u8) -> Vec<u8> {
        let mut h = vec![prefix; 32];
        h[0] = idx;
        h
    }

    /// Collect all per-op pattern_ids from a component's inner DFGraph.
    fn collect_op_pattern_ids(component: &super::super::ComponentNode) -> Vec<Vec<u8>> {
        use daggy::petgraph::visit::IntoNodeReferences;
        component
            .graph
            .graph
            .node_references()
            .map(|(_, node)| node.operation_pattern_id().to_vec())
            .collect()
    }

    /// Assert that a pattern_id is a valid pattern — either a v1 encoding
    /// (decodable) or a v2 hash (23 bytes, starts with HASH_VERSION).
    fn assert_valid_pattern(bytes: &[u8]) {
        assert!(!bytes.is_empty(), "pattern_id should not be empty");
        match bytes[0] {
            ENCODING_VERSION => {
                let desc = decode_pattern(bytes).expect("v1 pattern_id should be decodable");
                assert!(
                    !desc.nodes.is_empty(),
                    "decoded pattern should have at least one node"
                );
            }
            HASH_VERSION => {
                assert_eq!(bytes.len(), 23, "hashed pattern should be exactly 23 bytes");
            }
            other => panic!("unexpected pattern version byte: 0x{other:02x}"),
        }
    }

    /// Build DFGOps matching one ERC20 transferFrom call.
    ///
    /// The 9 compute ops (excluding verify/fromExternal which doesn't enter computations):
    ///   [0] Le(amount, currentAllowance)               → allowedTransfer
    ///   [1] Le(amount, balances[from])                  → canTransfer
    ///   [2] And(canTransfer, allowedTransfer)           → isTransferable
    ///   [3] Sub(currentAllowance, amount)               → allowanceDiff
    ///   [4] Select(isTransferable, allowanceDiff, currentAllowance) → newAllowance
    ///   [5] TrivialEncrypt(0)                           → zero
    ///   [6] Select(isTransferable, amount, zero)        → transferValue
    ///   [7] Add(balances[to], transferValue)            → newBalanceTo
    ///   [8] Sub(balances[from], transferValue)          → newBalanceFrom
    ///
    /// `prefix` differentiates handles across multiple calls in the same tx.
    fn build_transfer_from_ops(prefix: u8) -> Vec<DFGOp> {
        // External handles (not produced by this set of ops)
        let ext_amount = handle(0xE0, prefix);
        let ext_allowance = handle(0xE1, prefix);
        let ext_bal_from = handle(0xE2, prefix);
        let ext_bal_to = handle(0xE3, prefix);

        // Output handles for each op
        let h0 = handle(prefix, 0); // allowedTransfer
        let h1 = handle(prefix, 1); // canTransfer
        let h2 = handle(prefix, 2); // isTransferable
        let h3 = handle(prefix, 3); // allowanceDiff
        let h4 = handle(prefix, 4); // newAllowance
        let h5 = handle(prefix, 5); // zero
        let h6 = handle(prefix, 6); // transferValue
        let h7 = handle(prefix, 7); // newBalanceTo
        let h8 = handle(prefix, 8); // newBalanceFrom

        vec![
            // [0] Le(amount, currentAllowance) → allowedTransfer
            DFGOp {
                output_handle: h0.clone(),
                fhe_op: SupportedFheOperations::FheLe,
                inputs: vec![
                    DFGTaskInput::Dependence(ext_amount.clone()),
                    DFGTaskInput::Dependence(ext_allowance.clone()),
                ],
                is_allowed: false,
                ..Default::default()
            },
            // [1] Le(amount, balances[from]) → canTransfer
            DFGOp {
                output_handle: h1.clone(),
                fhe_op: SupportedFheOperations::FheLe,
                inputs: vec![
                    DFGTaskInput::Dependence(ext_amount.clone()),
                    DFGTaskInput::Dependence(ext_bal_from.clone()),
                ],
                is_allowed: false,
                ..Default::default()
            },
            // [2] And(canTransfer, allowedTransfer) → isTransferable
            DFGOp {
                output_handle: h2.clone(),
                fhe_op: SupportedFheOperations::FheBitAnd,
                inputs: vec![
                    DFGTaskInput::Dependence(h1.clone()),
                    DFGTaskInput::Dependence(h0.clone()),
                ],
                is_allowed: false,
                ..Default::default()
            },
            // [3] Sub(currentAllowance, amount) → allowanceDiff
            DFGOp {
                output_handle: h3.clone(),
                fhe_op: SupportedFheOperations::FheSub,
                inputs: vec![
                    DFGTaskInput::Dependence(ext_allowance.clone()),
                    DFGTaskInput::Dependence(ext_amount.clone()),
                ],
                is_allowed: false,
                ..Default::default()
            },
            // [4] Select(isTransferable, allowanceDiff, currentAllowance) → newAllowance
            DFGOp {
                output_handle: h4.clone(),
                fhe_op: SupportedFheOperations::FheIfThenElse,
                inputs: vec![
                    DFGTaskInput::Dependence(h2.clone()),
                    DFGTaskInput::Dependence(h3.clone()),
                    DFGTaskInput::Dependence(ext_allowance.clone()),
                ],
                is_allowed: true,
                ..Default::default()
            },
            // [5] TrivialEncrypt(0) → zero
            DFGOp {
                output_handle: h5.clone(),
                fhe_op: SupportedFheOperations::FheTrivialEncrypt,
                inputs: vec![DFGTaskInput::Value(
                    fhevm_engine_common::types::SupportedFheCiphertexts::Scalar(vec![0u8]),
                )],
                is_allowed: false,
                ..Default::default()
            },
            // [6] Select(isTransferable, amount, zero) → transferValue
            DFGOp {
                output_handle: h6.clone(),
                fhe_op: SupportedFheOperations::FheIfThenElse,
                inputs: vec![
                    DFGTaskInput::Dependence(h2.clone()),
                    DFGTaskInput::Dependence(ext_amount.clone()),
                    DFGTaskInput::Dependence(h5.clone()),
                ],
                is_allowed: false,
                ..Default::default()
            },
            // [7] Add(balances[to], transferValue) → newBalanceTo
            DFGOp {
                output_handle: h7.clone(),
                fhe_op: SupportedFheOperations::FheAdd,
                inputs: vec![
                    DFGTaskInput::Dependence(ext_bal_to.clone()),
                    DFGTaskInput::Dependence(h6.clone()),
                ],
                is_allowed: true,
                ..Default::default()
            },
            // [8] Sub(balances[from], transferValue) → newBalanceFrom
            DFGOp {
                output_handle: h8.clone(),
                fhe_op: SupportedFheOperations::FheSub,
                inputs: vec![
                    DFGTaskInput::Dependence(ext_bal_from.clone()),
                    DFGTaskInput::Dependence(h6.clone()),
                ],
                is_allowed: true,
                ..Default::default()
            },
        ]
    }

    #[test]
    fn transfer_from_partition_analysis() {
        let tx_id = vec![0xFFu8; 32];

        // === Single transferFrom ===
        let ops = build_transfer_from_ops(0x10);
        let (components, _unneeded) = build_component_nodes(ops, &tx_id).unwrap();
        // All ops of a single transfer should share the same logical pattern_id
        let mut all_op_patterns: Vec<Vec<u8>> =
            components.iter().flat_map(collect_op_pattern_ids).collect();
        all_op_patterns.sort();
        all_op_patterns.dedup();
        assert_eq!(
            all_op_patterns.len(),
            1,
            "all ops within one transfer should share the same pattern_id"
        );
        let single_pattern = &all_op_patterns[0];
        assert_valid_pattern(single_pattern);

        // === Three independent transferFrom calls in one tx ===
        let mut all_ops = build_transfer_from_ops(0x10);
        all_ops.extend(build_transfer_from_ops(0x20));
        all_ops.extend(build_transfer_from_ops(0x30));
        let (components_3, _) = build_component_nodes(all_ops, &tx_id).unwrap();

        // All ops across all three transfers should share ONE pattern_id
        let mut pattern_ids: Vec<Vec<u8>> = components_3
            .iter()
            .flat_map(collect_op_pattern_ids)
            .collect();
        pattern_ids.sort();
        pattern_ids.dedup();
        assert_eq!(
            pattern_ids.len(),
            1,
            "three independent identical transfers should produce 1 distinct pattern_id, got {}",
            pattern_ids.len()
        );
        assert_eq!(
            &pattern_ids[0], single_pattern,
            "the pattern_id should match the single-transfer pattern_id"
        );
    }

    /// Build DFGOps matching the "whitepaper" ERC20 transfer used by integration
    /// tests in scheduling_bench.rs (5 ops):
    ///   [0] FheGe(bals, trxa)                             → has_enough_funds
    ///   [1] FheAdd(bald, trxa)                            → new_to_amount_target
    ///   [2] FheIfThenElse(has_enough_funds, new_to_target, bald) → new_to_amount
    ///   [3] FheSub(bals, trxa)                            → new_from_amount_target
    ///   [4] FheIfThenElse(has_enough_funds, new_from_target, bals) → new_from_amount
    fn build_whitepaper_erc20_ops(prefix: u8) -> Vec<DFGOp> {
        let ext_bals = handle(0xA0, prefix); // balance source
        let ext_trxa = handle(0xA1, prefix); // transfer amount
        let ext_bald = handle(0xA2, prefix); // balance destination

        let h0 = handle(prefix, 0); // has_enough_funds
        let h1 = handle(prefix, 1); // new_to_amount_target
        let h2 = handle(prefix, 2); // new_to_amount
        let h3 = handle(prefix, 3); // new_from_amount_target
        let h4 = handle(prefix, 4); // new_from_amount

        vec![
            // [0] FheGe(bals, trxa) → has_enough_funds
            DFGOp {
                output_handle: h0.clone(),
                fhe_op: SupportedFheOperations::FheGe,
                inputs: vec![
                    DFGTaskInput::Dependence(ext_bals.clone()),
                    DFGTaskInput::Dependence(ext_trxa.clone()),
                ],
                is_allowed: true,
                ..Default::default()
            },
            // [1] FheAdd(bald, trxa) → new_to_amount_target
            DFGOp {
                output_handle: h1.clone(),
                fhe_op: SupportedFheOperations::FheAdd,
                inputs: vec![
                    DFGTaskInput::Dependence(ext_bald.clone()),
                    DFGTaskInput::Dependence(ext_trxa.clone()),
                ],
                is_allowed: true,
                ..Default::default()
            },
            // [2] FheIfThenElse(has_enough_funds, new_to_target, bald) → new_to_amount
            DFGOp {
                output_handle: h2.clone(),
                fhe_op: SupportedFheOperations::FheIfThenElse,
                inputs: vec![
                    DFGTaskInput::Dependence(h0.clone()),
                    DFGTaskInput::Dependence(h1.clone()),
                    DFGTaskInput::Dependence(ext_bald.clone()),
                ],
                is_allowed: true,
                ..Default::default()
            },
            // [3] FheSub(bals, trxa) → new_from_amount_target
            DFGOp {
                output_handle: h3.clone(),
                fhe_op: SupportedFheOperations::FheSub,
                inputs: vec![
                    DFGTaskInput::Dependence(ext_bals.clone()),
                    DFGTaskInput::Dependence(ext_trxa.clone()),
                ],
                is_allowed: true,
                ..Default::default()
            },
            // [4] FheIfThenElse(has_enough_funds, new_from_target, bals) → new_from_amount
            DFGOp {
                output_handle: h4.clone(),
                fhe_op: SupportedFheOperations::FheIfThenElse,
                inputs: vec![
                    DFGTaskInput::Dependence(h0.clone()),
                    DFGTaskInput::Dependence(h3.clone()),
                    DFGTaskInput::Dependence(ext_bals.clone()),
                ],
                is_allowed: true,
                ..Default::default()
            },
        ]
    }

    #[test]
    fn whitepaper_erc20_partition_analysis() {
        let tx_id = vec![0xFFu8; 32];

        // The whitepaper ERC20 test marks ALL ops is_allowed=true, which means
        // every op is its own cone boundary. This is a test artifact — in
        // production only final outputs are allowed. Each op becomes its own
        // single-node group, so we get per-op pattern_ids (degraded but correct).
        let ops = build_whitepaper_erc20_ops(0x10);
        let (components, _) = build_component_nodes(ops, &tx_id).unwrap();
        // Verify all ops have valid pattern_ids
        for c in &components {
            for pid in collect_op_pattern_ids(c) {
                assert_valid_pattern(&pid);
            }
        }

        // Seven samples: each transfer produces the same set of per-op patterns
        let mut all_ops = Vec::new();
        for i in 0..7u8 {
            all_ops.extend(build_whitepaper_erc20_ops(i + 1));
        }
        let (components_7, _) = build_component_nodes(all_ops, &tx_id).unwrap();
        let mut pattern_ids: Vec<Vec<u8>> = components_7
            .iter()
            .flat_map(collect_op_pattern_ids)
            .collect();
        pattern_ids.sort();
        pattern_ids.dedup();
        // With all-allowed, each op becomes its own single-node group. Ops with
        // the same opcode+structure share a pattern_id. The whitepaper ERC20 has
        // 4 distinct opcodes: FheGe, FheAdd, FheSub, FheIfThenElse.
        assert_eq!(
            pattern_ids.len(),
            4,
            "4 distinct opcodes (Ge, Add, Sub, IfThenElse) should produce 4 distinct pattern_ids"
        );
    }

    /// Build a transfer where one input comes from a prior allowed output
    /// (simulating a dependent transfer in a chain).
    fn build_transfer_from_ops_dependent(prefix: u8, prior_allowed_handle: Vec<u8>) -> Vec<DFGOp> {
        // Uses the prior_allowed_handle as ext_amount (instead of a DB handle)
        let ext_amount = prior_allowed_handle;
        let ext_allowance = handle(0xE1, prefix);
        let ext_bal_from = handle(0xE2, prefix);
        let ext_bal_to = handle(0xE3, prefix);

        let h0 = handle(prefix, 0);
        let h1 = handle(prefix, 1);
        let h2 = handle(prefix, 2);
        let h3 = handle(prefix, 3);
        let h4 = handle(prefix, 4);
        let h5 = handle(prefix, 5);
        let h6 = handle(prefix, 6);
        let h7 = handle(prefix, 7);
        let h8 = handle(prefix, 8);

        vec![
            DFGOp {
                output_handle: h0.clone(),
                fhe_op: SupportedFheOperations::FheLe,
                inputs: vec![
                    DFGTaskInput::Dependence(ext_amount.clone()),
                    DFGTaskInput::Dependence(ext_allowance.clone()),
                ],
                is_allowed: false,
                ..Default::default()
            },
            DFGOp {
                output_handle: h1.clone(),
                fhe_op: SupportedFheOperations::FheLe,
                inputs: vec![
                    DFGTaskInput::Dependence(ext_amount.clone()),
                    DFGTaskInput::Dependence(ext_bal_from.clone()),
                ],
                is_allowed: false,
                ..Default::default()
            },
            DFGOp {
                output_handle: h2.clone(),
                fhe_op: SupportedFheOperations::FheBitAnd,
                inputs: vec![
                    DFGTaskInput::Dependence(h1.clone()),
                    DFGTaskInput::Dependence(h0.clone()),
                ],
                is_allowed: false,
                ..Default::default()
            },
            DFGOp {
                output_handle: h3.clone(),
                fhe_op: SupportedFheOperations::FheSub,
                inputs: vec![
                    DFGTaskInput::Dependence(ext_allowance.clone()),
                    DFGTaskInput::Dependence(ext_amount.clone()),
                ],
                is_allowed: false,
                ..Default::default()
            },
            DFGOp {
                output_handle: h4.clone(),
                fhe_op: SupportedFheOperations::FheIfThenElse,
                inputs: vec![
                    DFGTaskInput::Dependence(h2.clone()),
                    DFGTaskInput::Dependence(h3.clone()),
                    DFGTaskInput::Dependence(ext_allowance.clone()),
                ],
                is_allowed: true,
                ..Default::default()
            },
            DFGOp {
                output_handle: h5.clone(),
                fhe_op: SupportedFheOperations::FheTrivialEncrypt,
                inputs: vec![DFGTaskInput::Value(
                    fhevm_engine_common::types::SupportedFheCiphertexts::Scalar(vec![0u8]),
                )],
                is_allowed: false,
                ..Default::default()
            },
            DFGOp {
                output_handle: h6.clone(),
                fhe_op: SupportedFheOperations::FheIfThenElse,
                inputs: vec![
                    DFGTaskInput::Dependence(h2.clone()),
                    DFGTaskInput::Dependence(ext_amount.clone()),
                    DFGTaskInput::Dependence(h5.clone()),
                ],
                is_allowed: false,
                ..Default::default()
            },
            DFGOp {
                output_handle: h7.clone(),
                fhe_op: SupportedFheOperations::FheAdd,
                inputs: vec![
                    DFGTaskInput::Dependence(ext_bal_to.clone()),
                    DFGTaskInput::Dependence(h6.clone()),
                ],
                is_allowed: true,
                ..Default::default()
            },
            DFGOp {
                output_handle: h8.clone(),
                fhe_op: SupportedFheOperations::FheSub,
                inputs: vec![
                    DFGTaskInput::Dependence(ext_bal_from.clone()),
                    DFGTaskInput::Dependence(h6.clone()),
                ],
                is_allowed: true,
                ..Default::default()
            },
        ]
    }

    #[test]
    fn three_mixed_transfers_same_pattern() {
        // Three transfers in one tx:
        //   1) From DB (standard)
        //   2) Dependent on transfer 1's newBalanceTo output
        //   3) From DB (standard, different prefix)
        // All three should produce the SAME logical pattern_id.
        let tx_id = vec![0xFFu8; 32];

        // Transfer 1: standard from DB
        let ops_1 = build_transfer_from_ops(0x10);
        // The allowed output handle from transfer 1 (newBalanceTo = h7)
        let prior_output = handle(0x10, 7);
        // Transfer 2: dependent — uses transfer 1's output as its amount input
        let ops_2 = build_transfer_from_ops_dependent(0x20, prior_output);
        // Transfer 3: standard from DB
        let ops_3 = build_transfer_from_ops(0x30);

        let mut all_ops = ops_1;
        all_ops.extend(ops_2);
        all_ops.extend(ops_3);
        let (components, _) = build_component_nodes(all_ops, &tx_id).unwrap();

        let mut pattern_ids: Vec<Vec<u8>> =
            components.iter().flat_map(collect_op_pattern_ids).collect();
        pattern_ids.sort();
        pattern_ids.dedup();
        assert_eq!(pattern_ids.len(), 1,
            "three mixed transfers (DB, dependent, DB) should all produce the same pattern_id, got {} distinct",
            pattern_ids.len());
    }

    #[test]
    fn transfer_vs_different_op_different_pattern() {
        // A transferFrom and a simple Add operation should have different pattern_ids.
        let tx_id = vec![0xFFu8; 32];

        let transfer_ops = build_transfer_from_ops(0x10);
        let (transfer_components, _) = build_component_nodes(transfer_ops, &tx_id).unwrap();
        let transfer_pattern = collect_op_pattern_ids(&transfer_components[0])[0].clone();

        // A simple FheAdd with two external inputs
        let simple_ops = vec![DFGOp {
            output_handle: handle(0x50, 0),
            fhe_op: SupportedFheOperations::FheAdd,
            inputs: vec![
                DFGTaskInput::Dependence(handle(0xD0, 0)),
                DFGTaskInput::Dependence(handle(0xD1, 0)),
            ],
            is_allowed: true,
            ..Default::default()
        }];
        let (simple_components, _) = build_component_nodes(simple_ops, &tx_id).unwrap();
        let simple_pattern = collect_op_pattern_ids(&simple_components[0])[0].clone();

        assert_ne!(
            transfer_pattern, simple_pattern,
            "a transferFrom and a simple FheAdd should produce different pattern_ids"
        );
    }

    // -----------------------------------------------------------------------
    // Tests using compute_logical_ids helper
    // -----------------------------------------------------------------------

    #[test]
    fn two_independent_groups_different_patterns() {
        // FheAdd✓ and FheSub✓ with disjoint external inputs → separate groups,
        // different pattern_ids.
        let ops = vec![
            DFGOp {
                output_handle: handle(0x10, 0),
                fhe_op: SupportedFheOperations::FheAdd,
                inputs: vec![
                    DFGTaskInput::Dependence(handle(0xE0, 0)),
                    DFGTaskInput::Dependence(handle(0xE1, 0)),
                ],
                is_allowed: true,
                ..Default::default()
            },
            DFGOp {
                output_handle: handle(0x20, 0),
                fhe_op: SupportedFheOperations::FheSub,
                inputs: vec![
                    DFGTaskInput::Dependence(handle(0xE2, 0)),
                    DFGTaskInput::Dependence(handle(0xE3, 0)),
                ],
                is_allowed: true,
                ..Default::default()
            },
        ];
        let ids = compute_logical_ids(ops);
        assert_eq!(ids.len(), 2, "both allowed nodes should have pattern_ids");
        let id_vals: Vec<&Vec<u8>> = ids.values().collect();
        assert_ne!(
            id_vals[0], id_vals[1],
            "different opcodes → different pattern_ids"
        );
    }

    #[test]
    fn chain_of_allowed_nodes_separate_groups() {
        // A✓(Add) → B✓(Sub) → C✓(Mul). Each allowed node is its own cone
        // boundary, so 3 groups with 3 distinct pattern_ids.
        let h_a = handle(0x10, 0);
        let h_b = handle(0x20, 0);
        let h_c = handle(0x30, 0);
        let ops = vec![
            DFGOp {
                output_handle: h_a.clone(),
                fhe_op: SupportedFheOperations::FheAdd,
                inputs: vec![
                    DFGTaskInput::Dependence(handle(0xE0, 0)),
                    DFGTaskInput::Dependence(handle(0xE1, 0)),
                ],
                is_allowed: true,
                ..Default::default()
            },
            DFGOp {
                output_handle: h_b.clone(),
                fhe_op: SupportedFheOperations::FheSub,
                inputs: vec![
                    DFGTaskInput::Dependence(h_a.clone()),
                    DFGTaskInput::Dependence(handle(0xE2, 0)),
                ],
                is_allowed: true,
                ..Default::default()
            },
            DFGOp {
                output_handle: h_c.clone(),
                fhe_op: SupportedFheOperations::FheMul,
                inputs: vec![
                    DFGTaskInput::Dependence(h_b.clone()),
                    DFGTaskInput::Dependence(handle(0xE3, 0)),
                ],
                is_allowed: true,
                ..Default::default()
            },
        ];
        let ids = compute_logical_ids(ops);
        assert_eq!(ids.len(), 3, "each allowed node should have a pattern_id");
        let mut unique: Vec<Vec<u8>> = ids.values().cloned().collect();
        unique.sort();
        unique.dedup();
        assert_eq!(
            unique.len(),
            3,
            "3 distinct opcodes → 3 distinct pattern_ids"
        );
    }

    #[test]
    fn source_node_feeding_two_groups_not_inherited() {
        // TrivialEncrypt feeds both FheAdd✓ and FheSub✓ (different groups).
        // Source node should NOT appear in result map (feeds >1 pattern).
        let h_src = handle(0x01, 0);
        let ops = vec![
            DFGOp {
                output_handle: h_src.clone(),
                fhe_op: SupportedFheOperations::FheTrivialEncrypt,
                inputs: vec![DFGTaskInput::Value(
                    fhevm_engine_common::types::SupportedFheCiphertexts::Scalar(vec![0u8]),
                )],
                is_allowed: false,
                ..Default::default()
            },
            DFGOp {
                output_handle: handle(0x10, 0),
                fhe_op: SupportedFheOperations::FheAdd,
                inputs: vec![
                    DFGTaskInput::Dependence(h_src.clone()),
                    DFGTaskInput::Dependence(handle(0xE0, 0)),
                ],
                is_allowed: true,
                ..Default::default()
            },
            DFGOp {
                output_handle: handle(0x20, 0),
                fhe_op: SupportedFheOperations::FheSub,
                inputs: vec![
                    DFGTaskInput::Dependence(h_src.clone()),
                    DFGTaskInput::Dependence(handle(0xE1, 0)),
                ],
                is_allowed: true,
                ..Default::default()
            },
        ];
        let ids = compute_logical_ids(ops);
        // The source node feeds two groups with different pattern_ids,
        // so it should not inherit either. Only the two allowed nodes appear.
        assert_eq!(
            ids.len(),
            2,
            "source node feeding 2 groups should not be in result"
        );
    }

    #[test]
    fn fhe_rand_is_source_node() {
        // FheRand (no deps) feeds FheAdd✓. FheRand should inherit the group's
        // pattern_id (same behavior as TrivialEncrypt feeding a single group).
        let h_rand = handle(0x01, 0);
        let h_add = handle(0x10, 0);
        let ops = vec![
            DFGOp {
                output_handle: h_rand.clone(),
                fhe_op: SupportedFheOperations::FheRand,
                inputs: vec![DFGTaskInput::Value(
                    fhevm_engine_common::types::SupportedFheCiphertexts::Scalar(vec![0u8]),
                )],
                is_allowed: false,
                ..Default::default()
            },
            DFGOp {
                output_handle: h_add.clone(),
                fhe_op: SupportedFheOperations::FheAdd,
                inputs: vec![
                    DFGTaskInput::Dependence(h_rand.clone()),
                    DFGTaskInput::Dependence(handle(0xE0, 0)),
                ],
                is_allowed: true,
                ..Default::default()
            },
        ];
        let ids = compute_logical_ids(ops);
        // Both FheRand and FheAdd should have the same pattern_id (source
        // feeds single group → inherits).
        assert_eq!(
            ids.len(),
            2,
            "both rand source and add should have pattern_ids"
        );
        let vals: Vec<Vec<u8>> = ids.values().cloned().collect();
        assert_eq!(
            vals[0], vals[1],
            "source feeding single group inherits pattern_id"
        );
    }

    #[test]
    fn three_way_fan_out_single_group() {
        // One intermediate (non-allowed) feeds 3 allowed outputs.
        // The intermediate and all 3 outputs should merge into one group.
        let h_mid = handle(0x01, 0);
        let h_a = handle(0x10, 0);
        let h_b = handle(0x20, 0);
        let h_c = handle(0x30, 0);
        let ops = vec![
            DFGOp {
                output_handle: h_mid.clone(),
                fhe_op: SupportedFheOperations::FheAdd,
                inputs: vec![
                    DFGTaskInput::Dependence(handle(0xE0, 0)),
                    DFGTaskInput::Dependence(handle(0xE1, 0)),
                ],
                is_allowed: false,
                ..Default::default()
            },
            DFGOp {
                output_handle: h_a.clone(),
                fhe_op: SupportedFheOperations::FheSub,
                inputs: vec![
                    DFGTaskInput::Dependence(h_mid.clone()),
                    DFGTaskInput::Dependence(handle(0xE2, 0)),
                ],
                is_allowed: true,
                ..Default::default()
            },
            DFGOp {
                output_handle: h_b.clone(),
                fhe_op: SupportedFheOperations::FheMul,
                inputs: vec![
                    DFGTaskInput::Dependence(h_mid.clone()),
                    DFGTaskInput::Dependence(handle(0xE3, 0)),
                ],
                is_allowed: true,
                ..Default::default()
            },
            DFGOp {
                output_handle: h_c.clone(),
                fhe_op: SupportedFheOperations::FheAdd,
                inputs: vec![
                    DFGTaskInput::Dependence(h_mid.clone()),
                    DFGTaskInput::Dependence(handle(0xE4, 0)),
                ],
                is_allowed: true,
                ..Default::default()
            },
        ];
        let ids = compute_logical_ids(ops);
        assert_eq!(
            ids.len(),
            4,
            "all 4 nodes (1 intermediate + 3 allowed) should be in result"
        );
        let mut unique: Vec<Vec<u8>> = ids.values().cloned().collect();
        unique.sort();
        unique.dedup();
        assert_eq!(
            unique.len(),
            1,
            "shared intermediate merges all into one group"
        );
    }

    #[test]
    fn all_non_allowed_empty_result() {
        // Two non-allowed computation nodes, no allowed nodes → empty result.
        let ops = vec![
            DFGOp {
                output_handle: handle(0x10, 0),
                fhe_op: SupportedFheOperations::FheAdd,
                inputs: vec![
                    DFGTaskInput::Dependence(handle(0xE0, 0)),
                    DFGTaskInput::Dependence(handle(0xE1, 0)),
                ],
                is_allowed: false,
                ..Default::default()
            },
            DFGOp {
                output_handle: handle(0x20, 0),
                fhe_op: SupportedFheOperations::FheSub,
                inputs: vec![
                    DFGTaskInput::Dependence(handle(0xE2, 0)),
                    DFGTaskInput::Dependence(handle(0xE3, 0)),
                ],
                is_allowed: false,
                ..Default::default()
            },
        ];
        let ids = compute_logical_ids(ops);
        assert!(ids.is_empty(), "no allowed nodes → no pattern_ids");
    }

    #[test]
    fn only_source_nodes_empty_result() {
        // Two TrivialEncrypt nodes, no allowed nodes → empty result.
        let ops = vec![
            DFGOp {
                output_handle: handle(0x10, 0),
                fhe_op: SupportedFheOperations::FheTrivialEncrypt,
                inputs: vec![DFGTaskInput::Value(
                    fhevm_engine_common::types::SupportedFheCiphertexts::Scalar(vec![0u8]),
                )],
                is_allowed: false,
                ..Default::default()
            },
            DFGOp {
                output_handle: handle(0x20, 0),
                fhe_op: SupportedFheOperations::FheTrivialEncrypt,
                inputs: vec![DFGTaskInput::Value(
                    fhevm_engine_common::types::SupportedFheCiphertexts::Scalar(vec![1u8]),
                )],
                is_allowed: false,
                ..Default::default()
            },
        ];
        let ids = compute_logical_ids(ops);
        assert!(ids.is_empty(), "only source nodes → no pattern_ids");
    }

    // -----------------------------------------------------------------------
    // Transaction-level pattern ID tests
    // -----------------------------------------------------------------------

    #[test]
    fn transaction_pattern_id_single_vs_triple_transfer() {
        let tx_id = vec![0xFFu8; 32];

        // Single transferFrom: one tx-level pattern
        let ops_single = build_transfer_from_ops(0x10);
        let (components_single, _) = build_component_nodes(ops_single, &tx_id).unwrap();
        let tx_pat_single = &components_single[0].transaction_pattern_id;
        // All components in the same tx share the same transaction_pattern_id
        for c in &components_single {
            assert_eq!(
                &c.transaction_pattern_id, tx_pat_single,
                "all components within one tx share the same transaction_pattern_id"
            );
        }

        // Three independent transferFrom calls in one tx: different tx-level pattern
        let mut ops_triple = build_transfer_from_ops(0x10);
        ops_triple.extend(build_transfer_from_ops(0x20));
        ops_triple.extend(build_transfer_from_ops(0x30));
        let (components_triple, _) = build_component_nodes(ops_triple, &tx_id).unwrap();
        let tx_pat_triple = &components_triple[0].transaction_pattern_id;
        for c in &components_triple {
            assert_eq!(
                &c.transaction_pattern_id, tx_pat_triple,
                "all components within one tx share the same transaction_pattern_id"
            );
        }

        // Single-transfer tx vs triple-transfer tx should differ
        assert_ne!(
            tx_pat_single, tx_pat_triple,
            "a 1-transfer tx and a 3-transfer tx should have different transaction_pattern_ids"
        );
    }

    #[test]
    fn transaction_pattern_id_same_structure_same_id() {
        let tx_id = vec![0xFFu8; 32];

        // Two txs with identical structure (same number of ops, same shape)
        // but different handle prefixes should produce the same tx pattern.
        let ops_a = build_transfer_from_ops(0x10);
        let ops_b = build_transfer_from_ops(0x20);
        let (components_a, _) = build_component_nodes(ops_a, &tx_id).unwrap();
        let (components_b, _) = build_component_nodes(ops_b, &tx_id).unwrap();

        assert_eq!(
            components_a[0].transaction_pattern_id, components_b[0].transaction_pattern_id,
            "same tx structure with different handles should produce same transaction_pattern_id"
        );
    }

    #[test]
    fn transaction_pattern_id_different_structure_different_id() {
        let tx_id = vec![0xFFu8; 32];

        // transferFrom (9 ops)
        let ops_transfer = build_transfer_from_ops(0x10);
        let (components_transfer, _) = build_component_nodes(ops_transfer, &tx_id).unwrap();

        // simple single FheAdd
        let ops_simple = vec![DFGOp {
            output_handle: handle(0x50, 0),
            fhe_op: SupportedFheOperations::FheAdd,
            inputs: vec![
                DFGTaskInput::Dependence(handle(0xD0, 0)),
                DFGTaskInput::Dependence(handle(0xD1, 0)),
            ],
            is_allowed: true,
            ..Default::default()
        }];
        let (components_simple, _) = build_component_nodes(ops_simple, &tx_id).unwrap();

        assert_ne!(
            components_transfer[0].transaction_pattern_id,
            components_simple[0].transaction_pattern_id,
            "transferFrom tx and simple-add tx should have different transaction_pattern_ids"
        );
    }

    // -----------------------------------------------------------------------
    // Regression: per-op pattern_id survives partition merging
    // -----------------------------------------------------------------------

    #[test]
    fn chain_of_allowed_nodes_per_op_pattern_ids() {
        // A✓(Add) → B✓(Sub) → C✓(Mul): each allowed node is its own cone
        // boundary, so each gets a DISTINCT pattern_id. But
        // partition_preserving_parallelism merges the chain (in-degree=1,
        // out-degree=1) into ONE component. With the old per-component
        // pattern_id this was a debug_assert! violation; with per-op
        // pattern_ids each OpNode keeps its own.
        let tx_id = vec![0xFFu8; 32];

        let h_a = handle(0x10, 0);
        let h_b = handle(0x20, 0);
        let h_c = handle(0x30, 0);
        let ops = vec![
            DFGOp {
                output_handle: h_a.clone(),
                fhe_op: SupportedFheOperations::FheAdd,
                inputs: vec![
                    DFGTaskInput::Dependence(handle(0xE0, 0)),
                    DFGTaskInput::Dependence(handle(0xE1, 0)),
                ],
                is_allowed: true,
                ..Default::default()
            },
            DFGOp {
                output_handle: h_b.clone(),
                fhe_op: SupportedFheOperations::FheSub,
                inputs: vec![
                    DFGTaskInput::Dependence(h_a.clone()),
                    DFGTaskInput::Dependence(handle(0xE2, 0)),
                ],
                is_allowed: true,
                ..Default::default()
            },
            DFGOp {
                output_handle: h_c.clone(),
                fhe_op: SupportedFheOperations::FheMul,
                inputs: vec![
                    DFGTaskInput::Dependence(h_b.clone()),
                    DFGTaskInput::Dependence(handle(0xE3, 0)),
                ],
                is_allowed: true,
                ..Default::default()
            },
        ];

        let (components, _) = build_component_nodes(ops, &tx_id).unwrap();

        // The chain should be merged into ONE component by the partitioner
        // (each link has in-degree=1 and out-degree=1).
        assert_eq!(
            components.len(),
            1,
            "A→B→C chain should be merged into one component"
        );

        // Each OpNode in that component's inner DFG must have a DISTINCT
        // non-empty operation_pattern_id (3 different opcodes → 3 distinct encodings).
        let op_pids = collect_op_pattern_ids(&components[0]);
        assert_eq!(op_pids.len(), 3, "component should contain 3 ops");

        for (i, pid) in op_pids.iter().enumerate() {
            assert!(
                !pid.is_empty(),
                "operation_pattern_id for op {i} should not be empty"
            );
            assert_valid_pattern(pid);
        }

        let mut unique = op_pids.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(
            unique.len(),
            3,
            "3 distinct opcodes (Add, Sub, Mul) in a chain of allowed nodes \
             must produce 3 distinct per-op pattern_ids, got {} distinct",
            unique.len()
        );

        // transaction_pattern_id is uniform across the component (whole-tx encoding)
        let tx_pid = &components[0].transaction_pattern_id;
        assert!(
            !tx_pid.is_empty(),
            "transaction_pattern_id should not be empty"
        );
        assert_valid_pattern(tx_pid);
    }

    // -----------------------------------------------------------------------
    // Encoding / decoding round-trip tests
    // -----------------------------------------------------------------------

    #[test]
    fn decode_single_node_pattern() {
        // Single FheAdd with 2 external inputs, is_allowed=true
        let ops = vec![DFGOp {
            output_handle: handle(0x10, 0),
            fhe_op: SupportedFheOperations::FheAdd,
            inputs: vec![
                DFGTaskInput::Dependence(handle(0xE0, 0)),
                DFGTaskInput::Dependence(handle(0xE1, 0)),
            ],
            is_allowed: true,
            ..Default::default()
        }];
        let ids = compute_logical_ids(ops);
        let pattern = ids.values().next().unwrap();

        let desc = decode_pattern(pattern).expect("should decode");
        assert_eq!(desc.nodes.len(), 1);
        assert_eq!(desc.nodes[0].opcode, SupportedFheOperations::FheAdd as i32);
        assert_eq!(desc.nodes[0].opcode_name, "FHE_ADD");
        assert!(desc.nodes[0].is_allowed);
        assert_eq!(desc.nodes[0].inputs.len(), 2);
        // Both inputs are external
        for inp in &desc.nodes[0].inputs {
            assert!(matches!(inp, super::PatternInput::External));
        }
        assert_eq!(format!("{desc}"), "FHE_ADD[a]");
    }

    #[test]
    fn decode_multi_node_pattern() {
        // Two-node group: FheAdd(ext, ext) → FheSub(Add, ext), Sub is allowed
        let h_add = handle(0x10, 0);
        let h_sub = handle(0x20, 0);
        let ops = vec![
            DFGOp {
                output_handle: h_add.clone(),
                fhe_op: SupportedFheOperations::FheAdd,
                inputs: vec![
                    DFGTaskInput::Dependence(handle(0xE0, 0)),
                    DFGTaskInput::Dependence(handle(0xE1, 0)),
                ],
                is_allowed: false,
                ..Default::default()
            },
            DFGOp {
                output_handle: h_sub.clone(),
                fhe_op: SupportedFheOperations::FheSub,
                inputs: vec![
                    DFGTaskInput::Dependence(h_add.clone()),
                    DFGTaskInput::Dependence(handle(0xE2, 0)),
                ],
                is_allowed: true,
                ..Default::default()
            },
        ];
        let ids = compute_logical_ids(ops);
        // Both should be in the same group
        assert_eq!(ids.len(), 2);
        let pattern = ids.values().next().unwrap();
        let desc = decode_pattern(pattern).expect("should decode");
        assert_eq!(desc.nodes.len(), 2);

        // Node 0: FheAdd, not allowed, 2 external inputs
        assert_eq!(desc.nodes[0].opcode_name, "FHE_ADD");
        assert!(!desc.nodes[0].is_allowed);
        assert_eq!(desc.nodes[0].inputs.len(), 2);

        // Node 1: FheSub, allowed, 1 internal + 1 external
        assert_eq!(desc.nodes[1].opcode_name, "FHE_SUB");
        assert!(desc.nodes[1].is_allowed);
        assert_eq!(desc.nodes[1].inputs.len(), 2);
        assert!(matches!(
            desc.nodes[1].inputs[0],
            super::PatternInput::Internal(0)
        ));
        assert!(matches!(
            desc.nodes[1].inputs[1],
            super::PatternInput::External
        ));

        assert_eq!(format!("{desc}"), "FHE_ADD,FHE_SUB[a]");
    }

    #[test]
    fn base64url_roundtrip() {
        let ops = vec![DFGOp {
            output_handle: handle(0x10, 0),
            fhe_op: SupportedFheOperations::FheAdd,
            inputs: vec![
                DFGTaskInput::Dependence(handle(0xE0, 0)),
                DFGTaskInput::Dependence(handle(0xE1, 0)),
            ],
            is_allowed: true,
            ..Default::default()
        }];
        let ids = compute_logical_ids(ops);
        let pattern = ids.values().next().unwrap();
        let b64 = pattern_to_base64url(pattern);
        // Verify the base64url decodes back to the same bytes
        let decoded = BASE64URL_NOPAD
            .decode(b64.as_bytes())
            .expect("should decode base64url");
        assert_eq!(&decoded, pattern);
        // Verify the base64url string is much shorter than the old 64-char hex
        assert!(
            b64.len() < 64,
            "base64url for single-node pattern should be shorter than 64 chars, got {}",
            b64.len()
        );
    }

    // -----------------------------------------------------------------------
    // V1 encoding limit tests
    // -----------------------------------------------------------------------

    #[test]
    fn oversize_group_returns_no_pattern() {
        // Build a group with > 255 nodes (V1_MAX_NODES).
        // All are independent allowed FheAdd ops with external inputs.
        let n = 256;
        let ops: Vec<DFGOp> = (0..n)
            .map(|i| {
                let mut h = vec![0u8; 32];
                h[0] = (i >> 8) as u8;
                h[1] = (i & 0xFF) as u8;
                DFGOp {
                    output_handle: h,
                    fhe_op: SupportedFheOperations::FheAdd,
                    inputs: vec![
                        DFGTaskInput::Dependence(handle(0xE0, i as u8)),
                        DFGTaskInput::Dependence(handle(0xE1, i as u8)),
                    ],
                    is_allowed: true,
                    ..Default::default()
                }
            })
            .collect();
        let ids = compute_logical_ids(ops);
        // With 256 independent allowed nodes, each is its own group (1 node),
        // so individual groups are within limits. The overflow applies to
        // transaction_pattern_id which encodes all 256 as one group.
        // Verify individual groups still work:
        assert!(
            !ids.is_empty(),
            "individual single-node groups should still encode"
        );

        // Now test that compute_transaction_pattern_id falls back to wide
        // encoding for >255-node groups (which exceed v1 u8 node_count).
        let n = 256;
        let mut ops2: Vec<DFGOp> = Vec::with_capacity(n);
        // Build a chain: each op depends on the previous, so the whole thing
        // is one connected component for the tx-level encoding.
        for i in 0..n {
            let mut h = vec![0u8; 32];
            h[0] = (i >> 8) as u8;
            h[1] = (i & 0xFF) as u8;
            let inputs = if i == 0 {
                vec![
                    DFGTaskInput::Dependence(handle(0xE0, 0)),
                    DFGTaskInput::Dependence(handle(0xE1, 0)),
                ]
            } else {
                let mut prev_h = vec![0u8; 32];
                prev_h[0] = ((i - 1) >> 8) as u8;
                prev_h[1] = ((i - 1) & 0xFF) as u8;
                vec![
                    DFGTaskInput::Dependence(prev_h),
                    DFGTaskInput::Dependence(handle(0xE1, i as u8)),
                ]
            };
            ops2.push(DFGOp {
                output_handle: h,
                fhe_op: SupportedFheOperations::FheAdd,
                inputs,
                is_allowed: i == n - 1, // only last is allowed
                ..Default::default()
            });
        }

        let tx_id = vec![0xFFu8; 32];
        let (components, _) = build_component_nodes(ops2, &tx_id).unwrap();
        // >255 nodes: v1 encoding fails, wide encoding fallback → hashed pattern
        let tx_pat = &components[0].transaction_pattern_id;
        assert!(
            is_hashed_pattern(tx_pat),
            "256-node tx should produce hashed transaction_pattern_id via wide fallback"
        );
        assert_eq!(tx_pat.len(), 23, "hashed pattern should be 23 bytes");
        let node_count = u16::from_be_bytes([tx_pat[1], tx_pat[2]]);
        assert_eq!(
            node_count, 256,
            "hashed pattern should encode node_count=256"
        );
    }

    // -----------------------------------------------------------------------
    // Two-tier encoding tests
    // -----------------------------------------------------------------------

    /// Helper: build a chain of `n` FheAdd ops where each depends on the
    /// previous (except the first which has external inputs). Only the last
    /// is `is_allowed`. This creates a single logical group of `n` computation
    /// nodes. Uses distinct handle prefixes to avoid collisions.
    fn build_chain(n: usize, prefix: u8) -> Vec<DFGOp> {
        let mut ops = Vec::with_capacity(n);
        for i in 0..n {
            let mut h = vec![0u8; 32];
            h[0] = prefix;
            h[1] = (i >> 8) as u8;
            h[2] = (i & 0xFF) as u8;
            let inputs = if i == 0 {
                vec![
                    DFGTaskInput::Dependence(handle(0xE0, prefix)),
                    DFGTaskInput::Dependence(handle(0xE1, prefix)),
                ]
            } else {
                let mut prev_h = vec![0u8; 32];
                prev_h[0] = prefix;
                prev_h[1] = ((i - 1) >> 8) as u8;
                prev_h[2] = ((i - 1) & 0xFF) as u8;
                vec![
                    DFGTaskInput::Dependence(prev_h),
                    DFGTaskInput::Dependence(handle(0xE1, (i & 0xFF) as u8)),
                ]
            };
            ops.push(DFGOp {
                output_handle: h,
                fhe_op: SupportedFheOperations::FheAdd,
                inputs,
                is_allowed: i == n - 1,
                ..Default::default()
            });
        }
        ops
    }

    #[test]
    fn pattern_below_threshold_is_encoding() {
        // A group with ≤ DEFAULT_PATTERN_HASH_THRESHOLD nodes stays v1.
        let ops = build_chain(10, 0xA0);
        let ids = compute_logical_ids(ops);
        assert!(!ids.is_empty());
        let pattern = ids.values().next().unwrap();
        assert_eq!(
            pattern[0], ENCODING_VERSION,
            "small group should produce v1 encoding"
        );
        assert!(
            decode_pattern(pattern).is_some(),
            "v1 encoding should be decodable"
        );
        assert!(
            !is_hashed_pattern(pattern),
            "v1 encoding should not be identified as hashed"
        );
    }

    #[test]
    fn pattern_above_threshold_is_hashed() {
        // A group with > DEFAULT_PATTERN_HASH_THRESHOLD nodes gets hashed.
        // Default threshold is 25, so 30 nodes should trigger hashing.
        let ops = build_chain(30, 0xB0);
        let ids = compute_logical_ids(ops);
        assert!(!ids.is_empty());
        let pattern = ids.values().next().unwrap();
        assert_eq!(
            pattern[0], HASH_VERSION,
            "large group should produce v2 hashed pattern"
        );
        assert_eq!(
            pattern.len(),
            23,
            "hashed pattern should be exactly 23 bytes"
        );
        // node_count is encoded as u16 big-endian at bytes 1-2
        let node_count = u16::from_be_bytes([pattern[1], pattern[2]]);
        assert_eq!(node_count, 30, "hashed pattern should encode node_count=30");
        assert!(
            is_hashed_pattern(pattern),
            "v2 pattern should be identified as hashed"
        );
        assert!(
            decode_pattern(pattern).is_none(),
            "hashed pattern should not be decodable"
        );
        // Base64url should be at most 31 chars (23 bytes → ceil(23*4/3) = 31)
        let b64 = pattern_to_base64url(pattern);
        assert!(
            b64.len() <= 31,
            "hashed pattern base64url should be ≤31 chars, got {}",
            b64.len()
        );
    }

    #[test]
    fn hashed_pattern_deterministic() {
        // Two chains of the same length with different handles should produce
        // the same hash (structure is identical, handles don't matter).
        let ops_a = build_chain(30, 0xC0);
        let ops_b = build_chain(30, 0xD0);
        let ids_a = compute_logical_ids(ops_a);
        let ids_b = compute_logical_ids(ops_b);
        let pat_a = ids_a.values().next().unwrap();
        let pat_b = ids_b.values().next().unwrap();
        assert_eq!(
            pat_a, pat_b,
            "same structure with different handles should produce identical hashed pattern"
        );
    }
}
