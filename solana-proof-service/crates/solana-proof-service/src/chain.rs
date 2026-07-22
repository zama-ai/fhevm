//! Confirmed JSON-RPC chain access for on-chain `EncryptedValue` peak checks.
//!
//! Deliberately thin: proof serving only needs `getAccountInfo`. Signature /
//! transaction catch-up belongs to the future bounded recovery adapter, not the
//! read-only proof path.

use async_trait::async_trait;
use serde_json::json;

const SOLANA_PROOF_COMMITMENT: &str = "confirmed";

#[derive(thiserror::Error, Debug)]
pub enum ChainError {
    #[error("RPC transport error: {0}")]
    Transport(String),
    #[error("RPC returned an error: {0}")]
    Rpc(String),
    #[error("malformed encoding: {0}")]
    Encoding(String),
}

/// The on-chain `EncryptedValue` state needed to verify a reconstructed proof.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OnChainLineageState {
    pub peaks: Vec<[u8; 32]>,
    pub leaf_count: u64,
}

#[async_trait]
pub trait ChainFetcher: Send + Sync {
    /// Fetches and decodes the live `EncryptedValue` account at confirmed commitment.
    async fn get_lineage_state(
        &self,
        address: [u8; 32],
    ) -> Result<Option<OnChainLineageState>, ChainError>;
}

pub struct RpcChainFetcher {
    client: reqwest::Client,
    rpc_url: String,
}

impl RpcChainFetcher {
    /// Confirmed JSON-RPC client with explicit connect/request timeouts so a
    /// stalled RPC cannot retain proof handlers indefinitely.
    pub fn new(rpc_url: String) -> Self {
        let client = reqwest::Client::builder()
            .connect_timeout(std::time::Duration::from_secs(5))
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("reqwest client");
        Self { client, rpc_url }
    }

    async fn call(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, ChainError> {
        let body = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": params,
        });
        let response = self
            .client
            .post(&self.rpc_url)
            .json(&body)
            .send()
            .await
            .map_err(|e| ChainError::Transport(e.to_string()))?;
        let value: serde_json::Value = response
            .json()
            .await
            .map_err(|e| ChainError::Transport(e.to_string()))?;
        if let Some(error) = value.get("error") {
            return Err(ChainError::Rpc(error.to_string()));
        }
        value
            .get("result")
            .cloned()
            .ok_or_else(|| ChainError::Rpc("missing result field".to_string()))
    }
}

fn base58_encode(bytes: &[u8; 32]) -> String {
    bs58::encode(bytes).into_string()
}

fn account_info_params(address: &[u8; 32]) -> serde_json::Value {
    json!([
        base58_encode(address),
        {"encoding": "base64", "commitment": SOLANA_PROOF_COMMITMENT}
    ])
}

fn base64_decode(input: &str) -> Result<Vec<u8>, String> {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut reverse = [255u8; 256];
    for (i, &c) in TABLE.iter().enumerate() {
        reverse[c as usize] = i as u8;
    }
    let clean: Vec<u8> = input.bytes().filter(|&b| b != b'=').collect();
    let mut out = Vec::with_capacity(clean.len() * 3 / 4);
    for chunk in clean.chunks(4) {
        let mut buf = [0u8; 4];
        for (i, &b) in chunk.iter().enumerate() {
            let v = reverse[b as usize];
            if v == 255 {
                return Err(format!("invalid base64 byte {b}"));
            }
            buf[i] = v;
        }
        let n = chunk.len();
        let combined = ((buf[0] as u32) << 18)
            | ((buf[1] as u32) << 12)
            | ((buf[2] as u32) << 6)
            | (buf[3] as u32);
        out.push((combined >> 16) as u8);
        if n > 2 {
            out.push((combined >> 8) as u8);
        }
        if n > 3 {
            out.push(combined as u8);
        }
    }
    Ok(out)
}

#[async_trait]
impl ChainFetcher for RpcChainFetcher {
    async fn get_lineage_state(
        &self,
        address: [u8; 32],
    ) -> Result<Option<OnChainLineageState>, ChainError> {
        let result = self
            .call("getAccountInfo", account_info_params(&address))
            .await?;
        if result.is_null() || result.get("value").map(|v| v.is_null()).unwrap_or(true) {
            return Ok(None);
        }
        let data_field = result["value"]["data"][0]
            .as_str()
            .ok_or_else(|| ChainError::Rpc("missing base64 account data".to_string()))?;
        let raw = base64_decode(data_field).map_err(ChainError::Encoding)?;
        let decoded = zama_solana_acl::decode_on_chain_account(&raw)
            .map_err(|e| ChainError::Rpc(format!("{e:?}")))?;
        Ok(Some(OnChainLineageState {
            peaks: decoded.peaks,
            leaf_count: decoded.leaf_count,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account_info_params_pin_confirmed_commitment() {
        let address = [42u8; 32];
        let account = account_info_params(&address);
        assert_eq!(account[1]["commitment"], "confirmed");
        assert_eq!(account[1]["encoding"], "base64");
    }

    #[test]
    fn base64_decode_matches_known_vector() {
        assert_eq!(base64_decode("aGVsbG8=").unwrap(), b"hello".to_vec());
    }
}
