//! Transfers encrypted balances between confidential token accounts.

use super::*;

/// Accounts for confidential balance transfer.
#[derive(Accounts)]
#[event_cpi]
pub struct ConfidentialTransfer<'info> {
    /// Sender and transfer authority.
    pub owner: Signer<'info>,
    /// Pays rent for the transferred-amount encrypted value account on its first bind.
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
    /// Sender's stable balance `EncryptedValue` encrypted value account; read for the current
    /// handle and superseded in place by this eval's CPI.
    #[account(mut, address = from_account.balance_encrypted_value)]
    pub from_balance_value: Box<Account<'info, zama_host::EncryptedValue>>,
    /// Recipient's stable balance `EncryptedValue` encrypted value account.
    #[account(mut, dup, address = to_account.balance_encrypted_value)]
    pub to_balance_value: Box<Account<'info, zama_host::EncryptedValue>>,
    /// CHECK: stable `transferred_amount` encrypted value account for `from_account`; created on
    /// the sender's first transfer, superseded thereafter.
    #[account(mut, address = encrypted_value_address(mint.key(), from_account.key(), transferred_amount_label()).0)]
    pub transferred_amount_value: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used for FHE operations.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// System program used for ACL account creation.
    pub system_program: Program<'info, System>,
    /// CHECK: forwarded verbatim into the ZamaHost `fhe_eval` CPI, which validates it against the
    /// canonical `["hcu-block-meter", compute_signer]` PDA. The per-mint HCU block meter — supplied
    /// by an untrusted mint under a metering-band cap, omitted otherwise.
    #[account(mut)]
    pub hcu_block_meter: Option<UncheckedAccount<'info>>,
    /// CHECK: forwarded verbatim into the ZamaHost `fhe_eval` CPI, which validates it. The HCU
    /// trust witness — present + valid bypasses the cap; absent means untrusted (metered).
    pub hcu_trusted_app_record: Option<UncheckedAccount<'info>>,
}

impl<'info> ConfidentialTransfer<'info> {
    pub(crate) fn as_transfer_accounts<'a>(
        &'a self,
        remaining_accounts: &'a [AccountInfo<'info>],
    ) -> TransferAccounts<'a, 'info> {
        TransferAccounts {
            payer: &self.payer,
            transfer_authority: self.owner.key(),
            mint: &self.mint,
            from_account: &self.from_account,
            to_account: &self.to_account,
            compute_signer: &self.compute_signer,
            from_balance_value: self.from_balance_value.to_account_info(),
            to_balance_value: self.to_balance_value.to_account_info(),
            transferred_amount_value: self.transferred_amount_value.to_account_info(),
            zama_event_authority: &self.zama_event_authority,
            zama_program: &self.zama_program,
            host_config: &self.host_config,
            deny_subject_records: remaining_accounts,
            system_program: &self.system_program,
            hcu_block_meter: self
                .hcu_block_meter
                .as_ref()
                .map(|account| account.to_account_info()),
            hcu_trusted_app_record: self
                .hcu_trusted_app_record
                .as_ref()
                .map(|account| account.to_account_info()),
        }
    }
}

/// Transfers an encrypted amount by rotating the sender and recipient balance handles.
pub fn confidential_transfer<'info>(
    ctx: Context<'info, ConfidentialTransfer<'info>>,
    amount_attestation: zama_host::CoprocessorInputAttestation,
) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.from_account.owner,
        ctx.accounts.owner.key(),
        ConfidentialTokenError::OwnerMismatch
    );
    let outcome = execute_transfer(
        ctx.accounts.as_transfer_accounts(ctx.remaining_accounts),
        ctx.bumps.compute_signer,
        TransferAmountSource::Attested(amount_attestation),
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
            transferred_encrypted_value: outcome.transferred_encrypted_value,
        });
        emit_cpi!(BalanceHandleUpdatedEvent {
            version: APP_EVENT_VERSION,
            mint: outcome.mint,
            owner: outcome.from_owner,
            token_account: outcome.from_token_account,
            old_handle: outcome.old_from_handle,
            old_encrypted_value: outcome.from_encrypted_value,
            new_handle: outcome.new_from_handle,
            new_encrypted_value: outcome.from_encrypted_value,
            reason: BalanceHandleUpdateReason::TransferDebit,
        });
        emit_cpi!(BalanceHandleUpdatedEvent {
            version: APP_EVENT_VERSION,
            mint: outcome.mint,
            owner: outcome.to_owner,
            token_account: outcome.to_token_account,
            old_handle: outcome.old_to_handle,
            old_encrypted_value: outcome.to_encrypted_value,
            new_handle: outcome.new_to_handle,
            new_encrypted_value: outcome.to_encrypted_value,
            reason: BalanceHandleUpdateReason::TransferCredit,
        });
    }
    Ok(())
}

/// Accounts for a confidential transfer that spends an existing on-chain `EncryptedValue` as the
/// amount, instead of a freshly attested client-side encryption.
///
/// Identical to [`ConfidentialTransfer`] except the 190-byte attestation argument is gone and one
/// account is added: `amount_value`, the encrypted amount to spend. It is read-only — the durable
/// operand the eval reads — and is never superseded or consumed; only the two balance encrypted value accounts
/// change through the same `ge -> sub -> select` debit and `add` credit.
#[derive(Accounts)]
#[event_cpi]
pub struct ConfidentialTransferFromValue<'info> {
    /// Sender and transfer authority. Must be in `amount_value`'s subject set (the spend gate).
    pub owner: Signer<'info>,
    /// Pays rent for the transferred-amount encrypted value account on its first bind.
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
    /// Sender's stable balance `EncryptedValue` encrypted value account; read for the current
    /// handle and superseded in place by this eval's CPI.
    #[account(mut, address = from_account.balance_encrypted_value)]
    pub from_balance_value: Box<Account<'info, zama_host::EncryptedValue>>,
    /// Recipient's stable balance `EncryptedValue` encrypted value account.
    #[account(mut, dup, address = to_account.balance_encrypted_value)]
    pub to_balance_value: Box<Account<'info, zama_host::EncryptedValue>>,
    /// CHECK: stable `transferred_amount` encrypted value account for `from_account`; created on
    /// the sender's first transfer, superseded thereafter.
    #[account(mut, address = encrypted_value_address(mint.key(), from_account.key(), transferred_amount_label()).0)]
    pub transferred_amount_value: UncheckedAccount<'info>,
    /// The existing encrypted amount to spend: a computed or received `euint64` handle. Read-only
    /// durable operand — never superseded, never consumed. Its address is the canonical PDA of its
    /// own `(acl_domain_key, app_account, encrypted_value_label)` fields, so a encrypted value account from any app
    /// may be passed here once its owner has granted the mint's compute subject via `allow_subjects`.
    pub amount_value: Box<Account<'info, zama_host::EncryptedValue>>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used for FHE operations.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// System program used for ACL account creation.
    pub system_program: Program<'info, System>,
    /// CHECK: forwarded verbatim into the ZamaHost `fhe_eval` CPI, which validates it against the
    /// canonical `["hcu-block-meter", compute_signer]` PDA. The per-mint HCU block meter — supplied
    /// by an untrusted mint under a metering-band cap, omitted otherwise.
    #[account(mut)]
    pub hcu_block_meter: Option<UncheckedAccount<'info>>,
    /// CHECK: forwarded verbatim into the ZamaHost `fhe_eval` CPI, which validates it. The HCU
    /// trust witness — present + valid bypasses the cap; absent means untrusted (metered).
    pub hcu_trusted_app_record: Option<UncheckedAccount<'info>>,
}

impl<'info> ConfidentialTransferFromValue<'info> {
    pub(crate) fn as_transfer_accounts<'a>(
        &'a self,
        remaining_accounts: &'a [AccountInfo<'info>],
    ) -> TransferAccounts<'a, 'info> {
        TransferAccounts {
            payer: &self.payer,
            transfer_authority: self.owner.key(),
            mint: &self.mint,
            from_account: &self.from_account,
            to_account: &self.to_account,
            compute_signer: &self.compute_signer,
            from_balance_value: self.from_balance_value.to_account_info(),
            to_balance_value: self.to_balance_value.to_account_info(),
            transferred_amount_value: self.transferred_amount_value.to_account_info(),
            zama_event_authority: &self.zama_event_authority,
            zama_program: &self.zama_program,
            host_config: &self.host_config,
            deny_subject_records: remaining_accounts,
            system_program: &self.system_program,
            hcu_block_meter: self
                .hcu_block_meter
                .as_ref()
                .map(|account| account.to_account_info()),
            hcu_trusted_app_record: self
                .hcu_trusted_app_record
                .as_ref()
                .map(|account| account.to_account_info()),
        }
    }
}

/// Transfers an encrypted amount taken from an existing on-chain `EncryptedValue`, rotating the
/// sender and recipient balance handles. The amount value is spent read-only.
pub fn confidential_transfer_from_value<'info>(
    ctx: Context<'info, ConfidentialTransferFromValue<'info>>,
) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.from_account.owner,
        ctx.accounts.owner.key(),
        ConfidentialTokenError::OwnerMismatch
    );
    let amount_value = &ctx.accounts.amount_value;
    // Reject a non-euint64 amount early for a clear error, before the eval builder / host would
    // reject the same handle deeper in the CPI (the host's binary type validation still covers it).
    require!(
        zama_host::handle_fhe_type(amount_value.current_handle) == BALANCE_FHE_TYPE,
        ConfidentialTokenError::AmountHandleTypeMismatch
    );
    // Token-level spend gate — EVM `FHE.isAllowed(amount, msg.sender)` parity: the signing owner
    // must be in the amount value's subject set. App-level by design; the host stays role-blind.
    require!(
        amount_value.has_subject(ctx.accounts.owner.key()),
        ConfidentialTokenError::AmountSpendSubjectMismatch
    );
    let amount_handle = amount_value.current_handle;
    let amount_value_info = amount_value.to_account_info();
    let outcome = execute_transfer(
        ctx.accounts.as_transfer_accounts(ctx.remaining_accounts),
        ctx.bumps.compute_signer,
        TransferAmountSource::ExistingValue {
            amount_value: amount_value_info,
            amount_handle,
        },
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
            transferred_encrypted_value: outcome.transferred_encrypted_value,
        });
        emit_cpi!(BalanceHandleUpdatedEvent {
            version: APP_EVENT_VERSION,
            mint: outcome.mint,
            owner: outcome.from_owner,
            token_account: outcome.from_token_account,
            old_handle: outcome.old_from_handle,
            old_encrypted_value: outcome.from_encrypted_value,
            new_handle: outcome.new_from_handle,
            new_encrypted_value: outcome.from_encrypted_value,
            reason: BalanceHandleUpdateReason::TransferDebit,
        });
        emit_cpi!(BalanceHandleUpdatedEvent {
            version: APP_EVENT_VERSION,
            mint: outcome.mint,
            owner: outcome.to_owner,
            token_account: outcome.to_token_account,
            old_handle: outcome.old_to_handle,
            old_encrypted_value: outcome.to_encrypted_value,
            new_handle: outcome.new_to_handle,
            new_encrypted_value: outcome.to_encrypted_value,
            reason: BalanceHandleUpdateReason::TransferCredit,
        });
    }
    Ok(())
}
