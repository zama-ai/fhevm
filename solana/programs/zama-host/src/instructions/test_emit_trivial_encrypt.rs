//! Emits trivial-encrypt events from the test shim.

use anchor_lang::prelude::*;

use super::common::*;
use crate::{
    events::TrivialEncryptEvent,
    state::{HostConfig, EVENT_VERSION, HOST_CONFIG_SEED},
};

/// Accounts for authority-gated test event shims.
///
/// These shims do not prove or mutate protocol state and must not be treated as
/// production APIs. The generated Anchor account type is intentionally still
/// named `TestEmitProtocolEvent` for compatibility with existing callers.
#[derive(Accounts)]
#[event_cpi]
pub struct TestEmitProtocolEvent<'info> {
    /// Configured test-shim authority.
    pub test_authority: Signer<'info>,
    /// Singleton config PDA with `test_shims_enabled`.
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
}

/// Emits a trivial-encrypt event after test-shim authority checks.
pub fn test_emit_trivial_encrypt(
    ctx: Context<TestEmitProtocolEvent>,
    subject: Pubkey,
    plaintext: [u8; 32],
    fhe_type: u8,
    result: [u8; 32],
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_test_shim_authority(&ctx.accounts.host_config, ctx.accounts.test_authority.key())?;
    #[cfg(feature = "emit-events")]
    emit_cpi!(TrivialEncryptEvent {
        version: EVENT_VERSION,
        subject: subject.to_bytes(),
        plaintext,
        fhe_type,
        result,
    });
    Ok(())
}
