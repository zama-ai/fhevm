//! Creates and updates pauser records: the admin manages the pauser set, like EVM
//! `PauserSet.addPauser` and `removePauser` (`onlyACLOwner`).

use anchor_lang::prelude::*;

use super::common::*;
use crate::event_cpi::emit_event_cpi;
use crate::events::PauserUpdatedEvent;
use crate::{errors::ZamaHostError, state::*};

/// Accounts for creating or updating a pauser record.
#[derive(Accounts)]
#[event_cpi]
pub struct SetPauser<'info> {
    /// Pays rent if the pauser PDA must be created.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Configured host admin.
    pub admin: Signer<'info>,
    /// Singleton config PDA.
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    /// CHECK: created or overwritten after canonical pauser PDA validation.
    #[account(mut)]
    pub pauser_record: UncheckedAccount<'info>,
    /// System program used for account creation.
    pub system_program: Program<'info, System>,
}

/// Grants or withdraws `pauser`'s right to set pause flags.
pub fn set_pauser(ctx: Context<SetPauser>, pauser: Pubkey, enabled: bool) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_admin(&ctx.accounts.host_config, &ctx.accounts.admin)?;
    let (expected, bump) = pauser_address(pauser);
    require_keys_eq!(
        expected,
        ctx.accounts.pauser_record.key(),
        ZamaHostError::PauserRecordMismatch
    );

    let info = ctx.accounts.pauser_record.to_account_info();
    if current_pauser_status(&info, pauser, bump)?.unwrap_or(false) == enabled {
        return Ok(());
    }

    create_pda_if_needed(
        &ctx.accounts.payer.to_account_info(),
        &info,
        &ctx.accounts.system_program.to_account_info(),
        8 + PauserRecord::SPACE,
        &[PAUSER_SEED, pauser.as_ref(), &[bump]],
    )?;

    write_account(
        &info,
        &PauserRecord {
            pauser,
            enabled,
            bump,
        },
    )?;
    emit_event_cpi(
        &ctx.accounts.event_authority,
        &PauserUpdatedEvent {
            version: EVENT_VERSION,
            pauser_record: ctx.accounts.pauser_record.key(),
            pauser,
            enabled,
            updated_slot: Clock::get()?.slot,
        },
    )?;
    Ok(())
}

fn current_pauser_status(info: &AccountInfo, pauser: Pubkey, bump: u8) -> Result<Option<bool>> {
    if is_uninitialized_pda_account(info, ZamaHostError::PauserRecordMismatch)? {
        return Ok(None);
    }
    require_keys_eq!(*info.owner, crate::ID, ZamaHostError::PauserRecordMismatch);
    require!(
        info.data_len() == 8 + PauserRecord::SPACE,
        ZamaHostError::PauserRecordMismatch
    );
    let data = info.try_borrow_data()?;
    let mut data_slice: &[u8] = &data;
    let record = PauserRecord::try_deserialize(&mut data_slice)?;
    require!(
        record.pauser == pauser && record.bump == bump,
        ZamaHostError::PauserRecordMismatch
    );
    Ok(Some(record.enabled))
}
