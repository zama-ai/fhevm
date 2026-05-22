// Anchor macros generate framework-shaped code that trips rustc/Clippy checks.
#![allow(unexpected_cfgs)]
#![allow(clippy::diverging_sub_expression, clippy::too_many_arguments)]

mod fhe;

use anchor_lang::prelude::*;
use anchor_spl::token::{self as spl_token, Mint as SplMint, Token, TokenAccount, TransferChecked};
use zama_host::{self, program::ZamaHost, AclSubjectEntry};

declare_id!("5GKzUSfqBSNjoVW83w3xPtTnAe84srZcDTBstpSoBCR4");

const BALANCE_FHE_TYPE: u8 = 5;
const APP_EVENT_VERSION: u8 = 0;

#[program]
pub mod confidential_token {
    use super::*;

    pub fn initialize_mint(ctx: Context<InitializeMint>) -> Result<()> {
        let mint_key = ctx.accounts.mint.key();
        let mint = &mut ctx.accounts.mint;
        mint.authority = ctx.accounts.authority.key();
        mint.acl_domain_key = mint_key;
        mint.compute_signer = compute_signer_address(mint_key).0;
        mint.underlying_mint = ctx.accounts.underlying_mint.key();
        mint.decimals = ctx.accounts.underlying_mint.decimals;
        Ok(())
    }

    pub fn initialize_token_account(
        ctx: Context<InitializeTokenAccount>,
        initial_balance: u64,
    ) -> Result<()> {
        let token_account = &mut ctx.accounts.token_account;
        token_account.owner = ctx.accounts.owner.key();
        token_account.mint = ctx.accounts.mint.key();
        token_account.balance_handle = [0; 32];
        token_account.balance_acl_record = Pubkey::default();
        token_account.next_balance_nonce_sequence = 1;
        token_account.bump = ctx.bumps.token_account;
        require_keys_eq!(
            ctx.accounts.mint.acl_domain_key,
            ctx.accounts.mint.key(),
            ConfidentialTokenError::AclDomainKeyMismatch
        );
        require_keys_eq!(
            ctx.accounts.compute_signer.key(),
            ctx.accounts.mint.compute_signer,
            ConfidentialTokenError::ComputeSignerMismatch
        );
        let acl_record = ctx.accounts.acl_record.key();
        let balance_handle = trivial_encrypt_balance_acl(
            &ctx.accounts.owner,
            &ctx.accounts.mint,
            &ctx.accounts.compute_signer,
            &ctx.accounts.token_account,
            ctx.accounts.acl_record.to_account_info(),
            &ctx.accounts.zama_event_authority,
            &ctx.accounts.zama_program,
            &ctx.accounts.system_program,
            ctx.bumps.compute_signer,
            0,
            initial_balance,
        )?;
        let token_account = &mut ctx.accounts.token_account;
        token_account.balance_handle = balance_handle;
        token_account.balance_acl_record = acl_record;
        emit_cpi!(BalanceHandleUpdatedEvent {
            version: APP_EVENT_VERSION,
            mint: ctx.accounts.mint.key(),
            owner: ctx.accounts.owner.key(),
            token_account: token_account.key(),
            old_handle: [0; 32],
            old_acl_record: Pubkey::default(),
            new_handle: balance_handle,
            new_acl_record: acl_record,
            reason: BalanceHandleUpdateReason::Initialize,
        });
        Ok(())
    }

    pub fn wrap_usdc(ctx: Context<WrapUsdc>, amount: u64) -> Result<()> {
        let mint = &ctx.accounts.mint;
        let token_account = &ctx.accounts.token_account;
        let nonce_sequence = token_account.next_balance_nonce_sequence;
        let compute_signer = mint.compute_signer;
        let old_balance_handle = token_account.balance_handle;
        let old_balance_acl_record = token_account.balance_acl_record;

        require_keys_eq!(
            token_account.owner,
            ctx.accounts.owner.key(),
            ConfidentialTokenError::OwnerMismatch
        );
        require_keys_eq!(
            token_account.mint,
            mint.key(),
            ConfidentialTokenError::MintMismatch
        );
        require_keys_eq!(
            mint.underlying_mint,
            ctx.accounts.underlying_mint.key(),
            ConfidentialTokenError::UnderlyingMintMismatch
        );
        require_keys_eq!(
            ctx.accounts.compute_signer.key(),
            compute_signer,
            ConfidentialTokenError::ComputeSignerMismatch
        );
        require_keys_eq!(
            ctx.accounts.current_compute_acl.key(),
            token_account.balance_acl_record,
            ConfidentialTokenError::CurrentAclRecordMismatch
        );

        spl_token::transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.key(),
                TransferChecked {
                    from: ctx.accounts.user_usdc.to_account_info(),
                    mint: ctx.accounts.underlying_mint.to_account_info(),
                    to: ctx.accounts.vault_usdc.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
            ),
            amount,
            mint.decimals,
        )?;

        let new_balance_handle = fhe::execute(
            fhe_context(
                &ctx.accounts.owner,
                &ctx.accounts.zama_event_authority,
                &ctx.accounts.zama_program,
                &ctx.accounts.compute_signer,
                token_account,
                mint.key(),
                ctx.bumps.compute_signer,
                &ctx.accounts.system_program,
            ),
            |fhe| {
                let current_balance = fhe.encrypted(fhe::EncryptedValue {
                    handle: token_account.balance_handle,
                    acl_record: ctx.accounts.current_compute_acl.to_account_info(),
                })?;
                let amount = fhe.trivial_encrypt_u64(amount, BALANCE_FHE_TYPE)?;
                let new_balance = fhe.add(current_balance, amount, BALANCE_FHE_TYPE)?;
                fhe.allow(
                    &new_balance,
                    fhe::DurableAllow {
                        acl_record: ctx.accounts.output_acl.to_account_info(),
                        app_account: token_account.key(),
                        nonce_key: balance_nonce_key(mint.key(), token_account.key()),
                        nonce_sequence,
                        encrypted_value_label: balance_label(),
                        subjects: balance_acl_subjects(token_account.owner, compute_signer),
                        public_decrypt: false,
                    },
                )?;
                Ok(new_balance.handle())
            },
        )?;

        let token_account = &mut ctx.accounts.token_account;
        token_account.balance_handle = new_balance_handle;
        token_account.balance_acl_record = ctx.accounts.output_acl.key();
        token_account.next_balance_nonce_sequence = nonce_sequence
            .checked_add(1)
            .ok_or(ConfidentialTokenError::AclNonceOverflow)?;
        emit_cpi!(BalanceHandleUpdatedEvent {
            version: APP_EVENT_VERSION,
            mint: mint.key(),
            owner: token_account.owner,
            token_account: token_account.key(),
            old_handle: old_balance_handle,
            old_acl_record: old_balance_acl_record,
            new_handle: new_balance_handle,
            new_acl_record: ctx.accounts.output_acl.key(),
            reason: BalanceHandleUpdateReason::Wrap,
        });
        Ok(())
    }

    pub fn confidential_transfer(
        ctx: Context<ConfidentialTransfer>,
        amount_handle: [u8; 32],
    ) -> Result<()> {
        let mint = &ctx.accounts.mint;
        let from = &ctx.accounts.from_account;
        let to = &ctx.accounts.to_account;
        let from_nonce_sequence = from.next_balance_nonce_sequence;
        let to_nonce_sequence = to.next_balance_nonce_sequence;
        let compute_signer = mint.compute_signer;
        let old_from_handle = from.balance_handle;
        let old_from_acl_record = from.balance_acl_record;
        let old_to_handle = to.balance_handle;
        let old_to_acl_record = to.balance_acl_record;

        require_keys_eq!(
            from.owner,
            ctx.accounts.owner.key(),
            ConfidentialTokenError::OwnerMismatch
        );
        require_keys_eq!(from.mint, mint.key(), ConfidentialTokenError::MintMismatch);
        require_keys_eq!(to.mint, mint.key(), ConfidentialTokenError::MintMismatch);
        require_keys_eq!(
            ctx.accounts.compute_signer.key(),
            compute_signer,
            ConfidentialTokenError::ComputeSignerMismatch
        );
        require_keys_eq!(
            ctx.accounts.from_current_compute_acl.key(),
            from.balance_acl_record,
            ConfidentialTokenError::CurrentAclRecordMismatch
        );
        require_keys_eq!(
            ctx.accounts.to_current_compute_acl.key(),
            to.balance_acl_record,
            ConfidentialTokenError::CurrentAclRecordMismatch
        );
        if from.owner == to.owner && from.mint == to.mint {
            return Ok(());
        }

        let (new_from_handle, new_to_handle) = fhe::execute(
            fhe_context(
                &ctx.accounts.owner,
                &ctx.accounts.zama_event_authority,
                &ctx.accounts.zama_program,
                &ctx.accounts.compute_signer,
                &ctx.accounts.from_account,
                mint.key(),
                ctx.bumps.compute_signer,
                &ctx.accounts.system_program,
            ),
            |fhe| {
                let from_balance = fhe.encrypted(fhe::EncryptedValue {
                    handle: from.balance_handle,
                    acl_record: ctx.accounts.from_current_compute_acl.to_account_info(),
                })?;
                let to_balance = fhe.encrypted(fhe::EncryptedValue {
                    handle: to.balance_handle,
                    acl_record: ctx.accounts.to_current_compute_acl.to_account_info(),
                })?;
                let amount = fhe.encrypted(fhe::EncryptedValue {
                    handle: amount_handle,
                    acl_record: ctx.accounts.amount_compute_acl.to_account_info(),
                })?;
                let new_from = fhe.sub(from_balance, amount.clone(), BALANCE_FHE_TYPE)?;
                fhe.allow(
                    &new_from,
                    fhe::DurableAllow {
                        acl_record: ctx.accounts.from_output_acl.to_account_info(),
                        app_account: from.key(),
                        nonce_key: balance_nonce_key(mint.key(), from.key()),
                        nonce_sequence: from_nonce_sequence,
                        encrypted_value_label: balance_label(),
                        subjects: balance_acl_subjects(from.owner, compute_signer),
                        public_decrypt: false,
                    },
                )?;
                let new_to = fhe.add(to_balance, amount, BALANCE_FHE_TYPE)?;
                fhe.allow(
                    &new_to,
                    fhe::DurableAllow {
                        acl_record: ctx.accounts.to_output_acl.to_account_info(),
                        app_account: to.key(),
                        nonce_key: balance_nonce_key(mint.key(), to.key()),
                        nonce_sequence: to_nonce_sequence,
                        encrypted_value_label: balance_label(),
                        subjects: balance_acl_subjects(to.owner, compute_signer),
                        public_decrypt: false,
                    },
                )?;
                Ok((new_from.handle(), new_to.handle()))
            },
        )?;

        let from = &mut ctx.accounts.from_account;
        from.balance_handle = new_from_handle;
        from.balance_acl_record = ctx.accounts.from_output_acl.key();
        from.next_balance_nonce_sequence = from_nonce_sequence
            .checked_add(1)
            .ok_or(ConfidentialTokenError::AclNonceOverflow)?;
        emit_cpi!(BalanceHandleUpdatedEvent {
            version: APP_EVENT_VERSION,
            mint: mint.key(),
            owner: from.owner,
            token_account: from.key(),
            old_handle: old_from_handle,
            old_acl_record: old_from_acl_record,
            new_handle: new_from_handle,
            new_acl_record: ctx.accounts.from_output_acl.key(),
            reason: BalanceHandleUpdateReason::TransferDebit,
        });

        let to = &mut ctx.accounts.to_account;
        to.balance_handle = new_to_handle;
        to.balance_acl_record = ctx.accounts.to_output_acl.key();
        to.next_balance_nonce_sequence = to_nonce_sequence
            .checked_add(1)
            .ok_or(ConfidentialTokenError::AclNonceOverflow)?;
        emit_cpi!(BalanceHandleUpdatedEvent {
            version: APP_EVENT_VERSION,
            mint: mint.key(),
            owner: to.owner,
            token_account: to.key(),
            old_handle: old_to_handle,
            old_acl_record: old_to_acl_record,
            new_handle: new_to_handle,
            new_acl_record: ctx.accounts.to_output_acl.key(),
            reason: BalanceHandleUpdateReason::TransferCredit,
        });
        Ok(())
    }

    /// PoC-only stand-in for the future external input path.
    ///
    /// Trivial-encrypts a transfer amount and creates durable compute ACL state
    /// through the same `fhe::execute` wrapper used by wrap and transfer.
    pub fn poc_authorize_transfer_amount(
        ctx: Context<PocAuthorizeTransferAmount>,
        amount: u64,
        nonce_sequence: u64,
    ) -> Result<()> {
        let mint = &ctx.accounts.mint;
        let token_account = &ctx.accounts.token_account;
        require_keys_eq!(
            token_account.owner,
            ctx.accounts.owner.key(),
            ConfidentialTokenError::OwnerMismatch
        );
        require_keys_eq!(
            token_account.mint,
            mint.key(),
            ConfidentialTokenError::MintMismatch
        );
        require_keys_eq!(
            ctx.accounts.compute_signer.key(),
            mint.compute_signer,
            ConfidentialTokenError::ComputeSignerMismatch
        );

        fhe::execute(
            fhe_context(
                &ctx.accounts.owner,
                &ctx.accounts.zama_event_authority,
                &ctx.accounts.zama_program,
                &ctx.accounts.compute_signer,
                token_account,
                mint.key(),
                ctx.bumps.compute_signer,
                &ctx.accounts.system_program,
            ),
            |fhe| {
                let amount = fhe.trivial_encrypt_u64(amount, BALANCE_FHE_TYPE)?;
                fhe.allow(
                    &amount,
                    fhe::DurableAllow {
                        acl_record: ctx.accounts.output_acl.to_account_info(),
                        app_account: token_account.key(),
                        nonce_key: transfer_amount_nonce_key(mint.key(), token_account.key()),
                        nonce_sequence,
                        encrypted_value_label: transfer_amount_label(),
                        subjects: vec![AclSubjectEntry {
                            pubkey: mint.compute_signer,
                        }],
                        public_decrypt: false,
                    },
                )?;
                Ok(())
            },
        )
    }
}

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer = authority, space = 8 + ConfidentialMint::SPACE)]
    pub mint: Account<'info, ConfidentialMint>,
    pub underlying_mint: Account<'info, SplMint>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[event_cpi]
pub struct InitializeTokenAccount<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
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
    pub zama_program: Program<'info, ZamaHost>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[event_cpi]
pub struct WrapUsdc<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    pub mint: Box<Account<'info, ConfidentialMint>>,
    #[account(mut)]
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    pub underlying_mint: Box<Account<'info, SplMint>>,
    #[account(
        mut,
        constraint = user_usdc.mint == underlying_mint.key() @ ConfidentialTokenError::UnderlyingMintMismatch,
        constraint = user_usdc.owner == owner.key() @ ConfidentialTokenError::OwnerMismatch
    )]
    pub user_usdc: Box<Account<'info, TokenAccount>>,
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
    pub current_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub output_acl: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    pub zama_program: Program<'info, ZamaHost>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[event_cpi]
pub struct ConfidentialTransfer<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    pub mint: Box<Account<'info, ConfidentialMint>>,
    #[account(mut)]
    pub from_account: Box<Account<'info, ConfidentialTokenAccount>>,
    // Anchor 1 rejects duplicate mutable Account<T> values unless the account opts in.
    // A self-transfer is a supported no-op, so from_account and to_account may be equal.
    #[account(mut, dup)]
    pub to_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    pub from_current_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    pub to_current_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    pub amount_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub from_output_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub to_output_acl: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    pub zama_program: Program<'info, ZamaHost>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[event_cpi]
pub struct PocAuthorizeTransferAmount<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    pub mint: Box<Account<'info, ConfidentialMint>>,
    #[account(
        constraint = token_account.owner == owner.key() @ ConfidentialTokenError::OwnerMismatch,
        constraint = token_account.mint == mint.key() @ ConfidentialTokenError::MintMismatch,
    )]
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub output_acl: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    pub zama_program: Program<'info, ZamaHost>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ConfidentialMint {
    pub authority: Pubkey,
    pub acl_domain_key: Pubkey,
    pub compute_signer: Pubkey,
    pub underlying_mint: Pubkey,
    pub decimals: u8,
}

impl ConfidentialMint {
    pub const SPACE: usize = 32 + 32 + 32 + 32 + 1;
}

#[account]
pub struct ConfidentialTokenAccount {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub balance_handle: [u8; 32],
    pub balance_acl_record: Pubkey,
    pub next_balance_nonce_sequence: u64,
    pub bump: u8,
}

#[event]
pub struct BalanceHandleUpdatedEvent {
    pub version: u8,
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub token_account: Pubkey,
    pub old_handle: [u8; 32],
    pub old_acl_record: Pubkey,
    pub new_handle: [u8; 32],
    pub new_acl_record: Pubkey,
    pub reason: BalanceHandleUpdateReason,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum BalanceHandleUpdateReason {
    Initialize,
    Wrap,
    TransferDebit,
    TransferCredit,
}

impl ConfidentialTokenAccount {
    pub const SPACE: usize = 32 + 32 + 32 + 32 + 8 + 1;
}

#[error_code]
pub enum ConfidentialTokenError {
    #[msg("Token owner does not match signer")]
    OwnerMismatch,
    #[msg("Token account mint does not match")]
    MintMismatch,
    #[msg("ACL nonce overflow")]
    AclNonceOverflow,
    #[msg("Underlying mint does not match confidential mint")]
    UnderlyingMintMismatch,
    #[msg("Vault token account authority does not match vault authority PDA")]
    VaultAuthorityMismatch,
    #[msg("Confidential mint ACL domain key is invalid")]
    AclDomainKeyMismatch,
    #[msg("Compute signer does not match confidential mint")]
    ComputeSignerMismatch,
    #[msg("current ACL record does not match token account state")]
    CurrentAclRecordMismatch,
}

fn fhe_context<'a, 'info>(
    payer: &'a Signer<'info>,
    zama_event_authority: &'a UncheckedAccount<'info>,
    zama_program: &'a Program<'info, ZamaHost>,
    compute_signer: &'a UncheckedAccount<'info>,
    token_account: &'a Account<'info, ConfidentialTokenAccount>,
    mint: Pubkey,
    compute_signer_bump: u8,
    system_program: &'a Program<'info, System>,
) -> fhe::Context<'a, 'info> {
    fhe::Context {
        payer,
        event_authority: zama_event_authority,
        zama_program,
        compute_signer,
        app_account_authority: token_account,
        acl_domain_key: mint,
        compute_signer_bump,
        system_program,
    }
}

fn trivial_encrypt_balance_acl<'info>(
    payer: &Signer<'info>,
    mint: &Account<'info, ConfidentialMint>,
    compute_signer: &UncheckedAccount<'info>,
    token_account: &Account<'info, ConfidentialTokenAccount>,
    acl_record: AccountInfo<'info>,
    zama_event_authority: &UncheckedAccount<'info>,
    zama_program: &Program<'info, ZamaHost>,
    system_program: &Program<'info, System>,
    compute_signer_bump: u8,
    nonce_sequence: u64,
    plaintext: u64,
) -> Result<[u8; 32]> {
    fhe::execute(
        fhe_context(
            payer,
            zama_event_authority,
            zama_program,
            compute_signer,
            token_account,
            mint.key(),
            compute_signer_bump,
            system_program,
        ),
        |fhe| {
            let balance = fhe.trivial_encrypt_u64(plaintext, BALANCE_FHE_TYPE)?;
            fhe.allow(
                &balance,
                fhe::DurableAllow {
                    acl_record,
                    app_account: token_account.key(),
                    nonce_key: balance_nonce_key(mint.key(), token_account.key()),
                    nonce_sequence,
                    encrypted_value_label: balance_label(),
                    subjects: balance_acl_subjects(token_account.owner, compute_signer.key()),
                    public_decrypt: false,
                },
            )?;
            Ok(balance.handle())
        },
    )
}

fn balance_acl_subjects(owner: Pubkey, compute_signer: Pubkey) -> Vec<AclSubjectEntry> {
    vec![
        AclSubjectEntry { pubkey: owner },
        AclSubjectEntry {
            pubkey: compute_signer,
        },
    ]
}

pub fn compute_signer_address(mint: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"fhe-compute", mint.as_ref()], &crate::ID)
}

pub fn balance_nonce_key(acl_domain_key: Pubkey, app_account: Pubkey) -> [u8; 32] {
    nonce_key(acl_domain_key, app_account, balance_label())
}

pub fn balance_label() -> [u8; 32] {
    *b"balance_________________________"
}

pub fn transfer_amount_label() -> [u8; 32] {
    *b"input___________________________"
}

pub fn transfer_amount_nonce_key(acl_domain_key: Pubkey, app_account: Pubkey) -> [u8; 32] {
    nonce_key(acl_domain_key, app_account, transfer_amount_label())
}

pub fn wrap_amount_label() -> [u8; 32] {
    *b"wrap_amount_____________________"
}

pub fn nonce_key(
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
) -> [u8; 32] {
    zama_host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label)
}
