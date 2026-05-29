use super::*;

/// Accounts for initializing a confidential mint.
#[derive(Accounts)]
#[event_cpi]
pub struct InitializeMint<'info> {
    /// Mint authority and rent payer.
    #[account(mut)]
    pub authority: Signer<'info>,
    /// Confidential mint account created by this instruction.
    #[account(init, payer = authority, space = 8 + ConfidentialMint::SPACE)]
    pub mint: Account<'info, ConfidentialMint>,
    /// Underlying SPL mint wrapped by this confidential mint.
    pub underlying_mint: Account<'info, SplMint>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    /// CHECK: Mint-scoped app authority for total-supply handles.
    #[account(seeds = [b"total-supply", mint.key().as_ref()], bump)]
    pub total_supply_authority: UncheckedAccount<'info>,
    /// CHECK: Ed25519 authority whose KMS response certificates disclose cleartexts.
    pub kms_verifier_authority: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub total_supply_acl_record: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used to create the initial total-supply handle.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Account<'info, zama_host::HostConfig>,
    /// System program used for account creation.
    pub system_program: Program<'info, System>,
}

/// Accounts for initializing a confidential token account.
#[derive(Accounts)]
#[event_cpi]
pub struct InitializeTokenAccount<'info> {
    /// Account owner and rent payer.
    #[account(mut)]
    pub owner: Signer<'info>,
    /// Confidential mint this account belongs to.
    pub mint: Account<'info, ConfidentialMint>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + ConfidentialTokenAccount::SPACE,
        seeds = [b"token-account", mint.key().as_ref(), owner.key().as_ref()],
        bump
    )]
    pub token_account: Account<'info, ConfidentialTokenAccount>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub acl_record: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used to create the initial balance handle.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Account<'info, zama_host::HostConfig>,
    /// System program used for account creation.
    pub system_program: Program<'info, System>,
}

/// Accounts for creating a token-scoped random encrypted amount.
#[derive(Accounts)]
#[event_cpi]
pub struct CreateRandomAmount<'info> {
    /// Token account owner and rent payer.
    #[account(mut)]
    pub owner: Signer<'info>,
    /// Confidential mint that scopes the encrypted amount.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Owner's confidential token account carrying the amount nonce allocator.
    #[account(mut)]
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub amount_acl_record: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used to create the random handle.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// System program used for ACL account creation.
    pub system_program: Program<'info, System>,
}

/// Accounts for wrapping public USDC into a confidential balance.
#[derive(Accounts)]
#[event_cpi]
pub struct WrapUsdc<'info> {
    /// Token owner and transfer authority.
    #[account(mut)]
    pub owner: Signer<'info>,
    /// Confidential mint.
    #[account(mut)]
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Confidential token account whose balance is increased.
    #[account(mut)]
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Underlying SPL mint.
    pub underlying_mint: Box<Account<'info, SplMint>>,
    /// Owner's source USDC token account.
    #[account(
        mut,
        constraint = user_usdc.mint == underlying_mint.key() @ ConfidentialTokenError::UnderlyingMintMismatch,
        constraint = user_usdc.owner == owner.key() @ ConfidentialTokenError::OwnerMismatch
    )]
    pub user_usdc: Box<Account<'info, TokenAccount>>,
    /// Program vault USDC token account.
    #[account(
        mut,
        constraint = vault_usdc.mint == underlying_mint.key() @ ConfidentialTokenError::UnderlyingMintMismatch,
        constraint = vault_usdc.owner == vault_authority.key() @ ConfidentialTokenError::VaultAuthorityMismatch
    )]
    pub vault_usdc: Box<Account<'info, TokenAccount>>,
    /// CHECK: PDA authority for the underlying-token vault.
    #[account(seeds = [b"vault-authority", mint.key().as_ref()], bump)]
    pub vault_authority: UncheckedAccount<'info>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    /// CHECK: Mint-scoped app authority for total-supply handles.
    #[account(seeds = [b"total-supply", mint.key().as_ref()], bump)]
    pub total_supply_authority: UncheckedAccount<'info>,
    /// Current balance ACL record used as the left-hand operand.
    pub current_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// Current total-supply ACL record used as the left-hand operand.
    pub current_total_supply_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub amount_compute_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub output_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub total_supply_output_acl: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used for FHE operations.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// SPL token program.
    pub token_program: Program<'info, Token>,
    /// System program used for ACL account creation.
    pub system_program: Program<'info, System>,
}

/// Accounts for confidential balance burn.
#[derive(Accounts)]
#[event_cpi]
pub struct ConfidentialBurn<'info> {
    /// Token owner and burn authority.
    #[account(mut)]
    pub owner: Signer<'info>,
    /// Confidential mint whose encrypted total supply is decreased.
    #[account(mut)]
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Token account whose balance is decreased.
    #[account(mut)]
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    /// CHECK: Mint-scoped app authority for total-supply handles.
    #[account(seeds = [b"total-supply", mint.key().as_ref()], bump)]
    pub total_supply_authority: UncheckedAccount<'info>,
    /// Current balance ACL record used as the left-hand operand.
    pub current_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// Current total-supply ACL record used as the left-hand operand.
    pub current_total_supply_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// Encrypted burn amount ACL record.
    pub amount_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub burn_success_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub debit_candidate_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub output_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub burned_amount_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub total_supply_output_acl: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used for FHE operations.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// System program used for ACL account creation.
    pub system_program: Program<'info, System>,
}

/// Accounts for confidential balance transfer.
#[derive(Accounts)]
#[event_cpi]
pub struct ConfidentialTransfer<'info> {
    /// Sender and transfer authority.
    #[account(mut)]
    pub owner: Signer<'info>,
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
    /// Encrypted amount ACL record.
    pub amount_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub transfer_success_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub debit_candidate_acl: UncheckedAccount<'info>,
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
}

impl<'info> ConfidentialTransfer<'info> {
    pub(crate) fn as_transfer_accounts(&mut self) -> TransferAccounts<'_, 'info> {
        TransferAccounts {
            payer: &self.owner,
            mint: &self.mint,
            from_account: &mut self.from_account,
            to_account: &mut self.to_account,
            compute_signer: &self.compute_signer,
            from_current_compute_acl: self.from_current_compute_acl.as_ref(),
            to_current_compute_acl: self.to_current_compute_acl.as_ref(),
            amount_compute_acl: &self.amount_compute_acl,
            transfer_success_acl: self.transfer_success_acl.to_account_info(),
            debit_candidate_acl: self.debit_candidate_acl.to_account_info(),
            from_output_acl: self.from_output_acl.to_account_info(),
            transferred_amount_acl: self.transferred_amount_acl.to_account_info(),
            to_output_acl: self.to_output_acl.to_account_info(),
            zama_event_authority: &self.zama_event_authority,
            zama_program: &self.zama_program,
            host_config: &self.host_config,
            system_program: &self.system_program,
        }
    }
}

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

/// Accounts for calling a receiver hook after an operator-driven confidential transfer.
#[derive(Accounts)]
#[instruction(sent_handle: [u8; 32])]
pub struct ConfidentialCallTransferReceiverFrom<'info> {
    /// Active operator that initiated or is authorized to continue the split transfer-and-call flow.
    #[account(mut)]
    pub operator: Signer<'info>,
    /// Confidential mint.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Original sender token account controlled by the operator row.
    pub from_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Original recipient token account.
    pub to_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Operator authorization row for `(from_account, operator)`.
    #[account(
        seeds = [b"operator", from_account.key().as_ref(), operator.key().as_ref()],
        bump = operator_record.bump
    )]
    pub operator_record: Account<'info, ConfidentialOperator>,
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
        payer = operator,
        space = 8 + TransferReceiverHookCall::SPACE,
        seeds = [b"transfer-hook", mint.key().as_ref(), sent_handle.as_ref()],
        bump
    )]
    pub hook_record: Account<'info, TransferReceiverHookCall>,
    /// System program used to create the one-shot hook marker.
    pub system_program: Program<'info, System>,
}

/// Accounts for setting or revoking an operator.
#[derive(Accounts)]
#[event_cpi]
pub struct SetOperator<'info> {
    /// Token account owner and rent payer.
    #[account(mut)]
    pub owner: Signer<'info>,
    /// Confidential mint.
    pub mint: Account<'info, ConfidentialMint>,
    /// Token account whose operator row is being changed.
    pub token_account: Account<'info, ConfidentialTokenAccount>,
    /// CHECK: Canonical operator PDA created or overwritten by this instruction.
    #[account(mut)]
    pub operator_record: UncheckedAccount<'info>,
    /// System program used for operator PDA creation.
    pub system_program: Program<'info, System>,
}

/// Accounts for closing an operator row.
#[derive(Accounts)]
#[event_cpi]
#[instruction(operator: Pubkey)]
pub struct CloseOperator<'info> {
    /// Optional token owner. Required when closing an active operator row.
    pub owner: Option<Signer<'info>>,
    /// Confidential mint.
    pub mint: Account<'info, ConfidentialMint>,
    /// Token account controlled by the operator row.
    pub token_account: Account<'info, ConfidentialTokenAccount>,
    /// Operator authorization row to close.
    #[account(
        mut,
        seeds = [b"operator", token_account.key().as_ref(), operator.as_ref()],
        bump = operator_record.bump,
        close = refund_recipient
    )]
    pub operator_record: Account<'info, ConfidentialOperator>,
    /// CHECK: Must be the stored token owner and receives the rent refund.
    #[account(mut)]
    pub refund_recipient: UncheckedAccount<'info>,
}

/// Accounts for confidential operator transfer.
#[derive(Accounts)]
#[event_cpi]
pub struct ConfidentialTransferFrom<'info> {
    /// Active operator and rent payer for output ACL records.
    #[account(mut)]
    pub operator: Signer<'info>,
    /// Confidential mint.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Sender token account controlled by the operator row.
    #[account(mut)]
    pub from_account: Box<Account<'info, ConfidentialTokenAccount>>,
    // Anchor 1 rejects duplicate mutable Account<T> values unless the account opts in.
    // A self-transfer is a supported no-op, so from_account and to_account may be equal.
    #[account(mut, dup)]
    pub to_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Operator authorization row for `(from_account, operator)`.
    #[account(
        seeds = [b"operator", from_account.key().as_ref(), operator.key().as_ref()],
        bump = operator_record.bump
    )]
    pub operator_record: Account<'info, ConfidentialOperator>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    /// Sender current balance ACL record.
    pub from_current_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// Recipient current balance ACL record.
    pub to_current_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// Encrypted amount ACL record.
    pub amount_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub transfer_success_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub debit_candidate_acl: UncheckedAccount<'info>,
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
}

impl<'info> ConfidentialTransferFrom<'info> {
    pub(crate) fn as_transfer_accounts(&mut self) -> TransferAccounts<'_, 'info> {
        TransferAccounts {
            payer: &self.operator,
            mint: &self.mint,
            from_account: &mut self.from_account,
            to_account: &mut self.to_account,
            compute_signer: &self.compute_signer,
            from_current_compute_acl: self.from_current_compute_acl.as_ref(),
            to_current_compute_acl: self.to_current_compute_acl.as_ref(),
            amount_compute_acl: &self.amount_compute_acl,
            transfer_success_acl: self.transfer_success_acl.to_account_info(),
            debit_candidate_acl: self.debit_candidate_acl.to_account_info(),
            from_output_acl: self.from_output_acl.to_account_info(),
            transferred_amount_acl: self.transferred_amount_acl.to_account_info(),
            to_output_acl: self.to_output_acl.to_account_info(),
            zama_event_authority: &self.zama_event_authority,
            zama_program: &self.zama_program,
            host_config: &self.host_config,
            system_program: &self.system_program,
        }
    }
}

/// Accounts for preparing receiver callback settlement and debiting any refund.
#[derive(Accounts)]
#[event_cpi]
#[instruction(sent_handle: [u8; 32])]
pub struct ConfidentialPrepareTransferCallback<'info> {
    /// Rent payer for callback-settlement output accounts.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: receiver-side authority key that produced the encrypted callback result.
    ///
    /// This authority is already enforced by the callback-success ACL record; it
    /// does not sign settlement so a failed callback can be refunded without
    /// recipient cooperation after the hook.
    pub callback_authority: UncheckedAccount<'info>,
    /// Confidential mint.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Original sender token account.
    pub from_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Original recipient token account; pays any best-effort refund in this prepare step.
    #[account(mut)]
    pub to_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    /// Original recipient current balance ACL record.
    pub to_current_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// ACL record for the prior transfer's all-or-zero sent amount.
    pub sent_amount_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// ACL record for the encrypted callback success bit.
    pub callback_success_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// Verified receiver-hook invocation for this sent amount.
    #[account(
        seeds = [b"transfer-hook", mint.key().as_ref(), sent_handle.as_ref()],
        bump = hook_record.bump
    )]
    pub hook_record: Account<'info, TransferReceiverHookCall>,
    /// Replay marker for this callback settlement.
    #[account(
        init,
        payer = payer,
        space = 8 + TransferCallbackSettlement::SPACE,
        seeds = [b"transfer-callback", mint.key().as_ref(), sent_handle.as_ref()],
        bump
    )]
    pub settlement_record: Account<'info, TransferCallbackSettlement>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub callback_zero_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub requested_refund_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub refund_success_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub refund_debit_candidate_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub to_output_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub refund_amount_acl: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used for FHE operations.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// System program used for ACL and replay-marker account creation.
    pub system_program: Program<'info, System>,
}

impl<'info> ConfidentialPrepareTransferCallback<'info> {
    pub(crate) fn as_prepare_callback_accounts(
        &mut self,
    ) -> PrepareTransferCallbackAccounts<'_, 'info> {
        PrepareTransferCallbackAccounts {
            payer: &self.payer,
            callback_authority: &self.callback_authority,
            mint: &self.mint,
            from_account: self.from_account.as_ref(),
            to_account: &mut self.to_account,
            compute_signer: &self.compute_signer,
            to_current_compute_acl: self.to_current_compute_acl.as_ref(),
            sent_amount_acl: &self.sent_amount_acl,
            callback_success_acl: &self.callback_success_acl,
            hook_record: &self.hook_record,
            settlement_record: &mut self.settlement_record,
            callback_zero_acl: self.callback_zero_acl.to_account_info(),
            requested_refund_acl: self.requested_refund_acl.to_account_info(),
            refund_success_acl: self.refund_success_acl.to_account_info(),
            refund_debit_candidate_acl: self.refund_debit_candidate_acl.to_account_info(),
            to_output_acl: self.to_output_acl.to_account_info(),
            refund_amount_acl: self.refund_amount_acl.to_account_info(),
            zama_event_authority: &self.zama_event_authority,
            zama_program: &self.zama_program,
            host_config: &self.host_config,
            system_program: &self.system_program,
        }
    }
}

/// Accounts for finalizing a prepared callback settlement and crediting any refund.
#[derive(Accounts)]
#[event_cpi]
pub struct ConfidentialFinalizeTransferCallback<'info> {
    /// Rent payer for final callback-settlement output accounts.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Confidential mint.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Original sender token account; receives any best-effort refund in this finalize step.
    #[account(mut)]
    pub from_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Original recipient token account; must match the prepared settlement.
    pub to_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    /// Original sender current balance ACL record.
    pub from_current_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// ACL record for the prior transfer's all-or-zero sent amount.
    pub sent_amount_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// Prepared callback settlement.
    #[account(mut)]
    pub settlement_record: Account<'info, TransferCallbackSettlement>,
    /// ACL record for the prepared refund amount.
    pub refund_amount_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub from_output_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub transferred_amount_acl: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used for FHE operations.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// System program used for ACL account creation.
    pub system_program: Program<'info, System>,
}

impl<'info> ConfidentialFinalizeTransferCallback<'info> {
    pub(crate) fn as_finalize_callback_accounts(
        &mut self,
    ) -> FinalizeTransferCallbackAccounts<'_, 'info> {
        FinalizeTransferCallbackAccounts {
            payer: &self.payer,
            mint: &self.mint,
            from_account: &mut self.from_account,
            to_account: self.to_account.as_ref(),
            compute_signer: &self.compute_signer,
            from_current_compute_acl: self.from_current_compute_acl.as_ref(),
            sent_amount_acl: &self.sent_amount_acl,
            settlement_record: &mut self.settlement_record,
            refund_amount_acl: &self.refund_amount_acl,
            from_output_acl: self.from_output_acl.to_account_info(),
            transferred_amount_acl: self.transferred_amount_acl.to_account_info(),
            zama_event_authority: &self.zama_event_authority,
            zama_program: &self.zama_program,
            host_config: &self.host_config,
            system_program: &self.system_program,
        }
    }
}

/// Empty account set for the test receiver hook endpoint.
#[derive(Accounts)]
pub struct TestReceiverReturnCallback {}

/// Accounts for requesting public disclosure of the current balance handle.
#[derive(Accounts)]
#[event_cpi]
pub struct RequestDiscloseBalance<'info> {
    /// Token account owner and disclosure authority.
    pub owner: Signer<'info>,
    /// Confidential mint.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Confidential token account whose current balance is disclosed.
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Current balance ACL record. Updated by ZamaHost CPI.
    #[account(mut)]
    pub balance_acl_record: Box<Account<'info, zama_host::AclRecord>>,
    /// CHECK: optional overflow permission witness for the owner authority.
    pub authority_permission_record: Option<UncheckedAccount<'info>>,
    /// CHECK: optional deny-list witness when host deny-lists are enabled.
    pub deny_subject_record: Option<UncheckedAccount<'info>>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used to update the ACL record.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for pause and deny-list checks.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
}

/// Accounts for requesting public disclosure of a token-scoped encrypted amount.
#[derive(Accounts)]
#[event_cpi]
pub struct RequestDiscloseAmount<'info> {
    /// Requester that must have `ACL_ROLE_PUBLIC_DECRYPT` on the amount ACL.
    pub requester: Signer<'info>,
    /// Confidential mint that scopes the encrypted amount.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Token-scoped amount ACL record. Updated by ZamaHost CPI.
    #[account(mut)]
    pub amount_acl_record: Box<Account<'info, zama_host::AclRecord>>,
    /// CHECK: optional overflow permission witness for the requester authority.
    pub authority_permission_record: Option<UncheckedAccount<'info>>,
    /// CHECK: optional deny-list witness when host deny-lists are enabled.
    pub deny_subject_record: Option<UncheckedAccount<'info>>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used to update the ACL record.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for pause and deny-list checks.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
}

/// Accounts for disclosing a KMS-certified current balance cleartext.
#[derive(Accounts)]
#[event_cpi]
pub struct DiscloseBalance<'info> {
    /// Confidential mint carrying the KMS verifier authority.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Confidential token account whose current balance is disclosed.
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Current balance ACL record for the disclosed handle.
    pub balance_acl_record: Box<Account<'info, zama_host::AclRecord>>,
    /// Material commitment witness for the disclosed handle.
    pub balance_material_commitment: Box<Account<'info, zama_host::HandleMaterialCommitment>>,
    /// CHECK: Solana instructions sysvar; handler verifies its address and previous Ed25519 ix.
    pub instructions_sysvar: UncheckedAccount<'info>,
}

/// Accounts for disclosing a KMS-certified token-scoped amount cleartext.
#[derive(Accounts)]
#[event_cpi]
pub struct DiscloseAmount<'info> {
    /// Confidential mint carrying the KMS verifier authority.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Token-scoped amount ACL record for the disclosed handle.
    pub amount_acl_record: Box<Account<'info, zama_host::AclRecord>>,
    /// Material commitment witness for the disclosed handle.
    pub amount_material_commitment: Box<Account<'info, zama_host::HandleMaterialCommitment>>,
    /// CHECK: Solana instructions sysvar; handler verifies its address and previous Ed25519 ix.
    pub instructions_sysvar: UncheckedAccount<'info>,
}

/// Accounts for redeeming a KMS-certified burned amount from the SPL vault.
#[derive(Accounts)]
#[instruction(burned_handle: [u8; 32], cleartext_amount: u64)]
#[event_cpi]
pub struct RedeemBurnedAmount<'info> {
    /// Token owner and redemption recipient.
    #[account(mut)]
    pub owner: Signer<'info>,
    /// Confidential mint whose vault backs the redeemed burned amount.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Confidential token account that produced the burned amount.
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Underlying SPL mint.
    pub underlying_mint: Box<Account<'info, SplMint>>,
    /// Program vault USDC token account.
    #[account(
        mut,
        constraint = vault_usdc.mint == underlying_mint.key() @ ConfidentialTokenError::UnderlyingMintMismatch,
        constraint = vault_usdc.owner == vault_authority.key() @ ConfidentialTokenError::VaultAuthorityMismatch
    )]
    pub vault_usdc: Box<Account<'info, TokenAccount>>,
    /// Owner's destination USDC token account.
    #[account(
        mut,
        constraint = destination_usdc.mint == underlying_mint.key() @ ConfidentialTokenError::UnderlyingMintMismatch,
        constraint = destination_usdc.owner == owner.key() @ ConfidentialTokenError::OwnerMismatch
    )]
    pub destination_usdc: Box<Account<'info, TokenAccount>>,
    /// CHECK: PDA authority for the underlying-token vault.
    #[account(seeds = [b"vault-authority", mint.key().as_ref()], bump)]
    pub vault_authority: UncheckedAccount<'info>,
    /// Burned amount ACL record whose handle is redeemed.
    pub burned_amount_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// Material commitment witness for the burned handle.
    pub burned_material_commitment: Box<Account<'info, zama_host::HandleMaterialCommitment>>,
    /// Replay marker for this burned handle.
    #[account(
        init,
        payer = owner,
        space = 8 + BurnRedemption::SPACE,
        seeds = [b"burn-redemption", mint.key().as_ref(), burned_handle.as_ref()],
        bump
    )]
    pub redemption_record: Account<'info, BurnRedemption>,
    /// CHECK: Solana instructions sysvar; handler verifies its address and previous Ed25519 ix.
    pub instructions_sysvar: UncheckedAccount<'info>,
    /// SPL token program.
    pub token_program: Program<'info, Token>,
    /// System program used for the replay marker.
    pub system_program: Program<'info, System>,
}
