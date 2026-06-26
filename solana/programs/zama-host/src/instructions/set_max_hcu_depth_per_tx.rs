//! Sets the per-`fhe_eval` critical-path (depth) HCU limit (mirrors EVM `setMaxHCUDepthPerTx`).

use anchor_lang::prelude::*;

use super::common::*;
use super::set_host_pause::HostAdmin;

/// Sets `max_hcu_depth_per_tx`. Admin-gated. `0` = unlimited (enforcement off).
///
/// # Invariants enforced
/// - INV-2 / INV-4: the admin must sign and match `host_config.admin` (`assert_admin`).
/// - INV-5: rejects any trailing accounts (`assert_no_remaining_accounts`).
/// - INV-7 / INV-15: preserves `max_hcu_per_tx >= max_hcu_depth_per_tx`, `0` = unlimited
///   (`check_hcu_ordering`).
/// - INV-17: advances `updated_slot` and emits the config-updated event carrying the new limits.
pub fn set_max_hcu_depth_per_tx(ctx: Context<HostAdmin>, value: u64) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_admin(&ctx.accounts.host_config, ctx.accounts.admin.key())?;
    let admin = ctx.accounts.admin.key();
    let config = &mut ctx.accounts.host_config;
    // INV-7: the new depth must not exceed the current total limit (0 = unlimited on either side).
    check_hcu_ordering(config.max_hcu_per_tx, value)?;
    config.max_hcu_depth_per_tx = value;
    config.updated_slot = Clock::get()?.slot;
    emit_config_updated(config, admin);
    Ok(())
}
