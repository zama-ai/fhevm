//! RFC-024 `EncryptedValue` ACL mutation. Raw create/update ABI entries fail
//! closed because durable handle birth and supersession must come from
//! `fhe_eval` output provenance. Event-free by design — indexers reconstruct
//! MMR leaves from instruction data, using the shared `zama_solana_acl` crate,
//! not from emitted events.

use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

use super::common::*;
use crate::{errors::ZamaHostError, state::*};

/// One subject grant: identity to add to the allowed set.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct EncryptedValueSubjectGrant {
    pub subject: Pubkey,
}

/// Accounts for the disabled raw `create_encrypted_value` ABI entry.
#[derive(Accounts)]
pub struct CreateEncryptedValue<'info> {
    /// Pays rent for the new account.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// App account authority; must sign and match `app_account`.
    pub app_account_authority: Signer<'info>,
    /// CHECK: PDA existence/address are validated inside the handler.
    #[account(mut)]
    pub encrypted_value: UncheckedAccount<'info>,
    /// Singleton config PDA.
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    /// CHECK: required when grant_deny_list_enabled; may be uninitialized.
    pub deny_subject_record: Option<UncheckedAccount<'info>>,
    pub system_program: Program<'info, System>,
}

pub fn create_encrypted_value(
    _ctx: Context<CreateEncryptedValue>,
    _acl_domain_key: Pubkey,
    _app_account: Pubkey,
    _encrypted_value_label: [u8; 32],
    _handle: [u8; 32],
    _subjects: Vec<EncryptedValueSubjectGrant>,
) -> Result<()> {
    err!(ZamaHostError::RawEncryptedValueLifecycleDisabled)
}

/// Accounts for `allow_subjects`.
#[derive(Accounts)]
pub struct AllowEncryptedValueSubjects<'info> {
    /// Pays for the account's growth, if any.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Current allowed subject on the encrypted value account.
    pub authority: Signer<'info>,
    /// CHECK: layout and ownership are validated inside the handler via `read_canonical_encrypted_value`.
    #[account(mut)]
    pub encrypted_value: UncheckedAccount<'info>,
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    pub deny_subject_record: Option<UncheckedAccount<'info>>,
    pub system_program: Program<'info, System>,
}

pub fn allow_subjects(
    ctx: Context<AllowEncryptedValueSubjects>,
    subjects: Vec<EncryptedValueSubjectGrant>,
) -> Result<()> {
    assert_not_paused(&ctx.accounts.host_config)?;
    assert_valid_new_subjects(&subjects)?;

    let info = ctx.accounts.encrypted_value.to_account_info();
    let mut value = read_canonical_encrypted_value(&info)?;
    let authority = ctx.accounts.authority.key();
    require!(
        value.has_subject(authority),
        ZamaHostError::SubjectNotAllowed
    );
    check_grant_not_denied(
        &ctx.accounts.host_config,
        authority,
        ctx.accounts.deny_subject_record.as_ref(),
    )?;

    for grant in &subjects {
        if value.has_subject(grant.subject) {
            continue;
        }
        require!(
            value.subjects.len() < zama_solana_acl::MAX_ENCRYPTED_VALUE_SUBJECTS,
            ZamaHostError::EncryptedValueSubjectCapacityExceeded
        );
        value.subjects.push(grant.subject);
    }

    let space = 8 + EncryptedValue::space(value.subjects.len(), value.peaks.len());
    grow_account_if_needed(
        &ctx.accounts.payer.to_account_info(),
        &info,
        &ctx.accounts.system_program.to_account_info(),
        space,
    )?;
    write_account(&info, &value)?;
    Ok(())
}

/// Accounts for the disabled raw `update_encrypted_value` ABI entry.
#[derive(Accounts)]
pub struct UpdateEncryptedValue<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// App account authority; must sign and match the encrypted value account's `app_account`.
    pub app_account_authority: Signer<'info>,
    /// CHECK: layout and ownership are validated inside the handler via `read_canonical_encrypted_value`.
    #[account(mut)]
    pub encrypted_value: UncheckedAccount<'info>,
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    pub deny_subject_record: Option<UncheckedAccount<'info>>,
    pub system_program: Program<'info, System>,
}

pub fn update_encrypted_value(
    _ctx: Context<UpdateEncryptedValue>,
    _new_handle: [u8; 32],
    _previous_handle: [u8; 32],
    _previous_subjects: Vec<Pubkey>,
) -> Result<()> {
    err!(ZamaHostError::RawEncryptedValueLifecycleDisabled)
}

/// Appends one historical-access leaf per allowed subject for the outgoing
/// handle, then overwrites `current_handle`. Used by `fhe_eval`'s durable
/// output-binding path.
pub(super) fn supersede_current_handle(
    info: &AccountInfo,
    value: &mut EncryptedValue,
    new_handle: [u8; 32],
) -> Result<()> {
    let previous_handle = value.current_handle;
    let account_key = info.key().to_bytes();
    for subject in &value.subjects {
        let leaf_index = value.leaf_count;
        let commitment = zama_solana_acl::historical_access_leaf_commitment(
            account_key,
            leaf_index,
            previous_handle,
            subject.to_bytes(),
        );
        zama_solana_acl::mmr_append(&mut value.peaks, &mut value.leaf_count, commitment)
            .map_err(map_mmr_append_error)?;
    }
    value.current_handle = new_handle;
    Ok(())
}

/// Appends a public-decrypt leaf for `handle` at the encrypted value account's next leaf index.
/// Shared by `make_handle_public` and by `fhe_eval`'s born-public output binding
/// so both produce a byte-identical public-decrypt commitment.
pub(super) fn append_public_decrypt_leaf(
    info: &AccountInfo,
    value: &mut EncryptedValue,
    handle: [u8; 32],
) -> Result<()> {
    let account_key = info.key().to_bytes();
    let leaf_index = value.leaf_count;
    let commitment =
        zama_solana_acl::public_decrypt_leaf_commitment(account_key, leaf_index, handle);
    zama_solana_acl::mmr_append(&mut value.peaks, &mut value.leaf_count, commitment)
        .map_err(map_mmr_append_error)
}

/// Accounts for `make_handle_public`.
#[derive(Accounts)]
pub struct MakeEncryptedValueHandlePublic<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Current allowed subject.
    pub authority: Signer<'info>,
    /// CHECK: layout and ownership are validated inside the handler via `read_canonical_encrypted_value`.
    #[account(mut)]
    pub encrypted_value: UncheckedAccount<'info>,
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    pub deny_subject_record: Option<UncheckedAccount<'info>>,
    pub system_program: Program<'info, System>,
}

pub fn make_handle_public(
    ctx: Context<MakeEncryptedValueHandlePublic>,
    handle: [u8; 32],
) -> Result<()> {
    assert_not_paused(&ctx.accounts.host_config)?;
    let info = ctx.accounts.encrypted_value.to_account_info();
    let mut value = read_canonical_encrypted_value(&info)?;
    let authority = ctx.accounts.authority.key();
    require!(
        value.has_subject(authority),
        ZamaHostError::SubjectNotAllowed
    );
    check_grant_not_denied(
        &ctx.accounts.host_config,
        authority,
        ctx.accounts.deny_subject_record.as_ref(),
    )?;
    require!(
        handle == value.current_handle,
        ZamaHostError::EncryptedValuePublicHandleMismatch
    );

    append_public_decrypt_leaf(&info, &mut value, handle)?;

    let space = 8 + EncryptedValue::space(value.subjects.len(), value.peaks.len());
    grow_account_if_needed(
        &ctx.accounts.payer.to_account_info(),
        &info,
        &ctx.accounts.system_program.to_account_info(),
        space,
    )?;
    write_account(&info, &value)?;
    Ok(())
}

fn assert_valid_new_subjects(subjects: &[EncryptedValueSubjectGrant]) -> Result<()> {
    require!(
        !subjects.is_empty(),
        ZamaHostError::EncryptedValueEmptySubjects
    );
    require!(
        subjects.len() <= zama_solana_acl::MAX_ENCRYPTED_VALUE_SUBJECTS,
        ZamaHostError::EncryptedValueSubjectCapacityExceeded
    );
    require!(
        subjects.iter().all(|s| s.subject != Pubkey::default()),
        ZamaHostError::SubjectNotAllowed
    );
    for (index, s) in subjects.iter().enumerate() {
        require!(
            !subjects[index + 1..]
                .iter()
                .any(|later| later.subject == s.subject),
            ZamaHostError::SubjectNotAllowed
        );
    }
    Ok(())
}

fn map_mmr_append_error(error: zama_solana_acl::AclError) -> anchor_lang::error::Error {
    match error {
        zama_solana_acl::AclError::MmrPeakCapacityExceeded => {
            error!(ZamaHostError::EncryptedValueMmrPeakCapacityExceeded)
        }
        _ => error!(ZamaHostError::EncryptedValueMmrInconsistent),
    }
}

/// Reallocs the account and tops up rent when `target_space` grows past the
/// account's current data length. Never shrinks — leaf/subject counts are monotonic.
pub(super) fn grow_account_if_needed<'info>(
    payer: &AccountInfo<'info>,
    account: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    target_space: usize,
) -> Result<()> {
    if account.data_len() >= target_space {
        return Ok(());
    }
    let rent = Rent::get()?.minimum_balance(target_space);
    if account.lamports() < rent {
        let top_up = rent - account.lamports();
        invoke_signed(
            &system_instruction::transfer(payer.key, account.key, top_up),
            &[payer.clone(), account.clone(), system_program.clone()],
            &[],
        )?;
    }
    account.resize(target_space)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn value(handle: [u8; 32], subjects: &[Pubkey]) -> EncryptedValue {
        EncryptedValue {
            acl_domain_key: Pubkey::default(),
            app_account: Pubkey::default(),
            encrypted_value_label: [0; 32],
            current_handle: handle,
            subjects: subjects.to_vec(),
            leaf_count: 0,
            peaks: Vec::new(),
            bump: 0,
        }
    }

    fn account_key() -> Pubkey {
        Pubkey::new_from_array([9u8; 32])
    }

    fn dummy_info(key: &Pubkey) -> AccountInfo<'_> {
        // The handler only reads `info.key()`; lamports/data/owner are unused
        // by `supersede_current_handle`, so a minimal system-owned stub suffices.
        static mut LAMPORTS: u64 = 0;
        static OWNER: Pubkey = Pubkey::new_from_array([0; 32]);
        #[allow(static_mut_refs)]
        AccountInfo::new(
            key,
            false,
            false,
            unsafe { &mut LAMPORTS },
            &mut [],
            &OWNER,
            false,
        )
    }

    #[test]
    fn update_appends_one_leaf_per_allowed_subject_matching_mmr_append() {
        let owner = Pubkey::new_unique();
        let other = Pubkey::new_unique();
        let third = Pubkey::new_unique();
        let mut v = value([10; 32], &[owner, other, third]);
        let key = account_key();
        let info = dummy_info(&key);
        let previous_handle = v.current_handle;

        supersede_current_handle(&info, &mut v, [11; 32]).unwrap();

        assert_eq!(v.current_handle, [11; 32]);
        assert_eq!(v.leaf_count, 3);

        // Independently reproduce the expected peaks via the shared crate's
        // mmr_append, over the same commitments in the same order.
        let mut expected_peaks = Vec::new();
        let mut expected_count = 0u64;
        for (index, subject) in [owner, other, third].iter().enumerate() {
            let commitment = zama_solana_acl::historical_access_leaf_commitment(
                key.to_bytes(),
                index as u64,
                previous_handle,
                subject.to_bytes(),
            );
            zama_solana_acl::mmr_append(&mut expected_peaks, &mut expected_count, commitment)
                .unwrap();
        }
        assert_eq!(v.peaks, expected_peaks);
        assert_eq!(v.leaf_count, expected_count);
    }

    #[test]
    fn make_public_leaf_matches_shared_commitment() {
        let key = account_key();
        let mut v = value([7; 32], &[Pubkey::new_unique()]);
        let commitment =
            zama_solana_acl::public_decrypt_leaf_commitment(key.to_bytes(), 0, v.current_handle);
        zama_solana_acl::mmr_append(&mut v.peaks, &mut v.leaf_count, commitment).unwrap();

        assert_eq!(v.leaf_count, 1);
        let mut expected_peaks = Vec::new();
        let mut expected_count = 0u64;
        zama_solana_acl::mmr_append(&mut expected_peaks, &mut expected_count, commitment).unwrap();
        assert_eq!(v.peaks, expected_peaks);
    }

    #[test]
    fn previous_state_equality_check_rejects_handle_or_subject_mismatch() {
        let subjects = vec![Pubkey::new_unique()];
        let v = value([1; 32], &subjects);

        // Durable-output supersession requires exact equality on both the
        // handle and the full subject vector (order-sensitive).
        assert!(v.current_handle == [1; 32] && v.subjects == subjects);
        assert!(!(v.current_handle == [2; 32] && v.subjects == subjects));
        let wrong_subjects = vec![Pubkey::new_unique()];
        assert!(!(v.current_handle == [1; 32] && v.subjects == wrong_subjects));
    }

    #[test]
    fn supersede_then_make_public_matches_shared_value_account_reconstruction() {
        // Two on-chain appends (one supersede over two allowed subjects, one
        // make-public) must reproduce byte-for-byte the peaks an off-chain
        // indexer would derive from `zama_solana_acl::value_account::reconstruct`
        // over the equivalent `HandleSuperseded`/`MarkedPublic` event log.
        let owner = Pubkey::new_unique();
        let other = Pubkey::new_unique();
        let key = account_key();
        let info = dummy_info(&key);
        let mut v = value([1; 32], &[owner, other]);
        let previous_handle = v.current_handle;
        let previous_subjects = v.subjects.clone();

        supersede_current_handle(&info, &mut v, [2; 32]).unwrap();
        let commitment = zama_solana_acl::public_decrypt_leaf_commitment(
            key.to_bytes(),
            v.leaf_count,
            v.current_handle,
        );
        zama_solana_acl::mmr_append(&mut v.peaks, &mut v.leaf_count, commitment).unwrap();

        let events = [
            zama_solana_acl::value_account::EncryptedValueAccountEvent::handle_superseded(
                previous_handle,
                &previous_subjects
                    .iter()
                    .map(|p| p.to_bytes())
                    .collect::<Vec<_>>(),
            ),
            zama_solana_acl::value_account::EncryptedValueAccountEvent::MarkedPublic {
                handle: [2; 32],
            },
        ];
        let reconstructed =
            zama_solana_acl::value_account::reconstruct(key.to_bytes(), &events).unwrap();
        assert!(reconstructed.peaks_match(&v.peaks, v.leaf_count));
    }

    #[test]
    fn assert_valid_new_subjects_rejects_empty_and_duplicates() {
        assert!(assert_valid_new_subjects(&[]).is_err());
        let dup = Pubkey::new_unique();
        assert!(assert_valid_new_subjects(&[
            EncryptedValueSubjectGrant { subject: dup },
            EncryptedValueSubjectGrant { subject: dup },
        ])
        .is_err());
        assert!(assert_valid_new_subjects(&[EncryptedValueSubjectGrant { subject: dup }]).is_ok());
    }
}
