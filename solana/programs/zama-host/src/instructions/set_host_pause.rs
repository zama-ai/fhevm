//! Toggles the host-wide pause flag.

use anchor_lang::prelude::*;

use super::common::*;
use crate::state::{HostConfig, HOST_CONFIG_SEED};

/// Shared account context for admin-only config updates.
///
/// The generated Anchor account type is intentionally still named `HostAdmin`
/// because runtime tests and callers already construct that account builder.
#[derive(Accounts)]
pub struct HostAdmin<'info> {
    /// Configured host admin.
    pub admin: Signer<'info>,
    /// Singleton config PDA.
    #[account(mut, seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
}

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
