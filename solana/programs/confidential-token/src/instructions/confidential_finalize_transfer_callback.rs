use super::*;

/// Finalizes a prepared callback settlement by crediting refund and recording final transfer.
pub fn confidential_finalize_transfer_callback(
    ctx: Context<ConfidentialFinalizeTransferCallback>,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    let outcome = finalize_transfer_callback_settlement(
        ctx.accounts.as_finalize_callback_accounts(),
        ctx.bumps.compute_signer,
    )?;
    emit_cpi!(ConfidentialTransferEvent {
        version: APP_EVENT_VERSION,
        mint: outcome.mint,
        from_owner: outcome.to_owner,
        from_token_account: outcome.to_token_account,
        to_owner: outcome.from_owner,
        to_token_account: outcome.from_token_account,
        transferred_handle: outcome.refund_handle,
        transferred_acl_record: outcome.refund_acl_record,
    });
    emit_cpi!(BalanceHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: outcome.mint,
        owner: outcome.from_owner,
        token_account: outcome.from_token_account,
        old_handle: outcome.old_from_handle,
        old_acl_record: outcome.old_from_acl_record,
        new_handle: outcome.new_from_handle,
        new_acl_record: outcome.new_from_acl_record,
        reason: BalanceHandleUpdateReason::TransferCallbackRefundCredit,
    });
    Ok(())
}
