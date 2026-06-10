//! Binds an encrypted input after verifying the coprocessor's EIP-712
//! `CiphertextVerification` attestation on-chain via `secp256k1_recover`.
//!
//! This is the gateway-compatible replacement for the native Ed25519
//! `verify_input_and_bind` / mock-input paths (RFC-021, issue #1494 Phase 3):
//! the coprocessor signs the attested handles + binding context as an EVM
//! EIP-712 message; the host recovers the signer and checks it against the
//! configured coprocessor signer set before binding the selected handle.

use anchor_lang::prelude::*;

use super::common::*;
use crate::{
    eip712::{verify_coprocessor_input, Eip712VerifierConfig},
    errors::ZamaHostError,
    events::{AclAllowedEvent, InputVerifiedEvent},
    state::*,
};

/// Accounts for the coprocessor-attested encrypted-input bind path.
#[derive(Accounts)]
#[instruction(
    input_handle: [u8; 32],
    ct_handles: Vec<[u8; 32]>,
    handle_index: u8,
    user_address: [u8; 32],
    contract_address: [u8; 32],
    contract_chain_id: u64,
    extra_data: Vec<u8>,
    signatures: Vec<[u8; 65]>,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64
)]
#[event_cpi]
pub struct VerifyCoprocessorInputAndBind<'info> {
    /// Pays rent for the output ACL record.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// App account signer authorizing the ACL metadata.
    pub app_account_authority: Signer<'info>,
    /// Singleton config PDA holding the gateway verifier trust config.
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    /// Canonical output ACL record created by this instruction.
    #[account(
        init,
        payer = payer,
        space = 8 + AclRecord::SPACE,
        seeds = [ACL_RECORD_SEED, output_nonce_key.as_ref(), &output_nonce_sequence.to_le_bytes()],
        bump
    )]
    pub output_acl_record: Account<'info, AclRecord>,
    /// System program used for ACL account creation.
    pub system_program: Program<'info, System>,
}

/// Binds an encrypted input after on-chain secp256k1 verification of the
/// coprocessor's EIP-712 `CiphertextVerification` attestation.
#[allow(clippy::too_many_arguments)]
pub fn verify_coprocessor_input_and_bind(
    ctx: Context<VerifyCoprocessorInputAndBind>,
    input_handle: [u8; 32],
    ct_handles: Vec<[u8; 32]>,
    handle_index: u8,
    user_address: [u8; 32],
    contract_address: [u8; 32],
    contract_chain_id: u64,
    extra_data: Vec<u8>,
    signatures: Vec<[u8; 65]>,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
    output_acl_domain_key: Pubkey,
    output_app_account: Pubkey,
    output_encrypted_value_label: [u8; 32],
    output_subjects: Vec<AclSubjectEntry>,
    output_public_decrypt: bool,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_not_paused(&ctx.accounts.host_config)?;

    let config = &ctx.accounts.host_config;
    require!(
        config.coprocessor_signer != [0u8; 20] && config.input_verification_contract != [0u8; 20],
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
        assert_input_handle_metadata(*handle, config.chain_id, index as u8)?;
    }
    let selected = ct_handles
        .get(handle_index as usize)
        .ok_or(ZamaHostError::InvalidInputHandleIndex)?;
    require!(*selected == input_handle, ZamaHostError::InvalidInputHandle);

    let verifier = Eip712VerifierConfig {
        gateway_chain_id: config.gateway_chain_id,
        verifying_contract: config.input_verification_contract,
        signers: std::slice::from_ref(&config.coprocessor_signer),
        threshold: 1,
    };
    require!(
        verify_coprocessor_input(
            &verifier,
            &ct_handles,
            &user_address,
            &contract_address,
            contract_chain_id,
            &extra_data,
            &signatures,
        ),
        ZamaHostError::InvalidInputAttestation
    );

    // Bind the ACL output to the identity the coprocessor actually attested (EVM parity:
    // on EVM the handle is allowed to the attested `contractAddress` for the attested user).
    // The attestation alone proves "the coprocessor blessed handle H for (user, contract)";
    // these checks stop a caller from stapling a valid attestation onto an app account or
    // subject set of their choosing. Combined with the `app_account_authority == output_app_account`
    // signer check in `assert_output_acl_metadata`, only the attested contract can bind the
    // handle into its own ACL domain, and only for the attested user.
    require!(
        contract_address == output_app_account.to_bytes(),
        ZamaHostError::InputBindContractMismatch
    );
    require!(
        output_subjects
            .iter()
            .any(|subject| subject.pubkey.to_bytes() == user_address),
        ZamaHostError::InputBindUserNotSubject
    );

    assert_output_acl_metadata(
        ctx.accounts.app_account_authority.key(),
        output_nonce_key,
        output_acl_domain_key,
        output_app_account,
        output_encrypted_value_label,
        &output_subjects,
    )?;
    assert_public_decrypt_not_set_at_birth(output_public_decrypt)?;

    write_acl_record(
        &mut ctx.accounts.output_acl_record,
        output_nonce_key,
        output_nonce_sequence,
        output_acl_domain_key,
        output_app_account,
        output_encrypted_value_label,
        input_handle,
        &output_subjects,
        output_public_decrypt,
        Clock::get()?.slot,
        ctx.bumps.output_acl_record,
    );

    emit_cpi!(InputVerifiedEvent {
        version: EVENT_VERSION,
        input_handle,
        result_handle: input_handle,
        user: user_address,
        acl_domain_key: output_acl_domain_key.to_bytes(),
    });
    emit_record_bound(
        ctx.accounts.output_acl_record.key(),
        &ctx.accounts.output_acl_record,
    );
    for output_subject in output_subjects {
        emit_cpi!(AclAllowedEvent {
            version: EVENT_VERSION,
            handle: input_handle,
            subject: output_subject.pubkey.to_bytes(),
        });
        emit_subject_event(
            ctx.accounts.output_acl_record.key(),
            input_handle,
            output_subject,
            Pubkey::default(),
        );
    }
    Ok(())
}
