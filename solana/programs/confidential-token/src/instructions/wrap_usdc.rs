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
    /// CHECK: total-supply encrypted-value ACL lineage; rotated via the Zama host CPI.
    #[account(mut)]
    pub total_supply_value_acl: UncheckedAccount<'info>,
    /// CHECK: balance encrypted-value ACL lineage; created/rotated via the Zama host CPI.
    #[account(mut)]
    pub balance_value_acl: UncheckedAccount<'info>,
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
}

/// Escrows public USDC and rotates the confidential balance by `amount`.
pub fn wrap_usdc(ctx: Context<WrapUsdc>, amount: u64) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_confidential_mint_shape(&ctx.accounts.mint)?;
    let mint_key = ctx.accounts.mint.key();
    let decimals = ctx.accounts.mint.decimals;
    let compute_signer = ctx.accounts.mint.compute_signer;
    let total_supply_authority = ctx.accounts.total_supply_authority.key();
    let total_supply_authority_bump = total_supply_authority_address(mint_key).1;
    let old_total_supply_handle = ctx.accounts.mint.total_supply_handle;
    let token_account = ctx.accounts.token_account.as_ref();
    let nonce_sequence = token_account.next_amount_nonce_sequence;
    let old_balance_handle = token_account.balance_handle;
    let owner = ctx.accounts.owner.key();

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

    // Balance and total supply are distinct ACL domains (owner-scoped vs.
    // mint-scoped), so each rotates through its own single-domain transient eval
    // and binds the returned handle into its encrypted-value ACL lineage.
    let compute_authority = fhe::ComputeAuthority::for_mint(
        &ctx.accounts.compute_signer,
        mint_key,
        ctx.bumps.compute_signer,
    )?;
    let new_balance_handle = credit_lineage_by_amount(
        CreditLineageByAmount {
            context: fhe::EvalContext {
                payer: &ctx.accounts.owner,
                event_authority: &ctx.accounts.zama_event_authority,
                zama_program: &ctx.accounts.zama_program,
                host_config: &ctx.accounts.host_config,
                compute_authority,
                system_program: &ctx.accounts.system_program,
            },
            eval_authority: fhe::OutputAuthority::token_account(&ctx.accounts.token_account)?,
            cpi: LineageCpi {
                zama_program: ctx.accounts.zama_program.to_account_info(),
                encrypted_value_acl: ctx.accounts.balance_value_acl.to_account_info(),
                payer: ctx.accounts.owner.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
            lineage: LineageAuthority::balance(ctx.accounts.token_account.as_ref()),
            acl_domain_key: mint_key,
            tag: b"wrap-balance",
            old_handle: old_balance_handle,
            nonce_sequence,
            subjects: vec![owner, compute_signer],
        },
        amount,
    )?;
    let compute_authority = fhe::ComputeAuthority::for_mint(
        &ctx.accounts.compute_signer,
        mint_key,
        ctx.bumps.compute_signer,
    )?;
    let new_total_supply_handle = credit_lineage_by_amount(
        CreditLineageByAmount {
            context: fhe::EvalContext {
                payer: &ctx.accounts.owner,
                event_authority: &ctx.accounts.zama_event_authority,
                zama_program: &ctx.accounts.zama_program,
                host_config: &ctx.accounts.host_config,
                compute_authority,
                system_program: &ctx.accounts.system_program,
            },
            eval_authority: fhe::OutputAuthority::total_supply(
                &ctx.accounts.total_supply_authority,
                mint_key,
                total_supply_authority_bump,
            )?,
            cpi: LineageCpi {
                zama_program: ctx.accounts.zama_program.to_account_info(),
                encrypted_value_acl: ctx.accounts.total_supply_value_acl.to_account_info(),
                payer: ctx.accounts.owner.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
            lineage: LineageAuthority::total_supply(
                &ctx.accounts.total_supply_authority,
                mint_key,
                total_supply_authority_bump,
            ),
            acl_domain_key: mint_key,
            tag: b"wrap-total-supply",
            old_handle: old_total_supply_handle,
            nonce_sequence,
            subjects: vec![compute_signer],
        },
        amount,
    )?;

    let token_account = &mut ctx.accounts.token_account;
    token_account.balance_handle = new_balance_handle;
    let mint = &mut ctx.accounts.mint;
    mint.total_supply_handle = new_total_supply_handle;
    emit_cpi!(BalanceHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        owner: token_account.owner,
        token_account: token_account.key(),
        old_handle: old_balance_handle,
        new_handle: new_balance_handle,
        reason: BalanceHandleUpdateReason::Wrap,
    });
    emit_cpi!(TotalSupplyHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        old_handle: old_total_supply_handle,
        new_handle: new_total_supply_handle,
        reason: TotalSupplyUpdateReason::Wrap,
    });
    Ok(())
}
