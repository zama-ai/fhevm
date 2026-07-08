//! Sets the per-app, per-slot HCU block cap enforced inside `fhe_eval`.
//!
//! Sentinels (revised during invariants): `u64::MAX` = unrestricted (the ship default; the cap
//! short-circuits and touches no meter), `0` = a deliberate ban of untrusted apps (trusted apps
//! still bypass), and any other value is the metering band. This inverts the `max_hcu_per_tx`
//! `0 = unlimited` convention on purpose, so an admin can express a hard ban and the default
//! deploy stays neutral.

use anchor_lang::prelude::*;

use super::common::*;
use super::set_host_pause::HostAdmin;

/// Sets `hcu_block_cap_per_app`. Admin-gated.
///
/// Enforced guarantees:
/// - The admin must sign and match `host_config.admin` (`assert_admin`).
/// - Rejects any trailing accounts (`assert_no_remaining_accounts`).
/// - Idempotent: setting the current value is a no-op and does not advance `updated_slot`.
/// - In the metering band (`0 < value < u64::MAX`), the cap must stay at or above `max_hcu_per_tx`
///   (unless that is `0` = unlimited), so a single legal frame is never structurally impossible.
///   The two sentinels (`0` = ban, `u64::MAX` = unrestricted) are exempt (`check_block_cap_ordering`).
/// - Advances `updated_slot` and emits the config-updated event carrying the new cap.
pub fn set_hcu_block_cap_per_app(ctx: Context<HostAdmin>, value: u64) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_admin(&ctx.accounts.host_config, ctx.accounts.admin.key())?;
    if ctx.accounts.host_config.hcu_block_cap_per_app == value {
        return Ok(());
    }
    let admin = ctx.accounts.admin.key();
    let config = &mut ctx.accounts.host_config;
    check_block_cap_ordering(value, config.max_hcu_per_tx)?;
    config.hcu_block_cap_per_app = value;
    config.updated_slot = Clock::get()?.slot;
    emit_config_updated(config, admin);
    Ok(())
}
