mod ops;
mod transaction_sender;

#[derive(Clone, Debug)]
pub struct ConfigSettings {
    pub db_url: String,
    pub db_pool_size: u32,

    pub verify_proof_resp_db_channel: String,
    pub add_ciphertexts_db_channel: String,

    pub verify_proof_resp_batch_limit: u32,
    pub verify_proof_resp_max_retries: u32,

    pub db_polling_interval_secs: u16,

    pub error_sleep_initial_secs: u16,
    pub error_sleep_max_secs: u16,
}

impl Default for ConfigSettings {
    fn default() -> Self {
        Self {
            db_url: std::env::var("DATABASE_URL").expect("DATABASE_URL is undefined"),
            db_pool_size: 10,
            verify_proof_resp_db_channel: "verify_proofs".to_owned(),
            add_ciphertexts_db_channel: "add_ciphertexts".to_owned(),
            verify_proof_resp_batch_limit: 128,
            verify_proof_resp_max_retries: 15,
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
