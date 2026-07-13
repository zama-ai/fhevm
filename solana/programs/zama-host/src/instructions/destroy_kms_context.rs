//! Destroys a KMS context (Solana mirror of `ProtocolConfig.destroyKmsContext`).
//!
//! Marks a non-current context as destroyed so its signer set can no longer certify
//! decryptions. The current active context cannot be destroyed. Admin-gated.

use anchor_lang::prelude::*;

use super::common::{assert_admin, assert_no_remaining_accounts};
#[cfg(feature = "emit-events")]
use crate::events::KmsContextDestroyedEvent;
use crate::{errors::ZamaHostError, state::*};

/// Accounts for destroying a KMS context.
#[derive(Accounts)]
#[instruction(context_id: u64)]
pub struct DestroyKmsContext<'info> {
    /// Configured host admin.
    pub admin: Signer<'info>,
    /// Singleton config PDA holding the current active context id.
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    /// KMS context PDA being destroyed.
    #[account(
        mut,
        seeds = [KMS_CONTEXT_SEED, &context_id.to_le_bytes()],
        bump = kms_context.bump
    )]
    pub kms_context: Account<'info, KmsContext>,
}

/// Marks a non-current KMS context as destroyed.
pub fn destroy_kms_context(ctx: Context<DestroyKmsContext>, context_id: u64) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_admin(&ctx.accounts.host_config, ctx.accounts.admin.key())?;
    require!(
        context_id != ctx.accounts.host_config.current_kms_context_id,
        ZamaHostError::CurrentKmsContextCannotBeDestroyed
    );
    ctx.accounts.kms_context.destroyed = true;

    #[cfg(feature = "emit-events")]
    emit!(KmsContextDestroyedEvent {
        version: EVENT_VERSION,
        kms_context_id: context_id,
    });
    Ok(())
}
