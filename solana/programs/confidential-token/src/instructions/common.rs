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
    pub(crate) mint: &'a Account<'info, ConfidentialMint>,
    pub(crate) from_account: &'a mut Box<Account<'info, ConfidentialTokenAccount>>,
    pub(crate) to_account: &'a mut Box<Account<'info, ConfidentialTokenAccount>>,
    pub(crate) compute_signer: &'a UncheckedAccount<'info>,
    pub(crate) from_current_compute_acl: &'a Account<'info, zama_host::AclRecord>,
    pub(crate) to_current_compute_acl: &'a Account<'info, zama_host::AclRecord>,
    pub(crate) amount_compute_acl: &'a Account<'info, zama_host::AclRecord>,
    pub(crate) transfer_success_acl: AccountInfo<'info>,
    pub(crate) debit_candidate_acl: AccountInfo<'info>,
    pub(crate) from_output_acl: AccountInfo<'info>,
    pub(crate) transferred_amount_acl: AccountInfo<'info>,
    pub(crate) to_output_acl: AccountInfo<'info>,
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
    pub(crate) old_from_acl_record: Pubkey,
    pub(crate) new_from_handle: [u8; 32],
    pub(crate) new_from_acl_record: Pubkey,
    pub(crate) transferred_handle: [u8; 32],
    pub(crate) transferred_acl_record: Pubkey,
    pub(crate) to_owner: Pubkey,
    pub(crate) to_token_account: Pubkey,
    pub(crate) old_to_handle: [u8; 32],
    pub(crate) old_to_acl_record: Pubkey,
    pub(crate) new_to_handle: [u8; 32],
    pub(crate) new_to_acl_record: Pubkey,
}

pub(crate) struct PrepareTransferCallbackAccounts<'a, 'info> {
    pub(crate) payer: &'a Signer<'info>,
    pub(crate) callback_authority: &'a UncheckedAccount<'info>,
    pub(crate) mint: &'a Account<'info, ConfidentialMint>,
    pub(crate) from_account: &'a Account<'info, ConfidentialTokenAccount>,
    pub(crate) to_account: &'a mut Box<Account<'info, ConfidentialTokenAccount>>,
    pub(crate) compute_signer: &'a UncheckedAccount<'info>,
    pub(crate) to_current_compute_acl: &'a Account<'info, zama_host::AclRecord>,
    pub(crate) sent_amount_acl: &'a Account<'info, zama_host::AclRecord>,
    pub(crate) callback_success_acl: &'a Account<'info, zama_host::AclRecord>,
    pub(crate) hook_record: &'a Account<'info, TransferReceiverHookCall>,
    pub(crate) settlement_record: &'a mut Account<'info, TransferCallbackSettlement>,
    pub(crate) callback_zero_acl: AccountInfo<'info>,
    pub(crate) requested_refund_acl: AccountInfo<'info>,
    pub(crate) refund_success_acl: AccountInfo<'info>,
    pub(crate) refund_debit_candidate_acl: AccountInfo<'info>,
    pub(crate) to_output_acl: AccountInfo<'info>,
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
    pub(crate) old_to_acl_record: Pubkey,
    pub(crate) new_to_handle: [u8; 32],
    pub(crate) new_to_acl_record: Pubkey,
}

pub(crate) struct FinalizeTransferCallbackAccounts<'a, 'info> {
    pub(crate) payer: &'a Signer<'info>,
    pub(crate) mint: &'a Account<'info, ConfidentialMint>,
    pub(crate) from_account: &'a mut Box<Account<'info, ConfidentialTokenAccount>>,
    pub(crate) to_account: &'a Account<'info, ConfidentialTokenAccount>,
    pub(crate) compute_signer: &'a UncheckedAccount<'info>,
    pub(crate) from_current_compute_acl: &'a Account<'info, zama_host::AclRecord>,
    pub(crate) sent_amount_acl: &'a Account<'info, zama_host::AclRecord>,
    pub(crate) settlement_record: &'a mut Account<'info, TransferCallbackSettlement>,
    pub(crate) refund_amount_acl: &'a Account<'info, zama_host::AclRecord>,
    pub(crate) from_output_acl: AccountInfo<'info>,
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
    pub(crate) old_from_acl_record: Pubkey,
    pub(crate) new_from_handle: [u8; 32],
    pub(crate) new_from_acl_record: Pubkey,
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
    let from_nonce_sequence = from.next_balance_nonce_sequence;
    let to_nonce_sequence = to.next_balance_nonce_sequence;
    let old_from_handle = from.balance_handle;
    let old_from_acl_record = from.balance_acl_record;
    let old_to_handle = to.balance_handle;
    let old_to_acl_record = to.balance_acl_record;

    assert_transfer_amount_acl(
        accounts.amount_compute_acl,
        amount_handle,
        mint_key,
        accounts.payer.key(),
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
    assert_current_balance_acl(
        accounts.from_current_compute_acl,
        accounts.from_current_compute_acl.key(),
        from,
        mint_key,
    )?;
    assert_current_balance_acl(
        accounts.to_current_compute_acl,
        accounts.to_current_compute_acl.key(),
        to,
        mint_key,
    )?;
    if from.key() == to.key() {
        assert_self_transfer_output_accounts(&accounts, mint_key, from.key(), from_nonce_sequence)?;
        return Ok(None);
    }

    let transfer_success_handle = ge_balance(
        accounts.payer,
        accounts.zama_event_authority,
        accounts.zama_program,
        accounts.host_config,
        accounts.compute_signer,
        from,
        accounts.from_current_compute_acl.to_account_info(),
        from.balance_handle,
        accounts.amount_compute_acl.to_account_info(),
        amount_handle,
        accounts.transfer_success_acl.clone(),
        mint_key,
        compute_signer_bump,
        accounts.system_program,
        from_nonce_sequence,
        transfer_success_label(),
    )?;
    let debit_candidate_handle = compute_balance_scratch(
        fhe::sub,
        BalanceScratch {
            payer: accounts.payer,
            zama_event_authority: accounts.zama_event_authority,
            zama_program: accounts.zama_program,
            host_config: accounts.host_config,
            compute_signer: accounts.compute_signer,
            token_account: from,
            lhs_acl_record: accounts.from_current_compute_acl.to_account_info(),
            lhs: from.balance_handle,
            rhs_acl_record: accounts.amount_compute_acl.to_account_info(),
            rhs: amount_handle,
            output_acl_record: accounts.debit_candidate_acl.clone(),
            mint: mint_key,
            compute_signer_bump,
            system_program: accounts.system_program,
            output_nonce_sequence: from_nonce_sequence,
            output_encrypted_value_label: debit_candidate_label(),
            output_subjects: compute_acl_subject(accounts.compute_signer.key()),
        },
    )?;
    let new_from_handle = select_balance(
        accounts.payer,
        accounts.zama_event_authority,
        accounts.zama_program,
        accounts.host_config,
        accounts.compute_signer,
        from,
        accounts.transfer_success_acl.clone(),
        transfer_success_handle,
        accounts.debit_candidate_acl.clone(),
        debit_candidate_handle,
        accounts.from_current_compute_acl.to_account_info(),
        from.balance_handle,
        accounts.from_output_acl.clone(),
        mint_key,
        compute_signer_bump,
        accounts.system_program,
        from_nonce_sequence,
    )?;
    let transferred_handle = compute_balance_scratch(
        fhe::sub,
        BalanceScratch {
            payer: accounts.payer,
            zama_event_authority: accounts.zama_event_authority,
            zama_program: accounts.zama_program,
            host_config: accounts.host_config,
            compute_signer: accounts.compute_signer,
            token_account: from,
            lhs_acl_record: accounts.from_current_compute_acl.to_account_info(),
            lhs: from.balance_handle,
            rhs_acl_record: accounts.from_output_acl.clone(),
            rhs: new_from_handle,
            output_acl_record: accounts.transferred_amount_acl.clone(),
            mint: mint_key,
            compute_signer_bump,
            system_program: accounts.system_program,
            output_nonce_sequence: from_nonce_sequence,
            output_encrypted_value_label: transferred_amount_label(),
            output_subjects: transferred_amount_acl_subjects(
                from.owner,
                to.owner,
                accounts.compute_signer.key(),
            ),
        },
    )?;
    let new_to_handle = add_balance(
        accounts.payer,
        accounts.zama_event_authority,
        accounts.zama_program,
        accounts.host_config,
        accounts.compute_signer,
        to,
        accounts.to_current_compute_acl.to_account_info(),
        to.balance_handle,
        accounts.transferred_amount_acl.clone(),
        transferred_handle,
        accounts.to_output_acl.clone(),
        mint_key,
        compute_signer_bump,
        accounts.system_program,
        to_nonce_sequence,
    )?;

    let from = accounts.from_account.as_mut();
    from.balance_handle = new_from_handle;
    from.balance_acl_record = accounts.from_output_acl.key();
    from.next_balance_nonce_sequence = from_nonce_sequence
        .checked_add(1)
        .ok_or(ConfidentialTokenError::AclNonceOverflow)?;
    let from_owner = from.owner;
    let from_token_account = from.key();
    let new_from_acl_record = accounts.from_output_acl.key();

    let to = accounts.to_account.as_mut();
    to.balance_handle = new_to_handle;
    to.balance_acl_record = accounts.to_output_acl.key();
    to.next_balance_nonce_sequence = to_nonce_sequence
        .checked_add(1)
        .ok_or(ConfidentialTokenError::AclNonceOverflow)?;
    Ok(Some(TransferOutcome {
        mint: mint_key,
        from_owner,
        from_token_account,
        old_from_handle,
        old_from_acl_record,
        new_from_handle,
        new_from_acl_record,
        transferred_handle,
        transferred_acl_record: accounts.transferred_amount_acl.key(),
        to_owner: to.owner,
        to_token_account: to.key(),
        old_to_handle,
        old_to_acl_record,
        new_to_handle,
        new_to_acl_record: accounts.to_output_acl.key(),
    }))
}

pub(crate) fn assert_self_transfer_output_accounts(
    accounts: &TransferAccounts<'_, '_>,
    mint: Pubkey,
    token_account: Pubkey,
    nonce_sequence: u64,
) -> Result<()> {
    assert_unused_acl_target(
        &accounts.transfer_success_acl,
        acl_record_address_for(
            mint,
            token_account,
            transfer_success_label(),
            nonce_sequence,
        ),
    )?;
    assert_unused_acl_target(
        &accounts.debit_candidate_acl,
        acl_record_address_for(mint, token_account, debit_candidate_label(), nonce_sequence),
    )?;
    let balance_output =
        acl_record_address_for(mint, token_account, balance_label(), nonce_sequence);
    assert_unused_acl_target(&accounts.from_output_acl, balance_output)?;
    assert_unused_acl_target(&accounts.to_output_acl, balance_output)?;
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
    let to_nonce_sequence = to.next_balance_nonce_sequence;
    let old_to_handle = to.balance_handle;
    let old_to_acl_record = to.balance_acl_record;
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
    assert_current_balance_acl(
        accounts.to_current_compute_acl,
        old_to_acl_record,
        to,
        mint_key,
    )?;
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

    let zero_handle = fhe::trivial_encrypt_u64(fhe::TrivialEncryptU64 {
        payer: accounts.payer,
        event_authority: accounts.zama_event_authority,
        zama_program: accounts.zama_program,
        host_config: accounts.host_config,
        compute_signer: accounts.compute_signer,
        app_account_authority: to,
        output_acl_record: accounts.callback_zero_acl.clone(),
        acl_domain_key: mint_key,
        compute_signer_bump,
        system_program: accounts.system_program,
        output_nonce_key: nonce_key(mint_key, to_token_account, callback_zero_label()),
        output_nonce_sequence: to_nonce_sequence,
        output_encrypted_value_label: callback_zero_label(),
        plaintext: 0,
        fhe_type: BALANCE_FHE_TYPE,
        output_subjects: compute_acl_subject(compute_signer),
        output_public_decrypt: false,
    })?;
    let requested_refund_handle = select_amount_scratch(TernaryScratch {
        payer: accounts.payer,
        zama_event_authority: accounts.zama_event_authority,
        zama_program: accounts.zama_program,
        host_config: accounts.host_config,
        compute_signer: accounts.compute_signer,
        token_account: to,
        control_acl_record: accounts.callback_success_acl.to_account_info(),
        control: callback_success_handle,
        if_true_acl_record: accounts.callback_zero_acl.clone(),
        if_true: zero_handle,
        if_false_acl_record: accounts.sent_amount_acl.to_account_info(),
        if_false: sent_handle,
        output_acl_record: accounts.requested_refund_acl.clone(),
        mint: mint_key,
        compute_signer_bump,
        system_program: accounts.system_program,
        output_nonce_sequence: to_nonce_sequence,
        output_encrypted_value_label: callback_refund_request_label(),
        output_subjects: amount_subjects.clone(),
    })?;
    let refund_success_handle = ge_balance(
        accounts.payer,
        accounts.zama_event_authority,
        accounts.zama_program,
        accounts.host_config,
        accounts.compute_signer,
        to,
        accounts.to_current_compute_acl.to_account_info(),
        old_to_handle,
        accounts.requested_refund_acl.clone(),
        requested_refund_handle,
        accounts.refund_success_acl.clone(),
        mint_key,
        compute_signer_bump,
        accounts.system_program,
        to_nonce_sequence,
        callback_refund_success_label(),
    )?;
    let refund_debit_candidate_handle = compute_balance_scratch(
        fhe::sub,
        BalanceScratch {
            payer: accounts.payer,
            zama_event_authority: accounts.zama_event_authority,
            zama_program: accounts.zama_program,
            host_config: accounts.host_config,
            compute_signer: accounts.compute_signer,
            token_account: to,
            lhs_acl_record: accounts.to_current_compute_acl.to_account_info(),
            lhs: old_to_handle,
            rhs_acl_record: accounts.requested_refund_acl.clone(),
            rhs: requested_refund_handle,
            output_acl_record: accounts.refund_debit_candidate_acl.clone(),
            mint: mint_key,
            compute_signer_bump,
            system_program: accounts.system_program,
            output_nonce_sequence: to_nonce_sequence,
            output_encrypted_value_label: callback_refund_debit_candidate_label(),
            output_subjects: compute_acl_subject(compute_signer),
        },
    )?;
    let new_to_handle = select_balance(
        accounts.payer,
        accounts.zama_event_authority,
        accounts.zama_program,
        accounts.host_config,
        accounts.compute_signer,
        to,
        accounts.refund_success_acl.clone(),
        refund_success_handle,
        accounts.refund_debit_candidate_acl.clone(),
        refund_debit_candidate_handle,
        accounts.to_current_compute_acl.to_account_info(),
        old_to_handle,
        accounts.to_output_acl.clone(),
        mint_key,
        compute_signer_bump,
        accounts.system_program,
        to_nonce_sequence,
    )?;
    let refund_handle = compute_balance_scratch(
        fhe::sub,
        BalanceScratch {
            payer: accounts.payer,
            zama_event_authority: accounts.zama_event_authority,
            zama_program: accounts.zama_program,
            host_config: accounts.host_config,
            compute_signer: accounts.compute_signer,
            token_account: to,
            lhs_acl_record: accounts.to_current_compute_acl.to_account_info(),
            lhs: old_to_handle,
            rhs_acl_record: accounts.to_output_acl.clone(),
            rhs: new_to_handle,
            output_acl_record: accounts.refund_amount_acl.clone(),
            mint: mint_key,
            compute_signer_bump,
            system_program: accounts.system_program,
            output_nonce_sequence: to_nonce_sequence,
            output_encrypted_value_label: callback_refund_amount_label(),
            output_subjects: amount_subjects.clone(),
        },
    )?;

    let new_to_acl_record = accounts.to_output_acl.key();
    let requested_refund_acl_record = accounts.requested_refund_acl.key();
    let refund_acl_record = accounts.refund_amount_acl.key();

    let to = accounts.to_account.as_mut();
    to.balance_handle = new_to_handle;
    to.balance_acl_record = new_to_acl_record;
    to.next_balance_nonce_sequence = to_nonce_sequence
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
    settlement.requested_refund_handle = requested_refund_handle;
    settlement.requested_refund_acl_record = requested_refund_acl_record;
    settlement.refund_handle = refund_handle;
    settlement.refund_acl_record = refund_acl_record;
    settlement.to_balance_handle = new_to_handle;
    settlement.to_balance_acl_record = new_to_acl_record;
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
        old_to_acl_record,
        new_to_handle,
        new_to_acl_record,
    })
}

pub(crate) fn assert_active_operator_record(
    operator_record: &Account<ConfidentialOperator>,
    token_account: &Account<ConfidentialTokenAccount>,
    operator: Pubkey,
) -> Result<()> {
    assert_confidential_token_account_shape(
        token_account,
        token_account.mint,
        token_account.owner,
    )?;
    assert_operator_record_shape(
        operator_record,
        token_account.key(),
        token_account.owner,
        operator,
    )?;
    let slot = Clock::get()?.slot;
    require!(
        operator_record.expiration_slot != 0 && operator_record.expiration_slot >= slot,
        ConfidentialTokenError::OperatorExpired
    );
    Ok(())
}

pub(crate) fn assert_operator_record_shape(
    operator_record: &Account<ConfidentialOperator>,
    token_account: Pubkey,
    owner: Pubkey,
    operator: Pubkey,
) -> Result<()> {
    let (expected_key, expected_bump) = operator_record_address(token_account, operator);
    require_keys_eq!(
        operator_record.key(),
        expected_key,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    require!(
        operator_record.to_account_info().data_len() == 8 + ConfidentialOperator::SPACE,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    require!(
        operator_record.bump == expected_bump,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    require_keys_eq!(
        operator_record.token_account,
        token_account,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    require_keys_eq!(
        operator_record.owner,
        owner,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    require_keys_eq!(
        operator_record.operator,
        operator,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    Ok(())
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
                    authority: 0,
                    operator_record: None,
                    mint: 1,
                    from_token_account: 2,
                    to_token_account: 3,
                    sent_acl_record: 11,
                },
                PreviousTransferAuthority {
                    authority: owner,
                    operator_record: None,
                },
                mint,
                from_token_account,
                to_token_account,
                sent_acl_record,
            )
        }
        (
            discriminator,
            PreviousTransferIntent::Operator {
                operator,
                operator_record,
            },
        ) if discriminator == crate::instruction::ConfidentialTransferFrom::DISCRIMINATOR => {
            assert_previous_transfer_accounts(
                &transfer_ix.accounts,
                PreviousTransferAccountIndexes {
                    authority: 0,
                    operator_record: Some(4),
                    mint: 1,
                    from_token_account: 2,
                    to_token_account: 3,
                    sent_acl_record: 12,
                },
                PreviousTransferAuthority {
                    authority: operator,
                    operator_record: Some(operator_record),
                },
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
    Direct {
        owner: Pubkey,
    },
    Operator {
        operator: Pubkey,
        operator_record: Pubkey,
    },
}

struct PreviousTransferAuthority {
    authority: Pubkey,
    operator_record: Option<Pubkey>,
}

struct PreviousTransferAccountIndexes {
    authority: usize,
    operator_record: Option<usize>,
    mint: usize,
    from_token_account: usize,
    to_token_account: usize,
    sent_acl_record: usize,
}

fn assert_previous_transfer_accounts(
    accounts: &[AccountMeta],
    indexes: PreviousTransferAccountIndexes,
    authority: PreviousTransferAuthority,
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
        authority.authority,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    if let (Some(index), Some(expected_operator_record)) =
        (indexes.operator_record, authority.operator_record)
    {
        let operator_record_meta = accounts
            .get(index)
            .ok_or(ConfidentialTokenError::ReceiverHookMismatch)?;
        require_keys_eq!(
            operator_record_meta.pubkey,
            expected_operator_record,
            ConfidentialTokenError::ReceiverHookMismatch
        );
    }
    require_keys_eq!(
        mint_meta.pubkey,
        mint,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    require_keys_eq!(
        from_meta.pubkey,
        from_token_account,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    require_keys_eq!(
        to_meta.pubkey,
        to_token_account,
        ConfidentialTokenError::ReceiverHookMismatch
    );
    require_keys_eq!(
        sent_meta.pubkey,
        sent_acl_record,
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
    let from_nonce_sequence = from.next_balance_nonce_sequence;
    let old_from_handle = from.balance_handle;
    let old_from_acl_record = from.balance_acl_record;
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
    assert_current_balance_acl(
        accounts.from_current_compute_acl,
        old_from_acl_record,
        from,
        mint_key,
    )?;
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

    let new_from_handle = add_balance(
        accounts.payer,
        accounts.zama_event_authority,
        accounts.zama_program,
        accounts.host_config,
        accounts.compute_signer,
        from,
        accounts.from_current_compute_acl.to_account_info(),
        old_from_handle,
        accounts.refund_amount_acl.to_account_info(),
        refund_handle,
        accounts.from_output_acl.clone(),
        mint_key,
        compute_signer_bump,
        accounts.system_program,
        from_nonce_sequence,
    )?;
    let final_transferred_handle = compute_balance_scratch(
        fhe::sub,
        BalanceScratch {
            payer: accounts.payer,
            zama_event_authority: accounts.zama_event_authority,
            zama_program: accounts.zama_program,
            host_config: accounts.host_config,
            compute_signer: accounts.compute_signer,
            token_account: from,
            lhs_acl_record: accounts.sent_amount_acl.to_account_info(),
            lhs: sent_handle,
            rhs_acl_record: accounts.refund_amount_acl.to_account_info(),
            rhs: refund_handle,
            output_acl_record: accounts.transferred_amount_acl.clone(),
            mint: mint_key,
            compute_signer_bump,
            system_program: accounts.system_program,
            output_nonce_sequence: from_nonce_sequence,
            output_encrypted_value_label: callback_final_transferred_label(),
            output_subjects: amount_subjects,
        },
    )?;

    let new_from_acl_record = accounts.from_output_acl.key();
    let transferred_acl_record = accounts.transferred_amount_acl.key();
    let from = accounts.from_account.as_mut();
    from.balance_handle = new_from_handle;
    from.balance_acl_record = new_from_acl_record;
    from.next_balance_nonce_sequence = from_nonce_sequence
        .checked_add(1)
        .ok_or(ConfidentialTokenError::AclNonceOverflow)?;

    let settlement = &mut accounts.settlement_record;
    settlement.from_balance_handle = new_from_handle;
    settlement.from_balance_acl_record = new_from_acl_record;
    settlement.transferred_handle = final_transferred_handle;
    settlement.transferred_acl_record = transferred_acl_record;
    settlement.status = CALLBACK_SETTLEMENT_FINALIZED;

    Ok(FinalizeTransferCallbackOutcome {
        mint: mint_key,
        from_owner,
        from_token_account,
        old_from_handle,
        old_from_acl_record,
        new_from_handle,
        new_from_acl_record,
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

pub(crate) fn assert_disclosure_signature(
    instructions_sysvar: &AccountInfo,
    verifier: Pubkey,
    mint: Pubkey,
    handle: [u8; 32],
    cleartext_amount: u64,
) -> Result<()> {
    require_keys_eq!(
        instructions_sysvar.key(),
        INSTRUCTIONS_SYSVAR_ID,
        ConfidentialTokenError::DisclosureProofSignatureMissing
    );
    let message = disclosure_proof_message(mint, handle, cleartext_amount, crate::ID);
    let current_index = load_current_index_checked(instructions_sysvar)
        .map_err(|_| error!(ConfidentialTokenError::DisclosureProofSignatureMissing))?;
    let verifier_index = current_index
        .checked_sub(1)
        .ok_or(ConfidentialTokenError::DisclosureProofSignatureMissing)?;
    let verifier_ix = load_instruction_at_checked(verifier_index as usize, instructions_sysvar)
        .map_err(|_| error!(ConfidentialTokenError::DisclosureProofSignatureMissing))?;
    require_keys_eq!(
        verifier_ix.program_id,
        ED25519_PROGRAM_ID,
        ConfidentialTokenError::DisclosureProofSignatureMissing
    );
    require!(
        ed25519_instruction_contains_message(&verifier_ix.data, verifier.as_ref(), &message),
        ConfidentialTokenError::DisclosureProofSignatureMissing
    );
    Ok(())
}

/// Builds the message that a KMS disclosure response signs for this token PoC.
pub fn disclosure_proof_message(
    mint: Pubkey,
    handle: [u8; 32],
    cleartext_amount: u64,
    program_id: Pubkey,
) -> Vec<u8> {
    let mut message = Vec::with_capacity(
        DISCLOSURE_PROOF_DOMAIN_SEPARATOR.len() + 32 + 32 + 32 + std::mem::size_of::<u64>(),
    );
    message.extend_from_slice(DISCLOSURE_PROOF_DOMAIN_SEPARATOR);
    message.extend_from_slice(program_id.as_ref());
    message.extend_from_slice(mint.as_ref());
    message.extend_from_slice(&handle);
    message.extend_from_slice(&cleartext_amount.to_le_bytes());
    message
}

pub(crate) fn ed25519_instruction_contains_message(
    data: &[u8],
    expected_pubkey: &[u8],
    expected_message: &[u8],
) -> bool {
    if data.len() < ED25519_SIGNATURE_OFFSETS_START {
        return false;
    }
    if data[1] != 0 {
        return false;
    }
    let signature_count = data[0] as usize;
    if signature_count == 0 {
        return false;
    }
    let expected_offsets_end = ED25519_SIGNATURE_OFFSETS_START
        .saturating_add(signature_count.saturating_mul(ED25519_SIGNATURE_OFFSETS_SERIALIZED_SIZE));
    if data.len() < expected_offsets_end {
        return false;
    }

    for signature_index in 0..signature_count {
        let start = ED25519_SIGNATURE_OFFSETS_START.saturating_add(
            signature_index.saturating_mul(ED25519_SIGNATURE_OFFSETS_SERIALIZED_SIZE),
        );
        let fields = &data[start..start + ED25519_SIGNATURE_OFFSETS_SERIALIZED_SIZE];
        let signature_offset = read_u16_le(fields, 0) as usize;
        let signature_instruction_index = read_u16_le(fields, 2);
        let public_key_offset = read_u16_le(fields, 4) as usize;
        let public_key_instruction_index = read_u16_le(fields, 6);
        let message_data_offset = read_u16_le(fields, 8) as usize;
        let message_data_size = read_u16_le(fields, 10) as usize;
        let message_instruction_index = read_u16_le(fields, 12);

        if signature_instruction_index != u16::MAX
            || public_key_instruction_index != u16::MAX
            || message_instruction_index != u16::MAX
        {
            continue;
        }
        let Some(signature_end) = signature_offset.checked_add(ED25519_SIGNATURE_SERIALIZED_SIZE)
        else {
            continue;
        };
        let Some(public_key_end) = public_key_offset.checked_add(ED25519_PUBKEY_SERIALIZED_SIZE)
        else {
            continue;
        };
        let Some(message_end) = message_data_offset.checked_add(message_data_size) else {
            continue;
        };
        if signature_end > data.len() || public_key_end > data.len() || message_end > data.len() {
            continue;
        }
        if &data[public_key_offset..public_key_end] != expected_pubkey {
            continue;
        }
        if &data[message_data_offset..message_end] == expected_message {
            return true;
        }
    }
    false
}

pub(crate) fn read_u16_le(data: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([data[offset], data[offset + 1]])
}

pub(crate) fn assert_current_balance_acl(
    balance_acl: &Account<zama_host::AclRecord>,
    balance_acl_key: Pubkey,
    token_account: &Account<ConfidentialTokenAccount>,
    mint: Pubkey,
) -> Result<()> {
    assert_current_acl_record_shape(balance_acl)?;
    require_keys_eq!(
        balance_acl_key,
        token_account.balance_acl_record,
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    require!(
        balance_acl.handle == token_account.balance_handle,
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    require_keys_eq!(
        balance_acl.acl_domain_key,
        mint,
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    require_keys_eq!(
        balance_acl.app_account,
        token_account.key(),
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    require!(
        balance_acl.encrypted_value_label == balance_label(),
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    require!(
        balance_acl.nonce_key == balance_nonce_key(mint, token_account.key()),
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    Ok(())
}

pub(crate) fn assert_current_total_supply_acl(
    supply_acl: &Account<zama_host::AclRecord>,
    supply_acl_key: Pubkey,
    mint: &Account<ConfidentialMint>,
    mint_key: Pubkey,
    total_supply_authority: Pubkey,
) -> Result<()> {
    assert_current_acl_record_shape(supply_acl)?;
    require_keys_eq!(
        supply_acl_key,
        mint.total_supply_acl_record,
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    require!(
        supply_acl.handle == mint.total_supply_handle,
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    require_keys_eq!(
        supply_acl.acl_domain_key,
        mint_key,
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    require_keys_eq!(
        supply_acl.app_account,
        total_supply_authority,
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    require!(
        supply_acl.encrypted_value_label == total_supply_label(),
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    require!(
        supply_acl.nonce_key == total_supply_nonce_key(mint_key, total_supply_authority),
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    Ok(())
}

pub(crate) fn assert_current_acl_record_shape(
    acl_record: &Account<zama_host::AclRecord>,
) -> Result<()> {
    let (expected_key, expected_bump) =
        zama_host::acl_record_address(acl_record.nonce_key, acl_record.nonce_sequence);
    require_keys_eq!(
        acl_record.key(),
        expected_key,
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    require!(
        acl_record.to_account_info().data_len() == 8 + zama_host::AclRecord::SPACE,
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    require!(
        acl_record.bump == expected_bump,
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
    require!(
        zama_host::acl_record_subject_slots_are_canonical(acl_record),
        ConfidentialTokenError::CurrentAclRecordMismatch
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

pub(crate) fn create_operator_record_if_needed<'info>(
    payer: &AccountInfo<'info>,
    operator_record: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    token_account: Pubkey,
    owner: Pubkey,
    operator: Pubkey,
    bump: u8,
) -> Result<()> {
    if operator_record.owner == &crate::ID {
        assert_existing_operator_record(operator_record, token_account, owner, operator, bump)?;
        return Ok(());
    }
    require_keys_eq!(
        *operator_record.owner,
        System::id(),
        ConfidentialTokenError::OperatorRecordMismatch
    );
    require!(
        operator_record.data_is_empty(),
        ConfidentialTokenError::OperatorRecordMismatch
    );
    require!(
        !operator_record.executable,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    let rent = Rent::get()?.minimum_balance(8 + ConfidentialOperator::SPACE);
    invoke_signed(
        &system_instruction::create_account(
            payer.key,
            operator_record.key,
            rent,
            (8 + ConfidentialOperator::SPACE) as u64,
            &crate::ID,
        ),
        &[
            payer.clone(),
            operator_record.clone(),
            system_program.clone(),
        ],
        &[&[
            b"operator",
            token_account.as_ref(),
            operator.as_ref(),
            &[bump],
        ]],
    )?;
    require_keys_eq!(
        *operator_record.owner,
        crate::ID,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    require!(
        !operator_record.executable,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    require!(
        operator_record.data_len() == 8 + ConfidentialOperator::SPACE,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    require!(
        operator_record.lamports() >= rent,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    Ok(())
}

pub(crate) fn assert_existing_operator_record(
    operator_record: &AccountInfo,
    token_account: Pubkey,
    owner: Pubkey,
    operator: Pubkey,
    bump: u8,
) -> Result<()> {
    require!(
        operator_record.data_len() == 8 + ConfidentialOperator::SPACE,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    let data = operator_record.try_borrow_data()?;
    let mut cursor = &data[..];
    let existing = ConfidentialOperator::try_deserialize(&mut cursor)
        .map_err(|_| error!(ConfidentialTokenError::OperatorRecordMismatch))?;
    require_keys_eq!(
        existing.token_account,
        token_account,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    require_keys_eq!(
        existing.owner,
        owner,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    require_keys_eq!(
        existing.operator,
        operator,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    require!(
        existing.bump == bump,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    Ok(())
}

pub(crate) fn write_operator_record(
    info: &AccountInfo,
    record: &ConfidentialOperator,
) -> Result<()> {
    let mut data = info.try_borrow_mut_data()?;
    let mut cursor = &mut data[..];
    record.try_serialize(&mut cursor)?;
    Ok(())
}

pub(crate) fn add_balance<'info>(
    payer: &Signer<'info>,
    zama_event_authority: &UncheckedAccount<'info>,
    zama_program: &Program<'info, ZamaHost>,
    host_config: &Account<'info, zama_host::HostConfig>,
    compute_signer: &UncheckedAccount<'info>,
    token_account: &Account<'info, ConfidentialTokenAccount>,
    lhs_acl_record: AccountInfo<'info>,
    lhs: [u8; 32],
    rhs_acl_record: AccountInfo<'info>,
    rhs: [u8; 32],
    output_acl_record: AccountInfo<'info>,
    mint: Pubkey,
    compute_signer_bump: u8,
    system_program: &Program<'info, System>,
    output_nonce_sequence: u64,
) -> Result<[u8; 32]> {
    compute_balance_with(
        fhe::add,
        BalanceCompute {
            payer,
            zama_event_authority,
            zama_program,
            host_config,
            compute_signer,
            token_account,
            lhs_acl_record,
            lhs,
            rhs_acl_record,
            rhs,
            output_acl_record,
            mint,
            compute_signer_bump,
            system_program,
            output_nonce_sequence,
        },
    )
}

pub(crate) fn ge_balance<'info>(
    payer: &Signer<'info>,
    zama_event_authority: &UncheckedAccount<'info>,
    zama_program: &Program<'info, ZamaHost>,
    host_config: &Account<'info, zama_host::HostConfig>,
    compute_signer: &UncheckedAccount<'info>,
    token_account: &Account<'info, ConfidentialTokenAccount>,
    lhs_acl_record: AccountInfo<'info>,
    lhs: [u8; 32],
    rhs_acl_record: AccountInfo<'info>,
    rhs: [u8; 32],
    output_acl_record: AccountInfo<'info>,
    mint: Pubkey,
    compute_signer_bump: u8,
    system_program: &Program<'info, System>,
    output_nonce_sequence: u64,
    output_encrypted_value_label: [u8; 32],
) -> Result<[u8; 32]> {
    fhe::ge(fhe::BinaryOp {
        payer,
        event_authority: zama_event_authority,
        zama_program,
        host_config,
        compute_signer,
        app_account_authority: token_account,
        lhs_acl_record,
        lhs,
        rhs_acl_record,
        rhs,
        scalar: false,
        output_acl_record,
        output_fhe_type: 0,
        acl_domain_key: mint,
        compute_signer_bump,
        system_program,
        output_nonce_key: nonce_key(mint, token_account.key(), output_encrypted_value_label),
        output_nonce_sequence,
        output_encrypted_value_label,
        output_subjects: compute_acl_subject(compute_signer.key()),
        output_public_decrypt: false,
    })
}

pub(crate) fn select_balance<'info>(
    payer: &Signer<'info>,
    zama_event_authority: &UncheckedAccount<'info>,
    zama_program: &Program<'info, ZamaHost>,
    host_config: &Account<'info, zama_host::HostConfig>,
    compute_signer: &UncheckedAccount<'info>,
    token_account: &Account<'info, ConfidentialTokenAccount>,
    control_acl_record: AccountInfo<'info>,
    control: [u8; 32],
    if_true_acl_record: AccountInfo<'info>,
    if_true: [u8; 32],
    if_false_acl_record: AccountInfo<'info>,
    if_false: [u8; 32],
    output_acl_record: AccountInfo<'info>,
    mint: Pubkey,
    compute_signer_bump: u8,
    system_program: &Program<'info, System>,
    output_nonce_sequence: u64,
) -> Result<[u8; 32]> {
    fhe::if_then_else(fhe::TernaryOp {
        payer,
        event_authority: zama_event_authority,
        zama_program,
        host_config,
        compute_signer,
        app_account_authority: token_account,
        control_acl_record,
        control,
        if_true_acl_record,
        if_true,
        if_false_acl_record,
        if_false,
        output_acl_record,
        output_fhe_type: BALANCE_FHE_TYPE,
        acl_domain_key: mint,
        compute_signer_bump,
        system_program,
        output_nonce_key: balance_nonce_key(mint, token_account.key()),
        output_nonce_sequence,
        output_encrypted_value_label: balance_label(),
        output_subjects: balance_acl_subjects(token_account.owner, compute_signer.key()),
        output_public_decrypt: false,
    })
}

pub(crate) struct BalanceCompute<'a, 'info> {
    pub(crate) payer: &'a Signer<'info>,
    pub(crate) zama_event_authority: &'a UncheckedAccount<'info>,
    pub(crate) zama_program: &'a Program<'info, ZamaHost>,
    pub(crate) host_config: &'a Account<'info, zama_host::HostConfig>,
    pub(crate) compute_signer: &'a UncheckedAccount<'info>,
    pub(crate) token_account: &'a Account<'info, ConfidentialTokenAccount>,
    pub(crate) lhs_acl_record: AccountInfo<'info>,
    pub(crate) lhs: [u8; 32],
    pub(crate) rhs_acl_record: AccountInfo<'info>,
    pub(crate) rhs: [u8; 32],
    pub(crate) output_acl_record: AccountInfo<'info>,
    pub(crate) mint: Pubkey,
    pub(crate) compute_signer_bump: u8,
    pub(crate) system_program: &'a Program<'info, System>,
    pub(crate) output_nonce_sequence: u64,
}

pub(crate) struct BalanceScratch<'a, 'info> {
    pub(crate) payer: &'a Signer<'info>,
    pub(crate) zama_event_authority: &'a UncheckedAccount<'info>,
    pub(crate) zama_program: &'a Program<'info, ZamaHost>,
    pub(crate) host_config: &'a Account<'info, zama_host::HostConfig>,
    pub(crate) compute_signer: &'a UncheckedAccount<'info>,
    pub(crate) token_account: &'a Account<'info, ConfidentialTokenAccount>,
    pub(crate) lhs_acl_record: AccountInfo<'info>,
    pub(crate) lhs: [u8; 32],
    pub(crate) rhs_acl_record: AccountInfo<'info>,
    pub(crate) rhs: [u8; 32],
    pub(crate) output_acl_record: AccountInfo<'info>,
    pub(crate) mint: Pubkey,
    pub(crate) compute_signer_bump: u8,
    pub(crate) system_program: &'a Program<'info, System>,
    pub(crate) output_nonce_sequence: u64,
    pub(crate) output_encrypted_value_label: [u8; 32],
    pub(crate) output_subjects: Vec<AclSubjectEntry>,
}

pub(crate) struct TernaryScratch<'a, 'info> {
    pub(crate) payer: &'a Signer<'info>,
    pub(crate) zama_event_authority: &'a UncheckedAccount<'info>,
    pub(crate) zama_program: &'a Program<'info, ZamaHost>,
    pub(crate) host_config: &'a Account<'info, zama_host::HostConfig>,
    pub(crate) compute_signer: &'a UncheckedAccount<'info>,
    pub(crate) token_account: &'a Account<'info, ConfidentialTokenAccount>,
    pub(crate) control_acl_record: AccountInfo<'info>,
    pub(crate) control: [u8; 32],
    pub(crate) if_true_acl_record: AccountInfo<'info>,
    pub(crate) if_true: [u8; 32],
    pub(crate) if_false_acl_record: AccountInfo<'info>,
    pub(crate) if_false: [u8; 32],
    pub(crate) output_acl_record: AccountInfo<'info>,
    pub(crate) mint: Pubkey,
    pub(crate) compute_signer_bump: u8,
    pub(crate) system_program: &'a Program<'info, System>,
    pub(crate) output_nonce_sequence: u64,
    pub(crate) output_encrypted_value_label: [u8; 32],
    pub(crate) output_subjects: Vec<AclSubjectEntry>,
}

pub(crate) fn compute_balance_with<'info>(
    op: for<'a> fn(fhe::BinaryOp<'a, 'info>) -> Result<[u8; 32]>,
    request: BalanceCompute<'_, 'info>,
) -> Result<[u8; 32]> {
    let token_account_key = request.token_account.key();
    op(fhe::BinaryOp {
        payer: request.payer,
        event_authority: request.zama_event_authority,
        zama_program: request.zama_program,
        host_config: request.host_config,
        compute_signer: request.compute_signer,
        app_account_authority: request.token_account,
        lhs_acl_record: request.lhs_acl_record,
        lhs: request.lhs,
        rhs_acl_record: request.rhs_acl_record,
        rhs: request.rhs,
        scalar: false,
        output_acl_record: request.output_acl_record,
        output_fhe_type: BALANCE_FHE_TYPE,
        acl_domain_key: request.mint,
        compute_signer_bump: request.compute_signer_bump,
        system_program: request.system_program,
        output_nonce_key: balance_nonce_key(request.mint, token_account_key),
        output_nonce_sequence: request.output_nonce_sequence,
        output_encrypted_value_label: balance_label(),
        output_subjects: balance_acl_subjects(
            request.token_account.owner,
            request.compute_signer.key(),
        ),
        output_public_decrypt: false,
    })
}

pub(crate) fn compute_balance_scratch<'info>(
    op: for<'a> fn(fhe::BinaryOp<'a, 'info>) -> Result<[u8; 32]>,
    request: BalanceScratch<'_, 'info>,
) -> Result<[u8; 32]> {
    op(fhe::BinaryOp {
        payer: request.payer,
        event_authority: request.zama_event_authority,
        zama_program: request.zama_program,
        host_config: request.host_config,
        compute_signer: request.compute_signer,
        app_account_authority: request.token_account,
        lhs_acl_record: request.lhs_acl_record,
        lhs: request.lhs,
        rhs_acl_record: request.rhs_acl_record,
        rhs: request.rhs,
        scalar: false,
        output_acl_record: request.output_acl_record,
        output_fhe_type: BALANCE_FHE_TYPE,
        acl_domain_key: request.mint,
        compute_signer_bump: request.compute_signer_bump,
        system_program: request.system_program,
        output_nonce_key: nonce_key(
            request.mint,
            request.token_account.key(),
            request.output_encrypted_value_label,
        ),
        output_nonce_sequence: request.output_nonce_sequence,
        output_encrypted_value_label: request.output_encrypted_value_label,
        output_subjects: request.output_subjects,
        output_public_decrypt: false,
    })
}

pub(crate) fn select_amount_scratch<'info>(request: TernaryScratch<'_, 'info>) -> Result<[u8; 32]> {
    fhe::if_then_else(fhe::TernaryOp {
        payer: request.payer,
        event_authority: request.zama_event_authority,
        zama_program: request.zama_program,
        host_config: request.host_config,
        compute_signer: request.compute_signer,
        app_account_authority: request.token_account,
        control_acl_record: request.control_acl_record,
        control: request.control,
        if_true_acl_record: request.if_true_acl_record,
        if_true: request.if_true,
        if_false_acl_record: request.if_false_acl_record,
        if_false: request.if_false,
        output_acl_record: request.output_acl_record,
        output_fhe_type: BALANCE_FHE_TYPE,
        acl_domain_key: request.mint,
        compute_signer_bump: request.compute_signer_bump,
        system_program: request.system_program,
        output_nonce_key: nonce_key(
            request.mint,
            request.token_account.key(),
            request.output_encrypted_value_label,
        ),
        output_nonce_sequence: request.output_nonce_sequence,
        output_encrypted_value_label: request.output_encrypted_value_label,
        output_subjects: request.output_subjects,
        output_public_decrypt: false,
    })
}

pub(crate) fn trivial_encrypt_balance_acl<'info>(
    payer: &Signer<'info>,
    mint: &Account<'info, ConfidentialMint>,
    compute_signer: &UncheckedAccount<'info>,
    token_account: &Account<'info, ConfidentialTokenAccount>,
    acl_record: AccountInfo<'info>,
    zama_event_authority: &UncheckedAccount<'info>,
    zama_program: &Program<'info, ZamaHost>,
    host_config: &Account<'info, zama_host::HostConfig>,
    system_program: &Program<'info, System>,
    compute_signer_bump: u8,
    nonce_sequence: u64,
    plaintext: u64,
) -> Result<[u8; 32]> {
    fhe::trivial_encrypt_u64(fhe::TrivialEncryptU64 {
        payer,
        event_authority: zama_event_authority,
        zama_program,
        host_config,
        compute_signer,
        app_account_authority: token_account,
        output_acl_record: acl_record,
        acl_domain_key: mint.key(),
        compute_signer_bump,
        system_program,
        output_nonce_key: balance_nonce_key(mint.key(), token_account.key()),
        output_nonce_sequence: nonce_sequence,
        output_encrypted_value_label: balance_label(),
        plaintext,
        fhe_type: BALANCE_FHE_TYPE,
        output_subjects: balance_acl_subjects(token_account.owner, compute_signer.key()),
        output_public_decrypt: false,
    })
}

pub(crate) fn balance_acl_subjects(owner: Pubkey, compute_signer: Pubkey) -> Vec<AclSubjectEntry> {
    vec![
        AclSubjectEntry::user(owner),
        AclSubjectEntry::compute(compute_signer),
    ]
}

pub(crate) fn compute_acl_subject(compute_signer: Pubkey) -> Vec<AclSubjectEntry> {
    vec![AclSubjectEntry::compute(compute_signer)]
}

pub(crate) fn transferred_amount_acl_subjects(
    from_owner: Pubkey,
    to_owner: Pubkey,
    compute_signer: Pubkey,
) -> Vec<AclSubjectEntry> {
    let mut subjects = vec![AclSubjectEntry::user(from_owner)];
    if to_owner != from_owner {
        subjects.push(AclSubjectEntry::user(to_owner));
    }
    subjects.push(AclSubjectEntry::compute(compute_signer));
    subjects
}

pub(crate) fn burned_amount_acl_subjects(
    owner: Pubkey,
    compute_signer: Pubkey,
) -> Vec<AclSubjectEntry> {
    balance_acl_subjects(owner, compute_signer)
}
