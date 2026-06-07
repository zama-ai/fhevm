//! Disables threshold verifier-set accounts.

use anchor_lang::prelude::*;

use super::common::*;
use crate::{errors::ZamaHostError, events::VerifierSetDisabledEvent, state::*};

/// Accounts for disabling a verifier set.
#[derive(Accounts)]
pub struct DisableVerifierSet<'info> {
    /// Configured host admin.
    pub admin: Signer<'info>,
    /// Singleton config PDA.
    #[account(mut, seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Box<Account<'info, HostConfig>>,
    /// Verifier set to disable.
    #[account(mut)]
    pub verifier_set: Box<Account<'info, VerifierSet>>,
}

/// Disables an existing verifier set.
pub fn disable_verifier_set(ctx: Context<DisableVerifierSet>) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_admin(&ctx.accounts.host_config, ctx.accounts.admin.key())?;
    assert_verifier_set_shape(
        &ctx.accounts.verifier_set,
        ctx.accounts.verifier_set.kind,
        ctx.accounts.verifier_set.scope,
        ctx.accounts.verifier_set.version,
    )?;
    require!(
        verifier_set_scope_is_valid_for_kind(
            ctx.accounts.verifier_set.kind,
            ctx.accounts.verifier_set.scope,
            ctx.accounts.host_config.key(),
        ),
        ZamaHostError::VerifierSetMismatch
    );
    require_keys_eq!(
        ctx.accounts.verifier_set.admin,
        ctx.accounts.admin.key(),
        ZamaHostError::HostConfigAdminMismatch
    );
    if ctx.accounts.verifier_set.state == VERIFIER_SET_STATE_DISABLED {
        return Ok(());
    }
    ctx.accounts.verifier_set.state = VERIFIER_SET_STATE_DISABLED;
    ctx.accounts.verifier_set.updated_slot = Clock::get()?.slot;
    emit!(VerifierSetDisabledEvent {
        version: EVENT_VERSION,
        verifier_set: ctx.accounts.verifier_set.key(),
        admin: ctx.accounts.admin.key(),
        kind: ctx.accounts.verifier_set.kind,
        scope: ctx.accounts.verifier_set.scope,
        set_version: ctx.accounts.verifier_set.version,
        updated_slot: ctx.accounts.verifier_set.updated_slot,
    });
    Ok(())
}
