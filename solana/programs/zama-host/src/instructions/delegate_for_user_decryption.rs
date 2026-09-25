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
    /// The application's scope: an account `program` owns, or the wildcard sentinel.
    /// CHECK: only its key and owner are read; the owner is checked against `program`.
    pub scope: UncheckedAccount<'info>,
    /// CHECK: created or overwritten after canonical delegation PDA validation.
    #[account(mut)]
    pub delegation_record: UncheckedAccount<'info>,
    /// System program used for account creation.
    pub system_program: Program<'info, System>,
}

/// Grants or renews `delegator → delegate` in the application `(program, scope)`, or in every
/// application through [`AppScope::WILDCARD`], until `expires_at` (Unix seconds, exclusive). The
/// checks are EVM's `delegateForUserDecryption`, with the application in place of
/// `contractAddress`.
pub fn delegate_for_user_decryption(
    ctx: Context<DelegateForUserDecryption>,
    delegate: Pubkey,
    program: Pubkey,
    expires_at: u64,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_not_paused(&ctx.accounts.host_config, PauseArea::AclWrites)?;
    let clock = Clock::get()?;
    let now =
        u64::try_from(clock.unix_timestamp).map_err(|_| error!(ZamaHostError::ClockBeforeEpoch))?;
    let delegator = ctx.accounts.delegator.key();
    let scope = ctx.accounts.scope.key();
    let app = AppScope { program, scope };
    require_top_level_unless_pda(&delegator, ZamaHostError::WalletDelegationThroughCpi)?;
    require!(
        delegate != Pubkey::default() && program != Pubkey::default(),
        ZamaHostError::InvalidDelegation
    );
    require!(
        delegate.to_bytes() != WILDCARD_APP,
        ZamaHostError::InvalidDelegation
    );
    // The sentinel fills the whole application or none of it.
    require!(
        (program.to_bytes() == WILDCARD_APP) == (scope.to_bytes() == WILDCARD_APP),
        ZamaHostError::InvalidDelegation
    );
    require_keys_neq!(delegator, delegate, ZamaHostError::InvalidDelegation);
    require_keys_neq!(delegator, program, ZamaHostError::InvalidDelegation);
    require_keys_neq!(delegate, program, ZamaHostError::InvalidDelegation);
    require!(expires_at > now, ZamaHostError::InvalidDelegation);
    // A grant names an application a store can have: `create_encrypted_store` requires the same.
    if app != AppScope::WILDCARD {
        require_keys_eq!(
            *ctx.accounts.scope.owner,
            program,
            ZamaHostError::DelegationScopeNotProgramAccount
        );
    }

    let (expected, bump) = user_decryption_delegation_address(delegator, delegate, app);
    require_keys_eq!(
        expected,
        ctx.accounts.delegation_record.key(),
        ZamaHostError::DelegationPdaMismatch
    );
    let info = ctx.accounts.delegation_record.to_account_info();
    let current = read_existing_delegation(&info, bump)?;
    let (delegator_bytes, delegate_bytes, program_bytes, scope_bytes) = (
        delegator.to_bytes(),
        delegate.to_bytes(),
        program.to_bytes(),
        scope.to_bytes(),
    );
    let [seed, delegator_seed, delegate_seed, program_seed, scope_seed] =
        zama_solana_acl::delegation_seeds(
            &delegator_bytes,
            &delegate_bytes,
            &program_bytes,
            &scope_bytes,
        );
    create_pda_if_needed(
        &ctx.accounts.payer.to_account_info(),
        &info,
        &ctx.accounts.system_program.to_account_info(),
        8 + UserDecryptionDelegation::SPACE,
        &[
            seed,
            delegator_seed,
            delegate_seed,
            program_seed,
            scope_seed,
            &[bump],
        ],
    )?;
    let delegation_counter = match current {
        Some(record) => {
            require!(
                record.delegator == delegator
                    && record.delegate == delegate
                    && record.program == program
                    && record.scope == scope,
                ZamaHostError::InvalidDelegation
            );
            require!(
                record.last_update_slot < clock.slot,
                ZamaHostError::DelegationUpdatedInCurrentSlot
            );
            require!(
                record.expires_at != expires_at,
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
            program,
            scope,
            expires_at,
            delegation_counter,
            last_update_slot: clock.slot,
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
