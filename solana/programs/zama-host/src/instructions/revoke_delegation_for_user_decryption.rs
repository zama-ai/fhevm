//! Revokes user-decryption delegations.

use anchor_lang::prelude::*;

use super::common::*;
#[cfg(feature = "emit-events")]
use crate::events::UserDecryptionDelegationUpdatedEvent;
use crate::{errors::ZamaHostError, state::*};

/// Accounts for revoking a user-decryption delegation.
#[derive(Accounts)]
pub struct RevokeDelegationForUserDecryption<'info> {
    /// Delegator that owns the delegation.
    pub delegator: Signer<'info>,
    /// Singleton config PDA.
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    /// Delegation record to revoke.
    #[account(mut)]
    pub delegation_record: Account<'info, UserDecryptionDelegation>,
}

/// Marks an existing user-decryption delegation as revoked.
pub fn revoke_delegation_for_user_decryption(
    ctx: Context<RevokeDelegationForUserDecryption>,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_not_paused(&ctx.accounts.host_config)?;
    let clock = Clock::get()?;
    require_keys_eq!(
        ctx.accounts.delegator.key(),
        ctx.accounts.delegation_record.delegator,
        ZamaHostError::InvalidDelegation
    );
    let (expected, bump) = user_decryption_delegation_address(
        ctx.accounts.delegation_record.delegator,
        ctx.accounts.delegation_record.delegate,
        ctx.accounts.delegation_record.app_account,
    );
    require_keys_eq!(
        expected,
        ctx.accounts.delegation_record.key(),
        ZamaHostError::DelegationPdaMismatch
    );
    require!(
        ctx.accounts.delegation_record.to_account_info().data_len()
            == 8 + UserDecryptionDelegation::SPACE,
        ZamaHostError::InvalidDelegation
    );
    require!(
        ctx.accounts.delegation_record.bump == bump,
        ZamaHostError::DelegationPdaMismatch
    );
    require!(
        ctx.accounts.delegation_record.last_update_slot < clock.slot,
        ZamaHostError::DelegationUpdatedInCurrentSlot
    );
    require!(
        !ctx.accounts.delegation_record.revoked,
        ZamaHostError::DelegationRevoked
    );
    #[cfg(feature = "emit-events")]
    let old_expiration_slot = ctx.accounts.delegation_record.expiration_slot;
    let delegation_counter = ctx
        .accounts
        .delegation_record
        .delegation_counter
        .checked_add(1)
        .ok_or(ZamaHostError::InvalidDelegation)?;
    ctx.accounts.delegation_record.revoked = true;
    ctx.accounts.delegation_record.expiration_slot = 0;
    ctx.accounts.delegation_record.delegation_counter = delegation_counter;
    ctx.accounts.delegation_record.last_update_slot = clock.slot;
    #[cfg(feature = "emit-events")]
    emit!(UserDecryptionDelegationUpdatedEvent {
        version: EVENT_VERSION,
        delegator: ctx.accounts.delegation_record.delegator,
        delegate: ctx.accounts.delegation_record.delegate,
        app_account: ctx.accounts.delegation_record.app_account,
        delegation_counter,
        old_expiration_slot,
        new_expiration_slot: ctx.accounts.delegation_record.expiration_slot,
        last_update_slot: clock.slot,
        revoked: true,
    });
    Ok(())
}
