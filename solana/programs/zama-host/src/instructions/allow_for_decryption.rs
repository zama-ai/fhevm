//! Enables public decrypt access on canonical handle records.

use anchor_lang::prelude::*;

use super::common::*;
use crate::{events::PublicDecryptAllowedEvent, state::*};

/// Accounts for enabling public decrypt on a handle.
#[derive(Accounts)]
#[event_cpi]
pub struct AllowForDecryption<'info> {
    /// Subject that must have `ACL_ROLE_PUBLIC_DECRYPT`.
    pub authority: Signer<'info>,
    /// Optional overflow permission witness when `authority` is not inline.
    pub authority_permission_record: Option<UncheckedAccount<'info>>,
    /// Canonical ACL record whose public-decrypt flag is updated.
    #[account(mut)]
    pub acl_record: Account<'info, AclRecord>,
    /// Singleton config PDA.
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    /// CHECK: required when grant_deny_list_enabled; may be uninitialized.
    pub deny_subject_record: Option<UncheckedAccount<'info>>,
}

/// Marks a handle as publicly decryptable after role and deny-list checks.
pub fn allow_for_decryption(ctx: Context<AllowForDecryption>, handle: [u8; 32]) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_not_paused(&ctx.accounts.host_config)?;
    let authority = ctx.accounts.authority.key();
    let record_key = ctx.accounts.acl_record.key();
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
        record_key,
        handle,
        authority,
        ACL_ROLE_PUBLIC_DECRYPT,
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
    if !ctx.accounts.acl_record.public_decrypt {
        let clock = Clock::get()?;
        ctx.accounts.acl_record.public_decrypt = true;
        #[cfg(feature = "emit-events")]
        emit_cpi!(PublicDecryptAllowedEvent {
            version: EVENT_VERSION,
            acl_record: record_key,
            handle,
            authority: authority.to_bytes(),
            updated_slot: clock.slot,
        });
    }
    Ok(())
}
