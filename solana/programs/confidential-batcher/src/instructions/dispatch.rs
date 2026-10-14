//! Dispatches a batch: burns its full encrypted balance for KMS certification.
//! Direction-free — the burn is always on the join mint (confidential
//! underlying for deposit batchers, confidential shares for redeem batchers).
//!
//! Permissionless after `min_batch_age_secs`. The batch account's own balance
//! encrypted store IS the burn amount (`confidential_burn_from_value`'s whole-balance
//! alias, deduped inside the token program), so the created-public burned handle
//! certifies exactly this batch's sum and nothing else.
//!
//! The same instruction opens the next batch, as EVM's `dispatchBatch` does (INVARIANTS #70).

use super::*;

/// Accounts for dispatching a batch.
#[derive(Accounts)]
pub struct Dispatch<'info> {
    /// Pays the burn's output ACL rent and opens the next batch. Anyone.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Batcher config carrying the minimum batch age; `next_batch_index` advances.
    #[account(mut)]
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
    /// SPL mint the join confidential mint wraps.
    pub join_underlying_mint: Box<Account<'info, SplMint>>,
    /// CHECK: ATA of `batch_authority` on `join_underlying_mint`. Uninitialized → not frozen.
    pub batch_authority_ata: UncheckedAccount<'info>,
    /// CHECK: mint-scoped total-supply authority PDA; validated by the token CPI.
    pub total_supply_authority: UncheckedAccount<'info>,
    /// CHECK: batch's confidential join token account; validated by the
    /// token CPI and pinned below.
    #[account(mut)]
    pub batch_join_token_account: UncheckedAccount<'info>,
    /// CHECK: batch's stable balance encrypted store — read as the burn amount AND
    /// replaced as the burn's balance output (the whole-balance alias).
    #[account(mut)]
    pub batch_balance_store: UncheckedAccount<'info>,
    /// CHECK: mint's stable total-supply encrypted store; replaced by the token CPI.
    #[account(mut)]
    pub total_supply_store: UncheckedAccount<'info>,
    /// CHECK: pending-burn PDA for the batch token account; created by the token CPI.
    #[account(mut)]
    pub pending_burn: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    #[account(seeds = [b"__event_authority"], bump = zama_host::EVENT_AUTHORITY_AND_BUMP.1, seeds::program = zama_host::ID)]
    pub zama_event_authority: UncheckedAccount<'info>,
    /// CHECK: shared transaction transient store, validated by ZamaHost.
    #[account(mut)]
    pub transient_store: UncheckedAccount<'info>,
    /// CHECK: runtime Instructions sysvar, validated by ZamaHost.
    pub instructions: UncheckedAccount<'info>,
    /// ZamaHost program (FHE compute + ACL).
    pub zama_program: Program<'info, ZamaHost>,
    #[account(seeds = [zama_host::HOST_CONFIG_SEED], bump = host_config.bump, seeds::program = zama_host::ID)]
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// CHECK: confidential-token event-CPI authority; validated by the token program.
    pub confidential_token_event_authority: UncheckedAccount<'info>,
    /// confidential-token program composed via CPI.
    pub confidential_token_program: Program<'info, ConfidentialToken>,
    /// SPL token program.
    pub token_program: Program<'info, Token>,
    /// System program used for account creation.
    pub system_program: Program<'info, System>,

    // --- the next batch, opened by this dispatch ---
    /// The next batch, at the batcher's next index.
    #[account(
        init,
        payer = payer,
        space = 8 + Batch::INIT_SPACE,
        seeds = [BATCH_SEED, batcher.key().as_ref(), &batcher.next_batch_index.to_le_bytes()],
        bump,
    )]
    pub next_batch: Box<Account<'info, Batch>>,
    /// CHECK: the next batch's authority PDA; owns its token accounts and pays their rent from the
    /// funding it receives here.
    #[account(mut, seeds = [BATCH_AUTHORITY_SEED, next_batch.key().as_ref()], bump)]
    pub next_batch_authority: UncheckedAccount<'info>,
    /// CHECK: the next batch's confidential join token account; created by the token CPI, or kept as
    /// is if someone already created it.
    #[account(mut)]
    pub next_batch_join_token_account: UncheckedAccount<'info>,
    /// CHECK: the next batch's join balance encrypted store; created by the host CPI.
    #[account(mut)]
    pub next_batch_join_balance_store: UncheckedAccount<'info>,
    /// Confidential mint claims pay out in.
    pub payout_confidential_mint: Box<Account<'info, ct::ConfidentialMint>>,
    /// SPL mint the payout confidential mint wraps.
    pub payout_underlying_mint: Box<Account<'info, SplMint>>,
    /// CHECK: the next batch's confidential payout token account; created by the token CPI, or kept
    /// as is if someone already created it.
    #[account(mut)]
    pub next_batch_payout_token_account: UncheckedAccount<'info>,
    /// CHECK: the next batch's payout balance encrypted store; created by the host CPI.
    #[account(mut)]
    pub next_batch_payout_balance_store: UncheckedAccount<'info>,
    /// The next batch's plain SPL account receiving its redeemed total at settle.
    #[account(
        init,
        payer = payer,
        seeds = [BATCH_JOIN_UNDERLYING_SEED, next_batch.key().as_ref()],
        bump,
        token::mint = join_underlying_mint,
        token::authority = next_batch_authority,
    )]
    pub next_batch_join_underlying: Box<Account<'info, TokenAccount>>,
    /// The next batch's plain SPL account receiving its vault phase's output at settle.
    #[account(
        init,
        payer = payer,
        seeds = [BATCH_PAYOUT_UNDERLYING_SEED, next_batch.key().as_ref()],
        bump,
        token::mint = payout_underlying_mint,
        token::authority = next_batch_authority,
    )]
    pub next_batch_payout_underlying: Box<Account<'info, TokenAccount>>,
    /// The join mint's HCU block meter for the burn and the next batch's join token account.
    /// Supplied while the block cap binds and the application is not trusted.
    /// CHECK: validated by ZamaHost.
    #[account(mut)]
    pub join_mint_hcu_block_meter: Option<UncheckedAccount<'info>>,
    /// The join mint's HCU trust record for the burn and the next batch's join token account.
    /// Supplied while the block cap binds and the application is trusted.
    /// CHECK: validated by ZamaHost.
    pub join_mint_hcu_trusted_app_record: Option<UncheckedAccount<'info>>,
    /// The payout mint's HCU block meter for the next batch's payout token account. Supplied while
    /// the block cap binds and the application is not trusted.
    /// CHECK: validated by ZamaHost.
    #[account(mut)]
    pub payout_mint_hcu_block_meter: Option<UncheckedAccount<'info>>,
    /// The payout mint's HCU trust record for the next batch's payout token account. Supplied while
    /// the block cap binds and the application is trusted.
    /// CHECK: validated by ZamaHost.
    pub payout_mint_hcu_trusted_app_record: Option<UncheckedAccount<'info>>,
}

/// Burns the batch's full balance, records the burned handle to settle, and opens the next batch.
pub fn dispatch<'info>(
    ctx: Context<'info, Dispatch<'info>>,
    authority_funding_lamports: u64,
) -> Result<()> {
    require_not_paused(&ctx.accounts.batcher)?;
    require!(
        ctx.accounts.batch.status == BatchStatus::Pending,
        BatcherError::BatchNotPending
    );
    let now = Clock::get()?.unix_timestamp;
    require!(
        now >= ctx.accounts.batch.dispatchable_at(),
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
    require_keys_eq!(
        ctx.accounts.pending_burn.key(),
        ct::pending_burn_address(mint_key, ctx.accounts.batch_join_token_account.key()).0,
        BatcherError::DerivedAccountMismatch
    );
    let [burn_deny_records, join_deny_records, payout_deny_records] =
        split_deny_records(&ctx.accounts.host_config, ctx.remaining_accounts, [1, 1, 1])?;
    ct::cpi::confidential_burn_from_value(
        CpiContext::new_with_signer(
            ctx.accounts.confidential_token_program.key(),
            ct::cpi::accounts::ConfidentialBurnFromValue {
                owner: ctx.accounts.batch_authority.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                mint: ctx.accounts.join_confidential_mint.to_account_info(),
                underlying_mint: ctx.accounts.join_underlying_mint.to_account_info(),
                owner_ata: ctx.accounts.batch_authority_ata.to_account_info(),
                token_account: ctx.accounts.batch_join_token_account.to_account_info(),
                total_supply_authority: ctx.accounts.total_supply_authority.to_account_info(),
                balance_store: ctx.accounts.batch_balance_store.to_account_info(),
                total_supply_store: ctx.accounts.total_supply_store.to_account_info(),
                pending_burn: ctx.accounts.pending_burn.to_account_info(),
                // Whole-balance burn: the balance encrypted store is also the amount.
                amount_store: ctx.accounts.batch_balance_store.to_account_info(),
                zama_event_authority: ctx.accounts.zama_event_authority.to_account_info(),
                transient_store: ctx.accounts.transient_store.to_account_info(),
                instructions: ctx.accounts.instructions.to_account_info(),
                zama_program: ctx.accounts.zama_program.to_account_info(),
                host_config: ctx.accounts.host_config.to_account_info(),
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
        .with_remaining_accounts(burn_deny_records.to_vec()),
        ct::balance_key(),
    )?;

    let burned_total_handle = fhe::read_state(&ctx.accounts.batch_balance_store)?
        .get(&ct::burned_amount_key())
        .ok_or(BatcherError::EncryptedStoreInvalid)?;
    let batch = &mut ctx.accounts.batch;
    batch.status = BatchStatus::Dispatched;
    batch.dispatched_at = now;
    batch.burned_total_handle = burned_total_handle;

    emit!(BatchDispatched {
        batch: batch_key,
        burned_total_handle,
    });

    let accounts = &mut *ctx.accounts;
    open_batch(
        OpenBatch {
            payer: &accounts.payer,
            batcher: &mut accounts.batcher,
            batch: &mut accounts.next_batch,
            batch_authority: &accounts.next_batch_authority,
            batch_authority_bump: ctx.bumps.next_batch_authority,
            join_confidential_mint: &accounts.join_confidential_mint,
            join_underlying_mint: &accounts.join_underlying_mint,
            batch_join_token_account: &accounts.next_batch_join_token_account,
            batch_join_balance_store: &accounts.next_batch_join_balance_store,
            join_mint_hcu_block_meter: &accounts.join_mint_hcu_block_meter,
            join_mint_hcu_trusted_app_record: &accounts.join_mint_hcu_trusted_app_record,
            payout_confidential_mint: &accounts.payout_confidential_mint,
            payout_underlying_mint: &accounts.payout_underlying_mint,
            batch_payout_token_account: &accounts.next_batch_payout_token_account,
            batch_payout_balance_store: &accounts.next_batch_payout_balance_store,
            payout_mint_hcu_block_meter: &accounts.payout_mint_hcu_block_meter,
            payout_mint_hcu_trusted_app_record: &accounts.payout_mint_hcu_trusted_app_record,
            zama_event_authority: &accounts.zama_event_authority,
            transient_store: &accounts.transient_store,
            instructions: &accounts.instructions,
            zama_program: &accounts.zama_program,
            host_config: &accounts.host_config,
            confidential_token_event_authority: &accounts.confidential_token_event_authority,
            confidential_token_program: &accounts.confidential_token_program,
            system_program: &accounts.system_program,
        },
        authority_funding_lamports,
        [join_deny_records, payout_deny_records],
    )
}
