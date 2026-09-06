//! Toggles the deny list: a denied application `(program, scope)` cannot compute, allow, or make a
//! handle public.

use anchor_lang::prelude::*;

use super::common::*;
use super::set_host_pause::HostAdmin;

/// Enables or disables the deny list (`HostConfig::grant_deny_list_enabled`).
pub fn set_grant_deny_list_enabled(ctx: Context<HostAdmin>, enabled: bool) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_admin(&ctx.accounts.host_config, &ctx.accounts.admin)?;
    if ctx.accounts.host_config.grant_deny_list_enabled == enabled {
        return Ok(());
    }
    ctx.accounts.host_config.grant_deny_list_enabled = enabled;
    ctx.accounts.host_config.updated_slot = Clock::get()?.slot;
    emit_config_updated(
        &ctx.accounts.host_config,
        ctx.accounts.admin.key(),
        &ctx.accounts.event_authority,
    )?;
    Ok(())
}
