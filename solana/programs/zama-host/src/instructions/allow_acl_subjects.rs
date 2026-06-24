//! Extends persistent ACL membership on canonical handle records.

use anchor_lang::prelude::*;

use super::common::*;
use crate::{
    events::{AclAllowedEvent, AclSubjectAllowedEvent},
    state::*,
};

/// Accounts for extending persistent ACL membership.
#[derive(Accounts)]
#[event_cpi]
pub struct AllowAclSubjects<'info> {
    /// Pays rent for any overflow permission PDAs.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Subject that must already have `ACL_ROLE_GRANT`.
    pub authority: Signer<'info>,
    /// Optional overflow permission witness when `authority` is not inline.
    pub authority_permission_record: Option<UncheckedAccount<'info>>,
    /// Canonical ACL record to extend.
    #[account(mut)]
    pub acl_record: Account<'info, AclRecord>,
    /// Singleton config PDA.
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    /// CHECK: required when grant_deny_list_enabled; may be uninitialized.
    pub deny_subject_record: Option<UncheckedAccount<'info>>,
    /// System program used for overflow permission creation.
    pub system_program: Program<'info, System>,
}

/// Grants one or more subjects on an existing canonical ACL record.
///
/// Subjects beyond the inline record capacity are written to canonical
/// `AclPermission` PDAs supplied through `remaining_accounts`.
pub fn allow_acl_subjects<'info>(
    ctx: Context<'info, AllowAclSubjects<'info>>,
    handle: [u8; 32],
    subjects: Vec<AclSubjectEntry>,
) -> Result<()> {
    assert_not_paused(&ctx.accounts.host_config)?;
    let authority = ctx.accounts.authority.key();
    let record_key = ctx.accounts.acl_record.key();
    assert_canonical_acl_record(
        &ctx.accounts.acl_record.to_account_info(),
        &ctx.accounts.acl_record,
    )?;
    assert_record_subject_role(
        &ctx.accounts.acl_record,
        record_key,
        handle,
        authority,
        ACL_ROLE_GRANT,
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

    let allowed_subjects = extend_acl_subjects(
        &ctx.accounts.payer.to_account_info(),
        &ctx.accounts.system_program.to_account_info(),
        record_key,
        &mut ctx.accounts.acl_record,
        &subjects,
        ctx.remaining_accounts,
    )?;
    let updated_slot = Clock::get()?.slot;
    for update in allowed_subjects {
        #[cfg(feature = "emit-events")]
        emit_cpi!(AclAllowedEvent {
            version: EVENT_VERSION,
            handle,
            subject: update.subject.pubkey.to_bytes(),
        });
        #[cfg(feature = "emit-events")]
        emit!(AclSubjectAllowedEvent {
            version: EVENT_VERSION,
            acl_record: record_key,
            handle,
            authority_subject: authority,
            subject: update.subject.pubkey.to_bytes(),
            role_flags: update.subject.role_flags,
            overflow_permission_record: update.permission_record,
            inline_index: update.inline_index,
            updated_slot,
        });
    }
    Ok(())
}
