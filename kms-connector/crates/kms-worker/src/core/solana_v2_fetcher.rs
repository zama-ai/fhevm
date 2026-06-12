//! Minimal Solana account fetcher for the V2 user-decryption ACL check.
//!
//! Reads a single account (a ZamaHost `AclRecord`) from the configured Solana host-chain RPC at
//! **`finalized`** commitment, via the JSON-RPC `getAccountInfo` method. It is intentionally tiny:
//! the connector only needs to read account bytes + owner; it does not need the full Solana SDK
//! RPC client, which is not in the connector's dependency set.
//!
//! Security-relevant choices:
//! - commitment is pinned to `finalized` so the connector never authorizes against a slot that can
//!   still be rolled back;
//! - the caller verifies `owner == ZamaHost program id` and re-derives the canonical ACL-record
//!   PDA before trusting the bytes (see [`crate::core::event_processor::decryption`]).

use crate::core::solana_acl::SolanaPubkeyBytes;
use anyhow::{Context, anyhow, bail};
use base64::{Engine, engine::general_purpose::STANDARD as BASE64_STANDARD};
use serde_json::Value;
use solana_pubkey::Pubkey;
use url::Url;

/// The Solana commitment level used for every ACL read. Authorizing decryption against anything
/// weaker than `finalized` would let a rolled-back slot grant access.
pub const SOLANA_COMMITMENT_FINALIZED: &str = "finalized";

/// Length of the Anchor account discriminator that prefixes every ZamaHost account; the
/// `AclRecord.handle` field begins immediately after it.
const ANCHOR_DISCRIMINATOR_LEN: usize = 8;

/// A fetched Solana account: its owner program and raw data bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaAccount {
    pub owner: SolanaPubkeyBytes,
    pub data: Vec<u8>,
}

/// Fetches account data via Solana JSON-RPC `getAccountInfo` at `finalized` commitment.
#[derive(Clone, Debug)]
pub struct SolanaV2Fetcher {
    url: Url,
    client: reqwest::Client,
}

impl SolanaV2Fetcher {
    pub fn new(url: Url, client: reqwest::Client) -> Self {
        Self { url, client }
    }

    /// Builds the JSON-RPC `getAccountInfo` request body for `account`, pinned to `finalized`
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
                    "commitment": SOLANA_COMMITMENT_FINALIZED,
                }
            ],
        })
    }

    /// Fetches `account` at `finalized` commitment. Returns `Ok(None)` when the account does not
    /// exist, `Err` on transport / decoding failures.
    pub async fn get_account(
        &self,
        account: &SolanaPubkeyBytes,
    ) -> anyhow::Result<Option<SolanaAccount>> {
        let body = Self::account_info_request_body(account);
        let response = self.post(&body).await?;
        parse_account_info_response(&response)
    }

    /// Builds the JSON-RPC `getProgramAccounts` body that finds ACL-record accounts owned by
    /// `program_id` whose stored handle (the first field after the 8-byte Anchor discriminator)
    /// equals `handle`. Pinned to `finalized` and base64; the `dataSlice` keeps the response small
    /// while we only need the account keys (data is re-fetched/decoded by `getAccountInfo`). Split
    /// out for unit-testing the `finalized` + `memcmp` shape.
    pub fn program_accounts_by_handle_body(
        program_id: &SolanaPubkeyBytes,
        handle: &[u8; 32],
    ) -> Value {
        serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getProgramAccounts",
            "params": [
                Pubkey::new_from_array(*program_id).to_string(),
                {
                    "encoding": "base64",
                    "commitment": SOLANA_COMMITMENT_FINALIZED,
                    "dataSlice": { "offset": 0, "length": 0 },
                    "filters": [
                        {
                            "memcmp": {
                                "offset": ANCHOR_DISCRIMINATOR_LEN,
                                // A 32-byte handle base58-encoded; `Pubkey` is a 32-byte newtype
                                // whose `Display` is exactly the base58 of its bytes.
                                "bytes": Pubkey::new_from_array(*handle).to_string(),
                                "encoding": "base58",
                            }
                        }
                    ],
                }
            ],
        })
    }

    /// Returns the account keys of every ACL record owned by `program_id` whose handle matches.
    /// The `seed` is reserved for callers that want to assert it; the on-chain filter is by handle
    /// offset since the PDA seeds (nonce metadata) are not known from the handle alone.
    pub async fn find_acl_records_by_handle(
        &self,
        program_id: &SolanaPubkeyBytes,
        _seed: &[u8],
        handle: &[u8; 32],
    ) -> anyhow::Result<Vec<SolanaPubkeyBytes>> {
        let body = Self::program_accounts_by_handle_body(program_id, handle);
        let response = self.post(&body).await?;
        parse_program_accounts_pubkeys(&response)
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

/// Parses a Solana JSON-RPC `getProgramAccounts` response into the matching account pubkeys. Pure
/// so the multi-match handling is unit-testable.
pub fn parse_program_accounts_pubkeys(body: &str) -> anyhow::Result<Vec<SolanaPubkeyBytes>> {
    let json: Value =
        serde_json::from_str(body).context("solana RPC response is not valid JSON")?;
    if let Some(error) = json.get("error") {
        bail!("solana RPC returned an error: {error}");
    }
    let result = json
        .get("result")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("solana getProgramAccounts response missing result array"))?;

    result
        .iter()
        .map(|entry| {
            let pubkey_str = entry
                .get("pubkey")
                .and_then(Value::as_str)
                .ok_or_else(|| anyhow!("solana program account entry missing pubkey"))?;
            Ok(Pubkey::try_from(pubkey_str)
                .map_err(|e| anyhow!("invalid solana account pubkey '{pubkey_str}': {e}"))?
                .to_bytes())
        })
        .collect()
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
    fn request_pins_finalized_commitment() {
        let account = [3u8; 32];
        let body = SolanaV2Fetcher::account_info_request_body(&account);

        assert_eq!(body["method"], "getAccountInfo");
        let params = &body["params"];
        assert_eq!(
            params[0].as_str().unwrap(),
            Pubkey::new_from_array(account).to_string()
        );
        assert_eq!(params[1]["commitment"], SOLANA_COMMITMENT_FINALIZED);
        assert_eq!(params[1]["commitment"], "finalized");
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
    fn program_accounts_request_pins_finalized_and_handle_memcmp() {
        let program_id = [9u8; 32];
        let handle = [4u8; 32];
        let body = SolanaV2Fetcher::program_accounts_by_handle_body(&program_id, &handle);

        assert_eq!(body["method"], "getProgramAccounts");
        let params = &body["params"];
        assert_eq!(params[1]["commitment"], "finalized");
        let memcmp = &params[1]["filters"][0]["memcmp"];
        assert_eq!(memcmp["offset"], ANCHOR_DISCRIMINATOR_LEN);
        assert_eq!(
            memcmp["bytes"].as_str().unwrap(),
            Pubkey::new_from_array(handle).to_string()
        );
    }

    #[test]
    fn parses_program_accounts_pubkeys() {
        let a = Pubkey::new_from_array([1u8; 32]);
        let b = Pubkey::new_from_array([2u8; 32]);
        let body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": [
                { "pubkey": a.to_string(), "account": { "data": ["", "base64"] } },
                { "pubkey": b.to_string(), "account": { "data": ["", "base64"] } },
            ]
        })
        .to_string();
        let keys = parse_program_accounts_pubkeys(&body).unwrap();
        assert_eq!(keys, vec![a.to_bytes(), b.to_bytes()]);
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
