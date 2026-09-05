//! Schema of the normative leaf vectors, shared by every implementation that hashes ACL leaves.
//!
//! The host program seals leaves on chain, the coprocessor recomputes them from instruction
//! bytes, and the KMS connector verifies proofs against them. All three use the same shared
//! crate today; these vectors pin the bytes so a future reimplementation (or the EVM twin,
//! RFC 034) can prove it hashes identically. Include this file with `#[path]` from a test
//! target, deserialize `leaves_v1.json`, and recompute each record.
//!
//! Every 64-bit number is a decimal string so a TypeScript consumer cannot lose precision.

use serde::{Deserialize, Serialize};

/// Schema identifier written into every file this shape can parse.
pub const LEAF_VECTOR_SCHEMA: &str = "zama-solana-leaf-vectors/v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeafVectorFile {
    pub schema: String,
    pub description: String,
    pub regenerate_with: String,
    /// The hash function of leaf commitments and MMR nodes, by name.
    pub hash: String,
    /// Domain-separation prefixes as ASCII, so a reader sees them without decoding.
    pub prefixes: Prefixes,
    pub vectors: Vec<LeafVector>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Prefixes {
    pub historical_access_leaf: String,
    pub public_decrypt_leaf: String,
    pub mmr_leaf_node: String,
    pub mmr_node: String,
}

/// One account history: its ordered events, and the leaves, peaks and one proof they imply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeafVector {
    pub id: String,
    pub comment: String,
    /// The encrypted value account address, hex.
    pub encrypted_value_account: String,
    pub events: Vec<LeafEvent>,
    /// `keccak256`-domain leaf commitments in event order, hex.
    pub leaves: Vec<String>,
    /// Number of leaves, decimal string.
    pub leaf_count: String,
    /// MMR peaks after all events, oldest mountain first, hex.
    pub peaks: Vec<String>,
    /// An inclusion proof for one leaf.
    pub proof: ProofVector,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum LeafEvent {
    /// One allow of `key` on `handle`.
    Allowed { handle: String, key: String },
    /// `handle` made publicly decryptable.
    MarkedPublic { handle: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProofVector {
    /// Decimal string.
    pub leaf_index: String,
    /// Authentication path from the leaf to its peak, hex.
    pub siblings: Vec<String>,
}
