//! Emits random-ciphertext events from the test shim.

use anchor_lang::prelude::*;

use super::common::*;
use super::test_emit_acl_allowed::TestEmitProtocolEvent;
use crate::{events::FheRandEvent, state::EVENT_VERSION};

/// Emits a random-ciphertext event after test-shim authority checks.
pub fn test_emit_fhe_rand(
    ctx: Context<TestEmitProtocolEvent>,
    subject: Pubkey,
    seed: [u8; 16],
    fhe_type: u8,
    result: [u8; 32],
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_test_shim_authority(&ctx.accounts.host_config, ctx.accounts.test_authority.key())?;
    #[cfg(feature = "emit-events")]
    emit_cpi!(FheRandEvent {
        version: EVENT_VERSION,
        subject: subject.to_bytes(),
        seed,
        fhe_type,
        result,
    });
    Ok(())
}
