use anyhow::{Context, Result};
use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD, Engine};
use reqwest::Client;
use serde_json::{json, Value};
use solana_pubkey::Pubkey;
use tracing::{debug, warn};

use crate::contracts::{FinalizedEventEnvelope, HandleBytes, ProgramEvent, INTERFACE_VERSION};
use crate::poller::{Cursor, EventSource, SourceBatch};

const OP_REQUESTED_ADD_DISC: [u8; 8] = [0x8D, 0x59, 0xAF, 0xBE, 0x59, 0x4B, 0x41, 0x61];
const OP_REQUESTED_SUB_DISC: [u8; 8] = [0xB8, 0x04, 0x1A, 0x3C, 0x56, 0x1C, 0x86, 0x92];
const OP_REQUESTED_BINARY_DISC: [u8; 8] = [0x56, 0xF9, 0x69, 0xC3, 0xFA, 0x79, 0xE8, 0xDD];
const OP_REQUESTED_UNARY_DISC: [u8; 8] = [0x17, 0x57, 0x06, 0x72, 0x02, 0xF8, 0x8F, 0x33];
const OP_REQUESTED_IF_THEN_ELSE_DISC: [u8; 8] = [0xFD, 0xD9, 0x97, 0x53, 0x3C, 0x18, 0x34, 0x01];
const OP_REQUESTED_CAST_DISC: [u8; 8] = [0x2E, 0x1D, 0xA1, 0x99, 0xF1, 0x6C, 0x17, 0xED];
const OP_REQUESTED_TRIVIAL_ENCRYPT_DISC: [u8; 8] = [0xD8, 0xB2, 0x86, 0x4E, 0xC6, 0xFE, 0xDE, 0x63];
const OP_REQUESTED_RAND_DISC: [u8; 8] = [0x07, 0x2A, 0x7C, 0x69, 0x3D, 0xEE, 0xBF, 0x26];
const OP_REQUESTED_RAND_BOUNDED_DISC: [u8; 8] = [0x06, 0x16, 0x6C, 0x2C, 0x76, 0x7B, 0x6F, 0x0F];
const HANDLE_ALLOWED_DISC: [u8; 8] = [0xC0, 0x6D, 0xFC, 0xBF, 0xC6, 0xE0, 0x9A, 0x9A];
// Manual event decode table for PoC host-program events.

#[derive(Clone)]
pub struct SolanaRpcEventSource {
    client: Client,
    rpc_url: String,
    program_id: String,
    host_chain_id: i64,
}

impl SolanaRpcEventSource {
    pub fn new(
        rpc_url: impl Into<String>,
        program_id: impl Into<String>,
        host_chain_id: i64,
    ) -> Self {
        Self {
            client: Client::new(),
            rpc_url: rpc_url.into(),
            program_id: program_id.into(),
            host_chain_id,
        }
    }

    async fn rpc_call(&self, method: &str, params: Value) -> Result<Value> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": params
        });
        let resp = self
            .client
            .post(&self.rpc_url)
            .json(&payload)
            .send()
            .await
            .with_context(|| format!("rpc call failed: {method}"))?;
        let status = resp.status();
        let body: Value = resp
            .json()
            .await
            .with_context(|| format!("decode rpc response failed: {method}"))?;
        if !status.is_success() {
            anyhow::bail!("rpc http failure for {method}: status={status} body={body}");
        }
        if let Some(err) = body.get("error") {
            anyhow::bail!("rpc returned error for {method}: {err}");
        }
        body.get("result")
            .cloned()
            .with_context(|| format!("missing result field for rpc method: {method}"))
    }

    async fn get_slot(&self, commitment: &str) -> Result<u64> {
        let result = self
            .rpc_call("getSlot", json!([{ "commitment": commitment }]))
            .await?;
        result
            .as_u64()
            .with_context(|| format!("invalid getSlot result payload: {result}"))
    }

    async fn get_block(&self, slot: u64, commitment: &str) -> Result<Option<Value>> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getBlock",
            "params": [
                slot,
                {
                    "encoding": "json",
                    "transactionDetails": "full",
                    "rewards": false,
                    "maxSupportedTransactionVersion": 0,
                    "commitment": commitment
                }
            ]
        });
        let resp = self
            .client
            .post(&self.rpc_url)
            .json(&payload)
            .send()
            .await
            .context("rpc getBlock call failed")?;
        let status = resp.status();
        let body: Value = resp
            .json()
            .await
            .context("decode getBlock response failed")?;
        if !status.is_success() {
            anyhow::bail!("getBlock http failure status={status} body={body}");
        }
        if let Some(err) = body.get("error") {
            let code = err.get("code").and_then(Value::as_i64).unwrap_or_default();
            if code == -32007 {
                return Ok(None);
            }
            anyhow::bail!("getBlock rpc error for slot {slot}: {err}");
        }
        Ok(body.get("result").cloned())
    }
}

fn cursor_marks_completed_slot(cursor: Cursor) -> bool {
    cursor.tx_index == u32::MAX && cursor.op_index == u16::MAX
}

fn from_slot_for_cursor(cursor: Cursor) -> u64 {
    if cursor_marks_completed_slot(cursor) {
        cursor.slot.saturating_add(1)
    } else {
        cursor.slot
    }
}

fn should_skip_event(slot: u64, tx_index: u32, op_index: u16, cursor: Cursor) -> bool {
    if slot != cursor.slot {
        return false;
    }
    if tx_index < cursor.tx_index {
        return true;
    }
    tx_index == cursor.tx_index && op_index <= cursor.op_index
}

#[async_trait]
impl EventSource for SolanaRpcEventSource {
    async fn next_batch(
        &mut self,
        cursor: Cursor,
        max_batch_size: usize,
        finalized_only: bool,
    ) -> Result<SourceBatch> {
        if max_batch_size == 0 {
            return Ok(SourceBatch::empty(cursor));
        }

        let commitment = if finalized_only {
            "finalized"
        } else {
            "confirmed"
        };
        let safe_tip = self.get_slot(commitment).await?;
        let from_slot = from_slot_for_cursor(cursor);
        if from_slot > safe_tip {
            return Ok(SourceBatch::empty(cursor));
        }

        let mut events = Vec::new();
        let mut scanned_transactions = 0usize;
        let mut candidate_transactions = 0usize;
        let mut next_cursor = cursor;

        for slot in from_slot..=safe_tip {
            let Some(block) = self.get_block(slot, commitment).await? else {
                // Do not advance the cursor on unavailable slots. Skipping here
                // could silently lose events on lagging/snapshot-jumped nodes.
                warn!(slot, commitment, "block unavailable, stopping batch");
                break;
            };

            let block_time_unix = block.get("blockTime").and_then(Value::as_i64).unwrap_or(0);
            let Some(transactions) = block.get("transactions").and_then(Value::as_array) else {
                warn!(slot, "block missing transactions array, stopping batch");
                break;
            };

            let mut slot_fully_consumed = true;
            for (tx_index, tx) in transactions.iter().enumerate() {
                scanned_transactions += 1;
                let tx_index = tx_index as u32;
                if events.len() >= max_batch_size {
                    slot_fully_consumed = false;
                    break;
                }
                if slot == cursor.slot && tx_index < cursor.tx_index {
                    continue;
                }
                if tx
                    .get("meta")
                    .and_then(|meta| meta.get("err"))
                    .map(|err| !err.is_null())
                    .unwrap_or(false)
                {
                    continue;
                }
                let account_keys = extract_account_keys(tx);
                if !account_keys
                    .iter()
                    .any(|account| account == &self.program_id)
                {
                    continue;
                }
                candidate_transactions += 1;

                let Some(signature_str) = tx
                    .get("transaction")
                    .and_then(|v| v.get("signatures"))
                    .and_then(Value::as_array)
                    .and_then(|sigs| sigs.first())
                    .and_then(Value::as_str)
                else {
                    continue;
                };
                let tx_signature = match bs58::decode(signature_str).into_vec() {
                    Ok(sig) => sig,
                    Err(err) => {
                        warn!(
                            signature = signature_str,
                            ?err,
                            "failed to decode tx signature"
                        );
                        continue;
                    }
                };

                let log_messages = tx
                    .get("meta")
                    .and_then(|meta| meta.get("logMessages"))
                    .and_then(Value::as_array);
                let log_lines: Vec<&str> = log_messages
                    .map(|lines| lines.iter().filter_map(Value::as_str).collect())
                    .unwrap_or_default();
                let mut decoded_events =
                    decode_program_events_from_logs(&log_lines, &self.program_id);
                decoded_events.extend(decode_program_events_from_inner_instructions(
                    tx,
                    &self.program_id,
                    &account_keys,
                ));

                for (op_index, event) in decoded_events.into_iter().enumerate() {
                    let op_index = op_index as u16;
                    if should_skip_event(slot, tx_index, op_index, cursor) {
                        continue;
                    }
                    if events.len() >= max_batch_size {
                        slot_fully_consumed = false;
                        break;
                    }
                    events.push(FinalizedEventEnvelope {
                        version: INTERFACE_VERSION,
                        host_chain_id: self.host_chain_id,
                        slot,
                        block_time_unix,
                        tx_signature: tx_signature.clone(),
                        tx_index,
                        op_index,
                        event,
                    });
                    next_cursor = Cursor {
                        slot,
                        tx_index,
                        op_index,
                    };
                }
            }

            if events.len() >= max_batch_size {
                break;
            }

            if slot_fully_consumed {
                next_cursor = Cursor {
                    slot,
                    tx_index: u32::MAX,
                    op_index: u16::MAX,
                };
            }
        }

        debug!(
            from_slot,
            safe_tip,
            scanned_transactions,
            candidate_transactions,
            event_count = events.len(),
            next_slot = next_cursor.slot,
            next_tx_index = next_cursor.tx_index,
            next_op_index = next_cursor.op_index,
            "fetched finalized rpc batch"
        );
        Ok(SourceBatch {
            events,
            next_cursor,
        })
    }
}

fn decode_program_events_from_logs(logs: &[&str], program_id: &str) -> Vec<ProgramEvent> {
    let mut events = Vec::new();
    let mut stack: Vec<String> = Vec::new();

    for line in logs {
        if let Some(invoked_program) = parse_program_invoke(line) {
            stack.push(invoked_program.to_string());
            continue;
        }
        if let Some(exited_program) = parse_program_exit(line) {
            if let Some(pos) = stack.iter().rposition(|id| id == exited_program) {
                stack.truncate(pos);
            }
            continue;
        }

        let Some(payload_b64) = line.strip_prefix("Program data: ") else {
            continue;
        };
        if stack.last().map(String::as_str) != Some(program_id) {
            continue;
        }

        match STANDARD.decode(payload_b64) {
            Ok(payload) => {
                if let Some(event) = decode_anchor_event(&payload) {
                    events.push(event);
                }
            }
            Err(err) => {
                warn!(?err, "failed to decode Program data payload");
            }
        }
    }

    events
}

fn decode_program_events_from_inner_instructions(
    tx: &Value,
    program_id: &str,
    account_keys: &[String],
) -> Vec<ProgramEvent> {
    let mut events = Vec::new();

    let Some(inner_sets) = tx
        .get("meta")
        .and_then(|meta| meta.get("innerInstructions"))
        .and_then(Value::as_array)
    else {
        return events;
    };

    for inner_set in inner_sets {
        let Some(instructions) = inner_set.get("instructions").and_then(Value::as_array) else {
            continue;
        };

        for ix in instructions {
            let Some(ix_program_id) = extract_instruction_program_id(ix, account_keys) else {
                continue;
            };
            if ix_program_id != program_id {
                continue;
            }

            let Some(data_b58) = ix.get("data").and_then(Value::as_str) else {
                continue;
            };
            let payload = match bs58::decode(data_b58).into_vec() {
                Ok(decoded) => decoded,
                Err(err) => {
                    warn!(?err, "failed to decode inner instruction data");
                    continue;
                }
            };

            if let Some(event) = decode_anchor_event(&payload) {
                events.push(event);
                continue;
            }
            if payload.len() > 8 {
                if let Some(event) = decode_anchor_event(&payload[8..]) {
                    events.push(event);
                }
            }
        }
    }

    events
}

fn extract_account_keys(tx: &Value) -> Vec<String> {
    let Some(keys) = tx
        .get("transaction")
        .and_then(|txn| txn.get("message"))
        .and_then(|msg| msg.get("accountKeys"))
        .and_then(Value::as_array)
    else {
        return Vec::new();
    };

    keys.iter()
        .filter_map(|entry| {
            entry.as_str().map(ToOwned::to_owned).or_else(|| {
                entry
                    .get("pubkey")
                    .and_then(Value::as_str)
                    .map(ToOwned::to_owned)
            })
        })
        .collect()
}

fn extract_instruction_program_id(ix: &Value, account_keys: &[String]) -> Option<String> {
    if let Some(program_id) = ix.get("programId").and_then(Value::as_str) {
        return Some(program_id.to_string());
    }
    let index = ix.get("programIdIndex").and_then(Value::as_u64)? as usize;
    account_keys.get(index).cloned()
}

fn parse_program_invoke(line: &str) -> Option<&str> {
    let content = line.strip_prefix("Program ")?;
    let (program, _) = content.split_once(" invoke [")?;
    Some(program)
}

fn parse_program_exit(line: &str) -> Option<&str> {
    let content = line.strip_prefix("Program ")?;
    if let Some((program, _)) = content.split_once(" success") {
        return Some(program);
    }
    if let Some((program, _)) = content.split_once(" failed:") {
        return Some(program);
    }
    None
}

fn decode_anchor_event(payload: &[u8]) -> Option<ProgramEvent> {
    if payload.len() < 8 {
        return None;
    }
    let discriminator: [u8; 8] = payload[..8].try_into().ok()?;
    let body = &payload[8..];

    match discriminator {
        OP_REQUESTED_ADD_DISC => decode_op_requested_add(body),
        OP_REQUESTED_SUB_DISC => decode_op_requested_sub(body),
        OP_REQUESTED_BINARY_DISC => decode_op_requested_binary(body),
        OP_REQUESTED_UNARY_DISC => decode_op_requested_unary(body),
        OP_REQUESTED_IF_THEN_ELSE_DISC => decode_op_requested_if_then_else(body),
        OP_REQUESTED_CAST_DISC => decode_op_requested_cast(body),
        OP_REQUESTED_TRIVIAL_ENCRYPT_DISC => decode_op_requested_trivial_encrypt(body),
        OP_REQUESTED_RAND_DISC => decode_op_requested_rand(body),
        OP_REQUESTED_RAND_BOUNDED_DISC => decode_op_requested_rand_bounded(body),
        HANDLE_ALLOWED_DISC => decode_handle_allowed(body),
        _ => None,
    }
}

fn decode_op_requested_add(body: &[u8]) -> Option<ProgramEvent> {
    if body.len() < 32 + 32 + 32 + 1 + 32 {
        return None;
    }
    let caller = Pubkey::new_from_array(body[0..32].try_into().ok()?);
    let lhs: HandleBytes = body[32..64].try_into().ok()?;
    let rhs: HandleBytes = body[64..96].try_into().ok()?;
    let is_scalar = body[96] != 0;
    let result_handle: HandleBytes = body[97..129].try_into().ok()?;

    Some(ProgramEvent::OpRequestedAdd {
        caller,
        lhs,
        rhs,
        is_scalar,
        result_handle,
    })
}

fn decode_op_requested_sub(body: &[u8]) -> Option<ProgramEvent> {
    if body.len() < 32 + 32 + 32 + 1 + 32 {
        return None;
    }
    let caller = Pubkey::new_from_array(body[0..32].try_into().ok()?);
    let lhs: HandleBytes = body[32..64].try_into().ok()?;
    let rhs: HandleBytes = body[64..96].try_into().ok()?;
    let is_scalar = body[96] != 0;
    let result_handle: HandleBytes = body[97..129].try_into().ok()?;

    Some(ProgramEvent::OpRequestedSub {
        caller,
        lhs,
        rhs,
        is_scalar,
        result_handle,
    })
}

fn decode_op_requested_binary(body: &[u8]) -> Option<ProgramEvent> {
    if body.len() < 32 + 32 + 32 + 1 + 32 + 1 {
        return None;
    }
    let caller = Pubkey::new_from_array(body[0..32].try_into().ok()?);
    let lhs: HandleBytes = body[32..64].try_into().ok()?;
    let rhs: HandleBytes = body[64..96].try_into().ok()?;
    let is_scalar = body[96] != 0;
    let result_handle: HandleBytes = body[97..129].try_into().ok()?;
    let opcode = body[129];

    Some(ProgramEvent::OpRequestedBinary {
        caller,
        lhs,
        rhs,
        is_scalar,
        result_handle,
        opcode,
    })
}

fn decode_op_requested_unary(body: &[u8]) -> Option<ProgramEvent> {
    if body.len() < 32 + 32 + 32 + 1 {
        return None;
    }
    let caller = Pubkey::new_from_array(body[0..32].try_into().ok()?);
    let input: HandleBytes = body[32..64].try_into().ok()?;
    let result_handle: HandleBytes = body[64..96].try_into().ok()?;
    let opcode = body[96];

    Some(ProgramEvent::OpRequestedUnary {
        caller,
        input,
        result_handle,
        opcode,
    })
}

fn decode_op_requested_if_then_else(body: &[u8]) -> Option<ProgramEvent> {
    if body.len() < 32 + 32 + 32 + 32 + 32 {
        return None;
    }
    let caller = Pubkey::new_from_array(body[0..32].try_into().ok()?);
    let control: HandleBytes = body[32..64].try_into().ok()?;
    let if_true: HandleBytes = body[64..96].try_into().ok()?;
    let if_false: HandleBytes = body[96..128].try_into().ok()?;
    let result_handle: HandleBytes = body[128..160].try_into().ok()?;

    Some(ProgramEvent::OpRequestedIfThenElse {
        caller,
        control,
        if_true,
        if_false,
        result_handle,
    })
}

fn decode_op_requested_cast(body: &[u8]) -> Option<ProgramEvent> {
    if body.len() < 32 + 32 + 1 + 32 {
        return None;
    }
    let caller = Pubkey::new_from_array(body[0..32].try_into().ok()?);
    let input: HandleBytes = body[32..64].try_into().ok()?;
    let to_type = body[64];
    let result_handle: HandleBytes = body[65..97].try_into().ok()?;

    Some(ProgramEvent::OpRequestedCast {
        caller,
        input,
        to_type,
        result_handle,
    })
}

fn decode_op_requested_trivial_encrypt(body: &[u8]) -> Option<ProgramEvent> {
    if body.len() < 32 + 32 + 1 + 32 {
        return None;
    }
    let caller = Pubkey::new_from_array(body[0..32].try_into().ok()?);
    let pt: HandleBytes = body[32..64].try_into().ok()?;
    let to_type = body[64];
    let result_handle: HandleBytes = body[65..97].try_into().ok()?;

    Some(ProgramEvent::OpRequestedTrivialEncrypt {
        caller,
        pt,
        to_type,
        result_handle,
    })
}

fn decode_op_requested_rand(body: &[u8]) -> Option<ProgramEvent> {
    if body.len() < 32 + 1 + 32 + 32 {
        return None;
    }
    let caller = Pubkey::new_from_array(body[0..32].try_into().ok()?);
    let rand_type = body[32];
    let seed: HandleBytes = body[33..65].try_into().ok()?;
    let result_handle: HandleBytes = body[65..97].try_into().ok()?;

    Some(ProgramEvent::OpRequestedRand {
        caller,
        rand_type,
        seed,
        result_handle,
    })
}

fn decode_op_requested_rand_bounded(body: &[u8]) -> Option<ProgramEvent> {
    if body.len() < 32 + 32 + 1 + 32 + 32 {
        return None;
    }
    let caller = Pubkey::new_from_array(body[0..32].try_into().ok()?);
    let upper_bound: HandleBytes = body[32..64].try_into().ok()?;
    let rand_type = body[64];
    let seed: HandleBytes = body[65..97].try_into().ok()?;
    let result_handle: HandleBytes = body[97..129].try_into().ok()?;

    Some(ProgramEvent::OpRequestedRandBounded {
        caller,
        upper_bound,
        rand_type,
        seed,
        result_handle,
    })
}

fn decode_handle_allowed(body: &[u8]) -> Option<ProgramEvent> {
    if body.len() < 32 + 32 + 32 {
        return None;
    }
    let caller = Pubkey::new_from_array(body[0..32].try_into().ok()?);
    let handle: HandleBytes = body[32..64].try_into().ok()?;
    let account = Pubkey::new_from_array(body[64..96].try_into().ok()?);
    Some(ProgramEvent::HandleAllowed {
        caller,
        handle,
        account,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const TARGET_PROGRAM: &str = "Fg6PaFpoGXkYsidMpWxTWqkZ4FK6s7vY8J3xA5rJQbSq";

    #[test]
    fn decodes_op_requested_add_from_program_data_line() {
        let mut payload = Vec::new();
        payload.extend_from_slice(&OP_REQUESTED_ADD_DISC);
        payload.extend_from_slice(&[1u8; 32]); // caller
        payload.extend_from_slice(&[2u8; 32]); // lhs
        payload.extend_from_slice(&[3u8; 32]); // rhs
        payload.push(1u8); // is_scalar
        payload.extend_from_slice(&[4u8; 32]); // result
        let b64 = STANDARD.encode(payload);

        let logs = [
            format!("Program {TARGET_PROGRAM} invoke [1]"),
            format!("Program data: {b64}"),
            format!("Program {TARGET_PROGRAM} success"),
        ];
        let refs: Vec<&str> = logs.iter().map(String::as_str).collect();
        let events = decode_program_events_from_logs(&refs, TARGET_PROGRAM);
        assert_eq!(events.len(), 1);
        match &events[0] {
            ProgramEvent::OpRequestedAdd {
                lhs,
                rhs,
                is_scalar,
                result_handle,
                ..
            } => {
                assert_eq!(*lhs, [2u8; 32]);
                assert_eq!(*rhs, [3u8; 32]);
                assert!(*is_scalar);
                assert_eq!(*result_handle, [4u8; 32]);
            }
            _ => panic!("expected OpRequestedAdd"),
        }
    }

    #[test]
    fn ignores_program_data_when_not_in_target_program_context() {
        let payload = STANDARD.encode([0u8; 40]);
        let logs = [
            "Program Another1111111111111111111111111111111111 invoke [1]".to_string(),
            format!("Program data: {payload}"),
            "Program Another1111111111111111111111111111111111 success".to_string(),
        ];
        let refs: Vec<&str> = logs.iter().map(String::as_str).collect();
        let events = decode_program_events_from_logs(&refs, TARGET_PROGRAM);
        assert!(events.is_empty());
    }

    #[test]
    fn decodes_event_from_cpi_inner_instruction_payload() {
        let mut payload = vec![0xFF; 8];
        payload.extend_from_slice(&OP_REQUESTED_ADD_DISC);
        payload.extend_from_slice(&[1u8; 32]); // caller
        payload.extend_from_slice(&[2u8; 32]); // lhs
        payload.extend_from_slice(&[3u8; 32]); // rhs
        payload.push(0u8); // is_scalar
        payload.extend_from_slice(&[4u8; 32]); // result

        let tx = json!({
            "transaction": {
                "message": {
                    "accountKeys": [TARGET_PROGRAM]
                }
            },
            "meta": {
                "innerInstructions": [{
                    "index": 0,
                    "instructions": [{
                        "programIdIndex": 0,
                        "data": bs58::encode(payload).into_string()
                    }]
                }]
            }
        });

        let account_keys = extract_account_keys(&tx);
        let events =
            decode_program_events_from_inner_instructions(&tx, TARGET_PROGRAM, &account_keys);
        assert_eq!(events.len(), 1);
        match &events[0] {
            ProgramEvent::OpRequestedAdd {
                lhs,
                rhs,
                is_scalar,
                result_handle,
                ..
            } => {
                assert_eq!(*lhs, [2u8; 32]);
                assert_eq!(*rhs, [3u8; 32]);
                assert!(!*is_scalar);
                assert_eq!(*result_handle, [4u8; 32]);
            }
            _ => panic!("expected OpRequestedAdd"),
        }
    }

    #[test]
    fn decodes_op_requested_sub_from_program_data_line() {
        let mut payload = Vec::new();
        payload.extend_from_slice(&OP_REQUESTED_SUB_DISC);
        payload.extend_from_slice(&[1u8; 32]); // caller
        payload.extend_from_slice(&[5u8; 32]); // lhs
        payload.extend_from_slice(&[6u8; 32]); // rhs
        payload.push(1u8); // is_scalar
        payload.extend_from_slice(&[7u8; 32]); // result
        let b64 = STANDARD.encode(payload);

        let logs = [
            format!("Program {TARGET_PROGRAM} invoke [1]"),
            format!("Program data: {b64}"),
            format!("Program {TARGET_PROGRAM} success"),
        ];
        let refs: Vec<&str> = logs.iter().map(String::as_str).collect();
        let events = decode_program_events_from_logs(&refs, TARGET_PROGRAM);
        assert_eq!(events.len(), 1);
        match &events[0] {
            ProgramEvent::OpRequestedSub {
                lhs,
                rhs,
                is_scalar,
                result_handle,
                ..
            } => {
                assert_eq!(*lhs, [5u8; 32]);
                assert_eq!(*rhs, [6u8; 32]);
                assert!(*is_scalar);
                assert_eq!(*result_handle, [7u8; 32]);
            }
            _ => panic!("expected OpRequestedSub"),
        }
    }

    #[test]
    fn decodes_op_requested_binary_from_program_data_line() {
        let mut payload = Vec::new();
        payload.extend_from_slice(&OP_REQUESTED_BINARY_DISC);
        payload.extend_from_slice(&[1u8; 32]); // caller
        payload.extend_from_slice(&[8u8; 32]); // lhs
        payload.extend_from_slice(&[9u8; 32]); // rhs
        payload.push(0u8); // is_scalar
        payload.extend_from_slice(&[10u8; 32]); // result
        payload.push(2u8); // mul opcode
        let b64 = STANDARD.encode(payload);

        let logs = [
            format!("Program {TARGET_PROGRAM} invoke [1]"),
            format!("Program data: {b64}"),
            format!("Program {TARGET_PROGRAM} success"),
        ];
        let refs: Vec<&str> = logs.iter().map(String::as_str).collect();
        let events = decode_program_events_from_logs(&refs, TARGET_PROGRAM);
        assert_eq!(events.len(), 1);
        match &events[0] {
            ProgramEvent::OpRequestedBinary {
                lhs,
                rhs,
                is_scalar,
                result_handle,
                opcode,
                ..
            } => {
                assert_eq!(*lhs, [8u8; 32]);
                assert_eq!(*rhs, [9u8; 32]);
                assert!(!*is_scalar);
                assert_eq!(*result_handle, [10u8; 32]);
                assert_eq!(*opcode, 2u8);
            }
            _ => panic!("expected OpRequestedBinary"),
        }
    }

    #[test]
    fn decodes_op_requested_unary_from_program_data_line() {
        let mut payload = Vec::new();
        payload.extend_from_slice(&OP_REQUESTED_UNARY_DISC);
        payload.extend_from_slice(&[1u8; 32]); // caller
        payload.extend_from_slice(&[11u8; 32]); // input
        payload.extend_from_slice(&[12u8; 32]); // result
        payload.push(20u8); // neg opcode
        let b64 = STANDARD.encode(payload);

        let logs = [
            format!("Program {TARGET_PROGRAM} invoke [1]"),
            format!("Program data: {b64}"),
            format!("Program {TARGET_PROGRAM} success"),
        ];
        let refs: Vec<&str> = logs.iter().map(String::as_str).collect();
        let events = decode_program_events_from_logs(&refs, TARGET_PROGRAM);
        assert_eq!(events.len(), 1);
        match &events[0] {
            ProgramEvent::OpRequestedUnary {
                input,
                result_handle,
                opcode,
                ..
            } => {
                assert_eq!(*input, [11u8; 32]);
                assert_eq!(*result_handle, [12u8; 32]);
                assert_eq!(*opcode, 20u8);
            }
            _ => panic!("expected OpRequestedUnary"),
        }
    }

    #[test]
    fn cursor_slot_math_skips_only_fully_consumed_slots() {
        let consumed = Cursor {
            slot: 100,
            tx_index: u32::MAX,
            op_index: u16::MAX,
        };
        let partial = Cursor {
            slot: 100,
            tx_index: 4,
            op_index: 2,
        };
        assert_eq!(from_slot_for_cursor(consumed), 101);
        assert_eq!(from_slot_for_cursor(partial), 100);
    }

    #[test]
    fn skip_rule_is_slot_local_and_position_aware() {
        let cursor = Cursor {
            slot: 42,
            tx_index: 2,
            op_index: 3,
        };
        assert!(should_skip_event(42, 1, 9, cursor));
        assert!(should_skip_event(42, 2, 3, cursor));
        assert!(!should_skip_event(42, 2, 4, cursor));
        assert!(!should_skip_event(43, 0, 0, cursor));
    }
}
