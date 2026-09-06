//! Joins the pending batch with a coprocessor-attested encrypted amount of the
//! batcher's join token (confidential underlying for deposit batchers,
//! confidential shares for redeem batchers — the code is direction-free).
//!
//! One user-signed transaction: the user's signature propagates through the
//! `confidential_transfer` CPI into the batch's own token account, and the batch authority's
//! `invoke_signed` signature lets the token program write the batcher a receipt in the same
//! execution: the user's joined value accumulates exactly what was transferred (`joined =
//! joined + transferred`, first join `transferred + 0`). Only the token program can compute on
//! its `transferred_amount` value, so the amount is pushed into the batcher's value rather than
//! read back — and nothing but the true transferred amount can ever be joined.

use super::*;

/// Accounts for joining a batch.
#[derive(Accounts)]
pub struct Join<'info> {
    /// Joining user; transfer authority over their confidential balance.
    pub user: Signer<'info>,
    /// Pays join-record rent, transfer output rent, and the batcher execution's
    /// ACL rent.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Batcher config.
    pub batcher: Box<Account<'info, Batcher>>,
    /// The pending batch being joined.
    #[account(mut, constraint = batch.batcher == batcher.key() @ BatcherError::BatchBatcherMismatch)]
    pub batch: Box<Account<'info, Batch>>,
    /// CHECK: per-batch authority PDA; recipient owner of the transfer and authority of the
    /// receipt the token program writes.
    #[account(seeds = [BATCH_AUTHORITY_SEED, batch.key().as_ref()], bump = batch.authority_bump)]
    pub batch_authority: UncheckedAccount<'info>,
    /// The user's join record for this batch; created on first join.
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + JoinRecord::SPACE,
        seeds = [JOIN_RECORD_SEED, batch.key().as_ref(), user.key().as_ref()],
        bump,
    )]
    pub join_record: Box<Account<'info, JoinRecord>>,
    /// Confidential mint users join batches with.
    pub join_confidential_mint: Box<Account<'info, ct::ConfidentialMint>>,
    /// CHECK: underlying SPL mint wrapped by `join_confidential_mint`. Token program is its owner.
    pub join_underlying_mint: UncheckedAccount<'info>,
    /// CHECK: ATA of `user` on `join_underlying_mint`. Uninitialized → not frozen.
    pub user_ata: UncheckedAccount<'info>,
    /// CHECK: ATA of `batch_authority` on `join_underlying_mint`. Uninitialized → not frozen.
    pub batch_authority_ata: UncheckedAccount<'info>,
    /// CHECK: user's confidential token account (transfer source); validated
    /// by the token CPI.
    #[account(mut)]
    pub user_token_account: UncheckedAccount<'info>,
    /// CHECK: batch's confidential join token account (transfer destination);
    /// validated by the token CPI and pinned below.
    #[account(mut)]
    pub batch_join_token_account: UncheckedAccount<'info>,
    /// CHECK: user's stable balance encrypted value account; replaced by the token CPI.
    #[account(mut)]
    pub user_balance_value: UncheckedAccount<'info>,
    /// CHECK: batch's stable balance encrypted value account; replaced by the token CPI.
    #[account(mut)]
    pub batch_balance_value: UncheckedAccount<'info>,
    /// CHECK: user's stable transferred-amount encrypted value account; replaced by the
    /// token CPI.
    #[account(mut)]
    pub user_transferred_value: UncheckedAccount<'info>,
    /// CHECK: the user's joined encrypted value account, the transfer's receipt; created on
    /// first join, accumulated on repeat joins. Pinned to its canonical address below.
    #[account(mut)]
    pub pending_join_value: UncheckedAccount<'info>,
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
    /// System program used for account creation.
    pub system_program: Program<'info, System>,
}

/// Transfers the attested amount into the batch account, accumulating it into the user's
/// joined encrypted value account as the transfer's receipt.
pub fn join<'info>(
    ctx: Context<'info, Join<'info>>,
    amount_attestation: zama_host::CoprocessorInputAttestation,
) -> Result<()> {
    require!(
        ctx.accounts.batch.status == BatchStatus::Pending,
        BatcherError::BatchNotPending
    );
    require_keys_eq!(
        ctx.accounts.join_confidential_mint.key(),
        ctx.accounts.batcher.join_confidential_mint,
        BatcherError::ConfidentialMintMismatch
    );
    let mint_key = ctx.accounts.join_confidential_mint.key();
    let batch_key = ctx.accounts.batch.key();
    let user = ctx.accounts.user.key();
    let batch_authority = ctx.accounts.batch_authority.key();
    require_keys_eq!(
        ctx.accounts.batch_join_token_account.key(),
        ct::token_account_address(mint_key, batch_authority).0,
        BatcherError::DerivedAccountMismatch
    );
    let joined_label = encrypted_pending_join_label(user);
    require_keys_eq!(
        ctx.accounts.pending_join_value.key(),
        batcher_encrypted_value_address(batch_key, batch_authority, joined_label).0,
        BatcherError::DerivedAccountMismatch
    );

    // The attested confidential transfer into the batch account. The user's outer signature
    // propagates as the transfer authority; the batch authority signs for the receipt. All-or-
    // zero: insufficient balance moves 0, and the receipt says so.
    let authority = BatchAuthoritySeeds::new(batch_key, ctx.accounts.batch.authority_bump);
    let authority_seeds = authority.seeds();
    ct::cpi::confidential_transfer(
        CpiContext::new_with_signer(
            ctx.accounts.confidential_token_program.key(),
            ct::cpi::accounts::ConfidentialTransfer {
                owner: ctx.accounts.user.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                mint: ctx.accounts.join_confidential_mint.to_account_info(),
                underlying_mint: ctx.accounts.join_underlying_mint.to_account_info(),
                from_ata: ctx.accounts.user_ata.to_account_info(),
                to_ata: ctx.accounts.batch_authority_ata.to_account_info(),
                from_account: ctx.accounts.user_token_account.to_account_info(),
                to_account: ctx.accounts.batch_join_token_account.to_account_info(),
                from_balance_value: ctx.accounts.user_balance_value.to_account_info(),
                to_balance_value: ctx.accounts.batch_balance_value.to_account_info(),
                transferred_amount_value: ctx.accounts.user_transferred_value.to_account_info(),
                zama_event_authority: ctx.accounts.zama_event_authority.to_account_info(),
                zama_program: ctx.accounts.zama_program.to_account_info(),
                host_config: ctx.accounts.host_config.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                hcu_block_meter: None,
                hcu_trusted_app_record: None,
                receipt_value: Some(ctx.accounts.pending_join_value.to_account_info()),
                receipt_authority: Some(ctx.accounts.batch_authority.to_account_info()),
                event_authority: ctx
                    .accounts
                    .confidential_token_event_authority
                    .to_account_info(),
                program: ctx.accounts.confidential_token_program.to_account_info(),
            },
            &[&authority_seeds],
        ),
        amount_attestation,
        Some(ct::TransferReceipt {
            program: crate::id(),
            scope: batch_key.to_bytes(),
            label: joined_label,
            authority_seeds: authority_seeds.iter().map(|seed| seed.to_vec()).collect(),
            // The user decrypts their pending amount; the batch authority computes refunds and
            // claims from it by signature.
            allows: vec![user],
        }),
    )?;

    let joined_handle = fhe::read_encrypted_value(&ctx.accounts.pending_join_value)?.current_handle;
    let record = &mut ctx.accounts.join_record;
    record.batch = batch_key;
    record.user = user;
    record.joined_encrypted_value = ctx.accounts.pending_join_value.key();
    record.bump = ctx.bumps.join_record;

    let batch = &mut ctx.accounts.batch;
    batch.join_count = batch.join_count.saturating_add(1);

    emit!(JoinedBatch {
        version: APP_EVENT_VERSION,
        batch: batch_key,
        user,
        joined_encrypted_value: ctx.accounts.pending_join_value.key(),
        joined_handle,
    });
    Ok(())
}
