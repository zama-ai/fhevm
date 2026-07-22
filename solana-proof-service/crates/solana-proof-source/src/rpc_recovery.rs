//! Bounded confirmed JSON-RPC recovery into the same [`CompletedBlock`] boundary.
//!
//! Recovery is **not** the live source. Live ingest remains filtered Yellowstone;
//! this adapter fills parent-chain gaps (and optional bootstrap ranges) via
//! `getBlocks` / `getBlock` at confirmed commitment, then the store applies the
//! resulting blocks through the same atomic path.
//!
//! # Bounds (PoC)
//!
//! Each attempt is capped by `max_slots` and `max_blocks`. Exhausting either
//! bound leaves history incomplete / `RecoveryRequired` — never silently marks
//! history complete. TODO(prod): tune horizons for archive depth and lag SLOs.

use serde::Deserialize;
use serde_json::json;
use tokio_util::sync::CancellationToken;
use zama_solana_transaction::{
    CompiledInstruction as CanonicalCompiledInstruction,
    InnerInstructionGroup as CanonicalInnerInstructionGroup,
};

use crate::{BlockCheckpoint, CanonicalTransaction, CompletedBlock, RawInstruction};

const SOLANA_PROOF_COMMITMENT: &str = "confirmed";

/// Well-known Solana vote program (`Vote111111111111111111111111111111111111111`).
fn vote_program_id() -> [u8; 32] {
    // Decoded once per process from the well-known base58; avoids a brittle
    // hand-maintained byte array in source.
    static DECODED: std::sync::OnceLock<[u8; 32]> = std::sync::OnceLock::new();
    *DECODED.get_or_init(|| {
        decode_hash(
            "vote program",
            "Vote111111111111111111111111111111111111111",
        )
        .expect("well-known vote program id must decode")
    })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecoveryBounds {
    /// Max slot span (`to_slot - from_slot + 1`) per recovery attempt.
    pub max_slots: u64,
    /// Max `getBlock` fetches (existing slots) per recovery attempt.
    pub max_blocks: u64,
}

impl Default for RecoveryBounds {
    fn default() -> Self {
        Self {
            max_slots: 256,
            max_blocks: 128,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RpcRecoveryConfig {
    pub rpc_url: String,
    pub program_id: String,
    pub bounds: RecoveryBounds,
    /// Bootstrap A start: completeness may flip only after recovery proves
    /// contiguous history from this slot through the durable tip.
    pub bootstrap_slot: Option<u64>,
}

#[derive(thiserror::Error, Clone, Debug, PartialEq, Eq)]
pub enum RecoveryError {
    #[error("RPC transport error: {0}")]
    Transport(String),
    #[error("RPC history unavailable: {0}")]
    HistoryUnavailable(String),
    #[error("invalid RPC block or configuration: {0}")]
    Invalid(String),
    #[error("recovery cancelled")]
    Cancelled,
    #[error("recovery bound exhausted: {0}")]
    BoundExhausted(String),
}

pub struct RpcRecoveryClient {
    client: reqwest::Client,
    config: RpcRecoveryConfig,
    program_id: [u8; 32],
}

impl RpcRecoveryClient {
    pub fn new(config: RpcRecoveryConfig) -> Result<Self, RecoveryError> {
        let program_id = decode_hash("program id", &config.program_id)?;
        if config.bounds.max_slots == 0 {
            return Err(RecoveryError::Invalid(
                "recovery.max_slots must be >= 1".to_owned(),
            ));
        }
        if config.bounds.max_blocks == 0 {
            return Err(RecoveryError::Invalid(
                "recovery.max_blocks must be >= 1".to_owned(),
            ));
        }
        if config.rpc_url.trim().is_empty() {
            return Err(RecoveryError::Invalid(
                "recovery.rpc_url must not be empty".to_owned(),
            ));
        }
        Ok(Self {
            client: reqwest::Client::new(),
            config,
            program_id,
        })
    }

    pub fn config(&self) -> &RpcRecoveryConfig {
        &self.config
    }

    pub fn program_id(&self) -> [u8; 32] {
        self.program_id
    }

    /// Fetches and normalizes completed blocks for every existing slot in
    /// `[from_slot, to_slot]` (inclusive), enforcing lean attempt bounds.
    ///
    /// Polls `cancel` before `getBlocks` and before each `getBlock` so long
    /// recovery loops remain cancellation-safe.
    pub async fn fetch_completed_blocks(
        &self,
        from_slot: u64,
        to_slot: u64,
        cancel: &CancellationToken,
    ) -> Result<Vec<CompletedBlock>, RecoveryError> {
        if cancel.is_cancelled() {
            return Err(RecoveryError::Cancelled);
        }
        if to_slot < from_slot {
            return Err(RecoveryError::Invalid(format!(
                "recovery range inverted: from_slot {from_slot} > to_slot {to_slot}"
            )));
        }
        let span = to_slot
            .checked_sub(from_slot)
            .and_then(|d| d.checked_add(1))
            .ok_or_else(|| RecoveryError::Invalid("recovery slot span overflow".to_owned()))?;
        if span > self.config.bounds.max_slots {
            return Err(RecoveryError::BoundExhausted(format!(
                "slot span {span} exceeds max_slots {}",
                self.config.bounds.max_slots
            )));
        }

        let slots = self.get_blocks(from_slot, to_slot).await?;
        if slots.len() as u64 > self.config.bounds.max_blocks {
            return Err(RecoveryError::BoundExhausted(format!(
                "{} existing slots in [{from_slot}, {to_slot}] exceed max_blocks {}",
                slots.len(),
                self.config.bounds.max_blocks
            )));
        }

        let mut blocks = Vec::with_capacity(slots.len());
        for slot in slots {
            if cancel.is_cancelled() {
                return Err(RecoveryError::Cancelled);
            }
            let Some(block) = self.get_block(slot).await? else {
                return Err(RecoveryError::HistoryUnavailable(format!(
                    "getBlock returned null for slot {slot} (pruned or unavailable at confirmed)"
                )));
            };
            blocks.push(block);
        }
        Ok(blocks)
    }

    /// Confirmed tip slot (`getSlot`), used to bound catch-up when Yellowstone
    /// inclusive replay is unavailable.
    pub async fn get_confirmed_slot(&self) -> Result<u64, RecoveryError> {
        let result = self
            .call(
                "getSlot",
                json!([{ "commitment": SOLANA_PROOF_COMMITMENT }]),
            )
            .await?;
        result
            .as_u64()
            .ok_or_else(|| RecoveryError::Invalid("malformed getSlot result".to_owned()))
    }

    async fn get_blocks(&self, from_slot: u64, to_slot: u64) -> Result<Vec<u64>, RecoveryError> {
        let result = self
            .call(
                "getBlocks",
                json!([from_slot, to_slot, { "commitment": SOLANA_PROOF_COMMITMENT }]),
            )
            .await?;
        let slots: Vec<u64> = serde_json::from_value(result)
            .map_err(|err| RecoveryError::Invalid(format!("malformed getBlocks result: {err}")))?;
        if slots.windows(2).any(|pair| pair[0] >= pair[1]) {
            return Err(RecoveryError::Invalid(
                "getBlocks returned non-increasing slots".to_owned(),
            ));
        }
        if slots
            .iter()
            .any(|slot| *slot < from_slot || *slot > to_slot)
        {
            return Err(RecoveryError::Invalid(
                "getBlocks returned slot outside requested range".to_owned(),
            ));
        }
        Ok(slots)
    }

    async fn get_block(&self, slot: u64) -> Result<Option<CompletedBlock>, RecoveryError> {
        let result = self
            .call(
                "getBlock",
                json!([
                    slot,
                    {
                        "encoding": "json",
                        "maxSupportedTransactionVersion": 0,
                        "transactionDetails": "full",
                        "rewards": false,
                        "commitment": SOLANA_PROOF_COMMITMENT,
                    }
                ]),
            )
            .await?;
        if result.is_null() {
            return Ok(None);
        }
        let parsed: RpcBlock = serde_json::from_value(result).map_err(|err| {
            RecoveryError::Invalid(format!("malformed getBlock result at slot {slot}: {err}"))
        })?;
        Ok(Some(normalize_rpc_block(slot, parsed, &self.program_id)?))
    }

    async fn call(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, RecoveryError> {
        let body = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": params,
        });
        let response = self
            .client
            .post(&self.config.rpc_url)
            .json(&body)
            .send()
            .await
            .map_err(|err| RecoveryError::Transport(err.to_string()))?;
        let value: serde_json::Value = response
            .json()
            .await
            .map_err(|err| RecoveryError::Transport(err.to_string()))?;
        if let Some(error) = value.get("error") {
            let message = error.to_string();
            // Solana archive nodes often use -32007 / BlockCleanedUp / SlotSkipped.
            if message.contains("Block cleaned up")
                || message.contains("Block not available")
                || message.contains("Slot skipped")
                || message.contains("-32007")
                || message.contains("-32009")
            {
                return Err(RecoveryError::HistoryUnavailable(message));
            }
            return Err(RecoveryError::Transport(message));
        }
        value
            .get("result")
            .cloned()
            .ok_or_else(|| RecoveryError::Invalid("missing result field".to_owned()))
    }
}

/// Whether Bootstrap A may flip `history_complete` after a successful recovery.
///
/// Requires:
/// - configured `bootstrap_slot` matching durable `history_start`
/// - durable tip extending that start
/// - durable tip equal to the confirmed tip this recovery established
///
/// Single-slot bootstrap alone is not enough when the chain tip is ahead:
/// `durable_tip.slot == confirmed_tip_slot` fails until catch-up proves the
/// full range. Empty recovery must never call this with a vacuous tip match.
pub fn history_complete_justified(
    bootstrap_slot: Option<u64>,
    history_start: Option<&BlockCheckpoint>,
    durable_tip: Option<&BlockCheckpoint>,
    confirmed_tip_slot: u64,
) -> bool {
    let (Some(bootstrap), Some(start), Some(tip)) = (bootstrap_slot, history_start, durable_tip)
    else {
        return false;
    };
    start.slot == bootstrap && tip.slot >= start.slot && tip.slot == confirmed_tip_slot
}

#[derive(Deserialize)]
struct RpcBlock {
    blockhash: String,
    #[serde(rename = "previousBlockhash")]
    previous_blockhash: String,
    #[serde(rename = "parentSlot")]
    parent_slot: u64,
    #[serde(rename = "blockTime")]
    block_time: Option<i64>,
    #[serde(rename = "blockHeight")]
    block_height: Option<u64>,
    #[serde(default)]
    transactions: Vec<RpcBlockTransaction>,
}

#[derive(Deserialize)]
struct RpcBlockTransaction {
    transaction: RpcTxEnvelope,
    meta: Option<RpcMeta>,
    /// Present when `maxSupportedTransactionVersion` is set: `"legacy"` or `0`.
    #[serde(default)]
    version: Option<RpcTxVersion>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
enum RpcTxVersion {
    Legacy(String),
    Number(u8),
}

impl RpcTxVersion {
    fn is_legacy(&self) -> bool {
        match self {
            Self::Legacy(label) => label.eq_ignore_ascii_case("legacy"),
            Self::Number(_) => false,
        }
    }
}

#[derive(Deserialize)]
struct RpcTxEnvelope {
    signatures: Vec<String>,
    message: RpcMessage,
}

#[derive(Deserialize)]
struct RpcMessage {
    #[serde(rename = "accountKeys")]
    account_keys: Vec<String>,
    instructions: Vec<RpcCompiledIx>,
}

#[derive(Deserialize)]
struct RpcCompiledIx {
    #[serde(rename = "programIdIndex")]
    program_id_index: usize,
    accounts: Vec<usize>,
    data: String,
    #[serde(rename = "stackHeight", default)]
    stack_height: Option<u32>,
}

#[derive(Deserialize)]
struct RpcMeta {
    #[serde(default)]
    err: Option<serde_json::Value>,
    #[serde(rename = "innerInstructions", default)]
    inner_instructions: Vec<RpcInnerIxGroup>,
    #[serde(rename = "loadedAddresses", default)]
    loaded_addresses: Option<RpcLoadedAddresses>,
}

#[derive(Deserialize, Default)]
struct RpcLoadedAddresses {
    #[serde(default)]
    writable: Vec<String>,
    #[serde(default)]
    readonly: Vec<String>,
}

#[derive(Deserialize)]
struct RpcInnerIxGroup {
    index: usize,
    instructions: Vec<RpcCompiledIx>,
}

/// Normalize one confirmed `getBlock` JSON result into a [`CompletedBlock`].
///
/// Transactions that do not mention `program_id` are dropped (Yellowstone
/// `account_include` parity). Failed and vote identities that mention the
/// program remain with empty instruction lists. Transaction `index` is the
/// full-block position so sparse indexes match the Yellowstone path.
fn normalize_rpc_block(
    slot: u64,
    block: RpcBlock,
    program_id: &[u8; 32],
) -> Result<CompletedBlock, RecoveryError> {
    let block_hash = decode_hash("blockhash", &block.blockhash)?;
    let parent_hash = decode_hash("previousBlockhash", &block.previous_blockhash)?;
    let executed_transaction_count = block.transactions.len() as u64;

    let mut transactions = Vec::new();
    for (index, tx) in block.transactions.into_iter().enumerate() {
        let index = index as u64;
        let Some(normalized) = normalize_rpc_transaction(tx, index, program_id)? else {
            continue;
        };
        transactions.push(normalized);
    }

    Ok(CompletedBlock {
        slot,
        block_hash,
        parent_slot: block.parent_slot,
        parent_hash,
        block_time: block.block_time,
        block_height: block.block_height,
        executed_transaction_count,
        transactions,
    })
}

fn normalize_rpc_transaction(
    tx: RpcBlockTransaction,
    index: u64,
    program_id: &[u8; 32],
) -> Result<Option<CanonicalTransaction>, RecoveryError> {
    let meta = tx.meta.as_ref().ok_or_else(|| {
        RecoveryError::Invalid(format!("transaction {index} has no status metadata"))
    })?;

    let static_keys = tx
        .transaction
        .message
        .account_keys
        .iter()
        .map(|key| decode_hash("account key", key))
        .collect::<Result<Vec<_>, _>>()?;
    let (loaded_writable, loaded_readonly) = match meta.loaded_addresses.as_ref() {
        Some(loaded) => (
            loaded
                .writable
                .iter()
                .map(|key| decode_hash("loaded writable", key))
                .collect::<Result<Vec<_>, _>>()?,
            loaded
                .readonly
                .iter()
                .map(|key| decode_hash("loaded readonly", key))
                .collect::<Result<Vec<_>, _>>()?,
        ),
        None => (Vec::new(), Vec::new()),
    };

    if !mentions_program(&static_keys, &loaded_writable, &loaded_readonly, program_id) {
        return Ok(None);
    }

    let signature =
        decode_signature(tx.transaction.signatures.first().ok_or_else(|| {
            RecoveryError::Invalid(format!("transaction {index} has no signature"))
        })?)?;

    // Match Yellowstone/geyser `is_vote`: Solana simple-vote semantics, not
    // "vote program appears anywhere in account keys".
    let is_vote = is_simple_vote_transaction(
        &tx.transaction.signatures,
        tx.version.as_ref(),
        &tx.transaction.message.instructions,
        &static_keys,
        meta.loaded_addresses.as_ref(),
    );
    let succeeded = meta.err.is_none();
    let instructions = if !succeeded || is_vote {
        Vec::new()
    } else {
        resolve_rpc_instructions(
            &static_keys,
            &loaded_writable,
            &loaded_readonly,
            &tx.transaction.message.instructions,
            &meta.inner_instructions,
        )?
    };

    Ok(Some(CanonicalTransaction {
        signature,
        index,
        succeeded,
        is_vote,
        instructions,
    }))
}

/// Approximate Solana `is_simple_vote_transaction` for JSON-RPC blocks.
///
/// Conditions (same as solana-sdk / geyser `is_vote`):
/// 1. 1 or 2 signatures
/// 2. legacy message
/// 3. exactly one top-level instruction
/// 4. that instruction's program is the vote program
fn is_simple_vote_transaction(
    signatures: &[String],
    version: Option<&RpcTxVersion>,
    top_level: &[RpcCompiledIx],
    static_keys: &[[u8; 32]],
    loaded_addresses: Option<&RpcLoadedAddresses>,
) -> bool {
    if signatures.len() >= 3 {
        return false;
    }
    let is_legacy = match version {
        Some(v) => v.is_legacy(),
        // Absent version: treat as legacy only when no address-table loads
        // (v0 messages populate loadedAddresses under maxSupportedTransactionVersion).
        None => match loaded_addresses {
            Some(loaded) => loaded.writable.is_empty() && loaded.readonly.is_empty(),
            None => true,
        },
    };
    if !is_legacy {
        return false;
    }
    if top_level.len() != 1 {
        return false;
    }
    let Some(ix) = top_level.first() else {
        return false;
    };
    static_keys
        .get(ix.program_id_index)
        .is_some_and(|program_id| program_id == &vote_program_id())
}

fn mentions_program(
    static_keys: &[[u8; 32]],
    loaded_writable: &[[u8; 32]],
    loaded_readonly: &[[u8; 32]],
    program_id: &[u8; 32],
) -> bool {
    static_keys.iter().any(|key| key == program_id)
        || loaded_writable.iter().any(|key| key == program_id)
        || loaded_readonly.iter().any(|key| key == program_id)
}

fn resolve_rpc_instructions(
    static_keys: &[[u8; 32]],
    loaded_writable: &[[u8; 32]],
    loaded_readonly: &[[u8; 32]],
    top_level: &[RpcCompiledIx],
    inner: &[RpcInnerIxGroup],
) -> Result<Vec<RawInstruction>, RecoveryError> {
    let top_level = top_level
        .iter()
        .map(|instruction| canonical_instruction(instruction, false))
        .collect::<Result<Vec<_>, _>>()?;
    let inner = inner
        .iter()
        .map(|group| {
            Ok(CanonicalInnerInstructionGroup {
                top_level_index: group.index,
                instructions: group
                    .instructions
                    .iter()
                    .map(|instruction| canonical_instruction(instruction, true))
                    .collect::<Result<Vec<_>, RecoveryError>>()?,
            })
        })
        .collect::<Result<Vec<_>, RecoveryError>>()?;
    zama_solana_transaction::resolve_transaction(
        static_keys,
        loaded_writable,
        loaded_readonly,
        top_level,
        inner,
    )
    .map_err(|err| RecoveryError::Invalid(format!("invalid transaction instructions: {err}")))
    .map(|instructions| {
        instructions
            .into_iter()
            .map(|instruction| RawInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: instruction.data,
                top_level_index: instruction.top_level_index,
                stack_height: Some(instruction.stack_height),
            })
            .collect()
    })
}

fn canonical_instruction(
    instruction: &RpcCompiledIx,
    is_inner: bool,
) -> Result<CanonicalCompiledInstruction, RecoveryError> {
    Ok(CanonicalCompiledInstruction {
        program_id_index: instruction.program_id_index,
        account_indices: instruction.accounts.clone(),
        data: bs58_decode(&instruction.data)?,
        stack_height: if is_inner {
            instruction.stack_height
        } else {
            None
        },
    })
}

fn decode_hash(name: &str, encoded: &str) -> Result<[u8; 32], RecoveryError> {
    let bytes = bs58::decode(encoded)
        .into_vec()
        .map_err(|err| RecoveryError::Invalid(format!("invalid {name}: {err}")))?;
    <[u8; 32]>::try_from(bytes.as_slice()).map_err(|_| {
        RecoveryError::Invalid(format!(
            "{name} has invalid length {}, expected 32 bytes",
            bytes.len()
        ))
    })
}

fn decode_signature(encoded: &str) -> Result<[u8; 64], RecoveryError> {
    let bytes = bs58::decode(encoded)
        .into_vec()
        .map_err(|err| RecoveryError::Invalid(format!("invalid signature: {err}")))?;
    <[u8; 64]>::try_from(bytes.as_slice()).map_err(|_| {
        RecoveryError::Invalid(format!(
            "signature has invalid length {}, expected 64 bytes",
            bytes.len()
        ))
    })
}

fn bs58_decode(input: &str) -> Result<Vec<u8>, RecoveryError> {
    bs58::decode(input)
        .into_vec()
        .map_err(|err| RecoveryError::Invalid(format!("invalid base58 instruction data: {err}")))
}

/// Parse RPC block JSON (the `result` object) for unit tests and adapters.
pub fn normalize_rpc_block_json(
    slot: u64,
    value: serde_json::Value,
    program_id: &[u8; 32],
) -> Result<CompletedBlock, RecoveryError> {
    let parsed: RpcBlock = serde_json::from_value(value)
        .map_err(|err| RecoveryError::Invalid(format!("malformed getBlock JSON: {err}")))?;
    normalize_rpc_block(slot, parsed, program_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use sha2::{Digest, Sha256};

    fn pk(tag: u8) -> [u8; 32] {
        [tag; 32]
    }

    fn b58(bytes: &[u8]) -> String {
        bs58::encode(bytes).into_string()
    }

    fn sig(tag: u8) -> String {
        b58(&[tag; 64])
    }

    fn program() -> [u8; 32] {
        pk(0x42)
    }

    fn empty_block_json(slot: u64) -> serde_json::Value {
        json!({
            "blockhash": b58(&pk(slot as u8)),
            "previousBlockhash": b58(&pk(slot.saturating_sub(1) as u8)),
            "parentSlot": slot.saturating_sub(1),
            "blockTime": 1_700_000_000,
            "blockHeight": 99,
            "transactions": [],
        })
    }

    #[test]
    fn normalizes_empty_block() {
        let block =
            normalize_rpc_block_json(7, empty_block_json(7), &program()).expect("empty block");
        assert_eq!(block.slot, 7);
        assert_eq!(block.block_hash, pk(7));
        assert_eq!(block.parent_slot, 6);
        assert_eq!(block.parent_hash, pk(6));
        assert_eq!(block.block_time, Some(1_700_000_000));
        assert_eq!(block.block_height, Some(99));
        assert_eq!(block.executed_transaction_count, 0);
        assert!(block.transactions.is_empty());
    }

    #[test]
    fn drops_unrelated_transactions_keeps_sparse_indexes() {
        let digest = Sha256::digest(b"global:make_handle_public");
        let data = b58(&digest[..8]);
        let host = program();
        let other = pk(0x99);
        let value = json!({
            "blockhash": b58(&pk(7)),
            "previousBlockhash": b58(&pk(6)),
            "parentSlot": 6,
            "blockTime": null,
            "blockHeight": null,
            "transactions": [
                {
                    "transaction": {
                        "signatures": [sig(1)],
                        "message": {
                            "accountKeys": [b58(&other), b58(&pk(1))],
                            "instructions": [{
                                "programIdIndex": 0,
                                "accounts": [1],
                                "data": data,
                            }],
                        },
                    },
                    "meta": { "err": null, "innerInstructions": [], "loadedAddresses": { "writable": [], "readonly": [] } },
                },
                {
                    "transaction": {
                        "signatures": [sig(2)],
                        "message": {
                            "accountKeys": [b58(&host), b58(&pk(2)), b58(&pk(3))],
                            "instructions": [{
                                "programIdIndex": 0,
                                "accounts": [1, 2],
                                "data": data,
                            }],
                        },
                    },
                    "meta": { "err": null, "innerInstructions": [], "loadedAddresses": { "writable": [], "readonly": [] } },
                },
                {
                    "transaction": {
                        "signatures": [sig(3)],
                        "message": {
                            "accountKeys": [b58(&other), b58(&pk(4))],
                            "instructions": [{
                                "programIdIndex": 0,
                                "accounts": [1],
                                "data": data,
                            }],
                        },
                    },
                    "meta": { "err": null, "innerInstructions": [], "loadedAddresses": { "writable": [], "readonly": [] } },
                },
            ],
        });
        let block = normalize_rpc_block_json(7, value, &host).unwrap();
        assert_eq!(block.executed_transaction_count, 3);
        assert_eq!(block.transactions.len(), 1);
        assert_eq!(block.transactions[0].index, 1);
        assert_eq!(block.transactions[0].signature, [2u8; 64]);
        assert!(block.transactions[0].succeeded);
        assert!(!block.transactions[0].is_vote);
        assert_eq!(block.transactions[0].instructions.len(), 1);
        assert_eq!(block.transactions[0].instructions[0].program_id, host);
    }

    #[test]
    fn failed_and_vote_identities_keep_empty_instructions() {
        let host = program();
        let value = json!({
            "blockhash": b58(&pk(7)),
            "previousBlockhash": b58(&pk(6)),
            "parentSlot": 6,
            "transactions": [
                {
                    "transaction": {
                        "signatures": [sig(1)],
                        "message": {
                            "accountKeys": [b58(&host), b58(&pk(1))],
                            "instructions": [{
                                "programIdIndex": 0,
                                "accounts": [1],
                                "data": b58(&[1, 2, 3]),
                            }],
                        },
                    },
                    "meta": { "err": { "InstructionError": [0, "Custom"] }, "innerInstructions": [] },
                    "version": "legacy",
                },
                {
                    "transaction": {
                        "signatures": [sig(2)],
                        "message": {
                            "accountKeys": [b58(&vote_program_id()), b58(&host), b58(&pk(2))],
                            "instructions": [{
                                "programIdIndex": 0,
                                "accounts": [2],
                                "data": b58(&[9]),
                            }],
                        },
                    },
                    "meta": { "err": null, "innerInstructions": [] },
                    "version": "legacy",
                },
            ],
        });
        let block = normalize_rpc_block_json(7, value, &host).unwrap();
        assert_eq!(block.transactions.len(), 2);
        assert!(!block.transactions[0].succeeded);
        assert!(block.transactions[0].instructions.is_empty());
        assert_eq!(block.transactions[0].index, 0);
        assert!(block.transactions[1].succeeded);
        assert!(block.transactions[1].is_vote);
        assert!(block.transactions[1].instructions.is_empty());
        assert_eq!(block.transactions[1].index, 1);
    }

    #[test]
    fn vote_program_in_accounts_alone_is_not_simple_vote() {
        let host = program();
        let value = json!({
            "blockhash": b58(&pk(7)),
            "previousBlockhash": b58(&pk(6)),
            "parentSlot": 6,
            "transactions": [{
                "transaction": {
                    "signatures": [sig(2)],
                    "message": {
                        "accountKeys": [b58(&host), b58(&pk(2)), b58(&vote_program_id())],
                        "instructions": [{
                            "programIdIndex": 0,
                            "accounts": [1, 2],
                            "data": b58(&[7]),
                        }],
                    },
                },
                "meta": { "err": null, "innerInstructions": [] },
                "version": "legacy",
            }],
        });
        let block = normalize_rpc_block_json(7, value, &host).unwrap();
        assert_eq!(block.transactions.len(), 1);
        assert!(!block.transactions[0].is_vote);
        assert_eq!(block.transactions[0].instructions.len(), 1);
    }

    #[test]
    fn resolves_inner_instructions_in_execution_order() {
        let host = program();
        let cpi = pk(0x55);
        let value = json!({
            "blockhash": b58(&pk(7)),
            "previousBlockhash": b58(&pk(6)),
            "parentSlot": 6,
            "transactions": [{
                "transaction": {
                    "signatures": [sig(9)],
                    "message": {
                        "accountKeys": [b58(&host), b58(&pk(1)), b58(&pk(2)), b58(&cpi)],
                        "instructions": [{
                            "programIdIndex": 0,
                            "accounts": [1, 2],
                            "data": b58(&[7, 7]),
                        }],
                    },
                },
                "meta": {
                    "err": null,
                    "innerInstructions": [{
                        "index": 0,
                        "instructions": [{
                            "programIdIndex": 3,
                            "accounts": [1],
                            "data": b58(&[8]),
                            "stackHeight": 2,
                        }],
                    }],
                    "loadedAddresses": { "writable": [], "readonly": [] },
                },
            }],
        });
        let block = normalize_rpc_block_json(7, value, &host).unwrap();
        let ixs = &block.transactions[0].instructions;
        assert_eq!(ixs.len(), 2);
        assert_eq!(ixs[0].program_id, host);
        assert_eq!(ixs[0].data, vec![7, 7]);
        assert_eq!(ixs[1].program_id, cpi);
        assert_eq!(ixs[1].data, vec![8]);
        assert_eq!(ixs[1].stack_height, Some(2));
    }

    #[test]
    fn history_complete_requires_bootstrap_match_and_confirmed_tip() {
        let start = BlockCheckpoint {
            slot: 10,
            block_hash: pk(10),
        };
        let tip = BlockCheckpoint {
            slot: 20,
            block_hash: pk(20),
        };
        let single = BlockCheckpoint {
            slot: 10,
            block_hash: pk(10),
        };
        assert!(!history_complete_justified(
            None,
            Some(&start),
            Some(&tip),
            20
        ));
        assert!(!history_complete_justified(
            Some(5),
            Some(&start),
            Some(&tip),
            20
        ));
        assert!(history_complete_justified(
            Some(10),
            Some(&start),
            Some(&tip),
            20
        ));
        assert!(!history_complete_justified(
            Some(10),
            Some(&start),
            Some(&tip),
            21
        ));
        assert!(!history_complete_justified(
            Some(10),
            Some(&start),
            None,
            20
        ));
        // Single-slot bootstrap with tip ahead must not flip complete.
        assert!(!history_complete_justified(
            Some(10),
            Some(&single),
            Some(&single),
            99
        ));
        // Single-slot bootstrap equals confirmed tip: full proven range.
        assert!(history_complete_justified(
            Some(10),
            Some(&single),
            Some(&single),
            10
        ));
    }

    #[test]
    fn rejects_zero_bounds() {
        let err = RpcRecoveryClient::new(RpcRecoveryConfig {
            rpc_url: "http://127.0.0.1:8899".to_owned(),
            program_id: b58(&program()),
            bounds: RecoveryBounds {
                max_slots: 0,
                max_blocks: 1,
            },
            bootstrap_slot: None,
        });
        assert!(matches!(err, Err(RecoveryError::Invalid(_))));
    }

    #[tokio::test]
    async fn fetch_rejects_span_over_max_slots_without_rpc() {
        let client = RpcRecoveryClient::new(RpcRecoveryConfig {
            rpc_url: "http://127.0.0.1:1".to_owned(),
            program_id: b58(&program()),
            bounds: RecoveryBounds {
                max_slots: 2,
                max_blocks: 128,
            },
            bootstrap_slot: None,
        })
        .expect("valid recovery client");
        let cancel = CancellationToken::new();
        let err = client
            .fetch_completed_blocks(10, 20, &cancel)
            .await
            .unwrap_err();
        assert!(matches!(err, RecoveryError::BoundExhausted(_)));
    }

    #[tokio::test]
    async fn fetch_respects_cancel_before_rpc() {
        let client = RpcRecoveryClient::new(RpcRecoveryConfig {
            rpc_url: "http://127.0.0.1:1".to_owned(),
            program_id: b58(&program()),
            bounds: RecoveryBounds {
                max_slots: 2,
                max_blocks: 128,
            },
            bootstrap_slot: None,
        })
        .expect("valid recovery client");
        let cancel = CancellationToken::new();
        cancel.cancel();
        let err = client
            .fetch_completed_blocks(10, 11, &cancel)
            .await
            .unwrap_err();
        assert!(matches!(err, RecoveryError::Cancelled));
    }
}
