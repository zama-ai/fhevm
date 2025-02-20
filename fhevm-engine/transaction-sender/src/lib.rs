mod ops;
mod transaction_sender;

#[derive(Clone, Debug)]
pub struct ConfigSettings {
    pub database_url: String,
    pub database_pool_size: u32,

    pub verify_proof_resp_db_channel: String,
    pub add_ciphertexts_db_channel: String,

    pub verify_proof_resp_batch_limit: u32,
    pub verify_proof_resp_max_retries: u32,
    pub verify_proof_remove_after_max_retries: bool,

    pub db_polling_interval_secs: u16,

    pub error_sleep_initial_secs: u16,
    pub error_sleep_max_secs: u16,
}

impl Default for ConfigSettings {
    fn default() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or("postgres://postgres:postgres@localhost/coprocessor".to_owned()),
            database_pool_size: 10,
            verify_proof_resp_db_channel: "verify_proof_responses".to_owned(),
            add_ciphertexts_db_channel: "add_ciphertexts".to_owned(),
            verify_proof_resp_batch_limit: 128,
            verify_proof_resp_max_retries: 15,
            verify_proof_remove_after_max_retries: true,
            db_polling_interval_secs: 5,
            error_sleep_initial_secs: 1,
            error_sleep_max_secs: 16,
        }
    }
}

pub use transaction_sender::TransactionSender;

pub const TXN_SENDER_TARGET: &str = "txn_sender";
pub const VERIFY_PROOFS_TARGET: &str = "verify_proofs";
pub const ADD_CIPHERTEXTS_TARGET: &str = "add_ciphertexts";
