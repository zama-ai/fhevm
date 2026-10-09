//! Activates a new epoch of the active KMS context (Solana mirror of
//! `ProtocolConfig.mirrorKmsEpoch`).
//!
//! A same-committee resharing keeps the context and its signer set and moves the KMS to new key
//! material, so only `HostConfig.current_kms_epoch_id` changes. As on EVM, `context_id` must be the
//! active context and `epoch_id` must be above the active epoch. The active context is never
//! destroyed (`destroy_kms_context` refuses it), so it needs no liveness check here. Admin-gated in
//! the PoC.

use anchor_lang::prelude::*;

use super::common::{assert_admin, assert_no_remaining_accounts};
use crate::event_cpi::emit_event_cpi;
use crate::events::NewKmsEpochEvent;
use crate::{errors::ZamaHostError, state::*};

/// Accounts for activating a new KMS epoch.
#[derive(Accounts)]
#[event_cpi]
pub struct DefineKmsEpoch<'info> {
    /// Configured host admin.
    pub admin: Signer<'info>,
    /// Singleton config PDA; its `current_kms_epoch_id` is set to `epoch_id`.
    #[account(mut, seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
}

/// Makes `epoch_id` the active epoch of the active context `context_id`.
pub fn define_kms_epoch(
    ctx: Context<DefineKmsEpoch>,
    context_id: [u8; 32],
    epoch_id: [u8; 32],
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_admin(&ctx.accounts.host_config, &ctx.accounts.admin)?;
    let host_config = &mut ctx.accounts.host_config;
    // Before the first context the active id is all-zero, which names no context.
    require!(
        context_id != [0u8; 32] && context_id == host_config.current_kms_context_id,
        ZamaHostError::InvalidKmsContext
    );
    // `[u8; 32]` compares lexicographically, which is EVM's big-endian `uint256` order.
    require!(
        epoch_id > host_config.current_kms_epoch_id,
        ZamaHostError::NonIncreasingKmsEpochId
    );
    host_config.current_kms_epoch_id = epoch_id;

    emit_event_cpi(
        &ctx.accounts.event_authority,
        &NewKmsEpochEvent {
            version: EVENT_VERSION,
            kms_context_id: context_id,
            kms_epoch_id: epoch_id,
        },
    )?;
    Ok(())
}
