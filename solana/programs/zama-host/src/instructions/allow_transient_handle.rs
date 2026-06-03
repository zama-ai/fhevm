//! Grants same-slot transient use of durable encrypted handles.

use anchor_lang::prelude::*;

use super::common::*;
use crate::state::*;

/// Accounts for granting transient use of an existing durable handle.
#[derive(Accounts)]
pub struct AllowTransientHandle<'info> {
    /// Subject that must already be allowed to use the durable handle.
    pub authority: Signer<'info>,
    /// Optional overflow permission witness when `authority` is not inline.
    pub authority_permission_record: Option<UncheckedAccount<'info>>,
    /// Canonical ACL record proving durable access to `handle`.
    pub acl_record: Account<'info, AclRecord>,
    /// Open transient session receiving the capability.
    #[account(
        mut,
        seeds = [TRANSIENT_SESSION_SEED, session.authority.as_ref(), session.session_nonce.as_ref()],
        bump = session.bump
    )]
    pub session: Account<'info, TransientSession>,
    /// Singleton config PDA.
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    /// CHECK: required when grant_deny_list_enabled; may be uninitialized.
    pub deny_subject_record: Option<UncheckedAccount<'info>>,
}

/// Adds a transient capability for an existing durable handle.
pub fn allow_transient_handle(
    ctx: Context<AllowTransientHandle>,
    handle: [u8; 32],
    capability: TransientCapabilityGrant,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_not_paused(&ctx.accounts.host_config)?;
    let authority = ctx.accounts.authority.key();
    assert_canonical_acl_record(
        &ctx.accounts.acl_record.to_account_info(),
        &ctx.accounts.acl_record,
    )?;
    assert_acl_record_handle_for_chain(
        &ctx.accounts.acl_record,
        ctx.accounts.host_config.chain_id,
    )?;
    assert_record_subject_role(
        &ctx.accounts.acl_record,
        ctx.accounts.acl_record.key(),
        handle,
        authority,
        ACL_ROLE_USE,
        ctx.accounts
            .authority_permission_record
            .as_ref()
            .map(|account| account.to_account_info())
            .as_ref(),
    )?;
    check_grant_not_denied(
        &ctx.accounts.host_config,
        authority,
        ctx.accounts.deny_subject_record.as_ref(),
    )?;
    assert_transient_session_account(
        &ctx.accounts.session.to_account_info(),
        &ctx.accounts.session,
    )?;

    let clock = Clock::get()?;
    let session_key = ctx.accounts.session.key();
    append_transient_capability_to_session(
        session_key,
        &mut ctx.accounts.session,
        authority,
        clock.slot,
        handle,
        capability,
    )
}
