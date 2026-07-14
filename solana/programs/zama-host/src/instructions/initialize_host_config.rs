//! Initializes the singleton ZamaHost configuration account.

use anchor_lang::prelude::*;

use super::common::*;
#[cfg(feature = "emit-events")]
use crate::events::HostConfigInitializedEvent;
use crate::{errors::ZamaHostError, state::*};

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

/// Initializes the singleton host config.
pub fn initialize_host_config(
    ctx: Context<InitializeHostConfig>,
    args: InitializeHostConfigArgs,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_valid_host_config_args(&args)?;
    let updated_slot = Clock::get()?.slot;
    #[cfg(feature = "emit-events")]
    let config_key = ctx.accounts.host_config.key();
    let config = &mut ctx.accounts.host_config;
    config.admin = ctx.accounts.admin.key();
    config.chain_id = args.chain_id;
    config.gateway_chain_id = args.gateway_chain_id;
    config.input_verification_contract = args.input_verification_contract;
    config.coprocessor_signer = args.coprocessor_signer;
    config.decryption_contract = args.decryption_contract;
    config.current_kms_context_id = 0;
    config.paused = false;
    config.grant_deny_list_enabled = args.grant_deny_list_enabled;
    // Ship HCU enforcement disabled (0 = unlimited); an admin enables it post-calibration.
    config.max_hcu_per_tx = 0;
    config.max_hcu_depth_per_tx = 0;
    // Ship the per-app block cap unrestricted (u64::MAX): the neutral state that short-circuits
    // the cap and touches no meter. A `0` default would instead ban every untrusted app on deploy.
    config.hcu_block_cap_per_app = u64::MAX;
    config.updated_slot = updated_slot;
    config.bump = ctx.bumps.host_config;
    #[cfg(feature = "emit-events")]
    emit!(HostConfigInitializedEvent {
        version: EVENT_VERSION,
        config: config_key,
        admin: config.admin,
        chain_id: config.chain_id,
    });
    Ok(())
}

fn assert_valid_host_config_args(args: &InitializeHostConfigArgs) -> Result<()> {
    // RFC-021 invariant: the ZamaHost is always a Solana host chain, so its
    // `chain_id` must set the chain-type high bit, while the EVM `gateway_chain_id`
    // must leave it clear. Setting the bit also guarantees `chain_id != 0`.
    require!(
        args.chain_id & SOLANA_CHAIN_TYPE_BIT != 0
            && args.gateway_chain_id & SOLANA_CHAIN_TYPE_BIT == 0,
        ZamaHostError::InvalidChainTypeBit
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_args() -> InitializeHostConfigArgs {
        InitializeHostConfigArgs {
            // The ZamaHost is a Solana host chain, so its chain id sets the
            // RFC-021 chain-type high bit.
            chain_id: SOLANA_CHAIN_TYPE_BIT | 42,
            gateway_chain_id: 0,
            input_verification_contract: [0u8; 20],
            coprocessor_signer: [0u8; 20],
            decryption_contract: [0u8; 20],
            grant_deny_list_enabled: false,
        }
    }

    #[test]
    fn valid_production_args_pass() {
        assert!(assert_valid_host_config_args(&valid_args()).is_ok());
    }

    #[test]
    fn local_chain_id_is_valid_without_relaxing_entropy() {
        let mut args = valid_args();
        args.chain_id = SOLANA_POC_CHAIN_ID;
        assert!(assert_valid_host_config_args(&args).is_ok());
    }

    #[test]
    fn rejects_solana_chain_id_without_chain_type_bit() {
        // A Solana host id that leaves the high bit clear (e.g. a bare EVM-style
        // value) violates the RFC-021 invariant and must be rejected.
        let mut args = valid_args();
        args.chain_id = 12345;
        let err = assert_valid_host_config_args(&args).unwrap_err();
        assert_eq!(err, error!(ZamaHostError::InvalidChainTypeBit));
    }

    #[test]
    fn rejects_zero_chain_id() {
        let mut args = valid_args();
        args.chain_id = 0;
        let err = assert_valid_host_config_args(&args).unwrap_err();
        assert_eq!(err, error!(ZamaHostError::InvalidChainTypeBit));
    }

    #[test]
    fn rejects_gateway_chain_id_with_chain_type_bit() {
        // The gateway is an EVM chain; its chain id must leave the high bit clear.
        let mut args = valid_args();
        args.gateway_chain_id = SOLANA_CHAIN_TYPE_BIT | 1;
        let err = assert_valid_host_config_args(&args).unwrap_err();
        assert_eq!(err, error!(ZamaHostError::InvalidChainTypeBit));
    }
}
