//! Emits trivial-encrypt events from the test shim.

use anchor_lang::prelude::*;

use super::common::*;
use super::test_emit_acl_allowed::TestEmitProtocolEvent;
use crate::{events::TrivialEncryptEvent, state::EVENT_VERSION};

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
