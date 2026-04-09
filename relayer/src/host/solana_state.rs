use anyhow::{anyhow, bail, Context, Result};
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use borsh::BorshDeserialize;
use reqwest::Client;
use serde_json::{json, Value};
use solana_host_contracts_core::HostProgramState;

const STATE_ACCOUNT_DISCRIMINATOR: [u8; 8] = *b"FHEHOST0";
const STATE_ACCOUNT_LAYOUT_VERSION: u32 = 1;

#[derive(Clone, Debug)]
pub struct SolanaStateClient {
    client: Client,
    rpc_url: String,
    state_pda: String,
}

impl SolanaStateClient {
    pub fn new(rpc_url: impl Into<String>, state_pda: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            rpc_url: rpc_url.into(),
            state_pda: state_pda.into(),
        }
    }

    pub async fn fetch_state(&self) -> Result<HostProgramState> {
        let result = self
            .rpc_call(
                "getAccountInfo",
                json!([
                    self.state_pda,
                    {
                        "encoding": "base64",
                        "commitment": "confirmed"
                    }
                ]),
            )
            .await?;

        let value = result
            .get("value")
            .ok_or_else(|| anyhow!("getAccountInfo missing value"))?;
        if value.is_null() {
            bail!(
                "state account {} not found on Solana host chain",
                self.state_pda
            );
        }

        let data_b64 = value
            .get("data")
            .and_then(Value::as_array)
            .and_then(|items| items.first())
            .and_then(Value::as_str)
            .context("getAccountInfo returned unsupported data format")?;

        let bytes = BASE64_STANDARD
            .decode(data_b64)
            .context("failed to decode Solana state account base64")?;
        decode_stored_state(&bytes)
    }

    async fn rpc_call(&self, method: &str, params: Value) -> Result<Value> {
        let response = self
            .client
            .post(&self.rpc_url)
            .json(&json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": method,
                "params": params,
            }))
            .send()
            .await
            .with_context(|| format!("rpc request failed for {method}"))?
            .error_for_status()
            .with_context(|| format!("rpc HTTP error for {method}"))?
            .json::<Value>()
            .await
            .with_context(|| format!("rpc JSON parse failed for {method}"))?;

        if let Some(error) = response.get("error") {
            bail!("rpc {method} returned error: {error}");
        }

        response
            .get("result")
            .cloned()
            .with_context(|| format!("rpc {method} missing result"))
    }
}

#[derive(Debug, BorshDeserialize)]
struct StoredHostProgramState {
    discriminator: [u8; 8],
    layout_version: u32,
    state: HostProgramState,
}

fn decode_stored_state(bytes: &[u8]) -> Result<HostProgramState> {
    let mut slice = bytes;
    let stored = StoredHostProgramState::deserialize(&mut slice)
        .map_err(|err| anyhow!("failed to deserialize Solana host state: {err}"))?;
    if stored.discriminator != STATE_ACCOUNT_DISCRIMINATOR
        || stored.layout_version != STATE_ACCOUNT_LAYOUT_VERSION
    {
        bail!("unexpected Solana host state account layout");
    }
    Ok(stored.state)
}
