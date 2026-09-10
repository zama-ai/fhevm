//! Cancels a pending burn by FHE-crediting the burned amount back onto confidential
//! balance and encrypted total supply.
//!
//! Alternate settlement path for a `PendingBurn` (fhevm-internal#1862 Wave 2). The single-pending
//! invariant keeps the burned handle current until the owner either redeems or cancels it. Cancel
//! reverses the burn's encrypted balance and supply effects without a KMS certificate.
//!
//! Rent note: `open_pending_burn` may be paid by a permissionless `payer` (e.g. batcher dispatch),
//! while redeem/cancel always close to `owner`. That is intentional crank subsidy, not theft.

use super::*;

/// Accounts for cancelling a pending burn into confidential balance and total supply.
#[derive(Accounts)]
#[event_cpi]
pub struct CancelPendingBurn<'info> {
    /// Token owner, cancel authority, and rent destination for the closed pending-burn account.
    #[account(mut)]
    pub owner: Signer<'info>,
    /// Confidential mint whose total supply is re-credited.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Token account whose balance is re-credited.
    #[account(mut)]
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// CHECK: Mint-scoped encrypted State authority for total-supply handles.
    #[account(seeds = [b"total-supply", mint.key().as_ref()], bump)]
    pub total_supply_authority: UncheckedAccount<'info>,
    /// Stable balance encrypted State; read for the current handle and replaced by this execution.
    #[account(mut, address = encrypted_state_address(mint.key(), token_account.key()).0)]
    pub balance_state: Box<Account<'info, zama_host::EncryptedState>>,
    /// Stable total-supply encrypted State; read for the current handle and replaced by this execution.
    #[account(mut, address = encrypted_state_address(mint.key(), total_supply_authority_address(mint.key()).0).0)]
    pub total_supply_state: Box<Account<'info, zama_host::EncryptedState>>,
    /// Pending-burn account; closed on successful cancellation.
    #[account(
        mut,
        close = owner,
        seeds = [
            PENDING_BURN_SEED,
            mint.key().as_ref(),
            token_account.key().as_ref()
        ],
        bump = pending_burn.bump,
    )]
    pub pending_burn: Account<'info, PendingBurn>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// CHECK: shared transaction scratch, validated by ZamaHost.
    #[account(mut)]
    pub scratch: UncheckedAccount<'info>,
    /// CHECK: runtime Instructions sysvar, validated by ZamaHost.
    pub instructions: UncheckedAccount<'info>,
    /// ZamaHost program used for FHE operations.
    pub zama_program: Program<'info, ZamaHost>,
    /// System program used for ACL account creation on the balance/supply update path.
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

/// Re-credits the pending burned amount onto the owner's confidential balance and encrypted total
/// supply, then closes the pending account.
pub fn cancel_pending_burn<'info>(ctx: Context<'info, CancelPendingBurn<'info>>) -> Result<()> {
    assert_confidential_mint_shape(&ctx.accounts.mint)?;
    let mint_key = ctx.accounts.mint.key();
    let token_account = ctx.accounts.token_account.as_ref();
    let owner = token_account.owner;
    let token_account_key = token_account.key();
    let pending = &ctx.accounts.pending_burn;

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
        pending.owner,
        owner,
        ConfidentialTokenError::PendingBurnMismatch
    );
    require_keys_eq!(
        pending.mint,
        mint_key,
        ConfidentialTokenError::PendingBurnMismatch
    );
    require_keys_eq!(
        pending.token_account,
        token_account_key,
        ConfidentialTokenError::PendingBurnMismatch
    );

    let burned_value = fhe::read_state(&ctx.accounts.balance_state.to_account_info())?;
    require!(
        pending.burned_handle == fhe::state_handle(&burned_value, burned_amount_key())?,
        ConfidentialTokenError::PendingBurnHandleNotCurrent
    );

    let old_balance_handle = fhe::state_handle(&ctx.accounts.balance_state, balance_key())?;
    let old_total_supply_handle =
        fhe::state_handle(&ctx.accounts.total_supply_state, total_supply_key())?;
    let token_authority = fhe::StateAuthority::token_account(&ctx.accounts.token_account)?;
    let total_supply_authority = fhe::StateAuthority::total_supply(
        &ctx.accounts.total_supply_authority,
        mint_key,
        ctx.bumps.total_supply_authority,
    )?;
    let balance_output = fhe::SlotOutput::new(
        ctx.accounts.balance_state.to_account_info(),
        balance_slot(mint_key, token_account_key),
        &token_authority,
        [owner],
    )?;
    let total_supply_output = fhe::SlotOutput::new(
        ctx.accounts.total_supply_state.to_account_info(),
        total_supply_slot(mint_key),
        &total_supply_authority,
        [],
    )?;
    let balance = fhe::uint64_operand(&ctx.accounts.balance_state, balance_key())?;
    let total_supply = fhe::uint64_operand(&ctx.accounts.total_supply_state, total_supply_key())?;
    let burned_amount = fhe::uint64_operand(&burned_value, burned_amount_key())?;

    let execution = zama_fhe::FheExecution::build(
        zama_fhe::State::new(&ctx.accounts.balance_state).id(),
        |builder| {
            let new_balance = builder.add(balance, burned_amount)?;
            builder.output(new_balance, balance_output.output())?;
            let new_total_supply = builder.add(total_supply, burned_amount)?;
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
        [token_authority, total_supply_authority],
    )?;
    fhe::execute(fhe::Execute {
        context: fhe::ExecuteContext {
            payer: &ctx.accounts.owner,
            event_authority: &ctx.accounts.zama_event_authority,
            scratch: &ctx.accounts.scratch,
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

    emit_cpi!(BalanceHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        owner,
        token_account: token_account_key,
        old_handle: old_balance_handle,
        old_encrypted_state: ctx.accounts.balance_state.key(),
        new_handle: new_balance_handle,
        new_encrypted_state: ctx.accounts.balance_state.key(),
        reason: BalanceHandleUpdateReason::CancelBurn,
    });
    emit_cpi!(TotalSupplyHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        old_handle: old_total_supply_handle,
        old_encrypted_state: ctx.accounts.total_supply_state.key(),
        new_handle: new_total_supply_handle,
        new_encrypted_state: ctx.accounts.total_supply_state.key(),
        reason: TotalSupplyUpdateReason::CancelBurn,
    });
    emit_cpi!(PendingBurnCancelledEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        owner,
        token_account: token_account_key,
        burned_handle: pending.burned_handle,
        burned_encrypted_state: ctx.accounts.balance_state.key(),
    });
    Ok(())
}
