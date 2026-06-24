//! Commits ciphertext material readiness for host-owned handles.

use anchor_lang::prelude::*;

use super::common::*;
use crate::{
    errors::ZamaHostError,
    events::{HandleMaterialCommittedEvent, HandleMaterialSealedEvent},
    state::*,
};

/// Accounts for committing ciphertext material readiness for a handle.
#[derive(Accounts)]
#[event_cpi]
pub struct CommitHandleMaterial<'info> {
    /// Pays rent for the material commitment account.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Configured material commitment authority.
    pub material_authority: Signer<'info>,
    /// Singleton host config PDA.
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    /// Canonical ACL record for the handle whose material is committed and sealed.
    #[account(mut)]
    pub acl_record: Account<'info, AclRecord>,
    /// One-shot material commitment PDA for this ACL record.
    #[account(
        init,
        payer = payer,
        space = 8 + HandleMaterialCommitment::SPACE,
        seeds = [HANDLE_MATERIAL_SEED, host_config.key().as_ref(), acl_record.key().as_ref()],
        bump
    )]
    pub material_commitment: Account<'info, HandleMaterialCommitment>,
    /// System program used for account creation.
    pub system_program: Program<'info, System>,
}

/// Commits ciphertext material availability for a host-owned handle.
pub fn commit_handle_material(
    ctx: Context<CommitHandleMaterial>,
    key_id: [u8; 32],
    ciphertext_digest: [u8; 32],
    sns_ciphertext_digest: [u8; 32],
    coprocessor_set_digest: [u8; 32],
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_not_paused(&ctx.accounts.host_config)?;
    require_keys_eq!(
        ctx.accounts.material_authority.key(),
        ctx.accounts.host_config.material_authority,
        ZamaHostError::MaterialAuthorityMismatch
    );
    assert_canonical_acl_record(
        &ctx.accounts.acl_record.to_account_info(),
        &ctx.accounts.acl_record,
    )?;
    assert_handle_for_chain(
        ctx.accounts.acl_record.handle,
        ctx.accounts.host_config.chain_id,
    )?;
    require!(
        key_id != [0; 32]
            && ciphertext_digest != [0; 32]
            && sns_ciphertext_digest != [0; 32]
            && coprocessor_set_digest != [0; 32],
        ZamaHostError::InvalidMaterialCommitment
    );
    require!(
        ctx.accounts.acl_record.material_commitment == Pubkey::default()
            && ctx.accounts.acl_record.material_commitment_hash == [0; 32]
            && ctx.accounts.acl_record.material_key_id == [0; 32],
        ZamaHostError::MaterialAlreadySealed
    );

    let acl_record_key = ctx.accounts.acl_record.key();
    let material_commitment_key = ctx.accounts.material_commitment.key();
    let (expected_key, expected_bump) = handle_material_address(acl_record_key);
    require_keys_eq!(
        material_commitment_key,
        expected_key,
        ZamaHostError::MaterialCommitmentPdaMismatch
    );
    require!(
        ctx.bumps.material_commitment == expected_bump,
        ZamaHostError::MaterialCommitmentPdaMismatch
    );

    let commitment_hash = handle_material_commitment_hash(
        material_commitment_key,
        acl_record_key,
        key_id,
        ciphertext_digest,
        sns_ciphertext_digest,
        coprocessor_set_digest,
    );
    let created_slot = Clock::get()?.slot;
    let commitment = &mut ctx.accounts.material_commitment;
    commitment.acl_record = acl_record_key;
    commitment.handle = ctx.accounts.acl_record.handle;
    commitment.key_id = key_id;
    commitment.ciphertext_digest = ciphertext_digest;
    commitment.sns_ciphertext_digest = sns_ciphertext_digest;
    commitment.coprocessor_set_digest = coprocessor_set_digest;
    commitment.material_commitment_hash = commitment_hash;
    commitment.created_slot = created_slot;
    commitment.state = HANDLE_MATERIAL_STATE_COMMITTED;
    commitment.bump = expected_bump;

    let acl_record = &mut ctx.accounts.acl_record;
    acl_record.material_commitment = material_commitment_key;
    acl_record.material_commitment_hash = commitment_hash;
    acl_record.material_key_id = key_id;

    #[cfg(feature = "emit-events")]
    emit_cpi!(HandleMaterialCommittedEvent {
        version: EVENT_VERSION,
        material_commitment: material_commitment_key,
        acl_record: acl_record_key,
        handle: commitment.handle,
        key_id,
        ciphertext_digest,
        sns_ciphertext_digest,
        coprocessor_set_digest,
        material_commitment_hash: commitment_hash,
        created_slot,
    });
    #[cfg(feature = "emit-events")]
    emit_cpi!(HandleMaterialSealedEvent {
        version: EVENT_VERSION,
        material_commitment: material_commitment_key,
        acl_record: acl_record_key,
        handle: commitment.handle,
        key_id,
        material_commitment_hash: commitment_hash,
        updated_slot: created_slot,
    });
    Ok(())
}
