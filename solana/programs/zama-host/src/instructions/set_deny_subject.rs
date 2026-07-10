//! Creates and updates grant deny-list records.

use anchor_lang::prelude::*;

use super::common::*;
#[cfg(feature = "emit-events")]
use crate::events::DenySubjectUpdatedEvent;
use crate::{errors::ZamaHostError, state::*};

/// Accounts for creating or updating a deny-list record.
#[derive(Accounts)]
pub struct SetDenySubject<'info> {
    /// Pays rent if the deny-list PDA must be created.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Configured host admin.
    pub admin: Signer<'info>,
    /// Singleton config PDA.
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    /// CHECK: created or overwritten after canonical deny-list PDA validation.
    #[account(mut)]
    pub deny_subject_record: UncheckedAccount<'info>,
    /// System program used for account creation.
    pub system_program: Program<'info, System>,
}

/// Creates or updates the deny-list state for `subject`.
pub fn set_deny_subject(ctx: Context<SetDenySubject>, subject: Pubkey, denied: bool) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_admin(&ctx.accounts.host_config, ctx.accounts.admin.key())?;
    let (expected, bump) = deny_subject_address(subject);
    require_keys_eq!(
        expected,
        ctx.accounts.deny_subject_record.key(),
        ZamaHostError::AclDenyRecordMismatch
    );

    let info = ctx.accounts.deny_subject_record.to_account_info();
    let current = current_deny_status(&info, subject, bump)?;
    if current.unwrap_or(false) == denied {
        return Ok(());
    }

    create_pda_if_needed(
        &ctx.accounts.payer.to_account_info(),
        &info,
        &ctx.accounts.system_program.to_account_info(),
        8 + DenySubjectRecord::SPACE,
        &[DENY_SUBJECT_SEED, subject.as_ref(), &[bump]],
    )?;

    write_account(
        &info,
        &DenySubjectRecord {
            subject,
            denied,
            bump,
        },
    )?;
    #[cfg(feature = "emit-events")]
    emit!(DenySubjectUpdatedEvent {
        version: EVENT_VERSION,
        deny_subject_record: ctx.accounts.deny_subject_record.key(),
        subject,
        denied,
        updated_slot: Clock::get()?.slot,
    });
    Ok(())
}

fn current_deny_status(info: &AccountInfo, subject: Pubkey, bump: u8) -> Result<Option<bool>> {
    if is_absent_deny_record(info)? {
        return Ok(None);
    }
    require_keys_eq!(*info.owner, crate::ID, ZamaHostError::AclDenyRecordMismatch);
    require!(
        info.data_len() == 8 + DenySubjectRecord::SPACE,
        ZamaHostError::AclDenyRecordMismatch
    );
    let data = info.try_borrow_data()?;
    let mut data_slice: &[u8] = &data;
    let record = DenySubjectRecord::try_deserialize(&mut data_slice)?;
    require_keys_eq!(
        record.subject,
        subject,
        ZamaHostError::AclDenyRecordMismatch
    );
    require!(record.bump == bump, ZamaHostError::AclDenyRecordMismatch);
    Ok(Some(record.denied))
}
