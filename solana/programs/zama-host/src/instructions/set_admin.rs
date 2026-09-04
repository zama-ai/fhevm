//! Transfers the host admin key.

use anchor_lang::prelude::*;

use super::common::*;
use crate::errors::ZamaHostError;
use crate::state::{HostConfig, HOST_CONFIG_SEED};

/// Accounts for transferring the host admin.
#[derive(Accounts)]
#[event_cpi]
pub struct SetAdmin<'info> {
    /// Current configured host admin.
    pub admin: Signer<'info>,
    /// Singleton config PDA.
    #[account(mut, seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    /// The incoming admin. A keypair must co-sign (`is_signer`); a PDA skips co-sign
    /// (`!is_on_curve()`).
    /// CHECK: key must equal `new_admin`; signer-or-PDA rule is enforced in the handler.
    pub new_admin: UncheckedAccount<'info>,
}

/// Sets `host_config.admin` to `new_admin`. Current admin signs. A new keypair co-signs;
/// a new PDA does not.
pub fn set_admin(ctx: Context<SetAdmin>, new_admin: Pubkey) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_admin(&ctx.accounts.host_config, &ctx.accounts.admin)?;
    require_keys_eq!(
        ctx.accounts.new_admin.key(),
        new_admin,
        ZamaHostError::HostConfigAdminMismatch
    );
    if !ctx.accounts.new_admin.is_signer {
        require!(
            !ctx.accounts.new_admin.key().is_on_curve(),
            ZamaHostError::HostConfigAdminMismatch
        );
    }
    if ctx.accounts.host_config.admin == new_admin {
        return Ok(());
    }
    let signer = ctx.accounts.admin.key();
    ctx.accounts.host_config.admin = new_admin;
    ctx.accounts.host_config.updated_slot = Clock::get()?.slot;
    emit_config_updated(
        &ctx.accounts.host_config,
        signer,
        &ctx.accounts.event_authority,
    )?;
    Ok(())
}
