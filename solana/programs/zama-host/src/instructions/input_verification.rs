//! Coprocessor EIP-712 `CiphertextVerification` attestation verification (RFC-021).
//!
//! The coprocessor signs the attested handles + binding context as an EVM EIP-712 message; the host
//! recovers each signer via `secp256k1_recover` and requires at least `coprocessor_threshold`
//! distinct signatures from the registered coprocessor signer set (n-of-m, EVM `InputVerifier`
//! parity). This is the shared verifier used by the `fhe_eval` `VerifiedInput` operand (the Solana
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
    /// Registered coprocessor signer set (fixed-cap; only the first `coprocessor_signer_count`
    /// entries are active). Carried by value so the `fhe_eval` operand resolver, which does not
    /// hold a `&HostConfig`, can verify attestations against the whole set.
    pub coprocessor_signers: [[u8; 20]; HostConfig::MAX_COPROCESSOR_SIGNERS],
    pub coprocessor_signer_count: u8,
    pub coprocessor_threshold: u8,
}

impl InputVerifierParams {
    pub fn from_config(config: &HostConfig) -> Self {
        Self {
            chain_id: config.chain_id,
            gateway_chain_id: config.gateway_chain_id,
            input_verification_contract: config.input_verification_contract,
            coprocessor_signers: config.coprocessor_signers,
            coprocessor_signer_count: config.coprocessor_signer_count,
            coprocessor_threshold: config.coprocessor_threshold,
        }
    }

    /// Active coprocessor signer set (the first `coprocessor_signer_count` entries).
    fn active_signers(&self) -> &[[u8; 20]] {
        &self.coprocessor_signers[..self.coprocessor_signer_count as usize]
    }
}

/// Verifies the coprocessor's EIP-712 `CiphertextVerification` attestation for an encrypted input:
/// config sanity, per-handle metadata, selected-handle match, and `secp256k1_recover` of the
/// signers against the registered coprocessor signer set at the configured threshold. Used by the
/// `fhe_eval` `VerifiedInput` operand.
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
        params.coprocessor_signer_count > 0 && params.input_verification_contract != [0u8; 20],
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
    // EVM parity: InputVerifier requires `contractChainId == block.chainid`. The attested
    // `contract_chain_id` is the HOST chain id (not the gateway chain id used for the EIP-712 domain).
    require!(
        contract_chain_id == params.chain_id,
        ZamaHostError::AttestationChainIdMismatch
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
        signers: params.active_signers(),
        threshold: params.coprocessor_threshold,
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
