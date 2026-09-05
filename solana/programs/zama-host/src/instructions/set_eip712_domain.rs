//! Writes the gateway EIP-712 domain fields together.

use anchor_lang::prelude::*;

use super::common::*;
use super::set_host_pause::HostAdmin;
use crate::errors::ZamaHostError;
use crate::state::SOLANA_CHAIN_TYPE_BIT;

/// Sets `gateway_chain_id`, `input_verification_contract`, and `decryption_contract`
/// together. Zeros are legal. The gateway chain id must leave the Solana chain-type bit clear.
pub fn set_eip712_domain(
    ctx: Context<HostAdmin>,
    gateway_chain_id: u64,
    input_verification_contract: [u8; 20],
    decryption_contract: [u8; 20],
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_admin(&ctx.accounts.host_config, &ctx.accounts.admin)?;
    require!(
        gateway_chain_id & SOLANA_CHAIN_TYPE_BIT == 0,
        ZamaHostError::InvalidChainTypeBit
    );
    let config = &ctx.accounts.host_config;
    if config.gateway_chain_id == gateway_chain_id
        && config.input_verification_contract == input_verification_contract
        && config.decryption_contract == decryption_contract
    {
        return Ok(());
    }
    let admin = ctx.accounts.admin.key();
    let config = &mut ctx.accounts.host_config;
    config.gateway_chain_id = gateway_chain_id;
    config.input_verification_contract = input_verification_contract;
    config.decryption_contract = decryption_contract;
    config.updated_slot = Clock::get()?.slot;
    emit_config_updated(
        &ctx.accounts.host_config,
        admin,
        &ctx.accounts.event_authority,
    )?;
    Ok(())
}
