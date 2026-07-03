//! Configuration for the Solana ACL MMR proof service, wired into the
//! relayer's existing `config`-crate-based `Settings` (see
//! `crate::config::settings::Settings::solana_proof`). Optional and
//! `#[serde(default)]`-absent so existing EVM-only deployments' config files
//! need no changes.

use serde::Deserialize;

fn default_poll_interval_secs() -> u64 {
    5
}

fn default_poll_signature_limit() -> usize {
    1000
}

#[derive(Debug, Deserialize, Clone)]
pub struct SolanaProofConfig {
    /// Solana JSON-RPC HTTP endpoint used for both ingestion and live account reads.
    pub rpc_url: String,
    /// zama-host program id, base58.
    pub program_id: String,
    /// Signature to start ingestion from (exclusive), base58. `None` starts
    /// from the oldest signature `getSignaturesForAddress` returns.
    #[serde(default)]
    pub start_signature: Option<String>,
    /// Poll loop interval.
    #[serde(default = "default_poll_interval_secs")]
    pub poll_interval_secs: u64,
    /// Max signatures fetched per poll cycle.
    #[serde(default = "default_poll_signature_limit")]
    pub poll_signature_limit: usize,
    /// Path to the file-backed `LeafStore`'s JSON file.
    pub leaf_store_path: String,
}

#[derive(thiserror::Error, Debug)]
pub enum SolanaProofConfigError {
    #[error("invalid Solana program_id: {0}")]
    InvalidProgramId(String),
}

impl SolanaProofConfig {
    pub fn program_id_bytes(&self) -> Result<[u8; 32], SolanaProofConfigError> {
        crate::http::utils::decode_solana_address(&self.program_id)
            .map_err(|e| SolanaProofConfigError::InvalidProgramId(e.to_string()))
    }
}
