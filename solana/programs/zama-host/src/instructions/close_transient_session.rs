//! Closes transient capability sessions and returns rent.

use anchor_lang::prelude::*;

use super::common::*;
use crate::{errors::ZamaHostError, state::*};

/// Accounts for closing a transient session and refunding rent.
#[derive(Accounts)]
pub struct CloseTransientSession<'info> {
    /// Optional session authority. Required before expiry; omitted for
    /// permissionless post-expiry cleanup.
    pub authority: Option<Signer<'info>>,
    /// Host-owned transient session PDA.
    #[account(
        mut,
        seeds = [TRANSIENT_SESSION_SEED, session.authority.as_ref(), session.session_nonce.as_ref()],
        bump = session.bump
    )]
    pub session: Account<'info, TransientSession>,
    /// Stored refund recipient.
    /// CHECK: Key is checked against session data and then receives lamports.
    #[account(mut)]
    pub refund_recipient: UncheckedAccount<'info>,
    /// Singleton config PDA.
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
}

/// Closes a transient session.
pub fn close_transient_session(ctx: Context<CloseTransientSession>) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    // Closing only reclaims rent from an existing transient session. It does not
    // authorize new FHE use, so cleanup stays available during an emergency pause.
    assert_transient_session_account(
        &ctx.accounts.session.to_account_info(),
        &ctx.accounts.session,
    )?;
    require_keys_eq!(
        ctx.accounts.refund_recipient.key(),
        ctx.accounts.session.refund_recipient,
        ZamaHostError::TransientSessionRefundMismatch
    );

    let clock = Clock::get()?;
    if clock.slot <= ctx.accounts.session.expires_slot {
        let authority = ctx
            .accounts
            .authority
            .as_ref()
            .ok_or(ZamaHostError::TransientSessionAuthorityMismatch)?;
        require_keys_eq!(
            authority.key(),
            ctx.accounts.session.authority,
            ZamaHostError::TransientSessionAuthorityMismatch
        );
    }

    ctx.accounts
        .session
        .close(ctx.accounts.refund_recipient.to_account_info())
}
