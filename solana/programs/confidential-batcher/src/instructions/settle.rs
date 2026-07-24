//! Settles a dispatched batch with the KMS certificate for its burned total.
//!
//! Four phases, one instruction, all permissionless — and the batch lifecycle's
//! only direction branch (the vault CPI; `initialize_batcher` additionally
//! validates the mint wiring per direction at setup):
//! 1. `redeem_burned_amount` — the host verifies the KMS certificate on-chain
//!    (current context + exact-handle MMR public-decrypt proof) and releases
//!    the certified plain tokens (vault underlying for deposit batchers,
//!    vault shares for redeem batchers) to the batch authority.
//! 2. The vault phase — `demo_vault::deposit` for deposit batchers,
//!    `demo_vault::withdraw` for redeem batchers. The one public number goes
//!    through the vault; the payout units come back.
//! 3. `wrap_usdc` on the payout mint — the received payout becomes
//!    confidential (the wrapped amount is the already-public aggregate,
//!    nothing new leaks). "Received" is the balance DELTA across phase 2, never
//!    the account's raw balance: SPL destinations cannot refuse transfers, so
//!    a preloaded balance must stay unpriced and unwrapped (inert).
//! 4. Record `payout_rate = payout_received * RATE_SCALE / total_joined`
//!    (informational, saturating). Claims use exact proportional division —
//!    `joined * payout_received / total_joined` — never this rate.
//!
//! A zero-total batch cancels after phase 1: the certificate still proves the
//! total (so cancellation is trustless), and the division never happens.
//!
//! The wrap and rate phases assume `grant_deny_list_enabled = false` and no
//! binding HCU cap: every token/host CPI passes `deny_subject_record`,
//! `hcu_block_meter`, and `hcu_trusted_app_record` as hardcoded `None` (the
//! PoC host fixtures never enable them).
//!
//! ## Known limitation (deposit direction only): a dust-total batch is stuck
//! Dispatched forever
//!
//! If a DEPOSIT batch's certified total is small enough that the vault floors
//! it to zero shares (`total < ~1 share's worth` at the current price), phase 2
//! reverts with the vault's `ZeroShares` and the whole settle reverts
//! atomically — retryable, but never to success: the demo vault's share price
//! only rises (floor rounding favors the vault, `harvest` only donates, there
//! is no loss path), so the batch stays Dispatched with its deposits burned
//! and unrecoverable. The grief is cheap for an attacker holding ~all vault
//! shares: `harvest`-donating pumps the price P (the donation accrues to
//! their own shares), bricking any batch whose total is below P. The loss is
//! bounded below one share's worth per batch. Pinned by
//! `mollusk_dust_total_settle_reverts_and_batch_stays_dispatched`; the future
//! fix is a cancel-and-refund path (tracked in fhevm-internal#1773).
//!
//! REDEEM batches have no analog: the vault's share price never drops below
//! 1:1 (floor rounding favors the vault; `harvest` only raises the price), so
//! `withdraw` of any non-zero share total always returns at least that many
//! underlying units and `ZeroAssets` is unreachable. Pinned by
//! `mollusk_redeem_one_share_dust_settles_at_extreme_price`.

use super::*;

/// Accounts for settling a batch.
#[derive(Accounts)]
pub struct Settle<'info> {
    /// Pays the batch authority funding. Anyone.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Batcher config (carries the direction).
    pub batcher: Box<Account<'info, Batcher>>,
    /// The dispatched batch being settled.
    #[account(mut, constraint = batch.batcher == batcher.key() @ BatcherError::BatchBatcherMismatch)]
    pub batch: Box<Account<'info, Batch>>,
    /// CHECK: per-batch authority PDA; redemption recipient, vault
    /// depositor/withdrawer, and wrap owner via invoke_signed. Receives
    /// funding for the rent the redeem marker and wrap eval charge to the
    /// owner.
    #[account(mut, seeds = [BATCH_AUTHORITY_SEED, batch.key().as_ref()], bump = batch.authority_bump)]
    pub batch_authority: UncheckedAccount<'info>,

    // --- phase 1: redeem the KMS-certified burned total ---
    /// Confidential mint the batch total was burned on.
    pub join_confidential_mint: Box<Account<'info, ct::ConfidentialMint>>,
    /// CHECK: batch's confidential join token account; validated by the token CPI.
    pub batch_join_token_account: UncheckedAccount<'info>,
    /// SPL mint the join confidential mint wraps (vault underlying for deposit
    /// batchers, vault shares for redeem batchers). Mutable because it is the
    /// vault share mint on the redeem direction (phase 2 burns from it).
    #[account(mut)]
    pub join_underlying_mint: Box<Account<'info, SplMint>>,
    /// CHECK: join mint's underlying-token vault (canonical ATA); validated
    /// by the token CPI.
    #[account(mut)]
    pub join_mint_vault_underlying: UncheckedAccount<'info>,
    /// CHECK: join mint's vault authority PDA; validated by the token CPI.
    pub join_mint_vault_authority: UncheckedAccount<'info>,
    /// Batch's plain SPL account receiving the redeemed batch total.
    #[account(mut, seeds = [BATCH_JOIN_UNDERLYING_SEED, batch.key().as_ref()], bump)]
    pub batch_join_underlying: Box<Account<'info, TokenAccount>>,
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

    // --- phase 2: move the public total through the vault ---
    /// Public vault the batcher fronts.
    pub vault: Box<Account<'info, demo_vault::Vault>>,
    /// CHECK: demo-vault authority PDA; validated by the vault CPI.
    pub vault_authority: UncheckedAccount<'info>,
    /// CHECK: vault's underlying token account; validated by the vault CPI.
    #[account(mut)]
    pub vault_token_account: UncheckedAccount<'info>,
    /// Batch's plain SPL account receiving the vault phase's output.
    #[account(mut, seeds = [BATCH_PAYOUT_UNDERLYING_SEED, batch.key().as_ref()], bump)]
    pub batch_payout_underlying: Box<Account<'info, TokenAccount>>,

    // --- phase 3: wrap the received payout into confidential payout tokens ---
    /// Confidential mint claims pay out in.
    #[account(mut)]
    pub payout_confidential_mint: Box<Account<'info, ct::ConfidentialMint>>,
    /// SPL mint the payout confidential mint wraps (vault shares for deposit
    /// batchers, vault underlying for redeem batchers). Mutable because it is
    /// the vault share mint on the deposit direction (phase 2 mints to it).
    #[account(mut)]
    pub payout_underlying_mint: Box<Account<'info, SplMint>>,
    /// CHECK: batch's confidential payout token account; validated by the token CPI.
    #[account(mut)]
    pub batch_payout_token_account: UncheckedAccount<'info>,
    /// CHECK: payout mint's underlying-token vault (canonical ATA); validated
    /// by the token CPI.
    #[account(mut)]
    pub payout_mint_vault_underlying: UncheckedAccount<'info>,
    /// CHECK: payout mint's vault authority PDA; validated by the token CPI.
    pub payout_mint_vault_authority: UncheckedAccount<'info>,
    /// CHECK: payout mint compute-signer PDA; validated by the token CPI.
    pub payout_compute_signer: UncheckedAccount<'info>,
    /// CHECK: payout mint total-supply authority PDA; validated by the token CPI.
    pub payout_total_supply_authority: UncheckedAccount<'info>,
    /// CHECK: batch's confidential payout balance lineage; superseded by the wrap.
    #[account(mut)]
    pub batch_payout_balance_value: UncheckedAccount<'info>,
    /// CHECK: payout mint's total-supply lineage; superseded by the wrap.
    #[account(mut)]
    pub payout_total_supply_value: UncheckedAccount<'info>,

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

/// Redeems, moves the total through the vault, wraps, and records the rate —
/// or cancels on zero total.
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
        ctx.accounts.vault.key(),
        ctx.accounts.batcher.vault,
        BatcherError::VaultMismatch
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
    let batch_key = ctx.accounts.batch.key();
    let burned_total_handle = ctx.accounts.batch.burned_total_handle;

    fund_batch_authority(
        &ctx.accounts.payer,
        &ctx.accounts.batch_authority,
        &ctx.accounts.system_program,
        authority_funding_lamports,
    )?;

    // Phase 1: on-chain KMS certificate verification + plain token release. The
    // token program asserts the certified cleartext equals `cleartext_total`
    // and writes the permanent per-handle redemption marker.
    let authority = BatchAuthoritySeeds::new(batch_key, ctx.accounts.batch.authority_bump);
    let authority_seeds = authority.seeds();
    ct::cpi::redeem_burned_amount(
        CpiContext::new_with_signer(
            ctx.accounts.confidential_token_program.key(),
            ct::cpi::accounts::RedeemBurnedAmount {
                owner: ctx.accounts.batch_authority.to_account_info(),
                mint: ctx.accounts.join_confidential_mint.to_account_info(),
                token_account: ctx.accounts.batch_join_token_account.to_account_info(),
                underlying_mint: ctx.accounts.join_underlying_mint.to_account_info(),
                vault_usdc: ctx.accounts.join_mint_vault_underlying.to_account_info(),
                destination_usdc: ctx.accounts.batch_join_underlying.to_account_info(),
                vault_authority: ctx.accounts.join_mint_vault_authority.to_account_info(),
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

    // Zero-total batch: nothing to move through the vault, no rate to record —
    // cancel. The certificate above still proved the total, so this branch is
    // trustless and public (it branches on the certified cleartext, never on
    // encrypted state). The only branch the redeem direction shares verbatim.
    if cleartext_total == 0 {
        ctx.accounts.batch.status = BatchStatus::Canceled;
        emit!(BatchCanceled {
            version: APP_EVENT_VERSION,
            batch: batch_key,
        });
        return Ok(());
    }

    // Phase 2: the one public number goes through the vault; the payout comes
    // back. The received payout is the batch payout account's balance DELTA
    // across this CPI, never its raw balance: SPL token destinations cannot
    // refuse incoming transfers, so anyone can preload `batch_payout_underlying`
    // — pricing a preloaded balance would let an attacker distort the batch
    // accounting. Preloaded tokens stay in the account, unwrapped and unpriced
    // (inert).
    let payout_balance_before = ctx.accounts.batch_payout_underlying.amount;
    match ctx.accounts.batcher.direction {
        BatchDirection::Deposit => demo_vault::cpi::deposit(
            CpiContext::new_with_signer(
                ctx.accounts.demo_vault_program.key(),
                demo_vault::cpi::accounts::Deposit {
                    depositor: ctx.accounts.batch_authority.to_account_info(),
                    vault: ctx.accounts.vault.to_account_info(),
                    vault_authority: ctx.accounts.vault_authority.to_account_info(),
                    underlying_mint: ctx.accounts.join_underlying_mint.to_account_info(),
                    share_mint: ctx.accounts.payout_underlying_mint.to_account_info(),
                    depositor_underlying: ctx.accounts.batch_join_underlying.to_account_info(),
                    vault_token_account: ctx.accounts.vault_token_account.to_account_info(),
                    depositor_shares: ctx.accounts.batch_payout_underlying.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                },
                &[&authority_seeds],
            ),
            cleartext_total,
        )?,
        BatchDirection::Redeem => demo_vault::cpi::withdraw(
            CpiContext::new_with_signer(
                ctx.accounts.demo_vault_program.key(),
                demo_vault::cpi::accounts::Withdraw {
                    owner: ctx.accounts.batch_authority.to_account_info(),
                    vault: ctx.accounts.vault.to_account_info(),
                    vault_authority: ctx.accounts.vault_authority.to_account_info(),
                    underlying_mint: ctx.accounts.payout_underlying_mint.to_account_info(),
                    share_mint: ctx.accounts.join_underlying_mint.to_account_info(),
                    owner_shares: ctx.accounts.batch_join_underlying.to_account_info(),
                    vault_token_account: ctx.accounts.vault_token_account.to_account_info(),
                    owner_underlying: ctx.accounts.batch_payout_underlying.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                },
                &[&authority_seeds],
            ),
            cleartext_total,
        )?,
    }
    ctx.accounts.batch_payout_underlying.reload()?;
    let payout_received = ctx
        .accounts
        .batch_payout_underlying
        .amount
        .checked_sub(payout_balance_before)
        .ok_or(BatcherError::PayoutBalanceUnderflow)?;

    // Phase 3: wrap only the vault-produced payout delta into the batch's
    // confidential payout account. The wrapped amount is the already-public
    // aggregate's payout value; any preloaded balance stays behind.
    ct::cpi::wrap_usdc(
        CpiContext::new_with_signer(
            ctx.accounts.confidential_token_program.key(),
            ct::cpi::accounts::WrapUsdc {
                owner: ctx.accounts.batch_authority.to_account_info(),
                mint: ctx.accounts.payout_confidential_mint.to_account_info(),
                token_account: ctx.accounts.batch_payout_token_account.to_account_info(),
                underlying_mint: ctx.accounts.payout_underlying_mint.to_account_info(),
                user_usdc: ctx.accounts.batch_payout_underlying.to_account_info(),
                vault_usdc: ctx.accounts.payout_mint_vault_underlying.to_account_info(),
                vault_authority: ctx.accounts.payout_mint_vault_authority.to_account_info(),
                compute_signer: ctx.accounts.payout_compute_signer.to_account_info(),
                total_supply_authority: ctx
                    .accounts
                    .payout_total_supply_authority
                    .to_account_info(),
                balance_value: ctx.accounts.batch_payout_balance_value.to_account_info(),
                total_supply_value: ctx.accounts.payout_total_supply_value.to_account_info(),
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
        payout_received,
    )?;

    // Phase 4: record the batch's informational public rate (saturating) — the
    // one plaintext division of the whole flow, display-only.
    let payout_rate = payout_rate(payout_received, cleartext_total)?;
    let batch = &mut ctx.accounts.batch;
    batch.status = BatchStatus::Settled;
    batch.total_joined = cleartext_total;
    batch.payout_received = payout_received;
    batch.payout_rate = payout_rate;

    emit!(BatchSettled {
        version: APP_EVENT_VERSION,
        batch: batch_key,
        total_joined: cleartext_total,
        payout_received,
        payout_rate,
    });
    Ok(())
}
