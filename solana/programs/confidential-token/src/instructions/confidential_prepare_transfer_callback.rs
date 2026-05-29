use super::*;

/// Prepares receiver callback settlement by computing the encrypted refund.
pub fn confidential_prepare_transfer_callback(
    ctx: Context<ConfidentialPrepareTransferCallback>,
    sent_handle: [u8; 32],
    callback_success_handle: [u8; 32],
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    let outcome = prepare_transfer_callback_settlement(
        ctx.accounts.as_prepare_callback_accounts(),
        ctx.bumps.compute_signer,
        ctx.bumps.settlement_record,
        sent_handle,
        callback_success_handle,
    )?;
    emit_cpi!(BalanceHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: outcome.mint,
        owner: outcome.to_owner,
        token_account: outcome.to_token_account,
        old_handle: outcome.old_to_handle,
        old_acl_record: outcome.old_to_acl_record,
        new_handle: outcome.new_to_handle,
        new_acl_record: outcome.new_to_acl_record,
        reason: BalanceHandleUpdateReason::TransferCallbackRefundDebit,
    });
    Ok(())
}
