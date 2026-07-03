//! Solana ACL MMR proof service (RFC-024).
//!
//! Ingests zama-host transactions, reconstructs per-lineage leaf histories,
//! and builds/verifies MMR inclusion proofs, reusing all proof math from
//! `zama_solana_acl::lineage`/`mmr` — this module owns ingestion, storage, and
//! HTTP wiring only.
//!
//! Layout:
//! - [`decode`]: instruction decoding (Anchor discriminators + borsh args).
//! - [`replay`]: per-lineage state tracking (`current_handle`, subjects/roles)
//!   that turns decoded instructions into `LineageEvent`s.
//! - [`chain`]: `ChainFetcher` trait + a plain-`reqwest` JSON-RPC implementation.
//! - [`store`]: `LeafStore` trait (append-only event log + cursor) + a
//!   file-backed implementation.
//! - [`ingest`]: the poll loop and targeted per-lineage catch-up.
//! - [`proof`]: `build_proof`, the public entry point tying the above together.
//! - [`http`]: an internal HTTP endpoint exposing `build_proof`, and the
//!   in-process `SolanaProofService` handle used to construct it.
//!
//! **Integration point for the future Solana user-decrypt orchestrator**: once
//! that path exists, call [`proof::build_proof`] directly in-process (it takes
//! `&impl ChainFetcher, &impl LeafStore` — no HTTP round-trip needed) instead
//! of routing through [`http::mmr_proof_handler`], which exists only so this
//! service is independently reachable before that wiring lands.

pub mod chain;
pub mod config;
pub mod decode;
pub mod http;
pub mod ingest;
pub mod poller;
pub mod proof;
pub mod replay;
pub mod store;

pub use chain::{ChainFetcher, RpcChainFetcher};
pub use config::SolanaProofConfig;
pub use proof::{build_proof, MmrProofResult, ProofError};
pub use store::{FileLeafStore, LeafStore};
