//! Closes expired disclosure request witnesses.

use super::*;

/// Accounts for closing an expired disclosure request witness.
#[derive(Accounts)]
pub struct CloseExpiredDisclosureRequest<'info> {
    /// Requester and rent recipient.
    #[account(mut)]
    pub requester: Signer<'info>,
    /// Expired request witness to close.
    #[account(mut, close = requester)]
    pub disclosure_request: Box<Account<'info, DisclosureRequest>>,
}

/// Closes an expired, unconsumed disclosure request witness.
pub fn close_expired_disclosure_request(ctx: Context<CloseExpiredDisclosureRequest>) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    require_keys_eq!(
        ctx.accounts.disclosure_request.requester,
        ctx.accounts.requester.key(),
        ConfidentialTokenError::OwnerMismatch
    );
    let clock = Clock::get()?;
    require!(
        ctx.accounts.disclosure_request.status == REQUEST_STATUS_PENDING
            && ctx.accounts.disclosure_request.expires_slot < clock.slot,
        ConfidentialTokenError::RequestWitnessUnavailable
    );
    Ok(())
}
