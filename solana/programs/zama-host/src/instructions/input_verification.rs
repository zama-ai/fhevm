//! Coprocessor EIP-712 `CiphertextVerification` attestation verification (RFC-021).
//!
//! The coprocessor signs the attested handles + binding context as an EVM EIP-712 message; the host
//! recovers the signer via `secp256k1_recover` and checks it against the configured coprocessor
//! signer. This is the shared verifier used by the `fhe_eval` `VerifiedInput` operand (the Solana
//! `FHE.fromExternal` analog): verification creates no persistent ACL — the input is transient-
//! allowed for the consuming `fhe_eval` only, and the caller-is-contract check + any durable output
//! ACL are enforced where the input is consumed.

use anchor_lang::prelude::*;

use crate::{
    eip712::{verify_coprocessor_input as verify_coprocessor_attestation, Eip712VerifierConfig},
    errors::ZamaHostError,
    state::*,
};

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
/// signer against the configured coprocessor signer. Used by the `fhe_eval` `VerifiedInput` operand.
/// The attested `contract_address` is the input's natural ACL domain (EVM parity with the
/// verifyInput contract); the caller-is-contract gate is enforced by the operand resolver.
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
