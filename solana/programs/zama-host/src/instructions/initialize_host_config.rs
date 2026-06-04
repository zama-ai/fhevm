//! Initializes the singleton ZamaHost configuration account.

use anchor_lang::prelude::*;

use super::common::*;
use crate::{errors::ZamaHostError, events::HostConfigInitializedEvent, state::*};

/// Accounts for initializing the singleton [`HostConfig`].
#[derive(Accounts)]
pub struct InitializeHostConfig<'info> {
    /// Pays rent for the config account.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Initial admin stored in the config.
    pub admin: Signer<'info>,
    /// Singleton config PDA.
    #[account(
        init,
        payer = payer,
        space = 8 + HostConfig::SPACE,
        seeds = [HOST_CONFIG_SEED],
        bump
    )]
    pub host_config: Account<'info, HostConfig>,
    /// System program used for account creation.
    pub system_program: Program<'info, System>,
}

/// Initializes the singleton host config and records authority gates.
pub fn initialize_host_config(
    ctx: Context<InitializeHostConfig>,
    args: InitializeHostConfigArgs,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    require!(
        args.chain_id != 0
            && args.input_verifier_authority != Pubkey::default()
            && args.material_authority != Pubkey::default()
            && args.test_authority != Pubkey::default(),
        ZamaHostError::InvalidHostConfig
    );
    let updated_slot = Clock::get()?.slot;
    let config_key = ctx.accounts.host_config.key();
    let config = &mut ctx.accounts.host_config;
    config.admin = ctx.accounts.admin.key();
    config.chain_id = args.chain_id;
    config.input_verifier_authority = args.input_verifier_authority;
    config.gateway_chain_id = args.gateway_chain_id;
    config.input_verification_contract = args.input_verification_contract;
    config.coprocessor_signer = args.coprocessor_signer;
    config.material_authority = args.material_authority;
    config.test_authority = args.test_authority;
    config.paused = false;
    config.mock_input_enabled = args.mock_input_enabled;
    config.test_shims_enabled = args.test_shims_enabled;
    config.grant_deny_list_enabled = args.grant_deny_list_enabled;
    config.updated_slot = updated_slot;
    config.bump = ctx.bumps.host_config;
    emit!(HostConfigInitializedEvent {
        version: EVENT_VERSION,
        config: config_key,
        admin: config.admin,
        chain_id: config.chain_id,
        input_verifier_authority: config.input_verifier_authority,
        material_authority: config.material_authority,
        test_authority: config.test_authority,
    });
    Ok(())
}
