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
    /// Confidential mint whose compute signer authorizes the balance/supply updates.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Token account whose balance is re-credited.
    #[account(mut)]
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    /// CHECK: Mint-scoped encrypted value account authority for total-supply handles.
    #[account(seeds = [b"total-supply", mint.key().as_ref()], bump)]
    pub total_supply_authority: UncheckedAccount<'info>,
    /// Stable balance encrypted value account; read for the current handle and replaced by this execution.
    #[account(mut, address = token_account.balance_encrypted_value)]
    pub balance_value: Box<Account<'info, zama_host::EncryptedValue>>,
    /// Stable total-supply encrypted value account; read for the current handle and replaced by this execution.
    #[account(mut, address = mint.total_supply_encrypted_value)]
    pub total_supply_value: Box<Account<'info, zama_host::EncryptedValue>>,
    /// Shared `burned_amount` encrypted value account; read as a persistent operand (left unchanged).
    #[account(address = encrypted_value_address(mint.key(), token_account.key(), encrypted_burned_amount_label()).0)]
    pub burned_amount_value: Box<Account<'info, zama_host::EncryptedValue>>,
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
    /// ZamaHost program used for FHE operations.
    pub zama_program: Program<'info, ZamaHost>,
    /// System program used for ACL account creation on the balance/supply update path.
    pub system_program: Program<'info, System>,
    /// CHECK: forwarded verbatim into the ZamaHost `fhe_execute` CPI, which validates it against the
    /// canonical `["hcu-block-meter", compute_signer]` PDA. Supplied by an untrusted mint under a
    /// metering-band cap; omitted when the mint is trusted or the cap is unrestricted.
    #[account(mut)]
    pub hcu_block_meter: Option<UncheckedAccount<'info>>,
    /// CHECK: forwarded verbatim into the ZamaHost `fhe_execute` CPI, which validates it against the
    /// canonical `["hcu-trusted", compute_signer]` PDA. Present + valid bypasses the cap; absent
    /// means the mint is metered.
    pub hcu_trusted_app_record: Option<UncheckedAccount<'info>>,
}

/// Re-credits the pending burned amount onto the owner's confidential balance and encrypted total
/// supply, then closes the pending account.
pub fn cancel_pending_burn<'info>(ctx: Context<'info, CancelPendingBurn<'info>>) -> Result<()> {
    assert_confidential_mint_shape(&ctx.accounts.mint)?;
    let mint_key = ctx.accounts.mint.key();
    let compute_signer = ctx.accounts.mint.compute_signer;
    let total_supply_authority = ctx.accounts.total_supply_authority.key();
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
        ctx.accounts.compute_signer.key(),
        compute_signer,
        ConfidentialTokenError::ComputeSignerMismatch
    );
    require_keys_eq!(
        total_supply_authority,
        total_supply_authority_address(mint_key).0,
        ConfidentialTokenError::TotalSupplyAuthorityMismatch
    );

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
    require_keys_eq!(
        pending.burned_encrypted_value,
        ctx.accounts.burned_amount_value.key(),
        ConfidentialTokenError::PendingBurnMismatch
    );

    let burned_value =
        fhe::read_encrypted_value(&ctx.accounts.burned_amount_value.to_account_info())?;
    require!(
        pending.burned_handle == burned_value.current_handle,
        ConfidentialTokenError::PendingBurnHandleNotCurrent
    );

    let mint_domain = zama_fhe::Domain::new(mint_key);
    let old_balance_handle = ctx.accounts.balance_value.current_handle;
    let old_total_supply_handle = ctx.accounts.total_supply_value.current_handle;
    let balance_output = fhe::PersistentOutput::new(
        ctx.accounts.balance_value.to_account_info(),
        encrypted_value_id(mint_domain, token_account_key, encrypted_balance_label()),
        fhe::PersistentAudience::for_owner(owner, compute_signer),
    )?;
    let total_supply_output = fhe::PersistentOutput::new(
        ctx.accounts.total_supply_value.to_account_info(),
        encrypted_value_id(
            mint_domain,
            total_supply_authority,
            encrypted_total_supply_label(),
        ),
        fhe::PersistentAudience::compute_only(compute_signer),
    )?;
    let balance = uint64_from_value(
        old_balance_handle,
        mint_domain,
        token_account_key,
        encrypted_balance_label(),
    )?;
    let total_supply = uint64_from_value(
        old_total_supply_handle,
        mint_domain,
        total_supply_authority,
        encrypted_total_supply_label(),
    )?;
    let burned_amount = uint64_from_value(
        burned_value.current_handle,
        zama_fhe::Domain::new(burned_value.domain),
        burned_value.encrypted_value_account_authority,
        burned_value.label,
    )?;

    let execution = zama_fhe::FheExecution::build(
        zama_fhe::ExecutionEncryptedValueAccountAuthority::new(token_account_key),
        |builder| {
            builder.add(balance, burned_amount, balance_output.output())?;
            builder.add(total_supply, burned_amount, total_supply_output.output())?;
            Ok(())
        },
    )
    .map_err(invalid_execution)?;
    let compute_authority = fhe::ComputeAuthority::for_mint(
        &ctx.accounts.compute_signer,
        mint_key,
        ctx.bumps.compute_signer,
    )?;
    let total_supply_authority_bump = total_supply_authority_address(mint_key).1;
    let execution_accounts = fhe::ExecutionAccountSet::for_execution(
        &execution,
        [
            balance_output.account_info(),
            total_supply_output.account_info(),
            ctx.accounts.burned_amount_value.to_account_info(),
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
    fhe::execute(fhe::Execute {
        context: fhe::ExecuteContext {
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
        old_encrypted_value: ctx.accounts.balance_value.key(),
        new_handle: new_balance_handle,
        new_encrypted_value: ctx.accounts.balance_value.key(),
        reason: BalanceHandleUpdateReason::CancelBurn,
    });
    emit_cpi!(TotalSupplyHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        old_handle: old_total_supply_handle,
        old_encrypted_value: ctx.accounts.total_supply_value.key(),
        new_handle: new_total_supply_handle,
        new_encrypted_value: ctx.accounts.total_supply_value.key(),
        reason: TotalSupplyUpdateReason::CancelBurn,
    });
    emit_cpi!(PendingBurnCancelledEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        owner,
        token_account: token_account_key,
        burned_handle: pending.burned_handle,
        burned_encrypted_value: ctx.accounts.burned_amount_value.key(),
    });
    Ok(())
}
