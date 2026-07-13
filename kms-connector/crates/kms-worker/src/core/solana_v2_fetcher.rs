//! Minimal Solana account fetcher for the V2 user-decryption ACL check.
//!
//! Reads a single ZamaHost `EncryptedValue` account from the configured Solana host-chain RPC at
//! **`confirmed`** commitment, via the JSON-RPC `getAccountInfo` method. It is intentionally tiny:
//! the connector only needs to read account bytes + owner; it does not need the full Solana SDK
//! RPC client, which is not in the connector's dependency set.
//!
//! Security-relevant choices:
//! - commitment is pinned to `confirmed`: a valid grant observed on a supermajority-confirmed fork
//!   is sufficient authorization even if that fork is exceptionally rolled back later;
//! - the caller verifies `owner == ZamaHost program id` and re-derives the canonical EncryptedValue
//!   PDA before trusting the bytes (see [`crate::core::event_processor::decryption`]).

use crate::core::solana_acl::SolanaPubkeyBytes;
use anyhow::{Context, anyhow, bail};
use base64::{Engine, engine::general_purpose::STANDARD as BASE64_STANDARD};
use serde_json::Value;
use solana_pubkey::Pubkey;
use url::Url;

/// The Solana commitment level used for every ACL read.
pub const SOLANA_COMMITMENT_CONFIRMED: &str = "confirmed";

/// A fetched Solana account: its owner program and raw data bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaAccount {
    pub owner: SolanaPubkeyBytes,
    pub data: Vec<u8>,
}

/// Fetches account data via Solana JSON-RPC `getAccountInfo` at `confirmed` commitment.
#[derive(Clone, Debug)]
pub struct SolanaV2Fetcher {
    url: Url,
    client: reqwest::Client,
}

impl SolanaV2Fetcher {
    pub fn new(url: Url, client: reqwest::Client) -> Self {
        Self { url, client }
    }

    /// Builds the JSON-RPC `getAccountInfo` request body for `account`, pinned to `confirmed`
    /// commitment and base64 encoding. Split out so it can be asserted in unit tests without a
    /// live RPC.
    pub fn account_info_request_body(account: &SolanaPubkeyBytes) -> Value {
        serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getAccountInfo",
            "params": [
                Pubkey::new_from_array(*account).to_string(),
                {
                    "encoding": "base64",
                    "commitment": SOLANA_COMMITMENT_CONFIRMED,
                }
            ],
        })
    }

    /// Fetches `account` at `confirmed` commitment. Returns `Ok(None)` when the account does not
    /// exist, `Err` on transport / decoding failures.
    pub async fn get_account(
        &self,
        account: &SolanaPubkeyBytes,
    ) -> anyhow::Result<Option<SolanaAccount>> {
        let body = Self::account_info_request_body(account);
        let response = self.post(&body).await?;
        parse_account_info_response(&response)
    }

    async fn post(&self, body: &Value) -> anyhow::Result<String> {
        self.client
            .post(self.url.clone())
            .header("content-type", "application/json")
            .body(serde_json::to_vec(body)?)
            .send()
            .await
            .context("solana RPC request failed")?
            .error_for_status()
            .context("solana RPC returned an HTTP error")?
            .text()
            .await
            .context("failed to read solana RPC body")
    }
}

/// Parses a Solana JSON-RPC `getAccountInfo` response body. Pure so it is unit-testable against
/// canned RPC payloads.
pub fn parse_account_info_response(body: &str) -> anyhow::Result<Option<SolanaAccount>> {
    let json: Value =
        serde_json::from_str(body).context("solana RPC response is not valid JSON")?;

    if let Some(error) = json.get("error") {
        bail!("solana RPC returned an error: {error}");
    }

    let value = json
        .get("result")
        .and_then(|r| r.get("value"))
        .ok_or_else(|| anyhow!("solana RPC response missing result.value"))?;

    if value.is_null() {
        return Ok(None);
    }

    let owner_str = value
        .get("owner")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("solana account missing owner"))?;
    let owner = Pubkey::try_from(owner_str)
        .map_err(|e| anyhow!("invalid solana account owner '{owner_str}': {e}"))?
        .to_bytes();

    let data_field = value
        .get("data")
        .ok_or_else(|| anyhow!("solana account missing data"))?;
    let data = decode_account_data(data_field)?;

    Ok(Some(SolanaAccount { owner, data }))
}

/// Decodes the `data` field of a `getAccountInfo` value. We only request `base64`, so the field is
/// `[base64_string, "base64"]`.
fn decode_account_data(data_field: &Value) -> anyhow::Result<Vec<u8>> {
    let array = data_field.as_array().ok_or_else(|| {
        anyhow!("unexpected solana account data shape (expected [data, encoding])")
    })?;
    let encoded = array
        .first()
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("solana account data missing base64 payload"))?;
    let encoding = array.get(1).and_then(Value::as_str).unwrap_or_default();
    if encoding != "base64" {
        bail!("unexpected solana account data encoding: '{encoding}', expected base64");
    }
    BASE64_STANDARD
        .decode(encoded)
        .context("failed to base64-decode solana account data")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_pins_confirmed_commitment() {
        let account = [3u8; 32];
        let body = SolanaV2Fetcher::account_info_request_body(&account);

        assert_eq!(body["method"], "getAccountInfo");
        let params = &body["params"];
        assert_eq!(
            params[0].as_str().unwrap(),
            Pubkey::new_from_array(account).to_string()
        );
        assert_eq!(params[1]["commitment"], SOLANA_COMMITMENT_CONFIRMED);
        assert_eq!(params[1]["commitment"], "confirmed");
        assert_eq!(params[1]["encoding"], "base64");
    }

    #[test]
    fn parses_existing_account() {
        let owner = Pubkey::new_from_array([7u8; 32]);
        let data = vec![1u8, 2, 3, 4];
        let encoded = BASE64_STANDARD.encode(&data);
        let body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "context": { "slot": 123 },
                "value": {
                    "owner": owner.to_string(),
                    "data": [encoded, "base64"],
                    "lamports": 42,
                    "executable": false,
                    "rentEpoch": 0,
                }
            }
        })
        .to_string();

        let account = parse_account_info_response(&body).unwrap().unwrap();
        assert_eq!(account.owner, owner.to_bytes());
        assert_eq!(account.data, data);
    }

    #[test]
    fn parses_missing_account_as_none() {
        let body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": { "context": { "slot": 1 }, "value": null }
        })
        .to_string();
        assert_eq!(parse_account_info_response(&body).unwrap(), None);
    }

    #[test]
    fn surfaces_rpc_error() {
        let body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "error": { "code": -32602, "message": "Invalid params" }
        })
        .to_string();
        assert!(parse_account_info_response(&body).is_err());
    }

    #[test]
    fn rejects_non_base64_encoding() {
        let body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "value": {
                    "owner": Pubkey::new_from_array([1u8; 32]).to_string(),
                    "data": ["deadbeef", "base58"],
                }
            }
        })
        .to_string();
        assert!(parse_account_info_response(&body).is_err());
    }
}
