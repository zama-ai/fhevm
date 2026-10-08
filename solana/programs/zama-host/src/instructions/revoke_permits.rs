//! Revokes a user's permits by raising their invalidation watermark.
//!
//! The instruction writes one number: the later of the stored watermark and the
//! current clock. Everything else about revocation is a reader's rule — a verifier
//! rejects a permit whose validity window starts before this moment, and a missing
//! account reads as zero — so this handler has no list to walk and nothing to
//! enumerate. That is the whole point of the design: one transaction, constant work,
//! however many permits are outstanding. What a raised watermark cannot reach — a
//! permit pre-signed to open in the future — is recorded on [`PermitInvalidation`].

use anchor_lang::prelude::*;

use super::common::*;
use crate::{errors::ZamaHostError, state::*};

/// Accounts for revoking a user's outstanding permits.
///
/// Deliberately small. There is no config account, because pausing the host must not
/// take away a user's ability to revoke — a lever that can be disabled by the operator
/// is not the user's lever. And there is no separate payer: the user pays for their own
/// watermark account, which keeps the signer set to exactly the one identity the
/// watermark is keyed by.
#[derive(Accounts)]
pub struct RevokePermits<'info> {
    /// The user revoking their permits, and the payer for the watermark account.
    #[account(mut)]
    pub user: Signer<'info>,
    /// The user's watermark, created on first revocation. The address is derived from the
    /// signer, so another user's watermark is never at this address.
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + PermitInvalidation::SPACE,
        seeds = [PERMIT_INVALIDATION_SEED, user.key().as_ref()],
        bump,
    )]
    pub invalidation: Account<'info, PermitInvalidation>,
    /// System program, used when the watermark account has to be created.
    pub system_program: Program<'info, System>,
}

/// Raises the caller's invalidation watermark to the current clock.
pub fn revoke_permits(ctx: Context<RevokePermits>) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;

    // The clock is refused rather than coerced when it reads before the epoch. The
    // watermark is unsigned seconds, so a cast would land near the top of the range and
    // permanently kill every permit this user will ever sign — an unrecoverable state
    // produced by a conversion. Failing closed cannot destroy an account.
    let now = u64::try_from(Clock::get()?.unix_timestamp)
        .map_err(|_| error!(ZamaHostError::ClockBeforeEpoch))?;

    // A record created by this call is zeroed, which is the watermark an absent account reads as.
    let user = ctx.accounts.user.key();
    let invalidation = &mut ctx.accounts.invalidation;
    invalidation.user = user;
    invalidation.bump = ctx.bumps.invalidation;
    // Monotonic by construction: the recorded value is a maximum, so a slot whose
    // clock lags cannot resurrect permits this user already killed.
    invalidation.invalidation_watermark = invalidation.invalidation_watermark.max(now);
    Ok(())
}
