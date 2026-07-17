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

/// Anchor-native mirror of `zama_solana_acl::MmrProof` for use as an instruction
/// argument. The shared ACL crate is deliberately Anchor-free (pure `borsh`), so
/// it cannot derive Anchor's IDL metadata; this local type carries the identical
/// wire shape and converts into the shared proof for verification.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct MmrInclusionProof {
    /// Index of the proven leaf within the lineage's MMR.
    pub leaf_index: u64,
    /// Authentication path from the leaf up to its mountain peak.
    pub siblings: Vec<[u8; 32]>,
}

impl From<MmrInclusionProof> for zama_solana_acl::MmrProof {
    fn from(proof: MmrInclusionProof) -> Self {
        zama_solana_acl::MmrProof {
            leaf_index: proof.leaf_index,
            siblings: proof.siblings,
        }
    }
}

/// Lineage checks shared by the request and redeem paths: burned-amount handle
/// type, canonical address, domain/app account, the burned-amount label, and
/// current membership for the owner and mint compute signer. Does NOT authorize
/// the specific handle: the redeem path adds an MMR public-decrypt proof, while
/// the request path binds the handle into the witness (its publicness is proven
/// at redeem, since the burn already made the handle public — DD-036 / Vector 2).
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
        amount_value.has_subject(owner),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_value.has_subject(compute_signer),
        ConfidentialTokenError::AmountAclMismatch
    );
    Ok(())
}

/// Redeem (consume) path: the burned handle need not be current. It is authorized
/// by an MMR public-decrypt proof against the lineage's current peaks, so a
/// redemption pinned to a handle stays valid after later burns supersede the
/// lineage — closing the fund-stranding window without a per-operation escrow.
/// Replay is still prevented by the per-handle `burn-redemption` marker PDA.
pub(crate) fn authorize_burned_amount_redeem(
    amount_value: &Account<zama_host::EncryptedValue>,
    encrypted_value_account: Pubkey,
    burned_handle: [u8; 32],
    proof: &zama_solana_acl::MmrProof,
    mint: Pubkey,
    token_account: Pubkey,
    owner: Pubkey,
    compute_signer: Pubkey,
) -> Result<()> {
    assert_burned_amount_lineage(
        amount_value,
        burned_handle,
        mint,
        token_account,
        owner,
        compute_signer,
    )?;
    zama_solana_acl::authorize_public(
        encrypted_value_account.to_bytes(),
        &amount_value.to_shared(),
        burned_handle,
        proof,
    )
    .map_err(|_| ConfidentialTokenError::PublicDecryptProofInvalid)?;
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
