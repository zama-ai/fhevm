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

/// Verifies the coprocessor's EIP-712 `CiphertextVerification` attestation for an encrypted input:
/// config sanity, per-handle metadata, selected-handle match, and `secp256k1_recover` of the
/// signers against the registered coprocessor signer set at the configured threshold. Success
/// means a quorum signed this blob; the caller is responsible for the contract bind
/// (`attestation.contract_address == compute_subject`). Used by the `fhe_execute`
/// `VerifiedInput` operand, which carries the whole attestation — taking it as one value keeps
/// the two 32-byte identities and two slices unswappable at the call site.
pub(crate) fn verify_input_attestation(
    config: &HostConfig,
    attestation: &CoprocessorInputAttestation,
) -> Result<()> {
    require!(
        !config.active_coprocessor_signers().is_empty()
            && config.input_verification_contract != [0u8; 20],
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
        attestation.contract_chain_id == config.chain_id,
        ZamaHostError::AttestationChainIdMismatch
    );
    for (index, handle) in attestation.ct_handles.iter().enumerate() {
        assert_input_handle_metadata(*handle, config.chain_id, index as u8)?;
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
        gateway_chain_id: config.gateway_chain_id,
        verifying_contract: config.input_verification_contract,
        signers: config.active_coprocessor_signers(),
        threshold: config.coprocessor_threshold,
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
