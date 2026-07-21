//! Leaves a pending batch: exact refund of the recorded joined amount,
//! all-or-nothing. Direction-free, and the demo's only exit before claims:
//! quit-before-dispatch is in scope; a deadline-cancel path for batches stuck
//! after dispatch is out of scope (tracked in fhevm-internal#1773).
//!
//! The batch authority spends the user's joined lineage as the transfer
//! amount (`confidential_transfer_from_value` back to the user), then resets
//! the lineage to an encrypted zero so a later re-join accumulates from zero.
//! The refund can never partially fail: the batch account's balance is the sum
//! of all recorded joins, so `ge(balance, joined)` always holds pending.

use super::*;

/// Accounts for quitting a batch.
#[derive(Accounts)]
pub struct Quit<'info> {
    /// Quitting user; owner of the refund destination.
    pub user: Signer<'info>,
    /// Pays the transfer output rent and the reset eval's ACL rent.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Batcher config.
    pub batcher: Box<Account<'info, Batcher>>,
    /// The pending batch being quit.
    #[account(constraint = batch.batcher == batcher.key() @ BatcherError::BatchBatcherMismatch)]
    pub batch: Box<Account<'info, Batch>>,
    /// CHECK: per-batch authority PDA; transfer authority for the refund and
    /// the reset eval's compute subject + app authority.
    #[account(seeds = [BATCH_AUTHORITY_SEED, batch.key().as_ref()], bump = batch.authority_bump)]
    pub batch_authority: UncheckedAccount<'info>,
    /// The user's join record for this batch.
    #[account(
        seeds = [JOIN_RECORD_SEED, batch.key().as_ref(), user.key().as_ref()],
        bump = join_record.bump,
    )]
    pub join_record: Box<Account<'info, JoinRecord>>,
    /// Confidential mint users join batches with.
    pub join_confidential_mint: Box<Account<'info, ct::ConfidentialMint>>,
    /// CHECK: join mint compute-signer PDA; validated by the token CPI.
    pub join_compute_signer: UncheckedAccount<'info>,
    /// CHECK: batch's confidential join token account (refund source);
    /// validated by the token CPI and pinned below.
    #[account(mut)]
    pub batch_join_token_account: UncheckedAccount<'info>,
    /// CHECK: user's confidential token account (refund destination);
    /// validated by the token CPI.
    #[account(mut)]
    pub user_token_account: UncheckedAccount<'info>,
    /// CHECK: batch's stable balance lineage; superseded by the token CPI.
    #[account(mut)]
    pub batch_balance_value: UncheckedAccount<'info>,
    /// CHECK: user's stable balance lineage; superseded by the token CPI.
    #[account(mut)]
    pub user_balance_value: UncheckedAccount<'info>,
    /// CHECK: batch account's stable transferred-amount lineage (the refund is
    /// a transfer FROM the batch account); superseded by the token CPI.
    #[account(mut)]
    pub batch_transferred_value: UncheckedAccount<'info>,
    /// CHECK: the user's joined lineage; spent read-only as the refund
    /// amount, then reset to an encrypted zero by the batcher eval.
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

/// Refunds the exact recorded amount and resets the joined lineage to zero.
pub fn quit<'info>(ctx: Context<'info, Quit<'info>>) -> Result<()> {
    require!(
        ctx.accounts.batch.status == BatchStatus::Pending,
        BatcherError::BatchNotPending
    );
    require_keys_eq!(
        ctx.accounts.join_confidential_mint.key(),
        ctx.accounts.batcher.join_confidential_mint,
        BatcherError::ConfidentialMintMismatch
    );
    require_keys_eq!(
        ctx.accounts.pending_join_value.key(),
        ctx.accounts.join_record.joined_encrypted_value,
        BatcherError::DerivedAccountMismatch
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

    // Leg 1: exact refund — the joined lineage IS the transfer amount. The
    // batch authority signs via invoke_signed; the token's spend gate accepts
    // it because every joined lineage carries the batch authority in its
    // audience from birth.
    let authority = BatchAuthoritySeeds::new(batch_key, ctx.accounts.batch.authority_bump);
    let authority_seeds = authority.seeds();
    ct::cpi::confidential_transfer_from_value(CpiContext::new_with_signer(
        ctx.accounts.confidential_token_program.key(),
        ct::cpi::accounts::ConfidentialTransferFromValue {
            owner: ctx.accounts.batch_authority.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            mint: ctx.accounts.join_confidential_mint.to_account_info(),
            from_account: ctx.accounts.batch_join_token_account.to_account_info(),
            to_account: ctx.accounts.user_token_account.to_account_info(),
            compute_signer: ctx.accounts.join_compute_signer.to_account_info(),
            from_balance_value: ctx.accounts.batch_balance_value.to_account_info(),
            to_balance_value: ctx.accounts.user_balance_value.to_account_info(),
            transferred_amount_value: ctx.accounts.batch_transferred_value.to_account_info(),
            amount_value: ctx.accounts.pending_join_value.to_account_info(),
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

    // Leg 2: reset the joined lineage to an encrypted zero (supersede in
    // place), so a later re-join of the same batch accumulates from zero.
    let joined_value = fhe::read_encrypted_value(&ctx.accounts.pending_join_value)?;
    let old_handle = joined_value.current_handle;
    let reset_binding = fhe::DurableBinding::bind(
        ctx.accounts.pending_join_value.to_account_info(),
        zama_fhe::DurableSlot::new(
            batch_key,
            batch_authority,
            zama_fhe::DurableLabel::new(pending_join_label(user)),
        ),
        zama_fhe::AccessPolicy::from_subjects(vec![
            zama_fhe::AccessSubject::owner(user),
            zama_fhe::AccessSubject::compute(batch_authority),
            zama_fhe::AccessSubject::compute(ctx.accounts.join_confidential_mint.compute_signer),
        ])
        .map_err(fhe::invalid_eval_plan)?,
    )?;
    let context_id = zama_fhe::EvalContextId::new(
        solana_sha256_hasher::hashv(&[
            b"confidential-batcher-quit-v1",
            batch_key.as_ref(),
            user.as_ref(),
            &old_handle,
        ])
        .to_bytes(),
    )
    .map_err(fhe::invalid_eval_plan)?;
    fhe::eval_as_batch_authority(
        fhe::BatchAuthorityEval {
            batch: batch_key,
            authority_bump: ctx.accounts.batch.authority_bump,
            batch_authority: ctx.accounts.batch_authority.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            host_config: ctx.accounts.host_config.to_account_info(),
            zama_event_authority: ctx.accounts.zama_event_authority.to_account_info(),
            zama_program: ctx.accounts.zama_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            deny_subject_records: ctx.remaining_accounts,
        },
        context_id,
        vec![reset_binding.account_info()],
        |builder| builder.trivial_encrypt_u64(0, reset_binding.output()),
    )?;

    emit!(QuitBatch {
        version: APP_EVENT_VERSION,
        batch: batch_key,
        user,
    });
    Ok(())
}
