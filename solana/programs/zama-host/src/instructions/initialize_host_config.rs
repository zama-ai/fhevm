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
    assert_valid_host_config_args(&args)?;
    let updated_slot = Clock::get()?.slot;
    let config_key = ctx.accounts.host_config.key();
    let config = &mut ctx.accounts.host_config;
    config.admin = ctx.accounts.admin.key();
    config.chain_id = args.chain_id;
    config.input_verifier_authority = args.input_verifier_authority;
    config.gateway_chain_id = args.gateway_chain_id;
    config.input_verification_contract = args.input_verification_contract;
    config.coprocessor_signer = args.coprocessor_signer;
    config.decryption_contract = args.decryption_contract;
    config.current_kms_context_id = 0;
    config.material_authority = args.material_authority;
    config.test_authority = args.test_authority;
    config.paused = false;
    config.mock_input_enabled = args.mock_input_enabled;
    config.test_shims_enabled = args.test_shims_enabled;
    config.grant_deny_list_enabled = args.grant_deny_list_enabled;
    // INV-14: ship HCU enforcement disabled (0 = unlimited); an admin enables it post-calibration.
    config.max_hcu_per_tx = 0;
    config.max_hcu_depth_per_tx = 0;
    config.updated_slot = updated_slot;
    config.bump = ctx.bumps.host_config;
    #[cfg(feature = "emit-events")]
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

fn assert_valid_host_config_args(args: &InitializeHostConfigArgs) -> Result<()> {
    require!(
        args.chain_id != 0
            && args.input_verifier_authority != Pubkey::default()
            && args.material_authority != Pubkey::default()
            && args.test_authority != Pubkey::default(),
        ZamaHostError::InvalidHostConfig
    );
    #[cfg(not(feature = "poc"))]
    require!(
        args.chain_id != SOLANA_POC_CHAIN_ID
            && !args.mock_input_enabled
            && !args.test_shims_enabled,
        ZamaHostError::InvalidHostConfig
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_args() -> InitializeHostConfigArgs {
        InitializeHostConfigArgs {
            chain_id: 42,
            input_verifier_authority: Pubkey::new_unique(),
            gateway_chain_id: 0,
            input_verification_contract: [0u8; 20],
            coprocessor_signer: [0u8; 20],
            decryption_contract: [0u8; 20],
            material_authority: Pubkey::new_unique(),
            test_authority: Pubkey::new_unique(),
            mock_input_enabled: false,
            test_shims_enabled: false,
            grant_deny_list_enabled: false,
        }
    }

    #[test]
    fn valid_production_args_pass() {
        assert!(assert_valid_host_config_args(&valid_args()).is_ok());
    }

    #[cfg(not(feature = "poc"))]
    #[test]
    fn production_args_reject_poc_chain_and_test_flags() {
        let mut poc_chain = valid_args();
        poc_chain.chain_id = SOLANA_POC_CHAIN_ID;
        assert!(assert_valid_host_config_args(&poc_chain).is_err());

        let mut mock_enabled = valid_args();
        mock_enabled.mock_input_enabled = true;
        assert!(assert_valid_host_config_args(&mock_enabled).is_err());

        let mut shims_enabled = valid_args();
        shims_enabled.test_shims_enabled = true;
        assert!(assert_valid_host_config_args(&shims_enabled).is_err());
    }

    #[cfg(feature = "poc")]
    #[test]
    fn poc_feature_accepts_poc_chain_and_test_flags() {
        let mut args = valid_args();
        args.chain_id = SOLANA_POC_CHAIN_ID;
        args.mock_input_enabled = true;
        args.test_shims_enabled = true;
        assert!(assert_valid_host_config_args(&args).is_ok());
    }
}
