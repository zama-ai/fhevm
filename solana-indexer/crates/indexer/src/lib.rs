//! Solana rotation-leaf indexer.
//!
//! Indexes the `zama-host` program's encrypted-value-ACL lineage instructions via
//! the Carbon framework (RPC transaction crawler, `finalized`), reconstructs each
//! lineage's ordered MMR leaf list through `zama_solana_acl::lineage`, and serves
//! inclusion proofs over HTTP/JSON for off-chain historical/public confidential-
//! balance decrypt. A client/SDK queries this service to obtain a proof BEFORE
//! signing a decrypt request; the KMS only verifies.

pub mod config;
pub mod decoder;
pub mod http;
pub mod lineage;
pub mod metrics;
pub mod pipeline;
pub mod rpc;
pub mod startup;
pub mod store;
pub mod tracing;

pub use startup::run;
