//! Creates and refreshes user-decryption delegations.

use anchor_lang::prelude::*;

use super::common::*;
use crate::{errors::ZamaHostError, state::*};

/// Accounts for creating or updating a user-decryption delegation.
#[derive(Accounts)]
pub struct DelegateForUserDecryption<'info> {
    /// Pays rent if the delegation PDA must be created.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// User granting delegated decrypt rights.
    pub delegator: Signer<'info>,
    /// Singleton config PDA.
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    /// CHECK: created or overwritten after canonical delegation PDA validation.
    #[account(mut)]
    pub delegation_record: UncheckedAccount<'info>,
    /// System program used for account creation.
    pub system_program: Program<'info, System>,
}

/// Creates or refreshes a delegation tuple for future KMS witness verification.
pub fn delegate_for_user_decryption(
    ctx: Context<DelegateForUserDecryption>,
    delegate: Pubkey,
    encrypted_value_account_authority: Pubkey,
    expiration_slot: u64,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_not_paused(&ctx.accounts.host_config)?;
    let clock = Clock::get()?;
    let delegator = ctx.accounts.delegator.key();
    require!(
        delegate != Pubkey::default() && encrypted_value_account_authority != Pubkey::default(),
        ZamaHostError::InvalidDelegation
    );
    require!(
        delegate.to_bytes() != WILDCARD_ENCRYPTED_VALUE_ACCOUNT_AUTHORITY_BYTES,
        ZamaHostError::InvalidDelegation
    );
    require_keys_neq!(delegator, delegate, ZamaHostError::InvalidDelegation);
    require_keys_neq!(
        delegator,
        encrypted_value_account_authority,
        ZamaHostError::InvalidDelegation
    );
    require_keys_neq!(
        delegate,
        encrypted_value_account_authority,
        ZamaHostError::InvalidDelegation
    );
    require!(
        expiration_slot > clock.slot,
        ZamaHostError::InvalidDelegation
    );

    let (expected, bump) =
        user_decryption_delegation_address(delegator, delegate, encrypted_value_account_authority);
    require_keys_eq!(
        expected,
        ctx.accounts.delegation_record.key(),
        ZamaHostError::DelegationPdaMismatch
    );
    let info = ctx.accounts.delegation_record.to_account_info();
    let current = read_existing_delegation(&info, bump)?;
    create_pda_if_needed(
        &ctx.accounts.payer.to_account_info(),
        &info,
        &ctx.accounts.system_program.to_account_info(),
        8 + UserDecryptionDelegation::SPACE,
        &[
            crate::state::DELEGATION_SEED,
            delegator.as_ref(),
            delegate.as_ref(),
            encrypted_value_account_authority.as_ref(),
            &[bump],
        ],
    )?;
    let delegation_counter = match current {
        Some(record) => {
            require_keys_eq!(
                record.delegator,
                delegator,
                ZamaHostError::InvalidDelegation
            );
            require_keys_eq!(record.delegate, delegate, ZamaHostError::InvalidDelegation);
            require_keys_eq!(
                record.encrypted_value_account_authority,
                encrypted_value_account_authority,
                ZamaHostError::InvalidDelegation
            );
            require!(
                record.last_update_slot < clock.slot,
                ZamaHostError::DelegationUpdatedInCurrentSlot
            );
            require!(
                record.revoked || record.expiration_slot != expiration_slot,
                ZamaHostError::InvalidDelegation
            );
            record
                .delegation_counter
                .checked_add(1)
                .ok_or(ZamaHostError::InvalidDelegation)?
        }
        None => 1,
    };
    write_account(
        &info,
        &UserDecryptionDelegation {
            delegator,
            delegate,
            encrypted_value_account_authority,
            expiration_slot,
            delegation_counter,
            last_update_slot: clock.slot,
            revoked: false,
            bump,
        },
    )?;
    Ok(())
}

fn read_existing_delegation(
    info: &AccountInfo,
    bump: u8,
) -> Result<Option<UserDecryptionDelegation>> {
    if info.owner != &crate::ID {
        return Ok(None);
    }
    require!(
        info.data_len() == 8 + UserDecryptionDelegation::SPACE,
        ZamaHostError::InvalidDelegation
    );
    let data = info.try_borrow_data()?;
    let mut data_slice: &[u8] = &data;
    let record = UserDecryptionDelegation::try_deserialize(&mut data_slice)?;
    require!(record.bump == bump, ZamaHostError::DelegationPdaMismatch);
    Ok(Some(record))
}
