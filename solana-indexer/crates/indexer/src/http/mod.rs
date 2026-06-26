//! HTTP layer: shared state + endpoints + error body + OpenAPI.

pub mod endpoints;
pub mod error;
pub mod openapi;
pub mod server;

use std::sync::Arc;

use crate::metrics::Metrics;
use crate::rpc::SolanaRpc;
use crate::store::repositories::lineage_repo::LineageRepo;

/// Shared application state injected into every handler.
#[derive(Clone)]
pub struct AppState {
    pub repo: LineageRepo,
    /// Optional RPC client for the on-chain `build_verified_proof` cross-check.
    /// `None` disables verification (proofs returned with `verified = false`).
    pub rpc: Option<SolanaRpc>,
    pub metrics: Arc<Metrics>,
}
