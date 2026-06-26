//! Burns encrypted token balances and rotates confidential supply state.

use super::*;

/// Accounts for confidential balance burn.
#[derive(Accounts)]
#[event_cpi]
pub struct ConfidentialBurn<'info> {
    /// Token owner and burn authority.
    #[account(mut)]
    pub owner: Signer<'info>,
    /// Confidential mint whose encrypted total supply is decreased.
    #[account(mut)]
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Token account whose balance is decreased.
    #[account(mut)]
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    /// CHECK: Mint-scoped app authority for total-supply handles.
    #[account(seeds = [b"total-supply", mint.key().as_ref()], bump)]
    pub total_supply_authority: UncheckedAccount<'info>,
    /// Encrypted burn amount ACL record.
    pub amount_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// CHECK: balance encrypted-value ACL lineage; rotated via the Zama host CPI.
    #[account(mut)]
    pub balance_value_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub burned_amount_acl: UncheckedAccount<'info>,
    /// CHECK: total-supply encrypted-value ACL lineage; rotated via the Zama host CPI.
    #[account(mut)]
    pub total_supply_value_acl: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used for FHE operations.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// System program used for ACL account creation.
    pub system_program: Program<'info, System>,
}

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
    let amount_nonce_sequence = token_account.next_amount_nonce_sequence;
    let old_balance_handle = token_account.balance_handle;
    let old_total_supply_handle = ctx.accounts.mint.total_supply_handle;
    let total_supply_authority_bump = total_supply_authority_address(mint_key).1;

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
    require_keys_eq!(
        total_supply_authority,
        total_supply_authority_address(mint_key).0,
        ConfidentialTokenError::TotalSupplyAuthorityMismatch
    );
    assert_burn_amount_acl(
        &ctx.accounts.amount_compute_acl,
        amount_handle,
        mint_key,
        owner,
        compute_signer,
    )?;
    // Balance domain: debit the balance lineage and mint the one-shot `burned`
    // amount (a durable ACL record, the burn's redeemable witness).
    let burned_output = fhe::DurableOutput::new(
        ctx.accounts.burned_amount_acl.to_account_info(),
        durable_slot(
            mint_key,
            token_account_key,
            burned_amount_label(),
            amount_nonce_sequence,
        ),
        access_policy_from_subjects(burned_amount_acl_subjects(owner, compute_signer))?,
    )?;
    let balance = zama_fhe::Uint64Handle::durable_at(
        old_balance_handle,
        ctx.accounts.balance_value_acl.key(),
    )
    .map_err(invalid_eval_plan)?;
    let amount = uint64_from_acl(amount_handle, &ctx.accounts.amount_compute_acl)?;
    let context_id = transfer_eval_context(
        b"burn-balance",
        mint_key,
        token_account_key,
        token_account_key,
        amount_handle,
        amount_nonce_sequence,
        amount_nonce_sequence,
    )?;
    let mut builder = zama_fhe::EvalBuilder::new(
        context_id,
        zama_fhe::EvalAppAuthority::new(token_account_key),
    );
    let burn_success = builder
        .ge(balance, amount, zama_fhe::Output::transient())
        .map_err(invalid_eval_plan)?;
    let debit_candidate = builder
        .sub(balance, amount, zama_fhe::Output::transient())
        .map_err(invalid_eval_plan)?;
    let new_balance = builder
        .if_then_else(
            burn_success,
            debit_candidate,
            balance,
            zama_fhe::Output::transient(),
        )
        .map_err(invalid_eval_plan)?;
    let balance_index = new_balance
        .producer_index()
        .ok_or(error!(ConfidentialTokenError::InvalidFheEvalPlan))?;
    builder
        .sub(balance, new_balance, burned_output.output())
        .map_err(invalid_eval_plan)?;
    let plan = builder.finish().map_err(invalid_eval_plan)?;
    let compute_authority = fhe::ComputeAuthority::for_mint(
        &ctx.accounts.compute_signer,
        mint_key,
        ctx.bumps.compute_signer,
    )?;
    let eval_accounts = fhe::EvalAccountSet::for_plan(
        &plan,
        [
            ctx.accounts.balance_value_acl.to_account_info(),
            ctx.accounts.amount_compute_acl.to_account_info(),
            burned_output.account_info(),
        ],
        [fhe::OutputAuthority::token_account(token_account)?],
    )?;
    fhe::eval(fhe::Eval {
        context: fhe::EvalContext {
            payer: &ctx.accounts.owner,
            event_authority: &ctx.accounts.zama_event_authority,
            zama_program: &ctx.accounts.zama_program,
            host_config: &ctx.accounts.host_config,
            compute_authority,
            system_program: &ctx.accounts.system_program,
        },
        accounts: &eval_accounts,
        plan,
    })?;
    let new_balance_handle = fhe::read_eval_output_handle(balance_index)?;
    let burned_handle = burned_output.handle()?;

    upsert_value_acl(
        &LineageCpi {
            zama_program: ctx.accounts.zama_program.to_account_info(),
            encrypted_value_acl: ctx.accounts.balance_value_acl.to_account_info(),
            payer: ctx.accounts.owner.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        },
        LineageAuthority::balance(&ctx.accounts.token_account),
        mint_key,
        new_balance_handle,
        vec![owner, compute_signer],
    )?;

    // Total-supply domain: subtract the just-minted `burned` amount from the
    // total-supply lineage in its own single-domain transient eval.
    let total_supply = zama_fhe::Uint64Handle::durable_at(
        old_total_supply_handle,
        ctx.accounts.total_supply_value_acl.key(),
    )
    .map_err(invalid_eval_plan)?;
    let burned_input = zama_fhe::Uint64Handle::durable(
        burned_handle,
        durable_slot(
            mint_key,
            token_account_key,
            burned_amount_label(),
            amount_nonce_sequence,
        ),
    )
    .map_err(invalid_eval_plan)?;
    let supply_context_id = transfer_eval_context(
        b"burn-total-supply",
        mint_key,
        total_supply_authority,
        token_account_key,
        burned_handle,
        amount_nonce_sequence,
        amount_nonce_sequence,
    )?;
    let mut supply_builder = zama_fhe::EvalBuilder::new(
        supply_context_id,
        zama_fhe::EvalAppAuthority::new(total_supply_authority),
    );
    let new_total_supply = supply_builder
        .sub(total_supply, burned_input, zama_fhe::Output::transient())
        .map_err(invalid_eval_plan)?;
    let supply_index = new_total_supply
        .producer_index()
        .ok_or(error!(ConfidentialTokenError::InvalidFheEvalPlan))?;
    let supply_plan = supply_builder.finish().map_err(invalid_eval_plan)?;
    let supply_compute_authority = fhe::ComputeAuthority::for_mint(
        &ctx.accounts.compute_signer,
        mint_key,
        ctx.bumps.compute_signer,
    )?;
    fhe::eval_transient(
        fhe::EvalContext {
            payer: &ctx.accounts.owner,
            event_authority: &ctx.accounts.zama_event_authority,
            zama_program: &ctx.accounts.zama_program,
            host_config: &ctx.accounts.host_config,
            compute_authority: supply_compute_authority,
            system_program: &ctx.accounts.system_program,
        },
        fhe::OutputAuthority::total_supply(
            &ctx.accounts.total_supply_authority,
            mint_key,
            total_supply_authority_bump,
        )?,
        [
            ctx.accounts.total_supply_value_acl.to_account_info(),
            ctx.accounts.burned_amount_acl.to_account_info(),
        ],
        supply_plan,
    )?;
    let new_total_supply_handle = fhe::read_eval_output_handle(supply_index)?;
    upsert_value_acl(
        &LineageCpi {
            zama_program: ctx.accounts.zama_program.to_account_info(),
            encrypted_value_acl: ctx.accounts.total_supply_value_acl.to_account_info(),
            payer: ctx.accounts.owner.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        },
        LineageAuthority::total_supply(
            &ctx.accounts.total_supply_authority,
            mint_key,
            total_supply_authority_bump,
        ),
        mint_key,
        new_total_supply_handle,
        vec![compute_signer],
    )?;

    let token_account = &mut ctx.accounts.token_account;
    token_account.balance_handle = new_balance_handle;
    token_account.next_amount_nonce_sequence = amount_nonce_sequence
        .checked_add(1)
        .ok_or(ConfidentialTokenError::AclNonceOverflow)?;
    let mint = &mut ctx.accounts.mint;
    mint.total_supply_handle = new_total_supply_handle;

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
        new_handle: new_balance_handle,
        reason: BalanceHandleUpdateReason::BurnDebit,
    });
    emit_cpi!(TotalSupplyHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        old_handle: old_total_supply_handle,
        new_handle: new_total_supply_handle,
        reason: TotalSupplyUpdateReason::Burn,
    });
    Ok(())
}
