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
    /// Sender's stable balance lineage: read for the current handle, then
    /// superseded in place as the output.
    pub(crate) from_balance_value: AccountInfo<'info>,
    /// Recipient's stable balance lineage: read for the current handle, then
    /// superseded in place as the output.
    pub(crate) to_balance_value: AccountInfo<'info>,
    /// Sender's stable transferred-amount lineage, superseded every transfer.
    pub(crate) transferred_amount_value: AccountInfo<'info>,
    pub(crate) zama_event_authority: &'a UncheckedAccount<'info>,
    pub(crate) zama_program: &'a Program<'info, ZamaHost>,
    pub(crate) host_config: &'a Account<'info, zama_host::HostConfig>,
    pub(crate) deny_subject_records: &'a [AccountInfo<'info>],
    pub(crate) system_program: &'a Program<'info, System>,
    /// Per-`compute_subject` HCU block meter forwarded into the host `fhe_eval` CPI (`None` =
    /// untrusted, no meter). The host keys the meter on the mint's compute signer PDA.
    pub(crate) hcu_block_meter: Option<AccountInfo<'info>>,
    /// HCU trust witness forwarded into the host `fhe_eval` CPI (`None` = untrusted).
    pub(crate) hcu_trusted_app_record: Option<AccountInfo<'info>>,
}

/// Where a transfer's amount comes from. The `ge -> sub -> select` debit and `add` credit that
/// move the two balance lineages are identical for both arms; only how the amount operand enters
/// the eval frame differs.
pub(crate) enum TransferAmountSource<'info> {
    /// EVM `FHE.fromExternal` parity: a coprocessor-attested fresh client-side encryption,
    /// verified in-frame and transient-allowed for this eval (no durable amount account).
    Attested(zama_host::CoprocessorInputAttestation),
    /// EVM computed/received `euint64` parity: an existing on-chain `EncryptedValue` lineage,
    /// spent as a read-only durable operand at its current handle. It is never superseded and
    /// never consumed — only the two balance lineages change. The token's spend gate (signing
    /// owner in the value's subject set) and euint64 type check run in the instruction handler
    /// before this reaches the eval builder; the host re-checks the handle is current and that the
    /// mint's compute subject is allowed on the value, in-frame.
    ExistingValue {
        amount_value: AccountInfo<'info>,
        amount_handle: [u8; 32],
    },
}

impl TransferAmountSource<'_> {
    fn amount_handle(&self) -> [u8; 32] {
        match self {
            Self::Attested(attestation) => attestation.input_handle,
            Self::ExistingValue { amount_handle, .. } => *amount_handle,
        }
    }

    /// Domain-separates the eval context id per arm so the two amount sources never derive
    /// colliding handles for the same (mint, from, to, amount handle) tuple.
    fn context_tag(&self) -> &'static [u8] {
        match self {
            Self::Attested(_) => b"combined",
            Self::ExistingValue { .. } => b"from-value",
        }
    }
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
        // is verified in-frame. The `ExistingValue` arm is gated instead by the token spend gate and
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

    let (new_from_handle, transferred_handle, new_to_handle) = execute_transfer_eval(
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
fn execute_transfer_eval<'info>(
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
    let context_id = transfer_eval_context(
        amount_source.context_tag(),
        mint_key,
        from_key,
        to_key,
        amount_source.amount_handle(),
    )?;
    let from_balance = uint64_from_value(old_from_handle, mint_key, from_key, balance_label())?;
    let to_balance = uint64_from_value(old_to_handle, mint_key, to_key, balance_label())?;
    let compute_signer = accounts.compute_signer.key();
    let balance_access = |owner| fhe::DurableAudience::for_owner(owner, compute_signer);
    let transferred_access = {
        let access = fhe::DurableAudience::for_owner(from_owner, compute_signer);
        if to_owner != from_owner {
            access.with_owner(to_owner)
        } else {
            access
        }
    };
    let from_output = fhe::DurableOutput::new(
        accounts.from_balance_value.clone(),
        durable_slot(mint_key, from_key, balance_label()),
        balance_access(from_owner),
    )?;
    let transferred_output = fhe::DurableOutput::new(
        accounts.transferred_amount_value.clone(),
        durable_slot(mint_key, from_key, transferred_amount_label()),
        transferred_access,
    )?;
    let to_output = fhe::DurableOutput::new(
        accounts.to_balance_value.clone(),
        durable_slot(mint_key, to_key, balance_label()),
        balance_access(to_owner),
    )?;
    let mut builder =
        zama_fhe::EvalBuilder::new(context_id, zama_fhe::EvalAppAuthority::new(from_key));
    let amount: zama_fhe::Uint64Handle = match amount_source {
        // fromExternal: the amount is a coprocessor-attested external input, verified in-frame and
        // transient-allowed for this eval (no durable amount handle / ACL account).
        TransferAmountSource::Attested(amount_attestation) => builder
            .verified_input(amount_attestation.clone())
            .map_err(invalid_eval_plan)?,
        // Existing value: the amount is an on-chain lineage's current handle, read as a durable
        // operand. The slot is derived from the value's own canonical fields, so its PDA equals the
        // passed account; the host re-checks handle-is-current and compute-subject membership.
        TransferAmountSource::ExistingValue { amount_value, .. } => {
            let value = fhe::read_encrypted_value(amount_value)?;
            uint64_from_value(
                value.current_handle,
                value.acl_domain_key,
                value.app_account,
                value.encrypted_value_label,
            )?
        }
    };
    let success = builder
        .ge(from_balance, amount, zama_fhe::Output::transient())
        .map_err(invalid_eval_plan)?;
    let debit_candidate = builder
        .sub(from_balance, amount, zama_fhe::Output::transient())
        .map_err(invalid_eval_plan)?;
    let new_from = builder
        .if_then_else(success, debit_candidate, from_balance, from_output.output())
        .map_err(invalid_eval_plan)?;
    let transferred = builder
        .sub(from_balance, new_from, transferred_output.output())
        .map_err(invalid_eval_plan)?;
    builder
        .add(to_balance, transferred, to_output.output())
        .map_err(invalid_eval_plan)?;
    let plan = builder.finish().map_err(invalid_eval_plan)?;
    let compute_authority =
        fhe::ComputeAuthority::for_mint(accounts.compute_signer, mint_key, compute_signer_bump)?;
    // Durable output accounts are the same for both arms; the existing-value arm adds the amount
    // lineage as a read-only durable input operand the plan now requires.
    let mut dynamic_accounts = vec![
        from_output.account_info(),
        transferred_output.account_info(),
        to_output.account_info(),
    ];
    if let TransferAmountSource::ExistingValue { amount_value, .. } = amount_source {
        // The amount lineage can legitimately alias one of the output accounts (spending the entire
        // balance, or re-sending a transferred_amount that is also this frame's output). The plan
        // already merges those into one slot, so only add the amount when it is a distinct account.
        if !dynamic_accounts
            .iter()
            .any(|account| account.key() == amount_value.key())
        {
            dynamic_accounts.push(amount_value.clone());
        }
    }
    let eval_accounts = fhe::EvalAccountSet::for_plan(
        &plan,
        dynamic_accounts,
        [
            fhe::OutputAuthority::token_account(accounts.from_account)?,
            fhe::OutputAuthority::token_account(accounts.to_account)?,
        ],
    )?;

    fhe::eval(fhe::Eval {
        context: fhe::EvalContext {
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
        accounts: &eval_accounts,
        plan,
    })?;

    Ok((
        from_output.handle()?,
        transferred_output.handle()?,
        to_output.handle()?,
    ))
}

pub(crate) fn invalid_eval_plan(error: zama_fhe::EvalBuildError) -> anchor_lang::error::Error {
    msg!("invalid FHE eval plan: {:?}", error);
    error!(ConfidentialTokenError::InvalidFheEvalPlan)
}

pub(crate) fn durable_slot(
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
) -> zama_fhe::DurableSlot {
    zama_fhe::DurableSlot::new(
        acl_domain_key,
        app_account,
        zama_fhe::DurableLabel::new(encrypted_value_label),
    )
}

pub(crate) fn uint64_from_value(
    handle: [u8; 32],
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
) -> Result<zama_fhe::Uint64Handle> {
    zama_fhe::Uint64Handle::durable(
        handle,
        durable_slot(acl_domain_key, app_account, encrypted_value_label),
    )
    .map_err(invalid_eval_plan)
}

pub(crate) fn transfer_eval_context(
    tag: &[u8],
    mint: Pubkey,
    from_token_account: Pubkey,
    to_token_account: Pubkey,
    amount_handle: [u8; 32],
) -> Result<zama_fhe::EvalContextId> {
    let context_id = solana_sha256_hasher::hashv(&[
        b"confidential-token-transfer-eval-v1",
        tag,
        mint.as_ref(),
        from_token_account.as_ref(),
        to_token_account.as_ref(),
        &amount_handle,
    ])
    .to_bytes();
    zama_fhe::EvalContextId::new(context_id).map_err(invalid_eval_plan)
}

/// Validates a coprocessor-attested transfer/burn amount (EVM `fromExternal` parity). The host
/// re-verifies the attestation signature and enforces caller == `contract_address` in-frame; the
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

/// Lineage checks for the redeem path: burned-amount handle type, canonical
/// address, domain/app account, the burned-amount label, and current membership
/// for the owner and mint compute signer. Does NOT authorize the specific handle:
/// the redeem path proves the handle's publicness via the exact-handle MMR
/// public-decrypt proof verified inside the `verify_public_decrypt` CPI, since the
/// burn already made the handle public (DD-036 / Vector 2). The handle need not be
/// the live one, so a historical handle superseded by a later burn stays redeemable.
pub(crate) fn assert_burned_amount_lineage(
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
        amount_value.acl_domain_key,
        mint,
        ConfidentialTokenError::AclDomainKeyMismatch
    );
    require_keys_eq!(
        amount_value.app_account,
        token_account,
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_value.encrypted_value_label == burned_amount_label(),
        ConfidentialTokenError::AmountAclMismatch
    );
    require_keys_eq!(
        amount_value.key(),
        encrypted_value_address(mint, token_account, burned_amount_label()).0,
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
        mint.acl_domain_key,
        mint.key(),
        ConfidentialTokenError::AclDomainKeyMismatch
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
