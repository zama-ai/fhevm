//! Dispatches a batch: burns its full encrypted balance for KMS certification.
//! Direction-free — the burn is always on the join mint (confidential
//! underlying for deposit batchers, confidential shares for redeem batchers).
//!
//! Permissionless after `min_batch_age_slots`. The batch account's own balance
//! encrypted value account IS the burn amount (`confidential_burn_from_value`'s whole-balance
//! alias, deduped inside the token program), so the born-public burned handle
//! certifies exactly this batch's sum and nothing else.

use super::*;

/// Accounts for dispatching a batch.
#[derive(Accounts)]
pub struct Dispatch<'info> {
    /// Pays the burn's output ACL rent. Anyone.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Batcher config carrying the minimum batch age.
    pub batcher: Box<Account<'info, Batcher>>,
    /// The pending batch being dispatched.
    #[account(mut, constraint = batch.batcher == batcher.key() @ BatcherError::BatchBatcherMismatch)]
    pub batch: Box<Account<'info, Batch>>,
    /// CHECK: per-batch authority PDA; burn authority via invoke_signed.
    #[account(seeds = [BATCH_AUTHORITY_SEED, batch.key().as_ref()], bump = batch.authority_bump)]
    pub batch_authority: UncheckedAccount<'info>,
    /// Confidential mint whose encrypted total supply the burn decreases.
    #[account(mut)]
    pub join_confidential_mint: Box<Account<'info, ct::ConfidentialMint>>,
    /// CHECK: join mint compute-signer PDA; validated by the token CPI.
    pub join_compute_signer: UncheckedAccount<'info>,
    /// CHECK: mint-scoped total-supply authority PDA; validated by the token CPI.
    pub total_supply_authority: UncheckedAccount<'info>,
    /// CHECK: batch's confidential join token account; validated by the
    /// token CPI and pinned below.
    #[account(mut)]
    pub batch_join_token_account: UncheckedAccount<'info>,
    /// CHECK: batch's stable balance encrypted value account — read as the burn amount AND
    /// superseded as the burn's balance output (the whole-balance alias).
    #[account(mut)]
    pub batch_balance_value: UncheckedAccount<'info>,
    /// CHECK: mint's stable total-supply encrypted value account; superseded by the token CPI.
    #[account(mut)]
    pub total_supply_value: UncheckedAccount<'info>,
    /// CHECK: batch account's burned-amount encrypted value account, born publicly
    /// decryptable; created by the token CPI (first and only burn per batch).
    #[account(mut)]
    pub batch_burned_amount_value: UncheckedAccount<'info>,
    /// CHECK: ZamaHost event-CPI authority; validated by the host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program (FHE compute + ACL).
    pub zama_program: Program<'info, ZamaHost>,
    /// CHECK: ZamaHost config PDA; validated by the host program.
    pub host_config: UncheckedAccount<'info>,
    /// CHECK: confidential-token event-CPI authority; validated by the token program.
    pub confidential_token_event_authority: UncheckedAccount<'info>,
    /// confidential-token program composed via CPI.
    pub confidential_token_program: Program<'info, ConfidentialToken>,
    /// System program used for ACL account creation.
    pub system_program: Program<'info, System>,
}

/// Burns the batch's full balance and records the burned handle to settle.
pub fn dispatch(ctx: Context<Dispatch>) -> Result<()> {
    require!(
        ctx.accounts.batch.status == BatchStatus::Pending,
        BatcherError::BatchNotPending
    );
    require_keys_eq!(
        ctx.accounts.join_confidential_mint.key(),
        ctx.accounts.batcher.join_confidential_mint,
        BatcherError::ConfidentialMintMismatch
    );
    let now = Clock::get()?.slot;
    require!(
        now >= ctx
            .accounts
            .batch
            .opened_slot
            .saturating_add(ctx.accounts.batcher.min_batch_age_slots),
        BatcherError::BatchTooYoung
    );
    let mint_key = ctx.accounts.join_confidential_mint.key();
    let batch_key = ctx.accounts.batch.key();
    let batch_authority = ctx.accounts.batch_authority.key();
    require_keys_eq!(
        ctx.accounts.batch_join_token_account.key(),
        ct::token_account_address(mint_key, batch_authority).0,
        BatcherError::DerivedAccountMismatch
    );

    let authority = BatchAuthoritySeeds::new(batch_key, ctx.accounts.batch.authority_bump);
    let authority_seeds = authority.seeds();
    ct::cpi::confidential_burn_from_value(CpiContext::new_with_signer(
        ctx.accounts.confidential_token_program.key(),
        ct::cpi::accounts::ConfidentialBurnFromValue {
            owner: ctx.accounts.batch_authority.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            mint: ctx.accounts.join_confidential_mint.to_account_info(),
            token_account: ctx.accounts.batch_join_token_account.to_account_info(),
            compute_signer: ctx.accounts.join_compute_signer.to_account_info(),
            total_supply_authority: ctx.accounts.total_supply_authority.to_account_info(),
            balance_value: ctx.accounts.batch_balance_value.to_account_info(),
            total_supply_value: ctx.accounts.total_supply_value.to_account_info(),
            burned_amount_value: ctx.accounts.batch_burned_amount_value.to_account_info(),
            // Whole-balance burn: the balance encrypted value account is also the amount.
            amount_value: ctx.accounts.batch_balance_value.to_account_info(),
            zama_event_authority: ctx.accounts.zama_event_authority.to_account_info(),
            zama_program: ctx.accounts.zama_program.to_account_info(),
            host_config: ctx.accounts.host_config.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: ctx
                .accounts
                .confidential_token_event_authority
                .to_account_info(),
            program: ctx.accounts.confidential_token_program.to_account_info(),
        },
        &[&authority_seeds],
    ))?;

    let burned_total_handle =
        fhe::read_encrypted_value(&ctx.accounts.batch_burned_amount_value)?.current_handle;
    let batch = &mut ctx.accounts.batch;
    batch.status = BatchStatus::Dispatched;
    batch.burned_total_handle = burned_total_handle;

    emit!(BatchDispatched {
        version: APP_EVENT_VERSION,
        batch: batch_key,
        burned_total_handle,
    });
    Ok(())
}
