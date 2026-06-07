//! Creates versioned threshold verifier-set accounts.

use anchor_lang::prelude::*;

use super::common::*;
use crate::{errors::ZamaHostError, events::VerifierSetCreatedEvent, state::*};

/// Accounts for creating a verifier set.
#[derive(Accounts)]
#[instruction(args: CreateVerifierSetArgs)]
pub struct CreateVerifierSet<'info> {
    /// Pays rent for the verifier-set account.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Configured host admin.
    pub admin: Signer<'info>,
    /// Singleton config PDA.
    #[account(mut, seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Box<Account<'info, HostConfig>>,
    /// Canonical verifier-set PDA for `(kind, scope, version)`.
    #[account(
        init,
        payer = payer,
        space = 8 + VerifierSet::SPACE,
        seeds = [
            VERIFIER_SET_SEED,
            &[args.kind],
            args.scope.as_ref(),
            &args.version.to_le_bytes(),
        ],
        bump
    )]
    pub verifier_set: Box<Account<'info, VerifierSet>>,
    /// System program used for account creation.
    pub system_program: Program<'info, System>,
}

/// Creates a versioned verifier set.
///
/// Host input sets are scoped to the host config PDA and rotate
/// `HostConfig::input_verifier_set` immediately. Token disclosure/redemption
/// sets are scoped to their confidential mint and do not mutate host config;
/// token programs opt into a specific set by storing its PDA.
pub fn create_verifier_set(
    ctx: Context<CreateVerifierSet>,
    args: CreateVerifierSetArgs,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_admin(&ctx.accounts.host_config, ctx.accounts.admin.key())?;
    require!(args.version != 0, ZamaHostError::InvalidVerifierSet);
    require!(
        verifier_set_scope_is_valid_for_kind(args.kind, args.scope, ctx.accounts.host_config.key(),),
        ZamaHostError::InvalidVerifierSet
    );
    require!(
        verifier_set_fields_are_valid(
            args.kind,
            args.threshold,
            args.signer_count,
            &args.signers,
            VERIFIER_SET_STATE_ACTIVE,
        ),
        ZamaHostError::InvalidVerifierSet
    );

    let now = Clock::get()?.slot;
    let verifier_set_key = ctx.accounts.verifier_set.key();
    let verifier_set = &mut ctx.accounts.verifier_set;
    verifier_set.admin = ctx.accounts.admin.key();
    verifier_set.kind = args.kind;
    verifier_set.scope = args.scope;
    verifier_set.version = args.version;
    verifier_set.threshold = args.threshold;
    verifier_set.signer_count = args.signer_count;
    verifier_set.signers = args.signers;
    verifier_set.state = VERIFIER_SET_STATE_ACTIVE;
    verifier_set.created_slot = now;
    verifier_set.updated_slot = now;
    verifier_set.bump = ctx.bumps.verifier_set;

    if verifier_set_rotates_host_input(verifier_set.kind) {
        require!(
            verifier_set.version > ctx.accounts.host_config.input_verifier_set_version,
            ZamaHostError::InvalidVerifierSet
        );
        ctx.accounts.host_config.input_verifier_set = verifier_set_key;
        ctx.accounts.host_config.input_verifier_set_version = verifier_set.version;
        ctx.accounts.host_config.updated_slot = now;
        emit_config_updated(&ctx.accounts.host_config, ctx.accounts.admin.key());
    }
    emit!(VerifierSetCreatedEvent {
        version: EVENT_VERSION,
        verifier_set: verifier_set_key,
        admin: verifier_set.admin,
        kind: verifier_set.kind,
        scope: verifier_set.scope,
        set_version: verifier_set.version,
        threshold: verifier_set.threshold,
        signer_count: verifier_set.signer_count,
        created_slot: verifier_set.created_slot,
    });
    Ok(())
}
