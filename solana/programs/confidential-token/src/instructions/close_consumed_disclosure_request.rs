//! Closes consumed disclosure request witnesses.

use super::*;

/// Accounts for closing a consumed disclosure request witness.
#[derive(Accounts)]
pub struct CloseConsumedDisclosureRequest<'info> {
    /// Requester and rent recipient.
    #[account(mut)]
    pub requester: Signer<'info>,
    /// Consumed request witness to close.
    #[account(mut, close = requester)]
    pub disclosure_request: Box<Account<'info, DisclosureRequest>>,
}

/// Closes a disclosure request after it has been consumed and expired.
pub fn close_consumed_disclosure_request(
    ctx: Context<CloseConsumedDisclosureRequest>,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    require_keys_eq!(
        ctx.accounts.disclosure_request.requester,
        ctx.accounts.requester.key(),
        ConfidentialTokenError::OwnerMismatch
    );
    let clock = Clock::get()?;
    require!(
        ctx.accounts.disclosure_request.status == REQUEST_STATUS_CONSUMED
            && ctx.accounts.disclosure_request.expires_slot < clock.slot,
        ConfidentialTokenError::RequestWitnessUnavailable
    );
    Ok(())
}
