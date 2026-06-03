//! Seals transient sessions so existing capabilities can be consumed.

use anchor_lang::prelude::*;

use super::common::*;
use crate::{errors::ZamaHostError, state::*};

/// Accounts for sealing a transient session.
#[derive(Accounts)]
pub struct SealTransientSession<'info> {
    /// Session authority.
    pub authority: Signer<'info>,
    /// Host-owned transient session PDA.
    #[account(
        mut,
        seeds = [TRANSIENT_SESSION_SEED, session.authority.as_ref(), session.session_nonce.as_ref()],
        bump = session.bump
    )]
    pub session: Account<'info, TransientSession>,
    /// Singleton config PDA.
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
}

/// Seals a transient session so capabilities can be consumed but not appended.
pub fn seal_transient_session(ctx: Context<SealTransientSession>) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_not_paused(&ctx.accounts.host_config)?;
    assert_transient_session_account(
        &ctx.accounts.session.to_account_info(),
        &ctx.accounts.session,
    )?;
    require_keys_eq!(
        ctx.accounts.session.authority,
        ctx.accounts.authority.key(),
        ZamaHostError::TransientSessionAuthorityMismatch
    );
    require!(
        ctx.accounts.session.state == TRANSIENT_SESSION_STATE_OPEN,
        ZamaHostError::TransientSessionStateInvalid
    );
    let clock = Clock::get()?;
    require!(
        clock.slot <= ctx.accounts.session.expires_slot,
        ZamaHostError::TransientSessionExpired
    );
    ctx.accounts.session.state = TRANSIENT_SESSION_STATE_SEALED;
    Ok(())
}
