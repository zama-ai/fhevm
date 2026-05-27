use anchor_lang::prelude::*;

use super::common::*;
use crate::state::AclRecord;

/// Accounts for verifying that an ACL record matches app metadata.
#[derive(Accounts)]
pub struct AssertAclRecord<'info> {
    /// Canonical ACL record expected to store the requested handle and subject.
    pub acl_record: Account<'info, AclRecord>,
    /// Optional overflow permission witness when `subject` is not stored inline.
    pub subject_permission_record: Option<UncheckedAccount<'info>>,
}

/// Checks ACL record canonicality, metadata, handle, and use membership.
pub fn assert_acl_record(
    ctx: Context<AssertAclRecord>,
    nonce_key: [u8; 32],
    nonce_sequence: u64,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    handle: [u8; 32],
    subject: Pubkey,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_record(
        &ctx.accounts.acl_record.to_account_info(),
        &ctx.accounts.acl_record,
        nonce_key,
        nonce_sequence,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        handle,
        subject,
        ctx.accounts
            .subject_permission_record
            .as_ref()
            .map(|account| account.to_account_info())
            .as_ref(),
    )?;
    Ok(())
}
