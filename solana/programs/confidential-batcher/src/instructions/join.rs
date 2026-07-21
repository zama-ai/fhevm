//! Joins the pending batch with a coprocessor-attested encrypted amount.
//!
//! One user-signed transaction: the user's signature propagates through the
//! `confidential_transfer` CPI into the batch's own token account, and the
//! batcher's own eval re-materializes the transferred amount into the user's
//! batch deposit lineage **in the same transaction**. Same-transaction is
//! load-bearing: the transfer's recipient rule places the batch authority in
//! the `transferred_amount` output audience by construction, but that lineage
//! is superseded by the user's next transfer and input admission pins the
//! current handle — so the re-materialization must happen before anything can
//! supersede it.

use super::*;

/// Accounts for joining a batch.
#[derive(Accounts)]
pub struct Join<'info> {
    /// Joining user; transfer authority over their confidential balance.
    pub user: Signer<'info>,
    /// Pays deposit-record rent, transfer output rent, and the batcher eval's
    /// ACL rent.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Batcher config.
    pub batcher: Box<Account<'info, Batcher>>,
    /// The pending batch being joined.
    #[account(mut, constraint = batch.batcher == batcher.key() @ BatcherError::BatchBatcherMismatch)]
    pub batch: Box<Account<'info, Batch>>,
    /// CHECK: per-batch authority PDA; recipient owner of the transfer and the
    /// batcher eval's compute subject + app authority.
    #[account(seeds = [BATCH_AUTHORITY_SEED, batch.key().as_ref()], bump = batch.authority_bump)]
    pub batch_authority: UncheckedAccount<'info>,
    /// The user's deposit record for this batch; created on first join.
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + DepositRecord::SPACE,
        seeds = [DEPOSIT_RECORD_SEED, batch.key().as_ref(), user.key().as_ref()],
        bump,
    )]
    pub deposit_record: Box<Account<'info, DepositRecord>>,
    /// Confidential mint users deposit through the batcher.
    pub deposit_confidential_mint: Box<Account<'info, ct::ConfidentialMint>>,
    /// CHECK: deposit mint compute-signer PDA; validated by the token CPI.
    pub deposit_compute_signer: UncheckedAccount<'info>,
    /// CHECK: user's confidential token account (transfer source); validated
    /// by the token CPI.
    #[account(mut)]
    pub user_token_account: UncheckedAccount<'info>,
    /// CHECK: batch's confidential deposit token account (transfer
    /// destination); validated by the token CPI and pinned below.
    #[account(mut)]
    pub batch_deposit_token_account: UncheckedAccount<'info>,
    /// CHECK: user's stable balance lineage; superseded by the token CPI.
    #[account(mut)]
    pub user_balance_value: UncheckedAccount<'info>,
    /// CHECK: batch's stable balance lineage; superseded by the token CPI.
    #[account(mut)]
    pub batch_balance_value: UncheckedAccount<'info>,
    /// CHECK: user's stable transferred-amount lineage; superseded by the
    /// token CPI, then read as the batcher eval's operand.
    #[account(mut)]
    pub user_transferred_value: UncheckedAccount<'info>,
    /// CHECK: the user's batch deposit lineage; created on first join,
    /// superseded (accumulated) on repeat joins by the batcher eval.
    #[account(mut)]
    pub pending_deposit_value: UncheckedAccount<'info>,
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

/// Transfers the attested amount into the batch account and accumulates it
/// into the user's deposit lineage, atomically.
pub fn join<'info>(
    ctx: Context<'info, Join<'info>>,
    amount_attestation: zama_host::CoprocessorInputAttestation,
) -> Result<()> {
    require!(
        ctx.accounts.batch.status == BatchStatus::Pending,
        BatcherError::BatchNotPending
    );
    require_keys_eq!(
        ctx.accounts.deposit_confidential_mint.key(),
        ctx.accounts.batcher.deposit_confidential_mint,
        BatcherError::ConfidentialMintMismatch
    );
    let mint_key = ctx.accounts.deposit_confidential_mint.key();
    let batch_key = ctx.accounts.batch.key();
    let user = ctx.accounts.user.key();
    let batch_authority = ctx.accounts.batch_authority.key();
    require_keys_eq!(
        ctx.accounts.batch_deposit_token_account.key(),
        ct::token_account_address(mint_key, batch_authority).0,
        BatcherError::DerivedAccountMismatch
    );

    // Leg 1: the attested confidential transfer into the batch account. The
    // user's outer signature propagates as the transfer authority — no
    // operator, no invoke_signed. All-or-zero: insufficient balance moves 0.
    ct::cpi::confidential_transfer(
        CpiContext::new(
            ctx.accounts.confidential_token_program.key(),
            ct::cpi::accounts::ConfidentialTransfer {
                owner: ctx.accounts.user.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                mint: ctx.accounts.deposit_confidential_mint.to_account_info(),
                from_account: ctx.accounts.user_token_account.to_account_info(),
                to_account: ctx.accounts.batch_deposit_token_account.to_account_info(),
                compute_signer: ctx.accounts.deposit_compute_signer.to_account_info(),
                from_balance_value: ctx.accounts.user_balance_value.to_account_info(),
                to_balance_value: ctx.accounts.batch_balance_value.to_account_info(),
                transferred_amount_value: ctx.accounts.user_transferred_value.to_account_info(),
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
        ),
        amount_attestation,
    )?;

    // Leg 2: re-materialize the just-transferred amount into the user's batch
    // deposit lineage. The batch authority reads the transferred lineage (it
    // is in its audience as the recipient owner) and accumulates:
    // first join creates `deposit = transferred + 0`, repeats supersede to
    // `deposit = deposit + transferred`.
    let transferred_value = fhe::read_encrypted_value(&ctx.accounts.user_transferred_value)?;
    let transferred = fhe::uint64_operand(&transferred_value)?;
    let deposit_binding = fhe::DurableBinding::bind(
        ctx.accounts.pending_deposit_value.to_account_info(),
        zama_fhe::DurableSlot::new(
            batch_key,
            batch_authority,
            zama_fhe::DurableLabel::new(pending_deposit_label(user)),
        ),
        // The user decrypts their pending deposit; the batch authority computes
        // refunds and claims from it; the deposit mint's compute signer lets
        // quit's transfer eval read it as the refund amount.
        zama_fhe::AccessPolicy::from_subjects(vec![
            zama_fhe::AccessSubject::owner(user),
            zama_fhe::AccessSubject::compute(batch_authority),
            zama_fhe::AccessSubject::compute(ctx.accounts.deposit_confidential_mint.compute_signer),
        ])
        .map_err(fhe::invalid_eval_plan)?,
    )?;
    let previous_deposit = match deposit_binding.previous_handle() {
        Some(_) => Some(fhe::uint64_operand(&fhe::read_encrypted_value(
            &ctx.accounts.pending_deposit_value,
        )?)?),
        None => None,
    };
    let context_id = zama_fhe::EvalContextId::new(
        solana_sha256_hasher::hashv(&[
            b"confidential-batcher-join-v1",
            batch_key.as_ref(),
            user.as_ref(),
            &transferred_value.current_handle,
        ])
        .to_bytes(),
    )
    .map_err(fhe::invalid_eval_plan)?;
    // The deposit and transferred lineages live in different ACL domains (the
    // batch vs the mint), so their PDAs are distinct by construction; the only
    // alias in this frame is the deposit lineage as both operand and output on
    // repeat joins, which is the standard same-slot supersede.
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
        vec![
            deposit_binding.account_info(),
            ctx.accounts.user_transferred_value.to_account_info(),
        ],
        |builder| match previous_deposit {
            Some(deposit) => builder.add(deposit, transferred, deposit_binding.output()),
            None => builder.add(
                transferred,
                zama_fhe::Scalar::<zama_fhe::Uint<64>>::u64(0),
                deposit_binding.output(),
            ),
        },
    )?;

    let deposit_handle = deposit_binding.handle_after_eval()?;
    let record = &mut ctx.accounts.deposit_record;
    record.batch = batch_key;
    record.user = user;
    record.deposit_encrypted_value = ctx.accounts.pending_deposit_value.key();
    record.bump = ctx.bumps.deposit_record;

    let batch = &mut ctx.accounts.batch;
    batch.join_count = batch.join_count.saturating_add(1);

    emit!(JoinedBatch {
        version: APP_EVENT_VERSION,
        batch: batch_key,
        user,
        deposit_encrypted_value: ctx.accounts.pending_deposit_value.key(),
        deposit_handle,
    });
    Ok(())
}
