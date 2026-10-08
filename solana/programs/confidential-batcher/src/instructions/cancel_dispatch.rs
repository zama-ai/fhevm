//! Cancels a dispatched batch burn and opens user refunds.
//!
//! This is the liveness path when a KMS certificate is unavailable or settlement cannot succeed.
//! The confidential-token CPI restores the burned amount to the batch token account and encrypted
//! total supply, closes the pending burn, and leaves the burned-amount encrypted store
//! unchanged. The batch becomes refund-only: no new joins or dispatch are accepted, while each user
//! may retrieve their recorded amount through `quit`.

use super::*;

/// Accounts for cancelling a dispatched batch burn.
#[derive(Accounts)]
pub struct CancelDispatch<'info> {
    /// Join-mint wrapper authority and optional funding payer.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Batcher config.
    pub batcher: Box<Account<'info, Batcher>>,
    /// The dispatched batch whose burn is cancelled.
    #[account(mut, constraint = batch.batcher == batcher.key() @ BatcherError::BatchBatcherMismatch)]
    pub batch: Box<Account<'info, Batch>>,
    /// CHECK: per-batch authority PDA; cancel authority via invoke_signed and pending-account rent
    /// destination. Mutable because it pays the token cancellation execution.
    #[account(mut, seeds = [BATCH_AUTHORITY_SEED, batch.key().as_ref()], bump = batch.authority_bump)]
    pub batch_authority: UncheckedAccount<'info>,
    /// Confidential mint whose encrypted total supply is restored.
    pub join_confidential_mint: Box<Account<'info, ct::ConfidentialMint>>,
    /// CHECK: mint-scoped total-supply authority PDA; validated by the token CPI.
    pub total_supply_authority: UncheckedAccount<'info>,
    /// CHECK: batch's confidential join token account; validated here and by the token CPI.
    #[account(mut)]
    pub batch_join_token_account: UncheckedAccount<'info>,
    /// CHECK: batch balance encrypted store; restored by the token CPI.
    #[account(mut)]
    pub batch_balance_store: UncheckedAccount<'info>,
    /// CHECK: mint total-supply encrypted store; restored by the token CPI.
    #[account(mut)]
    pub total_supply_store: UncheckedAccount<'info>,
    /// CHECK: pending-burn PDA for the batch token account; closed by the token CPI.
    #[account(mut)]
    pub pending_burn: UncheckedAccount<'info>,
    #[account(seeds = [zama_host::HOST_CONFIG_SEED], bump = host_config.bump, seeds::program = zama_host::ID)]
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    #[account(seeds = [b"__event_authority"], bump = zama_host::EVENT_AUTHORITY_AND_BUMP.1, seeds::program = zama_host::ID)]
    pub zama_event_authority: UncheckedAccount<'info>,
    /// CHECK: shared transaction transient store, validated by ZamaHost.
    #[account(mut)]
    pub transient_store: UncheckedAccount<'info>,
    /// CHECK: runtime Instructions sysvar, validated by ZamaHost.
    pub instructions: UncheckedAccount<'info>,
    /// ZamaHost program.
    pub zama_program: Program<'info, ZamaHost>,
    /// CHECK: confidential-token event-CPI authority; validated by the token program.
    pub confidential_token_event_authority: UncheckedAccount<'info>,
    /// Confidential-token program composed via CPI.
    pub confidential_token_program: Program<'info, ConfidentialToken>,
    /// System program used by the token execution and optional authority funding.
    pub system_program: Program<'info, System>,
    /// The join mint's HCU block meter for the token CPI. Supplied while the block cap binds and the
    /// application is not trusted.
    /// CHECK: validated by ZamaHost.
    #[account(mut)]
    pub join_mint_hcu_block_meter: Option<UncheckedAccount<'info>>,
    /// The join mint's HCU trust record for the token CPI. Supplied while the block cap binds and the
    /// application is trusted.
    /// CHECK: validated by ZamaHost.
    pub join_mint_hcu_trusted_app_record: Option<UncheckedAccount<'info>>,
}

/// Restores the dispatched burn and moves the batch into its refund-only state.
pub fn cancel_dispatch<'info>(
    ctx: Context<'info, CancelDispatch<'info>>,
    authority_funding_lamports: u64,
) -> Result<()> {
    require!(
        ctx.accounts.batch.status == BatchStatus::Dispatched,
        BatcherError::BatchNotDispatched
    );
    require_keys_eq!(
        ctx.accounts.join_confidential_mint.key(),
        ctx.accounts.batcher.join_confidential_mint,
        BatcherError::ConfidentialMintMismatch
    );
    require_keys_eq!(
        ctx.accounts.payer.key(),
        ctx.accounts.join_confidential_mint.authority,
        BatcherError::CancelAuthorityMismatch
    );

    let mint = ctx.accounts.join_confidential_mint.key();
    let batch = ctx.accounts.batch.key();
    let batch_authority = ctx.accounts.batch_authority.key();
    require_keys_eq!(
        ctx.accounts.batch_join_token_account.key(),
        ct::token_account_address(mint, batch_authority).0,
        BatcherError::DerivedAccountMismatch
    );
    require_keys_eq!(
        ctx.accounts.pending_burn.key(),
        ct::pending_burn_address(mint, ctx.accounts.batch_join_token_account.key()).0,
        BatcherError::DerivedAccountMismatch
    );

    fund_batch_authority(
        &ctx.accounts.payer,
        &ctx.accounts.batch_authority,
        &ctx.accounts.system_program,
        authority_funding_lamports,
    )?;

    let authority = BatchAuthoritySeeds::new(batch, ctx.accounts.batch.authority_bump);
    let authority_seeds = authority.seeds();
    let [cancel_deny_records] =
        split_deny_records(&ctx.accounts.host_config, ctx.remaining_accounts, [1])?;
    ct::cpi::cancel_pending_burn(
        CpiContext::new_with_signer(
            ctx.accounts.confidential_token_program.key(),
            ct::cpi::accounts::CancelPendingBurn {
                owner: ctx.accounts.batch_authority.to_account_info(),
                mint: ctx.accounts.join_confidential_mint.to_account_info(),
                token_account: ctx.accounts.batch_join_token_account.to_account_info(),
                total_supply_authority: ctx.accounts.total_supply_authority.to_account_info(),
                balance_store: ctx.accounts.batch_balance_store.to_account_info(),
                total_supply_store: ctx.accounts.total_supply_store.to_account_info(),
                pending_burn: ctx.accounts.pending_burn.to_account_info(),
                host_config: ctx.accounts.host_config.to_account_info(),
                zama_event_authority: ctx.accounts.zama_event_authority.to_account_info(),
                transient_store: ctx.accounts.transient_store.to_account_info(),
                instructions: ctx.accounts.instructions.to_account_info(),
                zama_program: ctx.accounts.zama_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                hcu_block_meter: ctx
                    .accounts
                    .join_mint_hcu_block_meter
                    .as_ref()
                    .map(|account| account.to_account_info()),
                hcu_trusted_app_record: ctx
                    .accounts
                    .join_mint_hcu_trusted_app_record
                    .as_ref()
                    .map(|account| account.to_account_info()),
                event_authority: ctx
                    .accounts
                    .confidential_token_event_authority
                    .to_account_info(),
                program: ctx.accounts.confidential_token_program.to_account_info(),
            },
            &[&authority_seeds],
        )
        .with_remaining_accounts(cancel_deny_records.to_vec()),
    )?;

    ctx.accounts.batch.status = BatchStatus::Refunding;
    ctx.accounts.batch.burned_total_handle = [0; 32];
    emit!(BatchDispatchCancelled {
        version: APP_EVENT_VERSION,
        batch,
    });
    Ok(())
}
