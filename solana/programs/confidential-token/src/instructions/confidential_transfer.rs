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
    /// CHECK: underlying SPL mint wrapped by `mint`. Token program is this account's owner.
    pub underlying_mint: UncheckedAccount<'info>,
    /// CHECK: ATA of `from_account.owner` on `underlying_mint`. Uninitialized → not frozen.
    pub from_ata: UncheckedAccount<'info>,
    /// CHECK: ATA of `to_account.owner`. May equal `from_ata` on self-transfer.
    #[account(dup)]
    pub to_ata: UncheckedAccount<'info>,
    /// Sender token account.
    #[account(mut)]
    pub from_account: Box<Account<'info, ConfidentialTokenAccount>>,
    // Anchor 1 rejects duplicate mutable Account<T> values unless the account opts in.
    // A self-transfer is a supported no-op, so from_account and to_account may be equal.
    #[account(mut, dup)]
    pub to_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Sender's stable balance `EncryptedValue` encrypted value account; read for the current
    /// handle and replaced in place by this execution's CPI.
    #[account(mut, address = from_account.balance_encrypted_value)]
    pub from_balance_value: Box<Account<'info, zama_host::EncryptedValue>>,
    /// Recipient's stable balance `EncryptedValue` encrypted value account.
    #[account(mut, dup, address = to_account.balance_encrypted_value)]
    pub to_balance_value: Box<Account<'info, zama_host::EncryptedValue>>,
    /// CHECK: stable `transferred_amount` encrypted value account for `from_account`; created on
    /// the sender's first transfer, replaced thereafter.
    #[account(mut, address = encrypted_value_address(mint.key(), from_account.key(), encrypted_transferred_amount_label()).0)]
    pub transferred_amount_value: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used for FHE operations.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// System program used for ACL account creation.
    pub system_program: Program<'info, System>,
    /// CHECK: forwarded verbatim into the ZamaHost `fhe_execute` CPI, which validates it against the
    /// canonical `["hcu-block-meter", program, mint]` PDA. The per-mint HCU block meter — supplied
    /// by an untrusted mint under a metering-band cap, omitted otherwise.
    #[account(mut)]
    pub hcu_block_meter: Option<UncheckedAccount<'info>>,
    /// CHECK: forwarded verbatim into the ZamaHost `fhe_execute` CPI, which validates it. The HCU
    /// trust witness — present + valid bypasses the cap; absent means untrusted (metered).
    pub hcu_trusted_app_record: Option<UncheckedAccount<'info>>,
    /// CHECK: the recipient program's receipt value (see [`TransferReceipt`]); created on its
    /// first write, accumulated thereafter. The host checks its canonical address.
    #[account(mut)]
    pub receipt_value: Option<UncheckedAccount<'info>>,
    /// The recipient program's PDA controlling the receipt, signing through `invoke_signed`.
    pub receipt_authority: Option<Signer<'info>>,
}

impl<'info> ConfidentialTransfer<'info> {
    pub(crate) fn as_transfer_accounts<'a>(
        &'a self,
        remaining_accounts: &'a [AccountInfo<'info>],
    ) -> TransferAccounts<'a, 'info> {
        TransferAccounts {
            payer: &self.payer,
            transfer_authority: &self.owner,
            mint: &self.mint,
            from_account: &self.from_account,
            to_account: &self.to_account,
            from_balance_value: self.from_balance_value.to_account_info(),
            to_balance_value: self.to_balance_value.to_account_info(),
            transferred_amount_value: self.transferred_amount_value.to_account_info(),
            zama_event_authority: &self.zama_event_authority,
            zama_program: &self.zama_program,
            host_config: &self.host_config,
            remaining_accounts,
            system_program: &self.system_program,
            hcu_block_meter: self
                .hcu_block_meter
                .as_ref()
                .map(|account| account.to_account_info()),
            hcu_trusted_app_record: self
                .hcu_trusted_app_record
                .as_ref()
                .map(|account| account.to_account_info()),
            underlying_mint: self.underlying_mint.to_account_info(),
            from_ata: self.from_ata.to_account_info(),
            to_ata: self.to_ata.to_account_info(),
            receipt: None,
        }
    }
}

/// Transfers an encrypted amount by updating the sender and recipient balance handles, and
/// writes the recipient program's receipt when one is asked for.
pub fn confidential_transfer<'info>(
    ctx: Context<'info, ConfidentialTransfer<'info>>,
    amount_attestation: zama_host::CoprocessorInputAttestation,
    receipt: Option<TransferReceipt>,
) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.from_account.owner,
        ctx.accounts.owner.key(),
        ConfidentialTokenError::OwnerMismatch
    );
    let mut accounts = ctx.accounts.as_transfer_accounts(ctx.remaining_accounts);
    accounts.receipt = ReceiptAccounts::bind(
        receipt,
        ctx.accounts.receipt_value.as_ref(),
        ctx.accounts.receipt_authority.as_ref(),
    )?;
    let outcome = execute_transfer(accounts, TransferAmountSource::Attested(amount_attestation))?;
    if let Some(outcome) = outcome {
        emit_transfer_events(&ctx, &outcome)?;
    }
    Ok(())
}

/// The three lifecycle events of a transfer. Kept out of the handler's stack frame: with the receipt
/// arguments the handler sits at the SBF 4 KiB stack limit, and the event CPIs each stage an
/// instruction on the stack.
#[inline(never)]
fn emit_transfer_events<'info>(
    ctx: &Context<'info, ConfidentialTransfer<'info>>,
    outcome: &TransferOutcome,
) -> Result<()> {
    {
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
/// account is added: `amount_value`, the encrypted amount to spend. It is read-only — the persistent
/// operand the execution reads — and is never replaced or consumed; only the two balance encrypted value accounts
/// change through the same `ge -> sub -> select` debit and `add` credit.
#[derive(Accounts)]
#[event_cpi]
pub struct ConfidentialTransferFromValue<'info> {
    /// Sender and transfer authority. Must control `amount_value` (the spend gate).
    pub owner: Signer<'info>,
    /// Pays rent for the transferred-amount encrypted value account on its first bind.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Confidential mint.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// CHECK: underlying SPL mint wrapped by `mint`. Token program is this account's owner.
    pub underlying_mint: UncheckedAccount<'info>,
    /// CHECK: ATA of `from_account.owner` on `underlying_mint`. Uninitialized → not frozen.
    pub from_ata: UncheckedAccount<'info>,
    /// CHECK: ATA of `to_account.owner`. May equal `from_ata` on self-transfer.
    #[account(dup)]
    pub to_ata: UncheckedAccount<'info>,
    /// Sender token account.
    #[account(mut)]
    pub from_account: Box<Account<'info, ConfidentialTokenAccount>>,
    // Anchor 1 rejects duplicate mutable Account<T> values unless the account opts in.
    // A self-transfer is a supported no-op, so from_account and to_account may be equal.
    #[account(mut, dup)]
    pub to_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Sender's stable balance `EncryptedValue` encrypted value account; read for the current
    /// handle and replaced in place by this execution's CPI.
    #[account(mut, address = from_account.balance_encrypted_value)]
    pub from_balance_value: Box<Account<'info, zama_host::EncryptedValue>>,
    /// Recipient's stable balance `EncryptedValue` encrypted value account.
    #[account(mut, dup, address = to_account.balance_encrypted_value)]
    pub to_balance_value: Box<Account<'info, zama_host::EncryptedValue>>,
    /// CHECK: stable `transferred_amount` encrypted value account for `from_account`; created on
    /// the sender's first transfer, replaced thereafter.
    #[account(mut, address = encrypted_value_address(mint.key(), from_account.key(), encrypted_transferred_amount_label()).0)]
    pub transferred_amount_value: UncheckedAccount<'info>,
    /// The existing encrypted amount to spend: a computed `euint64` handle. Read-only persistent
    /// operand — never replaced, never consumed. Its address is the canonical PDA of its own
    /// `(program, authority, scope, label)` fields, so an encrypted value account from any app may
    /// be passed here when that app's value authority is the signing `owner` (a program PDA
    /// authorizing through `invoke_signed`), or when it is one of the sender's own token values.
    pub amount_value: Box<Account<'info, zama_host::EncryptedValue>>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used for FHE operations.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// System program used for ACL account creation.
    pub system_program: Program<'info, System>,
    /// CHECK: forwarded verbatim into the ZamaHost `fhe_execute` CPI, which validates it against the
    /// canonical `["hcu-block-meter", program, mint]` PDA. The per-mint HCU block meter — supplied
    /// by an untrusted mint under a metering-band cap, omitted otherwise.
    #[account(mut)]
    pub hcu_block_meter: Option<UncheckedAccount<'info>>,
    /// CHECK: forwarded verbatim into the ZamaHost `fhe_execute` CPI, which validates it. The HCU
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
            transfer_authority: &self.owner,
            mint: &self.mint,
            from_account: &self.from_account,
            to_account: &self.to_account,
            from_balance_value: self.from_balance_value.to_account_info(),
            to_balance_value: self.to_balance_value.to_account_info(),
            transferred_amount_value: self.transferred_amount_value.to_account_info(),
            zama_event_authority: &self.zama_event_authority,
            zama_program: &self.zama_program,
            host_config: &self.host_config,
            remaining_accounts,
            system_program: &self.system_program,
            hcu_block_meter: self
                .hcu_block_meter
                .as_ref()
                .map(|account| account.to_account_info()),
            hcu_trusted_app_record: self
                .hcu_trusted_app_record
                .as_ref()
                .map(|account| account.to_account_info()),
            underlying_mint: self.underlying_mint.to_account_info(),
            from_ata: self.from_ata.to_account_info(),
            to_ata: self.to_ata.to_account_info(),
            receipt: None,
        }
    }
}

/// Transfers an encrypted amount taken from an existing on-chain `EncryptedValue`, updating the
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
    assert_amount_value_spendable(
        amount_value,
        ctx.accounts.owner.key(),
        ctx.accounts.from_account.key(),
    )?;
    let amount_value_info = amount_value.to_account_info();
    let outcome = execute_transfer(
        ctx.accounts.as_transfer_accounts(ctx.remaining_accounts),
        TransferAmountSource::ExistingValue {
            amount_value: amount_value_info,
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
