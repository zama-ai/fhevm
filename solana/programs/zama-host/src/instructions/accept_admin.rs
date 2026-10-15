//! Hands the host admin role to the nominee `set_admin` named.

use anchor_lang::prelude::*;

use super::common::*;
use crate::errors::ZamaHostError;
use crate::state::{HostConfig, HOST_CONFIG_SEED};

/// Accounts for accepting the host admin role.
#[derive(Accounts)]
#[event_cpi]
pub struct AcceptAdmin<'info> {
    /// The nominee, `HostConfig.pending_admin`.
    pub pending_admin: Signer<'info>,
    /// Singleton config PDA.
    #[account(mut, seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
}

/// Makes the signer the admin and clears the nomination. No one signs for `Pubkey::default()`,
/// so with no nomination every signer is refused.
pub fn accept_admin(ctx: Context<AcceptAdmin>) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    let signer = ctx.accounts.pending_admin.key();
    require_keys_eq!(
        ctx.accounts.host_config.pending_admin,
        signer,
        ZamaHostError::PendingAdminMismatch
    );
    let config = &mut ctx.accounts.host_config;
    config.admin = signer;
    config.pending_admin = Pubkey::default();
    emit_config_updated(
        &ctx.accounts.host_config,
        signer,
        &ctx.accounts.event_authority,
    )?;
    Ok(())
}
