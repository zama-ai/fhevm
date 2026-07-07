//! Solana chain access for the MMR proof service: signatures, transactions,
//! and the live `EncryptedValue` account, behind a `ChainFetcher` trait so
//! catch-up/replay logic is testable without a live RPC node.
//!
//! Deliberately a thin `reqwest` JSON-RPC client rather than a `solana-client`/
//! `solana-sdk` dependency: the relayer/kms-connector only ever pin narrow
//! `solana-*` leaf crates (e.g. `solana-pubkey`), and pulling in the full
//! client stack here would both bloat the binary and risk a version clash.
//! Uses `getTransaction` with `encoding: "json"`, which returns instructions
//! pre-compiled to account-index/base58-data form — the same shape whether
//! top-level or inner — so no raw wire-format (compact-array) parsing is needed.

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

use crate::solana_proof::decode::RawInstruction;

#[derive(thiserror::Error, Debug)]
pub enum ChainError {
    #[error("RPC transport error: {0}")]
    Transport(String),
    #[error("RPC returned an error: {0}")]
    Rpc(String),
    #[error("malformed base58: {0}")]
    Base58(String),
}

/// One fetched transaction, instructions flattened into on-chain execution
/// order (each top-level instruction immediately followed by the inner
/// instructions it spawned via CPI).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChainTransaction {
    pub signature: String,
    pub slot: u64,
    pub instructions: Vec<RawInstruction>,
}

/// The on-chain `EncryptedValue` state needed to verify a reconstructed proof.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OnChainLineageState {
    pub peaks: Vec<[u8; 32]>,
    pub leaf_count: u64,
}

#[async_trait]
pub trait ChainFetcher: Send + Sync {
    /// Signatures touching `address`, newest-first (as Solana's RPC returns
    /// them). `before` pages backward from a previous result; `until` bounds
    /// the oldest signature already processed.
    async fn get_signatures_for_address(
        &self,
        address: [u8; 32],
        before: Option<&str>,
        until: Option<&str>,
        limit: usize,
    ) -> Result<Vec<String>, ChainError>;

    async fn get_transaction(
        &self,
        signature: &str,
    ) -> Result<Option<ChainTransaction>, ChainError>;

    /// Fetches and decodes the live `EncryptedValue` account at finalized commitment.
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
    pub fn new(rpc_url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            rpc_url,
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

fn base58_to_32(address: &str) -> Result<[u8; 32], ChainError> {
    crate::http::utils::decode_solana_address(address)
        .map_err(|e| ChainError::Base58(e.to_string()))
}

fn base58_encode(bytes: &[u8; 32]) -> String {
    // Minimal encoder mirroring `solana_address.rs`'s decoder; kept local since
    // that module intentionally only exposes decode (validation-side use case).
    const ALPHABET: &[u8; 58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    let leading_zeros = bytes.iter().take_while(|b| **b == 0).count();
    let mut digits: Vec<u8> = vec![0];
    for &byte in bytes {
        let mut carry = byte as u32;
        for digit in digits.iter_mut() {
            carry += (*digit as u32) << 8;
            *digit = (carry % 58) as u8;
            carry /= 58;
        }
        while carry > 0 {
            digits.push((carry % 58) as u8);
            carry /= 58;
        }
    }
    let mut out: Vec<u8> = std::iter::repeat_n(ALPHABET[0], leading_zeros).collect();
    out.extend(digits.iter().rev().map(|&d| ALPHABET[d as usize]));
    String::from_utf8(out).unwrap()
}

#[derive(Deserialize)]
struct SignatureEntry {
    signature: String,
}

#[derive(Deserialize)]
struct CompiledIx {
    #[serde(rename = "programIdIndex")]
    program_id_index: usize,
    accounts: Vec<usize>,
    data: String,
}

#[derive(Deserialize)]
struct InnerIxGroup {
    index: usize,
    instructions: Vec<CompiledIx>,
}

#[derive(Deserialize)]
struct Message {
    #[serde(rename = "accountKeys")]
    account_keys: Vec<String>,
    instructions: Vec<CompiledIx>,
}

#[derive(Deserialize)]
struct TxEnvelope {
    message: Message,
}

#[derive(Deserialize)]
struct Meta {
    #[serde(rename = "innerInstructions", default)]
    inner_instructions: Vec<InnerIxGroup>,
    #[serde(default)]
    err: Option<serde_json::Value>,
    #[serde(rename = "loadedAddresses", default)]
    loaded_addresses: Option<LoadedAddresses>,
}

#[derive(Deserialize, Default)]
struct LoadedAddresses {
    #[serde(default)]
    writable: Vec<String>,
    #[serde(default)]
    readonly: Vec<String>,
}

#[derive(Deserialize)]
struct GetTransactionResult {
    slot: u64,
    transaction: TxEnvelope,
    meta: Option<Meta>,
}

fn compiled_to_raw(
    ix: &CompiledIx,
    account_keys: &[[u8; 32]],
) -> Result<RawInstruction, ChainError> {
    let program_id = *account_keys
        .get(ix.program_id_index)
        .ok_or_else(|| ChainError::Rpc("programIdIndex out of range".to_string()))?;
    let accounts = ix
        .accounts
        .iter()
        .map(|&idx| {
            account_keys
                .get(idx)
                .copied()
                .ok_or_else(|| ChainError::Rpc("account index out of range".to_string()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let data = bs58_decode(&ix.data)?;
    Ok(RawInstruction {
        program_id,
        accounts,
        data,
    })
}

/// Base58-decodes arbitrary-length instruction data (not fixed to 32 bytes, so
/// this cannot reuse `solana_address.rs`'s 32-byte-only decoder/validator).
fn bs58_decode(input: &str) -> Result<Vec<u8>, ChainError> {
    const ALPHABET: &[u8; 58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    let leading_ones = input.bytes().take_while(|b| *b == b'1').count();
    let mut bytes: Vec<u8> = Vec::with_capacity(input.len());
    for c in input.bytes() {
        let digit = ALPHABET
            .iter()
            .position(|&a| a == c)
            .ok_or_else(|| ChainError::Base58(format!("invalid base58 byte {c}")))?
            as u32;
        let mut carry = digit;
        for byte in bytes.iter_mut() {
            carry += (*byte as u32) * 58;
            *byte = (carry & 0xff) as u8;
            carry >>= 8;
        }
        while carry > 0 {
            bytes.push((carry & 0xff) as u8);
            carry >>= 8;
        }
    }
    let mut decoded = vec![0u8; leading_ones];
    decoded.extend(bytes.into_iter().rev());
    Ok(decoded)
}

#[async_trait]
impl ChainFetcher for RpcChainFetcher {
    async fn get_signatures_for_address(
        &self,
        address: [u8; 32],
        before: Option<&str>,
        until: Option<&str>,
        limit: usize,
    ) -> Result<Vec<String>, ChainError> {
        let mut opts = serde_json::Map::new();
        opts.insert("limit".to_string(), json!(limit));
        if let Some(before) = before {
            opts.insert("before".to_string(), json!(before));
        }
        if let Some(until) = until {
            opts.insert("until".to_string(), json!(until));
        }
        let result = self
            .call(
                "getSignaturesForAddress",
                json!([base58_encode(&address), opts]),
            )
            .await?;
        let entries: Vec<SignatureEntry> =
            serde_json::from_value(result).map_err(|e| ChainError::Rpc(e.to_string()))?;
        Ok(entries.into_iter().map(|e| e.signature).collect())
    }

    async fn get_transaction(
        &self,
        signature: &str,
    ) -> Result<Option<ChainTransaction>, ChainError> {
        let result = self
            .call(
                "getTransaction",
                json!([
                    signature,
                    {"encoding": "json", "maxSupportedTransactionVersion": 0, "commitment": "finalized"}
                ]),
            )
            .await?;
        if result.is_null() {
            return Ok(None);
        }
        let parsed: GetTransactionResult =
            serde_json::from_value(result).map_err(|e| ChainError::Rpc(e.to_string()))?;
        if parsed.meta.as_ref().and_then(|m| m.err.clone()).is_some() {
            // A failed transaction never persisted its instructions' effects on-chain.
            return Ok(Some(ChainTransaction {
                signature: signature.to_string(),
                slot: parsed.slot,
                instructions: Vec::new(),
            }));
        }
        let mut account_keys: Vec<[u8; 32]> = parsed
            .transaction
            .message
            .account_keys
            .iter()
            .map(|s| base58_to_32(s))
            .collect::<Result<_, _>>()?;
        if let Some(loaded) = parsed
            .meta
            .as_ref()
            .and_then(|m| m.loaded_addresses.as_ref())
        {
            for addr in loaded.writable.iter().chain(loaded.readonly.iter()) {
                account_keys.push(base58_to_32(addr)?);
            }
        }

        let instructions = flatten_execution_order(
            &parsed.transaction.message.instructions,
            parsed
                .meta
                .as_ref()
                .map(|m| m.inner_instructions.as_slice())
                .unwrap_or(&[]),
            &account_keys,
        )?;

        Ok(Some(ChainTransaction {
            signature: signature.to_string(),
            slot: parsed.slot,
            instructions,
        }))
    }

    async fn get_lineage_state(
        &self,
        address: [u8; 32],
    ) -> Result<Option<OnChainLineageState>, ChainError> {
        let result = self
            .call(
                "getAccountInfo",
                json!([base58_encode(&address), {"encoding": "base64", "commitment": "finalized"}]),
            )
            .await?;
        if result.is_null() || result.get("value").map(|v| v.is_null()).unwrap_or(true) {
            return Ok(None);
        }
        let data_field = result["value"]["data"][0]
            .as_str()
            .ok_or_else(|| ChainError::Rpc("missing base64 account data".to_string()))?;
        let raw = base64_decode(data_field).map_err(ChainError::Base58)?;
        let decoded = zama_solana_acl::decode_on_chain_account(&raw)
            .map_err(|e| ChainError::Rpc(format!("{e:?}")))?;
        Ok(Some(OnChainLineageState {
            peaks: decoded.peaks,
            leaf_count: decoded.leaf_count,
        }))
    }
}

/// Interleaves top-level instructions with the inner instructions they spawned
/// via CPI, in on-chain execution order: top-level instruction `i` runs, then
/// (if any) the inner instructions Solana recorded at group `index == i`.
fn flatten_execution_order(
    top_level: &[CompiledIx],
    inner_groups: &[InnerIxGroup],
    account_keys: &[[u8; 32]],
) -> Result<Vec<RawInstruction>, ChainError> {
    let mut by_index: std::collections::HashMap<usize, &Vec<CompiledIx>> =
        std::collections::HashMap::new();
    for group in inner_groups {
        by_index.insert(group.index, &group.instructions);
    }
    let mut out = Vec::new();
    for (i, ix) in top_level.iter().enumerate() {
        out.push(compiled_to_raw(ix, account_keys)?);
        if let Some(inner) = by_index.get(&i) {
            for inner_ix in inner.iter() {
                out.push(compiled_to_raw(inner_ix, account_keys)?);
            }
        }
    }
    Ok(out)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flatten_interleaves_inner_after_its_top_level_instruction() {
        let account_keys = vec![[1u8; 32], [2u8; 32], [3u8; 32]];
        let top_level = vec![
            CompiledIx {
                program_id_index: 0,
                accounts: vec![],
                data: "".to_string(),
            },
            CompiledIx {
                program_id_index: 1,
                accounts: vec![],
                data: "".to_string(),
            },
        ];
        let inner_groups = vec![InnerIxGroup {
            index: 0,
            instructions: vec![CompiledIx {
                program_id_index: 2,
                accounts: vec![],
                data: "".to_string(),
            }],
        }];
        let out = flatten_execution_order(&top_level, &inner_groups, &account_keys).unwrap();
        assert_eq!(out.len(), 3);
        assert_eq!(out[0].program_id, [1u8; 32]);
        assert_eq!(out[1].program_id, [3u8; 32]); // inner CPI spawned by top-level 0
        assert_eq!(out[2].program_id, [2u8; 32]);
    }

    #[test]
    fn base58_roundtrip_matches_solana_address_decoder() {
        let bytes = [42u8; 32];
        let encoded = base58_encode(&bytes);
        let decoded = base58_to_32(&encoded).unwrap();
        assert_eq!(decoded, bytes);
    }

    #[test]
    fn base64_decode_matches_known_vector() {
        // "hello" base64-encoded.
        assert_eq!(base64_decode("aGVsbG8=").unwrap(), b"hello".to_vec());
    }
}
