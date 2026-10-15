//! Shared account contexts and validation helpers for instruction modules.

use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::{get_stack_height, TRANSACTION_LEVEL_STACK_HEIGHT},
    program::{invoke, invoke_signed},
    system_instruction,
};

use crate::{
    constants::HCU_UNLIMITED,
    errors::ZamaHostError,
    state::{deny_scope_address, AppScope, DenyScopeRecord, HostConfig, PauseArea},
};
use crate::{events::HostConfigUpdatedEvent, state::EVENT_VERSION};

/// Error mapping for the shared EVM signer-address checks used by KMS contexts and the
/// coprocessor signer set.
pub(super) struct EvmSignerSetErrors {
    pub empty: ZamaHostError,
    pub too_many: ZamaHostError,
    pub duplicate: ZamaHostError,
    pub zero: ZamaHostError,
}

/// Shared registration invariants for an EVM secp256k1 signer set: non-empty, within `max`,
/// no zero address, no duplicates. Threshold checks stay at the call site (KMS has several).
pub(super) fn assert_evm_signer_set(
    signers: &[[u8; 20]],
    max: usize,
    errors: EvmSignerSetErrors,
) -> Result<()> {
    if signers.is_empty() {
        return Err(errors.empty.into());
    }
    if signers.len() > max {
        return Err(errors.too_many.into());
    }
    if signers.contains(&[0u8; 20]) {
        return Err(errors.zero.into());
    }
    for i in 0..signers.len() {
        for j in (i + 1)..signers.len() {
            if signers[i] == signers[j] {
                return Err(errors.duplicate.into());
            }
        }
    }
    Ok(())
}

/// A verification threshold must be at least one and at most the signer count.
pub(super) fn assert_quorum_threshold(
    threshold: u8,
    signer_count: usize,
    error: ZamaHostError,
) -> Result<()> {
    if threshold >= 1 && (threshold as usize) <= signer_count {
        Ok(())
    } else {
        Err(error.into())
    }
}

/// Validates a coprocessor signer set + threshold and packs it into the fixed-capacity array
/// stored in `HostConfig`. Shared by `initialize_host_config` and the admin setter.
/// Address checks run before the threshold check so a multi-violation input surfaces the
/// registration shape error first (empty / too-many / zero / duplicate), then threshold.
pub(super) fn validate_and_pack_coprocessor_signers(
    signers: &[[u8; 20]],
    threshold: u8,
) -> Result<([[u8; 20]; HostConfig::MAX_COPROCESSOR_SIGNERS], u8)> {
    assert_evm_signer_set(
        signers,
        HostConfig::MAX_COPROCESSOR_SIGNERS,
        EvmSignerSetErrors {
            empty: ZamaHostError::EmptyCoprocessorSignerSet,
            too_many: ZamaHostError::TooManyCoprocessorSigners,
            duplicate: ZamaHostError::DuplicateCoprocessorSigner,
            zero: ZamaHostError::ZeroCoprocessorSigner,
        },
    )?;
    assert_quorum_threshold(
        threshold,
        signers.len(),
        ZamaHostError::InvalidCoprocessorThreshold,
    )?;
    Ok((
        crate::state::pack_coprocessor_signers(signers),
        signers.len() as u8,
    ))
}

pub(super) fn assert_no_remaining_accounts(remaining_accounts: &[AccountInfo]) -> Result<()> {
    require!(
        remaining_accounts.is_empty(),
        ZamaHostError::UnexpectedRemainingAccounts
    );
    Ok(())
}

pub(super) fn assert_admin(config: &Account<HostConfig>, admin: &Signer) -> Result<()> {
    require_keys_eq!(
        config.admin,
        admin.key(),
        ZamaHostError::HostConfigAdminMismatch
    );
    Ok(())
}

/// Fails while `area` is paused.
pub(super) fn assert_not_paused(config: &Account<HostConfig>, area: PauseArea) -> Result<()> {
    config.paused.require_running(area)
}

/// A wallet's signature reaches every CPI of the transaction it signed, so any program the wallet
/// calls could act with it. A PDA signs only through its own program's `invoke_signed`, so a PDA,
/// such as a Squads vault, may still sign through CPI.
pub(super) fn require_top_level_unless_pda(signer: &Pubkey, error: ZamaHostError) -> Result<()> {
    if get_stack_height() == TRANSACTION_LEVEL_STACK_HEIGHT || !signer.is_on_curve() {
        return Ok(());
    }
    Err(error.into())
}

/// Emits the config snapshot after a config change. Every instruction that touches `HostConfig`
/// routes through here, except `define_kms_context` and `define_kms_epoch`, whose
/// `NewKmsContextEvent` and `NewKmsEpochEvent` carry the new current context and epoch. The emitter takes the event authority as an argument rather than using
/// `emit_cpi!` because that macro reads a binding named `ctx`, which a shared helper does not have.
pub(super) fn emit_config_updated(
    config: &HostConfig,
    signer: Pubkey,
    event_authority: &AccountInfo<'_>,
) -> Result<()> {
    crate::event_cpi::emit_event_cpi(
        event_authority,
        &HostConfigUpdatedEvent {
            version: EVENT_VERSION,
            config: crate::state::host_config_address().0,
            signer,
            admin: config.admin,
            pending_admin: config.pending_admin,
            paused: config.paused,
            grant_deny_list_enabled: config.grant_deny_list_enabled,
            max_hcu_per_tx: config.max_hcu_per_tx,
            max_hcu_depth_per_tx: config.max_hcu_depth_per_tx,
            hcu_block_cap_per_app: config.hcu_block_cap_per_app,
        },
    )
}

/// Enforces the per-app block-cap ordering guard: a metering-band cap (`0 < value < u64::MAX`)
/// must be at least `max_hcu_per_tx`, so a single legal max-per-tx execution always fits on a fresh
/// meter. The two sentinels are exempt: `value == u64::MAX` (unrestricted) and `value == 0`
/// (deliberate ban of untrusted apps). `max_hcu_per_tx == u64::MAX` means the per-transaction cap is
/// unlimited, so the guard is vacuous.
pub(super) fn check_block_cap_ordering(value: u64, max_hcu_per_tx: u64) -> Result<()> {
    require!(
        value == 0
            || value == HCU_UNLIMITED
            || max_hcu_per_tx == HCU_UNLIMITED
            || value >= max_hcu_per_tx,
        ZamaHostError::HcuBlockCapBelowMaxPerTx
    );
    Ok(())
}

/// Enforces the HCU limit ordering invariant `max_hcu_per_tx >= max_hcu_depth_per_tx`, treating
/// `u64::MAX` as unlimited on either side. Both setters reuse this in `(total, depth)` terms:
/// `set_max_hcu_per_tx(v)` calls `check_hcu_ordering(v, cfg.max_hcu_depth_per_tx)`;
/// `set_max_hcu_depth_per_tx(v)` calls `check_hcu_ordering(cfg.max_hcu_per_tx, v)`.
pub(super) fn check_hcu_ordering(total: u64, depth: u64) -> Result<()> {
    require!(
        depth == HCU_UNLIMITED || total >= depth,
        ZamaHostError::HcuLimitOrderingInvalid
    );
    Ok(())
}

/// The keys a persistent write allows on its new handle: none may be the zero key and none may
/// repeat, so every sealed leaf names a distinct real grantee.
pub(super) fn assert_allow_keys(keys: &[Pubkey]) -> Result<()> {
    for (index, key) in keys.iter().enumerate() {
        require!(*key != Pubkey::default(), ZamaHostError::InvalidAllowKey);
        require!(
            !keys[index + 1..].contains(key),
            ZamaHostError::InvalidAllowKey
        );
    }
    Ok(())
}

pub(super) fn check_scope_not_denied(
    config: &HostConfig,
    app: AppScope,
    deny_record: Option<&UncheckedAccount>,
) -> Result<()> {
    let info = deny_record.map(|account| account.to_account_info());
    check_scope_not_denied_info(config, app, info.as_ref())
}

/// Deny-list gate for an application: with the list enabled, its canonical deny record must be
/// supplied and must not mark it denied (an uninitialized record means "never denied"). With
/// the list disabled no record may be passed.
pub(super) fn check_scope_not_denied_info(
    config: &HostConfig,
    app: AppScope,
    deny_record: Option<&AccountInfo>,
) -> Result<()> {
    if !config.grant_deny_list_enabled {
        require!(deny_record.is_none(), ZamaHostError::DenyRecordMismatch);
        return Ok(());
    }
    let info = deny_record.ok_or_else(|| error!(ZamaHostError::DenyRecordMissing))?;
    let (expected, _) = deny_scope_address(app);
    require_keys_eq!(info.key(), expected, ZamaHostError::DenyRecordMismatch);

    if is_uninitialized_pda_account(info, ZamaHostError::DenyRecordMismatch)? {
        return Ok(());
    }
    // At the canonical address and program-owned, the record was created and written only by
    // `set_deny_scope` for this application.
    require_keys_eq!(*info.owner, crate::ID, ZamaHostError::DenyRecordMismatch);
    let mut data: &[u8] = &info.try_borrow_data()?;
    let record = DenyScopeRecord::try_deserialize(&mut data)?;
    require!(!record.denied, ZamaHostError::ScopeDenied);
    Ok(())
}

/// True when a PDA slot is still uninitialized: system-owned and empty. A system-owned empty
/// account can never be executable, so that combination fails closed with `error`. Shared by
/// the deny-record and HCU-witness readers — each caller supplies the error that names its own
/// account.
pub(super) fn is_uninitialized_pda_account(
    info: &AccountInfo,
    error: ZamaHostError,
) -> Result<bool> {
    if info.owner == &System::id() && info.data_is_empty() {
        if info.executable {
            return Err(error.into());
        }
        return Ok(true);
    }
    Ok(false)
}

/// Creates a program-owned PDA at `account`.
///
/// Common path (`lamports() == 0`): one `create_account` under `invoke_signed`. The seeds are
/// predictable for several PDAs (the HCU block meter, ACL permission/record accounts), so a
/// third party can pre-fund the address with a bare `transfer` before the program first creates
/// it. `create_account` refuses any non-zero-lamport target (`AccountAlreadyInUse`), which would
/// let that donation permanently block creation — a griefing DoS. The three-step path (transfer
/// the shortfall, `allocate`, `assign`) is the squat fallback only: an attacker can only add
/// lamports (they cannot sign for the PDA to `allocate`/`assign` it), and `allocate`/`assign`
/// ignore the balance. Returns the rent-exempt minimum so callers can assert `lamports() >= rent`.
fn fund_allocate_assign<'info>(
    payer: &AccountInfo<'info>,
    account: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    space: usize,
    seeds: &[&[u8]],
) -> Result<u64> {
    let rent = Rent::get()?.minimum_balance(space);
    let balance = account.lamports();
    if balance == 0 {
        invoke_signed(
            &system_instruction::create_account(
                payer.key,
                account.key,
                rent,
                space as u64,
                &crate::ID,
            ),
            &[payer.clone(), account.clone(), system_program.clone()],
            &[seeds],
        )?;
        return Ok(rent);
    }
    // Squat fallback: top up only the shortfall. A donation of `rent` or more makes the
    // transfer a no-op; the surplus is harmless (the account is simply more-than-rent-exempt).
    // The transfer is authorized by `payer`, already a transaction signer, so no PDA seeds are
    // needed for it.
    if balance < rent {
        invoke(
            &system_instruction::transfer(payer.key, account.key, rent - balance),
            &[payer.clone(), account.clone(), system_program.clone()],
        )?;
    }
    // Both allocate and assign are indifferent to the lamport balance; each requires the PDA to
    // "sign", which only this program can do via `seeds`, so a squatter cannot perform them.
    invoke_signed(
        &system_instruction::allocate(account.key, space as u64),
        &[account.clone(), system_program.clone()],
        &[seeds],
    )?;
    invoke_signed(
        &system_instruction::assign(account.key, &crate::ID),
        &[account.clone(), system_program.clone()],
        &[seeds],
    )?;
    Ok(rent)
}

/// Creates the program-owned PDA `account`, which must still be a system-owned, empty account; any
/// other starting state, or a creation that did not take, fails with `error`.
pub(super) fn create_pda<'info>(
    payer: &AccountInfo<'info>,
    account: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    space: usize,
    seeds: &[&[u8]],
    error: ZamaHostError,
) -> Result<()> {
    if *account.owner != System::id() || !account.data_is_empty() || account.executable {
        return Err(error.into());
    }
    // Predictable addresses are pre-fundable, so creation tolerates a donated balance (see
    // `fund_allocate_assign`); a lamport donation is not initialization, so the system-owned and
    // empty entry check above still holds the no-overwrite invariant.
    let rent = fund_allocate_assign(payer, account, system_program, space, seeds)?;
    if *account.owner != crate::ID
        || account.executable
        || account.data_len() != space
        || account.lamports() < rent
    {
        return Err(error.into());
    }
    Ok(())
}

pub(super) fn write_account<T: AccountSerialize>(info: &AccountInfo, account: &T) -> Result<()> {
    let mut data = info.try_borrow_mut_data()?;
    let mut cursor = &mut data[..];
    account.try_serialize(&mut cursor)?;
    Ok(())
}

/// Reallocs the account and tops up rent from `payer` when `target_space` grows past the
/// account's current data length. Never shrinks.
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
        invoke(
            &system_instruction::transfer(payer.key, account.key, top_up),
            &[payer.clone(), account.clone(), system_program.clone()],
        )?;
    }
    account.resize(target_space)?;
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

        assert!(is_uninitialized_pda_account(&info, ZamaHostError::DenyRecordMismatch).unwrap());
    }

    #[test]
    fn absent_deny_record_rejects_executable_system_empty_account() {
        let key = Pubkey::new_unique();
        let owner = System::id();
        let mut lamports = 0;
        let mut data = Vec::new();
        let info = AccountInfo::new(&key, false, false, &mut lamports, &mut data, &owner, true);

        assert_eq!(
            is_uninitialized_pda_account(&info, ZamaHostError::DenyRecordMismatch).unwrap_err(),
            error!(ZamaHostError::DenyRecordMismatch)
        );
    }

    #[test]
    fn absent_deny_record_ignores_non_system_empty_account() {
        let key = Pubkey::new_unique();
        let owner = crate::ID;
        let mut lamports = 0;
        let mut data = Vec::new();
        let info = AccountInfo::new(&key, false, false, &mut lamports, &mut data, &owner, false);

        assert!(!is_uninitialized_pda_account(&info, ZamaHostError::DenyRecordMismatch).unwrap());
    }

    #[test]
    fn allow_keys_reject_zero_and_duplicates_and_accept_empty() {
        let a = Pubkey::new_unique();
        let b = Pubkey::new_unique();
        assert!(assert_allow_keys(&[]).is_ok());
        assert!(assert_allow_keys(&[a, b]).is_ok());
        assert_eq!(
            assert_allow_keys(&[a, Pubkey::default()]).unwrap_err(),
            error!(ZamaHostError::InvalidAllowKey)
        );
        assert_eq!(
            assert_allow_keys(&[a, b, a]).unwrap_err(),
            error!(ZamaHostError::InvalidAllowKey)
        );
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
    fn check_hcu_ordering_max_is_unlimited() {
        // u64::MAX = +inf on either side (the unlimited sentinel); both MAX is the deploy default.
        assert!(check_hcu_ordering(u64::MAX, 5_000_000).is_ok());
        assert!(check_hcu_ordering(4_000_000, u64::MAX).is_ok());
        assert!(check_hcu_ordering(u64::MAX, u64::MAX).is_ok());
    }

    // ---- block-cap ordering guard: sentinels exempt, band must be >= max_hcu_per_tx ----

    #[test]
    fn check_block_cap_ordering_sentinels_always_ok() {
        // 0 (ban) and u64::MAX (unrestricted) bypass the ordering guard at any max_hcu_per_tx.
        assert!(check_block_cap_ordering(0, 20_000_000).is_ok());
        assert!(check_block_cap_ordering(u64::MAX, 20_000_000).is_ok());
    }

    #[test]
    fn check_block_cap_ordering_band_respects_max_per_tx() {
        // Band value at/above max_hcu_per_tx is accepted; below is rejected.
        assert!(check_block_cap_ordering(20_000_000, 20_000_000).is_ok()); // boundary
        assert!(check_block_cap_ordering(25_000_000, 20_000_000).is_ok());
        assert_eq!(
            check_block_cap_ordering(19_000_000, 20_000_000).unwrap_err(),
            error!(ZamaHostError::HcuBlockCapBelowMaxPerTx)
        );
    }

    #[test]
    fn check_block_cap_ordering_unlimited_per_tx_is_vacuous() {
        // max_hcu_per_tx == u64::MAX (per-transaction cap off) accepts even a tiny band value.
        assert!(check_block_cap_ordering(1, u64::MAX).is_ok());
    }
}
