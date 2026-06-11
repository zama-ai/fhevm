//! Closes expired burn-redemption request witnesses.

use super::*;

/// Accounts for closing an expired burn-redemption request witness.
#[derive(Accounts)]
pub struct CloseExpiredBurnRedemptionRequest<'info> {
    /// Request owner and rent recipient.
    #[account(mut)]
    pub owner: Signer<'info>,
    /// Expired request witness to close.
    #[account(mut, close = owner)]
    pub redemption_request: Box<Account<'info, BurnRedemptionRequest>>,
}

/// Closes an expired, unconsumed burn-redemption request witness.
pub fn close_expired_burn_redemption_request(
    ctx: Context<CloseExpiredBurnRedemptionRequest>,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    require_keys_eq!(
        ctx.accounts.redemption_request.owner,
        ctx.accounts.owner.key(),
        ConfidentialTokenError::OwnerMismatch
    );
    let clock = Clock::get()?;
    require!(
        ctx.accounts.redemption_request.status == REQUEST_STATUS_PENDING
            && ctx.accounts.redemption_request.expires_slot < clock.slot,
        ConfidentialTokenError::RequestWitnessUnavailable
    );
    Ok(())
}
