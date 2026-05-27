use crate::core::{
    solana_acl::SolanaPubkeyBytes,
    solana_live::{
        SOLANA_NATIVE_COMMITMENT_CONFIRMED, SOLANA_NATIVE_COMMITMENT_FINALIZED,
        SolanaNativeAccountFetchError, SolanaNativeAccountFetcher, SolanaNativeAccountSnapshotV0,
    },
    solana_request::SolanaNativeAccountWitnessV0,
};
use base64::{Engine, engine::general_purpose::STANDARD as BASE64_STANDARD};
use serde::{Deserialize, Serialize};
use solana_pubkey::Pubkey;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct SolanaJsonRpcAccountFetcher {
    client: reqwest::Client,
    rpc_url: String,
}

impl SolanaJsonRpcAccountFetcher {
    pub fn new(rpc_url: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            rpc_url: rpc_url.into(),
        }
    }

    pub fn with_client(rpc_url: impl Into<String>, client: reqwest::Client) -> Self {
        Self {
            client,
            rpc_url: rpc_url.into(),
        }
    }
}

impl SolanaNativeAccountFetcher for SolanaJsonRpcAccountFetcher {
    fn fetch_accounts(
        &self,
        account_keys: &[SolanaPubkeyBytes],
        commitment_level: u8,
    ) -> impl std::future::Future<
        Output = Result<SolanaNativeAccountSnapshotV0, SolanaNativeAccountFetchError>,
    > + Send {
        let client = self.client.clone();
        let rpc_url = self.rpc_url.clone();
        let account_keys = account_keys.to_vec();
        async move {
            let request = get_multiple_accounts_request(&account_keys, commitment_level)?;
            let response_text = client
                .post(rpc_url)
                .json(&request)
                .send()
                .await
                .map_err(|e| SolanaNativeAccountFetchError::Unavailable(e.to_string()))?
                .error_for_status()
                .map_err(|e| SolanaNativeAccountFetchError::Unavailable(e.to_string()))?
                .text()
                .await
                .map_err(|e| SolanaNativeAccountFetchError::Unavailable(e.to_string()))?;
            decode_get_multiple_accounts_response(
                &account_keys,
                commitment_level,
                response_text.as_bytes(),
            )
        }
    }
}

pub fn decode_get_multiple_accounts_response(
    account_keys: &[SolanaPubkeyBytes],
    commitment_level: u8,
    response_bytes: &[u8],
) -> Result<SolanaNativeAccountSnapshotV0, SolanaNativeAccountFetchError> {
    let response: JsonRpcResponse<GetMultipleAccountsResult> =
        serde_json::from_slice(response_bytes)
            .map_err(|e| SolanaNativeAccountFetchError::Unavailable(e.to_string()))?;
    if let Some(error) = response.error {
        return Err(SolanaNativeAccountFetchError::Unavailable(format!(
            "Solana RPC error {}: {}",
            error.code, error.message
        )));
    }
    let result = response.result.ok_or_else(|| {
        SolanaNativeAccountFetchError::Unavailable("missing Solana RPC result".to_string())
    })?;
    if result.value.len() != account_keys.len() {
        return Err(SolanaNativeAccountFetchError::Unavailable(format!(
            "Solana RPC returned {} accounts for {} requested keys",
            result.value.len(),
            account_keys.len()
        )));
    }

    let mut accounts = Vec::with_capacity(result.value.len());
    for (account_key, account) in account_keys.iter().zip(result.value) {
        let account = account.ok_or_else(|| {
            SolanaNativeAccountFetchError::Unavailable(format!(
                "Solana account {} was not found",
                Pubkey::new_from_array(*account_key)
            ))
        })?;
        let owner = Pubkey::from_str(&account.owner)
            .map_err(|e| SolanaNativeAccountFetchError::Unavailable(e.to_string()))?
            .to_bytes();
        let data = decode_account_data(&account.data)?;
        accounts.push(SolanaNativeAccountWitnessV0 {
            account_key: *account_key,
            owner,
            executable: account.executable,
            data,
        });
    }

    Ok(SolanaNativeAccountSnapshotV0 {
        observed_slot: result.context.slot,
        commitment_level,
        rpc_response_bytes: response_bytes.len(),
        accounts,
    })
}

fn get_multiple_accounts_request(
    account_keys: &[SolanaPubkeyBytes],
    commitment_level: u8,
) -> Result<GetMultipleAccountsRequest, SolanaNativeAccountFetchError> {
    let commitment = solana_rpc_commitment(commitment_level)?;
    Ok(GetMultipleAccountsRequest {
        jsonrpc: "2.0",
        id: 1,
        method: "getMultipleAccounts",
        params: (
            account_keys
                .iter()
                .map(|key| Pubkey::new_from_array(*key).to_string())
                .collect(),
            GetMultipleAccountsConfig {
                encoding: "base64",
                commitment,
            },
        ),
    })
}

fn solana_rpc_commitment(
    commitment_level: u8,
) -> Result<&'static str, SolanaNativeAccountFetchError> {
    match commitment_level {
        SOLANA_NATIVE_COMMITMENT_CONFIRMED => Ok("confirmed"),
        SOLANA_NATIVE_COMMITMENT_FINALIZED => Ok("finalized"),
        _ => Err(SolanaNativeAccountFetchError::Unavailable(format!(
            "unsupported Solana commitment level {commitment_level}"
        ))),
    }
}

fn decode_account_data(data: &AccountData) -> Result<Vec<u8>, SolanaNativeAccountFetchError> {
    if data.0.1 != "base64" {
        return Err(SolanaNativeAccountFetchError::Unavailable(format!(
            "unsupported Solana account encoding {}",
            data.0.1
        )));
    }
    BASE64_STANDARD
        .decode(&data.0.0)
        .map_err(|e| SolanaNativeAccountFetchError::Unavailable(e.to_string()))
}

#[derive(Debug, Serialize)]
struct GetMultipleAccountsRequest {
    jsonrpc: &'static str,
    id: u64,
    method: &'static str,
    params: (Vec<String>, GetMultipleAccountsConfig),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct GetMultipleAccountsConfig {
    encoding: &'static str,
    commitment: &'static str,
}

#[derive(Debug, Deserialize)]
struct JsonRpcResponse<T> {
    result: Option<T>,
    error: Option<JsonRpcError>,
}

#[derive(Debug, Deserialize)]
struct JsonRpcError {
    code: i64,
    message: String,
}

#[derive(Debug, Deserialize)]
struct GetMultipleAccountsResult {
    context: RpcContext,
    value: Vec<Option<RpcAccount>>,
}

#[derive(Debug, Deserialize)]
struct RpcContext {
    slot: u64,
}

#[derive(Debug, Deserialize)]
struct RpcAccount {
    owner: String,
    executable: bool,
    data: AccountData,
}

#[derive(Debug, Deserialize)]
struct AccountData((String, String));

#[cfg(test)]
mod tests {
    use super::*;
    use base64::{Engine, engine::general_purpose::STANDARD as BASE64_STANDARD};

    fn account_response(account_keys: &[SolanaPubkeyBytes], owner: SolanaPubkeyBytes) -> Vec<u8> {
        account_response_with_executable(account_keys, owner, false)
    }

    fn account_response_with_executable(
        account_keys: &[SolanaPubkeyBytes],
        owner: SolanaPubkeyBytes,
        executable: bool,
    ) -> Vec<u8> {
        let values = account_keys
            .iter()
            .enumerate()
            .map(|(index, _)| {
                format!(
                    r#"{{
                        "lamports": 1,
                        "owner": "{}",
                        "data": ["{}", "base64"],
                        "executable": {executable},
                        "rentEpoch": 0
                    }}"#,
                    Pubkey::new_from_array(owner),
                    BASE64_STANDARD.encode([index as u8, 2, 3])
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        format!(
            r#"{{
                "jsonrpc": "2.0",
                "result": {{
                    "context": {{ "slot": 77 }},
                    "value": [{}]
                }},
                "id": 1
            }}"#,
            values
        )
        .into_bytes()
    }

    #[test]
    fn decodes_get_multiple_accounts_response_into_snapshot() {
        let account_keys = vec![[1; 32], [2; 32]];
        let owner = [42; 32];
        let snapshot = decode_get_multiple_accounts_response(
            &account_keys,
            SOLANA_NATIVE_COMMITMENT_CONFIRMED,
            &account_response(&account_keys, owner),
        )
        .unwrap();

        assert_eq!(snapshot.observed_slot, 77);
        assert_eq!(
            snapshot.commitment_level,
            SOLANA_NATIVE_COMMITMENT_CONFIRMED
        );
        assert_eq!(snapshot.accounts.len(), 2);
        assert_eq!(snapshot.accounts[0].account_key, account_keys[0]);
        assert_eq!(snapshot.accounts[0].owner, owner);
        assert!(!snapshot.accounts[0].executable);
        assert_eq!(snapshot.accounts[0].data, vec![0, 2, 3]);
        assert_eq!(snapshot.accounts[1].data, vec![1, 2, 3]);
    }

    #[test]
    fn decodes_executable_account_flag() {
        let account_keys = vec![[1; 32]];
        let owner = [42; 32];
        let snapshot = decode_get_multiple_accounts_response(
            &account_keys,
            SOLANA_NATIVE_COMMITMENT_CONFIRMED,
            &account_response_with_executable(&account_keys, owner, true),
        )
        .unwrap();

        assert!(snapshot.accounts[0].executable);
    }

    #[test]
    fn rejects_missing_account_in_rpc_response() {
        let account_keys = vec![[1; 32]];
        let response = br#"{
            "jsonrpc": "2.0",
            "result": {
                "context": { "slot": 77 },
                "value": [null]
            },
            "id": 1
        }"#;

        assert!(matches!(
            decode_get_multiple_accounts_response(
                &account_keys,
                SOLANA_NATIVE_COMMITMENT_CONFIRMED,
                response,
            ),
            Err(SolanaNativeAccountFetchError::Unavailable(message))
                if message.contains("was not found")
        ));
    }

    #[test]
    fn rejects_unsupported_account_encoding() {
        let account_keys = vec![[1; 32]];
        let owner = Pubkey::new_from_array([42; 32]);
        let response = format!(
            r#"{{
                "jsonrpc": "2.0",
                "result": {{
                    "context": {{ "slot": 77 }},
                    "value": [{{
                        "lamports": 1,
                        "owner": "{owner}",
                        "data": ["abc", "base58"],
                        "executable": false,
                        "rentEpoch": 0
                    }}]
                }},
                "id": 1
            }}"#
        );

        assert!(matches!(
            decode_get_multiple_accounts_response(
                &account_keys,
                SOLANA_NATIVE_COMMITMENT_CONFIRMED,
                response.as_bytes(),
            ),
            Err(SolanaNativeAccountFetchError::Unavailable(message))
                if message.contains("unsupported Solana account encoding")
        ));
    }

    #[test]
    fn rejects_rpc_error_response() {
        let response = br#"{
            "jsonrpc": "2.0",
            "error": { "code": -32602, "message": "invalid params" },
            "id": 1
        }"#;

        assert!(matches!(
            decode_get_multiple_accounts_response(&[], SOLANA_NATIVE_COMMITMENT_CONFIRMED, response),
            Err(SolanaNativeAccountFetchError::Unavailable(message))
                if message.contains("invalid params")
        ));
    }
}
