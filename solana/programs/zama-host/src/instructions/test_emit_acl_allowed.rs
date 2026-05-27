use anchor_lang::prelude::*;

use super::common::*;
use crate::{events::AclAllowedEvent, state::EVENT_VERSION};

/// Emits the legacy ACL-allowed event after test-shim authority checks.
pub fn test_emit_acl_allowed(
    ctx: Context<TestEmitProtocolEvent>,
    handle: [u8; 32],
    subject: Pubkey,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_test_shim_authority(&ctx.accounts.host_config, ctx.accounts.test_authority.key())?;
    emit_cpi!(AclAllowedEvent {
        version: EVENT_VERSION,
        handle,
        subject: subject.to_bytes(),
    });
    Ok(())
}
