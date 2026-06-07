//! Migrates a legacy host config to verifier-set based input proofs.

use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, system_instruction},
};

use super::common::*;
use crate::{errors::ZamaHostError, events::VerifierSetCreatedEvent, state::*};

/// Accounts for migrating a legacy [`HostConfig`] layout.
#[derive(Accounts)]
#[instruction(args: CreateVerifierSetArgs)]
pub struct MigrateHostConfigVerifierSet<'info> {
    /// Rent payer for any account-size top-up needed by realloc.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Legacy and current host admin.
    pub admin: Signer<'info>,
    /// CHECK: legacy host config account; handler verifies owner, PDA, discriminator, size, and fields.
    #[account(mut)]
    pub host_config: UncheckedAccount<'info>,
    /// Canonical initial input verifier-set PDA.
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
    /// System program used for rent top-up and verifier-set creation.
    pub system_program: Program<'info, System>,
}

/// Legacy config layout before input verifier authorities became verifier sets.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
struct LegacyHostConfigV1 {
    pub admin: Pubkey,
    pub chain_id: u64,
    pub input_verifier_authority: Pubkey,
    pub material_authority: Pubkey,
    pub test_authority: Pubkey,
    pub paused: bool,
    pub mock_input_enabled: bool,
    pub test_shims_enabled: bool,
    pub grant_deny_list_enabled: bool,
    pub updated_slot: u64,
    pub bump: u8,
}

impl LegacyHostConfigV1 {
    const SPACE: usize = 32 + 8 + 32 + 32 + 32 + 1 + 1 + 1 + 1 + 8 + 1;
}

/// Migrates the singleton config and creates the initial input verifier set.
///
/// `create_verifier_set` cannot run against the legacy account because Anchor
/// cannot deserialize the old layout as the new one. This instruction performs
/// both steps atomically from the raw legacy bytes.
pub fn migrate_host_config_verifier_set(
    ctx: Context<MigrateHostConfigVerifierSet>,
    args: CreateVerifierSetArgs,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    let host_config_info = ctx.accounts.host_config.to_account_info();
    let host_config_key = ctx.accounts.host_config.key();
    let (expected_config_key, expected_config_bump) = host_config_address();
    require_keys_eq!(
        host_config_key,
        expected_config_key,
        ZamaHostError::HostConfigMismatch
    );
    require_keys_eq!(
        *host_config_info.owner,
        crate::ID,
        ZamaHostError::HostConfigMismatch
    );
    require!(
        !host_config_info.executable,
        ZamaHostError::HostConfigMismatch
    );

    let legacy = {
        let data = host_config_info.try_borrow_data()?;
        require!(
            data.len() == 8 + LegacyHostConfigV1::SPACE
                && data[..8] == HostConfig::DISCRIMINATOR[..],
            ZamaHostError::HostConfigMismatch
        );
        LegacyHostConfigV1::try_from_slice(&data[8..])
            .map_err(|_| error!(ZamaHostError::HostConfigMismatch))?
    };
    require_keys_eq!(
        legacy.admin,
        ctx.accounts.admin.key(),
        ZamaHostError::HostConfigAdminMismatch
    );
    require!(
        legacy.chain_id != 0
            && legacy.input_verifier_authority != Pubkey::default()
            && legacy.material_authority != Pubkey::default()
            && legacy.test_authority != Pubkey::default()
            && legacy.bump == expected_config_bump,
        ZamaHostError::InvalidHostConfig
    );
    require!(
        args.kind == VERIFIER_SET_KIND_INPUT
            && args.scope == host_config_key
            && args.version != 0
            && verifier_set_fields_are_valid(
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

    let required_lamports = Rent::get()?.minimum_balance(8 + HostConfig::SPACE);
    let current_lamports = host_config_info.lamports();
    if current_lamports < required_lamports {
        invoke(
            &system_instruction::transfer(
                &ctx.accounts.payer.key(),
                &host_config_key,
                required_lamports - current_lamports,
            ),
            &[
                ctx.accounts.payer.to_account_info(),
                host_config_info.clone(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
    }

    host_config_info.resize(8 + HostConfig::SPACE)?;
    let migrated = HostConfig {
        admin: legacy.admin,
        chain_id: legacy.chain_id,
        input_verifier_set: verifier_set_key,
        input_verifier_set_version: args.version,
        material_authority: legacy.material_authority,
        test_authority: legacy.test_authority,
        paused: legacy.paused,
        mock_input_enabled: legacy.mock_input_enabled,
        test_shims_enabled: legacy.test_shims_enabled,
        grant_deny_list_enabled: legacy.grant_deny_list_enabled,
        updated_slot: now,
        bump: legacy.bump,
    };
    {
        let mut data = host_config_info.try_borrow_mut_data()?;
        let mut writer: &mut [u8] = &mut data[..];
        migrated.try_serialize(&mut writer)?;
    }

    emit_config_updated(&migrated, ctx.accounts.admin.key());
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
