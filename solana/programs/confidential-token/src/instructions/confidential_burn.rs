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
    /// Stable balance lineage; read for the current handle and superseded by this eval.
    #[account(mut, address = token_account.balance_encrypted_value)]
    pub balance_value: Box<Account<'info, zama_host::EncryptedValue>>,
    /// Stable total-supply lineage; read for the current handle and superseded by this eval.
    #[account(mut, address = mint.total_supply_encrypted_value)]
    pub total_supply_value: Box<Account<'info, zama_host::EncryptedValue>>,
    /// CHECK: stable `burned_amount` lineage for `token_account`; created on the
    /// account's first burn, superseded in place thereafter to each burn's own
    /// delta. Each burn makes its own delta handle publicly decryptable at burn
    /// (ERC-7984 `unwrap` parity), so every burn stays permanently redeemable
    /// even after a later burn supersedes this lineage (DD-036 / Vector 2 closed).
    #[account(mut, address = encrypted_value_address(mint.key(), token_account.key(), burned_amount_label()).0)]
    pub burned_amount_value: UncheckedAccount<'info>,
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
pub fn confidential_burn<'info>(
    ctx: Context<'info, ConfidentialBurn<'info>>,
    amount_attestation: zama_host::CoprocessorInputAttestation,
) -> Result<()> {
    assert_confidential_mint_shape(&ctx.accounts.mint)?;
    let mint_key = ctx.accounts.mint.key();
    let compute_signer = ctx.accounts.mint.compute_signer;
    let total_supply_authority = ctx.accounts.total_supply_authority.key();
    let token_account = ctx.accounts.token_account.as_ref();
    let owner = token_account.owner;
    let token_account_key = token_account.key();
    let old_balance_handle = ctx.accounts.balance_value.current_handle;
    let old_total_supply_handle = ctx.accounts.total_supply_value.current_handle;

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
    // fromExternal parity: the burn amount is a coprocessor-attested external input authored by the
    // owner and bound to the mint compute-signer PDA (see assert_amount_attestation_binding).
    assert_amount_attestation_binding(&amount_attestation, owner, compute_signer)?;

    let balance_output = fhe::DurableOutput::new(
        ctx.accounts.balance_value.to_account_info(),
        durable_slot(mint_key, token_account_key, balance_label()),
        zama_fhe::AccessPolicy::for_owner_and_compute(owner, compute_signer)
            .map_err(invalid_eval_plan)?,
    )?;
    // ERC-7984 `unwrap` parity (`makePubliclyDecryptable(unwrapAmount)`): the burned delta is born
    // publicly decryptable inside this eval CPI, so the burn is permanently redeemable even after a
    // later burn supersedes this shared lineage (DD-036 / Vector 2) — with no second make-public CPI.
    let burned_output = fhe::DurableOutput::new_public(
        ctx.accounts.burned_amount_value.to_account_info(),
        durable_slot(mint_key, token_account_key, burned_amount_label()),
        access_policy_from_subjects(burned_amount_acl_subjects(owner, compute_signer))?,
    )?;
    let total_supply_output = fhe::DurableOutput::new(
        ctx.accounts.total_supply_value.to_account_info(),
        durable_slot(mint_key, total_supply_authority, total_supply_label()),
        zama_fhe::AccessPolicy::for_compute(compute_signer).map_err(invalid_eval_plan)?,
    )?;

    let balance = uint64_from_value(
        old_balance_handle,
        mint_key,
        token_account_key,
        balance_label(),
    )?;
    let total_supply = uint64_from_value(
        old_total_supply_handle,
        mint_key,
        total_supply_authority,
        total_supply_label(),
    )?;
    let context_id = transfer_eval_context(
        b"burn-balance",
        mint_key,
        token_account_key,
        token_account_key,
        amount_attestation.input_handle,
    )?;
    let mut builder = zama_fhe::EvalBuilder::new(
        context_id,
        zama_fhe::EvalAppAuthority::new(token_account_key),
    );
    let amount: zama_fhe::Uint64Handle = builder
        .verified_input(amount_attestation)
        .map_err(invalid_eval_plan)?;
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
            balance_output.output(),
        )
        .map_err(invalid_eval_plan)?;
    let burned = builder
        .sub(balance, new_balance, burned_output.output())
        .map_err(invalid_eval_plan)?;
    builder
        .sub(total_supply, burned, total_supply_output.output())
        .map_err(invalid_eval_plan)?;
    let plan = builder.finish().map_err(invalid_eval_plan)?;
    let compute_authority = fhe::ComputeAuthority::for_mint(
        &ctx.accounts.compute_signer,
        mint_key,
        ctx.bumps.compute_signer,
    )?;
    let total_supply_authority_bump = total_supply_authority_address(mint_key).1;
    let eval_accounts = fhe::EvalAccountSet::for_plan(
        &plan,
        [
            balance_output.account_info(),
            burned_output.account_info(),
            total_supply_output.account_info(),
        ],
        [
            fhe::OutputAuthority::token_account(token_account)?,
            fhe::OutputAuthority::total_supply(
                &ctx.accounts.total_supply_authority,
                mint_key,
                total_supply_authority_bump,
            )?,
        ],
    )?;
    fhe::eval(fhe::Eval {
        context: fhe::EvalContext {
            payer: &ctx.accounts.owner,
            event_authority: &ctx.accounts.zama_event_authority,
            zama_program: &ctx.accounts.zama_program,
            host_config: &ctx.accounts.host_config,
            deny_subject_records: ctx.remaining_accounts,
            compute_authority,
            system_program: &ctx.accounts.system_program,
        },
        accounts: &eval_accounts,
        plan,
    })?;
    let new_balance_handle = balance_output.handle()?;
    let burned_handle = burned_output.handle()?;
    let new_total_supply_handle = total_supply_output.handle()?;

    emit_cpi!(ConfidentialBurnEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        owner,
        token_account: token_account_key,
        burned_handle,
        burned_encrypted_value: ctx.accounts.burned_amount_value.key(),
    });
    emit_cpi!(BalanceHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        owner,
        token_account: token_account_key,
        old_handle: old_balance_handle,
        old_encrypted_value: ctx.accounts.balance_value.key(),
        new_handle: new_balance_handle,
        new_encrypted_value: ctx.accounts.balance_value.key(),
        reason: BalanceHandleUpdateReason::BurnDebit,
    });
    emit_cpi!(TotalSupplyHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        old_handle: old_total_supply_handle,
        old_encrypted_value: ctx.accounts.total_supply_value.key(),
        new_handle: new_total_supply_handle,
        new_encrypted_value: ctx.accounts.total_supply_value.key(),
        reason: TotalSupplyUpdateReason::Burn,
    });
    Ok(())
}
