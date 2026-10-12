//! Creates the batcher config wiring the two confidential mints and the vault
//! for one direction, and opens its batch 0.

use super::*;

/// Accounts for initializing a batcher.
#[derive(Accounts)]
pub struct InitializeBatcher<'info> {
    /// Rent payer and the account that submits the initialization; also opens batch 0.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Batcher config account, created here. A fresh keypair signs its own
    /// creation (the demo-vault `Vault` pattern).
    #[account(init, payer = payer, space = 8 + Batcher::INIT_SPACE)]
    pub batcher: Box<Account<'info, Batcher>>,
    /// Public vault the batcher fronts.
    pub vault: Box<Account<'info, demo_vault::Vault>>,
    /// Batch 0, opened with the batcher.
    #[account(
        init,
        payer = payer,
        space = 8 + Batch::INIT_SPACE,
        seeds = [BATCH_SEED, batcher.key().as_ref(), &0u64.to_le_bytes()],
        bump,
    )]
    pub batch: Box<Account<'info, Batch>>,
    /// CHECK: per-batch authority PDA; owns every batch token account, signs
    /// the batch's fhe_execute CPIs and token CPIs, and pays owner-charged rent from
    /// the funding it receives here.
    #[account(mut, seeds = [BATCH_AUTHORITY_SEED, batch.key().as_ref()], bump)]
    pub batch_authority: UncheckedAccount<'info>,
    /// Confidential mint users join batches with.
    pub join_confidential_mint: Box<Account<'info, ct::ConfidentialMint>>,
    /// CHECK: batch's confidential join token account; created by the token CPI, or kept as is if
    /// someone already created it.
    #[account(mut)]
    pub batch_join_token_account: UncheckedAccount<'info>,
    /// CHECK: batch join balance encrypted store; created by the host CPI.
    #[account(mut)]
    pub batch_join_balance_store: UncheckedAccount<'info>,
    /// Confidential mint claims pay out in.
    pub payout_confidential_mint: Box<Account<'info, ct::ConfidentialMint>>,
    /// CHECK: batch's confidential payout token account; created by the token CPI, or kept as is if
    /// someone already created it.
    #[account(mut)]
    pub batch_payout_token_account: UncheckedAccount<'info>,
    /// CHECK: batch payout balance encrypted store; created by the host CPI.
    #[account(mut)]
    pub batch_payout_balance_store: UncheckedAccount<'info>,
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
    /// Batch's plain SPL account receiving the vault phase's output at settle.
    #[account(
        init,
        payer = payer,
        seeds = [BATCH_PAYOUT_UNDERLYING_SEED, batch.key().as_ref()],
        bump,
        token::mint = payout_underlying_mint,
        token::authority = batch_authority,
    )]
    pub batch_payout_underlying: Box<Account<'info, TokenAccount>>,
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
    /// The join mint's HCU block meter for the token CPI. Supplied while the block cap binds and the
    /// application is not trusted.
    /// CHECK: validated by ZamaHost.
    #[account(mut)]
    pub join_mint_hcu_block_meter: Option<UncheckedAccount<'info>>,
    /// The join mint's HCU trust record for the token CPI. Supplied while the block cap binds and the
    /// application is trusted.
    /// CHECK: validated by ZamaHost.
    pub join_mint_hcu_trusted_app_record: Option<UncheckedAccount<'info>>,
    /// The payout mint's HCU block meter for the token CPI. Supplied while the block cap binds and the
    /// application is not trusted.
    /// CHECK: validated by ZamaHost.
    #[account(mut)]
    pub payout_mint_hcu_block_meter: Option<UncheckedAccount<'info>>,
    /// The payout mint's HCU trust record for the token CPI. Supplied while the block cap binds and the
    /// application is trusted.
    /// CHECK: validated by ZamaHost.
    pub payout_mint_hcu_trusted_app_record: Option<UncheckedAccount<'info>>,
}

/// Validates the direction's mint/vault wiring, records the batcher config and opens batch 0.
pub fn initialize_batcher<'info>(
    ctx: Context<'info, InitializeBatcher<'info>>,
    min_batch_age_secs: u64,
    settle_deadline_secs: u64,
    direction: BatchDirection,
    authority_funding_lamports: u64,
    admin: Pubkey,
) -> Result<()> {
    require_valid_min_batch_age(min_batch_age_secs)?;
    require_valid_settle_deadline(settle_deadline_secs)?;
    require_keys_neq!(admin, Pubkey::default(), BatcherError::InvalidAdmin);
    // The join mint must wrap what the batch total goes INTO the vault as,
    // and the payout mint what comes back OUT.
    let (join_vault_mint, payout_vault_mint) = match direction {
        BatchDirection::Deposit => (
            ctx.accounts.vault.underlying_mint,
            ctx.accounts.vault.share_mint,
        ),
        BatchDirection::Redeem => (
            ctx.accounts.vault.share_mint,
            ctx.accounts.vault.underlying_mint,
        ),
    };
    require_keys_eq!(
        ctx.accounts.join_confidential_mint.underlying_mint,
        join_vault_mint,
        BatcherError::JoinMintVaultMismatch
    );
    require_keys_eq!(
        ctx.accounts.payout_confidential_mint.underlying_mint,
        payout_vault_mint,
        BatcherError::PayoutMintVaultMismatch
    );

    let batcher = &mut ctx.accounts.batcher;
    batcher.direction = direction;
    batcher.join_confidential_mint = ctx.accounts.join_confidential_mint.key();
    batcher.payout_confidential_mint = ctx.accounts.payout_confidential_mint.key();
    batcher.vault = ctx.accounts.vault.key();
    batcher.admin = admin;
    batcher.paused = false;
    batcher.min_batch_age_secs = min_batch_age_secs;
    batcher.settle_deadline_secs = settle_deadline_secs;
    batcher.next_batch_index = 0;

    emit!(BatcherInitialized {
        version: APP_EVENT_VERSION,
        batcher: batcher.key(),
        direction,
        join_confidential_mint: batcher.join_confidential_mint,
        payout_confidential_mint: batcher.payout_confidential_mint,
        vault: batcher.vault,
        admin,
        min_batch_age_secs,
        settle_deadline_secs,
    });

    let deny_records =
        split_deny_records(&ctx.accounts.host_config, ctx.remaining_accounts, [1, 1])?;
    let accounts = &mut *ctx.accounts;
    open_batch(
        OpenBatch {
            payer: &accounts.payer,
            batcher: &mut accounts.batcher,
            batch: &mut accounts.batch,
            batch_bump: ctx.bumps.batch,
            batch_authority: &accounts.batch_authority,
            batch_authority_bump: ctx.bumps.batch_authority,
            join_confidential_mint: &accounts.join_confidential_mint,
            join_underlying_mint: &accounts.join_underlying_mint,
            batch_join_token_account: &accounts.batch_join_token_account,
            batch_join_balance_store: &accounts.batch_join_balance_store,
            join_mint_hcu_block_meter: &accounts.join_mint_hcu_block_meter,
            join_mint_hcu_trusted_app_record: &accounts.join_mint_hcu_trusted_app_record,
            payout_confidential_mint: &accounts.payout_confidential_mint,
            payout_underlying_mint: &accounts.payout_underlying_mint,
            batch_payout_token_account: &accounts.batch_payout_token_account,
            batch_payout_balance_store: &accounts.batch_payout_balance_store,
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
        deny_records,
    )
}
