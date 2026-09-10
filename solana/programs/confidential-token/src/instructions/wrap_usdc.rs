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
    pub underlying_mint: Box<InterfaceAccount<'info, SplMint>>,
    /// Owner's source USDC token account.
    #[account(
        mut,
        constraint = user_usdc.mint == underlying_mint.key() @ ConfidentialTokenError::UnderlyingMintMismatch,
        constraint = user_usdc.owner == owner.key() @ ConfidentialTokenError::OwnerMismatch
    )]
    pub user_usdc: Box<InterfaceAccount<'info, TokenAccount>>,
    /// Program vault USDC token account.
    #[account(
        mut,
        constraint = vault_usdc.mint == underlying_mint.key() @ ConfidentialTokenError::UnderlyingMintMismatch,
        constraint = vault_usdc.owner == vault_authority.key() @ ConfidentialTokenError::VaultAuthorityMismatch
    )]
    pub vault_usdc: Box<InterfaceAccount<'info, TokenAccount>>,
    /// CHECK: PDA authority for the underlying-token vault.
    #[account(seeds = [b"vault-authority", mint.key().as_ref()], bump)]
    pub vault_authority: UncheckedAccount<'info>,
    /// CHECK: Mint-scoped encrypted store authority for total-supply handles.
    #[account(seeds = [b"total-supply", mint.key().as_ref()], bump)]
    pub total_supply_authority: UncheckedAccount<'info>,
    /// Stable balance encrypted store; read for the current handle and replaced by this execution.
    #[account(mut, address = encrypted_store_address(mint.key(), token_account.key()).0)]
    pub balance_store: Box<Account<'info, zama_host::EncryptedStore>>,
    /// Stable total-supply encrypted store; read for the current handle and replaced by this execution.
    #[account(mut, address = encrypted_store_address(mint.key(), total_supply_authority_address(mint.key()).0).0)]
    pub total_supply_store: Box<Account<'info, zama_host::EncryptedStore>>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// CHECK: shared transaction transient store, validated by ZamaHost.
    #[account(mut)]
    pub transient_store: UncheckedAccount<'info>,
    /// CHECK: runtime Instructions sysvar, validated by ZamaHost.
    pub instructions: UncheckedAccount<'info>,
    /// ZamaHost program used for FHE operations.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// Classic Token or Token-2022 program owning the underlying mint and token accounts.
    pub token_program: Interface<'info, TokenInterface>,
    /// System program used for ACL account creation.
    pub system_program: Program<'info, System>,
    /// CHECK: forwarded verbatim into the ZamaHost `fhe_execute` CPI, which validates it against the
    /// canonical `["hcu-block-meter", program, mint]` PDA. Supplied by an untrusted mint under a
    /// metering-band cap; omitted when the mint is trusted or the cap is unrestricted.
    #[account(mut)]
    pub hcu_block_meter: Option<UncheckedAccount<'info>>,
    /// CHECK: forwarded verbatim into the ZamaHost `fhe_execute` CPI, which validates it against the
    /// canonical `["hcu-trusted", program, mint]` PDA. Present + valid bypasses the cap; absent
    /// means the mint is metered.
    pub hcu_trusted_app_record: Option<UncheckedAccount<'info>>,
}

/// Escrows public USDC and updates the confidential balance by `amount`.
pub fn wrap_usdc<'info>(ctx: Context<'info, WrapUsdc<'info>>, amount: u64) -> Result<()> {
    assert_confidential_mint_shape(&ctx.accounts.mint)?;
    assert_supported_underlying_mint(&ctx.accounts.underlying_mint, &ctx.accounts.token_program)?;
    assert_supported_underlying_token_account(
        &ctx.accounts.user_usdc,
        &ctx.accounts.token_program,
    )?;
    assert_supported_underlying_token_account(
        &ctx.accounts.vault_usdc,
        &ctx.accounts.token_program,
    )?;
    let mint_key = ctx.accounts.mint.key();
    let decimals = ctx.accounts.mint.decimals;
    let old_total_supply_handle =
        fhe::store_handle(&ctx.accounts.total_supply_store, total_supply_key())?;
    let token_account = ctx.accounts.token_account.as_ref();
    let old_balance_handle = fhe::store_handle(&ctx.accounts.balance_store, balance_key())?;

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
        ctx.accounts.token_program.key(),
    )?;
    let balance_authority = fhe::StoreAuthority::token_account(&ctx.accounts.token_account)?;
    let total_supply_authority_signer = fhe::StoreAuthority::total_supply(
        &ctx.accounts.total_supply_authority,
        mint_key,
        ctx.bumps.total_supply_authority,
    )?;
    let balance_output = fhe::SlotOutput::new(
        ctx.accounts.balance_store.to_account_info(),
        balance_slot(mint_key, token_account.key()),
        &balance_authority,
        [token_account.owner],
    )?;
    let total_supply_output = fhe::SlotOutput::new(
        ctx.accounts.total_supply_store.to_account_info(),
        total_supply_slot(mint_key),
        &total_supply_authority_signer,
        [],
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

    let balance = fhe::uint64_operand(&ctx.accounts.balance_store, balance_key())?;
    let total_supply = fhe::uint64_operand(&ctx.accounts.total_supply_store, total_supply_key())?;
    let execution = zama_fhe::FheExecution::build(
        zama_fhe::Store::new(&ctx.accounts.balance_store).id(),
        |builder| {
            let encrypted_amount = builder.trivial_encrypt_u64(amount)?;
            // EVM `tryIncrease` parity: encrypted add wraps mod 2^64, so clamp the credit to zero
            // when either balance or total supply cannot take `amount`. Unreachable on a 1:1 SPL
            // `u64` vault; required before any rate, fee, or extra mint.
            let max = builder.trivial_encrypt_u64(u64::MAX)?;
            let room = builder.sub(max, encrypted_amount)?;
            let balance_ok = builder.ge(room, balance)?;
            let supply_ok = builder.ge(room, total_supply)?;
            let ok = builder.and(balance_ok, supply_ok)?;
            let zero = builder.trivial_encrypt_u64(0)?;
            let added = builder.if_then_else(ok, encrypted_amount, zero)?;
            let new_balance = builder.add(balance, added)?;
            builder.output(new_balance, balance_output.output())?;
            let new_total_supply = builder.add(total_supply, added)?;
            builder.output(new_total_supply, total_supply_output.output())?;
            Ok(())
        },
    )
    .map_err(invalid_execution)?;
    let execution_accounts = fhe::ExecutionAccountSet::for_execution(
        &execution,
        [
            balance_output.account_info(),
            total_supply_output.account_info(),
        ],
        [balance_authority, total_supply_authority_signer],
    )?;
    fhe::execute(fhe::Execute {
        context: fhe::ExecuteContext {
            payer: &ctx.accounts.owner,
            event_authority: &ctx.accounts.zama_event_authority,
            transient_store: &ctx.accounts.transient_store,
            instructions: &ctx.accounts.instructions,
            zama_program: &ctx.accounts.zama_program,
            host_config: &ctx.accounts.host_config,
            deny_scope_records: fhe::deny_scope_records(
                &ctx.accounts.host_config,
                ctx.remaining_accounts,
                [token_app(mint_key)],
            )?,
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
        accounts: &execution_accounts,
        execution,
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
        old_encrypted_store: ctx.accounts.balance_store.key(),
        new_handle: new_balance_handle,
        new_encrypted_store: ctx.accounts.balance_store.key(),
        reason: BalanceHandleUpdateReason::Wrap,
    });
    emit_cpi!(TotalSupplyHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        old_handle: old_total_supply_handle,
        old_encrypted_store: ctx.accounts.total_supply_store.key(),
        new_handle: new_total_supply_handle,
        new_encrypted_store: ctx.accounts.total_supply_store.key(),
        reason: TotalSupplyUpdateReason::Wrap,
    });
    Ok(())
}
