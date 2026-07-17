//! Stateful per-app, per-slot HCU block cap for [`super::fhe_eval`].
//!
//! Unlike the pure per-frame meter in [`super::hcu`], the block cap touches accounts (the per-app
//! `HcuBlockMeter`) and a sysvar-derived slot, so it lives here and runs from the admission /
//! execution phases rather than inside the pure walk.
//!
//! Two passes share one resolution path so they trip identically (single `Clock` slot, reused
//! `frame_total`, same cap read from the read-only `host_config`):
//! - [`check`] — read-only, in admission: reads the meter (or treats a not-yet-created meter as
//!   `0`), asserts `used + frame_total <= cap`. No write, no lazy-create. An over-budget frame
//!   therefore fails before execution burns CU or creates any ACL record.
//! - [`charge`] — the single meter write, in execution: lazy-creates or lazy-resets, accumulates
//!   with checked arithmetic (overflow fails closed), asserts the cap again, and writes once.
//!
//! Cap sentinels: `u64::MAX` = unrestricted (short-circuit, touch nothing), `0` = ban untrusted
//! apps (trusted still bypass), otherwise the metering band.
//!
//! The metered identity is the `compute_subject` — the mandatory signed caller identity already
//! used for ACL admission (the `msg.sender` analog), never `payer` and never `app_account_authority`
//! (an output-ACL role that degenerates to a per-user key). Metering the already-signed
//! `compute_subject` means no caller can rotate a fresh signer to mint a fresh per-slot meter: the
//! same subject that authorizes the frame's encrypted inputs is the one whose usage accumulates.

use anchor_lang::prelude::*;

use super::super::common::{create_pda_if_needed, is_uninitialized_pda_account, write_account};
use super::FheEval;
use crate::errors::ZamaHostError;
use crate::state::{
    hcu_block_meter_address, hcu_trusted_app_address, HcuBlockMeter, HcuTrustedAppRecord,
    HCU_BLOCK_METER_SEED,
};

/// Read-only admission pass: no write, no lazy-create.
pub(super) fn check<'info>(
    ctx: &Context<'info, FheEval<'info>>,
    frame_total: u64,
    slot: u64,
) -> Result<()> {
    let cap = ctx.accounts.host_config.hcu_block_cap_per_app;
    // Unrestricted (ship default): short-circuit, touching neither optional account. Zero contention.
    if cap == u64::MAX {
        return Ok(());
    }
    let app = ctx.accounts.compute_subject.key();
    // A well-formed trusted witness bypasses the cap entirely — even under a ban.
    if resolve_trusted(ctx, app)? {
        return Ok(());
    }
    // Ban: every untrusted frame is rejected before the meter is even required.
    if cap == 0 {
        return Err(error!(ZamaHostError::HcuBlockLimitExceeded));
    }
    // Metering band: an untrusted app must supply its meter (fail closed, never silently un-metered).
    let meter = ctx
        .accounts
        .hcu_block_meter
        .as_ref()
        .ok_or(ZamaHostError::HcuBlockMeterMissing)?;
    let (expected, bump) = hcu_block_meter_address(app);
    let used = meter_used_for_slot(&meter.to_account_info(), app, expected, bump, slot)?;
    let projected = used
        .checked_add(frame_total)
        .ok_or(ZamaHostError::HcuBlockLimitExceeded)?;
    require!(projected <= cap, ZamaHostError::HcuBlockLimitExceeded);
    Ok(())
}

/// Execution pass: the single meter write. Mirrors [`check`]'s resolution, then persists.
pub(super) fn charge<'info>(
    ctx: &Context<'info, FheEval<'info>>,
    frame_total: u64,
    slot: u64,
) -> Result<()> {
    let cap = ctx.accounts.host_config.hcu_block_cap_per_app;
    if cap == u64::MAX {
        return Ok(());
    }
    let app = ctx.accounts.compute_subject.key();
    if resolve_trusted(ctx, app)? {
        return Ok(());
    }
    if cap == 0 {
        return Err(error!(ZamaHostError::HcuBlockLimitExceeded));
    }
    let meter = ctx
        .accounts
        .hcu_block_meter
        .as_ref()
        .ok_or(ZamaHostError::HcuBlockMeterMissing)?;
    let info = meter.to_account_info();
    let (expected, bump) = hcu_block_meter_address(app);
    let used = meter_used_for_slot(&info, app, expected, bump, slot)?;
    let projected = used
        .checked_add(frame_total)
        .ok_or(ZamaHostError::HcuBlockLimitExceeded)?;
    require!(projected <= cap, ZamaHostError::HcuBlockLimitExceeded);

    // Lazy-create on the first metered frame (noop once program-owned). A pre-squatted, non-empty
    // account at the meter PDA fails here rather than being adopted.
    create_pda_if_needed(
        &ctx.accounts.payer.to_account_info(),
        &info,
        &ctx.accounts.system_program.to_account_info(),
        8 + HcuBlockMeter::SPACE,
        &[HCU_BLOCK_METER_SEED, app.as_ref(), &[bump]],
    )?;
    write_account(
        &info,
        &HcuBlockMeter {
            app,
            last_seen_slot: slot,
            used_hcu: projected,
            bump,
        },
    )?;
    Ok(())
}

/// Resolves the trust witness for `app`:
/// - `None` / absent (system-owned + empty) ⇒ untrusted (benign — fall through to the meter);
/// - present, program-owned, canonical PDA, `trusted == true` ⇒ bypass;
/// - present, program-owned, canonical PDA, `trusted == false` ⇒ untrusted (fall through);
/// - present but wrong PDA / owner / layout ⇒ `HcuTrustedAppRecordMismatch`.
fn resolve_trusted<'info>(ctx: &Context<'info, FheEval<'info>>, app: Pubkey) -> Result<bool> {
    let Some(witness) = ctx.accounts.hcu_trusted_app_record.as_ref() else {
        return Ok(false);
    };
    let info = witness.to_account_info();
    let (expected, expected_bump) = hcu_trusted_app_address(app);
    require_keys_eq!(
        info.key(),
        expected,
        ZamaHostError::HcuTrustedAppRecordMismatch
    );
    // Present but never created is benign: the app is simply untrusted (metered).
    if is_uninitialized_pda_account(&info)? {
        return Ok(false);
    }
    require_keys_eq!(
        *info.owner,
        crate::ID,
        ZamaHostError::HcuTrustedAppRecordMismatch
    );
    require!(
        info.data_len() == 8 + HcuTrustedAppRecord::SPACE,
        ZamaHostError::HcuTrustedAppRecordMismatch
    );
    let data = info.try_borrow_data()?;
    let mut data_slice: &[u8] = &data;
    let record = HcuTrustedAppRecord::try_deserialize(&mut data_slice)?;
    require!(
        record.bump == expected_bump,
        ZamaHostError::HcuTrustedAppRecordMismatch
    );
    require_keys_eq!(record.app, app, ZamaHostError::HcuTrustedAppRecordMismatch);
    Ok(record.trusted)
}

/// Effective in-slot usage from a possibly-uninitialized meter. The supplied account must always be
/// the canonical meter PDA — checked first, so a misaddressed meter is rejected cheaply in the
/// read-only admission pass rather than only when `charge` tries to create it. A non-program-owned
/// account (uninitialized / system-owned / squatted) is then treated as `0` — `charge`'s lazy-create
/// rejects a squatter, and `check` never writes. A program-owned meter is validated
/// (owner / length / recorded app / bump) before any field is read, then lazy-reset: a
/// `last_seen_slot` other than the current slot means `0` for this frame.
fn meter_used_for_slot(
    info: &AccountInfo,
    app: Pubkey,
    expected: Pubkey,
    bump: u8,
    slot: u64,
) -> Result<u64> {
    require_keys_eq!(info.key(), expected, ZamaHostError::HcuBlockMeterMismatch);
    if info.owner != &crate::ID {
        return Ok(0);
    }
    require!(
        info.data_len() == 8 + HcuBlockMeter::SPACE,
        ZamaHostError::HcuBlockMeterMismatch
    );
    let data = info.try_borrow_data()?;
    let mut data_slice: &[u8] = &data;
    let record = HcuBlockMeter::try_deserialize(&mut data_slice)?;
    require_keys_eq!(record.app, app, ZamaHostError::HcuBlockMeterMismatch);
    require!(record.bump == bump, ZamaHostError::HcuBlockMeterMismatch);
    Ok(if record.last_seen_slot != slot {
        0
    } else {
        record.used_hcu
    })
}
