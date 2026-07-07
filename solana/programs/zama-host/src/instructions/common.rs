//! Shared account contexts and validation helpers for instruction modules.

use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

use crate::{
    errors::ZamaHostError,
    events::HostConfigUpdatedEvent,
    state::{
        assert_handle_for_chain, deny_subject_address, encrypted_value_address,
        host_config_address, AclSubjectEntry, DenySubjectRecord, EncryptedValue, HostConfig,
        EVENT_VERSION, MAX_ACL_SUBJECTS,
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

pub(super) fn assert_output_acl_metadata(
    app_account_authority: Pubkey,
    app_account: Pubkey,
    subjects: &[AclSubjectEntry],
) -> Result<()> {
    require_keys_eq!(
        app_account_authority,
        app_account,
        ZamaHostError::AppAccountAuthorityMismatch
    );
    require!(
        !subjects.is_empty() && subjects.len() <= MAX_ACL_SUBJECTS,
        ZamaHostError::EncryptedValueSubjectCapacityExceeded
    );
    require!(
        subjects
            .iter()
            .all(|subject| subject.pubkey != Pubkey::default()),
        ZamaHostError::SubjectNotAllowed
    );
    for (index, subject) in subjects.iter().enumerate() {
        require!(
            !subjects
                .iter()
                .skip(index + 1)
                .any(|later| later.pubkey == subject.pubkey),
            ZamaHostError::SubjectNotAllowed
        );
    }
    Ok(())
}

/// Decodes an `EncryptedValue` and checks it is program-owned and the
/// canonical PDA for its stored `(domain, app_account, label)` triple.
pub(super) fn read_canonical_encrypted_value(info: &AccountInfo) -> Result<EncryptedValue> {
    require_keys_eq!(
        *info.owner,
        crate::ID,
        ZamaHostError::EncryptedValueAccountInvalid
    );
    let data = info.try_borrow_data()?;
    let mut slice: &[u8] = &data;
    let value = EncryptedValue::try_deserialize(&mut slice)?;
    let (expected, expected_bump) = encrypted_value_address(value.value_key());
    require_keys_eq!(
        info.key(),
        expected,
        ZamaHostError::EncryptedValuePdaMismatch
    );
    require!(
        value.bump == expected_bump,
        ZamaHostError::EncryptedValuePdaMismatch
    );
    Ok(value)
}

/// Durable input authorization: the account must be a canonical program-owned
/// `EncryptedValue`, `handle` must be its *current* handle (for this chain),
/// and `subject` must be a current allowed member.
pub(super) fn assert_encrypted_value_subject_allowed(
    info: &AccountInfo,
    handle: [u8; 32],
    chain_id: u64,
    subject: Pubkey,
) -> Result<()> {
    let value = read_canonical_encrypted_value(info)?;
    assert_handle_for_chain(value.current_handle, chain_id)?;
    require!(
        value.current_handle == handle,
        ZamaHostError::PreviousStateMismatch
    );
    require!(
        value.subject_index(subject).is_some(),
        ZamaHostError::SubjectNotFound
    );
    Ok(())
}

pub(super) fn check_grant_not_denied(
    config: &HostConfig,
    subject: Pubkey,
    deny_record: Option<&UncheckedAccount>,
) -> Result<()> {
    let info = deny_record.map(|account| account.to_account_info());
    check_grant_not_denied_info(config, subject, info.as_ref())
}

pub(super) fn check_grant_not_denied_info(
    config: &HostConfig,
    subject: Pubkey,
    deny_record: Option<&AccountInfo>,
) -> Result<()> {
    if !config.grant_deny_list_enabled {
        require!(deny_record.is_none(), ZamaHostError::AclDenyRecordMismatch);
        return Ok(());
    }
    let info = deny_record.ok_or_else(|| error!(ZamaHostError::AclDenyRecordMissing))?;
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

    // ---- ordering invariant, expressed in (total, depth) terms ----

    #[test]
    fn check_hcu_ordering_accepts_total_ge_depth() {
        assert!(check_hcu_ordering(20_000_000, 5_000_000).is_ok()); // total > depth
        assert!(check_hcu_ordering(5_000_000, 5_000_000).is_ok()); // total == depth (boundary)
    }

    #[test]
    fn check_hcu_ordering_rejects_total_lt_depth() {
        // depth=5M, set total=4M -> reject. total=20M, set depth=21M -> reject.
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
        // 0 = +inf on either side (the unlimited sentinel); both 0 is the deploy default.
        assert!(check_hcu_ordering(0, 5_000_000).is_ok());
        assert!(check_hcu_ordering(4_000_000, 0).is_ok());
        assert!(check_hcu_ordering(0, 0).is_ok());
    }

    #[test]
    fn hcu_ordering_unreachable_under_setter_sequences() {
        // No ordered setter sequence can reach 0 < total < depth. Simulate both setters as
        // guarded mutations over a small value space; the bad state must never be reachable.
        let values = [0u64, 1, 5, 10, 20];
        for &a in &values {
            for &b in &values {
                for &c in &values {
                    let (mut total, mut depth) = (0u64, 0u64); // init state (both disabled)
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

    // ---- the config-updated event carries the HCU limits (compile-time proof) ----

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
