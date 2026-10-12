//! The batcher admin's instructions: pause and unpause (INVARIANTS #71), the policy that batches
//! opened from now on copy, and handing the role on.

use anchor_lang::solana_program::system_program;

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

/// Accounts for handing the admin role to a new key.
#[derive(Accounts)]
pub struct SetAdmin<'info> {
    /// The batcher's current admin.
    pub admin: Signer<'info>,
    /// Batcher config.
    #[account(mut, has_one = admin @ BatcherError::AdminMismatch)]
    pub batcher: Box<Account<'info, Batcher>>,
    /// CHECK: the incoming admin. A keypair co-signs; a PDA skips the co-sign only when it is
    /// off-curve and already owned by a program, so a mistyped key cannot become admin.
    pub new_admin: UncheckedAccount<'info>,
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
    emit!(BatcherPaused {
        version: APP_EVENT_VERSION,
        batcher: batcher.key(),
    });
    Ok(())
}

pub fn unpause(ctx: Context<BatcherAdmin>) -> Result<()> {
    let batcher = &mut ctx.accounts.batcher;
    require!(batcher.paused, BatcherError::NotPaused);
    batcher.paused = false;
    emit!(BatcherUnpaused {
        version: APP_EVENT_VERSION,
        batcher: batcher.key(),
    });
    Ok(())
}

pub fn set_min_batch_age(ctx: Context<BatcherAdmin>, min_batch_age_secs: u64) -> Result<()> {
    require_valid_min_batch_age(min_batch_age_secs)?;
    let batcher = &mut ctx.accounts.batcher;
    batcher.min_batch_age_secs = min_batch_age_secs;
    emit!(MinBatchAgeSet {
        version: APP_EVENT_VERSION,
        batcher: batcher.key(),
        min_batch_age_secs,
    });
    Ok(())
}

pub fn set_settle_deadline(ctx: Context<BatcherAdmin>, settle_deadline_secs: u64) -> Result<()> {
    require_valid_settle_deadline(settle_deadline_secs)?;
    let batcher = &mut ctx.accounts.batcher;
    batcher.settle_deadline_secs = settle_deadline_secs;
    emit!(SettleDeadlineSet {
        version: APP_EVENT_VERSION,
        batcher: batcher.key(),
        settle_deadline_secs,
    });
    Ok(())
}

pub fn set_admin(ctx: Context<SetAdmin>) -> Result<()> {
    let new_admin = &ctx.accounts.new_admin;
    require!(
        new_admin.is_signer
            || (!new_admin.key().is_on_curve() && new_admin.owner != &system_program::ID),
        BatcherError::NewAdminNotAccepted
    );
    let batcher = &mut ctx.accounts.batcher;
    let previous_admin = batcher.admin;
    batcher.admin = new_admin.key();
    emit!(AdminSet {
        version: APP_EVENT_VERSION,
        batcher: batcher.key(),
        previous_admin,
        new_admin: new_admin.key(),
    });
    Ok(())
}
