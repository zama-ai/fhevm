//! Nominates the next host admin.

use anchor_lang::prelude::*;

use super::common::*;
use super::host_admin::HostAdmin;

/// Sets `host_config.pending_admin` to `new_admin`, replacing any earlier nomination;
/// `Pubkey::default()` withdraws it. The admin changes only when the nominee calls `accept_admin`.
pub fn set_admin(ctx: Context<HostAdmin>, new_admin: Pubkey) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_admin(&ctx.accounts.host_config, &ctx.accounts.admin)?;
    if ctx.accounts.host_config.pending_admin == new_admin {
        return Ok(());
    }
    let signer = ctx.accounts.admin.key();
    ctx.accounts.host_config.pending_admin = new_admin;
    emit_config_updated(
        &ctx.accounts.host_config,
        signer,
        &ctx.accounts.event_authority,
    )?;
    Ok(())
}
