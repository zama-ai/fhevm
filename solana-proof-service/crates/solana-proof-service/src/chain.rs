//! Confirmed JSON-RPC chain access for on-chain `EncryptedValue` peak checks.
//!
//! Deliberately thin: proof serving only needs `getAccountInfo`. Signature /
//! transaction catch-up belongs to the future bounded recovery adapter, not the
//! read-only proof path.
//!
//! Account payloads travel as Base64 (Base58 is for addresses only). The RPC
//! response must be `[payload, "base64"]`, the account owner must match the
//! configured host program, and `decode_on_chain_account` enforces discriminator
//! + Borsh layout before peaks are trusted.

use async_trait::async_trait;
use base64::Engine;
use serde_json::json;

const SOLANA_PROOF_COMMITMENT: &str = "confirmed";
const EXPECTED_DATA_ENCODING: &str = "base64";

#[derive(thiserror::Error, Debug)]
pub enum ChainError {
    #[error("RPC transport error: {0}")]
    Transport(String),
    #[error("RPC returned an error: {0}")]
    Rpc(String),
    #[error("malformed encoding: {0}")]
    Encoding(String),
    #[error("account owner mismatch: expected {expected}, got {observed}")]
    WrongOwner { expected: String, observed: String },
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
    /// Host program id; returned accounts must be owned by this program.
    program_id: [u8; 32],
}

impl RpcChainFetcher {
    /// Confirmed JSON-RPC client with explicit connect/request timeouts so a
    /// stalled RPC cannot retain proof handlers indefinitely.
    pub fn new(rpc_url: String, program_id: [u8; 32]) -> Self {
        let client = reqwest::Client::builder()
            // Stay inside the outer HTTP proof budget (30s) so timeouts surface as
            // typed `chain_error` rather than racing an empty HTTP 408.
            .connect_timeout(std::time::Duration::from_secs(3))
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("reqwest client");
        Self {
            client,
            rpc_url,
            program_id,
        }
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
    base64::engine::general_purpose::STANDARD
        .decode(input)
        .map_err(|err| err.to_string())
}

/// Fail-closed parse of `getAccountInfo` `result.value`.
///
/// Requires `data == [payload, "base64"]`, matching owner, valid Base64, and a
/// well-formed EncryptedValue account body (discriminator + Borsh).
fn parse_account_value(
    value: &serde_json::Value,
    expected_program_id: &[u8; 32],
) -> Result<OnChainLineageState, ChainError> {
    let owner = value
        .get("owner")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ChainError::Rpc("missing account owner".to_string()))?;
    let expected = base58_encode(expected_program_id);
    if owner != expected {
        return Err(ChainError::WrongOwner {
            expected,
            observed: owner.to_owned(),
        });
    }

    let data = value
        .get("data")
        .ok_or_else(|| ChainError::Rpc("missing account data".to_string()))?;
    let data_arr = data
        .as_array()
        .ok_or_else(|| ChainError::Encoding("account data must be [payload, encoding]".into()))?;
    if data_arr.len() != 2 {
        return Err(ChainError::Encoding(
            "account data must be exactly [payload, encoding]".into(),
        ));
    }
    let payload = data_arr[0]
        .as_str()
        .ok_or_else(|| ChainError::Encoding("account data payload must be a string".into()))?;
    let encoding = data_arr[1]
        .as_str()
        .ok_or_else(|| ChainError::Encoding("account data encoding tag must be a string".into()))?;
    if encoding != EXPECTED_DATA_ENCODING {
        return Err(ChainError::Encoding(format!(
            "unexpected account encoding `{encoding}`; expected `{EXPECTED_DATA_ENCODING}`"
        )));
    }

    let raw = base64_decode(payload).map_err(ChainError::Encoding)?;
    // Length/discriminator validation lives in decode_on_chain_account.
    let decoded = zama_solana_acl::decode_on_chain_account(&raw)
        .map_err(|e| ChainError::Encoding(format!("account body rejected: {e:?}")))?;
    Ok(OnChainLineageState {
        peaks: decoded.peaks,
        leaf_count: decoded.leaf_count,
    })
}

/// Interprets `getAccountInfo` `result`: only explicit `"value": null` means absent.
fn lineage_from_rpc_result(
    result: &serde_json::Value,
    expected_program_id: &[u8; 32],
) -> Result<Option<OnChainLineageState>, ChainError> {
    if result.is_null() {
        return Err(ChainError::Rpc("null getAccountInfo result".to_string()));
    }
    match result.get("value") {
        None => Err(ChainError::Rpc(
            "malformed getAccountInfo result: missing value field".to_string(),
        )),
        Some(value) if value.is_null() => Ok(None),
        Some(value) => Ok(Some(parse_account_value(value, expected_program_id)?)),
    }
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
        lineage_from_rpc_result(&result, &self.program_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::Engine;
    use zama_solana_acl::encrypted_value_discriminator;

    fn program_id() -> [u8; 32] {
        [7u8; 32]
    }

    fn minimal_account_body() -> Vec<u8> {
        // Discriminator + empty-ish body is rejected by Borsh; build a tiny
        // well-formed account via encode path isn't public, so use a body that
        // passes discriminator then fails Borsh — for success tests we need a
        // real EncryptedValue. Prefer fixture: discriminator + borsh of zeros
        // won't work. Use decode tests only for fail-closed paths with a
        // hand-crafted successful account from acl tests pattern.
        let disc = encrypted_value_discriminator();
        // OnChainEncryptedValue: 4x32 + vec subjects + u64 + vec peaks + u8
        // Empty subjects/peaks: subjects len 0 (4 bytes LE), peaks len 0 (4), bump
        let mut body = Vec::new();
        body.extend_from_slice(&disc);
        body.extend_from_slice(&[0u8; 32]); // acl_domain_key
        body.extend_from_slice(&[0u8; 32]); // app_account
        body.extend_from_slice(&[0u8; 32]); // label
        body.extend_from_slice(&[0u8; 32]); // handle
        body.extend_from_slice(&0u32.to_le_bytes()); // subjects len
        body.extend_from_slice(&0u64.to_le_bytes()); // leaf_count
        body.extend_from_slice(&0u32.to_le_bytes()); // peaks len
        body.push(0); // bump
        body
    }

    fn account_json(owner: &str, payload_b64: &str, encoding: &str) -> serde_json::Value {
        json!({
            "owner": owner,
            "data": [payload_b64, encoding],
            "lamports": 1,
        })
    }

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

    #[test]
    fn base64_decode_rejects_malformed_input() {
        assert!(base64_decode("a").is_err());
        assert!(base64_decode("====").is_err());
        assert!(base64_decode("aGVsbG8").is_err()); // missing padding
    }

    #[test]
    fn parse_rejects_unexpected_encoding_tag() {
        let owner = base58_encode(&program_id());
        let payload = base64::engine::general_purpose::STANDARD.encode(minimal_account_body());
        let value = account_json(&owner, &payload, "base58");
        let err = parse_account_value(&value, &program_id()).unwrap_err();
        assert!(matches!(err, ChainError::Encoding(_)));
        assert!(err.to_string().contains("unexpected account encoding"));
    }

    #[test]
    fn parse_rejects_malformed_base64_payload() {
        let owner = base58_encode(&program_id());
        let value = account_json(&owner, "not-base64!!!", "base64");
        let err = parse_account_value(&value, &program_id()).unwrap_err();
        assert!(matches!(err, ChainError::Encoding(_)));
    }

    #[test]
    fn parse_rejects_wrong_owner() {
        let payload = base64::engine::general_purpose::STANDARD.encode(minimal_account_body());
        let value = account_json(
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
            &payload,
            "base64",
        );
        let err = parse_account_value(&value, &program_id()).unwrap_err();
        assert!(matches!(err, ChainError::WrongOwner { .. }));
    }

    #[test]
    fn parse_rejects_non_array_data_shape() {
        let owner = base58_encode(&program_id());
        let value = json!({
            "owner": owner,
            "data": "aGVsbG8=",
        });
        let err = parse_account_value(&value, &program_id()).unwrap_err();
        assert!(matches!(err, ChainError::Encoding(_)));
    }

    #[test]
    fn parse_accepts_valid_account() {
        let owner = base58_encode(&program_id());
        let payload = base64::engine::general_purpose::STANDARD.encode(minimal_account_body());
        let value = account_json(&owner, &payload, "base64");
        let state = parse_account_value(&value, &program_id()).unwrap();
        assert_eq!(state.leaf_count, 0);
        assert!(state.peaks.is_empty());
    }

    #[test]
    fn parse_rejects_bad_discriminator() {
        let owner = base58_encode(&program_id());
        let mut raw = minimal_account_body();
        raw[0] ^= 0xff;
        let payload = base64::engine::general_purpose::STANDARD.encode(raw);
        let value = account_json(&owner, &payload, "base64");
        let err = parse_account_value(&value, &program_id()).unwrap_err();
        assert!(matches!(err, ChainError::Encoding(_)));
    }

    #[test]
    fn null_value_means_account_absent() {
        let result = json!({ "value": null, "context": { "slot": 1 } });
        assert!(lineage_from_rpc_result(&result, &program_id())
            .unwrap()
            .is_none());
    }

    #[test]
    fn missing_value_field_is_malformed_rpc() {
        let result = json!({ "context": { "slot": 1 } });
        let err = lineage_from_rpc_result(&result, &program_id()).unwrap_err();
        assert!(matches!(err, ChainError::Rpc(_)));
        assert!(err.to_string().contains("missing value field"));
    }

    #[test]
    fn null_result_is_malformed_rpc() {
        let err = lineage_from_rpc_result(&serde_json::Value::Null, &program_id()).unwrap_err();
        assert!(matches!(err, ChainError::Rpc(_)));
        assert!(err.to_string().contains("null getAccountInfo result"));
    }
}
