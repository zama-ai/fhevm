use super::*;

/// Burns an encrypted amount by rotating the account balance and encrypted total supply.
pub fn confidential_burn(ctx: Context<ConfidentialBurn>, amount_handle: [u8; 32]) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_confidential_mint_shape(&ctx.accounts.mint)?;
    let mint_key = ctx.accounts.mint.key();
    let compute_signer = ctx.accounts.mint.compute_signer;
    let total_supply_authority = ctx.accounts.total_supply_authority.key();
    let token_account = ctx.accounts.token_account.as_ref();
    let owner = token_account.owner;
    let token_account_key = token_account.key();
    let balance_nonce_sequence = token_account.next_balance_nonce_sequence;
    let old_balance_handle = token_account.balance_handle;
    let old_balance_acl_record = token_account.balance_acl_record;
    let total_supply_nonce_sequence = ctx.accounts.mint.next_total_supply_nonce_sequence;
    let old_total_supply_handle = ctx.accounts.mint.total_supply_handle;
    let old_total_supply_acl_record = ctx.accounts.mint.total_supply_acl_record;

    require_keys_eq!(
        owner,
        ctx.accounts.owner.key(),
        ConfidentialTokenError::OwnerMismatch
    );
    require_keys_eq!(
        token_account.mint,
        mint_key,
        ConfidentialTokenError::MintMismatch
    );
    assert_confidential_token_account_shape(token_account, mint_key, owner)?;
    require_keys_eq!(
        ctx.accounts.compute_signer.key(),
        compute_signer,
        ConfidentialTokenError::ComputeSignerMismatch
    );
    assert_current_balance_acl(
        &ctx.accounts.current_compute_acl,
        ctx.accounts.current_compute_acl.key(),
        token_account,
        mint_key,
    )?;
    require_keys_eq!(
        total_supply_authority,
        total_supply_authority_address(mint_key).0,
        ConfidentialTokenError::TotalSupplyAuthorityMismatch
    );
    assert_current_total_supply_acl(
        &ctx.accounts.current_total_supply_acl,
        ctx.accounts.current_total_supply_acl.key(),
        ctx.accounts.mint.as_ref(),
        mint_key,
        total_supply_authority,
    )?;
    assert_burn_amount_acl(
        &ctx.accounts.amount_compute_acl,
        amount_handle,
        mint_key,
        owner,
        compute_signer,
    )?;

    let burn_success_handle = ge_balance(
        &ctx.accounts.owner,
        &ctx.accounts.zama_event_authority,
        &ctx.accounts.zama_program,
        &ctx.accounts.host_config,
        &ctx.accounts.compute_signer,
        token_account,
        ctx.accounts.current_compute_acl.to_account_info(),
        old_balance_handle,
        ctx.accounts.amount_compute_acl.to_account_info(),
        amount_handle,
        ctx.accounts.burn_success_acl.to_account_info(),
        mint_key,
        ctx.bumps.compute_signer,
        &ctx.accounts.system_program,
        balance_nonce_sequence,
        burn_success_label(),
    )?;
    let debit_candidate_handle = compute_balance_scratch(
        fhe::sub,
        BalanceScratch {
            payer: &ctx.accounts.owner,
            zama_event_authority: &ctx.accounts.zama_event_authority,
            zama_program: &ctx.accounts.zama_program,
            host_config: &ctx.accounts.host_config,
            compute_signer: &ctx.accounts.compute_signer,
            token_account,
            lhs_acl_record: ctx.accounts.current_compute_acl.to_account_info(),
            lhs: old_balance_handle,
            rhs_acl_record: ctx.accounts.amount_compute_acl.to_account_info(),
            rhs: amount_handle,
            output_acl_record: ctx.accounts.debit_candidate_acl.to_account_info(),
            mint: mint_key,
            compute_signer_bump: ctx.bumps.compute_signer,
            system_program: &ctx.accounts.system_program,
            output_nonce_sequence: balance_nonce_sequence,
            output_encrypted_value_label: burn_debit_candidate_label(),
            output_subjects: compute_acl_subject(ctx.accounts.compute_signer.key()),
        },
    )?;
    let new_balance_handle = select_balance(
        &ctx.accounts.owner,
        &ctx.accounts.zama_event_authority,
        &ctx.accounts.zama_program,
        &ctx.accounts.host_config,
        &ctx.accounts.compute_signer,
        token_account,
        ctx.accounts.burn_success_acl.to_account_info(),
        burn_success_handle,
        ctx.accounts.debit_candidate_acl.to_account_info(),
        debit_candidate_handle,
        ctx.accounts.current_compute_acl.to_account_info(),
        old_balance_handle,
        ctx.accounts.output_acl.to_account_info(),
        mint_key,
        ctx.bumps.compute_signer,
        &ctx.accounts.system_program,
        balance_nonce_sequence,
    )?;
    let burned_handle = compute_balance_scratch(
        fhe::sub,
        BalanceScratch {
            payer: &ctx.accounts.owner,
            zama_event_authority: &ctx.accounts.zama_event_authority,
            zama_program: &ctx.accounts.zama_program,
            host_config: &ctx.accounts.host_config,
            compute_signer: &ctx.accounts.compute_signer,
            token_account,
            lhs_acl_record: ctx.accounts.current_compute_acl.to_account_info(),
            lhs: old_balance_handle,
            rhs_acl_record: ctx.accounts.output_acl.to_account_info(),
            rhs: new_balance_handle,
            output_acl_record: ctx.accounts.burned_amount_acl.to_account_info(),
            mint: mint_key,
            compute_signer_bump: ctx.bumps.compute_signer,
            system_program: &ctx.accounts.system_program,
            output_nonce_sequence: balance_nonce_sequence,
            output_encrypted_value_label: burned_amount_label(),
            output_subjects: burned_amount_acl_subjects(owner, ctx.accounts.compute_signer.key()),
        },
    )?;

    let total_supply_authority_bump = [ctx.bumps.total_supply_authority];
    let total_supply_authority_seeds: &[&[u8]] = &[
        b"total-supply",
        mint_key.as_ref(),
        &total_supply_authority_bump,
    ];
    let new_total_supply_handle = fhe::sub_with_app_pda(fhe::BinaryOpWithAppPda {
        payer: &ctx.accounts.owner,
        event_authority: &ctx.accounts.zama_event_authority,
        zama_program: &ctx.accounts.zama_program,
        host_config: &ctx.accounts.host_config,
        compute_signer: &ctx.accounts.compute_signer,
        app_account_authority: &ctx.accounts.total_supply_authority,
        app_signer_seeds: total_supply_authority_seeds,
        output_app_account: total_supply_authority,
        lhs_acl_record: ctx.accounts.current_total_supply_acl.to_account_info(),
        lhs: old_total_supply_handle,
        rhs_acl_record: ctx.accounts.burned_amount_acl.to_account_info(),
        rhs: burned_handle,
        scalar: false,
        output_acl_record: ctx.accounts.total_supply_output_acl.to_account_info(),
        output_fhe_type: BALANCE_FHE_TYPE,
        acl_domain_key: mint_key,
        compute_signer_bump: ctx.bumps.compute_signer,
        system_program: &ctx.accounts.system_program,
        output_nonce_key: total_supply_nonce_key(mint_key, total_supply_authority),
        output_nonce_sequence: total_supply_nonce_sequence,
        output_encrypted_value_label: total_supply_label(),
        output_subjects: compute_acl_subject(ctx.accounts.compute_signer.key()),
        output_public_decrypt: false,
    })?;

    let token_account = &mut ctx.accounts.token_account;
    token_account.balance_handle = new_balance_handle;
    token_account.balance_acl_record = ctx.accounts.output_acl.key();
    token_account.next_balance_nonce_sequence = balance_nonce_sequence
        .checked_add(1)
        .ok_or(ConfidentialTokenError::AclNonceOverflow)?;
    let mint = &mut ctx.accounts.mint;
    mint.total_supply_handle = new_total_supply_handle;
    mint.total_supply_acl_record = ctx.accounts.total_supply_output_acl.key();
    mint.next_total_supply_nonce_sequence = total_supply_nonce_sequence
        .checked_add(1)
        .ok_or(ConfidentialTokenError::AclNonceOverflow)?;

    emit_cpi!(ConfidentialBurnEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        owner,
        token_account: token_account_key,
        burned_handle,
        burned_acl_record: ctx.accounts.burned_amount_acl.key(),
    });
    emit_cpi!(BalanceHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        owner,
        token_account: token_account_key,
        old_handle: old_balance_handle,
        old_acl_record: old_balance_acl_record,
        new_handle: new_balance_handle,
        new_acl_record: ctx.accounts.output_acl.key(),
        reason: BalanceHandleUpdateReason::BurnDebit,
    });
    emit_cpi!(TotalSupplyHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        old_handle: old_total_supply_handle,
        old_acl_record: old_total_supply_acl_record,
        new_handle: new_total_supply_handle,
        new_acl_record: ctx.accounts.total_supply_output_acl.key(),
        reason: TotalSupplyUpdateReason::Burn,
    });
    Ok(())
}
