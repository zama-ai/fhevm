//! Sets the per-transaction total HCU limit (mirrors EVM `setMaxHCUPerTx`).
//!
//! The total sums every `fhe_execute` in the transaction: they all charge the one transient store
//! the transaction's final instruction closes, as EVM's transient counters cover every call.

use anchor_lang::prelude::*;

use super::common::*;
use super::host_admin::HostAdmin;

/// Sets `max_hcu_per_tx`. Admin-gated. `u64::MAX` = unlimited (enforcement off); `0` is
/// rejected so "off" has exactly one spelling across every HCU knob.
///
/// Enforced guarantees:
/// - The admin must sign and match `host_config.admin` (`assert_admin`).
/// - Rejects any trailing accounts (`assert_no_remaining_accounts`).
/// - Preserves the `max_hcu_per_tx >= max_hcu_depth_per_tx` ordering, with `u64::MAX` = unlimited
///   (`check_hcu_ordering`).
/// - Preserves the block-cap ordering from the other side: a metering-band
///   `hcu_block_cap_per_app` must stay at or above the new total, so raising the per-transaction
///   limit cannot silently make a single legal execution exceed the block cap
///   (`check_block_cap_ordering`).
/// - Emits the config-updated event carrying the new limits.
pub fn set_max_hcu_per_tx(ctx: Context<HostAdmin>, value: u64) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_admin(&ctx.accounts.host_config, &ctx.accounts.admin)?;
    require!(
        value != 0,
        crate::errors::ZamaHostError::HcuLimitZeroReserved
    );
    if ctx.accounts.host_config.max_hcu_per_tx == value {
        return Ok(());
    }
    let admin = ctx.accounts.admin.key();
    let config = &mut ctx.accounts.host_config;
    // The new total must not fall below the current depth limit (u64::MAX = unlimited).
    check_hcu_ordering(value, config.max_hcu_depth_per_tx)?;
    // And a metering-band block cap must not fall below the new total (sentinels exempt).
    check_block_cap_ordering(config.hcu_block_cap_per_app, value)?;
    config.max_hcu_per_tx = value;
    emit_config_updated(
        &ctx.accounts.host_config,
        admin,
        &ctx.accounts.event_authority,
    )?;
    Ok(())
}
