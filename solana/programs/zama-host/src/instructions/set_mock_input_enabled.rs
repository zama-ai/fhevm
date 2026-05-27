use anchor_lang::prelude::*;

use super::common::*;

/// Enables or disables the mock encrypted-input bind path.
pub fn set_mock_input_enabled(ctx: Context<HostAdmin>, enabled: bool) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_admin(&ctx.accounts.host_config, ctx.accounts.admin.key())?;
    if ctx.accounts.host_config.mock_input_enabled == enabled {
        return Ok(());
    }
    ctx.accounts.host_config.mock_input_enabled = enabled;
    ctx.accounts.host_config.updated_slot = Clock::get()?.slot;
    emit_config_updated(&ctx.accounts.host_config, ctx.accounts.admin.key());
    Ok(())
}
