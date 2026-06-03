//! Toggles deny-list witnesses for persistent grant authorities.

use anchor_lang::prelude::*;

use super::common::*;
use super::set_host_pause::HostAdmin;

/// Enables or disables deny-list witnesses for persistent grant authorities.
pub fn set_grant_deny_list_enabled(ctx: Context<HostAdmin>, enabled: bool) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_admin(&ctx.accounts.host_config, ctx.accounts.admin.key())?;
    if ctx.accounts.host_config.grant_deny_list_enabled == enabled {
        return Ok(());
    }
    ctx.accounts.host_config.grant_deny_list_enabled = enabled;
    ctx.accounts.host_config.updated_slot = Clock::get()?.slot;
    emit_config_updated(&ctx.accounts.host_config, ctx.accounts.admin.key());
    Ok(())
}
