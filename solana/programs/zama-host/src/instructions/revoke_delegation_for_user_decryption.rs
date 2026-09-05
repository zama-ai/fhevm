//! Revokes user-decryption delegations.

use anchor_lang::prelude::*;

use super::common::*;
use crate::{errors::ZamaHostError, state::*};

/// Accounts for revoking a user-decryption delegation.
#[derive(Accounts)]
pub struct RevokeDelegationForUserDecryption<'info> {
    /// Delegator that owns the delegation.
    pub delegator: Signer<'info>,
    /// Singleton config PDA. Carried for the account list the clients already build, and pinned to
    /// the canonical singleton by its seeds; no flag on it gates this instruction.
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    /// Delegation record to revoke.
    #[account(mut)]
    pub delegation_record: Account<'info, UserDecryptionDelegation>,
}

/// Marks an existing user-decryption delegation as revoked.
///
/// Deliberately not pause-gated, unlike granting. Revoking is the delegator's abort, and a lever
/// the operator can switch off is not the delegator's lever — the same reasoning `revoke_permits`
/// records. The asymmetry is what makes pause coherent: a paused host stops the Connector from
/// serving user decryptions (its authorization reads `HostConfig.paused`), and existing grants
/// would otherwise stay frozen in place with no way for their delegator to withdraw them.
pub fn revoke_delegation_for_user_decryption(
    ctx: Context<RevokeDelegationForUserDecryption>,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    let clock = Clock::get()?;
    require_keys_eq!(
        ctx.accounts.delegator.key(),
        ctx.accounts.delegation_record.delegator,
        ZamaHostError::InvalidDelegation
    );
    let (expected, bump) = user_decryption_delegation_address(
        ctx.accounts.delegation_record.delegator,
        ctx.accounts.delegation_record.delegate,
        ctx.accounts
            .delegation_record
            .encrypted_value_account_authority,
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
    Ok(())
}
