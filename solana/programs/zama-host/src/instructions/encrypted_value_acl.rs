//! Encrypted-value ACL lifecycle instructions (fhevm-internal#1569 / RFC-024).
//!
//! One [`EncryptedValueAcl`] per encrypted-value lineage, reused across every
//! rotation. The app program (e.g. `confidential-token`) signs as
//! `app_account_authority` — the account that keys the lineage — and drives:
//!
//! - `initialize_encrypted_value_acl`: create the lineage at its first handle.
//! - `rotate_encrypted_value`: record a historical-access leaf per current
//!   subject for the old handle, then set the new handle/subjects.
//! - `allow_encrypted_value_subjects`: extend current durable membership.
//! - `mark_encrypted_value_public`: record an exact public-decrypt leaf.
//!
//! The layout, MMR, and authorization live in the shared `zama_solana_acl` crate
//! (so the KMS cannot drift). The account is a program-owned blob loaded/stored
//! through that crate's codec and grown with `realloc` as history accrues; a
//! lineage that is never rotated keeps `leaf_count == 0` and stays tiny. There
//! is no on-chain decrypt gate — the KMS verifies proofs off-chain.

use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction};

use super::common::{assert_no_remaining_accounts, create_pda_strict};
use crate::errors::ZamaHostError;
use crate::mmr::mmr_append;
use crate::state::*;
use zama_solana_acl::acl_nonce_key;

fn map_acl_err(error: AclError) -> Error {
    match error {
        AclError::MmrInconsistent => ZamaHostError::MmrStateInconsistent.into(),
        AclError::SubjectCapacityExceeded => {
            ZamaHostError::EncryptedValueSubjectCapacityExceeded.into()
        }
        AclError::SubjectMissing => ZamaHostError::EncryptedValueSubjectMissing.into(),
        AclError::HandleMismatch => ZamaHostError::EncryptedValueHandleMismatch.into(),
        AclError::HistoricalProofInvalid => ZamaHostError::HistoricalAccessProofInvalid.into(),
        AclError::PublicDecryptProofInvalid => ZamaHostError::PublicDecryptProofInvalid.into(),
        AclError::BadDiscriminator | AclError::BadAccountData => {
            ZamaHostError::EncryptedValueAclPdaMismatch.into()
        }
    }
}

/// Maps a subject pubkey set to canonical bytes: nonempty, distinct, within capacity.
fn checked_subjects(subjects: &[Pubkey]) -> Result<Vec<[u8; 32]>> {
    require!(
        !subjects.is_empty() && subjects.len() <= MAX_ENCRYPTED_VALUE_SUBJECTS,
        ZamaHostError::EncryptedValueSubjectCapacityExceeded
    );
    let mut out = Vec::with_capacity(subjects.len());
    for subject in subjects {
        require!(
            *subject != Pubkey::default(),
            ZamaHostError::EncryptedValueSubjectMissing
        );
        let bytes = subject.to_bytes();
        require!(
            !out.contains(&bytes),
            ZamaHostError::EncryptedValueSubjectCapacityExceeded
        );
        out.push(bytes);
    }
    Ok(out)
}

/// Loads and canonically validates the lineage at `info` (program-owned, canonical PDA).
fn load_lineage(info: &AccountInfo) -> Result<EncryptedValueAcl> {
    require_keys_eq!(
        *info.owner,
        crate::ID,
        ZamaHostError::EncryptedValueAclPdaMismatch
    );
    let acl = decode_account(&info.try_borrow_data()?).map_err(map_acl_err)?;
    let value_key = acl_nonce_key(
        acl.acl_domain_key,
        acl.app_account,
        acl.encrypted_value_label,
    );
    let (expected, expected_bump) = encrypted_value_acl_address(value_key);
    require_keys_eq!(
        info.key(),
        expected,
        ZamaHostError::EncryptedValueAclPdaMismatch
    );
    require!(
        acl.bump == expected_bump && acl.subjects.len() <= MAX_ENCRYPTED_VALUE_SUBJECTS,
        ZamaHostError::EncryptedValueAclPdaMismatch
    );
    Ok(acl)
}

/// Serializes `acl` into `info`, growing it (and topping up rent) when needed.
fn store_lineage<'info>(
    info: &AccountInfo<'info>,
    payer: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    acl: &EncryptedValueAcl,
) -> Result<()> {
    let data = encode_account(acl).map_err(map_acl_err)?;
    if info.data_len() != data.len() {
        let deficit = Rent::get()?
            .minimum_balance(data.len())
            .saturating_sub(info.lamports());
        if deficit > 0 {
            invoke(
                &system_instruction::transfer(payer.key, info.key, deficit),
                &[payer.clone(), info.clone(), system_program.clone()],
            )?;
        }
        info.resize(data.len())?;
    }
    info.try_borrow_mut_data()?.copy_from_slice(&data);
    Ok(())
}

// ---------------------------------------------------------------------------
// initialize_encrypted_value_acl
// ---------------------------------------------------------------------------

/// Accounts for creating an encrypted-value ACL lineage.
#[derive(Accounts)]
pub struct InitializeEncryptedValueAcl<'info> {
    /// Rent payer for the lineage account.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// App account that owns this lineage (keys the PDA).
    pub app_account_authority: Signer<'info>,
    /// CHECK: created and validated as the canonical lineage PDA in the handler.
    #[account(mut)]
    pub encrypted_value_acl: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

/// Creates a lineage at its first handle with an initial durable subject set.
pub fn initialize_encrypted_value_acl(
    ctx: Context<InitializeEncryptedValueAcl>,
    value_key: [u8; 32],
    acl_domain_key: Pubkey,
    encrypted_value_label: [u8; 32],
    handle: [u8; 32],
    subjects: Vec<Pubkey>,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    let app_account = ctx.accounts.app_account_authority.key();
    require!(
        value_key
            == acl_nonce_key(
                acl_domain_key.to_bytes(),
                app_account.to_bytes(),
                encrypted_value_label
            ),
        ZamaHostError::EncryptedValueAclMismatch
    );
    let (expected, bump) = encrypted_value_acl_address(value_key);
    let info = ctx.accounts.encrypted_value_acl.to_account_info();
    require_keys_eq!(
        info.key(),
        expected,
        ZamaHostError::EncryptedValueAclPdaMismatch
    );

    let acl = EncryptedValueAcl {
        acl_domain_key: acl_domain_key.to_bytes(),
        app_account: app_account.to_bytes(),
        encrypted_value_label,
        current_handle: handle,
        subjects: checked_subjects(&subjects)?,
        leaf_count: 0,
        peaks: Vec::new(),
        bump,
    };
    create_pda_strict(
        &ctx.accounts.payer.to_account_info(),
        &info,
        &ctx.accounts.system_program.to_account_info(),
        EncryptedValueAcl::account_size(acl.subjects.len(), 0),
        &[ENCRYPTED_VALUE_ACL_SEED, value_key.as_ref(), &[bump]],
    )?;
    store_lineage(
        &info,
        &ctx.accounts.payer.to_account_info(),
        &ctx.accounts.system_program.to_account_info(),
        &acl,
    )
}

// ---------------------------------------------------------------------------
// Mutating lifecycle over an existing lineage
// ---------------------------------------------------------------------------

/// Accounts for any instruction that mutates an existing lineage. The signer
/// must be the lineage's `app_account`; `payer` funds any `realloc` growth.
#[derive(Accounts)]
pub struct UpdateEncryptedValueAcl<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub app_account_authority: Signer<'info>,
    /// CHECK: validated as the canonical lineage PDA in the handler.
    #[account(mut)]
    pub encrypted_value_acl: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> UpdateEncryptedValueAcl<'info> {
    /// Loads the lineage and checks the signer owns it.
    fn authorized_lineage(&self) -> Result<EncryptedValueAcl> {
        let acl = load_lineage(&self.encrypted_value_acl.to_account_info())?;
        require!(
            acl.app_account == self.app_account_authority.key().to_bytes(),
            ZamaHostError::EncryptedValueAclMismatch
        );
        Ok(acl)
    }

    fn store(&self, acl: &EncryptedValueAcl) -> Result<()> {
        store_lineage(
            &self.encrypted_value_acl.to_account_info(),
            &self.payer.to_account_info(),
            &self.system_program.to_account_info(),
            acl,
        )
    }
}

/// Rotates the lineage: record a historical-access leaf per current subject for
/// the old handle, then set the new handle and subjects.
pub fn rotate_encrypted_value(
    ctx: Context<UpdateEncryptedValueAcl>,
    new_handle: [u8; 32],
    new_subjects: Vec<Pubkey>,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    let acl_key = ctx.accounts.encrypted_value_acl.key().to_bytes();
    let mut acl = ctx.accounts.authorized_lineage()?;
    let old_handle = acl.current_handle;
    for index in 0..acl.subjects.len() {
        let leaf_index = acl.leaf_count;
        let commitment =
            historical_access_leaf_commitment(acl_key, leaf_index, old_handle, acl.subjects[index]);
        mmr_append(&mut acl.peaks, &mut acl.leaf_count, commitment).map_err(map_acl_err)?;
    }
    acl.subjects = checked_subjects(&new_subjects)?;
    acl.current_handle = new_handle;
    ctx.accounts.store(&acl)
}

/// Extends current durable membership with additional distinct subjects.
pub fn allow_encrypted_value_subjects(
    ctx: Context<UpdateEncryptedValueAcl>,
    subjects: Vec<Pubkey>,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    require!(
        subjects.len() <= MAX_ENCRYPTED_VALUE_SUBJECTS,
        ZamaHostError::EncryptedValueSubjectCapacityExceeded
    );
    let mut acl = ctx.accounts.authorized_lineage()?;
    for subject in &subjects {
        require!(
            *subject != Pubkey::default(),
            ZamaHostError::EncryptedValueSubjectMissing
        );
        let bytes = subject.to_bytes();
        if acl.subjects.contains(&bytes) {
            continue;
        }
        require!(
            acl.subjects.len() < MAX_ENCRYPTED_VALUE_SUBJECTS,
            ZamaHostError::EncryptedValueSubjectCapacityExceeded
        );
        acl.subjects.push(bytes);
    }
    ctx.accounts.store(&acl)
}

/// Records an exact public-decrypt leaf for the lineage's current handle.
pub fn mark_encrypted_value_public(ctx: Context<UpdateEncryptedValueAcl>) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    let acl_key = ctx.accounts.encrypted_value_acl.key().to_bytes();
    let mut acl = ctx.accounts.authorized_lineage()?;
    let leaf_index = acl.leaf_count;
    let commitment = public_decrypt_leaf_commitment(acl_key, leaf_index, acl.current_handle);
    mmr_append(&mut acl.peaks, &mut acl.leaf_count, commitment).map_err(map_acl_err)?;
    ctx.accounts.store(&acl)
}
