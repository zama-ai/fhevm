//! Closes revoked or expired confidential-token operator records.

use super::*;

/// Accounts for closing an operator row.
#[derive(Accounts)]
#[event_cpi]
#[instruction(operator: Pubkey)]
pub struct CloseOperator<'info> {
    /// Optional token owner. Required when closing an active operator row.
    pub owner: Option<Signer<'info>>,
    /// Confidential mint.
    pub mint: Account<'info, ConfidentialMint>,
    /// Token account controlled by the operator row.
    pub token_account: Account<'info, ConfidentialTokenAccount>,
    /// Operator authorization row to close.
    #[account(
        mut,
        seeds = [b"operator", token_account.key().as_ref(), operator.as_ref()],
        bump = operator_record.bump,
        close = refund_recipient
    )]
    pub operator_record: Account<'info, ConfidentialOperator>,
    /// CHECK: Must be the stored token owner and receives the rent refund.
    #[account(mut)]
    pub refund_recipient: UncheckedAccount<'info>,
}

/// Closes a revoked or expired operator row and refunds rent to the token owner.
pub fn close_operator(ctx: Context<CloseOperator>, operator: Pubkey) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_confidential_mint_shape(&ctx.accounts.mint)?;
    require_keys_eq!(
        ctx.accounts.token_account.owner,
        ctx.accounts.operator_record.owner,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    require_keys_eq!(
        ctx.accounts.token_account.mint,
        ctx.accounts.mint.key(),
        ConfidentialTokenError::MintMismatch
    );
    assert_confidential_token_account_shape(
        &ctx.accounts.token_account,
        ctx.accounts.mint.key(),
        ctx.accounts.token_account.owner,
    )?;
    assert_operator_record_shape(
        &ctx.accounts.operator_record,
        ctx.accounts.token_account.key(),
        ctx.accounts.token_account.owner,
        operator,
    )?;
    require_keys_eq!(
        ctx.accounts.refund_recipient.key(),
        ctx.accounts.operator_record.owner,
        ConfidentialTokenError::OwnerMismatch
    );

    let slot = Clock::get()?.slot;
    let operator_active = ctx.accounts.operator_record.expiration_slot != 0
        && ctx.accounts.operator_record.expiration_slot >= slot;
    if operator_active {
        let owner = ctx
            .accounts
            .owner
            .as_ref()
            .ok_or(ConfidentialTokenError::OwnerMismatch)?;
        require_keys_eq!(
            owner.key(),
            ctx.accounts.operator_record.owner,
            ConfidentialTokenError::OwnerMismatch
        );
    } else if let Some(owner) = ctx.accounts.owner.as_ref() {
        require_keys_eq!(
            owner.key(),
            ctx.accounts.operator_record.owner,
            ConfidentialTokenError::OwnerMismatch
        );
    }

    emit_cpi!(OperatorClosedEvent {
        version: APP_EVENT_VERSION,
        mint: ctx.accounts.mint.key(),
        token_account: ctx.accounts.token_account.key(),
        owner: ctx.accounts.operator_record.owner,
        operator,
        closed_while_active: operator_active,
    });
    Ok(())
}
