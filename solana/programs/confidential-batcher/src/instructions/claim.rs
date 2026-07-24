//! Claims a user's confidential payout from a settled batch. Direction-free:
//! the payout mint is confidential shares for deposit batchers, confidential
//! underlying for redeem batchers.
//!
//! One MulDiv eval frame — the exact proportional floor
//! `encrypted(joined) x payout_received / total_joined` — then a confidential
//! transfer of the resulting handle from the batch's payout account to the
//! user. Permissionless pull: anyone can trigger a user's claim; the payout
//! can only land in the user's own account.
//!
//! Rounding guarantees delivery: every claim floors its exact share of the
//! aggregate, so the sum of all claims never exceeds the wrapped payout and
//! the all-or-zero transfer always moves the full claim. Exact division (not
//! the informational `payout_rate`) avoids the double rounding that stranded
//! up to `RATE_SCALE`-scale dust per batch at u64 amounts. The MulDiv's
//! intermediate `joined * payout_received < 2^128` stays inside the
//! coprocessor's widened MulDiv, and the result is at most `payout_received`,
//! so it fits euint64. `total_joined > 0` because zero-total batches cancel.
//!
//! The eval and transfer assume `grant_deny_list_enabled = false` and no
//! binding HCU cap: `hcu_block_meter` and `hcu_trusted_app_record` are
//! hardcoded `None` (the PoC host fixtures never enable them), and deny-list
//! records ride in as the (empty) remaining accounts.

use super::*;

/// Accounts for claiming a payout.
#[derive(Accounts)]
pub struct Claim<'info> {
    /// Pays the claim encrypted value account and transfer output rent. Anyone.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: the user being claimed for; pinned by the join record's PDA
    /// seeds. Not a signer — claims are permissionless pulls.
    pub user: UncheckedAccount<'info>,
    /// Batcher config.
    pub batcher: Box<Account<'info, Batcher>>,
    /// The settled batch being claimed from.
    #[account(constraint = batch.batcher == batcher.key() @ BatcherError::BatchBatcherMismatch)]
    pub batch: Box<Account<'info, Batch>>,
    /// CHECK: per-batch authority PDA; the claim eval's compute subject + app
    /// authority and the payout transfer's authority via invoke_signed.
    #[account(seeds = [BATCH_AUTHORITY_SEED, batch.key().as_ref()], bump = batch.authority_bump)]
    pub batch_authority: UncheckedAccount<'info>,
    /// The user's join record; marked claimed here.
    #[account(
        mut,
        seeds = [JOIN_RECORD_SEED, batch.key().as_ref(), user.key().as_ref()],
        bump = join_record.bump,
    )]
    pub join_record: Box<Account<'info, JoinRecord>>,
    /// CHECK: the user's joined encrypted value account; read as the MulDiv operand.
    pub pending_join_value: UncheckedAccount<'info>,
    /// CHECK: the user's claim encrypted value account; created by the claim eval and spent as
    /// the transfer amount.
    #[account(mut)]
    pub claim_amount_value: UncheckedAccount<'info>,
    /// Confidential mint claims pay out in.
    pub payout_confidential_mint: Box<Account<'info, ct::ConfidentialMint>>,
    /// CHECK: payout mint compute-signer PDA; validated by the token CPI.
    pub payout_compute_signer: UncheckedAccount<'info>,
    /// CHECK: batch's confidential payout token account (transfer source);
    /// validated by the token CPI and pinned below.
    #[account(mut)]
    pub batch_payout_token_account: UncheckedAccount<'info>,
    /// CHECK: user's confidential payout token account (transfer destination);
    /// must already exist — the user initializes it once. Validated by the
    /// token CPI and pinned below.
    #[account(mut)]
    pub user_payout_token_account: UncheckedAccount<'info>,
    /// CHECK: batch's confidential payout balance encrypted value account; superseded by the token CPI.
    #[account(mut)]
    pub batch_payout_balance_value: UncheckedAccount<'info>,
    /// CHECK: user's confidential payout balance encrypted value account; superseded by the token CPI.
    #[account(mut)]
    pub user_payout_balance_value: UncheckedAccount<'info>,
    /// CHECK: batch payout account's transferred-amount encrypted value account; superseded by
    /// the token CPI.
    #[account(mut)]
    pub batch_payout_transferred_value: UncheckedAccount<'info>,
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

/// Computes the user's encrypted payout amount and transfers it to them.
pub fn claim<'info>(ctx: Context<'info, Claim<'info>>) -> Result<()> {
    require!(
        ctx.accounts.batch.status == BatchStatus::Settled,
        BatcherError::BatchNotSettled
    );
    require!(
        !ctx.accounts.join_record.claimed,
        BatcherError::AlreadyClaimed
    );
    require_keys_eq!(
        ctx.accounts.payout_confidential_mint.key(),
        ctx.accounts.batcher.payout_confidential_mint,
        BatcherError::ConfidentialMintMismatch
    );
    require_keys_eq!(
        ctx.accounts.pending_join_value.key(),
        ctx.accounts.join_record.joined_encrypted_value,
        BatcherError::DerivedAccountMismatch
    );
    let payout_mint_key = ctx.accounts.payout_confidential_mint.key();
    let batch_key = ctx.accounts.batch.key();
    let user = ctx.accounts.user.key();
    let batch_authority = ctx.accounts.batch_authority.key();
    require_keys_eq!(
        ctx.accounts.batch_payout_token_account.key(),
        ct::token_account_address(payout_mint_key, batch_authority).0,
        BatcherError::DerivedAccountMismatch
    );
    require_keys_eq!(
        ctx.accounts.user_payout_token_account.key(),
        ct::token_account_address(payout_mint_key, user).0,
        BatcherError::DerivedAccountMismatch
    );

    // Leg 1: the one MulDiv frame — the encrypted joined amount's exact
    // proportional share of the public aggregate payout.
    let joined_value = fhe::read_encrypted_value(&ctx.accounts.pending_join_value)?;
    let joined = fhe::uint64_operand(&joined_value)?;
    let claim_binding = fhe::DurableBinding::bind(
        ctx.accounts.claim_amount_value.to_account_info(),
        zama_fhe::DurableSlot::new(
            batch_key,
            batch_authority,
            zama_fhe::DurableLabel::new(claim_amount_label(user)),
        ),
        // The user decrypts their claimed amount; the batch authority spends
        // it as the transfer amount; the payout mint's compute signer lets the
        // transfer eval read it.
        zama_fhe::AccessPolicy::from_subjects(vec![
            zama_fhe::AccessSubject::owner(user),
            zama_fhe::AccessSubject::compute(batch_authority),
            zama_fhe::AccessSubject::compute(ctx.accounts.payout_confidential_mint.compute_signer),
        ])
        .map_err(fhe::invalid_eval_plan)?,
    )?;
    let context_id = zama_fhe::EvalContextId::new(
        solana_sha256_hasher::hashv(&[
            b"confidential-batcher-claim-v1",
            batch_key.as_ref(),
            user.as_ref(),
            &joined_value.current_handle,
        ])
        .to_bytes(),
    )
    .map_err(fhe::invalid_eval_plan)?;
    let payout_received = ctx.accounts.batch.payout_received;
    let total_joined = ctx.accounts.batch.total_joined;
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
            claim_binding.account_info(),
            ctx.accounts.pending_join_value.to_account_info(),
        ],
        |builder| {
            builder.mul_div(
                joined,
                zama_fhe::Scalar::<zama_fhe::Uint<64>>::u64(payout_received),
                zama_fhe::Scalar::<zama_fhe::Uint<64>>::u64(total_joined),
                claim_binding.output(),
            )
        },
    )?;

    // Leg 2: transfer the freshly computed claim handle to the user.
    let authority = BatchAuthoritySeeds::new(batch_key, ctx.accounts.batch.authority_bump);
    let authority_seeds = authority.seeds();
    ct::cpi::confidential_transfer_from_value(CpiContext::new_with_signer(
        ctx.accounts.confidential_token_program.key(),
        ct::cpi::accounts::ConfidentialTransferFromValue {
            owner: ctx.accounts.batch_authority.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            mint: ctx.accounts.payout_confidential_mint.to_account_info(),
            from_account: ctx.accounts.batch_payout_token_account.to_account_info(),
            to_account: ctx.accounts.user_payout_token_account.to_account_info(),
            compute_signer: ctx.accounts.payout_compute_signer.to_account_info(),
            from_balance_value: ctx.accounts.batch_payout_balance_value.to_account_info(),
            to_balance_value: ctx.accounts.user_payout_balance_value.to_account_info(),
            transferred_amount_value: ctx
                .accounts
                .batch_payout_transferred_value
                .to_account_info(),
            amount_value: ctx.accounts.claim_amount_value.to_account_info(),
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

    ctx.accounts.join_record.claimed = true;

    emit!(PayoutClaimed {
        version: APP_EVENT_VERSION,
        batch: batch_key,
        user,
        claim_encrypted_value: ctx.accounts.claim_amount_value.key(),
    });
    Ok(())
}
