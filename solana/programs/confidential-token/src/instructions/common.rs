//! Shared confidential-token instruction helpers.
//!
//! This module holds cross-instruction account shape checks, FHE CPI builders,
//! and deterministic labels used by the token handlers.

use super::*;

pub(crate) fn assert_no_remaining_accounts(remaining_accounts: &[AccountInfo]) -> Result<()> {
    require!(
        remaining_accounts.is_empty(),
        ConfidentialTokenError::UnexpectedRemainingAccounts
    );
    Ok(())
}

pub(crate) struct TransferAccounts<'a, 'info> {
    pub(crate) payer: &'a Signer<'info>,
    pub(crate) transfer_authority: Pubkey,
    pub(crate) mint: &'a Account<'info, ConfidentialMint>,
    pub(crate) from_account: &'a Account<'info, ConfidentialTokenAccount>,
    pub(crate) to_account: &'a Account<'info, ConfidentialTokenAccount>,
    pub(crate) compute_signer: &'a UncheckedAccount<'info>,
    /// Sender's stable balance encrypted value account: read for the current handle, then
    /// replaced in place as the output.
    pub(crate) from_balance_value: AccountInfo<'info>,
    /// Recipient's stable balance encrypted value account: read for the current handle, then
    /// replaced in place as the output.
    pub(crate) to_balance_value: AccountInfo<'info>,
    /// Sender's stable transferred-amount encrypted value account, replaced every transfer.
    pub(crate) transferred_amount_value: AccountInfo<'info>,
    pub(crate) zama_event_authority: &'a UncheckedAccount<'info>,
    pub(crate) zama_program: &'a Program<'info, ZamaHost>,
    pub(crate) host_config: &'a Account<'info, zama_host::HostConfig>,
    pub(crate) deny_subject_records: &'a [AccountInfo<'info>],
    pub(crate) system_program: &'a Program<'info, System>,
    /// Per-`compute_subject` HCU block meter forwarded into the host `fhe_execute` CPI (`None` =
    /// untrusted, no meter). The host keys the meter on the mint's compute signer PDA.
    pub(crate) hcu_block_meter: Option<AccountInfo<'info>>,
    /// HCU trust witness forwarded into the host `fhe_execute` CPI (`None` = untrusted).
    pub(crate) hcu_trusted_app_record: Option<AccountInfo<'info>>,
}

/// Where a transfer's amount comes from. The `ge -> sub -> select` debit and `add` credit that
/// move the two balance encrypted value accounts are identical for both arms; only how the amount operand enters
/// the execution differs.
pub(crate) enum TransferAmountSource<'info> {
    /// EVM `FHE.fromExternal` parity: a coprocessor-attested fresh client-side encryption,
    /// verified in-execution and transient-allowed for this eval (no persistent amount account).
    Attested(zama_host::CoprocessorInputAttestation),
    /// EVM computed/received `euint64` parity: an existing on-chain `EncryptedValue` encrypted value account,
    /// spent as a read-only persistent operand at its current handle. It is never replaced and
    /// never consumed — only the two balance encrypted value accounts change. The token's spend gate (signing
    /// owner in the value's subject set) and euint64 type check run in the instruction handler
    /// before this reaches the eval builder; the host re-checks the handle is current and that the
    /// mint's compute subject is allowed on the value, in-execution.
    ExistingValue { amount_value: AccountInfo<'info> },
}

pub(crate) struct TransferOutcome {
    pub(crate) mint: Pubkey,
    pub(crate) from_owner: Pubkey,
    pub(crate) from_token_account: Pubkey,
    pub(crate) old_from_handle: [u8; 32],
    pub(crate) new_from_handle: [u8; 32],
    pub(crate) from_encrypted_value: Pubkey,
    pub(crate) transferred_handle: [u8; 32],
    pub(crate) transferred_encrypted_value: Pubkey,
    pub(crate) to_owner: Pubkey,
    pub(crate) to_token_account: Pubkey,
    pub(crate) old_to_handle: [u8; 32],
    pub(crate) new_to_handle: [u8; 32],
    pub(crate) to_encrypted_value: Pubkey,
}

pub(crate) fn execute_transfer<'info>(
    accounts: TransferAccounts<'_, 'info>,
    compute_signer_bump: u8,
    amount_source: TransferAmountSource<'info>,
) -> Result<Option<TransferOutcome>> {
    assert_confidential_mint_shape(accounts.mint)?;
    let mint_key = accounts.mint.key();
    let compute_signer = accounts.mint.compute_signer;
    let from = accounts.from_account;
    let to = accounts.to_account;

    if let TransferAmountSource::Attested(amount_attestation) = &amount_source {
        // EVM `fromExternal` parity for the amount: the attested input must be authored by the
        // sender (user) and bound to this mint's compute-signer PDA (the `msg.sender`/contract
        // analog the host re-checks against `compute_subject`). The coprocessor signature over both
        // is verified in-execution. The `ExistingValue` arm is gated instead by the token spend gate and
        // euint64 type check in its instruction handler.
        assert_amount_attestation_binding(
            amount_attestation,
            accounts.transfer_authority,
            compute_signer,
        )?;
    }
    require_keys_eq!(from.mint, mint_key, ConfidentialTokenError::MintMismatch);
    require_keys_eq!(to.mint, mint_key, ConfidentialTokenError::MintMismatch);
    assert_confidential_token_account_shape(from, mint_key, from.owner)?;
    assert_confidential_token_account_shape(to, mint_key, to.owner)?;
    require_keys_eq!(
        accounts.compute_signer.key(),
        compute_signer,
        ConfidentialTokenError::ComputeSignerMismatch
    );
    require_keys_eq!(
        accounts.from_balance_value.key(),
        from.balance_encrypted_value,
        ConfidentialTokenError::CurrentEncryptedValueMismatch
    );
    require_keys_eq!(
        accounts.to_balance_value.key(),
        to.balance_encrypted_value,
        ConfidentialTokenError::CurrentEncryptedValueMismatch
    );
    let from_key = from.key();
    let to_key = to.key();
    let from_owner = from.owner;
    let to_owner = to.owner;
    let from_encrypted_value = accounts.from_balance_value.key();
    let to_encrypted_value = accounts.to_balance_value.key();
    if from_key == to_key {
        assert_no_remaining_accounts(accounts.deny_subject_records)?;
        return Ok(None);
    }

    let old_from_handle = fhe::read_encrypted_value(&accounts.from_balance_value)?.current_handle;
    let old_to_handle = fhe::read_encrypted_value(&accounts.to_balance_value)?.current_handle;

    let (new_from_handle, transferred_handle, new_to_handle) = compute_transfer_handles(
        &accounts,
        compute_signer_bump,
        &amount_source,
        mint_key,
        old_from_handle,
        old_to_handle,
    )?;

    let transferred_encrypted_value = accounts.transferred_amount_value.key();

    Ok(Some(TransferOutcome {
        mint: mint_key,
        from_owner,
        from_token_account: from_key,
        old_from_handle,
        new_from_handle,
        from_encrypted_value,
        transferred_handle,
        transferred_encrypted_value,
        to_owner,
        to_token_account: to_key,
        old_to_handle,
        new_to_handle,
        to_encrypted_value,
    }))
}

#[allow(clippy::too_many_arguments)]
fn compute_transfer_handles<'info>(
    accounts: &TransferAccounts<'_, 'info>,
    compute_signer_bump: u8,
    amount_source: &TransferAmountSource<'info>,
    mint_key: Pubkey,
    old_from_handle: [u8; 32],
    old_to_handle: [u8; 32],
) -> Result<([u8; 32], [u8; 32], [u8; 32])> {
    let from_key = accounts.from_account.key();
    let to_key = accounts.to_account.key();
    let from_owner = accounts.from_account.owner;
    let to_owner = accounts.to_account.owner;
    // The token program's ACL domain is the mint, so every value slot below is derived under it.
    let mint_domain = zama_fhe::Domain::new(mint_key);
    let from_balance = uint64_from_value(
        old_from_handle,
        mint_domain,
        from_key,
        encrypted_balance_label(),
    )?;
    let to_balance = uint64_from_value(
        old_to_handle,
        mint_domain,
        to_key,
        encrypted_balance_label(),
    )?;
    let compute_signer = accounts.compute_signer.key();
    let balance_access = |owner| fhe::PersistentAudience::for_owner(owner, compute_signer);
    let transferred_access = {
        let access = fhe::PersistentAudience::for_owner(from_owner, compute_signer);
        if to_owner != from_owner {
            access.with_owner(to_owner)
        } else {
            access
        }
    };
    let from_output = fhe::PersistentOutput::new(
        accounts.from_balance_value.clone(),
        encrypted_value_id(mint_domain, from_key, encrypted_balance_label()),
        balance_access(from_owner),
    )?;
    let transferred_output = fhe::PersistentOutput::new(
        accounts.transferred_amount_value.clone(),
        encrypted_value_id(mint_domain, from_key, encrypted_transferred_amount_label()),
        transferred_access,
    )?;
    let to_output = fhe::PersistentOutput::new(
        accounts.to_balance_value.clone(),
        encrypted_value_id(mint_domain, to_key, encrypted_balance_label()),
        balance_access(to_owner),
    )?;
    // Existing value: the amount is an on-chain encrypted value account's current handle, read as a
    // persistent operand. The slot is derived from the value's own canonical fields, so its PDA
    // equals the passed account; the host re-checks handle-is-current and compute-subject
    // membership. Read here rather than inside the execution closure: a stored value belongs to no
    // builder, and reading the account is this program's error to report, not the builder's.
    let stored_amount = match amount_source {
        TransferAmountSource::Attested(_) => None,
        TransferAmountSource::ExistingValue { amount_value, .. } => {
            let value = fhe::read_encrypted_value(amount_value)?;
            Some(uint64_from_value(
                value.current_handle,
                zama_fhe::Domain::new(value.domain),
                value.encrypted_value_account_authority,
                value.label,
            )?)
        }
    };
    let execution = zama_fhe::FheExecution::build(
        zama_fhe::ExecutionEncryptedValueAccountAuthority::new(from_key),
        |builder| {
            let amount = match (amount_source, stored_amount) {
                // fromExternal: the amount is a coprocessor-attested external input, verified in-execution
                // and transient-allowed for this eval (no persistent amount handle / ACL account).
                (TransferAmountSource::Attested(amount_attestation), _) => {
                    builder.verified_input(amount_attestation.clone())?
                }
                (_, Some(stored)) => stored.into(),
                (TransferAmountSource::ExistingValue { .. }, None) => {
                    unreachable!("an existing-value transfer always reads its stored amount above")
                }
            };
            let success = builder.ge(from_balance, amount, zama_fhe::Output::transient())?;
            let debit_candidate =
                builder.sub(from_balance, amount, zama_fhe::Output::transient())?;
            let new_from = builder.if_then_else(
                success,
                debit_candidate,
                from_balance,
                zama_fhe::Output::transient(),
            )?;
            let transferred = builder.sub(from_balance, new_from, transferred_output.output())?;
            builder.add(
                new_from,
                zama_fhe::Scalar::<zama_fhe::Uint<64>>::u64(0),
                from_output.output(),
            )?;
            builder.add(to_balance, transferred, to_output.output())?;
            Ok(())
        },
    )
    .map_err(invalid_execution)?;
    let compute_authority =
        fhe::ComputeAuthority::for_mint(accounts.compute_signer, mint_key, compute_signer_bump)?;
    // Persistent output accounts are the same for both arms; the existing-value arm adds the amount
    // encrypted value account as a read-only persistent input operand the execution now requires.
    let mut dynamic_accounts = vec![
        from_output.account_info(),
        transferred_output.account_info(),
        to_output.account_info(),
    ];
    if let TransferAmountSource::ExistingValue { amount_value, .. } = amount_source {
        // The amount encrypted value account can legitimately alias one of the output accounts (spending the entire
        // balance, or re-sending a transferred_amount that is also this execution's output). The execution
        // already merges those into one slot, so only add the amount when it is a distinct account.
        if !dynamic_accounts
            .iter()
            .any(|account| account.key() == amount_value.key())
        {
            dynamic_accounts.push(amount_value.clone());
        }
    }
    let execution_accounts = fhe::ExecutionAccountSet::for_execution(
        &execution,
        dynamic_accounts,
        [
            fhe::OutputAuthority::token_account(accounts.from_account)?,
            fhe::OutputAuthority::token_account(accounts.to_account)?,
        ],
    )?;

    fhe::execute(fhe::Execute {
        context: fhe::ExecuteContext {
            payer: accounts.payer,
            event_authority: accounts.zama_event_authority,
            zama_program: accounts.zama_program,
            host_config: accounts.host_config,
            deny_subject_records: accounts.deny_subject_records,
            compute_authority,
            system_program: accounts.system_program,
            hcu_block_meter: accounts.hcu_block_meter.clone(),
            hcu_trusted_app_record: accounts.hcu_trusted_app_record.clone(),
        },
        accounts: &execution_accounts,
        execution,
    })?;

    Ok((
        from_output.handle()?,
        transferred_output.handle()?,
        to_output.handle()?,
    ))
}

pub(crate) fn invalid_execution(
    error: zama_fhe::FheExecutionBuildError,
) -> anchor_lang::error::Error {
    msg!("invalid FHE execution: {:?}", error);
    error!(ConfidentialTokenError::InvalidFheExecution)
}

pub(crate) fn encrypted_value_id(
    domain: zama_fhe::Domain,
    encrypted_value_account_authority: Pubkey,
    encrypted_value_label: [u8; 32],
) -> zama_fhe::EncryptedValueId {
    zama_fhe::EncryptedValueId::new(
        domain,
        encrypted_value_account_authority,
        zama_fhe::EncryptedValueLabel::new(encrypted_value_label),
    )
}

pub(crate) fn uint64_from_value(
    handle: [u8; 32],
    domain: zama_fhe::Domain,
    encrypted_value_account_authority: Pubkey,
    encrypted_value_label: [u8; 32],
) -> Result<zama_fhe::Uint64Handle> {
    zama_fhe::Uint64Handle::persistent(
        handle,
        encrypted_value_id(
            domain,
            encrypted_value_account_authority,
            encrypted_value_label,
        ),
    )
    .map_err(invalid_execution)
}

/// Validates a coprocessor-attested transfer/burn amount (EVM `fromExternal` parity). The host
/// re-verifies the attestation signature and enforces caller == `contract_address` in-execution; the
/// program binds the attested identities to this transaction: the input must be authored by
/// `expected_user` (the sender/burner) and bound to `expected_contract` (the mint compute-signer
/// PDA the host checks against `compute_subject`). The amount handle must be a confidential balance.
pub(crate) fn assert_amount_attestation_binding(
    attestation: &zama_host::CoprocessorInputAttestation,
    expected_user: Pubkey,
    expected_contract: Pubkey,
) -> Result<()> {
    require!(
        zama_host::handle_fhe_type(attestation.input_handle) == BALANCE_FHE_TYPE,
        ConfidentialTokenError::AmountHandleTypeMismatch
    );
    require_keys_eq!(
        Pubkey::new_from_array(attestation.user_address),
        expected_user,
        ConfidentialTokenError::AttestationUserMismatch
    );
    require_keys_eq!(
        Pubkey::new_from_array(attestation.contract_address),
        expected_contract,
        ConfidentialTokenError::AttestationContractMismatch
    );
    Ok(())
}

/// ValueAccount checks for the redeem path: burned-amount handle type, canonical address,
/// domain/encrypted value account authority, the burned-amount label, and current membership for
/// the owner and mint compute signer. Does NOT authorize the specific handle: the redeem path
/// proves the handle's publicness via the exact-handle MMR public-decrypt proof verified inside the
/// `verify_public_decrypt` CPI, since the burn already made the handle public (DD-036 / Vector 2).
/// The handle need not be the live one, so a historical handle replaced by a later burn stays
/// redeemable.
pub(crate) fn assert_burned_amount_value_account(
    amount_value: &Account<zama_host::EncryptedValue>,
    burned_handle: [u8; 32],
    mint: Pubkey,
    token_account: Pubkey,
    owner: Pubkey,
    compute_signer: Pubkey,
) -> Result<()> {
    require!(
        zama_host::handle_fhe_type(burned_handle) == BALANCE_FHE_TYPE,
        ConfidentialTokenError::AmountHandleTypeMismatch
    );
    require_keys_eq!(
        amount_value.domain,
        mint,
        ConfidentialTokenError::DomainMismatch
    );
    require_keys_eq!(
        amount_value.encrypted_value_account_authority,
        token_account,
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_value.label == encrypted_burned_amount_label(),
        ConfidentialTokenError::AmountAclMismatch
    );
    require_keys_eq!(
        amount_value.key(),
        encrypted_value_address(mint, token_account, encrypted_burned_amount_label()).0,
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_value.has_subject(owner),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_value.has_subject(compute_signer),
        ConfidentialTokenError::AmountAclMismatch
    );
    Ok(())
}

/// Encodes a u64 cleartext as the 32-byte big-endian (abi `uint256`) decrypted result
/// the KMS signs over in the `PublicDecryptVerification` certificate (cert-secp path).
pub(crate) fn kms_decrypted_result_bytes(cleartext_amount: u64) -> [u8; 32] {
    let mut decrypted = [0u8; 32];
    decrypted[24..].copy_from_slice(&cleartext_amount.to_be_bytes());
    decrypted
}

pub(crate) fn assert_host_config_allows_token_response(
    host_config: &Account<zama_host::HostConfig>,
) -> Result<()> {
    let (expected_key, expected_bump) = zama_host::host_config_address();
    require_keys_eq!(
        host_config.key(),
        expected_key,
        ConfidentialTokenError::RequestWitnessMismatch
    );
    require!(
        host_config.to_account_info().data_len() == 8 + zama_host::HostConfig::SPACE
            && host_config.bump == expected_bump
            && !host_config.paused,
        ConfidentialTokenError::RequestWitnessUnavailable
    );
    Ok(())
}

pub(crate) fn assert_canonical_vault_token_account(
    vault_usdc: Pubkey,
    vault_authority: Pubkey,
    underlying_mint: Pubkey,
) -> Result<()> {
    require_keys_eq!(
        vault_usdc,
        get_associated_token_address_with_program_id(
            &vault_authority,
            &underlying_mint,
            &spl_token::ID,
        ),
        ConfidentialTokenError::VaultAccountMismatch
    );
    Ok(())
}

pub(crate) fn assert_confidential_token_account_key(
    token_account: Pubkey,
    mint: Pubkey,
    owner: Pubkey,
) -> Result<()> {
    require_keys_eq!(
        token_account,
        token_account_address(mint, owner).0,
        ConfidentialTokenError::TokenAccountMismatch
    );
    Ok(())
}

pub(crate) fn assert_confidential_mint_shape(mint: &Account<ConfidentialMint>) -> Result<()> {
    require!(
        mint.to_account_info().data_len() == 8 + ConfidentialMint::SPACE,
        ConfidentialTokenError::MintAccountMismatch
    );
    require_keys_eq!(
        mint.domain,
        mint.key(),
        ConfidentialTokenError::DomainMismatch
    );
    require_keys_eq!(
        mint.compute_signer,
        compute_signer_address(mint.key()).0,
        ConfidentialTokenError::ComputeSignerMismatch
    );
    Ok(())
}

pub(crate) fn assert_confidential_token_account_shape(
    token_account: &Account<ConfidentialTokenAccount>,
    mint: Pubkey,
    owner: Pubkey,
) -> Result<()> {
    let expected_bump = token_account_address(mint, owner).1;
    assert_confidential_token_account_key(token_account.key(), mint, owner)?;
    require!(
        token_account.to_account_info().data_len() == 8 + ConfidentialTokenAccount::SPACE,
        ConfidentialTokenError::TokenAccountMismatch
    );
    require!(
        token_account.bump == expected_bump,
        ConfidentialTokenError::TokenAccountMismatch
    );
    require_keys_eq!(
        token_account.mint,
        mint,
        ConfidentialTokenError::MintMismatch
    );
    require_keys_eq!(
        token_account.owner,
        owner,
        ConfidentialTokenError::OwnerMismatch
    );
    Ok(())
}

/// Explicit deny-list consultation at redeem payout (fhevm-internal#1763): a denied signer cannot
/// cash out. Mirrors the host's own `check_grant_not_denied` model so the token layer reads the deny
/// list exactly as the host would.
///
/// When the host grant deny-list is disabled, no `deny_subject_record` may be passed. When it is
/// enabled, the canonical record PDA for `subject` must be passed: an absent (system-owned, empty)
/// record means "never denied" and clears; a present record must be the host-owned canonical PDA for
/// `subject` and must not mark it denied.
pub(crate) fn assert_redeem_subject_not_denied(
    host_config: &Account<zama_host::HostConfig>,
    subject: Pubkey,
    deny_subject_record: Option<&UncheckedAccount>,
) -> Result<()> {
    if !host_config.grant_deny_list_enabled {
        require!(
            deny_subject_record.is_none(),
            ConfidentialTokenError::RedemptionDenyRecordInvalid
        );
        return Ok(());
    }
    let info = deny_subject_record
        .ok_or(ConfidentialTokenError::RedemptionDenyRecordInvalid)?
        .to_account_info();
    let (expected, expected_bump) = zama_host::deny_subject_address(subject);
    require_keys_eq!(
        info.key(),
        expected,
        ConfidentialTokenError::RedemptionDenyRecordInvalid
    );
    // An uninitialized (system-owned, empty) record means the subject was never denied.
    if *info.owner == System::id() && info.data_is_empty() {
        require!(
            !info.executable,
            ConfidentialTokenError::RedemptionDenyRecordInvalid
        );
        return Ok(());
    }
    require_keys_eq!(
        *info.owner,
        zama_host::ID,
        ConfidentialTokenError::RedemptionDenyRecordInvalid
    );
    require!(
        info.data_len() == 8 + zama_host::DenySubjectRecord::SPACE,
        ConfidentialTokenError::RedemptionDenyRecordInvalid
    );
    let mut data: &[u8] = &info.try_borrow_data()?;
    let record = zama_host::DenySubjectRecord::try_deserialize(&mut data)?;
    require!(
        record.bump == expected_bump && record.subject == subject,
        ConfidentialTokenError::RedemptionDenyRecordInvalid
    );
    require!(
        !record.denied,
        ConfidentialTokenError::RedemptionSubjectDenied
    );
    Ok(())
}
