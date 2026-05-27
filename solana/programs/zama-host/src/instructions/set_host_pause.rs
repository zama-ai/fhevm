use anchor_lang::prelude::*;

use super::common::*;

/// Updates the host pause flag.
pub fn set_host_pause(ctx: Context<HostAdmin>, paused: bool) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_admin(&ctx.accounts.host_config, ctx.accounts.admin.key())?;
    if ctx.accounts.host_config.paused == paused {
        return Ok(());
    }
    ctx.accounts.host_config.paused = paused;
    ctx.accounts.host_config.updated_slot = Clock::get()?.slot;
    emit_config_updated(&ctx.accounts.host_config, ctx.accounts.admin.key());
    Ok(())
}
