//! Returns a finished batch's unspent authority funding to the operator.
//!
//! `open_batch`, `cancel_dispatch` and `settle` move `authority_funding_lamports` from their payer
//! to the batch authority PDA so it can pay the rent the token CPIs charge to the account owner.
//! Once a batch is settled, canceled or refunding, nothing charges the authority any more: claims
//! and quits pay their own rent through their `payer`, and the authority only signs. The PDA's
//! whole balance is returned to the join mint's wrapper authority, the operator role that funds
//! batches and cancels dispatches.

use super::*;

/// Accounts for reclaiming a finished batch's authority funding.
#[derive(Accounts)]
pub struct ReclaimBatchAuthority<'info> {
    /// Join-mint wrapper authority; receives the lamports.
    #[account(mut)]
    pub authority: Signer<'info>,
    /// Batcher config.
    pub batcher: Box<Account<'info, Batcher>>,
    /// The finished batch (settled, canceled or refunding).
    #[account(constraint = batch.batcher == batcher.key() @ BatcherError::BatchBatcherMismatch)]
    pub batch: Box<Account<'info, Batch>>,
    /// CHECK: per-batch authority PDA, a data-less system account drained here by `invoke_signed`.
    #[account(mut, seeds = [BATCH_AUTHORITY_SEED, batch.key().as_ref()], bump = batch.authority_bump)]
    pub batch_authority: UncheckedAccount<'info>,
    /// Confidential mint users join batches with; its authority is the reclaim authority.
    pub join_confidential_mint: Box<Account<'info, ct::ConfidentialMint>>,
    /// System program that moves the lamports.
    pub system_program: Program<'info, System>,
}

/// Moves the batch authority's whole balance to the join mint's wrapper authority.
pub fn reclaim_batch_authority(ctx: Context<ReclaimBatchAuthority>) -> Result<()> {
    require!(
        matches!(
            ctx.accounts.batch.status,
            BatchStatus::Settled | BatchStatus::Canceled | BatchStatus::Refunding
        ),
        BatcherError::BatchStillLive
    );
    require_keys_eq!(
        ctx.accounts.join_confidential_mint.key(),
        ctx.accounts.batcher.join_confidential_mint,
        BatcherError::ConfidentialMintMismatch
    );
    require_keys_eq!(
        ctx.accounts.authority.key(),
        ctx.accounts.join_confidential_mint.authority,
        BatcherError::ReclaimAuthorityMismatch
    );

    let batch = ctx.accounts.batch.key();
    let lamports = ctx.accounts.batch_authority.lamports();
    let authority = BatchAuthoritySeeds::new(batch, ctx.accounts.batch.authority_bump);
    let authority_seeds = authority.seeds();
    anchor_lang::system_program::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.system_program.key(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.batch_authority.to_account_info(),
                to: ctx.accounts.authority.to_account_info(),
            },
            &[&authority_seeds],
        ),
        lamports,
    )?;

    emit!(BatchAuthorityReclaimed {
        version: APP_EVENT_VERSION,
        batch,
        lamports,
    });
    Ok(())
}
