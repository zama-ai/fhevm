//! Sets and revokes confidential-token operators.

use super::*;

/// Accounts for setting or revoking an operator.
#[derive(Accounts)]
#[event_cpi]
pub struct SetOperator<'info> {
    /// Token account owner and rent payer.
    #[account(mut)]
    pub owner: Signer<'info>,
    /// Confidential mint.
    pub mint: Account<'info, ConfidentialMint>,
    /// Token account whose operator row is being changed.
    pub token_account: Account<'info, ConfidentialTokenAccount>,
    /// CHECK: Canonical operator PDA created or overwritten by this instruction.
    #[account(mut)]
    pub operator_record: UncheckedAccount<'info>,
    /// System program used for operator PDA creation.
    pub system_program: Program<'info, System>,
}

/// Sets or revokes an operator for this confidential token account.
pub fn set_operator(
    ctx: Context<SetOperator>,
    operator: Pubkey,
    expiration_slot: u64,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_confidential_mint_shape(&ctx.accounts.mint)?;
    require_keys_eq!(
        ctx.accounts.token_account.owner,
        ctx.accounts.owner.key(),
        ConfidentialTokenError::OwnerMismatch
    );
    require_keys_eq!(
        ctx.accounts.token_account.mint,
        ctx.accounts.mint.key(),
        ConfidentialTokenError::MintMismatch
    );
    assert_confidential_token_account_shape(
        &ctx.accounts.token_account,
        ctx.accounts.mint.key(),
        ctx.accounts.owner.key(),
    )?;
    let (expected, bump) = operator_record_address(ctx.accounts.token_account.key(), operator);
    require_keys_eq!(
        ctx.accounts.operator_record.key(),
        expected,
        ConfidentialTokenError::OperatorRecordMismatch
    );
    create_operator_record_if_needed(
        &ctx.accounts.owner.to_account_info(),
        &ctx.accounts.operator_record.to_account_info(),
        &ctx.accounts.system_program.to_account_info(),
        ctx.accounts.token_account.key(),
        ctx.accounts.owner.key(),
        operator,
        bump,
    )?;
    write_operator_record(
        &ctx.accounts.operator_record.to_account_info(),
        &ConfidentialOperator {
            token_account: ctx.accounts.token_account.key(),
            owner: ctx.accounts.owner.key(),
            operator,
            expiration_slot,
            bump,
        },
    )?;
    emit_cpi!(OperatorSetEvent {
        version: APP_EVENT_VERSION,
        mint: ctx.accounts.mint.key(),
        token_account: ctx.accounts.token_account.key(),
        owner: ctx.accounts.owner.key(),
        operator,
        expiration_slot,
    });
    Ok(())
}
