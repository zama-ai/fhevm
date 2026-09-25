//! Resumes host areas: the admin only, like EVM `ACL.unpause` (`onlyOwner`).

use anchor_lang::prelude::*;

use super::common::*;
use super::host_admin::HostAdmin;
use crate::state::PauseFlags;

/// Clears the pause flag of every area `areas` names.
pub fn unpause(ctx: Context<HostAdmin>, areas: PauseFlags) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_admin(&ctx.accounts.host_config, &ctx.accounts.admin)?;
    let config = &mut ctx.accounts.host_config;
    let paused = config.paused.without(areas);
    if paused == config.paused {
        return Ok(());
    }
    config.paused = paused;
    config.updated_slot = Clock::get()?.slot;
    emit_config_updated(
        config,
        ctx.accounts.admin.key(),
        &ctx.accounts.event_authority,
    )
}
