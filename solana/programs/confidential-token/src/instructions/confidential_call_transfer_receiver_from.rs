//! Invokes receiver hooks for operator-driven confidential transfers.

use super::*;

/// Accounts for calling a receiver hook after an operator-driven confidential transfer.
#[derive(Accounts)]
#[instruction(sent_handle: [u8; 32])]
pub struct ConfidentialCallTransferReceiverFrom<'info> {
    /// Active operator that initiated or is authorized to continue the split transfer-and-call flow.
    #[account(mut)]
    pub operator: Signer<'info>,
    /// Confidential mint.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Original sender token account controlled by the operator row.
    pub from_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Original recipient token account.
    pub to_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Operator authorization row for `(from_account, operator)`.
    #[account(
        seeds = [b"operator", from_account.key().as_ref(), operator.key().as_ref()],
        bump = operator_record.bump
    )]
    pub operator_record: Account<'info, ConfidentialOperator>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    /// ACL record for the prior transfer's all-or-zero sent amount.
    pub sent_amount_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// ACL record for the receiver-produced encrypted callback success bit.
    pub callback_success_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// CHECK: Receiver hook program invoked with the remaining accounts.
    pub receiver_program: UncheckedAccount<'info>,
    /// CHECK: Solana instructions sysvar used to prove same-transaction transfer intent.
    #[account(address = INSTRUCTIONS_SYSVAR_ID)]
    pub instructions_sysvar: UncheckedAccount<'info>,
    /// One-shot marker for this receiver hook invocation.
    #[account(
        init,
        payer = operator,
        space = 8 + TransferReceiverHookCall::SPACE,
        seeds = [b"transfer-hook", mint.key().as_ref(), sent_handle.as_ref()],
        bump
    )]
    pub hook_record: Account<'info, TransferReceiverHookCall>,
    /// System program used to create the one-shot hook marker.
    pub system_program: Program<'info, System>,
}

/// Calls a receiver hook after an operator-driven confidential transfer.
pub fn confidential_call_transfer_receiver_from<'info>(
    ctx: Context<'info, ConfidentialCallTransferReceiverFrom<'info>>,
    sent_handle: [u8; 32],
    callback_success_handle: [u8; 32],
    receiver_instruction_data: Vec<u8>,
) -> Result<()> {
    assert_active_operator_record(
        &ctx.accounts.operator_record,
        &ctx.accounts.from_account,
        ctx.accounts.operator.key(),
    )?;
    call_transfer_receiver_hook(
        &ctx.accounts.mint,
        &ctx.accounts.from_account,
        &ctx.accounts.to_account,
        PreviousTransferIntent::Operator {
            operator: ctx.accounts.operator.key(),
            operator_record: ctx.accounts.operator_record.key(),
        },
        &ctx.accounts.compute_signer,
        &ctx.accounts.sent_amount_acl,
        &ctx.accounts.callback_success_acl,
        &ctx.accounts.receiver_program,
        &ctx.accounts.instructions_sysvar.to_account_info(),
        ctx.remaining_accounts,
        sent_handle,
        callback_success_handle,
        receiver_instruction_data,
    )?;
    write_transfer_receiver_hook_call(
        &mut ctx.accounts.hook_record,
        ctx.accounts.mint.key(),
        ctx.accounts.from_account.key(),
        ctx.accounts.to_account.key(),
        sent_handle,
        ctx.accounts.sent_amount_acl.key(),
        callback_success_handle,
        ctx.accounts.callback_success_acl.key(),
        ctx.accounts.receiver_program.key(),
        ctx.accounts.operator.key(),
        ctx.bumps.hook_record,
    );
    Ok(())
}
