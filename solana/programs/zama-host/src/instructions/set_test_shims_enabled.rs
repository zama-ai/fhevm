//! Toggles the local PoC zero-birth-entropy fallback.

use anchor_lang::prelude::*;

use super::common::*;
use super::set_host_pause::HostAdmin;

/// Enables or disables the local PoC zero-birth-entropy fallback.
pub fn set_test_shims_enabled(ctx: Context<HostAdmin>, enabled: bool) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_admin(&ctx.accounts.host_config, ctx.accounts.admin.key())?;
    if ctx.accounts.host_config.test_shims_enabled == enabled {
        return Ok(());
    }
    ctx.accounts.host_config.test_shims_enabled = enabled;
    ctx.accounts.host_config.updated_slot = Clock::get()?.slot;
    emit_config_updated(&ctx.accounts.host_config, ctx.accounts.admin.key());
    Ok(())
}
