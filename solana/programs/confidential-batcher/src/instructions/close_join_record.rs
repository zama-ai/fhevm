//! Closes a user's join record once it has nothing left to do, returning its rent to the user.
//!
//! A join record is the user's ticket in a batch: `quit` reads it for the refund, `claim` marks it
//! claimed. It is spent once the payout is claimed, or as soon as the batch is canceled (nothing to
//! claim). In a refunding batch the record still authorizes `quit`, and the program cannot tell a
//! refunded record from a live one without decrypting the join store, so those stay open. The
//! joined-amount encrypted store keeps its ACL grants to the user and is not closed here.

use super::*;

/// Accounts for closing a spent join record.
#[derive(Accounts)]
pub struct CloseJoinRecord<'info> {
    /// The user who joined; receives the record's rent.
    #[account(mut)]
    pub user: Signer<'info>,
    /// The batch the record belongs to.
    pub batch: Box<Account<'info, Batch>>,
    /// The user's join record for this batch; closed to `user`.
    #[account(
        mut,
        close = user,
        seeds = [JOIN_RECORD_SEED, batch.key().as_ref(), user.key().as_ref()],
        bump = join_record.bump,
    )]
    pub join_record: Box<Account<'info, JoinRecord>>,
}

/// Closes the record when its batch is canceled or its payout has been claimed.
pub fn close_join_record(ctx: Context<CloseJoinRecord>) -> Result<()> {
    let spent = match ctx.accounts.batch.status {
        BatchStatus::Settled => ctx.accounts.join_record.claimed,
        BatchStatus::Canceled => true,
        BatchStatus::Pending | BatchStatus::Dispatched | BatchStatus::Refunding => false,
    };
    require!(spent, BatcherError::JoinRecordStillLive);

    emit!(JoinRecordClosed {
        version: APP_EVENT_VERSION,
        batch: ctx.accounts.batch.key(),
        user: ctx.accounts.user.key(),
    });
    Ok(())
}
