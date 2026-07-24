//! Wraps public USDC into a confidential token balance.

use super::*;

/// Accounts for wrapping public USDC into a confidential balance.
#[derive(Accounts)]
#[event_cpi]
pub struct WrapUsdc<'info> {
    /// Token owner and transfer authority.
    #[account(mut)]
    pub owner: Signer<'info>,
    /// Confidential mint.
    #[account(mut)]
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Confidential token account whose balance is increased.
    #[account(mut)]
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Underlying SPL mint.
    pub underlying_mint: Box<Account<'info, SplMint>>,
    /// Owner's source USDC token account.
    #[account(
        mut,
        constraint = user_usdc.mint == underlying_mint.key() @ ConfidentialTokenError::UnderlyingMintMismatch,
        constraint = user_usdc.owner == owner.key() @ ConfidentialTokenError::OwnerMismatch
    )]
    pub user_usdc: Box<Account<'info, TokenAccount>>,
    /// Program vault USDC token account.
    #[account(
        mut,
        constraint = vault_usdc.mint == underlying_mint.key() @ ConfidentialTokenError::UnderlyingMintMismatch,
        constraint = vault_usdc.owner == vault_authority.key() @ ConfidentialTokenError::VaultAuthorityMismatch
    )]
    pub vault_usdc: Box<Account<'info, TokenAccount>>,
    /// CHECK: PDA authority for the underlying-token vault.
    #[account(seeds = [b"vault-authority", mint.key().as_ref()], bump)]
    pub vault_authority: UncheckedAccount<'info>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    /// CHECK: Mint-scoped app authority for total-supply handles.
    #[account(seeds = [b"total-supply", mint.key().as_ref()], bump)]
    pub total_supply_authority: UncheckedAccount<'info>,
    /// Stable balance encrypted value account; read for the current handle and superseded by this eval.
    #[account(mut, address = token_account.balance_encrypted_value)]
    pub balance_value: Box<Account<'info, zama_host::EncryptedValue>>,
    /// Stable total-supply encrypted value account; read for the current handle and superseded by this eval.
    #[account(mut, address = mint.total_supply_encrypted_value)]
    pub total_supply_value: Box<Account<'info, zama_host::EncryptedValue>>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used for FHE operations.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// SPL token program.
    pub token_program: Program<'info, Token>,
    /// System program used for ACL account creation.
    pub system_program: Program<'info, System>,
    /// CHECK: forwarded verbatim into the ZamaHost `fhe_eval` CPI, which validates it against the
    /// canonical `["hcu-block-meter", compute_signer]` PDA. Supplied by an untrusted mint under a
    /// metering-band cap; omitted when the mint is trusted or the cap is unrestricted.
    #[account(mut)]
    pub hcu_block_meter: Option<UncheckedAccount<'info>>,
    /// CHECK: forwarded verbatim into the ZamaHost `fhe_eval` CPI, which validates it against the
    /// canonical `["hcu-trusted", compute_signer]` PDA. Present + valid bypasses the cap; absent
    /// means the mint is metered.
    pub hcu_trusted_app_record: Option<UncheckedAccount<'info>>,
}

/// Escrows public USDC and rotates the confidential balance by `amount`.
pub fn wrap_usdc<'info>(ctx: Context<'info, WrapUsdc<'info>>, amount: u64) -> Result<()> {
    assert_confidential_mint_shape(&ctx.accounts.mint)?;
    let mint_key = ctx.accounts.mint.key();
    let decimals = ctx.accounts.mint.decimals;
    let compute_signer = ctx.accounts.mint.compute_signer;
    let total_supply_authority = ctx.accounts.total_supply_authority.key();
    let old_total_supply_handle = ctx.accounts.total_supply_value.current_handle;
    let token_account = ctx.accounts.token_account.as_ref();
    let old_balance_handle = ctx.accounts.balance_value.current_handle;

    require_keys_eq!(
        token_account.owner,
        ctx.accounts.owner.key(),
        ConfidentialTokenError::OwnerMismatch
    );
    require_keys_eq!(
        token_account.mint,
        mint_key,
        ConfidentialTokenError::MintMismatch
    );
    assert_confidential_token_account_shape(token_account, mint_key, ctx.accounts.owner.key())?;
    require_keys_eq!(
        ctx.accounts.mint.underlying_mint,
        ctx.accounts.underlying_mint.key(),
        ConfidentialTokenError::UnderlyingMintMismatch
    );
    assert_canonical_vault_token_account(
        ctx.accounts.vault_usdc.key(),
        ctx.accounts.vault_authority.key(),
        ctx.accounts.underlying_mint.key(),
    )?;
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
    let balance_output = fhe::DurableOutput::new(
        ctx.accounts.balance_value.to_account_info(),
        durable_slot(mint_key, token_account.key(), balance_label()),
        fhe::DurableAudience::for_owner(token_account.owner, compute_signer),
    )?;
    let total_supply_output = fhe::DurableOutput::new(
        ctx.accounts.total_supply_value.to_account_info(),
        durable_slot(mint_key, total_supply_authority, total_supply_label()),
        fhe::DurableAudience::compute_only(compute_signer),
    )?;

    spl_token::transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.key(),
            TransferChecked {
                from: ctx.accounts.user_usdc.to_account_info(),
                mint: ctx.accounts.underlying_mint.to_account_info(),
                to: ctx.accounts.vault_usdc.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ),
        amount,
        decimals,
    )?;

    let balance = uint64_from_value(
        old_balance_handle,
        mint_key,
        token_account.key(),
        balance_label(),
    )?;
    let total_supply = uint64_from_value(
        old_total_supply_handle,
        mint_key,
        total_supply_authority,
        total_supply_label(),
    )?;
    let mut amount_context = [0u8; 32];
    amount_context[24..].copy_from_slice(&amount.to_be_bytes());
    let context_id = transfer_eval_context(
        b"wrap-balance",
        mint_key,
        token_account.key(),
        token_account.key(),
        amount_context,
    )?;
    let mut builder = zama_fhe::EvalBuilder::new(
        context_id,
        zama_fhe::EvalAppAuthority::new(token_account.key()),
    );
    let encrypted_amount = builder
        .trivial_encrypt_u64(amount, zama_fhe::Output::transient())
        .map_err(invalid_eval_plan)?;
    builder
        .add(balance, encrypted_amount, balance_output.output())
        .map_err(invalid_eval_plan)?;
    builder
        .add(total_supply, encrypted_amount, total_supply_output.output())
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
            total_supply_output.account_info(),
        ],
        [
            fhe::OutputAuthority::token_account(&ctx.accounts.token_account)?,
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
            hcu_block_meter: ctx
                .accounts
                .hcu_block_meter
                .as_ref()
                .map(|account| account.to_account_info()),
            hcu_trusted_app_record: ctx
                .accounts
                .hcu_trusted_app_record
                .as_ref()
                .map(|account| account.to_account_info()),
        },
        accounts: &eval_accounts,
        plan,
    })?;
    let new_balance_handle = balance_output.handle()?;
    let new_total_supply_handle = total_supply_output.handle()?;

    let token_account_key = ctx.accounts.token_account.key();
    let owner = ctx.accounts.token_account.owner;
    emit_cpi!(BalanceHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        owner,
        token_account: token_account_key,
        old_handle: old_balance_handle,
        old_encrypted_value: ctx.accounts.balance_value.key(),
        new_handle: new_balance_handle,
        new_encrypted_value: ctx.accounts.balance_value.key(),
        reason: BalanceHandleUpdateReason::Wrap,
    });
    emit_cpi!(TotalSupplyHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        old_handle: old_total_supply_handle,
        old_encrypted_value: ctx.accounts.total_supply_value.key(),
        new_handle: new_total_supply_handle,
        new_encrypted_value: ctx.accounts.total_supply_value.key(),
        reason: TotalSupplyUpdateReason::Wrap,
    });
    Ok(())
}
