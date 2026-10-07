//! Creates and refreshes user-decryption delegations.

use anchor_lang::prelude::*;

use super::common::*;
use crate::{errors::ZamaHostError, state::*};
use zama_solana_acl::DELEGATION_SEED;

/// Accounts for creating or updating a user-decryption delegation.
#[derive(Accounts)]
#[instruction(delegate: Pubkey, program: Pubkey)]
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
    /// The `delegator → delegate` record for the application, created on first grant.
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + UserDecryptionDelegation::SPACE,
        seeds = [DELEGATION_SEED, delegator.key().as_ref(), delegate.as_ref(), program.as_ref(), scope.key().as_ref()],
        bump,
    )]
    pub delegation_record: Account<'info, UserDecryptionDelegation>,
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

    // A record created by this call is zeroed; every written record has `delegation_counter >= 1`
    // (a revoke keeps the record and raises the counter).
    let record = &mut ctx.accounts.delegation_record;
    let delegation_counter = if record.delegation_counter == 0 {
        1
    } else {
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
    };
    record.set_inner(UserDecryptionDelegation {
        delegator,
        delegate,
        program,
        scope,
        expires_at,
        delegation_counter,
        last_update_slot: clock.slot,
        bump: ctx.bumps.delegation_record,
    });
    Ok(())
}
