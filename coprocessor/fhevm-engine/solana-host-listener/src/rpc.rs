use anyhow::{bail, Context, Result};
use serde_json::{json, Value};

#[derive(Clone, Debug)]
pub struct BlockInfo {
    pub slot: u64,
    pub blockhash: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct SignatureInfo {
    pub signature: String,
    pub slot: u64,
}

#[derive(Clone, Debug)]
pub struct ConfirmedTransaction {
    pub signature: String,
    pub slot: u64,
    pub blockhash: Vec<u8>,
    pub log_messages: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct SolanaRpcClient {
    client: reqwest::Client,
    rpc_url: String,
}

impl SolanaRpcClient {
    pub fn new(rpc_url: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            rpc_url: rpc_url.into(),
        }
    }

    pub async fn get_signatures_for_address(
        &self,
        program_id: &str,
        limit: usize,
        commitment: &str,
    ) -> Result<Vec<SignatureInfo>> {
        let result = self
            .rpc_call(
                "getSignaturesForAddress",
                json!([
                    program_id,
                    {
                        "limit": limit,
                        "commitment": commitment
                    }
                ]),
            )
            .await?;

        let Some(entries) = result.as_array() else {
            bail!("getSignaturesForAddress returned non-array payload");
        };

        let mut signatures = Vec::with_capacity(entries.len());
        for entry in entries {
            if !entry["err"].is_null() {
                continue;
            }
            let signature = entry["signature"]
                .as_str()
                .context("missing signature")?
                .to_owned();
            let slot = entry["slot"].as_u64().context("missing slot")?;
            signatures.push(SignatureInfo { signature, slot });
        }
        Ok(signatures)
    }

    pub async fn get_slot(&self, commitment: &str) -> Result<u64> {
        let result = self
            .rpc_call("getSlot", json!([{ "commitment": commitment }]))
            .await?;

        result.as_u64().context("missing slot in getSlot result")
    }

    pub async fn get_blocks(
        &self,
        start_slot: u64,
        end_slot: u64,
        commitment: &str,
    ) -> Result<Vec<u64>> {
        let result = self
            .rpc_call(
                "getBlocks",
                json!([
                    start_slot,
                    end_slot,
                    {
                        "commitment": commitment
                    }
                ]),
            )
            .await?;

        let Some(entries) = result.as_array() else {
            bail!("getBlocks returned non-array payload");
        };

        entries
            .iter()
            .map(|value| value.as_u64().context("block slot is not a u64"))
            .collect()
    }

    pub async fn get_block(
        &self,
        slot: u64,
        commitment: &str,
    ) -> Result<Option<BlockInfo>> {
        let result = self
            .rpc_call(
                "getBlock",
                json!([
                    slot,
                    {
                        "encoding": "json",
                        "transactionDetails": "none",
                        "rewards": false,
                        "commitment": commitment,
                        "maxSupportedTransactionVersion": 0
                    }
                ]),
            )
            .await?;

        if result.is_null() {
            return Ok(None);
        }

        let blockhash = result["blockhash"]
            .as_str()
            .context("missing blockhash in getBlock result")?;

        Ok(Some(BlockInfo {
            slot,
            blockhash: bs58::decode(blockhash)
                .into_vec()
                .context("decode blockhash from base58")?,
        }))
    }

    pub async fn get_transaction(
        &self,
        signature: &str,
        commitment: &str,
    ) -> Result<Option<ConfirmedTransaction>> {
        let result = self
            .rpc_call(
                "getTransaction",
                json!([
                    signature,
                    {
                        "encoding": "json",
                        "commitment": commitment,
                        "maxSupportedTransactionVersion": 0
                    }
                ]),
            )
            .await?;

        if result.is_null() {
            return Ok(None);
        }

        let slot = result["slot"]
            .as_u64()
            .context("missing transaction slot")?;
        let Some(block) = self.get_block(slot, commitment).await? else {
            return Ok(None);
        };
        let log_messages = result["meta"]["logMessages"]
            .as_array()
            .context("missing transaction logMessages")?
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .map(str::to_owned)
                    .context("log message is not a string")
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Some(ConfirmedTransaction {
            signature: signature.to_owned(),
            slot,
            blockhash: block.blockhash,
            log_messages,
        }))
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
