//! Shared account contexts and validation helpers for instruction modules.

use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction, system_program};

// AclRecordBoundEvent / AclSubjectAllowedEvent are only used by the emit funnels,
// which are no-ops when `emit-events` is off.
#[cfg(feature = "emit-events")]
use crate::events::{AclRecordBoundEvent, AclSubjectAllowedEvent};
use crate::{
    errors::ZamaHostError,
    events::HostConfigUpdatedEvent,
    state::{
        acl_nonce_key, acl_permission_address, acl_record_address,
        acl_record_subject_slots_are_canonical, assert_handle_for_chain, deny_subject_address,
        host_config_address, role_flags_are_known, subject_has_role, AclPermission, AclRecord,
        AclSubjectEntry, DenySubjectRecord, HostConfig, ACL_PERMISSION_SEED,
        ACL_ROLE_PUBLIC_DECRYPT, ACL_ROLE_USE, EVENT_VERSION, MAX_ACL_SUBJECTS,
        MAX_ACL_SUBJECT_GRANTS_PER_CALL,
    },
};

pub(super) fn assert_no_remaining_accounts(remaining_accounts: &[AccountInfo]) -> Result<()> {
    require!(
        remaining_accounts.is_empty(),
        ZamaHostError::UnexpectedRemainingAccounts
    );
    Ok(())
}

pub(super) fn assert_host_config_shape(config: &Account<HostConfig>) -> Result<()> {
    let (expected_key, expected_bump) = host_config_address();
    require_keys_eq!(
        config.key(),
        expected_key,
        ZamaHostError::HostConfigMismatch
    );
    require!(
        config.to_account_info().data_len() == 8 + HostConfig::SPACE,
        ZamaHostError::HostConfigMismatch
    );
    require!(
        config.bump == expected_bump,
        ZamaHostError::HostConfigMismatch
    );
    Ok(())
}

pub(super) fn assert_admin(config: &Account<HostConfig>, admin: Pubkey) -> Result<()> {
    assert_host_config_shape(config)?;
    require_keys_eq!(config.admin, admin, ZamaHostError::HostConfigAdminMismatch);
    Ok(())
}

pub(super) fn assert_not_paused(config: &Account<HostConfig>) -> Result<()> {
    assert_host_config_shape(config)?;
    require!(!config.paused, ZamaHostError::HostConfigPaused);
    Ok(())
}

#[cfg(feature = "poc")]
pub(super) fn assert_test_shim_authority(
    config: &Account<HostConfig>,
    authority: Pubkey,
) -> Result<()> {
    assert_not_paused(config)?;
    // Confine the `test_emit_*` event shims to the local PoC chain, matching the
    // confinement already applied to the zero birth-entropy fallback
    // (state.rs `zero_birth_entropy_allowed`). This prevents an admin on a
    // deployed chain from emitting forged protocol events that a downstream
    // host-listener/indexer could ingest as genuine.
    require!(
        config.test_shims_enabled && config.is_local_poc_chain(),
        ZamaHostError::TestShimsDisabled
    );
    require_keys_eq!(
        config.test_authority,
        authority,
        ZamaHostError::TestShimAuthorityMismatch
    );
    Ok(())
}

pub(super) fn emit_config_updated(config: &HostConfig, admin: Pubkey) {
    #[cfg(feature = "emit-events")]
    emit!(HostConfigUpdatedEvent {
        version: EVENT_VERSION,
        config: crate::state::host_config_address().0,
        admin,
        paused: config.paused,
        mock_input_enabled: config.mock_input_enabled,
        test_shims_enabled: config.test_shims_enabled,
        grant_deny_list_enabled: config.grant_deny_list_enabled,
        max_hcu_per_tx: config.max_hcu_per_tx,
        max_hcu_depth_per_tx: config.max_hcu_depth_per_tx,
        updated_slot: config.updated_slot,
    });
}

/// Enforces the HCU limit ordering invariant `max_hcu_per_tx >= max_hcu_depth_per_tx`, treating `0`
/// as unlimited on either side. Both setters reuse this in `(total, depth)` terms:
/// `set_max_hcu_per_tx(v)` calls `check_hcu_ordering(v, cfg.max_hcu_depth_per_tx)`;
/// `set_max_hcu_depth_per_tx(v)` calls `check_hcu_ordering(cfg.max_hcu_per_tx, v)`.
pub(super) fn check_hcu_ordering(total: u64, depth: u64) -> Result<()> {
    require!(
        total == 0 || depth == 0 || total >= depth,
        ZamaHostError::HcuLimitOrderingInvalid
    );
    Ok(())
}

pub(super) fn write_acl_record(
    record: &mut Account<AclRecord>,
    nonce_key: [u8; 32],
    nonce_sequence: u64,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    handle: [u8; 32],
    subjects: &[AclSubjectEntry],
    public_decrypt: bool,
    created_slot: u64,
    bump: u8,
) {
    record.handle = handle;
    record.nonce_key = nonce_key;
    record.nonce_sequence = nonce_sequence;
    record.acl_domain_key = acl_domain_key;
    record.app_account = app_account;
    record.encrypted_value_label = encrypted_value_label;
    record.subjects = [Pubkey::default(); MAX_ACL_SUBJECTS];
    record.subject_roles = [0; MAX_ACL_SUBJECTS];
    record.subject_count = subjects.len() as u8;
    record.overflow_subject_count = 0;
    record.public_decrypt = public_decrypt;
    record.material_commitment = Pubkey::default();
    record.material_commitment_hash = [0; 32];
    record.material_key_id = [0; 32];
    record.created_slot = created_slot;
    record.bump = bump;

    for (index, subject) in subjects.iter().enumerate() {
        record.subjects[index] = subject.pubkey;
        record.subject_roles[index] = subject.role_flags;
    }
}

pub(super) struct AclSubjectUpdate {
    pub subject: AclSubjectEntry,
    pub permission_record: Pubkey,
    pub inline_index: u8,
}

pub(super) fn extend_acl_subjects<'info>(
    payer: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    record_key: Pubkey,
    record: &mut Account<AclRecord>,
    subjects: &[AclSubjectEntry],
    permission_accounts: &[AccountInfo<'info>],
) -> Result<Vec<AclSubjectUpdate>> {
    require!(
        !subjects.is_empty() && subjects.len() <= MAX_ACL_SUBJECT_GRANTS_PER_CALL,
        ZamaHostError::AclSubjectCapacityExceeded
    );

    let mut overflow_index = 0usize;
    let mut subject_count = record.subject_count as usize;
    let mut emitted = Vec::new();
    for subject in subjects {
        require!(
            subject.pubkey != Pubkey::default() && role_flags_are_known(subject.role_flags),
            ZamaHostError::AclSubjectRoleMismatch
        );
        if let Some(index) = record.inline_subject_index(subject.pubkey) {
            let updated_roles = record.subject_roles[index] | subject.role_flags;
            if updated_roles != record.subject_roles[index] {
                record.subject_roles[index] = updated_roles;
                emitted.push(AclSubjectUpdate {
                    subject: *subject,
                    permission_record: Pubkey::default(),
                    inline_index: index as u8,
                });
            }
            continue;
        }
        if subject_count < MAX_ACL_SUBJECTS {
            let inline_index = subject_count as u8;
            record.subjects[subject_count] = subject.pubkey;
            record.subject_roles[subject_count] = subject.role_flags;
            subject_count += 1;
            emitted.push(AclSubjectUpdate {
                subject: *subject,
                permission_record: Pubkey::default(),
                inline_index,
            });
            continue;
        }

        let permission = permission_accounts
            .get(overflow_index)
            .ok_or(ZamaHostError::AclPermissionMissing)?;
        overflow_index += 1;
        let update = create_or_update_permission(
            payer,
            system_program,
            permission,
            record_key,
            subject.pubkey,
            subject.role_flags,
        )?;
        if update.created {
            record.overflow_subject_count = record.overflow_subject_count.saturating_add(1);
        }
        if update.changed {
            emitted.push(AclSubjectUpdate {
                subject: *subject,
                permission_record: permission.key(),
                inline_index: u8::MAX,
            });
        }
    }
    record.subject_count = subject_count as u8;
    require!(
        overflow_index == permission_accounts.len(),
        ZamaHostError::AclPermissionMismatch
    );
    Ok(emitted)
}

pub(super) fn assert_output_acl_metadata(
    app_account_authority: Pubkey,
    nonce_key: [u8; 32],
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    subjects: &[AclSubjectEntry],
) -> Result<()> {
    require_keys_eq!(
        app_account_authority,
        app_account,
        ZamaHostError::AppAccountAuthorityMismatch
    );
    assert_nonce_key_matches_fields(
        nonce_key,
        acl_domain_key,
        app_account,
        encrypted_value_label,
    )?;
    require!(
        !subjects.is_empty() && subjects.len() <= MAX_ACL_SUBJECTS,
        ZamaHostError::AclSubjectCapacityExceeded
    );
    require!(
        subjects
            .iter()
            .all(|subject| subject.pubkey != Pubkey::default()
                && role_flags_are_known(subject.role_flags)
                && subject_has_role(subject.role_flags, ACL_ROLE_USE)),
        ZamaHostError::AclSubjectRoleMismatch
    );
    for (index, subject) in subjects.iter().enumerate() {
        require!(
            !subjects
                .iter()
                .skip(index + 1)
                .any(|later| later.pubkey == subject.pubkey),
            ZamaHostError::AclSubjectRoleMismatch
        );
    }
    Ok(())
}

pub(super) fn assert_public_decrypt_not_set_at_birth(output_public_decrypt: bool) -> Result<()> {
    require!(
        !output_public_decrypt,
        ZamaHostError::PublicDecryptAtBirthUnsupported
    );
    Ok(())
}

pub(super) fn assert_derived_public_decrypt_roles_allowed(
    subjects: &[AclSubjectEntry],
    propagated_public_decrypt_allowed: bool,
    app_account_authority: &AccountInfo,
) -> Result<()> {
    let grants_public_decrypt = subjects
        .iter()
        .any(|subject| subject_has_role(subject.role_flags, ACL_ROLE_PUBLIC_DECRYPT));
    if !grants_public_decrypt || propagated_public_decrypt_allowed {
        return Ok(());
    }
    require!(
        *app_account_authority.owner != system_program::ID
            && !app_account_authority.executable
            && !app_account_authority.data_is_empty(),
        ZamaHostError::DerivedOutputPublicDecryptDenied
    );
    Ok(())
}

pub(super) fn assert_record(
    record_info: &AccountInfo,
    record: &Account<AclRecord>,
    nonce_key: [u8; 32],
    nonce_sequence: u64,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    handle: [u8; 32],
    subject: Pubkey,
    permission_info: Option<&AccountInfo>,
) -> Result<()> {
    assert_nonce_key_matches_fields(
        nonce_key,
        acl_domain_key,
        app_account,
        encrypted_value_label,
    )?;
    assert_canonical_acl_record(record_info, record)?;
    require!(
        record.nonce_key == nonce_key,
        ZamaHostError::AclNonceKeyMismatch
    );
    require!(
        record.nonce_sequence == nonce_sequence,
        ZamaHostError::AclNonceSequenceMismatch
    );
    require_keys_eq!(
        record.acl_domain_key,
        acl_domain_key,
        ZamaHostError::AclDomainKeyMismatch
    );
    require_keys_eq!(
        record.app_account,
        app_account,
        ZamaHostError::AclAppAccountMismatch
    );
    require!(
        record.encrypted_value_label == encrypted_value_label,
        ZamaHostError::AclEncryptedValueLabelMismatch
    );
    assert_record_subject_role(
        record,
        record_info.key(),
        handle,
        subject,
        ACL_ROLE_USE,
        permission_info,
    )
}

pub(super) fn assert_canonical_acl_record(
    record_info: &AccountInfo,
    record: &Account<AclRecord>,
) -> Result<()> {
    require!(
        record_info.data_len() == 8 + AclRecord::SPACE,
        ZamaHostError::AclRecordPdaMismatch
    );
    assert_canonical_acl_record_data(record_info.key(), record)
}

pub(super) fn assert_acl_record_handle_for_chain(record: &AclRecord, chain_id: u64) -> Result<()> {
    assert_handle_for_chain(record.handle, chain_id)
}

fn assert_canonical_acl_record_data(record_key: Pubkey, record: &AclRecord) -> Result<()> {
    assert_nonce_key_matches_fields(
        record.nonce_key,
        record.acl_domain_key,
        record.app_account,
        record.encrypted_value_label,
    )?;

    let (expected, expected_bump) = acl_record_address(record.nonce_key, record.nonce_sequence);
    require_keys_eq!(record_key, expected, ZamaHostError::AclRecordPdaMismatch);
    require!(
        record.bump == expected_bump,
        ZamaHostError::AclRecordPdaMismatch
    );
    require!(
        acl_record_subject_slots_are_canonical(record),
        ZamaHostError::AclSubjectRoleMismatch
    );
    Ok(())
}

pub(super) fn assert_unchecked_acl_record_subject_role(
    record_info: &AccountInfo,
    handle: [u8; 32],
    chain_id: u64,
    subject: Pubkey,
    role: u8,
    permission_info: Option<&AccountInfo>,
) -> Result<()> {
    require_keys_eq!(
        *record_info.owner,
        crate::ID,
        ZamaHostError::AclRecordPdaMismatch
    );
    let record = read_acl_record(record_info)?;
    assert_canonical_acl_record_data(record_info.key(), &record)?;
    assert_acl_record_handle_for_chain(&record, chain_id)?;
    assert_record_subject_role(
        &record,
        record_info.key(),
        handle,
        subject,
        role,
        permission_info,
    )
}

pub(super) fn unchecked_acl_record_subject_has_role(
    record_info: &AccountInfo,
    handle: [u8; 32],
    subject: Pubkey,
    role: u8,
    permission_info: Option<&AccountInfo>,
) -> Result<bool> {
    require_keys_eq!(
        *record_info.owner,
        crate::ID,
        ZamaHostError::AclRecordPdaMismatch
    );
    let record = read_acl_record(record_info)?;
    assert_canonical_acl_record_data(record_info.key(), &record)?;
    require!(record.handle == handle, ZamaHostError::AclHandleMismatch);
    if let Some(index) = record.inline_subject_index(subject) {
        require!(
            permission_info.is_none(),
            ZamaHostError::AclPermissionMismatch
        );
        return Ok(subject_has_role(record.subject_roles[index], role));
    }
    let Some(permission_info) = permission_info else {
        return Ok(false);
    };
    let permission = read_permission(permission_info)?;
    let (expected, expected_bump) = acl_permission_address(record_info.key(), subject);
    require_keys_eq!(
        permission_info.key(),
        expected,
        ZamaHostError::AclPermissionPdaMismatch
    );
    require!(
        permission.bump == expected_bump,
        ZamaHostError::AclPermissionPdaMismatch
    );
    require_keys_eq!(
        permission.acl_record,
        record_info.key(),
        ZamaHostError::AclPermissionMismatch
    );
    require_keys_eq!(
        permission.subject,
        subject,
        ZamaHostError::AclPermissionMismatch
    );
    Ok(subject_has_role(permission.role_flags, role))
}

pub(super) fn read_acl_record(record_info: &AccountInfo) -> Result<AclRecord> {
    require!(
        record_info.data_len() == 8 + AclRecord::SPACE,
        ZamaHostError::AclRecordPdaMismatch
    );
    let data = record_info.try_borrow_data()?;
    let mut data_slice: &[u8] = &data;
    AclRecord::try_deserialize(&mut data_slice)
}

fn assert_nonce_key_matches_fields(
    nonce_key: [u8; 32],
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
) -> Result<()> {
    require!(
        nonce_key == acl_nonce_key(acl_domain_key, app_account, encrypted_value_label),
        ZamaHostError::AclNonceKeyMismatch
    );
    Ok(())
}

pub(super) fn assert_record_subject_role(
    record: &AclRecord,
    record_key: Pubkey,
    handle: [u8; 32],
    subject: Pubkey,
    role: u8,
    permission_info: Option<&AccountInfo>,
) -> Result<()> {
    require!(record.handle == handle, ZamaHostError::AclHandleMismatch);
    if let Some(index) = record.inline_subject_index(subject) {
        require!(
            permission_info.is_none(),
            ZamaHostError::AclPermissionMismatch
        );
        if subject_has_role(record.subject_roles[index], role) {
            return Ok(());
        }
        return err!(ZamaHostError::AclSubjectRoleMismatch);
    }
    let permission_info = permission_info.ok_or(ZamaHostError::AclPermissionMissing)?;
    let permission = read_permission(permission_info)?;
    let (expected, expected_bump) = acl_permission_address(record_key, subject);
    require_keys_eq!(
        permission_info.key(),
        expected,
        ZamaHostError::AclPermissionPdaMismatch
    );
    require!(
        permission.bump == expected_bump,
        ZamaHostError::AclPermissionPdaMismatch
    );
    require_keys_eq!(
        permission.acl_record,
        record_key,
        ZamaHostError::AclPermissionMismatch
    );
    require_keys_eq!(
        permission.subject,
        subject,
        ZamaHostError::AclPermissionMismatch
    );
    require!(
        subject_has_role(permission.role_flags, role),
        ZamaHostError::AclSubjectRoleMismatch
    );
    Ok(())
}

pub(super) fn check_grant_not_denied(
    config: &HostConfig,
    subject: Pubkey,
    deny_record: Option<&UncheckedAccount>,
) -> Result<()> {
    if !config.grant_deny_list_enabled {
        require!(deny_record.is_none(), ZamaHostError::AclDenyRecordMismatch);
        return Ok(());
    }
    let deny_record = deny_record.ok_or(ZamaHostError::AclDenyRecordMissing)?;
    let info = deny_record.to_account_info();
    let (expected, expected_bump) = deny_subject_address(subject);
    require_keys_eq!(info.key(), expected, ZamaHostError::AclDenyRecordMismatch);

    if is_absent_deny_record(&info)? {
        return Ok(());
    }
    require_keys_eq!(*info.owner, crate::ID, ZamaHostError::AclDenyRecordMismatch);
    require!(
        info.data_len() == 8 + DenySubjectRecord::SPACE,
        ZamaHostError::AclDenyRecordMismatch
    );
    let mut data: &[u8] = &info.try_borrow_data()?;
    let record = DenySubjectRecord::try_deserialize(&mut data)?;
    require!(
        record.bump == expected_bump,
        ZamaHostError::AclDenyRecordMismatch
    );
    require_keys_eq!(
        record.subject,
        subject,
        ZamaHostError::AclDenyRecordMismatch
    );
    require!(!record.denied, ZamaHostError::AclSubjectDenied);
    Ok(())
}

pub(super) fn is_absent_deny_record(info: &AccountInfo) -> Result<bool> {
    if info.owner == &System::id() && info.data_is_empty() {
        require!(!info.executable, ZamaHostError::AclDenyRecordMismatch);
        return Ok(true);
    }
    Ok(false)
}

struct PermissionUpdate {
    created: bool,
    changed: bool,
}

fn create_or_update_permission<'info>(
    payer: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    permission_info: &AccountInfo<'info>,
    record_key: Pubkey,
    subject: Pubkey,
    role_flags: u8,
) -> Result<PermissionUpdate> {
    let (expected, bump) = acl_permission_address(record_key, subject);
    require_keys_eq!(
        permission_info.key(),
        expected,
        ZamaHostError::AclPermissionPdaMismatch
    );
    let created = permission_info.owner != &crate::ID;
    create_pda_if_needed(
        payer,
        permission_info,
        system_program,
        8 + AclPermission::SPACE,
        &[
            ACL_PERMISSION_SEED,
            record_key.as_ref(),
            subject.as_ref(),
            &[bump],
        ],
    )?;

    let mut permission = if created {
        AclPermission {
            acl_record: record_key,
            subject,
            role_flags: 0,
            bump,
        }
    } else {
        read_permission(permission_info)?
    };
    require_keys_eq!(
        permission.acl_record,
        record_key,
        ZamaHostError::AclPermissionMismatch
    );
    require_keys_eq!(
        permission.subject,
        subject,
        ZamaHostError::AclPermissionMismatch
    );
    require!(
        permission.bump == bump,
        ZamaHostError::AclPermissionPdaMismatch
    );
    let updated_roles = permission.role_flags | role_flags;
    let changed = created || updated_roles != permission.role_flags;
    permission.role_flags = updated_roles;
    write_account(permission_info, &permission)?;
    Ok(PermissionUpdate { created, changed })
}

fn read_permission(permission_info: &AccountInfo) -> Result<AclPermission> {
    require_keys_eq!(
        *permission_info.owner,
        crate::ID,
        ZamaHostError::AclPermissionMismatch
    );
    require!(
        permission_info.data_len() == 8 + AclPermission::SPACE,
        ZamaHostError::AclPermissionMismatch
    );
    let data = permission_info.try_borrow_data()?;
    let mut data_slice: &[u8] = &data;
    AclPermission::try_deserialize(&mut data_slice)
}

pub(super) fn create_pda_if_needed<'info>(
    payer: &AccountInfo<'info>,
    account: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    space: usize,
    seeds: &[&[u8]],
) -> Result<()> {
    if account.owner == &crate::ID {
        return Ok(());
    }
    require_keys_eq!(
        *account.owner,
        System::id(),
        ZamaHostError::PdaCreationMismatch
    );
    require!(account.data_is_empty(), ZamaHostError::PdaCreationMismatch);
    require!(!account.executable, ZamaHostError::PdaCreationMismatch);
    let rent = Rent::get()?.minimum_balance(space);
    invoke_signed(
        &system_instruction::create_account(payer.key, account.key, rent, space as u64, &crate::ID),
        &[payer.clone(), account.clone(), system_program.clone()],
        &[seeds],
    )?;
    require_keys_eq!(
        *account.owner,
        crate::ID,
        ZamaHostError::PdaCreationMismatch
    );
    require!(!account.executable, ZamaHostError::PdaCreationMismatch);
    require!(
        account.data_len() == space,
        ZamaHostError::PdaCreationMismatch
    );
    require!(
        account.lamports() >= rent,
        ZamaHostError::PdaCreationMismatch
    );
    Ok(())
}

pub(super) fn create_pda_strict<'info>(
    payer: &AccountInfo<'info>,
    account: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    space: usize,
    seeds: &[&[u8]],
) -> Result<()> {
    require!(account.is_writable, ZamaHostError::InvalidFheEvalAccount);
    require_keys_eq!(
        *account.owner,
        System::id(),
        ZamaHostError::FheEvalOutputAlreadyInitialized
    );
    require!(
        account.data_is_empty(),
        ZamaHostError::FheEvalOutputAlreadyInitialized
    );
    require!(
        !account.executable,
        ZamaHostError::FheEvalOutputAlreadyInitialized
    );
    let rent = Rent::get()?.minimum_balance(space);
    invoke_signed(
        &system_instruction::create_account(payer.key, account.key, rent, space as u64, &crate::ID),
        &[payer.clone(), account.clone(), system_program.clone()],
        &[seeds],
    )?;
    require_keys_eq!(
        *account.owner,
        crate::ID,
        ZamaHostError::FheEvalOutputAlreadyInitialized
    );
    require!(
        !account.executable,
        ZamaHostError::FheEvalOutputAlreadyInitialized
    );
    require!(
        account.data_len() == space,
        ZamaHostError::FheEvalOutputAlreadyInitialized
    );
    require!(
        account.lamports() >= rent,
        ZamaHostError::FheEvalOutputAlreadyInitialized
    );
    Ok(())
}

pub(super) fn write_account<T: AccountSerialize>(info: &AccountInfo, account: &T) -> Result<()> {
    let mut data = info.try_borrow_mut_data()?;
    let mut cursor = &mut data[..];
    account.try_serialize(&mut cursor)?;
    Ok(())
}

/// With `emit-events` disabled, off-chain reconstruction is the sole event source,
/// so the ACL-record-bound emit is a no-op.
#[cfg(not(feature = "emit-events"))]
pub(super) fn emit_record_bound(_record_key: Pubkey, _record: &AclRecord) {}

#[cfg(feature = "emit-events")]
pub(super) fn emit_record_bound(record_key: Pubkey, record: &AclRecord) {
    emit!(AclRecordBoundEvent {
        version: EVENT_VERSION,
        acl_record: record_key,
        handle: record.handle,
        nonce_key: record.nonce_key,
        nonce_sequence: record.nonce_sequence,
        acl_domain_key: record.acl_domain_key,
        app_account: record.app_account,
        encrypted_value_label: record.encrypted_value_label,
        subject_count: record.subject_count,
        public_decrypt: record.public_decrypt,
        created_slot: record.created_slot,
    });
}

/// With `emit-events` disabled, off-chain reconstruction is the sole event source,
/// so the ACL-subject-allowed emit is a no-op.
#[cfg(not(feature = "emit-events"))]
pub(super) fn emit_subject_event(
    _record_key: Pubkey,
    _handle: [u8; 32],
    _subject: AclSubjectEntry,
    _overflow_permission_record: Pubkey,
) {
}

#[cfg(feature = "emit-events")]
pub(super) fn emit_subject_event(
    record_key: Pubkey,
    handle: [u8; 32],
    subject: AclSubjectEntry,
    overflow_permission_record: Pubkey,
) {
    let updated_slot = Clock::get().map_or(0, |clock| clock.slot);
    emit!(AclSubjectAllowedEvent {
        version: EVENT_VERSION,
        acl_record: record_key,
        handle,
        authority_subject: Pubkey::default(),
        subject: subject.pubkey.to_bytes(),
        role_flags: subject.role_flags,
        overflow_permission_record,
        inline_index: u8::MAX,
        updated_slot,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absent_deny_record_accepts_non_executable_system_empty_account() {
        let key = Pubkey::new_unique();
        let owner = System::id();
        let mut lamports = 0;
        let mut data = Vec::new();
        let info = AccountInfo::new(&key, false, false, &mut lamports, &mut data, &owner, false);

        assert!(is_absent_deny_record(&info).unwrap());
    }

    #[test]
    fn absent_deny_record_rejects_executable_system_empty_account() {
        let key = Pubkey::new_unique();
        let owner = System::id();
        let mut lamports = 0;
        let mut data = Vec::new();
        let info = AccountInfo::new(&key, false, false, &mut lamports, &mut data, &owner, true);

        assert_eq!(
            is_absent_deny_record(&info).unwrap_err(),
            error!(ZamaHostError::AclDenyRecordMismatch)
        );
    }

    #[test]
    fn absent_deny_record_ignores_non_system_empty_account() {
        let key = Pubkey::new_unique();
        let owner = crate::ID;
        let mut lamports = 0;
        let mut data = Vec::new();
        let info = AccountInfo::new(&key, false, false, &mut lamports, &mut data, &owner, false);

        assert!(!is_absent_deny_record(&info).unwrap());
    }

    // ---- INV-6 / INV-7: ordering invariant, expressed in (total, depth) terms ----

    #[test]
    fn check_hcu_ordering_accepts_total_ge_depth() {
        assert!(check_hcu_ordering(20_000_000, 5_000_000).is_ok()); // total > depth
        assert!(check_hcu_ordering(5_000_000, 5_000_000).is_ok()); // total == depth (boundary)
    }

    #[test]
    fn check_hcu_ordering_rejects_total_lt_depth() {
        // INV-6: depth=5M, set total=4M -> reject. INV-7: total=20M, set depth=21M -> reject.
        assert_eq!(
            check_hcu_ordering(4_000_000, 5_000_000).unwrap_err(),
            error!(ZamaHostError::HcuLimitOrderingInvalid)
        );
        assert_eq!(
            check_hcu_ordering(20_000_000, 21_000_000).unwrap_err(),
            error!(ZamaHostError::HcuLimitOrderingInvalid)
        );
    }

    #[test]
    fn check_hcu_ordering_zero_is_unlimited() {
        // 0 = +inf on either side (INV-13 sentinel); both 0 is the deploy default (INV-14).
        assert!(check_hcu_ordering(0, 5_000_000).is_ok());
        assert!(check_hcu_ordering(4_000_000, 0).is_ok());
        assert!(check_hcu_ordering(0, 0).is_ok());
    }

    #[test]
    fn hcu_ordering_unreachable_under_setter_sequences() {
        // INV-15: no ordered setter sequence can reach 0 < total < depth. Simulate both setters as
        // guarded mutations over a small value space; the bad state must never be reachable.
        let values = [0u64, 1, 5, 10, 20];
        for &a in &values {
            for &b in &values {
                for &c in &values {
                    let (mut total, mut depth) = (0u64, 0u64); // init state (INV-14)
                    // sequence: set_total(a), set_depth(b), set_total(c)
                    if check_hcu_ordering(a, depth).is_ok() {
                        total = a;
                    }
                    if check_hcu_ordering(total, b).is_ok() {
                        depth = b;
                    }
                    if check_hcu_ordering(c, depth).is_ok() {
                        total = c;
                    }
                    let bad = total != 0 && depth != 0 && total < depth;
                    assert!(!bad, "reached 0<total<depth: total={total} depth={depth}");
                }
            }
        }
    }

    // ---- INV-17: the config-updated event carries the HCU limits (compile-time proof) ----

    #[test]
    fn host_config_updated_event_carries_hcu_limits() {
        // If the two fields were missing, this would not build. (The updated_slot write + emit! are
        // exercised end-to-end by the Mollusk setter tests in runtime-tests/tests/host_mollusk.rs.)
        let _event = HostConfigUpdatedEvent {
            version: EVENT_VERSION,
            config: Pubkey::new_unique(),
            admin: Pubkey::new_unique(),
            paused: false,
            mock_input_enabled: false,
            test_shims_enabled: false,
            grant_deny_list_enabled: false,
            max_hcu_per_tx: 20_000_000,
            max_hcu_depth_per_tx: 5_000_000,
            updated_slot: 42,
        };
    }
}
