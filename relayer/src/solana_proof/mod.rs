//! Solana ACL MMR proof service (RFC-024).
//!
//! Ingests zama-host transactions, reconstructs per-lineage leaf histories,
//! and builds/verifies MMR inclusion proofs, reusing all proof math from
//! `zama_solana_acl::lineage`/`mmr` — this module owns ingestion, storage, and
//! HTTP wiring only.
//!
//! Layout:
//! - [`decode`]: instruction decoding (Anchor discriminators + borsh args).
//! - [`replay`]: per-lineage state tracking (`current_handle`, subjects)
//!   that turns decoded instructions into `LineageEvent`s.
//! - [`chain`]: `ChainFetcher` trait + a plain-`reqwest` JSON-RPC implementation.
//! - [`store`]: `LeafStore` trait (append-only event log + cursor) + a
//!   file-backed implementation.
//! - [`ingest`]: the poll loop and targeted per-lineage catch-up.
//! - [`proof`]: `build_proof`, the public entry point tying the above together.
//! - [`http`]: an internal HTTP endpoint exposing `build_proof` so a client can
//!   discover a proof before signing and submitting a Solana user-decrypt request.

pub mod chain;
pub mod config;
pub mod decode;
pub mod http;
pub mod ingest;
pub mod poller;
pub mod proof;
pub mod replay;
pub mod store;
// This behavior-neutral source seam is consumed by the stacked SQL ingestion
// cutover; keeping it private avoids publishing an intermediate provider API.
#[allow(dead_code)]
pub(crate) mod yellowstone_source;

pub use chain::{ChainFetcher, RpcChainFetcher};
pub use config::SolanaProofConfig;
pub use proof::{build_proof, MmrProofResult, ProofError};
pub use store::{FileLeafStore, LeafStore};
