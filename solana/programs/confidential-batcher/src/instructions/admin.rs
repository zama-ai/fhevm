//! The batcher admin's instructions: pause and unpause (INVARIANTS #71), the policy that batches
//! opened from now on copy, and handing the role on.

use super::*;

/// Accounts for an admin-only change to the batcher config.
#[derive(Accounts)]
pub struct BatcherAdmin<'info> {
    /// The batcher's admin.
    pub admin: Signer<'info>,
    /// Batcher config.
    #[account(mut, has_one = admin @ BatcherError::AdminMismatch)]
    pub batcher: Box<Account<'info, Batcher>>,
}

/// Accounts for taking the admin role.
#[derive(Accounts)]
pub struct AcceptAdmin<'info> {
    /// The nominee, `Batcher.pending_admin`.
    pub pending_admin: Signer<'info>,
    /// Batcher config.
    #[account(mut, has_one = pending_admin @ BatcherError::PendingAdminMismatch)]
    pub batcher: Box<Account<'info, Batcher>>,
}

/// Refuses an instruction the pause blocks.
pub(crate) fn require_not_paused(batcher: &Batcher) -> Result<()> {
    require!(!batcher.paused, BatcherError::Paused);
    Ok(())
}

/// Checks a minimum batch age against `MAX_MIN_BATCH_AGE_SECS`.
pub(crate) fn require_valid_min_batch_age(secs: u64) -> Result<()> {
    require!(
        secs <= MAX_MIN_BATCH_AGE_SECS,
        BatcherError::InvalidMinBatchAge
    );
    Ok(())
}

/// Checks a settle deadline against `MAX_SETTLE_DEADLINE_SECS`. Zero is allowed: such a batch
/// cannot settle and anyone may cancel it as soon as it is dispatched, as on EVM.
pub(crate) fn require_valid_settle_deadline(secs: u64) -> Result<()> {
    require!(
        secs <= MAX_SETTLE_DEADLINE_SECS,
        BatcherError::InvalidSettleDeadline
    );
    Ok(())
}

pub fn pause(ctx: Context<BatcherAdmin>) -> Result<()> {
    let batcher = &mut ctx.accounts.batcher;
    require_not_paused(batcher)?;
    batcher.paused = true;
    Ok(())
}

pub fn unpause(ctx: Context<BatcherAdmin>) -> Result<()> {
    let batcher = &mut ctx.accounts.batcher;
    require!(batcher.paused, BatcherError::NotPaused);
    batcher.paused = false;
    Ok(())
}

pub fn set_min_batch_age(ctx: Context<BatcherAdmin>, min_batch_age_secs: u64) -> Result<()> {
    require_valid_min_batch_age(min_batch_age_secs)?;
    let batcher = &mut ctx.accounts.batcher;
    batcher.min_batch_age_secs = min_batch_age_secs;
    Ok(())
}

pub fn set_settle_deadline(ctx: Context<BatcherAdmin>, settle_deadline_secs: u64) -> Result<()> {
    require_valid_settle_deadline(settle_deadline_secs)?;
    let batcher = &mut ctx.accounts.batcher;
    batcher.settle_deadline_secs = settle_deadline_secs;
    Ok(())
}

pub fn set_admin(ctx: Context<BatcherAdmin>, new_admin: Pubkey) -> Result<()> {
    ctx.accounts.batcher.pending_admin = new_admin;
    Ok(())
}

/// No one signs for `Pubkey::default()`, so with no nomination every signer is refused.
pub fn accept_admin(ctx: Context<AcceptAdmin>) -> Result<()> {
    let batcher = &mut ctx.accounts.batcher;
    batcher.admin = ctx.accounts.pending_admin.key();
    batcher.pending_admin = Pubkey::default();
    Ok(())
}
