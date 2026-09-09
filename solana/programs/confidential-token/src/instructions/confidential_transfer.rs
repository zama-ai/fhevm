//! Transfers encrypted balances between confidential token accounts.

use super::*;

/// Accounts for confidential balance transfer.
#[derive(Accounts)]
#[event_cpi]
pub struct ConfidentialTransfer<'info> {
    /// Sender and transfer authority.
    pub owner: Signer<'info>,
    /// Pays rent for the transferred-amount encrypted State on its first bind.
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
    /// Sender state: the host reads and updates its balance slot.
    #[account(mut, address = encrypted_state_address(mint.key(), from_account.key()).0)]
    pub from_state: Box<Account<'info, zama_host::EncryptedState>>,
    /// Recipient state: the host reads and updates its balance slot.
    #[account(mut, dup, address = encrypted_state_address(mint.key(), to_account.key()).0)]
    pub to_state: Box<Account<'info, zama_host::EncryptedState>>,
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
    /// CHECK: canonical consumer state, authenticated by its authority and the host.
    pub result_state: Option<UncheckedAccount<'info>>,
    /// CHECK: host-owned scratch belonging to result_state.
    #[account(mut)]
    pub result_scratch: Option<UncheckedAccount<'info>>,
    pub result_authority: Option<Signer<'info>>,
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
            from_state: self.from_state.to_account_info(),
            to_state: self.to_state.to_account_info(),
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
            result_grant: None,
        }
    }
}

/// Updates both balances and returns the transferred handle, optionally granting its use
/// to the caller through scratch.
pub fn confidential_transfer<'info>(
    ctx: Context<'info, ConfidentialTransfer<'info>>,
    amount_attestation: zama_host::CoprocessorInputAttestation,
) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.from_account.owner,
        ctx.accounts.owner.key(),
        ConfidentialTokenError::OwnerMismatch
    );
    let mut accounts = ctx.accounts.as_transfer_accounts(ctx.remaining_accounts);
    accounts.result_grant = ResultGrantAccounts::bind(
        ctx.accounts.result_state.as_ref(),
        ctx.accounts.result_scratch.as_ref(),
        ctx.accounts.result_authority.as_ref(),
    )?;
    let outcome = execute_transfer(accounts, TransferAmountSource::Attested(amount_attestation))?;
    if let Some(outcome) = outcome {
        emit_transfer_events(&ctx, &outcome)?;
        anchor_lang::solana_program::program::set_return_data(&outcome.transferred_handle);
    }
    Ok(())
}

/// Keep event CPI instruction allocations outside the handler's bounded SBF stack frame.
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
            transferred_encrypted_state: outcome.transferred_encrypted_state,
        });
        emit_cpi!(BalanceHandleUpdatedEvent {
            version: APP_EVENT_VERSION,
            mint: outcome.mint,
            owner: outcome.from_owner,
            token_account: outcome.from_token_account,
            old_handle: outcome.old_from_handle,
            old_encrypted_state: outcome.from_encrypted_state,
            new_handle: outcome.new_from_handle,
            new_encrypted_state: outcome.from_encrypted_state,
            reason: BalanceHandleUpdateReason::TransferDebit,
        });
        emit_cpi!(BalanceHandleUpdatedEvent {
            version: APP_EVENT_VERSION,
            mint: outcome.mint,
            owner: outcome.to_owner,
            token_account: outcome.to_token_account,
            old_handle: outcome.old_to_handle,
            old_encrypted_state: outcome.to_encrypted_state,
            new_handle: outcome.new_to_handle,
            new_encrypted_state: outcome.to_encrypted_state,
            reason: BalanceHandleUpdateReason::TransferCredit,
        });
    }
    Ok(())
}

/// Accounts for a transfer whose amount comes from a state slot or scratch grant.
/// The host verifies the input permission; the token verifies the sender can spend the balance.
#[derive(Accounts)]
#[event_cpi]
pub struct ConfidentialTransferFromValue<'info> {
    /// Sender and transfer authority. Must control `amount_state` (the spend gate).
    pub owner: Signer<'info>,
    /// Pays rent for the transferred-amount encrypted State on its first bind.
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
    /// Sender state: the host reads and updates its balance slot.
    #[account(mut, address = encrypted_state_address(mint.key(), from_account.key()).0)]
    pub from_state: Box<Account<'info, zama_host::EncryptedState>>,
    /// Recipient state: the host reads and updates its balance slot.
    #[account(mut, dup, address = encrypted_state_address(mint.key(), to_account.key()).0)]
    pub to_state: Box<Account<'info, zama_host::EncryptedState>>,
    /// CHECK: state containing a stored amount, when amount_source is Slot.
    pub amount_state: Option<UncheckedAccount<'info>>,
    pub amount_authority: Option<Signer<'info>>,
    /// CHECK: host validates an exact handle grant to the sender token state.
    pub amount_scratch: Option<UncheckedAccount<'info>>,
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
            from_state: self.from_state.to_account_info(),
            to_state: self.to_state.to_account_info(),
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
            result_grant: None,
        }
    }
}

/// Transfers an amount from a state slot or scratch grant and returns the transferred handle.
pub fn confidential_transfer_from_value<'info>(
    ctx: Context<'info, ConfidentialTransferFromValue<'info>>,
    amount_source: TransferInput,
) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.from_account.owner,
        ctx.accounts.owner.key(),
        ConfidentialTokenError::OwnerMismatch
    );
    let source = match amount_source {
        TransferInput::Slot { key } => {
            require!(
                ctx.accounts.amount_scratch.is_none(),
                ConfidentialTokenError::AmountAclMismatch
            );
            let info = ctx
                .accounts
                .amount_state
                .as_ref()
                .ok_or_else(|| error!(ConfidentialTokenError::AmountAclMismatch))?
                .to_account_info();
            let state = fhe::read_state(&info)?;
            let authority = ctx
                .accounts
                .amount_authority
                .as_ref()
                .map(|a| a.to_account_info());
            let spender = authority
                .as_ref()
                .map(|a| a.key())
                .unwrap_or(ctx.accounts.owner.key());
            assert_amount_state_spendable(&state, key, spender, ctx.accounts.from_account.key())?;
            TransferAmountSource::StateSlot {
                amount_state: info,
                authority,
                key,
            }
        }
        TransferInput::Grant { handle } => {
            require!(
                ctx.accounts.amount_state.is_none() && ctx.accounts.amount_authority.is_none(),
                ConfidentialTokenError::AmountAclMismatch
            );
            let scratch = ctx
                .accounts
                .amount_scratch
                .as_ref()
                .ok_or_else(|| error!(ConfidentialTokenError::AmountAclMismatch))?
                .to_account_info();
            TransferAmountSource::Grant { scratch, handle }
        }
    };
    let outcome = execute_transfer(
        ctx.accounts.as_transfer_accounts(ctx.remaining_accounts),
        source,
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
            transferred_encrypted_state: outcome.transferred_encrypted_state,
        });
        emit_cpi!(BalanceHandleUpdatedEvent {
            version: APP_EVENT_VERSION,
            mint: outcome.mint,
            owner: outcome.from_owner,
            token_account: outcome.from_token_account,
            old_handle: outcome.old_from_handle,
            old_encrypted_state: outcome.from_encrypted_state,
            new_handle: outcome.new_from_handle,
            new_encrypted_state: outcome.from_encrypted_state,
            reason: BalanceHandleUpdateReason::TransferDebit,
        });
        emit_cpi!(BalanceHandleUpdatedEvent {
            version: APP_EVENT_VERSION,
            mint: outcome.mint,
            owner: outcome.to_owner,
            token_account: outcome.to_token_account,
            old_handle: outcome.old_to_handle,
            old_encrypted_state: outcome.to_encrypted_state,
            new_handle: outcome.new_to_handle,
            new_encrypted_state: outcome.to_encrypted_state,
            reason: BalanceHandleUpdateReason::TransferCredit,
        });
        anchor_lang::solana_program::program::set_return_data(&outcome.transferred_handle);
    }
    Ok(())
}

/// A persistent slot or an exact transient grant addressed to the sender token state.
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum TransferInput {
    Slot { key: [u8; 32] },
    Grant { handle: [u8; 32] },
}
