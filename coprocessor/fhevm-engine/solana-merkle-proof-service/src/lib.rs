//! The Solana Merkle proof service: `solana_merkle_indexer` rebuilds the leaf record of
//! every encrypted store from finalized blocks into its own database, and
//! `solana_merkle_proof_server` answers the KMS connector's inclusion proofs from it.

pub mod answer_cache;
pub mod indexer;
pub mod kms_tx_senders;
pub mod server;
pub mod store;
pub mod store_check;

/// The record's schema. The indexer applies it at start; the proof server only reads.
pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

/// Seconds since the Unix epoch; 0 before it.
pub(crate) fn unix_now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs())
}

/// `value` as a Prometheus integer gauge holds it, saturated.
pub(crate) fn gauge_value(value: u64) -> i64 {
    i64::try_from(value).unwrap_or(i64::MAX)
}
