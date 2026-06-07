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
    /// Active threshold verifier set for input proofs.
    pub input_verifier_set: Box<Account<'info, VerifierSet>>,
    /// App account signer authorizing the ACL metadata.
    pub app_account_authority: Signer<'info>,
    /// Singleton config PDA.
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Box<Account<'info, HostConfig>>,
    /// Canonical output ACL record created by this instruction.
    #[account(
        init,
        payer = payer,
        space = 8 + AclRecord::SPACE,
        seeds = [ACL_RECORD_SEED, output_nonce_key.as_ref(), &output_nonce_sequence.to_le_bytes()],
        bump
    )]
    pub output_acl_record: Box<Account<'info, AclRecord>>,
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
    assert_input_verifier_set(&ctx.accounts.host_config, &ctx.accounts.input_verifier_set)?;
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

    let proof_message = input_proof_message_for_verifier_set(
        &proof,
        &bind_intent,
        crate::ID,
        ctx.accounts.host_config.chain_id,
        ctx.accounts.input_verifier_set.key(),
        ctx.accounts.input_verifier_set.kind,
        ctx.accounts.input_verifier_set.scope,
        ctx.accounts.input_verifier_set.version,
    );
    assert_threshold_ed25519_instructions(
        &ctx.accounts.instructions_sysvar.to_account_info(),
        &ctx.accounts.input_verifier_set,
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

pub(super) fn assert_input_verifier_set(
    host_config: &Account<HostConfig>,
    verifier_set: &Account<VerifierSet>,
) -> Result<()> {
    require_keys_eq!(
        verifier_set.key(),
        host_config.input_verifier_set,
        ZamaHostError::InputVerifierMismatch
    );
    require!(
        host_config.input_verifier_set != Pubkey::default()
            && verifier_set.version == host_config.input_verifier_set_version,
        ZamaHostError::InputVerifierMismatch
    );
    assert_verifier_set_shape(
        verifier_set,
        VERIFIER_SET_KIND_INPUT,
        host_config.key(),
        verifier_set.version,
    )?;
    require!(verifier_set.is_active(), ZamaHostError::VerifierSetDisabled);
    Ok(())
}

pub(super) fn read_input_verifier_set<'info>(
    host_config: &Account<HostConfig>,
    verifier_set_info: &'info AccountInfo<'info>,
) -> Result<Account<'info, VerifierSet>> {
    let verifier_set = Account::<VerifierSet>::try_from(verifier_set_info)
        .map_err(|_| error!(ZamaHostError::VerifierSetMismatch))?;
    assert_input_verifier_set(host_config, &verifier_set)?;
    Ok(verifier_set)
}

pub(super) fn assert_threshold_ed25519_instructions(
    instructions_sysvar: &AccountInfo,
    verifier_set: &VerifierSet,
    message: &[u8],
) -> Result<()> {
    require_keys_eq!(
        instructions_sysvar.key(),
        INSTRUCTIONS_SYSVAR_ID,
        ZamaHostError::InputProofSignatureMissing
    );
    let current_index = load_current_index_checked(instructions_sysvar)
        .map_err(|_| error!(ZamaHostError::InputProofSignatureMissing))?;
    let signer_refs: Vec<&[u8]> = verifier_set
        .signer_slice()
        .iter()
        .map(Pubkey::as_ref)
        .collect();
    let mut matched = 0u128;
    let mut saw_adjacent_ed25519 = false;
    let mut instruction_index = current_index
        .checked_sub(1)
        .ok_or(ZamaHostError::InputProofSignatureMissing)?;
    loop {
        let verifier_ix =
            load_instruction_at_checked(instruction_index as usize, instructions_sysvar)
                .map_err(|_| error!(ZamaHostError::InputProofSignatureMissing))?;
        msg!(
            "threshold scan ix {} program {}",
            instruction_index,
            verifier_ix.program_id
        );
        if verifier_ix.program_id != ED25519_PROGRAM_ID {
            break;
        }
        saw_adjacent_ed25519 = true;
        let bitmask = solana_ed25519_instruction::matching_signer_bitmask(
            &verifier_ix.data,
            &signer_refs,
            message,
        )
        .map_err(|error| match error {
            solana_ed25519_instruction::MatchingSignerError::DuplicateSigner => {
                ZamaHostError::VerifierSetDuplicateSigner
            }
            solana_ed25519_instruction::MatchingSignerError::TooManyExpectedPubkeys => {
                ZamaHostError::VerifierSetMismatch
            }
        })?;
        msg!("threshold bitmask {}", bitmask);
        require!(
            matched & bitmask == 0,
            ZamaHostError::VerifierSetDuplicateSigner
        );
        matched |= bitmask;
        if matched.count_ones() as u8 >= verifier_set.threshold {
            return Ok(());
        }
        if instruction_index == 0 {
            break;
        }
        instruction_index -= 1;
    }
    require!(
        saw_adjacent_ed25519,
        ZamaHostError::InputProofSignatureMissing
    );
    require!(
        matched.count_ones() as u8 >= verifier_set.threshold,
        ZamaHostError::VerifierSetThresholdNotMet
    );
    Ok(())
}
