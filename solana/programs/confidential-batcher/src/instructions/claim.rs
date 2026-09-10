//! Claims a user's confidential payout from a settled batch. Direction-free:
//! the payout mint is confidential shares for deposit batchers, confidential
//! underlying for redeem batchers.
//!
//! One MulDiv batch — the exact proportional floor
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
//! The execution and transfer assume `grant_deny_list_enabled = false` and no
//! binding HCU cap: `hcu_block_meter` and `hcu_trusted_app_record` are
//! hardcoded `None` (the PoC host fixtures never enable them), and deny-list
//! records ride in as the (empty) remaining accounts.

use super::*;

/// Accounts for claiming a payout.
#[derive(Accounts)]
pub struct Claim<'info> {
    /// Pays the claim encrypted State and transfer output rent. Anyone.
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
    /// CHECK: per-batch authority PDA; the claim execution's value authority and the payout
    /// transfer's authority via invoke_signed.
    #[account(seeds = [BATCH_AUTHORITY_SEED, batch.key().as_ref()], bump = batch.authority_bump)]
    pub batch_authority: UncheckedAccount<'info>,
    /// The user's join record; marked claimed here.
    #[account(
        mut,
        seeds = [JOIN_RECORD_SEED, batch.key().as_ref(), user.key().as_ref()],
        bump = join_record.bump,
    )]
    pub join_record: Box<Account<'info, JoinRecord>>,
    /// CHECK: canonical state controlled by JoinRecord.
    #[account(mut)]
    pub join_state: UncheckedAccount<'info>,
    /// CHECK: host validates the shared transaction transient store.
    #[account(mut)]
    pub transient_store: UncheckedAccount<'info>,
    /// CHECK: host validates final transient store close using the Instructions sysvar.
    pub instructions: UncheckedAccount<'info>,
    /// Confidential mint claims pay out in.
    pub payout_confidential_mint: Box<Account<'info, ct::ConfidentialMint>>,
    /// CHECK: underlying SPL mint wrapped by `payout_confidential_mint`. Token program is its owner.
    pub payout_underlying_mint: UncheckedAccount<'info>,
    /// CHECK: ATA of `batch_authority` on `payout_underlying_mint`. Uninitialized → not frozen.
    pub batch_authority_payout_ata: UncheckedAccount<'info>,
    /// CHECK: ATA of `user` on `payout_underlying_mint`. Uninitialized → not frozen.
    pub user_payout_ata: UncheckedAccount<'info>,
    /// CHECK: batch's confidential payout token account (transfer source);
    /// validated by the token CPI and pinned below.
    #[account(mut)]
    pub batch_payout_token_account: UncheckedAccount<'info>,
    /// CHECK: user's confidential payout token account (transfer destination);
    /// must already exist — the user initializes it once. Validated by the
    /// token CPI and pinned below.
    #[account(mut)]
    pub user_payout_token_account: UncheckedAccount<'info>,
    /// CHECK: batch's confidential payout balance encrypted State; replaced by the token CPI.
    #[account(mut)]
    pub batch_payout_balance_state: UncheckedAccount<'info>,
    /// CHECK: user's confidential payout balance encrypted State; replaced by the token CPI.
    #[account(mut)]
    pub user_payout_balance_state: UncheckedAccount<'info>,
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
        ctx.accounts.join_state.key(),
        join_state_id(ctx.accounts.batch.key(), ctx.accounts.join_record.key()).address(),
        BatcherError::DerivedAccountMismatch
    );
    let payout_mint_key = ctx.accounts.payout_confidential_mint.key();
    let batch_key = ctx.accounts.batch.key();
    let user = ctx.accounts.user.key();
    let batch_authority = ctx.accounts.batch_authority.key();
    let authority = BatchAuthoritySeeds::new(batch_key, ctx.accounts.batch.authority_bump);
    let authority_seeds = authority.seeds();
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

    let account = fhe::read_state(&ctx.accounts.join_state)?;
    let state = zama_fhe::State::new(&account);
    let joined = state
        .get::<zama_fhe::Uint<64>>(joined_amount_key())
        .map_err(fhe::invalid_execution)?;
    let payout_state = fhe::read_state(&ctx.accounts.batch_payout_balance_state)?;
    let output = state
        .result()
        .allow(user)
        .allow_transient(zama_fhe::State::new(&payout_state).id());
    let execution = zama_fhe::FheExecution::build_returning(state.id(), |builder| {
        let payout = builder.mul_div(
            joined,
            zama_fhe::Scalar::<zama_fhe::Uint<64>>::u64(ctx.accounts.batch.payout_received),
            zama_fhe::Scalar::<zama_fhe::Uint<64>>::u64(ctx.accounts.batch.total_joined),
        )?;
        builder.output(payout, output)?;
        Ok(payout)
    })
    .map_err(fhe::invalid_execution)?;
    let claim_handle = fhe::JoinExecute {
        batch: batch_key,
        user,
        bump: ctx.accounts.join_record.bump,
        record: ctx.accounts.join_record.to_account_info(),
        payer: ctx.accounts.payer.to_account_info(),
        host_config: ctx.accounts.host_config.to_account_info(),
        event_authority: ctx.accounts.zama_event_authority.to_account_info(),
        transient_store: ctx.accounts.transient_store.to_account_info(),
        instructions: ctx.accounts.instructions.to_account_info(),
        program: ctx.accounts.zama_program.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        deny_records: ctx.remaining_accounts,
    }
    .invoke(
        execution,
        vec![
            ctx.accounts.join_state.to_account_info(),
            ctx.accounts.batch_payout_balance_state.to_account_info(),
        ],
    )?;

    // Phase 2: transfer the freshly computed claim handle to the user.
    ct::cpi::confidential_transfer_from_value(
        CpiContext::new_with_signer(
            ctx.accounts.confidential_token_program.key(),
            ct::cpi::accounts::ConfidentialTransferFromValue {
                owner: ctx.accounts.batch_authority.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                mint: ctx.accounts.payout_confidential_mint.to_account_info(),
                underlying_mint: ctx.accounts.payout_underlying_mint.to_account_info(),
                from_ata: ctx.accounts.batch_authority_payout_ata.to_account_info(),
                to_ata: ctx.accounts.user_payout_ata.to_account_info(),
                from_account: ctx.accounts.batch_payout_token_account.to_account_info(),
                to_account: ctx.accounts.user_payout_token_account.to_account_info(),
                from_state: ctx.accounts.batch_payout_balance_state.to_account_info(),
                to_state: ctx.accounts.user_payout_balance_state.to_account_info(),
                amount_state: None,
                amount_authority: None,

                zama_event_authority: ctx.accounts.zama_event_authority.to_account_info(),
                transient_store: ctx.accounts.transient_store.to_account_info(),
                instructions: ctx.accounts.instructions.to_account_info(),
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
        ct::TransferInput::Grant {
            handle: claim_handle,
        },
    )?;

    ctx.accounts.join_record.claimed = true;

    emit!(PayoutClaimed {
        version: APP_EVENT_VERSION,
        batch: batch_key,
        user,
        claim_state: ctx.accounts.join_state.key(),
        claim_handle,
    });
    Ok(())
}
