//! Opens the next batch: the `Batch` account, its per-batch authority, its own
//! confidential token accounts (join and payout side), and its plain SPL
//! accounts for settle's redeem/vault/wrap legs.
//!
//! One batch, one token account: the burned/revealed total is exactly this
//! batch's sum, so dust from an earlier batch can never leak into it (the EVM
//! batcher documents the inter-batch dust leak this prevents).
//!
//! Direction-free: the join/payout roles are fixed by the batcher config, so
//! deposit and redeem batches open identically.

use super::*;

/// Accounts for opening a batch.
#[derive(Accounts)]
pub struct OpenBatch<'info> {
    /// Pays batch-account rent and the batch authority funding.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Batcher config; `next_batch_index` advances.
    #[account(mut)]
    pub batcher: Box<Account<'info, Batcher>>,
    /// The immediately preceding batch; required except for the first open.
    /// A new batch may not open while it is still pending.
    pub previous_batch: Option<Box<Account<'info, Batch>>>,
    /// The batch, at the batcher's next index.
    #[account(
        init,
        payer = payer,
        space = 8 + Batch::SPACE,
        seeds = [BATCH_SEED, batcher.key().as_ref(), &batcher.next_batch_index.to_le_bytes()],
        bump,
    )]
    pub batch: Box<Account<'info, Batch>>,
    /// CHECK: per-batch authority PDA; owns every batch token account, signs
    /// the batch's FHE evals and token CPIs, and pays owner-charged rent from
    /// the funding it receives here.
    #[account(mut, seeds = [BATCH_AUTHORITY_SEED, batch.key().as_ref()], bump)]
    pub batch_authority: UncheckedAccount<'info>,
    /// Confidential mint users join batches with.
    pub join_confidential_mint: Box<Account<'info, ct::ConfidentialMint>>,
    /// CHECK: join mint compute-signer PDA; validated by the token CPI.
    pub join_compute_signer: UncheckedAccount<'info>,
    /// CHECK: batch's confidential join token account; created by the token CPI.
    #[account(mut)]
    pub batch_join_token_account: UncheckedAccount<'info>,
    /// CHECK: batch join balance encrypted value account; created by the host CPI.
    #[account(mut)]
    pub batch_join_balance_value: UncheckedAccount<'info>,
    /// Confidential mint claims pay out in.
    pub payout_confidential_mint: Box<Account<'info, ct::ConfidentialMint>>,
    /// CHECK: payout mint compute-signer PDA; validated by the token CPI.
    pub payout_compute_signer: UncheckedAccount<'info>,
    /// CHECK: batch's confidential payout token account; created by the token CPI.
    #[account(mut)]
    pub batch_payout_token_account: UncheckedAccount<'info>,
    /// CHECK: batch payout balance encrypted value account; created by the host CPI.
    #[account(mut)]
    pub batch_payout_balance_value: UncheckedAccount<'info>,
    /// SPL mint the join confidential mint wraps (vault underlying for deposit
    /// batchers, vault shares for redeem batchers).
    pub join_underlying_mint: Box<Account<'info, SplMint>>,
    /// SPL mint the payout confidential mint wraps (vault shares for deposit
    /// batchers, vault underlying for redeem batchers).
    pub payout_underlying_mint: Box<Account<'info, SplMint>>,
    /// Batch's plain SPL account receiving the redeemed batch total at settle.
    #[account(
        init,
        payer = payer,
        seeds = [BATCH_JOIN_UNDERLYING_SEED, batch.key().as_ref()],
        bump,
        token::mint = join_underlying_mint,
        token::authority = batch_authority,
    )]
    pub batch_join_underlying: Box<Account<'info, TokenAccount>>,
    /// Batch's plain SPL account receiving the vault leg's output at settle.
    #[account(
        init,
        payer = payer,
        seeds = [BATCH_PAYOUT_UNDERLYING_SEED, batch.key().as_ref()],
        bump,
        token::mint = payout_underlying_mint,
        token::authority = batch_authority,
    )]
    pub batch_payout_underlying: Box<Account<'info, TokenAccount>>,
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
    /// SPL token program.
    pub token_program: Program<'info, Token>,
    /// System program used for account creation.
    pub system_program: Program<'info, System>,
}

/// Creates the batch and both of its confidential token accounts.
pub fn open_batch(ctx: Context<OpenBatch>, authority_funding_lamports: u64) -> Result<()> {
    let index = ctx.accounts.batcher.next_batch_index;
    require_keys_eq!(
        ctx.accounts.join_confidential_mint.key(),
        ctx.accounts.batcher.join_confidential_mint,
        BatcherError::ConfidentialMintMismatch
    );
    require_keys_eq!(
        ctx.accounts.payout_confidential_mint.key(),
        ctx.accounts.batcher.payout_confidential_mint,
        BatcherError::ConfidentialMintMismatch
    );
    require_keys_eq!(
        ctx.accounts.join_underlying_mint.key(),
        ctx.accounts.join_confidential_mint.underlying_mint,
        BatcherError::JoinMintVaultMismatch
    );
    require_keys_eq!(
        ctx.accounts.payout_underlying_mint.key(),
        ctx.accounts.payout_confidential_mint.underlying_mint,
        BatcherError::PayoutMintVaultMismatch
    );
    match (&ctx.accounts.previous_batch, index) {
        (None, 0) => {}
        (None, _) => return Err(BatcherError::PreviousBatchMismatch.into()),
        (Some(previous), _) => {
            require!(index > 0, BatcherError::PreviousBatchMismatch);
            require_keys_eq!(
                previous.key(),
                batch_address(ctx.accounts.batcher.key(), index - 1).0,
                BatcherError::PreviousBatchMismatch
            );
            require!(
                previous.status != BatchStatus::Pending,
                BatcherError::PreviousBatchStillPending
            );
        }
    }

    fund_batch_authority(
        &ctx.accounts.payer,
        &ctx.accounts.batch_authority,
        &ctx.accounts.system_program,
        authority_funding_lamports,
    )?;

    let batch_key = ctx.accounts.batch.key();
    let authority = BatchAuthoritySeeds::new(batch_key, ctx.bumps.batch_authority);
    let authority_seeds = authority.seeds();
    for (mint, compute_signer, token_account, balance_value) in [
        (
            &ctx.accounts.join_confidential_mint,
            &ctx.accounts.join_compute_signer,
            &ctx.accounts.batch_join_token_account,
            &ctx.accounts.batch_join_balance_value,
        ),
        (
            &ctx.accounts.payout_confidential_mint,
            &ctx.accounts.payout_compute_signer,
            &ctx.accounts.batch_payout_token_account,
            &ctx.accounts.batch_payout_balance_value,
        ),
    ] {
        ct::cpi::initialize_token_account(
            CpiContext::new_with_signer(
                ctx.accounts.confidential_token_program.key(),
                ct::cpi::accounts::InitializeTokenAccount {
                    owner: ctx.accounts.batch_authority.to_account_info(),
                    mint: mint.to_account_info(),
                    compute_signer: compute_signer.to_account_info(),
                    token_account: token_account.to_account_info(),
                    balance_encrypted_value: balance_value.to_account_info(),
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
            ),
            0,
        )?;
    }

    let opened_slot = Clock::get()?.slot;
    let batch = &mut ctx.accounts.batch;
    batch.batcher = ctx.accounts.batcher.key();
    batch.index = index;
    batch.status = BatchStatus::Pending;
    batch.opened_slot = opened_slot;
    batch.join_count = 0;
    batch.authority_bump = ctx.bumps.batch_authority;
    batch.bump = ctx.bumps.batch;
    batch.burned_total_handle = [0; 32];
    batch.total_joined = 0;
    batch.payout_received = 0;
    batch.payout_rate = 0;

    let batcher = &mut ctx.accounts.batcher;
    batcher.next_batch_index = index
        .checked_add(1)
        .ok_or(BatcherError::BatchIndexOverflow)?;

    emit!(BatchOpened {
        version: APP_EVENT_VERSION,
        batcher: batcher.key(),
        batch: batch_key,
        index,
        batch_authority: ctx.accounts.batch_authority.key(),
        opened_slot,
    });
    Ok(())
}
