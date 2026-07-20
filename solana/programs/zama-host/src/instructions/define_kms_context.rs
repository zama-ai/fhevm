//! Defines a new KMS context (Solana mirror of `ProtocolConfig.defineNewKmsContext`).
//!
//! Creates the `KmsContext` PDA for the next sequential context id, records the KMS
//! node signer set + thresholds, and advances `HostConfig.current_kms_context_id`.
//! Admin-gated in the PoC; a gateway-sync authority would drive this in production
//! from `KMSVerifier.NewContextSet` events.

use anchor_lang::prelude::*;

use super::common::{
    assert_admin, assert_evm_signer_set, assert_no_remaining_accounts, assert_quorum_threshold,
    EvmSignerSetErrors,
};
#[cfg(feature = "emit-events")]
use crate::events::NewKmsContextEvent;
use crate::{errors::ZamaHostError, state::*};

/// Accounts for defining a new KMS context.
#[derive(Accounts)]
#[instruction(context_id: u64)]
pub struct DefineKmsContext<'info> {
    /// Configured host admin and rent payer for the context account.
    #[account(mut)]
    pub admin: Signer<'info>,
    /// Singleton config PDA; its `current_kms_context_id` is advanced to `context_id`.
    #[account(mut, seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    /// KMS context PDA created for `context_id`.
    #[account(
        init,
        payer = admin,
        space = 8 + KmsContext::SPACE,
        seeds = [KMS_CONTEXT_SEED, &context_id.to_le_bytes()],
        bump
    )]
    pub kms_context: Account<'info, KmsContext>,
    /// System program used for account creation.
    pub system_program: Program<'info, System>,
}

/// Records a new KMS context and makes it the active one.
pub fn define_kms_context(
    ctx: Context<DefineKmsContext>,
    context_id: u64,
    signers: Vec<[u8; 20]>,
    thresholds: KmsThresholds,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_admin(&ctx.accounts.host_config, ctx.accounts.admin.key())?;
    require!(
        context_id == ctx.accounts.host_config.current_kms_context_id + 1,
        ZamaHostError::InvalidKmsContextId
    );
    assert_evm_signer_set(
        &signers,
        KmsContext::MAX_SIGNERS,
        EvmSignerSetErrors {
            empty: ZamaHostError::EmptyKmsContext,
            too_many: ZamaHostError::TooManyKmsSigners,
            duplicate: ZamaHostError::DuplicateKmsSigner,
            zero: ZamaHostError::ZeroKmsSigner,
        },
    )?;
    let signer_count = signers.len();
    assert_quorum_threshold(
        thresholds.public_decryption,
        signer_count,
        ZamaHostError::InvalidKmsThreshold,
    )?;
    assert_quorum_threshold(
        thresholds.user_decryption,
        signer_count,
        ZamaHostError::InvalidKmsThreshold,
    )?;
    // `kms_gen` / `mpc` are stored for fidelity and may be zero; only an upper bound applies.
    require!(
        thresholds.kms_gen as usize <= signer_count && thresholds.mpc as usize <= signer_count,
        ZamaHostError::InvalidKmsThreshold
    );

    let kms_context = &mut ctx.accounts.kms_context;
    kms_context.context_id = context_id;
    kms_context.signers = signers.clone();
    kms_context.thresholds = thresholds;
    kms_context.destroyed = false;
    kms_context.bump = ctx.bumps.kms_context;
    ctx.accounts.host_config.current_kms_context_id = context_id;

    #[cfg(feature = "emit-events")]
    emit!(NewKmsContextEvent {
        version: EVENT_VERSION,
        kms_context_id: context_id,
        signers,
        public_decryption_threshold: thresholds.public_decryption,
        user_decryption_threshold: thresholds.user_decryption,
    });
    Ok(())
}
