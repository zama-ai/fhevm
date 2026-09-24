//! Revokes user-decryption delegations.

use anchor_lang::prelude::*;

use super::common::*;
use crate::{errors::ZamaHostError, state::*};

/// Accounts for revoking a user-decryption delegation.
#[derive(Accounts)]
pub struct RevokeDelegationForUserDecryption<'info> {
    /// Delegator that owns the delegation.
    pub delegator: Signer<'info>,
    /// Singleton config PDA, whose `acl_writes` pause flag gates this instruction.
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    /// Delegation record to revoke.
    #[account(mut)]
    pub delegation_record: Account<'info, UserDecryptionDelegation>,
}

/// Ends an existing user-decryption delegation by setting `expires_at` to 0, as EVM's
/// `revokeDelegationForUserDecryption` does.
///
/// Paused with the other ACL writes, as EVM's revocation is `whenNotPaused`. The pause does not
/// reach decryption, so while `acl_writes` is set a delegate can still decrypt over HTTP and the
/// delegator cannot revoke; EVM has the same gap. `revoke_permits` stays unpaused, as EVM's
/// `invalidateDecryptionSignaturesBefore` is.
pub fn revoke_delegation_for_user_decryption(
    ctx: Context<RevokeDelegationForUserDecryption>,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_not_paused(
        &ctx.accounts.host_config,
        |paused| paused.acl_writes,
        ZamaHostError::AclWritesPaused,
    )?;
    let clock = Clock::get()?;
    require_keys_eq!(
        ctx.accounts.delegator.key(),
        ctx.accounts.delegation_record.delegator,
        ZamaHostError::InvalidDelegation
    );
    let record = &ctx.accounts.delegation_record;
    let (expected, bump) = user_decryption_delegation_address(
        record.delegator,
        record.delegate,
        AppScope {
            program: record.program,
            scope: record.scope,
        },
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
        ctx.accounts.delegation_record.expires_at != 0,
        ZamaHostError::NotDelegatedYet
    );
    let delegation_counter = ctx
        .accounts
        .delegation_record
        .delegation_counter
        .checked_add(1)
        .ok_or(ZamaHostError::InvalidDelegation)?;
    ctx.accounts.delegation_record.expires_at = 0;
    ctx.accounts.delegation_record.delegation_counter = delegation_counter;
    ctx.accounts.delegation_record.last_update_slot = clock.slot;
    Ok(())
}
