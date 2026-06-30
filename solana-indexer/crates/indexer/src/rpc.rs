//! Thin RPC accessor for the on-chain cross-check (`build_verified_proof`) and
//! the partial-backfill state synthesis.
//!
//! Fetches the live PDA account at the configured commitment and hands back its
//! raw data for `zama_solana_acl::decode_account`. All failures are surfaced as
//! `None`/`Err` so callers can fall back to the unverified path with a warning.

use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_pubkey::Pubkey;
use std::sync::Arc;

#[derive(Clone)]
pub struct SolanaRpc {
    client: Arc<RpcClient>,
}

impl SolanaRpc {
    pub fn new(rpc_url: String, commitment: CommitmentConfig) -> Self {
        Self {
            client: Arc::new(RpcClient::new_with_commitment(rpc_url, commitment)),
        }
    }

    /// Fetches the raw account data at `pda`, or `None` if it does not exist or
    /// the RPC call fails. The error is intentionally swallowed (logged by the
    /// caller) so verification degrades gracefully to the unverified path.
    pub async fn account_data(&self, pda: [u8; 32]) -> Option<Vec<u8>> {
        let key = Pubkey::new_from_array(pda);
        match self.client.get_account(&key).await {
            Ok(account) => Some(account.data),
            Err(_) => None,
        }
    }
}
