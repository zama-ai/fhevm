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
    pub(crate) from_account: &'a mut Box<Account<'info, ConfidentialTokenAccount>>,
    pub(crate) to_account: &'a mut Box<Account<'info, ConfidentialTokenAccount>>,
    pub(crate) compute_signer: &'a UncheckedAccount<'info>,
    pub(crate) amount_compute_acl: &'a Account<'info, zama_host::AclRecord>,
    pub(crate) from_balance_value_acl: AccountInfo<'info>,
    pub(crate) transferred_amount_acl: AccountInfo<'info>,
    pub(crate) to_balance_value_acl: AccountInfo<'info>,
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
    pub(crate) transferred_handle: [u8; 32],
    pub(crate) transferred_acl_record: Pubkey,
    pub(crate) to_owner: Pubkey,
    pub(crate) to_token_account: Pubkey,
    pub(crate) old_to_handle: [u8; 32],
    pub(crate) new_to_handle: [u8; 32],
}

pub(crate) struct PrepareTransferCallbackAccounts<'a, 'info> {
    pub(crate) payer: &'a Signer<'info>,
    pub(crate) callback_authority: &'a UncheckedAccount<'info>,
    pub(crate) mint: &'a Account<'info, ConfidentialMint>,
    pub(crate) from_account: &'a Account<'info, ConfidentialTokenAccount>,
    pub(crate) to_account: &'a mut Box<Account<'info, ConfidentialTokenAccount>>,
    pub(crate) compute_signer: &'a UncheckedAccount<'info>,
    pub(crate) sent_amount_acl: &'a Account<'info, zama_host::AclRecord>,
    pub(crate) callback_success_acl: &'a Account<'info, zama_host::AclRecord>,
    pub(crate) hook_record: &'a Account<'info, TransferReceiverHookCall>,
    pub(crate) settlement_record: &'a mut Account<'info, TransferCallbackSettlement>,
    pub(crate) to_balance_value_acl: AccountInfo<'info>,
    pub(crate) refund_amount_acl: AccountInfo<'info>,
    pub(crate) zama_event_authority: &'a UncheckedAccount<'info>,
    pub(crate) zama_program: &'a Program<'info, ZamaHost>,
    pub(crate) host_config: &'a Account<'info, zama_host::HostConfig>,
    pub(crate) system_program: &'a Program<'info, System>,
}

pub(crate) struct PrepareTransferCallbackOutcome {
    pub(crate) mint: Pubkey,
    pub(crate) to_owner: Pubkey,
    pub(crate) to_token_account: Pubkey,
    pub(crate) old_to_handle: [u8; 32],
    pub(crate) new_to_handle: [u8; 32],
}

pub(crate) struct FinalizeTransferCallbackAccounts<'a, 'info> {
    pub(crate) payer: &'a Signer<'info>,
    pub(crate) mint: &'a Account<'info, ConfidentialMint>,
    pub(crate) from_account: &'a mut Box<Account<'info, ConfidentialTokenAccount>>,
    pub(crate) to_account: &'a Account<'info, ConfidentialTokenAccount>,
    pub(crate) compute_signer: &'a UncheckedAccount<'info>,
    pub(crate) sent_amount_acl: &'a Account<'info, zama_host::AclRecord>,
    pub(crate) settlement_record: &'a mut Account<'info, TransferCallbackSettlement>,
    pub(crate) refund_amount_acl: &'a Account<'info, zama_host::AclRecord>,
    pub(crate) from_balance_value_acl: AccountInfo<'info>,
    pub(crate) transferred_amount_acl: AccountInfo<'info>,
    pub(crate) zama_event_authority: &'a UncheckedAccount<'info>,
    pub(crate) zama_program: &'a Program<'info, ZamaHost>,
    pub(crate) host_config: &'a Account<'info, zama_host::HostConfig>,
    pub(crate) system_program: &'a Program<'info, System>,
}

pub(crate) struct FinalizeTransferCallbackOutcome {
    pub(crate) mint: Pubkey,
    pub(crate) from_owner: Pubkey,
    pub(crate) from_token_account: Pubkey,
    pub(crate) old_from_handle: [u8; 32],
    pub(crate) new_from_handle: [u8; 32],
    pub(crate) to_owner: Pubkey,
    pub(crate) to_token_account: Pubkey,
    pub(crate) refund_handle: [u8; 32],
    pub(crate) refund_acl_record: Pubkey,
}

pub(crate) fn execute_transfer<'info>(
    accounts: TransferAccounts<'_, 'info>,
    compute_signer_bump: u8,
    amount_handle: [u8; 32],
) -> Result<Option<TransferOutcome>> {
    assert_confidential_mint_shape(accounts.mint)?;
    let mint_key = accounts.mint.key();
    let compute_signer = accounts.mint.compute_signer;
    let from = accounts.from_account.as_ref();
    let to = accounts.to_account.as_ref();
    let amount_nonce_sequence = from.next_amount_nonce_sequence;
    let old_from_handle = from.balance_handle;
    let old_to_handle = to.balance_handle;

    assert_transfer_amount_acl(
        accounts.amount_compute_acl,
        amount_handle,
        mint_key,
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
    if from.key() == to.key() {
        assert_self_transfer_output_accounts(
            &accounts,
            mint_key,
            from.key(),
            amount_nonce_sequence,
        )?;
        return Ok(None);
    }

    let (new_from_handle, transferred_handle, new_to_handle) = execute_transfer_eval(
        &accounts,
        compute_signer_bump,
        amount_handle,
        mint_key,
        amount_nonce_sequence,
        from,
        to,
    )?;

    let from_owner = from.owner;
    let to_owner = to.owner;
    let from_token_account = from.key();
    let to_token_account = to.key();
    let zama_program = accounts.zama_program.to_account_info();
    let payer = accounts.payer.to_account_info();
    let system_program = accounts.system_program.to_account_info();
    upsert_value_acl(
        &LineageCpi {
            zama_program: zama_program.clone(),
            encrypted_value_acl: accounts.from_balance_value_acl.clone(),
            payer: payer.clone(),
            system_program: system_program.clone(),
        },
        LineageAuthority::balance(from),
        mint_key,
        new_from_handle,
        vec![from_owner, compute_signer],
    )?;
    upsert_value_acl(
        &LineageCpi {
            zama_program,
            encrypted_value_acl: accounts.to_balance_value_acl.clone(),
            payer,
            system_program,
        },
        LineageAuthority::balance(to),
        mint_key,
        new_to_handle,
        vec![to_owner, compute_signer],
    )?;

    let from = accounts.from_account.as_mut();
    from.balance_handle = new_from_handle;
    from.next_amount_nonce_sequence = amount_nonce_sequence
        .checked_add(1)
        .ok_or(ConfidentialTokenError::AclNonceOverflow)?;

    let to = accounts.to_account.as_mut();
    to.balance_handle = new_to_handle;
    Ok(Some(TransferOutcome {
        mint: mint_key,
        from_owner,
        from_token_account,
        old_from_handle,
        new_from_handle,
        transferred_handle,
        transferred_acl_record: accounts.transferred_amount_acl.key(),
        to_owner,
        to_token_account,
        old_to_handle,
        new_to_handle,
    }))
}

fn execute_transfer_eval<'info>(
    accounts: &TransferAccounts<'_, 'info>,
    compute_signer_bump: u8,
    amount_handle: [u8; 32],
    mint_key: Pubkey,
    amount_nonce_sequence: u64,
    from: &Account<'info, ConfidentialTokenAccount>,
    to: &Account<'info, ConfidentialTokenAccount>,
) -> Result<([u8; 32], [u8; 32], [u8; 32])> {
    let context_id = transfer_eval_context(
        b"combined",
        mint_key,
        from.key(),
        to.key(),
        amount_handle,
        amount_nonce_sequence,
        amount_nonce_sequence,
    )?;
    let from_balance = zama_fhe::Uint64Handle::durable_at(
        from.balance_handle,
        accounts.from_balance_value_acl.key(),
    )
    .map_err(invalid_eval_plan)?;
    let amount = uint64_from_acl(amount_handle, accounts.amount_compute_acl)?;
    let to_balance =
        zama_fhe::Uint64Handle::durable_at(to.balance_handle, accounts.to_balance_value_acl.key())
            .map_err(invalid_eval_plan)?;
    let compute_signer = accounts.compute_signer.key();
    let transferred_access = {
        let mut access =
            zama_fhe::AccessPolicy::for_owner(from.owner).map_err(invalid_eval_plan)?;
        if to.owner != from.owner {
            access = access.with_owner(to.owner).map_err(invalid_eval_plan)?;
        }
        access
            .with_compute(compute_signer)
            .map_err(invalid_eval_plan)?
    };
    let transferred_output = fhe::DurableOutput::new(
        accounts.transferred_amount_acl.clone(),
        durable_slot(
            mint_key,
            from.key(),
            transferred_amount_label(),
            amount_nonce_sequence,
        ),
        transferred_access,
    )?;
    let mut builder =
        zama_fhe::EvalBuilder::new(context_id, zama_fhe::EvalAppAuthority::new(from.key()));
    let success = builder
        .ge(from_balance, amount, zama_fhe::Output::transient())
        .map_err(invalid_eval_plan)?;
    let debit_candidate = builder
        .sub(from_balance, amount, zama_fhe::Output::transient())
        .map_err(invalid_eval_plan)?;
    let new_from = builder
        .if_then_else(
            success,
            debit_candidate,
            from_balance,
            zama_fhe::Output::transient(),
        )
        .map_err(invalid_eval_plan)?;
    let new_from_index = new_from
        .producer_index()
        .ok_or(error!(ConfidentialTokenError::InvalidFheEvalPlan))?;
    let transferred = builder
        .sub(from_balance, new_from, transferred_output.output())
        .map_err(invalid_eval_plan)?;
    let new_to = builder
        .add(to_balance, transferred, zama_fhe::Output::transient())
        .map_err(invalid_eval_plan)?;
    let new_to_index = new_to
        .producer_index()
        .ok_or(error!(ConfidentialTokenError::InvalidFheEvalPlan))?;
    let plan = builder.finish().map_err(invalid_eval_plan)?;
    let compute_authority =
        fhe::ComputeAuthority::for_mint(accounts.compute_signer, mint_key, compute_signer_bump)?;
    let eval_accounts = fhe::EvalAccountSet::for_plan(
        &plan,
        [
            accounts.from_balance_value_acl.clone(),
            accounts.amount_compute_acl.to_account_info(),
            accounts.to_balance_value_acl.clone(),
            transferred_output.account_info(),
        ],
        [fhe::OutputAuthority::token_account(from)?],
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
        fhe::read_eval_output_handle(new_from_index)?,
        transferred_output.handle()?,
        fhe::read_eval_output_handle(new_to_index)?,
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
    nonce_sequence: u64,
) -> zama_fhe::DurableSlot {
    zama_fhe::DurableSlot::new(
        acl_domain_key,
        app_account,
        zama_fhe::DurableLabel::new(encrypted_value_label),
        nonce_sequence,
    )
}

pub(crate) fn durable_slot_from_acl(acl: &zama_host::AclRecord) -> zama_fhe::DurableSlot {
    durable_slot(
        acl.acl_domain_key,
        acl.app_account,
        acl.encrypted_value_label,
        acl.nonce_sequence,
    )
}

pub(crate) fn uint64_from_acl(
    handle: [u8; 32],
    acl: &zama_host::AclRecord,
) -> Result<zama_fhe::Uint64Handle> {
    zama_fhe::Uint64Handle::durable(handle, durable_slot_from_acl(acl)).map_err(invalid_eval_plan)
}

pub(crate) fn bool_from_acl(
    handle: [u8; 32],
    acl: &zama_host::AclRecord,
) -> Result<zama_fhe::BoolHandle> {
    zama_fhe::BoolHandle::durable(handle, durable_slot_from_acl(acl)).map_err(invalid_eval_plan)
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
    from_nonce_sequence: u64,
    to_nonce_sequence: u64,
) -> Result<zama_fhe::EvalContextId> {
    let from_sequence_bytes = from_nonce_sequence.to_be_bytes();
    let to_sequence_bytes = to_nonce_sequence.to_be_bytes();
    let context_id = solana_sha256_hasher::hashv(&[
        b"confidential-token-transfer-eval-v1",
        tag,
        mint.as_ref(),
        from_token_account.as_ref(),
        to_token_account.as_ref(),
        &amount_handle,
        &from_sequence_bytes,
        &to_sequence_bytes,
    ])
    .to_bytes();
    zama_fhe::EvalContextId::new(context_id).map_err(invalid_eval_plan)
}

pub(crate) fn assert_self_transfer_output_accounts(
    accounts: &TransferAccounts<'_, '_>,
    mint: Pubkey,
    token_account: Pubkey,
    nonce_sequence: u64,
) -> Result<()> {
    // Balances are encrypted-value ACL lineages (no fresh record minted), so a
    // self-transfer no-op only needs to prove the one-shot transferred-amount
    // record is still unused.
    assert_unused_acl_target(
        &accounts.transferred_amount_acl,
        acl_record_address_for(
            mint,
            token_account,
            transferred_amount_label(),
            nonce_sequence,
        ),
    )?;
    Ok(())
}

pub(crate) fn assert_unused_acl_target(account: &AccountInfo, expected_key: Pubkey) -> Result<()> {
    require_keys_eq!(
        account.key(),
        expected_key,
        ConfidentialTokenError::AmountAclMismatch
    );
    require_keys_eq!(
        *account.owner,
        System::id(),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        account.data_is_empty(),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        !account.executable,
        ConfidentialTokenError::AmountAclMismatch
    );
    Ok(())
}

pub(crate) fn acl_record_address_for(
    mint: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    nonce_sequence: u64,
) -> Pubkey {
    zama_host::acl_record_address(
        nonce_key(mint, app_account, encrypted_value_label),
        nonce_sequence,
    )
    .0
}

pub(crate) fn prepare_transfer_callback_settlement<'info>(
    mut accounts: PrepareTransferCallbackAccounts<'_, 'info>,
    compute_signer_bump: u8,
    settlement_bump: u8,
    sent_handle: [u8; 32],
    callback_success_handle: [u8; 32],
) -> Result<PrepareTransferCallbackOutcome> {
    assert_confidential_mint_shape(accounts.mint)?;
    let mint_key = accounts.mint.key();
    let compute_signer = accounts.mint.compute_signer;
    let from = accounts.from_account;
    let to = accounts.to_account.as_ref();
    let from_token_account = from.key();
    let to_token_account = to.key();
    let from_owner = from.owner;
    let to_owner = to.owner;
    let amount_nonce_sequence = to.next_amount_nonce_sequence;
    let old_to_handle = to.balance_handle;
    let amount_subjects = transferred_amount_acl_subjects(from_owner, to_owner, compute_signer);

    require!(
        from_token_account != to_token_account,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require_keys_eq!(from.mint, mint_key, ConfidentialTokenError::MintMismatch);
    require_keys_eq!(to.mint, mint_key, ConfidentialTokenError::MintMismatch);
    assert_confidential_token_account_shape(from, mint_key, from_owner)?;
    assert_confidential_token_account_shape(to, mint_key, to_owner)?;
    require_keys_eq!(
        accounts.callback_authority.key(),
        to_owner,
        ConfidentialTokenError::OwnerMismatch
    );
    require_keys_eq!(
        accounts.compute_signer.key(),
        compute_signer,
        ConfidentialTokenError::ComputeSignerMismatch
    );
    assert_transferred_amount_acl(
        accounts.sent_amount_acl,
        sent_handle,
        mint_key,
        from_token_account,
        from_owner,
        to_owner,
        compute_signer,
    )?;
    assert_callback_success_acl(
        accounts.callback_success_acl,
        callback_success_handle,
        mint_key,
        to_owner,
        compute_signer,
    )?;
    assert_transfer_receiver_hook_call_shape(
        accounts.hook_record,
        mint_key,
        from_token_account,
        to_token_account,
        sent_handle,
        accounts.sent_amount_acl.key(),
        callback_success_handle,
        accounts.callback_success_acl.key(),
    )?;

    let to_balance =
        zama_fhe::Uint64Handle::durable_at(old_to_handle, accounts.to_balance_value_acl.key())
            .map_err(invalid_eval_plan)?;
    let sent_amount = uint64_from_acl(sent_handle, accounts.sent_amount_acl)?;
    let callback_success = bool_from_acl(callback_success_handle, accounts.callback_success_acl)?;
    let context_id = transfer_eval_context(
        b"callback-prepare",
        mint_key,
        from_token_account,
        to_token_account,
        sent_handle,
        amount_nonce_sequence,
        amount_nonce_sequence,
    )?;
    let refund_amount_output = fhe::DurableOutput::new(
        accounts.refund_amount_acl.clone(),
        durable_slot(
            mint_key,
            to_token_account,
            callback_refund_amount_label(),
            amount_nonce_sequence,
        ),
        access_policy_from_subjects(amount_subjects.clone())?,
    )?;
    let mut builder = zama_fhe::EvalBuilder::new(
        context_id,
        zama_fhe::EvalAppAuthority::new(to_token_account),
    );
    let zero = builder
        .trivial_encrypt_u64(0, zama_fhe::Output::transient())
        .map_err(invalid_eval_plan)?;
    let requested_refund = builder
        .if_then_else(
            callback_success,
            zero,
            sent_amount,
            zama_fhe::Output::transient(),
        )
        .map_err(invalid_eval_plan)?;
    let refund_success = builder
        .ge(to_balance, requested_refund, zama_fhe::Output::transient())
        .map_err(invalid_eval_plan)?;
    let refund_debit_candidate = builder
        .sub(to_balance, requested_refund, zama_fhe::Output::transient())
        .map_err(invalid_eval_plan)?;
    let new_to = builder
        .if_then_else(
            refund_success,
            refund_debit_candidate,
            to_balance,
            zama_fhe::Output::transient(),
        )
        .map_err(invalid_eval_plan)?;
    let new_to_index = new_to
        .producer_index()
        .ok_or(error!(ConfidentialTokenError::InvalidFheEvalPlan))?;
    builder
        .sub(to_balance, new_to, refund_amount_output.output())
        .map_err(invalid_eval_plan)?;
    let plan = builder.finish().map_err(invalid_eval_plan)?;
    let compute_authority =
        fhe::ComputeAuthority::for_mint(accounts.compute_signer, mint_key, compute_signer_bump)?;
    let eval_accounts = fhe::EvalAccountSet::for_plan(
        &plan,
        [
            accounts.to_balance_value_acl.clone(),
            accounts.sent_amount_acl.to_account_info(),
            accounts.callback_success_acl.to_account_info(),
            refund_amount_output.account_info(),
        ],
        [fhe::OutputAuthority::token_account(accounts.to_account)?],
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

    let new_to_handle = fhe::read_eval_output_handle(new_to_index)?;
    let refund_handle = refund_amount_output.handle()?;
    let refund_acl_record = accounts.refund_amount_acl.key();

    upsert_value_acl(
        &LineageCpi {
            zama_program: accounts.zama_program.to_account_info(),
            encrypted_value_acl: accounts.to_balance_value_acl.clone(),
            payer: accounts.payer.to_account_info(),
            system_program: accounts.system_program.to_account_info(),
        },
        LineageAuthority::balance(accounts.to_account.as_ref()),
        mint_key,
        new_to_handle,
        vec![to_owner, compute_signer],
    )?;

    let to = accounts.to_account.as_mut();
    to.balance_handle = new_to_handle;
    to.next_amount_nonce_sequence = amount_nonce_sequence
        .checked_add(1)
        .ok_or(ConfidentialTokenError::AclNonceOverflow)?;

    let settlement = &mut accounts.settlement_record;
    settlement.mint = mint_key;
    settlement.from_owner = from_owner;
    settlement.from_token_account = from_token_account;
    settlement.to_owner = to_owner;
    settlement.to_token_account = to_token_account;
    settlement.sent_handle = sent_handle;
    settlement.sent_acl_record = accounts.sent_amount_acl.key();
    settlement.callback_success_handle = callback_success_handle;
    settlement.callback_success_acl_record = accounts.callback_success_acl.key();
    settlement.refund_handle = refund_handle;
    settlement.refund_acl_record = refund_acl_record;
    settlement.to_balance_handle = new_to_handle;
    settlement.to_balance_acl_record = Pubkey::default();
    settlement.from_balance_handle = [0; 32];
    settlement.from_balance_acl_record = Pubkey::default();
    settlement.transferred_handle = [0; 32];
    settlement.transferred_acl_record = Pubkey::default();
    settlement.status = CALLBACK_SETTLEMENT_PREPARED;
    settlement.bump = settlement_bump;

    Ok(PrepareTransferCallbackOutcome {
        mint: mint_key,
        to_owner,
        to_token_account,
        old_to_handle,
        new_to_handle,
    })
}

pub(crate) fn call_transfer_receiver_hook<'info>(
    mint: &Account<'info, ConfidentialMint>,
    from_account: &Account<'info, ConfidentialTokenAccount>,
    to_account: &Account<'info, ConfidentialTokenAccount>,
    previous_transfer_intent: PreviousTransferIntent,
    compute_signer_account: &UncheckedAccount<'info>,
    sent_amount_acl: &Account<'info, zama_host::AclRecord>,
    callback_success_acl: &Account<'info, zama_host::AclRecord>,
    receiver_program_account: &UncheckedAccount<'info>,
    instructions_sysvar: &AccountInfo<'info>,
    remaining_accounts: &[AccountInfo<'info>],
    sent_handle: [u8; 32],
    callback_success_handle: [u8; 32],
    receiver_instruction_data: Vec<u8>,
) -> Result<()> {
    assert_confidential_mint_shape(mint)?;
    let mint_key = mint.key();
    let compute_signer = mint.compute_signer;
    let from = from_account;
    let to = to_account;
    let from_token_account = from.key();
    let to_token_account = to.key();
    let receiver_program = receiver_program_account.key();

    require!(
        receiver_instruction_data.len() <= MAX_RECEIVER_HOOK_DATA_LEN
            && remaining_accounts.len() <= MAX_RECEIVER_HOOK_ACCOUNTS,
        ConfidentialTokenError::ReceiverHookInputTooLarge
    );
    require!(
        receiver_program_account.executable,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    require!(
        from_token_account != to_token_account,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    require_keys_eq!(from.mint, mint_key, ConfidentialTokenError::MintMismatch);
    require_keys_eq!(to.mint, mint_key, ConfidentialTokenError::MintMismatch);
    assert_confidential_token_account_shape(from, mint_key, from.owner)?;
    assert_confidential_token_account_shape(to, mint_key, to.owner)?;
    assert_previous_transfer_for_receiver_hook(
        instructions_sysvar,
        previous_transfer_intent,
        mint_key,
        from_token_account,
        to_token_account,
        sent_amount_acl.key(),
    )?;
    require_keys_eq!(
        compute_signer_account.key(),
        compute_signer,
        ConfidentialTokenError::ComputeSignerMismatch
    );
    assert_transferred_amount_acl(
        sent_amount_acl,
        sent_handle,
        mint_key,
        from_token_account,
        from.owner,
        to.owner,
        compute_signer,
    )?;
    assert_callback_success_acl(
        callback_success_acl,
        callback_success_handle,
        mint_key,
        to.owner,
        compute_signer,
    )?;

    let metas = remaining_accounts
        .iter()
        .map(|account| {
            if account.is_writable {
                AccountMeta::new(*account.key, account.is_signer)
            } else {
                AccountMeta::new_readonly(*account.key, account.is_signer)
            }
        })
        .collect();
    set_return_data(&[]);
    invoke(
        &Instruction {
            program_id: receiver_program,
            accounts: metas,
            data: receiver_instruction_data,
        },
        remaining_accounts,
    )?;

    let Some((return_program, return_data)) = get_return_data() else {
        return err!(ConfidentialTokenError::ReceiverHookMismatch);
    };
    require_keys_eq!(
        return_program,
        receiver_program,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    let returned = TransferReceiverReturn::decode(&return_data)
        .map_err(|_| error!(ConfidentialTokenError::ReceiverHookMismatch))?;
    require_keys_eq!(
        returned.mint,
        mint_key,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    require_keys_eq!(
        returned.from_token_account,
        from_token_account,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    require_keys_eq!(
        returned.to_token_account,
        to_token_account,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    require!(
        returned.sent_handle == sent_handle,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    require_keys_eq!(
        returned.sent_acl_record,
        sent_amount_acl.key(),
        ConfidentialTokenError::ReceiverHookMismatch
    );
    require!(
        returned.callback_success_handle == callback_success_handle,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    require_keys_eq!(
        returned.callback_success_acl_record,
        callback_success_acl.key(),
        ConfidentialTokenError::ReceiverHookMismatch
    );

    Ok(())
}

pub(crate) fn assert_previous_transfer_for_receiver_hook(
    instructions_sysvar: &AccountInfo,
    intent: PreviousTransferIntent,
    mint: Pubkey,
    from_token_account: Pubkey,
    to_token_account: Pubkey,
    sent_acl_record: Pubkey,
) -> Result<()> {
    require_keys_eq!(
        instructions_sysvar.key(),
        INSTRUCTIONS_SYSVAR_ID,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    let current_index = load_current_index_checked(instructions_sysvar)
        .map_err(|_| error!(ConfidentialTokenError::ReceiverHookMismatch))?;
    let transfer_index = current_index
        .checked_sub(1)
        .ok_or(ConfidentialTokenError::ReceiverHookMismatch)?;
    let transfer_ix = load_instruction_at_checked(transfer_index as usize, instructions_sysvar)
        .map_err(|_| error!(ConfidentialTokenError::ReceiverHookMismatch))?;
    require_keys_eq!(
        transfer_ix.program_id,
        crate::ID,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    require!(
        transfer_ix.data.len() >= 8,
        ConfidentialTokenError::ReceiverHookMismatch
    );

    let discriminator = &transfer_ix.data[..8];
    match (discriminator, intent) {
        (discriminator, PreviousTransferIntent::Direct { owner })
            if discriminator == crate::instruction::ConfidentialTransfer::DISCRIMINATOR =>
        {
            assert_previous_transfer_accounts(
                &transfer_ix.accounts,
                PreviousTransferAccountIndexes {
                    authority: ConfidentialTransfer::OWNER_ACCOUNT_INDEX,
                    mint: ConfidentialTransfer::MINT_ACCOUNT_INDEX,
                    from_token_account: ConfidentialTransfer::FROM_ACCOUNT_INDEX,
                    to_token_account: ConfidentialTransfer::TO_ACCOUNT_INDEX,
                    sent_acl_record: ConfidentialTransfer::TRANSFERRED_AMOUNT_ACL_INDEX,
                },
                owner,
                mint,
                from_token_account,
                to_token_account,
                sent_acl_record,
            )
        }
        _ => err!(ConfidentialTokenError::ReceiverHookMismatch),
    }
}

pub(crate) enum PreviousTransferIntent {
    Direct { owner: Pubkey },
}

struct PreviousTransferAccountIndexes {
    authority: usize,
    mint: usize,
    from_token_account: usize,
    to_token_account: usize,
    sent_acl_record: usize,
}

fn assert_previous_transfer_accounts(
    accounts: &[AccountMeta],
    indexes: PreviousTransferAccountIndexes,
    authority: Pubkey,
    mint: Pubkey,
    from_token_account: Pubkey,
    to_token_account: Pubkey,
    sent_acl_record: Pubkey,
) -> Result<()> {
    let authority_meta = accounts
        .get(indexes.authority)
        .ok_or(ConfidentialTokenError::ReceiverHookMismatch)?;
    let mint_meta = accounts
        .get(indexes.mint)
        .ok_or(ConfidentialTokenError::ReceiverHookMismatch)?;
    let from_meta = accounts
        .get(indexes.from_token_account)
        .ok_or(ConfidentialTokenError::ReceiverHookMismatch)?;
    let to_meta = accounts
        .get(indexes.to_token_account)
        .ok_or(ConfidentialTokenError::ReceiverHookMismatch)?;
    let sent_meta = accounts
        .get(indexes.sent_acl_record)
        .ok_or(ConfidentialTokenError::ReceiverHookMismatch)?;
    require_keys_eq!(
        authority_meta.pubkey,
        authority,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    require!(
        authority_meta.is_signer,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    require_keys_eq!(
        mint_meta.pubkey,
        mint,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    require!(
        !mint_meta.is_signer && !mint_meta.is_writable,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    require_keys_eq!(
        from_meta.pubkey,
        from_token_account,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    require!(
        !from_meta.is_signer && from_meta.is_writable,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    require_keys_eq!(
        to_meta.pubkey,
        to_token_account,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    require!(
        !to_meta.is_signer && to_meta.is_writable,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    require_keys_eq!(
        sent_meta.pubkey,
        sent_acl_record,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    require!(
        !sent_meta.is_signer && sent_meta.is_writable,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    Ok(())
}

pub(crate) fn write_transfer_receiver_hook_call(
    hook_record: &mut Account<TransferReceiverHookCall>,
    mint: Pubkey,
    from_token_account: Pubkey,
    to_token_account: Pubkey,
    sent_handle: [u8; 32],
    sent_acl_record: Pubkey,
    callback_success_handle: [u8; 32],
    callback_success_acl_record: Pubkey,
    receiver_program: Pubkey,
    caller: Pubkey,
    bump: u8,
) {
    hook_record.mint = mint;
    hook_record.from_token_account = from_token_account;
    hook_record.to_token_account = to_token_account;
    hook_record.sent_handle = sent_handle;
    hook_record.sent_acl_record = sent_acl_record;
    hook_record.callback_success_handle = callback_success_handle;
    hook_record.callback_success_acl_record = callback_success_acl_record;
    hook_record.receiver_program = receiver_program;
    hook_record.caller = caller;
    hook_record.bump = bump;
}

pub(crate) fn finalize_transfer_callback_settlement<'info>(
    mut accounts: FinalizeTransferCallbackAccounts<'_, 'info>,
    compute_signer_bump: u8,
) -> Result<FinalizeTransferCallbackOutcome> {
    assert_confidential_mint_shape(accounts.mint)?;
    let mint_key = accounts.mint.key();
    let compute_signer = accounts.mint.compute_signer;
    let from = accounts.from_account.as_ref();
    let to = accounts.to_account;
    let amount_nonce_sequence = from.next_amount_nonce_sequence;
    let old_from_handle = from.balance_handle;
    let from_owner = accounts.settlement_record.from_owner;
    let to_owner = accounts.settlement_record.to_owner;
    let from_token_account = accounts.settlement_record.from_token_account;
    let to_token_account = accounts.settlement_record.to_token_account;
    let sent_handle = accounts.settlement_record.sent_handle;
    let refund_handle = accounts.settlement_record.refund_handle;
    let refund_acl_record = accounts.settlement_record.refund_acl_record;
    let amount_subjects = transferred_amount_acl_subjects(from_owner, to_owner, compute_signer);

    assert_transfer_callback_settlement_shape(accounts.settlement_record, mint_key, sent_handle)?;
    require!(
        accounts.settlement_record.status == CALLBACK_SETTLEMENT_PREPARED,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require_keys_eq!(
        accounts.settlement_record.mint,
        mint_key,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require_keys_eq!(
        from.key(),
        from_token_account,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require_keys_eq!(
        to.key(),
        to_token_account,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require_keys_eq!(
        from.owner,
        from_owner,
        ConfidentialTokenError::OwnerMismatch
    );
    require_keys_eq!(to.owner, to_owner, ConfidentialTokenError::OwnerMismatch);
    require_keys_eq!(from.mint, mint_key, ConfidentialTokenError::MintMismatch);
    require_keys_eq!(to.mint, mint_key, ConfidentialTokenError::MintMismatch);
    assert_confidential_token_account_shape(from, mint_key, from_owner)?;
    assert_confidential_token_account_shape(to, mint_key, to_owner)?;
    require_keys_eq!(
        accounts.compute_signer.key(),
        compute_signer,
        ConfidentialTokenError::ComputeSignerMismatch
    );
    // Intentionally do NOT require the recipient's live balance to still equal
    // the prepare-time snapshot (`settlement_record.to_balance_handle` /
    // `to_balance_acl_record`). Finalize credits the *sender* from the durable
    // `refund_handle` recorded at prepare time and never reads the recipient's
    // balance for the credit math. Requiring an unchanged recipient balance here
    // would let any ordinary recipient balance op between prepare and finalize
    // permanently strand the refund (the recipient keeps the over-debit and the
    // sender is never credited). Because finalize is permissionless and the
    // refund is snapshotted durably, dropping the snapshot-equality guard makes
    // the credit always recoverable; the `status == PREPARED -> FINALIZED` flip
    // still prevents double finalize. The recipient token account is still bound
    // by the key/owner/mint/shape checks above. See DD-018.
    require_keys_eq!(
        accounts.sent_amount_acl.key(),
        accounts.settlement_record.sent_acl_record,
        ConfidentialTokenError::AmountAclMismatch
    );
    assert_transferred_amount_acl(
        accounts.sent_amount_acl,
        sent_handle,
        mint_key,
        from_token_account,
        from_owner,
        to_owner,
        compute_signer,
    )?;
    assert_callback_refund_acl(
        accounts.refund_amount_acl,
        refund_handle,
        mint_key,
        to_token_account,
        from_owner,
        to_owner,
        compute_signer,
    )?;
    require_keys_eq!(
        accounts.refund_amount_acl.key(),
        accounts.settlement_record.refund_acl_record,
        ConfidentialTokenError::AmountAclMismatch
    );

    let from_balance =
        zama_fhe::Uint64Handle::durable_at(old_from_handle, accounts.from_balance_value_acl.key())
            .map_err(invalid_eval_plan)?;
    let sent_amount = uint64_from_acl(sent_handle, accounts.sent_amount_acl)?;
    let refund_amount = uint64_from_acl(refund_handle, accounts.refund_amount_acl)?;
    let context_id = transfer_eval_context(
        b"callback-finalize",
        mint_key,
        from_token_account,
        to_token_account,
        sent_handle,
        amount_nonce_sequence,
        amount_nonce_sequence,
    )?;
    let transferred_output = fhe::DurableOutput::new(
        accounts.transferred_amount_acl.clone(),
        durable_slot(
            mint_key,
            from_token_account,
            callback_final_transferred_label(),
            amount_nonce_sequence,
        ),
        access_policy_from_subjects(amount_subjects)?,
    )?;
    let mut builder = zama_fhe::EvalBuilder::new(
        context_id,
        zama_fhe::EvalAppAuthority::new(from_token_account),
    );
    let new_from = builder
        .add(from_balance, refund_amount, zama_fhe::Output::transient())
        .map_err(invalid_eval_plan)?;
    let new_from_index = new_from
        .producer_index()
        .ok_or(error!(ConfidentialTokenError::InvalidFheEvalPlan))?;
    builder
        .sub(sent_amount, refund_amount, transferred_output.output())
        .map_err(invalid_eval_plan)?;
    let plan = builder.finish().map_err(invalid_eval_plan)?;
    let compute_authority =
        fhe::ComputeAuthority::for_mint(accounts.compute_signer, mint_key, compute_signer_bump)?;
    let eval_accounts = fhe::EvalAccountSet::for_plan(
        &plan,
        [
            accounts.from_balance_value_acl.clone(),
            accounts.sent_amount_acl.to_account_info(),
            accounts.refund_amount_acl.to_account_info(),
            transferred_output.account_info(),
        ],
        [fhe::OutputAuthority::token_account(accounts.from_account)?],
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

    let new_from_handle = fhe::read_eval_output_handle(new_from_index)?;
    let final_transferred_handle = transferred_output.handle()?;
    let transferred_acl_record = accounts.transferred_amount_acl.key();

    upsert_value_acl(
        &LineageCpi {
            zama_program: accounts.zama_program.to_account_info(),
            encrypted_value_acl: accounts.from_balance_value_acl.clone(),
            payer: accounts.payer.to_account_info(),
            system_program: accounts.system_program.to_account_info(),
        },
        LineageAuthority::balance(accounts.from_account.as_ref()),
        mint_key,
        new_from_handle,
        vec![from_owner, compute_signer],
    )?;

    let from = accounts.from_account.as_mut();
    from.balance_handle = new_from_handle;
    from.next_amount_nonce_sequence = amount_nonce_sequence
        .checked_add(1)
        .ok_or(ConfidentialTokenError::AclNonceOverflow)?;

    let settlement = &mut accounts.settlement_record;
    settlement.from_balance_handle = new_from_handle;
    settlement.from_balance_acl_record = Pubkey::default();
    settlement.transferred_handle = final_transferred_handle;
    settlement.transferred_acl_record = transferred_acl_record;
    settlement.status = CALLBACK_SETTLEMENT_FINALIZED;

    Ok(FinalizeTransferCallbackOutcome {
        mint: mint_key,
        from_owner,
        from_token_account,
        old_from_handle,
        new_from_handle,
        to_owner,
        to_token_account,
        refund_handle,
        refund_acl_record,
    })
}

pub(crate) fn assert_transfer_callback_settlement_shape(
    settlement_record: &Account<TransferCallbackSettlement>,
    mint: Pubkey,
    sent_handle: [u8; 32],
) -> Result<()> {
    let (expected_key, expected_bump) = transfer_callback_settlement_address(mint, sent_handle);
    require_keys_eq!(
        settlement_record.key(),
        expected_key,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require!(
        settlement_record.to_account_info().data_len() == 8 + TransferCallbackSettlement::SPACE,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require!(
        settlement_record.bump == expected_bump,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    Ok(())
}

pub(crate) fn assert_transfer_receiver_hook_call_shape(
    hook_record: &Account<TransferReceiverHookCall>,
    mint: Pubkey,
    from_token_account: Pubkey,
    to_token_account: Pubkey,
    sent_handle: [u8; 32],
    sent_acl_record: Pubkey,
    callback_success_handle: [u8; 32],
    callback_success_acl_record: Pubkey,
) -> Result<()> {
    let (expected_key, expected_bump) = transfer_receiver_hook_address(mint, sent_handle);
    require_keys_eq!(
        hook_record.key(),
        expected_key,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require!(
        hook_record.to_account_info().data_len() == 8 + TransferReceiverHookCall::SPACE,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require!(
        hook_record.bump == expected_bump,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require_keys_eq!(
        hook_record.mint,
        mint,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require_keys_eq!(
        hook_record.from_token_account,
        from_token_account,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require_keys_eq!(
        hook_record.to_token_account,
        to_token_account,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require!(
        hook_record.sent_handle == sent_handle,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require_keys_eq!(
        hook_record.sent_acl_record,
        sent_acl_record,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require!(
        hook_record.callback_success_handle == callback_success_handle,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    require_keys_eq!(
        hook_record.callback_success_acl_record,
        callback_success_acl_record,
        ConfidentialTokenError::CallbackSettlementMismatch
    );
    Ok(())
}

pub(crate) fn assert_transfer_amount_acl(
    amount_acl: &Account<zama_host::AclRecord>,
    amount_handle: [u8; 32],
    mint: Pubkey,
    transfer_authority: Pubkey,
    compute_signer: Pubkey,
) -> Result<()> {
    assert_amount_acl_record_shape(amount_acl)?;
    require!(
        zama_host::handle_fhe_type(amount_handle) == BALANCE_FHE_TYPE,
        ConfidentialTokenError::AmountHandleTypeMismatch
    );
    require!(
        amount_acl.handle == amount_handle,
        ConfidentialTokenError::AmountAclMismatch
    );
    require_keys_eq!(
        amount_acl.acl_domain_key,
        mint,
        ConfidentialTokenError::AmountAclMismatch
    );
    require_keys_eq!(
        amount_acl.app_account,
        transfer_authority,
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.encrypted_value_label == transfer_amount_label(),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.nonce_key == nonce_key(mint, amount_acl.app_account, transfer_amount_label()),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.inline_subject_has_role(compute_signer, zama_host::ACL_ROLE_USE),
        ConfidentialTokenError::AmountAclMismatch
    );
    Ok(())
}

pub(crate) fn assert_burn_amount_acl(
    amount_acl: &Account<zama_host::AclRecord>,
    amount_handle: [u8; 32],
    mint: Pubkey,
    owner: Pubkey,
    compute_signer: Pubkey,
) -> Result<()> {
    assert_owner_amount_acl(
        amount_acl,
        amount_handle,
        mint,
        owner,
        compute_signer,
        burn_amount_label(),
    )
}

pub(crate) fn assert_transferred_amount_acl(
    amount_acl: &Account<zama_host::AclRecord>,
    amount_handle: [u8; 32],
    mint: Pubkey,
    from_token_account: Pubkey,
    from_owner: Pubkey,
    to_owner: Pubkey,
    compute_signer: Pubkey,
) -> Result<()> {
    assert_amount_acl_record_shape(amount_acl)?;
    require!(
        zama_host::handle_fhe_type(amount_handle) == BALANCE_FHE_TYPE,
        ConfidentialTokenError::AmountHandleTypeMismatch
    );
    require!(
        amount_acl.handle == amount_handle,
        ConfidentialTokenError::AmountAclMismatch
    );
    require_keys_eq!(
        amount_acl.acl_domain_key,
        mint,
        ConfidentialTokenError::AmountAclMismatch
    );
    require_keys_eq!(
        amount_acl.app_account,
        from_token_account,
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.encrypted_value_label == transferred_amount_label(),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.nonce_key == nonce_key(mint, from_token_account, transferred_amount_label()),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.inline_subject_has_role(from_owner, zama_host::ACL_ROLE_PUBLIC_DECRYPT),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.inline_subject_has_role(to_owner, zama_host::ACL_ROLE_PUBLIC_DECRYPT),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.inline_subject_has_role(compute_signer, zama_host::ACL_ROLE_USE),
        ConfidentialTokenError::AmountAclMismatch
    );
    Ok(())
}

pub(crate) fn assert_callback_success_acl(
    success_acl: &Account<zama_host::AclRecord>,
    success_handle: [u8; 32],
    mint: Pubkey,
    callback_authority: Pubkey,
    compute_signer: Pubkey,
) -> Result<()> {
    assert_amount_acl_record_shape(success_acl)?;
    require!(
        zama_host::handle_fhe_type(success_handle) == 0,
        ConfidentialTokenError::AmountHandleTypeMismatch
    );
    require!(
        success_acl.handle == success_handle,
        ConfidentialTokenError::AmountAclMismatch
    );
    require_keys_eq!(
        success_acl.acl_domain_key,
        mint,
        ConfidentialTokenError::AmountAclMismatch
    );
    require_keys_eq!(
        success_acl.app_account,
        callback_authority,
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        success_acl.encrypted_value_label == callback_success_label(),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        success_acl.nonce_key == nonce_key(mint, callback_authority, callback_success_label()),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        success_acl.inline_subject_has_role(compute_signer, zama_host::ACL_ROLE_USE),
        ConfidentialTokenError::AmountAclMismatch
    );
    Ok(())
}

pub(crate) fn assert_callback_refund_acl(
    refund_acl: &Account<zama_host::AclRecord>,
    refund_handle: [u8; 32],
    mint: Pubkey,
    to_token_account: Pubkey,
    from_owner: Pubkey,
    to_owner: Pubkey,
    compute_signer: Pubkey,
) -> Result<()> {
    assert_amount_acl_record_shape(refund_acl)?;
    require!(
        zama_host::handle_fhe_type(refund_handle) == BALANCE_FHE_TYPE,
        ConfidentialTokenError::AmountHandleTypeMismatch
    );
    require!(
        refund_acl.handle == refund_handle,
        ConfidentialTokenError::AmountAclMismatch
    );
    require_keys_eq!(
        refund_acl.acl_domain_key,
        mint,
        ConfidentialTokenError::AmountAclMismatch
    );
    require_keys_eq!(
        refund_acl.app_account,
        to_token_account,
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        refund_acl.encrypted_value_label == callback_refund_amount_label(),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        refund_acl.nonce_key == nonce_key(mint, to_token_account, callback_refund_amount_label()),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        refund_acl.inline_subject_has_role(from_owner, zama_host::ACL_ROLE_PUBLIC_DECRYPT),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        refund_acl.inline_subject_has_role(to_owner, zama_host::ACL_ROLE_PUBLIC_DECRYPT),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        refund_acl.inline_subject_has_role(compute_signer, zama_host::ACL_ROLE_USE),
        ConfidentialTokenError::AmountAclMismatch
    );
    Ok(())
}

pub(crate) fn assert_owner_amount_acl(
    amount_acl: &Account<zama_host::AclRecord>,
    amount_handle: [u8; 32],
    mint: Pubkey,
    owner: Pubkey,
    compute_signer: Pubkey,
    encrypted_value_label: [u8; 32],
) -> Result<()> {
    assert_amount_acl_record_shape(amount_acl)?;
    require!(
        zama_host::handle_fhe_type(amount_handle) == BALANCE_FHE_TYPE,
        ConfidentialTokenError::AmountHandleTypeMismatch
    );
    require!(
        amount_acl.handle == amount_handle,
        ConfidentialTokenError::AmountAclMismatch
    );
    require_keys_eq!(
        amount_acl.acl_domain_key,
        mint,
        ConfidentialTokenError::AmountAclMismatch
    );
    require_keys_eq!(
        amount_acl.app_account,
        owner,
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.encrypted_value_label == encrypted_value_label,
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.nonce_key == nonce_key(mint, owner, encrypted_value_label),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.inline_subject_has_role(compute_signer, zama_host::ACL_ROLE_USE),
        ConfidentialTokenError::AmountAclMismatch
    );
    Ok(())
}

pub(crate) fn assert_token_amount_acl(
    amount_acl: &Account<zama_host::AclRecord>,
    amount_handle: [u8; 32],
    mint: Pubkey,
    compute_signer: Pubkey,
) -> Result<()> {
    assert_amount_acl_record_shape(amount_acl)?;
    require!(
        zama_host::handle_fhe_type(amount_handle) == BALANCE_FHE_TYPE,
        ConfidentialTokenError::AmountHandleTypeMismatch
    );
    require!(
        amount_acl.handle == amount_handle,
        ConfidentialTokenError::AmountAclMismatch
    );
    require_keys_eq!(
        amount_acl.acl_domain_key,
        mint,
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        is_token_amount_label(amount_acl.encrypted_value_label),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.nonce_key
            == nonce_key(
                mint,
                amount_acl.app_account,
                amount_acl.encrypted_value_label
            ),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.inline_subject_has_role(compute_signer, zama_host::ACL_ROLE_USE),
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
        || encrypted_value_label == callback_refund_amount_label()
        || encrypted_value_label == callback_final_transferred_label()
}

pub(crate) fn assert_burned_amount_acl(
    amount_acl: &Account<zama_host::AclRecord>,
    burned_handle: [u8; 32],
    mint: Pubkey,
    token_account: Pubkey,
    owner: Pubkey,
    compute_signer: Pubkey,
) -> Result<()> {
    assert_amount_acl_record_shape(amount_acl)?;
    require!(
        zama_host::handle_fhe_type(burned_handle) == BALANCE_FHE_TYPE,
        ConfidentialTokenError::AmountHandleTypeMismatch
    );
    require!(
        amount_acl.handle == burned_handle,
        ConfidentialTokenError::AmountAclMismatch
    );
    require_keys_eq!(
        amount_acl.acl_domain_key,
        mint,
        ConfidentialTokenError::AmountAclMismatch
    );
    require_keys_eq!(
        amount_acl.app_account,
        token_account,
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.encrypted_value_label == burned_amount_label(),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.nonce_key == nonce_key(mint, token_account, burned_amount_label()),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.inline_subject_has_role(owner, zama_host::ACL_ROLE_PUBLIC_DECRYPT),
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        amount_acl.inline_subject_has_role(compute_signer, zama_host::ACL_ROLE_USE),
        ConfidentialTokenError::AmountAclMismatch
    );
    Ok(())
}

pub(crate) fn assert_material_commitment(
    material: &Account<zama_host::HandleMaterialCommitment>,
    material_key: Pubkey,
    acl_record: &Account<zama_host::AclRecord>,
    handle: [u8; 32],
) -> Result<()> {
    let acl_record_key = acl_record.key();
    let (expected_key, expected_bump) = zama_host::handle_material_address(acl_record_key);
    require_keys_eq!(
        material_key,
        expected_key,
        ConfidentialTokenError::MaterialCommitmentMismatch
    );
    require!(
        material.bump == expected_bump,
        ConfidentialTokenError::MaterialCommitmentMismatch
    );
    require!(
        material.to_account_info().data_len() == 8 + zama_host::HandleMaterialCommitment::SPACE,
        ConfidentialTokenError::MaterialCommitmentMismatch
    );
    require_keys_eq!(
        material.acl_record,
        acl_record_key,
        ConfidentialTokenError::MaterialCommitmentMismatch
    );
    require!(
        material.handle == handle,
        ConfidentialTokenError::MaterialCommitmentMismatch
    );
    require!(
        material.state == zama_host::HANDLE_MATERIAL_STATE_COMMITTED,
        ConfidentialTokenError::MaterialCommitmentMismatch
    );
    require!(
        material.material_commitment_hash
            == zama_host::handle_material_commitment_hash(
                material_key,
                acl_record_key,
                material.key_id,
                material.ciphertext_digest,
                material.sns_ciphertext_digest,
                material.coprocessor_set_digest,
            ),
        ConfidentialTokenError::MaterialCommitmentMismatch
    );
    require_keys_eq!(
        acl_record.material_commitment,
        material_key,
        ConfidentialTokenError::MaterialCommitmentMismatch
    );
    require!(
        acl_record.material_commitment_hash == material.material_commitment_hash
            && acl_record.material_key_id == material.key_id,
        ConfidentialTokenError::MaterialCommitmentMismatch
    );
    Ok(())
}

pub(crate) fn assert_public_decrypt_released(
    acl_record: &Account<zama_host::AclRecord>,
) -> Result<()> {
    assert_amount_acl_record_shape(acl_record)?;
    require!(
        acl_record.public_decrypt,
        ConfidentialTokenError::PublicDecryptNotReleased
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

#[allow(clippy::too_many_arguments)]
pub(crate) fn assert_disclosure_request_witness(
    request: &Account<DisclosureRequest>,
    request_key: Pubkey,
    mode: u8,
    mint: Pubkey,
    token_account: Pubkey,
    app_account: Pubkey,
    handle: [u8; 32],
    acl_record: Pubkey,
    material_commitment: &Account<zama_host::HandleMaterialCommitment>,
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
            && request.acl_record == acl_record
            && request.material_commitment == material_commitment.key()
            && request.material_commitment_hash == material_commitment.material_commitment_hash
            && request.material_key_id == material_commitment.key_id
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
        request.acl_record,
        request.material_commitment,
        request.material_commitment_hash,
        request.material_key_id,
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
    burned_acl_record: Pubkey,
    material_commitment: &Account<zama_host::HandleMaterialCommitment>,
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
            && request.burned_acl_record == burned_acl_record
            && request.material_commitment == material_commitment.key()
            && request.material_commitment_hash == material_commitment.material_commitment_hash
            && request.material_key_id == material_commitment.key_id
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
        request.burned_acl_record,
        request.material_commitment,
        request.material_commitment_hash,
        request.material_key_id,
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

pub(crate) fn assert_amount_acl_record_shape(
    acl_record: &Account<zama_host::AclRecord>,
) -> Result<()> {
    let (expected_key, expected_bump) =
        zama_host::acl_record_address(acl_record.nonce_key, acl_record.nonce_sequence);
    require_keys_eq!(
        acl_record.key(),
        expected_key,
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        acl_record.to_account_info().data_len() == 8 + zama_host::AclRecord::SPACE,
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        acl_record.bump == expected_bump,
        ConfidentialTokenError::AmountAclMismatch
    );
    require!(
        zama_host::acl_record_subject_slots_are_canonical(acl_record),
        ConfidentialTokenError::AmountAclMismatch
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

pub(crate) fn transferred_amount_acl_subjects(
    from_owner: Pubkey,
    to_owner: Pubkey,
    compute_signer: Pubkey,
) -> Vec<zama_fhe::AccessSubject> {
    let mut subjects = vec![zama_fhe::AccessSubject::owner(from_owner)];
    if to_owner != from_owner {
        subjects.push(zama_fhe::AccessSubject::owner(to_owner));
    }
    subjects.push(zama_fhe::AccessSubject::compute(compute_signer));
    subjects
}

pub(crate) fn burned_amount_acl_subjects(
    owner: Pubkey,
    compute_signer: Pubkey,
) -> Vec<zama_fhe::AccessSubject> {
    balance_acl_subjects(owner, compute_signer)
}
