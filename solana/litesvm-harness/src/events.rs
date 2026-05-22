//! Collect typed protocol events from `emit_cpi!` inner instructions.
//!
//! This matches the ingestion path used by `host-listener` (inner CPI bytes with
//! `ANCHOR_EVENT_IX_TAG_LE`), not log-based `emit!` / `msg!` parsing.

use anchor_lang::{AnchorDeserialize, Discriminator, Event};
use litesvm::types::TransactionMetadata;
use solana_sdk::pubkey::Pubkey;
use zama_host as host;

/// Anchor self-CPI event instruction tag (LE). Same constant as `host-listener` codegen.
pub const ANCHOR_EVENT_IX_TAG_LE: [u8; 8] = 0x1d9acb512ea545e4_u64.to_le_bytes();

/// Typed ZamaHost events decoded from inner instruction data.
pub enum ZamaHostEvent {
    FheBinaryOp(host::FheBinaryOpEvent),
    TrivialEncrypt(host::TrivialEncryptEvent),
    FheRand(host::FheRandEvent),
    AclAllowed(host::AclAllowedEvent),
    InputVerified(host::InputVerifiedEvent),
}

/// Decode one Anchor `emit_cpi!` payload for a ZamaHost event.
pub fn decode_zama_host_cpi_event(data: &[u8]) -> Option<ZamaHostEvent> {
    decode_cpi_event::<host::FheBinaryOpEvent>(data).map(ZamaHostEvent::FheBinaryOp)
        .or_else(|| decode_cpi_event::<host::TrivialEncryptEvent>(data).map(ZamaHostEvent::TrivialEncrypt))
        .or_else(|| decode_cpi_event::<host::FheRandEvent>(data).map(ZamaHostEvent::FheRand))
        .or_else(|| decode_cpi_event::<host::AclAllowedEvent>(data).map(ZamaHostEvent::AclAllowed))
        .or_else(|| {
            decode_cpi_event::<host::InputVerifiedEvent>(data).map(ZamaHostEvent::InputVerified)
        })
}

/// Walk inner instructions and decode CPI event payloads with `decode`.
pub fn collect_cpi_events<T>(
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
    program_id: Pubkey,
    decode: impl Fn(&[u8]) -> Option<T>,
) -> Vec<T> {
    meta.inner_instructions
        .iter()
        .flatten()
        .filter(|ix| *ix.instruction.program_id(account_keys) == program_id)
        .filter_map(|ix| decode(&ix.instruction.data))
        .collect()
}

/// Walk `meta.inner_instructions` and return all ZamaHost CPI events for `program_id`.
pub fn collect_zama_host_events(
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
    program_id: Pubkey,
) -> Vec<ZamaHostEvent> {
    collect_cpi_events(meta, account_keys, program_id, decode_zama_host_cpi_event)
}

pub fn binary_op_events(
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
    program_id: Pubkey,
) -> Vec<host::FheBinaryOpEvent> {
    collect_zama_host_events(meta, account_keys, program_id)
        .into_iter()
        .filter_map(|event| match event {
            ZamaHostEvent::FheBinaryOp(event) => Some(event),
            _ => None,
        })
        .collect()
}

pub fn trivial_encrypt_events(
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
    program_id: Pubkey,
) -> Vec<host::TrivialEncryptEvent> {
    collect_zama_host_events(meta, account_keys, program_id)
        .into_iter()
        .filter_map(|event| match event {
            ZamaHostEvent::TrivialEncrypt(event) => Some(event),
            _ => None,
        })
        .collect()
}

pub fn acl_allowed_events(
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
    program_id: Pubkey,
) -> Vec<host::AclAllowedEvent> {
    collect_zama_host_events(meta, account_keys, program_id)
        .into_iter()
        .filter_map(|event| match event {
            ZamaHostEvent::AclAllowed(event) => Some(event),
            _ => None,
        })
        .collect()
}

pub fn count_tfhe_host_events(events: &[ZamaHostEvent]) -> usize {
    events
        .iter()
        .filter(|event| {
            matches!(
                event,
                ZamaHostEvent::FheBinaryOp(_)
                    | ZamaHostEvent::TrivialEncrypt(_)
                    | ZamaHostEvent::FheRand(_)
            )
        })
        .count()
}

pub fn count_acl_allowed_events(events: &[ZamaHostEvent]) -> usize {
    events
        .iter()
        .filter(|event| matches!(event, ZamaHostEvent::AclAllowed(_)))
        .count()
}

/// App-level token events (`confidential-token` `emit_cpi!`).
pub fn decode_token_cpi_event(data: &[u8]) -> Option<confidential_token::BalanceHandleUpdatedEvent> {
    decode_cpi_event::<confidential_token::BalanceHandleUpdatedEvent>(data)
}

pub fn balance_handle_updated_events(
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
    program_id: Pubkey,
) -> Vec<confidential_token::BalanceHandleUpdatedEvent> {
    collect_cpi_events(meta, account_keys, program_id, decode_token_cpi_event)
}

pub fn max_cpi_depth(meta: &TransactionMetadata) -> u64 {
    meta.logs
        .iter()
        .filter_map(|log| {
            log.strip_suffix(']')?
                .rsplit_once(" invoke [")?
                .1
                .parse::<u64>()
                .ok()
        })
        .max()
        .unwrap_or(1)
}

fn decode_cpi_event<T>(data: &[u8]) -> Option<T>
where
    T: AnchorDeserialize + Discriminator + Event,
{
    let rest = data.strip_prefix(&ANCHOR_EVENT_IX_TAG_LE)?;
    if rest.len() < 8 || rest[..8] != *T::DISCRIMINATOR {
        return None;
    }
    T::deserialize(&mut &rest[8..]).ok()
}
