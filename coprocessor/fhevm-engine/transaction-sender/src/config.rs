use std::time::Duration;

pub const DEFAULT_GAS_LIMIT_OVERPROVISION_PERCENT: u32 = 120;

#[derive(Clone, Debug)]
pub struct ConfigSettings {
    pub verify_proof_resp_db_channel: String,
    pub add_ciphertexts_db_channel: String,
    pub allow_handle_db_channel: String,

    pub verify_proof_resp_batch_limit: u32,
    pub verify_proof_resp_max_retries: u32,
    pub verify_proof_remove_after_max_retries: bool,

    pub add_ciphertexts_batch_limit: u32,

    // For now, use i32 as that's what we have in the DB as integer type.
    pub add_ciphertexts_max_retries: i32,

    pub allow_handle_batch_limit: u32,

    // For now, use i32 as that's what we have in the DB as integer type.
    pub allow_handle_max_retries: i32,

    pub db_polling_interval_secs: u16,

    pub error_sleep_initial_secs: u16,
    pub error_sleep_max_secs: u16,

    pub send_txn_sync_timeout_secs: u16,

    pub review_after_unlimited_retries: u16,

    pub health_check_port: u16,

    pub health_check_timeout: Duration,

    pub gas_limit_overprovision_percent: u32,

    pub graceful_shutdown_timeout: Duration,

    pub delegation_fallback_polling: u64,
    pub delegation_max_retry: u64,
}

impl Default for ConfigSettings {
    fn default() -> Self {
        Self {
            verify_proof_resp_db_channel: "event_zkpok_computed".to_owned(),
            add_ciphertexts_db_channel: "event_ciphertexts_uploaded".to_owned(),
            allow_handle_db_channel: "event_allowed_handle".to_owned(),
            verify_proof_resp_batch_limit: 128,
            verify_proof_resp_max_retries: 6,
            verify_proof_remove_after_max_retries: true,
            db_polling_interval_secs: 5,
            error_sleep_initial_secs: 1,
            error_sleep_max_secs: 4,
            add_ciphertexts_batch_limit: 10,
            add_ciphertexts_max_retries: i32::MAX,
            allow_handle_batch_limit: 10,
            allow_handle_max_retries: i32::MAX,
            send_txn_sync_timeout_secs: 4,
            review_after_unlimited_retries: 30,
            health_check_port: 8080,
            health_check_timeout: Duration::from_secs(4),
            gas_limit_overprovision_percent: DEFAULT_GAS_LIMIT_OVERPROVISION_PERCENT,
            graceful_shutdown_timeout: Duration::from_secs(8),
            delegation_fallback_polling: 30,
            delegation_max_retry: 100_000,
        }
    }
}
