//! Removes one current subject from an `EncryptedValue` allowed set.

use anchor_lang::prelude::*;

use super::common::*;
use crate::{errors::ZamaHostError, state::*};

/// Accounts for `remove_subject`.
#[derive(Accounts)]
pub struct RemoveEncryptedValueSubject<'info> {
    /// Current allowed subject on the encrypted value account.
    pub authority: Signer<'info>,
    /// CHECK: layout and ownership are validated inside the handler via `read_canonical_encrypted_value`.
    #[account(mut)]
    pub encrypted_value: UncheckedAccount<'info>,
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    pub deny_subject_record: Option<UncheckedAccount<'info>>,
}

pub fn remove_subject(ctx: Context<RemoveEncryptedValueSubject>, subject: Pubkey) -> Result<()> {
    assert_not_paused(&ctx.accounts.host_config)?;

    let info = ctx.accounts.encrypted_value.to_account_info();
    let mut value = read_canonical_encrypted_value(&info)?;
    let authority = ctx.accounts.authority.key();
    require!(
        value.has_subject(authority),
        ZamaHostError::SubjectNotAllowed
    );
    check_grant_not_denied(
        &ctx.accounts.host_config,
        authority,
        ctx.accounts.deny_subject_record.as_ref(),
    )?;

    let subject_index = value
        .subject_index(subject)
        .ok_or_else(|| error!(ZamaHostError::SubjectNotFound))?;
    require!(
        value.subjects.len() > 1,
        ZamaHostError::EncryptedValueLastSubject
    );
    value.subjects.remove(subject_index);
    write_account(&info, &value)?;
    Ok(())
}
