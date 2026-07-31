//! Coprocessor EIP-712 `CiphertextVerification` attestation verification (RFC-021).
//!
//! The coprocessor signs the attested handles + binding context as an EVM EIP-712 message; the host
//! recovers each signer via `secp256k1_recover` and requires at least `coprocessor_threshold`
//! distinct signatures from the registered coprocessor signer set (n-of-m, EVM `InputVerifier`
//! parity). This is the shared verifier used by the `fhe_execute` `VerifiedInput` operand (the Solana
//! `FHE.fromExternal` analog): verification creates no persistent ACL — the input is transient-
//! allowed for the consuming `fhe_execute` only, and the caller-is-contract check + any persistent output
//! ACL are enforced where the input is consumed.

use anchor_lang::prelude::*;

use crate::{
    eip712::{verify_coprocessor_input as verify_coprocessor_attestation, Eip712VerifierConfig},
    errors::ZamaHostError,
    state::*,
};

/// The host_config fields needed to verify an input attestation, copied out so callers that don't
/// hold a `&HostConfig` (the `fhe_execute` operand resolver) can carry them by value.
#[derive(Clone, Copy)]
pub(crate) struct InputVerifierParams {
    pub chain_id: u64,
    pub gateway_chain_id: u64,
    pub input_verification_contract: [u8; 20],
    /// Registered coprocessor signer set (fixed-cap; only the first `coprocessor_signer_count`
    /// entries are active).
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
    /// Count is write-validated (`≤ MAX`); clamp defends a corrupted singleton without panicking.
    fn active_signers(&self) -> &[[u8; 20]] {
        let count =
            (self.coprocessor_signer_count as usize).min(HostConfig::MAX_COPROCESSOR_SIGNERS);
        &self.coprocessor_signers[..count]
    }
}

/// Verifies the coprocessor's EIP-712 `CiphertextVerification` attestation for an encrypted input:
/// config sanity, per-handle metadata, selected-handle match, and `secp256k1_recover` of the
/// signers against the registered coprocessor signer set at the configured threshold. Used by the
/// `fhe_execute` `VerifiedInput` operand, which carries the whole attestation — taking it as one
/// value keeps the two 32-byte identities and two slices unswappable at the call site.
/// The attested `contract_address` is the input's natural ACL domain (EVM parity with the
/// verifyInput contract); the caller-is-contract gate is enforced by the operand resolver.
pub(crate) fn verify_input_attestation(
    params: &InputVerifierParams,
    attestation: &CoprocessorInputAttestation,
) -> Result<()> {
    require!(
        params.coprocessor_signer_count > 0 && params.input_verification_contract != [0u8; 20],
        ZamaHostError::GatewayVerifierConfigUnset
    );
    require!(
        !attestation.ct_handles.is_empty()
            && attestation.ct_handles.len() <= MAX_INPUT_ATTESTATION_HANDLES,
        ZamaHostError::MalformedInputAttestation
    );
    require!(
        attestation.extra_data.len() <= MAX_INPUT_ATTESTATION_EXTRA_DATA,
        ZamaHostError::MalformedInputAttestation
    );
    // EVM parity: InputVerifier requires `contractChainId == block.chainid`. The attested
    // `contract_chain_id` is the HOST chain id (not the gateway chain id used for the EIP-712 domain).
    require!(
        attestation.contract_chain_id == params.chain_id,
        ZamaHostError::AttestationChainIdMismatch
    );
    for (index, handle) in attestation.ct_handles.iter().enumerate() {
        assert_input_handle_metadata(*handle, params.chain_id, index as u8)?;
    }
    let selected = attestation
        .ct_handles
        .get(attestation.handle_index as usize)
        .ok_or(ZamaHostError::InvalidInputHandleIndex)?;
    require!(
        *selected == attestation.input_handle,
        ZamaHostError::InvalidInputHandle
    );

    let verifier = Eip712VerifierConfig {
        gateway_chain_id: params.gateway_chain_id,
        verifying_contract: params.input_verification_contract,
        signers: params.active_signers(),
        threshold: params.coprocessor_threshold,
    };
    require!(
        verify_coprocessor_attestation(
            &verifier,
            &attestation.ct_handles,
            &attestation.user_address,
            &attestation.contract_address,
            attestation.contract_chain_id,
            &attestation.extra_data,
            &attestation.signatures,
        ),
        ZamaHostError::InvalidInputAttestation
    );
    Ok(())
}
