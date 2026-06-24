//! Emits input-verified events from the test shim.

use anchor_lang::prelude::*;

use super::common::*;
use super::test_emit_acl_allowed::TestEmitProtocolEvent;
use crate::{events::InputVerifiedEvent, state::EVENT_VERSION};

/// Emits an input-verified event after test-shim authority checks.
pub fn test_emit_input_verified(
    ctx: Context<TestEmitProtocolEvent>,
    input_handle: [u8; 32],
    result_handle: [u8; 32],
    user: Pubkey,
    acl_domain_key: Pubkey,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_test_shim_authority(&ctx.accounts.host_config, ctx.accounts.test_authority.key())?;
    #[cfg(feature = "emit-events")]
    emit_cpi!(InputVerifiedEvent {
        version: EVENT_VERSION,
        input_handle,
        result_handle,
        user: user.to_bytes(),
        acl_domain_key: acl_domain_key.to_bytes(),
    });
    Ok(())
}
