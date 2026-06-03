//! Invokes receiver hooks for direct confidential transfers.

use super::*;

/// Accounts for calling a receiver hook after a confidential transfer.
#[derive(Accounts)]
#[instruction(sent_handle: [u8; 32])]
pub struct ConfidentialCallTransferReceiver<'info> {
    /// Original sender owner and rent payer for the hook invocation transaction.
    #[account(mut)]
    pub caller: Signer<'info>,
    /// Confidential mint.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Original sender token account.
    pub from_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Original recipient token account.
    pub to_account: Box<Account<'info, ConfidentialTokenAccount>>,
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
        payer = caller,
        space = 8 + TransferReceiverHookCall::SPACE,
        seeds = [b"transfer-hook", mint.key().as_ref(), sent_handle.as_ref()],
        bump
    )]
    pub hook_record: Account<'info, TransferReceiverHookCall>,
    /// System program used to create the one-shot hook marker.
    pub system_program: Program<'info, System>,
}

/// Calls an arbitrary receiver hook and verifies its encrypted callback-success result.
pub fn confidential_call_transfer_receiver<'info>(
    ctx: Context<'info, ConfidentialCallTransferReceiver<'info>>,
    sent_handle: [u8; 32],
    callback_success_handle: [u8; 32],
    receiver_instruction_data: Vec<u8>,
) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.caller.key(),
        ctx.accounts.from_account.owner,
        ConfidentialTokenError::OwnerMismatch
    );
    call_transfer_receiver_hook(
        &ctx.accounts.mint,
        &ctx.accounts.from_account,
        &ctx.accounts.to_account,
        PreviousTransferIntent::Direct {
            owner: ctx.accounts.caller.key(),
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
        ctx.accounts.caller.key(),
        ctx.bumps.hook_record,
    );
    Ok(())
}
