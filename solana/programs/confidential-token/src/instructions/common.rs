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
    pub(crate) system_program: &'a Program<'info, System>,
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
    amount_attestation: zama_host::CoprocessorInputAttestation,
) -> Result<Option<TransferOutcome>> {
    assert_confidential_mint_shape(accounts.mint)?;
    let mint_key = accounts.mint.key();
    let compute_signer = accounts.mint.compute_signer;
    let from = accounts.from_account;
    let to = accounts.to_account;

    // EVM `fromExternal` parity for the amount: the attested input must be authored by the sender
    // (user) and bound to this mint's compute-signer PDA (the `msg.sender`/contract analog the host
    // re-checks against `compute_subject`). The coprocessor signature over both is verified in-frame.
    assert_amount_attestation_binding(
        &amount_attestation,
        accounts.transfer_authority,
        compute_signer,
    )?;
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
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    require_keys_eq!(
        accounts.to_balance_value.key(),
        to.balance_encrypted_value,
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    let from_key = from.key();
    let to_key = to.key();
    let from_owner = from.owner;
    let to_owner = to.owner;
    let from_encrypted_value = accounts.from_balance_value.key();
    let to_encrypted_value = accounts.to_balance_value.key();
    if from_key == to_key {
        return Ok(None);
    }

    let old_from_handle = fhe::read_encrypted_value(&accounts.from_balance_value)?.current_handle;
    let old_to_handle = fhe::read_encrypted_value(&accounts.to_balance_value)?.current_handle;

    let (new_from_handle, transferred_handle, new_to_handle) = execute_transfer_eval(
        &accounts,
        compute_signer_bump,
        amount_attestation,
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
    amount_attestation: zama_host::CoprocessorInputAttestation,
    mint_key: Pubkey,
    old_from_handle: [u8; 32],
    old_to_handle: [u8; 32],
) -> Result<([u8; 32], [u8; 32], [u8; 32])> {
    let from_key = accounts.from_account.key();
    let to_key = accounts.to_account.key();
    let from_owner = accounts.from_account.owner;
    let to_owner = accounts.to_account.owner;
    let context_id = transfer_eval_context(
        b"combined",
        mint_key,
        from_key,
        to_key,
        amount_attestation.input_handle,
    )?;
    let from_balance = uint64_from_value(old_from_handle, mint_key, from_key, balance_label())?;
    let to_balance = uint64_from_value(old_to_handle, mint_key, to_key, balance_label())?;
    let compute_signer = accounts.compute_signer.key();
    let balance_access = |owner| {
        zama_fhe::AccessPolicy::for_owner_and_compute(owner, compute_signer)
            .map_err(invalid_eval_plan)
    };
    let transferred_access = {
        let mut access =
            zama_fhe::AccessPolicy::for_owner(from_owner).map_err(invalid_eval_plan)?;
        if to_owner != from_owner {
            access = access.with_owner(to_owner).map_err(invalid_eval_plan)?;
        }
        access
            .with_compute(compute_signer)
            .map_err(invalid_eval_plan)?
    };
    let from_output = fhe::DurableOutput::new(
        accounts.from_balance_value.clone(),
        durable_slot(mint_key, from_key, balance_label()),
        balance_access(from_owner)?,
    )?;
    let transferred_output = fhe::DurableOutput::new(
        accounts.transferred_amount_value.clone(),
        durable_slot(mint_key, from_key, transferred_amount_label()),
        transferred_access,
    )?;
    let to_output = fhe::DurableOutput::new(
        accounts.to_balance_value.clone(),
        durable_slot(mint_key, to_key, balance_label()),
        balance_access(to_owner)?,
    )?;
    let mut builder =
        zama_fhe::EvalBuilder::new(context_id, zama_fhe::EvalAppAuthority::new(from_key));
    // fromExternal: the amount is a coprocessor-attested external input, verified in-frame and
    // transient-allowed for this eval (no durable amount handle / ACL account).
    let amount: zama_fhe::Uint64Handle = builder
        .verified_input(amount_attestation)
        .map_err(invalid_eval_plan)?;
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
    let eval_accounts = fhe::EvalAccountSet::for_plan(
        &plan,
        [
            from_output.account_info(),
            transferred_output.account_info(),
            to_output.account_info(),
        ],
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
            compute_authority,
            system_program: accounts.system_program,
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

pub(crate) fn access_policy_from_subjects(
    subjects: Vec<zama_fhe::AccessSubject>,
) -> Result<zama_fhe::AccessPolicy> {
    zama_fhe::AccessPolicy::from_subjects(subjects).map_err(invalid_eval_plan)
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

/// Verifies a token-scoped amount `EncryptedValue` lineage: canonical address, correct domain,
/// a recognized amount label, and `ACL_ROLE_USE` for the mint's compute signer.
pub(crate) fn assert_token_amount_encrypted_value(
    amount_value: &Account<zama_host::EncryptedValue>,
    amount_handle: [u8; 32],
    mint: Pubkey,
    compute_signer: Pubkey,
) -> Result<()> {
    require!(
        zama_host::handle_fhe_type(amount_handle) == BALANCE_FHE_TYPE,
        ConfidentialTokenError::AmountHandleTypeMismatch
    );
    require!(
        amount_value.current_handle == amount_handle,
        ConfidentialTokenError::AmountAclMismatch
    );
    require_keys_eq!(
        amount_value.acl_domain_key,
        mint,
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        is_token_amount_label(amount_value.encrypted_value_label),
        ConfidentialTokenError::AmountAclMismatch
    );
    require_keys_eq!(
        amount_value.key(),
        encrypted_value_address(
            mint,
            amount_value.app_account,
            amount_value.encrypted_value_label
        )
        .0,
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_value.subject_has_role(compute_signer, zama_host::ACL_ROLE_USE),
        ConfidentialTokenError::AmountAclMismatch
    );
    Ok(())
}

pub(crate) fn is_token_amount_label(encrypted_value_label: [u8; 32]) -> bool {
    encrypted_value_label == wrap_amount_label()
        || encrypted_value_label == burn_amount_label()
        || encrypted_value_label == transfer_amount_label()
        || encrypted_value_label == burned_amount_label()
        || encrypted_value_label == transferred_amount_label()
}

/// Verifies a token account's burned-amount `EncryptedValue` lineage: canonical address, correct
/// domain/app account, the burned-amount label, and `ACL_ROLE_PUBLIC_DECRYPT`/`ACL_ROLE_USE` grants.
pub(crate) fn assert_burned_amount_encrypted_value(
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
    require!(
        amount_value.current_handle == burned_handle,
        ConfidentialTokenError::AmountAclMismatch
    );
    require_keys_eq!(
        amount_value.acl_domain_key,
        mint,
        ConfidentialTokenError::AmountAclMismatch
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
        amount_value.subject_has_role(owner, zama_host::ACL_ROLE_PUBLIC_DECRYPT),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_value.subject_has_role(compute_signer, zama_host::ACL_ROLE_USE),
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

/// Verifies a token account's current balance `EncryptedValue` lineage against its stored pointer.
pub(crate) fn assert_current_balance_encrypted_value(
    balance_value: &Account<zama_host::EncryptedValue>,
    token_account: &Account<ConfidentialTokenAccount>,
    mint: Pubkey,
) -> Result<()> {
    require_keys_eq!(
        balance_value.key(),
        token_account.balance_encrypted_value,
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    require_keys_eq!(
        balance_value.acl_domain_key,
        mint,
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    require_keys_eq!(
        balance_value.app_account,
        token_account.key(),
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    require!(
        balance_value.encrypted_value_label == balance_label(),
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn assert_disclosure_request_witness(
    request: &Account<DisclosureRequest>,
    request_key: Pubkey,
    mode: u8,
    mint: Pubkey,
    token_account: Pubkey,
    app_account: Pubkey,
    handle: [u8; 32],
    encrypted_value: Pubkey,
    host_config: Pubkey,
) -> Result<()> {
    let (expected_key, expected_bump) =
        disclosure_request_address(mint, request.requester, handle, request.request_nonce);
    require_keys_eq!(
        request_key,
        expected_key,
        ConfidentialTokenError::RequestWitnessMismatch
    );
    require!(
        request.to_account_info().data_len() == 8 + DisclosureRequest::SPACE
            && request.bump == expected_bump,
        ConfidentialTokenError::RequestWitnessMismatch
    );
    require!(
        request.status == REQUEST_STATUS_PENDING && request.expires_slot >= Clock::get()?.slot,
        ConfidentialTokenError::RequestWitnessUnavailable
    );
    require!(
        request.mode == mode
            && request.mint == mint
            && request.token_account == token_account
            && request.app_account == app_account
            && request.handle == handle
            && request.encrypted_value == encrypted_value
            && request.host_config == host_config
            && request.kms_context_id != 0
            && request.chain_id != 0,
        ConfidentialTokenError::RequestWitnessMismatch
    );
    let recomputed_hash = disclosure_request_hash(
        crate::ID,
        request_key,
        request.mint,
        request.requester,
        request.token_account,
        request.app_account,
        request.handle,
        request.encrypted_value,
        request.host_config,
        request.kms_context_id,
        request.request_nonce,
        request.chain_id,
        request.expires_slot,
        request.mode,
    );
    require!(
        request.request_hash == recomputed_hash,
        ConfidentialTokenError::RequestWitnessMismatch
    );
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn assert_burn_redemption_request_witness(
    request: &Account<BurnRedemptionRequest>,
    request_key: Pubkey,
    mint: Pubkey,
    owner: Pubkey,
    token_account: Pubkey,
    underlying_mint: Pubkey,
    destination_owner: Pubkey,
    destination_account: Pubkey,
    burned_handle: [u8; 32],
    burned_encrypted_value: Pubkey,
    host_config: Pubkey,
) -> Result<()> {
    let (expected_key, expected_bump) =
        burn_redemption_request_address(mint, owner, burned_handle, request.request_nonce);
    require_keys_eq!(
        request_key,
        expected_key,
        ConfidentialTokenError::RequestWitnessMismatch
    );
    require!(
        request.to_account_info().data_len() == 8 + BurnRedemptionRequest::SPACE
            && request.bump == expected_bump,
        ConfidentialTokenError::RequestWitnessMismatch
    );
    require!(
        request.status == REQUEST_STATUS_PENDING && request.expires_slot >= Clock::get()?.slot,
        ConfidentialTokenError::RequestWitnessUnavailable
    );
    require!(
        request.mint == mint
            && request.owner == owner
            && request.token_account == token_account
            && request.underlying_mint == underlying_mint
            && request.destination_owner == destination_owner
            && request.destination_account == destination_account
            && request.burned_handle == burned_handle
            && request.burned_encrypted_value == burned_encrypted_value
            && request.host_config == host_config
            && request.kms_context_id != 0
            && request.chain_id != 0,
        ConfidentialTokenError::RequestWitnessMismatch
    );
    let recomputed_hash = burn_redemption_request_hash(
        crate::ID,
        request_key,
        request.mint,
        request.owner,
        request.token_account,
        request.underlying_mint,
        request.destination_owner,
        request.destination_account,
        request.burned_handle,
        request.burned_encrypted_value,
        request.host_config,
        request.kms_context_id,
        request.request_nonce,
        request.chain_id,
        request.expires_slot,
    );
    require!(
        request.request_hash == recomputed_hash,
        ConfidentialTokenError::RequestWitnessMismatch
    );
    Ok(())
}

/// Verifies a KMS `PublicDecryptVerification` secp256k1 EIP-712 certificate against the
/// KMS context a request witness was pinned to at request time.
///
/// The context is resolved two ways and required to agree: the passed `kms_context` account
/// must be the canonical PDA for `request_kms_context_id` (the id stored in the witness), and
/// the id the certificate itself commits to via `extra_data` (EVM `_extractContextId` parity)
/// must equal that same id. Binding to the witness id — not the *current* context — is what
/// closes the rotation-reuse window: a cert minted under context N cannot satisfy a request
/// pinned to N, then be replayed against a rotated context, nor can a witness be steered to a
/// different context than the one it was created under.
pub(crate) fn assert_kms_public_decrypt_cert_for_request(
    host_config: &Account<zama_host::HostConfig>,
    kms_context: &Account<zama_host::KmsContext>,
    request_kms_context_id: u64,
    ct_handle: [u8; 32],
    cleartext_amount: u64,
    signatures: &[[u8; 65]],
    extra_data: &[u8],
) -> Result<()> {
    require!(
        host_config.decryption_contract != [0u8; 20] && request_kms_context_id != 0,
        ConfidentialTokenError::GatewayVerifierConfigUnset
    );
    require!(
        !kms_context.destroyed,
        ConfidentialTokenError::InvalidKmsContext
    );
    // The passed context account must be the canonical PDA for the witness-pinned id.
    require!(
        kms_context.context_id == request_kms_context_id
            && kms_context.key() == zama_host::kms_context_address(request_kms_context_id).0,
        ConfidentialTokenError::InvalidKmsContext
    );
    // The id the certificate commits to (via signed extra_data) must equal the witness id, so a
    // cert minted under a different context cannot be presented against this request.
    let cert_context_id =
        zama_host::eip712::extract_kms_context_id(extra_data, request_kms_context_id)
            .ok_or(ConfidentialTokenError::InvalidKmsContext)?;
    require!(
        cert_context_id == request_kms_context_id,
        ConfidentialTokenError::InvalidKmsContext
    );
    let verifier = zama_host::eip712::Eip712VerifierConfig {
        gateway_chain_id: host_config.gateway_chain_id,
        verifying_contract: host_config.decryption_contract,
        signers: &kms_context.signers,
        threshold: kms_context.thresholds.public_decryption,
    };
    require!(
        zama_host::eip712::verify_kms_public_decrypt(
            &verifier,
            &[ct_handle],
            &kms_decrypted_result_bytes(cleartext_amount),
            extra_data,
            signatures,
        ),
        ConfidentialTokenError::InvalidKmsCertificate
    );
    Ok(())
}

pub(crate) fn balance_acl_subjects(
    owner: Pubkey,
    compute_signer: Pubkey,
) -> Vec<zama_fhe::AccessSubject> {
    vec![
        zama_fhe::AccessSubject::owner(owner),
        zama_fhe::AccessSubject::compute(compute_signer),
    ]
}

pub(crate) fn burned_amount_acl_subjects(
    owner: Pubkey,
    compute_signer: Pubkey,
) -> Vec<zama_fhe::AccessSubject> {
    balance_acl_subjects(owner, compute_signer)
}
