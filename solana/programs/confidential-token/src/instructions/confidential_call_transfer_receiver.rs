use super::*;

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
