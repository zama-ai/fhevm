use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::sync::{LazyLock, Mutex};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use daggy::{Dag, NodeIndex};
use lru::LruCache;
use sha3::{Digest, Keccak256};

use super::types::{PatternDescription, PatternInput, PatternNode};
use crate::dfg::types::DFGTaskInput;
use crate::dfg::{DFGOp, OpEdge};
use fhevm_engine_common::common::FheOperation;

/// Tag byte for the inline, self-describing pattern encoding.
pub(super) const INLINE_PATTERN_TAG: u8 = 0x01;

/// Tag byte for the hashed pattern encoding (Keccak-256 truncated, not decodable).
pub(super) const HASHED_PATTERN_TAG: u8 = 0x02;

/// Number of bytes to keep from the Keccak-256 digest.
const HASH_DIGEST_LEN: usize = 20;

/// Default node-count threshold: groups with more nodes than this get hashed.
const DEFAULT_PATTERN_HASH_THRESHOLD: usize = 25;

/// LRU dedup cache so we log the full encoding once per unique hash.
/// Bounded at [`HASH_LOG_CACHE_SIZE`] entries; when full, the least-recently
/// seen pattern is evicted (and will be re-logged if it reappears).
static HASH_LOG_SEEN: LazyLock<Mutex<LruCache<Vec<u8>, ()>>> = LazyLock::new(|| {
    Mutex::new(LruCache::new(
        NonZeroUsize::new(HASH_LOG_CACHE_SIZE).unwrap(),
    ))
});

/// Maximum entries in the hash-log dedup LRU cache.
const HASH_LOG_CACHE_SIZE: usize = 10_000;

// ---------------------------------------------------------------------------
// Encoding / decoding
// ---------------------------------------------------------------------------

/// Encode the `pattern_id` bytes as a base64url (no-pad) string for use in
/// OTel span attributes.
pub fn pattern_to_base64url(bytes: &[u8]) -> String {
    URL_SAFE_NO_PAD.encode(bytes)
}

/// Decode a binary pattern encoding into a [`PatternDescription`].
///
/// Returns `None` if the encoding is malformed (wrong tag, truncated, etc.)
/// or if the pattern is a hashed form (hashes are not decodable).
pub fn decode_pattern(bytes: &[u8]) -> Option<PatternDescription> {
    if bytes.len() < 2 {
        return None;
    }
    // Only inline encodings are decodable; hashed and unknown tags are not.
    if bytes[0] != INLINE_PATTERN_TAG {
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

/// Returns `true` if the pattern bytes represent a hashed pattern.
pub fn is_hashed_pattern(bytes: &[u8]) -> bool {
    !bytes.is_empty() && bytes[0] == HASHED_PATTERN_TAG
}

/// Hash threshold read once from `FHEVM_PATTERN_HASH_THRESHOLD` env var,
/// falling back to [`DEFAULT_PATTERN_HASH_THRESHOLD`].
pub(super) static PATTERN_HASH_THRESHOLD: LazyLock<usize> = LazyLock::new(|| {
    std::env::var("FHEVM_PATTERN_HASH_THRESHOLD")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_PATTERN_HASH_THRESHOLD)
});

/// If the inline encoding has `node_count ≤ threshold`, return it as-is.
/// Otherwise hash it into a compact 23-byte form and log the full
/// encoding once (per unique hash) for operator linkability.
pub(super) fn finalize_pattern(encoding: Vec<u8>, threshold: usize) -> Vec<u8> {
    debug_assert!(encoding.len() >= 2 && encoding[0] == INLINE_PATTERN_TAG);
    let node_count = encoding[1] as usize;

    if node_count <= threshold {
        return encoding;
    }

    build_hash_pattern(&encoding, node_count, true)
}

/// Build a 23-byte hashed pattern from arbitrary encoding bytes.
///
/// ## Hashed pattern binary layout
///
/// ```text
/// Byte 0:     0x02 (HASHED_PATTERN_TAG)
/// Bytes 1-2:  node_count as u16 big-endian
/// Bytes 3-22: first 20 bytes of Keccak-256(encoding)
/// Total: 23 bytes
/// ```
pub(super) fn build_hash_pattern(
    encoding: &[u8],
    node_count: usize,
    log_full_encoding: bool,
) -> Vec<u8> {
    let digest = Keccak256::digest(encoding);

    let mut buf = Vec::with_capacity(1 + 2 + HASH_DIGEST_LEN);
    buf.push(HASHED_PATTERN_TAG);
    let node_count_u16 = u16::try_from(node_count).unwrap_or(u16::MAX);
    buf.extend_from_slice(&node_count_u16.to_be_bytes());
    buf.extend_from_slice(&digest[..HASH_DIGEST_LEN]);

    if log_full_encoding {
        // Log the full encoding once per unique hash for linkability.
        // The LRU cache evicts the least-recently seen pattern when full;
        // if an evicted pattern reappears it simply gets re-logged.
        let mut cache = HASH_LOG_SEEN.lock().unwrap();
        if cache.put(buf.clone(), ()).is_none() {
            let b64_hash = URL_SAFE_NO_PAD.encode(&buf);
            let b64_full = URL_SAFE_NO_PAD.encode(encoding);
            tracing::info!(
                pattern_hash = %b64_hash,
                pattern_full = %b64_full,
                node_count,
                "new hashed pattern: full encoding logged for linkability"
            );
        }
    }

    buf
}

// ---------------------------------------------------------------------------
// Subgraph encoding
// ---------------------------------------------------------------------------

/// Filter the parent graph's topo order to group members and build a
/// position map for internal-ref encoding.
pub(super) fn compute_subgraph_layout(
    group: &[usize],
    parent_topo: &[NodeIndex],
    graph: &Dag<(bool, usize), OpEdge>,
) -> Option<(Vec<usize>, HashMap<usize, usize>)> {
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

    let mut topo_pos: HashMap<usize, usize> = HashMap::with_capacity(local_topo.len());
    for (pos, &op_idx) in local_topo.iter().enumerate() {
        topo_pos.insert(op_idx, pos);
    }
    Some((local_topo, topo_pos))
}

/// Maximum number of nodes that an inline encoding can represent (node_count is u8).
const V1_MAX_NODES: usize = 255;

/// Maximum number of inputs per node that an inline encoding can represent
/// (3-bit field in the flags byte).
const V1_MAX_INPUTS_PER_NODE: usize = 7;

/// Maximum topo position that fits in an internal-ref byte (7-bit field).
const V1_MAX_INTERNAL_REF: usize = 127;

/// Encode a subgraph (a set of node indices within the pre-partition graph)
/// as a compact self-describing binary structure.
///
/// The encoding captures opcodes, `is_allowed` flags, input counts, and edges
/// (internal refs vs external) — all in topological order. It is decodable
/// without any external lookup (see [`decode_pattern`]). Only graph **shape**
/// contributes to the fingerprint; runtime identifiers are excluded.
///
/// Nodes in `group` are the computation nodes belonging to one logical operation.
/// `group` must be sorted. All inputs that come from outside the group (source
/// nodes, allowed nodes from other groups, DB handles) are treated as external
/// (byte 0x00).
///
/// Returns `None` if the group exceeds the inline encoding limits (\>255 nodes,
/// \>7 inputs per node, or \>127 internal-ref position). Callers can fall back
/// to wide-format hashing so oversized groups are still attributable via hash.
///
/// ## Inline binary layout
///
/// ```text
/// Byte 0:  0x01 (inline pattern tag)
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
pub(super) fn encode_subgraph(
    operations: &[DFGOp],
    group: &[usize],
    produced_handles: &HashMap<Vec<u8>, usize>,
    parent_topo: &[NodeIndex],
    graph: &Dag<(bool, usize), OpEdge>,
) -> Option<Vec<u8>> {
    let (local_topo, topo_pos) = compute_subgraph_layout(group, parent_topo, graph)?;

    let node_count = local_topo.len();
    if node_count > V1_MAX_NODES {
        return None;
    }

    // Pre-allocate: tag(1) + count(1) + per-node ~4 bytes average
    let mut buf: Vec<u8> = Vec::with_capacity(2 + node_count * 4);

    // Header
    buf.push(INLINE_PATTERN_TAG);
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
                            if src_pos > V1_MAX_INTERNAL_REF {
                                return None;
                            }
                            0x80 | (src_pos as u8)
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

/// Wide-format encoding used only as hash input when a group exceeds inline encoding limits.
///
/// This is intentionally not emitted as a pattern attribute directly: only the
/// resulting hash is used.
pub(super) fn encode_subgraph_hashable(
    operations: &[DFGOp],
    group: &[usize],
    produced_handles: &HashMap<Vec<u8>, usize>,
    parent_topo: &[NodeIndex],
    graph: &Dag<(bool, usize), OpEdge>,
) -> Option<Vec<u8>> {
    let (local_topo, topo_pos) = compute_subgraph_layout(group, parent_topo, graph)?;

    let node_count = local_topo.len();
    let mut buf: Vec<u8> = Vec::with_capacity(5 + node_count * 10);
    buf.push(0xFE); // wide format marker
    buf.extend_from_slice(&(node_count as u32).to_be_bytes());

    for &global_idx in &local_topo {
        let op = &operations[global_idx];
        buf.push(op.fhe_op as u8);
        buf.push(if op.is_allowed { 0x80 } else { 0x00 });
        buf.extend_from_slice(&(op.inputs.len() as u32).to_be_bytes());

        for input in &op.inputs {
            match input {
                DFGTaskInput::Dependence(h) => {
                    if let Some(&producer_idx) = produced_handles.get(h) {
                        if let Some(&src_pos) = topo_pos.get(&producer_idx) {
                            buf.push(0x01); // internal
                            buf.extend_from_slice(&(src_pos as u32).to_be_bytes());
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
