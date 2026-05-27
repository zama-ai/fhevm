use anchor_lang::prelude::*;

use super::common::*;
use crate::{errors::ZamaHostError, state::*};

/// Accounts for creating a one-slot transient capability session.
#[derive(Accounts)]
#[instruction(session_nonce: [u8; 32])]
pub struct CreateTransientSession<'info> {
    /// Pays rent for the transient session account.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Session authority. This signer is required again for append, consume,
    /// seal, and pre-expiry close.
    pub authority: Signer<'info>,
    /// Host-owned transient session PDA.
    #[account(
        init,
        payer = payer,
        space = 8 + TransientSession::SPACE,
        seeds = [TRANSIENT_SESSION_SEED, authority.key().as_ref(), session_nonce.as_ref()],
        bump
    )]
    pub session: Account<'info, TransientSession>,
    /// Singleton config PDA.
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    /// System program used for session account creation.
    pub system_program: Program<'info, System>,
}

/// Creates an open transient session that expires in the current slot.
pub fn create_transient_session(
    ctx: Context<CreateTransientSession>,
    session_nonce: [u8; 32],
    refund_recipient: Pubkey,
    compute_subject: Pubkey,
    expires_slot: u64,
    // Retained in the instruction ABI so clients can pass the intended capacity.
    // The host currently accepts only the one-shot capacity.
    max_entries: u8,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_not_paused(&ctx.accounts.host_config)?;
    require!(
        session_nonce != [0; 32],
        ZamaHostError::InvalidTransientSessionNonce
    );
    require!(
        max_entries == MAX_TRANSIENT_CAPABILITIES as u8,
        ZamaHostError::TransientSessionCapacityInvalid
    );
    require_keys_neq!(
        refund_recipient,
        Pubkey::default(),
        ZamaHostError::TransientSessionRefundMismatch
    );
    require_keys_neq!(
        compute_subject,
        Pubkey::default(),
        ZamaHostError::TransientCapabilityUnauthorized
    );

    let clock = Clock::get()?;
    require!(
        expires_slot == clock.slot,
        ZamaHostError::TransientSessionExpired
    );

    ctx.accounts.session.session_nonce = session_nonce;
    ctx.accounts.session.authority = ctx.accounts.authority.key();
    ctx.accounts.session.refund_recipient = refund_recipient;
    ctx.accounts.session.compute_subject = compute_subject;
    ctx.accounts.session.created_slot = clock.slot;
    ctx.accounts.session.expires_slot = expires_slot;
    ctx.accounts.session.state = TRANSIENT_SESSION_STATE_OPEN;
    ctx.accounts.session.max_entries = max_entries;
    ctx.accounts.session.entries = Vec::new();
    ctx.accounts.session.bump = ctx.bumps.session;
    Ok(())
}
