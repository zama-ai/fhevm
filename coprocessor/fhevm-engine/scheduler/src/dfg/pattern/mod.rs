//! # DFG Pattern IDs
//!
//! Deterministic structural fingerprints of a Data Flow Graph, used as
//! low-cardinality span attributes for latency segmentation in OpenTelemetry
//! metrics.
//!
//! Two transactions performing the same FHE computation (e.g., an ERC20
//! `transferFrom`) produce the **same** `operation_pattern_id` regardless of
//! which ciphertext handles, addresses, or transaction IDs are involved.
//! A `transaction_pattern_id` fingerprints the **entire** transaction graph
//! for dashboard segmentation at the tx level.
//!
//! Only graph **shape** contributes to the fingerprint — runtime identifiers
//! (output handles, transaction IDs, ciphertext data) are excluded.
//!
//! ## Module organization
//!
//! - [`types`] — Data types (`PatternDescription`, `PatternNode`, `PatternInput`)
//! - [`encoding`] — Binary encoding/decoding, hash finalization
//! - [`grouping`] — Logical-operation grouping algorithm, union-find

mod encoding;
mod grouping;
mod types;

pub use encoding::{decode_pattern, is_hashed_pattern, pattern_to_base64url};
pub use grouping::{compute_logical_pattern_ids, compute_transaction_pattern_id};
pub use types::{PatternDescription, PatternInput, PatternNode};

#[cfg(test)]
use encoding::{ENCODING_VERSION, HASH_VERSION};

#[cfg(test)]
mod tests;
