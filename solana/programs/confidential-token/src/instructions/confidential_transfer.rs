use super::*;

/// Transfers an encrypted amount by rotating the sender and recipient balance handles.
pub fn confidential_transfer(
    ctx: Context<ConfidentialTransfer>,
    amount_handle: [u8; 32],
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    require_keys_eq!(
        ctx.accounts.from_account.owner,
        ctx.accounts.owner.key(),
        ConfidentialTokenError::OwnerMismatch
    );
    let outcome = execute_transfer(
        ctx.accounts.as_transfer_accounts(),
        ctx.bumps.compute_signer,
        amount_handle,
    )?;
    if let Some(outcome) = outcome {
        emit_cpi!(ConfidentialTransferEvent {
            version: APP_EVENT_VERSION,
            mint: outcome.mint,
            from_owner: outcome.from_owner,
            from_token_account: outcome.from_token_account,
            to_owner: outcome.to_owner,
            to_token_account: outcome.to_token_account,
            transferred_handle: outcome.transferred_handle,
            transferred_acl_record: outcome.transferred_acl_record,
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
            reason: BalanceHandleUpdateReason::TransferDebit,
        });
        emit_cpi!(BalanceHandleUpdatedEvent {
            version: APP_EVENT_VERSION,
            mint: outcome.mint,
            owner: outcome.to_owner,
            token_account: outcome.to_token_account,
            old_handle: outcome.old_to_handle,
            old_acl_record: outcome.old_to_acl_record,
            new_handle: outcome.new_to_handle,
            new_acl_record: outcome.new_to_acl_record,
            reason: BalanceHandleUpdateReason::TransferCredit,
        });
    }
    Ok(())
}
