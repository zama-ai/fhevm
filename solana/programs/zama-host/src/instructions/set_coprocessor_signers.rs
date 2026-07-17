//! Sets the registered coprocessor signer set + threshold used to verify input attestations.
//!
//! EVM `InputVerifier` parity: input `CiphertextVerification` attestations are verified against a
//! registered n-of-m coprocessor signer set rather than a single hardcoded signer. This admin-gated
//! setter rotates that set + threshold in place on the singleton `HostConfig`, following the same
//! admin + pause-neutral pattern as the other `set_*` config instructions. In production a
//! gateway-sync authority would drive this from the EVM `GatewayConfig` coprocessor registry.

use anchor_lang::prelude::*;

use super::common::*;
use super::set_host_pause::HostAdmin;

/// Replaces the coprocessor signer set + threshold. Admin-gated.
///
/// Enforced guarantees (via `validate_and_pack_coprocessor_signers`):
/// - The admin must sign and match `host_config.admin` (`assert_admin`).
/// - Rejects any trailing accounts (`assert_no_remaining_accounts`).
/// - Non-empty set, within `HostConfig::MAX_COPROCESSOR_SIGNERS`, `1 <= threshold <= set.len()`,
///   no zero-address signer, no duplicate signer.
/// - Advances `updated_slot` and emits the config-updated event.
pub fn set_coprocessor_signers(
    ctx: Context<HostAdmin>,
    signers: Vec<[u8; 20]>,
    threshold: u8,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_admin(&ctx.accounts.host_config, ctx.accounts.admin.key())?;
    let (packed, count) = validate_and_pack_coprocessor_signers(&signers, threshold)?;
    let admin = ctx.accounts.admin.key();
    let config = &mut ctx.accounts.host_config;
    config.coprocessor_signers = packed;
    config.coprocessor_signer_count = count;
    config.coprocessor_threshold = threshold;
    config.updated_slot = Clock::get()?.slot;
    emit_config_updated(config, admin);
    Ok(())
}
