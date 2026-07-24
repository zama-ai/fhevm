use std::time::Duration;

use fhevm_engine_common::gateway_http::{
    DEFAULT_GATEWAY_HTTP_REQUEST_TIMEOUT, DEFAULT_GATEWAY_HTTP_REQUEST_TIMEOUT_SECS,
};

pub const DEFAULT_GAS_LIMIT_OVERPROVISION_PERCENT: u32 = 120;

#[derive(Clone, Debug)]
pub struct ConfigSettings {
    pub verify_proof_resp_db_channel: String,
    pub add_ciphertexts_db_channel: String,
    pub verify_proof_resp_batch_limit: u32,
    pub verify_proof_resp_max_retries: u32,
    pub verify_proof_remove_after_max_retries: bool,

    pub add_ciphertexts_batch_limit: u32,

    // For now, use i32 as that's what we have in the DB as integer type.
    pub add_ciphertexts_max_retries: i32,

    pub db_polling_interval_secs: u16,

    pub error_sleep_initial_secs: u16,
    pub error_sleep_max_secs: u16,

    pub send_txn_sync_timeout_secs: u16,

    pub review_after_unlimited_retries: u16,

    pub health_check_port: u16,

    pub health_check_timeout: Duration,

    pub gas_limit_overprovision_percent: u32,

    pub graceful_shutdown_timeout: Duration,

    /// Used during blue/green (GCS) upgrade migrations. True when this binary
    /// is the incoming green stack (its `STACK_VERSION` is newer than the live
    /// `versioning.stack_version`; auto-detected via `resolve_gcs_mode`).
    ///
    /// When true, the txn-sender starts fully parked and does nothing until the
    /// cutover finalizes — `execute_cutover` fires `event_stack_version_upgraded`
    /// atomically with the version bump, which flips this binary out of GCS mode
    /// and lets it begin submitting. It never participates in the dry-run window,
    /// since submitting on-chain there would double-submit against the still-live
    /// blue stack. Writes always target `public` (by cutover time the `gcs`
    /// schema has been merged into `public` and dropped). When false (default),
    /// the binary is the live (blue) stack and runs normally.
    pub gcs_mode: bool,
}

impl Default for ConfigSettings {
    fn default() -> Self {
        Self {
            verify_proof_resp_db_channel: "event_zkpok_computed".to_owned(),
            add_ciphertexts_db_channel: "event_ciphertexts_uploaded".to_owned(),
            verify_proof_resp_batch_limit: 128,
            verify_proof_resp_max_retries: 6,
            verify_proof_remove_after_max_retries: true,
            db_polling_interval_secs: 5,
            error_sleep_initial_secs: 1,
            error_sleep_max_secs: 4,
            add_ciphertexts_batch_limit: 10,
            add_ciphertexts_max_retries: i32::MAX,
            send_txn_sync_timeout_secs: DEFAULT_GATEWAY_HTTP_REQUEST_TIMEOUT_SECS,
            review_after_unlimited_retries: 30,
            health_check_port: 8080,
            health_check_timeout: DEFAULT_GATEWAY_HTTP_REQUEST_TIMEOUT,
            gas_limit_overprovision_percent: DEFAULT_GAS_LIMIT_OVERPROVISION_PERCENT,
            graceful_shutdown_timeout: Duration::from_secs(8),
            gcs_mode: false,
        }
    }
}
