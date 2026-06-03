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
    /// Current balance ACL record used as the left-hand operand.
    pub current_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// Current total-supply ACL record used as the left-hand operand.
    pub current_total_supply_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub amount_compute_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub output_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub total_supply_output_acl: UncheckedAccount<'info>,
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
    let old_total_supply_handle = ctx.accounts.mint.total_supply_handle;
    let old_total_supply_acl_record = ctx.accounts.mint.total_supply_acl_record;
    let total_supply_nonce_sequence = ctx.accounts.mint.next_total_supply_nonce_sequence;
    let token_account = ctx.accounts.token_account.as_ref();
    let nonce_sequence = token_account.next_balance_nonce_sequence;
    let old_balance_handle = token_account.balance_handle;
    let old_balance_acl_record = token_account.balance_acl_record;

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
        ctx.accounts.current_compute_acl.key(),
        token_account.balance_acl_record,
        ConfidentialTokenError::CurrentAclRecordMismatch
    );
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

    let amount_handle = fhe::trivial_encrypt_u64(fhe::TrivialEncryptU64 {
        payer: &ctx.accounts.owner,
        event_authority: &ctx.accounts.zama_event_authority,
        zama_program: &ctx.accounts.zama_program,
        host_config: &ctx.accounts.host_config,
        compute_signer: &ctx.accounts.compute_signer,
        app_account_authority: token_account,
        output_acl_record: ctx.accounts.amount_compute_acl.to_account_info(),
        acl_domain_key: mint_key,
        compute_signer_bump: ctx.bumps.compute_signer,
        system_program: &ctx.accounts.system_program,
        output_nonce_key: nonce_key(mint_key, token_account.key(), wrap_amount_label()),
        output_nonce_sequence: nonce_sequence,
        output_encrypted_value_label: wrap_amount_label(),
        plaintext: amount,
        fhe_type: BALANCE_FHE_TYPE,
        output_subjects: compute_acl_subject(compute_signer),
        output_public_decrypt: false,
    })?;

    let new_balance_handle = add_balance(
        &ctx.accounts.owner,
        &ctx.accounts.zama_event_authority,
        &ctx.accounts.zama_program,
        &ctx.accounts.host_config,
        &ctx.accounts.compute_signer,
        &ctx.accounts.token_account,
        ctx.accounts.current_compute_acl.to_account_info(),
        token_account.balance_handle,
        ctx.accounts.amount_compute_acl.to_account_info(),
        amount_handle,
        ctx.accounts.output_acl.to_account_info(),
        mint_key,
        ctx.bumps.compute_signer,
        &ctx.accounts.system_program,
        nonce_sequence,
    )?;
    let total_supply_authority_bump = [ctx.bumps.total_supply_authority];
    let total_supply_authority_seeds: &[&[u8]] = &[
        b"total-supply",
        mint_key.as_ref(),
        &total_supply_authority_bump,
    ];
    let new_total_supply_handle = fhe::add_with_app_pda(fhe::BinaryOpWithAppPda {
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
        rhs_acl_record: ctx.accounts.amount_compute_acl.to_account_info(),
        rhs: amount_handle,
        scalar: false,
        output_acl_record: ctx.accounts.total_supply_output_acl.to_account_info(),
        output_fhe_type: BALANCE_FHE_TYPE,
        acl_domain_key: mint_key,
        compute_signer_bump: ctx.bumps.compute_signer,
        system_program: &ctx.accounts.system_program,
        output_nonce_key: total_supply_nonce_key(mint_key, total_supply_authority),
        output_nonce_sequence: total_supply_nonce_sequence,
        output_encrypted_value_label: total_supply_label(),
        output_subjects: compute_acl_subject(compute_signer),
        output_public_decrypt: false,
    })?;

    let token_account = &mut ctx.accounts.token_account;
    token_account.balance_handle = new_balance_handle;
    token_account.balance_acl_record = ctx.accounts.output_acl.key();
    token_account.next_balance_nonce_sequence = nonce_sequence
        .checked_add(1)
        .ok_or(ConfidentialTokenError::AclNonceOverflow)?;
    let mint = &mut ctx.accounts.mint;
    mint.total_supply_handle = new_total_supply_handle;
    mint.total_supply_acl_record = ctx.accounts.total_supply_output_acl.key();
    mint.next_total_supply_nonce_sequence = total_supply_nonce_sequence
        .checked_add(1)
        .ok_or(ConfidentialTokenError::AclNonceOverflow)?;
    emit_cpi!(BalanceHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        owner: token_account.owner,
        token_account: token_account.key(),
        old_handle: old_balance_handle,
        old_acl_record: old_balance_acl_record,
        new_handle: new_balance_handle,
        new_acl_record: ctx.accounts.output_acl.key(),
        reason: BalanceHandleUpdateReason::Wrap,
    });
    emit_cpi!(TotalSupplyHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        old_handle: old_total_supply_handle,
        old_acl_record: old_total_supply_acl_record,
        new_handle: new_total_supply_handle,
        new_acl_record: ctx.accounts.total_supply_output_acl.key(),
        reason: TotalSupplyUpdateReason::Wrap,
    });
    Ok(())
}
