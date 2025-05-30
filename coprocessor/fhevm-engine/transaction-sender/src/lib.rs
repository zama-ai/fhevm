mod nonce_managed_provider;
mod ops;
mod transaction_sender;

use std::sync::Arc;

use alloy::network::TxSigner;
use alloy::providers::Provider;
use alloy::providers::ProviderBuilder;
use alloy::providers::WsConnect;
use alloy::signers::Signature;
use alloy::signers::Signer;
use alloy::transports::http::reqwest::Url;
pub use nonce_managed_provider::FillersWithoutNonceManagement;
pub use nonce_managed_provider::NonceManagedProvider;
pub use transaction_sender::TransactionSender;

#[derive(Clone, Debug)]
pub struct ConfigSettings {
    pub database_url: String,
    pub database_pool_size: u32,

    pub verify_proof_resp_db_channel: String,
    pub add_ciphertexts_db_channel: String,
    pub allow_handle_db_channel: String,

    pub verify_proof_resp_batch_limit: u32,
    pub verify_proof_resp_max_retries: u32,
    pub verify_proof_remove_after_max_retries: bool,

    pub add_ciphertexts_batch_limit: u32,
    pub add_ciphertexts_max_retries: u32,

    pub allow_handle_batch_limit: u32,
    pub allow_handle_max_retries: u32,

    pub db_polling_interval_secs: u16,

    pub error_sleep_initial_secs: u16,
    pub error_sleep_max_secs: u16,

    pub txn_receipt_timeout_secs: u16,

    pub required_txn_confirmations: u16,

    pub review_after_unlimited_retries: u16,
}

impl Default for ConfigSettings {
    fn default() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or("postgres://postgres:postgres@localhost/coprocessor".to_owned()),
            database_pool_size: 10,
            verify_proof_resp_db_channel: "verify_proof_responses".to_owned(),
            add_ciphertexts_db_channel: "add_ciphertexts".to_owned(),
            allow_handle_db_channel: "event_allowed_handle".to_owned(),
            verify_proof_resp_batch_limit: 128,
            verify_proof_resp_max_retries: 3,
            verify_proof_remove_after_max_retries: true,
            db_polling_interval_secs: 5,
            error_sleep_initial_secs: 1,
            error_sleep_max_secs: 16,
            add_ciphertexts_batch_limit: 10,
            add_ciphertexts_max_retries: 15,
            allow_handle_batch_limit: 10,
            allow_handle_max_retries: 10,
            txn_receipt_timeout_secs: 10,
            required_txn_confirmations: 0,
            review_after_unlimited_retries: 30,
        }
    }
}

pub const REVIEW: &str = "review";

// A signer that can both sign transactions and messages. Only needed for `AbstractSigner` (see below).
pub trait CombinedSigner: TxSigner<Signature> + Signer<Signature> {}
impl<T: TxSigner<Signature> + Signer<Signature>> CombinedSigner for T {}

// A thread-safe abstract signer that can sign both transactions and messages.
pub type AbstractSigner = Arc<dyn CombinedSigner + Send + Sync>;

pub fn make_abstract_signer<S>(signer: S) -> AbstractSigner
where
    S: CombinedSigner + Send + Sync + 'static,
{
    Arc::new(signer)
}

pub async fn get_chain_id(ws_url: Url) -> anyhow::Result<u64> {
    let provider = ProviderBuilder::new().on_ws(WsConnect::new(ws_url)).await?;
    Ok(provider.get_chain_id().await?)
}
