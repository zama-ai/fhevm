//! Sets the per-`fhe_eval` total HCU limit (mirrors EVM `setMaxHCUPerTx`).

use anchor_lang::prelude::*;

use super::common::*;
use super::set_host_pause::HostAdmin;

/// Sets `max_hcu_per_tx`. Admin-gated. `0` = unlimited (enforcement off).
///
/// Enforced guarantees:
/// - The admin must sign and match `host_config.admin` (`assert_admin`).
/// - Rejects any trailing accounts (`assert_no_remaining_accounts`).
/// - Preserves the `max_hcu_per_tx >= max_hcu_depth_per_tx` ordering, with `0` = unlimited
///   (`check_hcu_ordering`).
/// - Advances `updated_slot` and emits the config-updated event carrying the new limits.
pub fn set_max_hcu_per_tx(ctx: Context<HostAdmin>, value: u64) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_admin(&ctx.accounts.host_config, ctx.accounts.admin.key())?;
    let admin = ctx.accounts.admin.key();
    let config = &mut ctx.accounts.host_config;
    // The new total must not fall below the current depth limit (0 = unlimited on either side).
    check_hcu_ordering(value, config.max_hcu_depth_per_tx)?;
    config.max_hcu_per_tx = value;
    config.updated_slot = Clock::get()?.slot;
    emit_config_updated(config, admin);
    Ok(())
}
