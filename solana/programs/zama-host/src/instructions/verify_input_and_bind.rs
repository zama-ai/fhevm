//! Verifies signed native Solana encrypted inputs and binds handle ACL state.

use anchor_lang::prelude::*;
use solana_instructions_sysvar::{
    load_current_index_checked, load_instruction_at_checked, ID as INSTRUCTIONS_SYSVAR_ID,
};

use super::common::*;
use crate::{
    errors::ZamaHostError,
    events::{AclAllowedEvent, InputVerifiedEvent},
    state::*,
};

const ED25519_PROGRAM_ID: Pubkey =
    anchor_lang::pubkey!("Ed25519SigVerify111111111111111111111111111");

/// Accounts for the signed native Solana encrypted-input bind path.
#[derive(Accounts)]
#[instruction(
    input_handle: [u8; 32],
    proof: SolanaInputProof,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64
)]
#[event_cpi]
pub struct VerifyInputAndBind<'info> {
    /// Pays rent for the output ACL record.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: This account is only used for its pubkey. The handler requires it
    /// to match `HostConfig::input_verifier_authority` and verifies the matching
    /// Ed25519 pre-instruction over the canonical proof message.
    pub input_verifier_authority: UncheckedAccount<'info>,
    /// App account signer authorizing the ACL metadata.
    pub app_account_authority: Signer<'info>,
    /// Singleton config PDA.
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
    /// CHECK: Constrained to the instructions sysvar address and only read
    /// through `solana_instructions_sysvar`.
    #[account(address = INSTRUCTIONS_SYSVAR_ID)]
    pub instructions_sysvar: UncheckedAccount<'info>,
    /// System program used for ACL account creation.
    pub system_program: Program<'info, System>,
}

/// Binds an externally verified encrypted input after native Ed25519 proof checks.
pub fn verify_input_and_bind(
    ctx: Context<VerifyInputAndBind>,
    input_handle: [u8; 32],
    proof: SolanaInputProof,
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
    require_keys_eq!(
        ctx.accounts.input_verifier_authority.key(),
        ctx.accounts.host_config.input_verifier_authority,
        ZamaHostError::InputVerifierMismatch
    );
    let bind_intent = SolanaInputBindIntent {
        output_nonce_key,
        output_nonce_sequence,
        output_acl_domain_key,
        output_app_account,
        output_encrypted_value_label,
        output_subjects: output_subjects.clone(),
        output_public_decrypt,
    };
    assert_input_proof(
        &proof,
        input_handle,
        ctx.accounts.host_config.chain_id,
        output_acl_domain_key,
        output_app_account,
    )?;
    assert_output_acl_metadata(
        ctx.accounts.app_account_authority.key(),
        output_nonce_key,
        output_acl_domain_key,
        output_app_account,
        output_encrypted_value_label,
        &output_subjects,
    )?;
    assert_public_decrypt_not_set_at_birth(output_public_decrypt)?;

    let proof_message = input_proof_message(
        &proof,
        &bind_intent,
        crate::ID,
        ctx.accounts.host_config.chain_id,
    );
    assert_previous_ed25519_instruction(
        &ctx.accounts.instructions_sysvar.to_account_info(),
        ctx.accounts.input_verifier_authority.key(),
        &proof_message,
    )?;

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
        user: proof.user.to_bytes(),
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

pub(super) fn assert_input_proof(
    proof: &SolanaInputProof,
    input_handle: [u8; 32],
    chain_id: u64,
    output_acl_domain_key: Pubkey,
    output_app_account: Pubkey,
) -> Result<()> {
    require!(
        !proof.handles.is_empty() && proof.handles.len() <= MAX_INPUT_PROOF_HANDLES,
        ZamaHostError::InvalidInputProof
    );
    require!(
        proof.extra_data.len() <= MAX_INPUT_PROOF_EXTRA_DATA,
        ZamaHostError::InvalidInputProof
    );
    for (index, handle) in proof.handles.iter().enumerate() {
        assert_input_handle_metadata(*handle, chain_id, index as u8)?;
    }
    let selected = proof
        .handles
        .get(proof.handle_index as usize)
        .ok_or(ZamaHostError::InvalidInputHandleIndex)?;
    require!(*selected == input_handle, ZamaHostError::InvalidInputHandle);
    require_keys_eq!(
        proof.app_account,
        output_app_account,
        ZamaHostError::InvalidInputProof
    );
    require_keys_eq!(
        proof.acl_domain_key,
        output_acl_domain_key,
        ZamaHostError::InvalidInputProof
    );
    require!(
        proof.user != Pubkey::default(),
        ZamaHostError::InvalidInputProof
    );
    Ok(())
}

pub(super) fn assert_previous_ed25519_instruction(
    instructions_sysvar: &AccountInfo,
    verifier: Pubkey,
    message: &[u8],
) -> Result<()> {
    require_keys_eq!(
        instructions_sysvar.key(),
        INSTRUCTIONS_SYSVAR_ID,
        ZamaHostError::InputProofSignatureMissing
    );
    let current_index = load_current_index_checked(instructions_sysvar)
        .map_err(|_| error!(ZamaHostError::InputProofSignatureMissing))?;
    let verifier_index = current_index
        .checked_sub(1)
        .ok_or(ZamaHostError::InputProofSignatureMissing)?;
    let verifier_ix = load_instruction_at_checked(verifier_index as usize, instructions_sysvar)
        .map_err(|_| error!(ZamaHostError::InputProofSignatureMissing))?;
    require_keys_eq!(
        verifier_ix.program_id,
        ED25519_PROGRAM_ID,
        ZamaHostError::InputProofSignatureMissing
    );
    require!(
        solana_ed25519_instruction::instruction_contains_message(
            &verifier_ix.data,
            verifier.as_ref(),
            message,
        ),
        ZamaHostError::InputProofSignatureMissing
    );
    Ok(())
}
