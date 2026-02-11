use anchor_lang::{AnchorDeserialize, Discriminator};
use anyhow::{Context, Result};
use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD, Engine};
use solana_client::{
    client_error::{ClientError, ClientErrorKind},
    nonblocking::rpc_client::RpcClient,
    rpc_config::RpcBlockConfig,
    rpc_config::{TransactionDetails, UiTransactionEncoding},
    rpc_custom_error::{
        JSON_RPC_SERVER_ERROR_BLOCK_NOT_AVAILABLE,
        JSON_RPC_SERVER_ERROR_BLOCK_STATUS_NOT_AVAILABLE_YET, JSON_RPC_SERVER_ERROR_SLOT_SKIPPED,
    },
    rpc_request::RpcError,
    rpc_response::{OptionSerializer, UiConfirmedBlock},
};
use solana_commitment_config::CommitmentConfig;
use tracing::{debug, warn};
use zama_host::{
    HandleAllowed, OpRequestedAdd, OpRequestedBinary, OpRequestedCast, OpRequestedIfThenElse,
    OpRequestedRand, OpRequestedRandBounded, OpRequestedSub, OpRequestedTrivialEncrypt,
    OpRequestedUnary,
};

use crate::contracts::{FinalizedEventEnvelope, ProgramEvent, INTERFACE_VERSION};
use crate::poller::{Cursor, EventSource, SourceBatch};

pub struct SolanaRpcEventSource {
    rpc: RpcClient,
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
            rpc: RpcClient::new(rpc_url.into()),
            program_id: program_id.into(),
            host_chain_id,
        }
    }

    async fn get_slot(&self, commitment: CommitmentConfig) -> Result<u64> {
        self.rpc
            .get_slot_with_commitment(commitment)
            .await
            .context("rpc getSlot failed")
    }

    async fn get_block(
        &self,
        slot: u64,
        commitment: CommitmentConfig,
    ) -> Result<Option<UiConfirmedBlock>> {
        let config = RpcBlockConfig {
            encoding: Some(UiTransactionEncoding::Base64),
            transaction_details: Some(TransactionDetails::Full),
            rewards: Some(false),
            commitment: Some(commitment),
            max_supported_transaction_version: Some(0),
        };

        match self.rpc.get_block_with_config(slot, config).await {
            Ok(block) => Ok(Some(block)),
            Err(err) if is_block_temporarily_unavailable(&err) => Ok(None),
            Err(err) => Err(err).with_context(|| format!("rpc getBlock failed for slot {slot}")),
        }
    }
}

fn is_block_temporarily_unavailable(err: &ClientError) -> bool {
    matches!(
        err.kind(),
        ClientErrorKind::RpcError(RpcError::RpcResponseError { code, .. })
            if *code == JSON_RPC_SERVER_ERROR_BLOCK_NOT_AVAILABLE
                || *code == JSON_RPC_SERVER_ERROR_SLOT_SKIPPED
                || *code == JSON_RPC_SERVER_ERROR_BLOCK_STATUS_NOT_AVAILABLE_YET
    )
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
            CommitmentConfig::finalized()
        } else {
            CommitmentConfig::confirmed()
        };
        let commitment_label = if finalized_only {
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
                // Do not advance cursor on unavailable blocks to prevent silent event loss.
                warn!(
                    slot,
                    commitment = commitment_label,
                    "block unavailable, stopping batch"
                );
                break;
            };

            let block_time_unix = block.block_time.unwrap_or(0);
            let block_hash = decode_base58_32("blockhash", &block.blockhash)?;
            let Some(transactions) = block.transactions else {
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
                    .meta
                    .as_ref()
                    .and_then(|meta| meta.err.as_ref())
                    .is_some()
                {
                    continue;
                }

                let Some(decoded_tx) = tx.transaction.decode() else {
                    warn!(slot, tx_index, "failed to decode transaction payload");
                    continue;
                };
                let Some(signature) = decoded_tx.signatures.first() else {
                    continue;
                };
                let tx_signature = signature.as_ref().to_vec();

                let log_lines: Vec<&str> = tx
                    .meta
                    .as_ref()
                    .and_then(|meta| match meta.log_messages.as_ref() {
                        OptionSerializer::Some(lines) => {
                            Some(lines.iter().map(String::as_str).collect::<Vec<&str>>())
                        }
                        _ => None,
                    })
                    .unwrap_or_default();

                let decoded_events = decode_program_events_from_logs(&log_lines, &self.program_id);
                if decoded_events.is_empty() {
                    continue;
                }
                candidate_transactions += 1;

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
                        block_hash: block_hash.clone(),
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
            "fetched rpc batch"
        );
        Ok(SourceBatch {
            events,
            next_cursor,
        })
    }
}

fn decode_base58_32(label: &str, value: &str) -> Result<Vec<u8>> {
    let decoded = bs58::decode(value)
        .into_vec()
        .with_context(|| format!("decode {label}: {value}"))?;
    anyhow::ensure!(
        decoded.len() == 32,
        "unexpected {label} length: {}",
        decoded.len()
    );
    Ok(decoded)
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
    if let Some(event) = try_decode_event::<OpRequestedAdd>(payload) {
        return Some(ProgramEvent::OpRequestedAdd {
            caller: event.caller,
            lhs: event.lhs,
            rhs: event.rhs,
            is_scalar: event.is_scalar,
            result_handle: event.result_handle,
        });
    }

    if let Some(event) = try_decode_event::<OpRequestedSub>(payload) {
        return Some(ProgramEvent::OpRequestedSub {
            caller: event.caller,
            lhs: event.lhs,
            rhs: event.rhs,
            is_scalar: event.is_scalar,
            result_handle: event.result_handle,
        });
    }

    if let Some(event) = try_decode_event::<OpRequestedBinary>(payload) {
        return Some(ProgramEvent::OpRequestedBinary {
            caller: event.caller,
            lhs: event.lhs,
            rhs: event.rhs,
            is_scalar: event.is_scalar,
            result_handle: event.result_handle,
            opcode: event.opcode,
        });
    }

    if let Some(event) = try_decode_event::<OpRequestedUnary>(payload) {
        return Some(ProgramEvent::OpRequestedUnary {
            caller: event.caller,
            input: event.input,
            result_handle: event.result_handle,
            opcode: event.opcode,
        });
    }

    if let Some(event) = try_decode_event::<OpRequestedIfThenElse>(payload) {
        return Some(ProgramEvent::OpRequestedIfThenElse {
            caller: event.caller,
            control: event.control,
            if_true: event.if_true,
            if_false: event.if_false,
            result_handle: event.result_handle,
        });
    }

    if let Some(event) = try_decode_event::<OpRequestedCast>(payload) {
        return Some(ProgramEvent::OpRequestedCast {
            caller: event.caller,
            input: event.input,
            to_type: event.to_type,
            result_handle: event.result_handle,
        });
    }

    if let Some(event) = try_decode_event::<OpRequestedTrivialEncrypt>(payload) {
        return Some(ProgramEvent::OpRequestedTrivialEncrypt {
            caller: event.caller,
            pt: event.pt,
            to_type: event.to_type,
            result_handle: event.result_handle,
        });
    }

    if let Some(event) = try_decode_event::<OpRequestedRand>(payload) {
        return Some(ProgramEvent::OpRequestedRand {
            caller: event.caller,
            rand_type: event.rand_type,
            seed: event.seed,
            result_handle: event.result_handle,
        });
    }

    if let Some(event) = try_decode_event::<OpRequestedRandBounded>(payload) {
        return Some(ProgramEvent::OpRequestedRandBounded {
            caller: event.caller,
            upper_bound: event.upper_bound,
            rand_type: event.rand_type,
            seed: event.seed,
            result_handle: event.result_handle,
        });
    }

    if let Some(event) = try_decode_event::<HandleAllowed>(payload) {
        return Some(ProgramEvent::HandleAllowed {
            caller: event.caller,
            handle: event.handle,
            account: event.account,
        });
    }

    None
}

fn try_decode_event<T>(payload: &[u8]) -> Option<T>
where
    T: AnchorDeserialize + Discriminator,
{
    let discriminator = T::DISCRIMINATOR;
    let body = payload.strip_prefix(discriminator)?;
    T::try_from_slice(body).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::Event;

    const TARGET_PROGRAM: &str = "Fg6PaFpoGXkYsidMpWxTWqkZ4FK6s7vY8J3xA5rJQbSq";

    #[test]
    fn decodes_op_requested_add_from_program_data_line() {
        let payload = OpRequestedAdd {
            caller: anchor_lang::prelude::Pubkey::new_from_array([1u8; 32]),
            lhs: [2u8; 32],
            rhs: [3u8; 32],
            is_scalar: true,
            result_handle: [4u8; 32],
        }
        .data();
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
    fn decodes_op_requested_sub_from_program_data_line() {
        let payload = OpRequestedSub {
            caller: anchor_lang::prelude::Pubkey::new_from_array([1u8; 32]),
            lhs: [5u8; 32],
            rhs: [6u8; 32],
            is_scalar: true,
            result_handle: [7u8; 32],
        }
        .data();
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
        let payload = OpRequestedBinary {
            caller: anchor_lang::prelude::Pubkey::new_from_array([1u8; 32]),
            lhs: [8u8; 32],
            rhs: [9u8; 32],
            is_scalar: false,
            result_handle: [10u8; 32],
            opcode: 2,
        }
        .data();
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
        let payload = OpRequestedUnary {
            caller: anchor_lang::prelude::Pubkey::new_from_array([1u8; 32]),
            input: [11u8; 32],
            result_handle: [12u8; 32],
            opcode: 20,
        }
        .data();
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
