//! Transfers encrypted balances between confidential token accounts.

use super::*;

/// Accounts for confidential balance transfer.
#[derive(Accounts)]
#[event_cpi]
pub struct ConfidentialTransfer<'info> {
    /// Sender and transfer authority.
    pub owner: Signer<'info>,
    /// Pays rent for output ACL records.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Confidential mint.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Sender token account.
    #[account(mut)]
    pub from_account: Box<Account<'info, ConfidentialTokenAccount>>,
    // Anchor 1 rejects duplicate mutable Account<T> values unless the account opts in.
    // A self-transfer is a supported no-op, so from_account and to_account may be equal.
    #[account(mut, dup)]
    pub to_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    /// Sender current balance ACL record.
    pub from_current_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// Recipient current balance ACL record.
    pub to_current_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub from_output_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub transferred_amount_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub to_output_acl: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used for FHE operations.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// System program used for ACL account creation.
    pub system_program: Program<'info, System>,
    /// CHECK: forwarded verbatim into the ZamaHost `fhe_eval` CPI, which validates it. The per-app
    /// HCU block meter — supplied by an untrusted app under a metering-band cap, omitted otherwise.
    #[account(mut)]
    pub hcu_block_meter: Option<UncheckedAccount<'info>>,
    /// CHECK: forwarded verbatim into the ZamaHost `fhe_eval` CPI, which validates it. The HCU
    /// trust witness — present + valid bypasses the cap; absent means untrusted (metered).
    pub hcu_trusted_app_record: Option<UncheckedAccount<'info>>,
    /// CHECK: validated against the canonical `["hcu-authority", mint]` PDA and program-signed
    /// into the CPI. The mint-scoped identity the host block cap meters and trusts — mandatory
    /// on every eval, matching the host account shape.
    pub hcu_authority: UncheckedAccount<'info>,
}

impl<'info> ConfidentialTransfer<'info> {
    pub(crate) fn as_transfer_accounts(&mut self) -> TransferAccounts<'_, 'info> {
        TransferAccounts {
            payer: &self.payer,
            transfer_authority: self.owner.key(),
            mint: &self.mint,
            from_account: &mut self.from_account,
            to_account: &mut self.to_account,
            compute_signer: &self.compute_signer,
            from_current_compute_acl: self.from_current_compute_acl.as_ref(),
            to_current_compute_acl: self.to_current_compute_acl.as_ref(),
            from_output_acl: self.from_output_acl.to_account_info(),
            transferred_amount_acl: self.transferred_amount_acl.to_account_info(),
            to_output_acl: self.to_output_acl.to_account_info(),
            zama_event_authority: &self.zama_event_authority,
            zama_program: &self.zama_program,
            host_config: &self.host_config,
            system_program: &self.system_program,
            hcu_block_meter: self
                .hcu_block_meter
                .as_ref()
                .map(|account| account.to_account_info()),
            hcu_trusted_app_record: self
                .hcu_trusted_app_record
                .as_ref()
                .map(|account| account.to_account_info()),
            hcu_authority: &self.hcu_authority,
        }
    }
}

/// Transfers an encrypted amount by rotating the sender and recipient balance handles.
pub fn confidential_transfer(
    ctx: Context<ConfidentialTransfer>,
    amount_attestation: zama_host::CoprocessorInputAttestation,
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
        amount_attestation,
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
