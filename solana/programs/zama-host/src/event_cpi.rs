//! The one event-CPI emitter this program uses.
//!
//! Every event ZamaHost emits goes through here, and none of them go through `emit!`. A logged event
//! can be truncated by the RPC provider an indexer reads through, so it is only ever a hint: anything
//! an off-chain component must be able to query has to survive the trip. Event CPI does, because the
//! payload lands in the transaction's inner instructions rather than its logs, and only this program
//! can sign the authority PDA that carries it. Everything else is not emitted at all — the listener
//! reconstructs it from instruction data over Yellowstone, which is the normal path (see
//! `events.rs`).
//!
//! This is a hand expansion of `anchor_lang::emit_cpi!`, and a faithful one: same `EVENT_IX_TAG_LE`,
//! same `Event::data` payload, same single readonly-signer event authority, same `invoke_signed`
//! seeds. It is written out rather than invoked for two reasons. The macro hardcodes the identifier
//! `ctx`, so it cannot be called from a shared helper like `emit_config_updated`, which six
//! config-update instructions route through — using it would mean copying a ten-field event literal
//! into each of them. And having the `Instruction` exist as a value is what lets a unit test assert
//! on the emitted shape without a runtime, which is where the `fhe_execute` CPI-data bound is checked.
//!
//! Only the *assembly* is ours. The tag and the payload encoding come from anchor-lang, so they track
//! upstream automatically; what would not track upstream is a change to the shape itself (an extra
//! account, a different order). That is caught at runtime rather than by the compiler: each self-CPI
//! re-enters this program through Anchor's generated `__event_dispatch`, which checks the tag and the
//! authority signer, so any Mollusk test that runs an emitting instruction against the real SBF
//! artifact fails on drift.

use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::{AccountMeta, Instruction},
    program::invoke_signed,
};

/// The self-CPI instruction carrying `event`: the event tag, then the event's own discriminator and
/// borsh body, addressed to this program with the event authority as its lone readonly signer.
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
