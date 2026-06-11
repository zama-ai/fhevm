//! Closes consumed burn-redemption request witnesses.

use super::*;

/// Accounts for closing a consumed burn-redemption request witness.
#[derive(Accounts)]
pub struct CloseConsumedBurnRedemptionRequest<'info> {
    /// Request owner and rent recipient.
    #[account(mut)]
    pub owner: Signer<'info>,
    /// Consumed request witness to close.
    #[account(mut, close = owner)]
    pub redemption_request: Box<Account<'info, BurnRedemptionRequest>>,
}

/// Closes a burn-redemption request after it has been consumed and expired.
pub fn close_consumed_burn_redemption_request(
    ctx: Context<CloseConsumedBurnRedemptionRequest>,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    require_keys_eq!(
        ctx.accounts.redemption_request.owner,
        ctx.accounts.owner.key(),
        ConfidentialTokenError::OwnerMismatch
    );
    let clock = Clock::get()?;
    require!(
        ctx.accounts.redemption_request.status == REQUEST_STATUS_CONSUMED
            && ctx.accounts.redemption_request.expires_slot < clock.slot,
        ConfidentialTokenError::RequestWitnessUnavailable
    );
    Ok(())
}
