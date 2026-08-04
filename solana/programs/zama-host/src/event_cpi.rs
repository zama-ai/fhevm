//! The one event-CPI emitter this program uses. Every event ZamaHost emits goes through here; see
//! `events.rs` for which events are emitted and DD-044 for why the ones that are not, are not.
//!
//! This is a hand expansion of `anchor_lang::emit_cpi!`: same `EVENT_IX_TAG_LE`, same `Event::data`
//! payload, same single readonly-signer event authority, same `invoke_signed` seeds. It is written out
//! for two reasons. The macro hardcodes the identifier `ctx`, so it cannot be called from a shared
//! helper like `emit_config_updated`, which six config-update instructions route through. And having
//! the `Instruction` exist as a value is what lets a unit test assert on the emitted shape without a
//! runtime — which is the only thing that checks that shape, so it matters more than it sounds.
//!
//! Only the assembly is ours; the tag and payload encoding come from anchor-lang and track upstream. If
//! the assembly drifts, the runtime catches less than you would hope. A wrong tag fails the
//! transaction, because Anchor's `dispatch` routes on it and an unrouted instruction hits the fallback.
//! Past that, the generated `__event_dispatch` checks that the *first* account is a signer and is the
//! canonical event authority, and nothing else: it never reads the event data and ignores any account
//! after the first. An extra account, or a changed payload encoding, would reach no check at all. The
//! tests are what pin it — `event_transport.rs`'s assertion on the built `Instruction`, and
//! `host_mollusk.rs`'s `sole_emitted_event`, which reads an event back out of the inner instructions.
//! Between them they cover `PublicOutputsProducedEvent` and `NewKmsContextEvent`. Keep it that way: if
//! those stop being covered, this becomes an unchecked copy of an upstream wire format.

use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::{AccountMeta, Instruction},
    program::invoke_signed,
};

/// The self-CPI instruction carrying `event`: the event tag, then the event's own discriminator and
/// borsh body, addressed to this program with the event authority as its lone readonly signer.
///
/// One deliberate divergence from `emit_cpi!`, which takes the meta's pubkey from the passed account:
/// this takes it from the constant. `#[event_cpi]` pins that account with an `address` constraint, so
/// they are the same key, and a mismatch would fail `invoke_signed` rather than emit to the wrong PDA.
pub(crate) fn event_cpi_instruction<T: anchor_lang::Event>(event: &T) -> Instruction {
    let data = anchor_lang::event::EVENT_IX_TAG_LE
        .iter()
        .copied()
        .chain(anchor_lang::Event::data(event))
        .collect::<Vec<_>>();
    Instruction::new_with_bytes(
        crate::ID,
        &data,
        vec![AccountMeta::new_readonly(
            crate::EVENT_AUTHORITY_AND_BUMP.0,
            true,
        )],
    )
}

/// Emits `event` as an inner instruction. `event_authority` is the account `#[event_cpi]` adds to the
/// instruction's accounts, already pinned to `EVENT_AUTHORITY_AND_BUMP.0` by an `address` constraint.
pub(crate) fn emit_event_cpi<T: anchor_lang::Event>(
    event_authority: &AccountInfo<'_>,
    event: &T,
) -> Result<()> {
    invoke_signed(
        &event_cpi_instruction(event),
        std::slice::from_ref(event_authority),
        &[&[b"__event_authority", &[crate::EVENT_AUTHORITY_AND_BUMP.1]]],
    )?;
    Ok(())
}
