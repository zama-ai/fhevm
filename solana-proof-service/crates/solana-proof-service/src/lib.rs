//! Internal Solana ACL MMR proof HTTP service (fhevm-internal #1682 / RFC-024).
//!
//! Proof reads are snapshot-only: [`solana_proof_store::SqlProofStore::proof_snapshot`]
//! plus a confirmed on-chain peak check. Request-triggered catch-up is intentionally
//! absent so background completed-block ingest remains the sole writer.

pub mod chain;
pub mod config;
pub mod http;
pub mod ingest_health;
pub mod metrics;
pub mod proof;
pub mod readiness;
pub mod startup_validation;

pub use chain::{ChainError, ChainFetcher, OnChainLineageState, RpcChainFetcher};
pub use config::ServiceConfig;
pub use http::{router, AppState};
pub use ingest_health::IngestHealth;
pub use proof::{build_proof, MmrProofResult, ProofError, ProofSnapshotSource};
pub use readiness::{evaluate_readiness, ReadinessClass, ReadinessQueryable, ReadinessReport};
