//! The Solana Merkle proof service: `solana_merkle_indexer` rebuilds the RFC 035 leaf record of
//! every encrypted store from confirmed blocks into its own database, and
//! `solana_merkle_proof_server` answers the KMS connector's inclusion proofs from it.

pub mod indexer;
pub mod kms_tx_senders;
pub mod server;
pub mod store;

/// The record's schema. The indexer applies it at start; the proof server only reads.
pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
