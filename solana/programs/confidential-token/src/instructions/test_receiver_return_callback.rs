//! Test receiver hook that returns a supplied callback-success witness.

use super::*;

/// Empty account set for the test receiver hook endpoint.
#[derive(Accounts)]
pub struct TestReceiverReturnCallback {}

/// Test-only receiver endpoint that returns the supplied callback-success witness.
pub fn test_receiver_return_callback(
    ctx: Context<TestReceiverReturnCallback>,
    mint: Pubkey,
    from_token_account: Pubkey,
    to_token_account: Pubkey,
    sent_handle: [u8; 32],
    sent_acl_record: Pubkey,
    callback_success_handle: [u8; 32],
    callback_success_acl_record: Pubkey,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    set_return_data(&transfer_receiver_return_data(
        mint,
        from_token_account,
        to_token_account,
        sent_handle,
        sent_acl_record,
        callback_success_handle,
        callback_success_acl_record,
    ));
    Ok(())
}
