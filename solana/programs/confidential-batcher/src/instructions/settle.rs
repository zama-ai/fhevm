//! Settles a dispatched batch with the KMS certificate for its burned total.
//!
//! Four legs, one instruction, all permissionless:
//! 1. `redeem_burned_amount` — the host verifies the KMS certificate on-chain
//!    (current context + exact-handle MMR public-decrypt proof) and releases
//!    the certified plain tokens to the batch authority.
//! 2. `demo_vault::deposit` — the one public number goes into the vault.
//! 3. `wrap_usdc` on the shares mint — the received shares become confidential
//!    (the wrapped amount is the already-public aggregate, nothing new leaks).
//!    "Received" is the vault-minted balance DELTA across leg 2, never the
//!    account's raw balance: SPL destinations cannot refuse transfers, so a
//!    preloaded share balance must stay unpriced and unwrapped (inert).
//! 4. Freeze `share_rate = shares_received * RATE_SCALE / total` — the single
//!    plaintext division of the whole flow.
//!
//! A zero-total batch cancels after leg 1: the certificate still proves the
//! total (so cancellation is trustless), and the division never happens.
//!
//! ## Known limitation: a dust-total batch is stuck Dispatched forever
//!
//! If the certified total is small enough that the vault floors it to zero
//! shares (`total < ~1 share's worth` at the current price), leg 2 reverts
//! with the vault's `ZeroShares` and the whole settle reverts atomically —
//! retryable, but never to success: the demo vault's share price only rises
//! (floor rounding favors the vault, `harvest` only donates, there is no loss
//! path), so the batch stays Dispatched with its deposits burned and
//! unrecoverable. The grief is cheap for an attacker holding ~all vault
//! shares: `harvest`-donating pumps the price P (the donation accrues to
//! their own shares), bricking any batch whose total is below P. The loss is
//! bounded below one share's worth per batch. Pinned by
//! `mollusk_dust_total_settle_reverts_and_batch_stays_dispatched`; the future
//! fix is a cancel-and-refund path (wrap the redeemed underlying back into
//! the batch's confidential account and refund each user's encrypted deposit
//! via `confidential_transfer_from_value`, quit's mechanism).

use super::*;

/// Accounts for settling a batch.
#[derive(Accounts)]
pub struct Settle<'info> {
    /// Pays the batch authority funding. Anyone.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Batcher config.
    pub batcher: Box<Account<'info, Batcher>>,
    /// The dispatched batch being settled.
    #[account(mut, constraint = batch.batcher == batcher.key() @ BatcherError::BatchBatcherMismatch)]
    pub batch: Box<Account<'info, Batch>>,
    /// CHECK: per-batch authority PDA; redemption recipient, vault depositor,
    /// and wrap owner via invoke_signed. Receives funding for the rent the
    /// redeem marker and wrap eval charge to the owner.
    #[account(mut, seeds = [BATCH_AUTHORITY_SEED, batch.key().as_ref()], bump = batch.authority_bump)]
    pub batch_authority: UncheckedAccount<'info>,

    // --- leg 1: redeem the KMS-certified burned total ---
    /// Confidential mint the batch total was burned on.
    pub deposit_confidential_mint: Box<Account<'info, ct::ConfidentialMint>>,
    /// CHECK: batch's confidential deposit token account; validated by the token CPI.
    pub batch_deposit_token_account: UncheckedAccount<'info>,
    /// Vault underlying SPL mint.
    pub underlying_mint: Box<Account<'info, SplMint>>,
    /// CHECK: deposit mint's underlying-token vault (canonical ATA); validated
    /// by the token CPI.
    #[account(mut)]
    pub deposit_vault_underlying: UncheckedAccount<'info>,
    /// CHECK: deposit mint's vault authority PDA; validated by the token CPI.
    pub deposit_vault_authority: UncheckedAccount<'info>,
    /// Batch's plain SPL account receiving the redeemed underlying tokens.
    #[account(mut, seeds = [BATCH_UNDERLYING_SEED, batch.key().as_ref()], bump)]
    pub batch_underlying_tokens: Box<Account<'info, TokenAccount>>,
    /// CHECK: batch's burned-amount lineage; validated by the token CPI.
    pub batch_burned_amount_value: UncheckedAccount<'info>,
    /// CHECK: per-handle redemption replay marker; created by the token CPI.
    #[account(mut)]
    pub redemption_record: UncheckedAccount<'info>,
    /// CHECK: ZamaHost config PDA; validated by host/token CPIs.
    pub host_config: UncheckedAccount<'info>,
    /// CHECK: KMS context for the host's current context id; validated by the
    /// verifier CPI.
    pub kms_context: UncheckedAccount<'info>,

    // --- leg 2: deposit the public total into the vault ---
    /// Public vault the batcher fronts.
    pub vault: Box<Account<'info, demo_vault::Vault>>,
    /// CHECK: demo-vault authority PDA; validated by the vault CPI.
    pub vault_authority: UncheckedAccount<'info>,
    /// Vault share SPL mint.
    #[account(mut)]
    pub share_mint: Box<Account<'info, SplMint>>,
    /// CHECK: vault's underlying token account; validated by the vault CPI.
    #[account(mut)]
    pub vault_token_account: UncheckedAccount<'info>,
    /// Batch's plain SPL account receiving the minted vault shares.
    #[account(mut, seeds = [BATCH_SHARE_TOKENS_SEED, batch.key().as_ref()], bump)]
    pub batch_share_tokens: Box<Account<'info, TokenAccount>>,

    // --- leg 3: wrap the received shares into confidential shares ---
    /// Confidential mint wrapping the vault's share mint.
    #[account(mut)]
    pub shares_confidential_mint: Box<Account<'info, ct::ConfidentialMint>>,
    /// CHECK: batch's confidential shares token account; validated by the token CPI.
    #[account(mut)]
    pub batch_shares_token_account: UncheckedAccount<'info>,
    /// CHECK: shares mint's underlying-token vault (canonical ATA of the share
    /// mint); validated by the token CPI.
    #[account(mut)]
    pub shares_vault_underlying: UncheckedAccount<'info>,
    /// CHECK: shares mint's vault authority PDA; validated by the token CPI.
    pub shares_vault_authority: UncheckedAccount<'info>,
    /// CHECK: shares mint compute-signer PDA; validated by the token CPI.
    pub shares_compute_signer: UncheckedAccount<'info>,
    /// CHECK: shares mint total-supply authority PDA; validated by the token CPI.
    pub shares_total_supply_authority: UncheckedAccount<'info>,
    /// CHECK: batch's confidential shares balance lineage; superseded by the wrap.
    #[account(mut)]
    pub batch_shares_balance_value: UncheckedAccount<'info>,
    /// CHECK: shares mint's total-supply lineage; superseded by the wrap.
    #[account(mut)]
    pub shares_total_supply_value: UncheckedAccount<'info>,

    /// CHECK: ZamaHost event-CPI authority; validated by the host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program (FHE compute + ACL).
    pub zama_program: Program<'info, ZamaHost>,
    /// CHECK: confidential-token event-CPI authority; validated by the token program.
    pub confidential_token_event_authority: UncheckedAccount<'info>,
    /// confidential-token program composed via CPI.
    pub confidential_token_program: Program<'info, ConfidentialToken>,
    /// demo-vault program composed via CPI.
    pub demo_vault_program: Program<'info, DemoVault>,
    /// SPL token program.
    pub token_program: Program<'info, Token>,
    /// System program used for account creation.
    pub system_program: Program<'info, System>,
}

/// Redeems, deposits, wraps, and freezes the rate — or cancels on zero total.
pub fn settle(
    ctx: Context<Settle>,
    cleartext_total: u64,
    signatures: Vec<[u8; 65]>,
    extra_data: Vec<u8>,
    proof: zama_host::instructions::MmrInclusionProof,
    authority_funding_lamports: u64,
) -> Result<()> {
    require!(
        ctx.accounts.batch.status == BatchStatus::Dispatched,
        BatcherError::BatchNotDispatched
    );
    require_keys_eq!(
        ctx.accounts.deposit_confidential_mint.key(),
        ctx.accounts.batcher.deposit_confidential_mint,
        BatcherError::ConfidentialMintMismatch
    );
    require_keys_eq!(
        ctx.accounts.shares_confidential_mint.key(),
        ctx.accounts.batcher.shares_confidential_mint,
        BatcherError::ConfidentialMintMismatch
    );
    require_keys_eq!(
        ctx.accounts.vault.key(),
        ctx.accounts.batcher.vault,
        BatcherError::VaultMismatch
    );
    let batch_key = ctx.accounts.batch.key();
    let burned_total_handle = ctx.accounts.batch.burned_total_handle;

    fund_batch_authority(
        &ctx.accounts.payer,
        &ctx.accounts.batch_authority,
        &ctx.accounts.system_program,
        authority_funding_lamports,
    )?;

    // Leg 1: on-chain KMS certificate verification + plain token release. The
    // token program asserts the certified cleartext equals `cleartext_total`
    // and writes the permanent per-handle redemption marker.
    let authority = BatchAuthoritySeeds::new(batch_key, ctx.accounts.batch.authority_bump);
    let authority_seeds = authority.seeds();
    ct::cpi::redeem_burned_amount(
        CpiContext::new_with_signer(
            ctx.accounts.confidential_token_program.key(),
            ct::cpi::accounts::RedeemBurnedAmount {
                owner: ctx.accounts.batch_authority.to_account_info(),
                mint: ctx.accounts.deposit_confidential_mint.to_account_info(),
                token_account: ctx.accounts.batch_deposit_token_account.to_account_info(),
                underlying_mint: ctx.accounts.underlying_mint.to_account_info(),
                vault_usdc: ctx.accounts.deposit_vault_underlying.to_account_info(),
                destination_usdc: ctx.accounts.batch_underlying_tokens.to_account_info(),
                vault_authority: ctx.accounts.deposit_vault_authority.to_account_info(),
                burned_amount_value: ctx.accounts.batch_burned_amount_value.to_account_info(),
                redemption_record: ctx.accounts.redemption_record.to_account_info(),
                host_config: ctx.accounts.host_config.to_account_info(),
                kms_context: ctx.accounts.kms_context.to_account_info(),
                deny_subject_record: None,
                zama_program: ctx.accounts.zama_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                event_authority: ctx
                    .accounts
                    .confidential_token_event_authority
                    .to_account_info(),
                program: ctx.accounts.confidential_token_program.to_account_info(),
            },
            &[&authority_seeds],
        ),
        burned_total_handle,
        cleartext_total,
        signatures,
        extra_data,
        proof,
    )?;

    // Zero-total batch: nothing to deposit, no rate to freeze — cancel. The
    // certificate above still proved the total, so this branch is trustless
    // and public (it branches on the certified cleartext, never on encrypted
    // state).
    if cleartext_total == 0 {
        ctx.accounts.batch.status = BatchStatus::Canceled;
        emit!(BatchCanceled {
            version: APP_EVENT_VERSION,
            batch: batch_key,
        });
        return Ok(());
    }

    // Leg 2: the one public number goes into the vault; shares come back.
    // The received shares are the vault-minted DELTA across this CPI, never
    // the account's raw balance: SPL token destinations cannot refuse
    // incoming transfers, so anyone can preload `batch_share_tokens` with
    // shares — pricing a preloaded balance would let an attacker inflate the
    // rate past u64 and brick the batch. Preloaded shares stay in the
    // account, unwrapped and unpriced (inert).
    let share_balance_before_deposit = ctx.accounts.batch_share_tokens.amount;
    demo_vault::cpi::deposit(
        CpiContext::new_with_signer(
            ctx.accounts.demo_vault_program.key(),
            demo_vault::cpi::accounts::Deposit {
                depositor: ctx.accounts.batch_authority.to_account_info(),
                vault: ctx.accounts.vault.to_account_info(),
                vault_authority: ctx.accounts.vault_authority.to_account_info(),
                underlying_mint: ctx.accounts.underlying_mint.to_account_info(),
                share_mint: ctx.accounts.share_mint.to_account_info(),
                depositor_underlying: ctx.accounts.batch_underlying_tokens.to_account_info(),
                vault_token_account: ctx.accounts.vault_token_account.to_account_info(),
                depositor_shares: ctx.accounts.batch_share_tokens.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
            &[&authority_seeds],
        ),
        cleartext_total,
    )?;
    ctx.accounts.batch_share_tokens.reload()?;
    let shares_received = ctx
        .accounts
        .batch_share_tokens
        .amount
        .checked_sub(share_balance_before_deposit)
        .ok_or(BatcherError::ShareBalanceUnderflow)?;

    // Leg 3: wrap only the vault-minted share delta into the batch's
    // confidential shares account. The wrapped amount is the already-public
    // aggregate's share value; any preloaded balance stays behind.
    ct::cpi::wrap_usdc(
        CpiContext::new_with_signer(
            ctx.accounts.confidential_token_program.key(),
            ct::cpi::accounts::WrapUsdc {
                owner: ctx.accounts.batch_authority.to_account_info(),
                mint: ctx.accounts.shares_confidential_mint.to_account_info(),
                token_account: ctx.accounts.batch_shares_token_account.to_account_info(),
                underlying_mint: ctx.accounts.share_mint.to_account_info(),
                user_usdc: ctx.accounts.batch_share_tokens.to_account_info(),
                vault_usdc: ctx.accounts.shares_vault_underlying.to_account_info(),
                vault_authority: ctx.accounts.shares_vault_authority.to_account_info(),
                compute_signer: ctx.accounts.shares_compute_signer.to_account_info(),
                total_supply_authority: ctx
                    .accounts
                    .shares_total_supply_authority
                    .to_account_info(),
                balance_value: ctx.accounts.batch_shares_balance_value.to_account_info(),
                total_supply_value: ctx.accounts.shares_total_supply_value.to_account_info(),
                zama_event_authority: ctx.accounts.zama_event_authority.to_account_info(),
                zama_program: ctx.accounts.zama_program.to_account_info(),
                host_config: ctx.accounts.host_config.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
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
        shares_received,
    )?;

    // Leg 4: freeze the batch's public rate — the one plaintext division.
    let share_rate = freeze_share_rate(shares_received, cleartext_total)?;
    let batch = &mut ctx.accounts.batch;
    batch.status = BatchStatus::Settled;
    batch.total_deposited = cleartext_total;
    batch.shares_received = shares_received;
    batch.share_rate = share_rate;

    emit!(BatchSettled {
        version: APP_EVENT_VERSION,
        batch: batch_key,
        total_deposited: cleartext_total,
        shares_received,
        share_rate,
    });
    Ok(())
}
