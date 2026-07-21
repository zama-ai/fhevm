//! Claims a user's confidential shares from a settled batch.
//!
//! One MulDiv eval frame — `encrypted(deposit) x share_rate / RATE_SCALE` —
//! then a confidential transfer of the resulting handle from the batch's
//! shares account to the user. Permissionless pull: anyone can trigger a
//! user's claim; the shares can only land in the user's own account.
//!
//! Rounding guarantees delivery: the rate and every claim round down, so the
//! sum of all claims never exceeds the wrapped shares and the all-or-zero
//! transfer always moves the full claim.

use super::*;

/// Accounts for claiming shares.
#[derive(Accounts)]
pub struct Claim<'info> {
    /// Pays the claim lineage and transfer output rent. Anyone.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: the user being claimed for; pinned by the deposit record's PDA
    /// seeds. Not a signer — claims are permissionless pulls.
    pub user: UncheckedAccount<'info>,
    /// Batcher config.
    pub batcher: Box<Account<'info, Batcher>>,
    /// The settled batch being claimed from.
    #[account(constraint = batch.batcher == batcher.key() @ BatcherError::BatchBatcherMismatch)]
    pub batch: Box<Account<'info, Batch>>,
    /// CHECK: per-batch authority PDA; the claim eval's compute subject + app
    /// authority and the share transfer's authority via invoke_signed.
    #[account(seeds = [BATCH_AUTHORITY_SEED, batch.key().as_ref()], bump = batch.authority_bump)]
    pub batch_authority: UncheckedAccount<'info>,
    /// The user's deposit record; marked claimed here.
    #[account(
        mut,
        seeds = [DEPOSIT_RECORD_SEED, batch.key().as_ref(), user.key().as_ref()],
        bump = deposit_record.bump,
    )]
    pub deposit_record: Box<Account<'info, DepositRecord>>,
    /// CHECK: the user's batch deposit lineage; read as the MulDiv operand.
    pub pending_deposit_value: UncheckedAccount<'info>,
    /// CHECK: the user's claim lineage; created by the claim eval and spent as
    /// the transfer amount.
    #[account(mut)]
    pub claim_amount_value: UncheckedAccount<'info>,
    /// Confidential mint wrapping the vault's share mint.
    pub shares_confidential_mint: Box<Account<'info, ct::ConfidentialMint>>,
    /// CHECK: shares mint compute-signer PDA; validated by the token CPI.
    pub shares_compute_signer: UncheckedAccount<'info>,
    /// CHECK: batch's confidential shares token account (transfer source);
    /// validated by the token CPI and pinned below.
    #[account(mut)]
    pub batch_shares_token_account: UncheckedAccount<'info>,
    /// CHECK: user's confidential shares token account (transfer destination);
    /// must already exist — the user initializes it once. Validated by the
    /// token CPI and pinned below.
    #[account(mut)]
    pub user_shares_token_account: UncheckedAccount<'info>,
    /// CHECK: batch's confidential shares balance lineage; superseded by the token CPI.
    #[account(mut)]
    pub batch_shares_balance_value: UncheckedAccount<'info>,
    /// CHECK: user's confidential shares balance lineage; superseded by the token CPI.
    #[account(mut)]
    pub user_shares_balance_value: UncheckedAccount<'info>,
    /// CHECK: batch shares account's transferred-amount lineage; superseded by
    /// the token CPI.
    #[account(mut)]
    pub batch_shares_transferred_value: UncheckedAccount<'info>,
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

/// Computes the user's encrypted share amount and transfers it to them.
pub fn claim<'info>(ctx: Context<'info, Claim<'info>>) -> Result<()> {
    require!(
        ctx.accounts.batch.status == BatchStatus::Settled,
        BatcherError::BatchNotSettled
    );
    require!(
        !ctx.accounts.deposit_record.claimed,
        BatcherError::AlreadyClaimed
    );
    require_keys_eq!(
        ctx.accounts.shares_confidential_mint.key(),
        ctx.accounts.batcher.shares_confidential_mint,
        BatcherError::ConfidentialMintMismatch
    );
    require_keys_eq!(
        ctx.accounts.pending_deposit_value.key(),
        ctx.accounts.deposit_record.deposit_encrypted_value,
        BatcherError::DerivedAccountMismatch
    );
    let shares_mint_key = ctx.accounts.shares_confidential_mint.key();
    let batch_key = ctx.accounts.batch.key();
    let user = ctx.accounts.user.key();
    let batch_authority = ctx.accounts.batch_authority.key();
    require_keys_eq!(
        ctx.accounts.batch_shares_token_account.key(),
        ct::token_account_address(shares_mint_key, batch_authority).0,
        BatcherError::DerivedAccountMismatch
    );
    require_keys_eq!(
        ctx.accounts.user_shares_token_account.key(),
        ct::token_account_address(shares_mint_key, user).0,
        BatcherError::DerivedAccountMismatch
    );

    // Leg 1: the one MulDiv frame — encrypted deposit times the public rate.
    let deposit_value = fhe::read_encrypted_value(&ctx.accounts.pending_deposit_value)?;
    let deposit = fhe::uint64_operand(&deposit_value)?;
    let claim_binding = fhe::DurableBinding::bind(
        ctx.accounts.claim_amount_value.to_account_info(),
        zama_fhe::DurableSlot::new(
            batch_key,
            batch_authority,
            zama_fhe::DurableLabel::new(claim_amount_label(user)),
        ),
        // The user decrypts their claimed amount; the batch authority spends
        // it as the transfer amount; the shares mint's compute signer lets the
        // transfer eval read it.
        zama_fhe::AccessPolicy::from_subjects(vec![
            zama_fhe::AccessSubject::owner(user),
            zama_fhe::AccessSubject::compute(batch_authority),
            zama_fhe::AccessSubject::compute(ctx.accounts.shares_confidential_mint.compute_signer),
        ])
        .map_err(fhe::invalid_eval_plan)?,
    )?;
    let context_id = zama_fhe::EvalContextId::new(
        solana_sha256_hasher::hashv(&[
            b"confidential-batcher-claim-v1",
            batch_key.as_ref(),
            user.as_ref(),
            &deposit_value.current_handle,
        ])
        .to_bytes(),
    )
    .map_err(fhe::invalid_eval_plan)?;
    let share_rate = ctx.accounts.batch.share_rate;
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
            ctx.accounts.pending_deposit_value.to_account_info(),
        ],
        |builder| {
            builder.mul_div(
                deposit,
                zama_fhe::Scalar::<zama_fhe::Uint<64>>::u64(share_rate),
                zama_fhe::Scalar::<zama_fhe::Uint<64>>::u64(RATE_SCALE),
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
            mint: ctx.accounts.shares_confidential_mint.to_account_info(),
            from_account: ctx.accounts.batch_shares_token_account.to_account_info(),
            to_account: ctx.accounts.user_shares_token_account.to_account_info(),
            compute_signer: ctx.accounts.shares_compute_signer.to_account_info(),
            from_balance_value: ctx.accounts.batch_shares_balance_value.to_account_info(),
            to_balance_value: ctx.accounts.user_shares_balance_value.to_account_info(),
            transferred_amount_value: ctx
                .accounts
                .batch_shares_transferred_value
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

    ctx.accounts.deposit_record.claimed = true;

    emit!(SharesClaimed {
        version: APP_EVENT_VERSION,
        batch: batch_key,
        user,
        claim_encrypted_value: ctx.accounts.claim_amount_value.key(),
    });
    Ok(())
}
