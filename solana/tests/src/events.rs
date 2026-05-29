//! Collect typed protocol events from `emit_cpi!` inner instructions.
//!
//! Decoders are generated from the checked-in ZamaHost Anchor IDL in the shared
//! `zama-host-events` crate (same path as `host-listener`).

use anchor_lang::{AnchorDeserialize, Discriminator, Event};
use litesvm::types::TransactionMetadata;
use solana_sdk::pubkey::Pubkey;
pub use zama_host_events::{
    decode_anchor_cpi_event, AclAllowedEvent, AclPublicDecryptAllowedEvent, FheBinaryOpEvent,
    FheRandEvent, InputVerifiedEvent, TrivialEncryptEvent, ZamaHostEvent, ANCHOR_EVENT_IX_TAG_LE,
};

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
    collect_cpi_events(meta, account_keys, program_id, decode_anchor_cpi_event)
}

/// Generates a `<name>(meta, account_keys, program_id) -> Vec<$ty>` that returns
/// the ZamaHost CPI events of one variant. The fn names stay greppable.
macro_rules! zama_host_event_filter {
    ($name:ident, $variant:ident, $ty:ty) => {
        pub fn $name(
            meta: &TransactionMetadata,
            account_keys: &[Pubkey],
            program_id: Pubkey,
        ) -> Vec<$ty> {
            collect_zama_host_events(meta, account_keys, program_id)
                .into_iter()
                .filter_map(|event| match event {
                    ZamaHostEvent::$variant(event) => Some(event),
                    _ => None,
                })
                .collect()
        }
    };
}

zama_host_event_filter!(binary_op_events, FheBinaryOp, FheBinaryOpEvent);
zama_host_event_filter!(trivial_encrypt_events, TrivialEncrypt, TrivialEncryptEvent);
zama_host_event_filter!(fhe_rand_events, FheRand, FheRandEvent);
zama_host_event_filter!(acl_allowed_events, AclAllowed, AclAllowedEvent);
zama_host_event_filter!(
    acl_public_decrypt_allowed_events,
    AclPublicDecryptAllowed,
    AclPublicDecryptAllowedEvent
);
zama_host_event_filter!(input_verified_events, InputVerified, InputVerifiedEvent);

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
        .filter(|event| {
            matches!(
                event,
                ZamaHostEvent::AclAllowed(_) | ZamaHostEvent::AclPublicDecryptAllowed(_)
            )
        })
        .count()
}

/// App-level token events (`confidential-token` `emit_cpi!`).
pub fn decode_token_cpi_event(
    data: &[u8],
) -> Option<confidential_token::BalanceHandleUpdatedEvent> {
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
