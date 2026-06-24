//! Verifies the coprocessor's EIP-712 `CiphertextVerification` attestation on-chain
//! via `secp256k1_recover` and emits the verified-input receipt.
//!
//! Gateway-compatible encrypted-input path (RFC-021, issue #1494 Phase 3): the
//! coprocessor signs the attested handles + binding context as an EVM EIP-712 message;
//! the host recovers the signer and checks it against the configured coprocessor signer
//! set before emitting `InputVerifiedEvent`.
//!
//! EVM parity (`FHEVMExecutor.verifyInput`): verification creates NO persistent ACL. On
//! EVM a verified input gets a tx-scoped transient allow; Solana has no transient-storage
//! analog, so the verified input is surfaced solely as the signed `InputVerifiedEvent`
//! receipt, and any durable permission on an input-derived handle is a separate, explicit
//! app grant. This avoids persisting (and paying rent for) one ACL account per input.

use anchor_lang::prelude::*;

use super::common::*;
use crate::{
    eip712::{verify_coprocessor_input as verify_coprocessor_attestation, Eip712VerifierConfig},
    errors::ZamaHostError,
    events::InputVerifiedEvent,
    state::*,
};

/// Accounts for the coprocessor-attested encrypted-input verification path.
#[derive(Accounts)]
#[event_cpi]
pub struct VerifyCoprocessorInput<'info> {
    /// Singleton config PDA holding the gateway verifier trust config.
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
}

/// The host_config fields needed to verify an input attestation, copied out so callers that don't
/// hold a `&HostConfig` (the `fhe_eval` operand resolver) can carry them by value.
#[derive(Clone, Copy)]
pub(crate) struct InputVerifierParams {
    pub chain_id: u64,
    pub gateway_chain_id: u64,
    pub input_verification_contract: [u8; 20],
    pub coprocessor_signer: [u8; 20],
}

impl InputVerifierParams {
    pub fn from_config(config: &HostConfig) -> Self {
        Self {
            chain_id: config.chain_id,
            gateway_chain_id: config.gateway_chain_id,
            input_verification_contract: config.input_verification_contract,
            coprocessor_signer: config.coprocessor_signer,
        }
    }
}

/// Verifies the coprocessor's EIP-712 `CiphertextVerification` attestation for an encrypted input:
/// config sanity, per-handle metadata, selected-handle match, and `secp256k1_recover` of the
/// signer against the configured coprocessor signer. Shared by the standalone instruction and the
/// `fhe_eval` `VerifiedInput` operand. The attested `contract_address` IS the input's ACL domain
/// key (the natural domain for an input-derived handle, EVM parity with the verifyInput contract).
#[allow(clippy::too_many_arguments)]
pub(crate) fn verify_input_attestation(
    params: &InputVerifierParams,
    input_handle: [u8; 32],
    ct_handles: &[[u8; 32]],
    handle_index: u8,
    user_address: &[u8; 32],
    contract_address: &[u8; 32],
    contract_chain_id: u64,
    extra_data: &[u8],
    signatures: &[[u8; 65]],
) -> Result<()> {
    require!(
        params.coprocessor_signer != [0u8; 20] && params.input_verification_contract != [0u8; 20],
        ZamaHostError::GatewayVerifierConfigUnset
    );
    require!(
        !ct_handles.is_empty() && ct_handles.len() <= MAX_INPUT_PROOF_HANDLES,
        ZamaHostError::InvalidInputProof
    );
    require!(
        extra_data.len() <= MAX_INPUT_PROOF_EXTRA_DATA,
        ZamaHostError::InvalidInputProof
    );
    for (index, handle) in ct_handles.iter().enumerate() {
        assert_input_handle_metadata(*handle, params.chain_id, index as u8)?;
    }
    let selected = ct_handles
        .get(handle_index as usize)
        .ok_or(ZamaHostError::InvalidInputHandleIndex)?;
    require!(*selected == input_handle, ZamaHostError::InvalidInputHandle);

    let verifier = Eip712VerifierConfig {
        gateway_chain_id: params.gateway_chain_id,
        verifying_contract: params.input_verification_contract,
        signers: std::slice::from_ref(&params.coprocessor_signer),
        threshold: 1,
    };
    require!(
        verify_coprocessor_attestation(
            &verifier,
            ct_handles,
            user_address,
            contract_address,
            contract_chain_id,
            extra_data,
            signatures,
        ),
        ZamaHostError::InvalidInputAttestation
    );
    Ok(())
}

/// Verifies an encrypted input by recovering the coprocessor's EIP-712
/// `CiphertextVerification` attestation on-chain and emitting the verified-input receipt.
#[allow(clippy::too_many_arguments)]
pub fn verify_coprocessor_input(
    ctx: Context<VerifyCoprocessorInput>,
    input_handle: [u8; 32],
    ct_handles: Vec<[u8; 32]>,
    handle_index: u8,
    user_address: [u8; 32],
    contract_address: [u8; 32],
    contract_chain_id: u64,
    extra_data: Vec<u8>,
    signatures: Vec<[u8; 65]>,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_not_paused(&ctx.accounts.host_config)?;

    verify_input_attestation(
        &InputVerifierParams::from_config(&ctx.accounts.host_config),
        input_handle,
        &ct_handles,
        handle_index,
        &user_address,
        &contract_address,
        contract_chain_id,
        &extra_data,
        &signatures,
    )?;

    // Verification only: the signed attestation proves "the coprocessor blessed handle H for
    // (user, contract)", which we surface as the receipt below. No ACL is created here — durable
    // permission on an input-derived handle is a separate, explicit app grant (EVM parity).
    // `acl_domain_key` carries the attested contract identity, the natural ACL domain for the input.
    #[cfg(feature = "emit-events")]
    emit_cpi!(InputVerifiedEvent {
        version: EVENT_VERSION,
        input_handle,
        result_handle: input_handle,
        user: user_address,
        acl_domain_key: contract_address,
    });
    Ok(())
}
