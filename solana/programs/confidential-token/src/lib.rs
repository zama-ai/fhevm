use anchor_lang::prelude::*;
use anchor_spl::token::{self as spl_token, Mint as SplMint, Token, TokenAccount, TransferChecked};
use zama_host::{
    self, cpi,
    cpi::accounts::{BindAclRecord, EmitProtocolEvent, FheBinaryOp},
    program::ZamaHost,
    AclPermission, AclSubjectEntry, FheBinaryOpCode,
};

declare_id!("5GKzUSfqBSNjoVW83w3xPtTnAe84srZcDTBstpSoBCR4");

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
        balance_handle: [u8; 32],
    ) -> Result<()> {
        let token_account = &mut ctx.accounts.token_account;
        token_account.owner = ctx.accounts.owner.key();
        token_account.mint = ctx.accounts.mint.key();
        token_account.balance_handle = balance_handle;
        token_account.balance_acl_record = Pubkey::default();
        token_account.next_balance_nonce_sequence = 1;
        token_account.bump = ctx.bumps.token_account;
        Ok(())
    }

    pub fn authorize_balance_acl(ctx: Context<AuthorizeBalanceAcl>) -> Result<()> {
        let token_account = &ctx.accounts.token_account;
        require_keys_eq!(
            token_account.owner,
            ctx.accounts.owner.key(),
            ConfidentialTokenError::OwnerMismatch
        );
        require_keys_eq!(
            token_account.mint,
            ctx.accounts.mint.key(),
            ConfidentialTokenError::MintMismatch
        );
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
        let nonce_sequence = current_balance_nonce_sequence(token_account)?;
        let acl_record = ctx.accounts.acl_record.key();
        bind_token_account_acl(
            &ctx.accounts.owner,
            &ctx.accounts.mint,
            &ctx.accounts.compute_signer,
            &ctx.accounts.token_account,
            ctx.accounts.acl_record.to_account_info(),
            &ctx.accounts.zama_event_authority,
            &ctx.accounts.zama_program,
            &ctx.accounts.system_program,
            nonce_sequence,
            token_account.balance_handle,
        )?;
        let token_account = &mut ctx.accounts.token_account;
        token_account.balance_acl_record = acl_record;
        Ok(())
    }

    pub fn wrap_usdc(
        ctx: Context<WrapUsdc>,
        amount: u64,
        amount_handle: [u8; 32],
        new_balance_handle: [u8; 32],
    ) -> Result<()> {
        let mint = &ctx.accounts.mint;
        let token_account = &ctx.accounts.token_account;
        let nonce_sequence = token_account.next_balance_nonce_sequence;
        let compute_signer = mint.compute_signer;

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

        spl_token::transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
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
        emit_trivial_encrypt(
            &ctx.accounts.zama_event_authority,
            &ctx.accounts.zama_program,
            compute_signer,
            amount_to_plaintext(amount),
            5,
            amount_handle,
        )?;
        emit_binary_op(
            &ctx.accounts.zama_event_authority,
            &ctx.accounts.zama_program,
            &ctx.accounts.compute_signer,
            ctx.accounts.current_compute_acl.to_account_info(),
            FheBinaryOpCode::Add,
            token_account.balance_handle,
            ctx.accounts.amount_compute_acl.to_account_info(),
            amount_handle,
            new_balance_handle,
            mint.key(),
            ctx.bumps.compute_signer,
        )?;
        bind_token_account_acl(
            &ctx.accounts.owner,
            mint,
            &ctx.accounts.compute_signer,
            &ctx.accounts.token_account,
            ctx.accounts.output_acl.to_account_info(),
            &ctx.accounts.zama_event_authority,
            &ctx.accounts.zama_program,
            &ctx.accounts.system_program,
            nonce_sequence,
            new_balance_handle,
        )?;

        let token_account = &mut ctx.accounts.token_account;
        token_account.balance_handle = new_balance_handle;
        token_account.balance_acl_record = ctx.accounts.output_acl.key();
        token_account.next_balance_nonce_sequence = nonce_sequence
            .checked_add(1)
            .ok_or(ConfidentialTokenError::AclNonceOverflow)?;
        Ok(())
    }

    pub fn confidential_transfer(
        ctx: Context<ConfidentialTransfer>,
        amount_handle: [u8; 32],
        new_from_handle: [u8; 32],
        new_to_handle: [u8; 32],
    ) -> Result<()> {
        let mint = &ctx.accounts.mint;
        let from = &ctx.accounts.from_account;
        let to = &ctx.accounts.to_account;
        let from_nonce_sequence = from.next_balance_nonce_sequence;
        let to_nonce_sequence = to.next_balance_nonce_sequence;
        let compute_signer = mint.compute_signer;

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

        emit_binary_op(
            &ctx.accounts.zama_event_authority,
            &ctx.accounts.zama_program,
            &ctx.accounts.compute_signer,
            ctx.accounts.from_current_compute_acl.to_account_info(),
            FheBinaryOpCode::Sub,
            from.balance_handle,
            ctx.accounts.amount_compute_acl.to_account_info(),
            amount_handle,
            new_from_handle,
            mint.key(),
            ctx.bumps.compute_signer,
        )?;
        emit_binary_op(
            &ctx.accounts.zama_event_authority,
            &ctx.accounts.zama_program,
            &ctx.accounts.compute_signer,
            ctx.accounts.to_current_compute_acl.to_account_info(),
            FheBinaryOpCode::Add,
            to.balance_handle,
            ctx.accounts.amount_compute_acl.to_account_info(),
            amount_handle,
            new_to_handle,
            mint.key(),
            ctx.bumps.compute_signer,
        )?;

        bind_token_account_acl(
            &ctx.accounts.owner,
            mint,
            &ctx.accounts.compute_signer,
            &ctx.accounts.from_account,
            ctx.accounts.from_output_acl.to_account_info(),
            &ctx.accounts.zama_event_authority,
            &ctx.accounts.zama_program,
            &ctx.accounts.system_program,
            from_nonce_sequence,
            new_from_handle,
        )?;
        bind_token_account_acl(
            &ctx.accounts.owner,
            mint,
            &ctx.accounts.compute_signer,
            &ctx.accounts.to_account,
            ctx.accounts.to_output_acl.to_account_info(),
            &ctx.accounts.zama_event_authority,
            &ctx.accounts.zama_program,
            &ctx.accounts.system_program,
            to_nonce_sequence,
            new_to_handle,
        )?;

        let from = &mut ctx.accounts.from_account;
        from.balance_handle = new_from_handle;
        from.balance_acl_record = ctx.accounts.from_output_acl.key();
        from.next_balance_nonce_sequence = from_nonce_sequence
            .checked_add(1)
            .ok_or(ConfidentialTokenError::AclNonceOverflow)?;

        let to = &mut ctx.accounts.to_account;
        to.balance_handle = new_to_handle;
        to.balance_acl_record = ctx.accounts.to_output_acl.key();
        to.next_balance_nonce_sequence = to_nonce_sequence
            .checked_add(1)
            .ok_or(ConfidentialTokenError::AclNonceOverflow)?;
        Ok(())
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
pub struct InitializeTokenAccount<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    pub mint: Account<'info, ConfidentialMint>,
    #[account(
        init,
        payer = owner,
        space = 8 + ConfidentialTokenAccount::SPACE,
        seeds = [b"token-account", mint.key().as_ref(), owner.key().as_ref()],
        bump
    )]
    pub token_account: Account<'info, ConfidentialTokenAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AuthorizeBalanceAcl<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    pub mint: Account<'info, ConfidentialMint>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    #[account(mut)]
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
pub struct WrapUsdc<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    pub mint: Account<'info, ConfidentialMint>,
    #[account(mut)]
    pub token_account: Account<'info, ConfidentialTokenAccount>,
    pub underlying_mint: Account<'info, SplMint>,
    #[account(
        mut,
        constraint = user_usdc.mint == underlying_mint.key() @ ConfidentialTokenError::UnderlyingMintMismatch,
        constraint = user_usdc.owner == owner.key() @ ConfidentialTokenError::OwnerMismatch
    )]
    pub user_usdc: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = vault_usdc.mint == underlying_mint.key() @ ConfidentialTokenError::UnderlyingMintMismatch,
        constraint = vault_usdc.owner == vault_authority.key() @ ConfidentialTokenError::VaultAuthorityMismatch
    )]
    pub vault_usdc: Account<'info, TokenAccount>,
    /// CHECK: PDA authority for the underlying-token vault.
    #[account(seeds = [b"vault-authority", mint.key().as_ref()], bump)]
    pub vault_authority: UncheckedAccount<'info>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    pub current_compute_acl: Account<'info, zama_host::AclRecord>,
    pub amount_compute_acl: Account<'info, zama_host::AclRecord>,
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
pub struct ConfidentialTransfer<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    pub mint: Account<'info, ConfidentialMint>,
    #[account(mut)]
    pub from_account: Account<'info, ConfidentialTokenAccount>,
    #[account(mut)]
    pub to_account: Account<'info, ConfidentialTokenAccount>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    pub from_current_compute_acl: Account<'info, zama_host::AclRecord>,
    pub to_current_compute_acl: Account<'info, zama_host::AclRecord>,
    pub amount_compute_acl: Account<'info, zama_host::AclRecord>,
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
}

fn current_balance_nonce_sequence(token_account: &ConfidentialTokenAccount) -> Result<u64> {
    token_account
        .next_balance_nonce_sequence
        .checked_sub(1)
        .ok_or(ConfidentialTokenError::AclNonceOverflow.into())
}

fn emit_binary_op<'info>(
    zama_event_authority: &UncheckedAccount<'info>,
    zama_program: &Program<'info, ZamaHost>,
    compute_signer: &UncheckedAccount<'info>,
    lhs_acl_record: AccountInfo<'info>,
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs_acl_record: AccountInfo<'info>,
    rhs: [u8; 32],
    result: [u8; 32],
    mint: Pubkey,
    compute_signer_bump: u8,
) -> Result<()> {
    let bump = [compute_signer_bump];
    let signer_seeds: &[&[&[u8]]] = &[&[b"fhe-compute", mint.as_ref(), &bump]];
    cpi::fhe_binary_op(
        CpiContext::new_with_signer(
            zama_program.to_account_info(),
            FheBinaryOp {
                compute_subject: compute_signer.to_account_info(),
                lhs_acl_record,
                rhs_acl_record,
                event_authority: zama_event_authority.to_account_info(),
                program: zama_program.to_account_info(),
            },
            signer_seeds,
        ),
        op,
        lhs,
        rhs,
        false,
        result,
    )
}

fn emit_trivial_encrypt<'info>(
    zama_event_authority: &UncheckedAccount<'info>,
    zama_program: &Program<'info, ZamaHost>,
    subject: Pubkey,
    plaintext: [u8; 32],
    fhe_type: u8,
    result: [u8; 32],
) -> Result<()> {
    cpi::trivial_encrypt(
        CpiContext::new(
            zama_program.to_account_info(),
            EmitProtocolEvent {
                event_authority: zama_event_authority.to_account_info(),
                program: zama_program.to_account_info(),
            },
        ),
        subject,
        plaintext,
        fhe_type,
        result,
    )
}

fn amount_to_plaintext(amount: u64) -> [u8; 32] {
    let mut plaintext = [0_u8; 32];
    plaintext[24..].copy_from_slice(&amount.to_be_bytes());
    plaintext
}

fn bind_token_account_acl<'info>(
    payer: &Signer<'info>,
    mint: &Account<'info, ConfidentialMint>,
    compute_signer: &UncheckedAccount<'info>,
    token_account: &Account<'info, ConfidentialTokenAccount>,
    acl_record: AccountInfo<'info>,
    zama_event_authority: &UncheckedAccount<'info>,
    zama_program: &Program<'info, ZamaHost>,
    system_program: &Program<'info, System>,
    nonce_sequence: u64,
    handle: [u8; 32],
) -> Result<()> {
    let token_account_key = token_account.key();
    let nonce_key = balance_nonce_key(mint.key(), token_account_key);
    let bump = [token_account.bump];
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"token-account",
        token_account.mint.as_ref(),
        token_account.owner.as_ref(),
        &bump,
    ]];
    cpi::bind_acl_record(
        CpiContext::new_with_signer(
            zama_program.to_account_info(),
            BindAclRecord {
                payer: payer.to_account_info(),
                app_account_authority: token_account.to_account_info(),
                acl_record,
                system_program: system_program.to_account_info(),
                event_authority: zama_event_authority.to_account_info(),
                program: zama_program.to_account_info(),
            },
            signer_seeds,
        ),
        nonce_key,
        nonce_sequence,
        mint.key(),
        token_account_key,
        balance_label(),
        handle,
        vec![
            AclSubjectEntry {
                pubkey: token_account.owner,
                permission: AclPermission::UserDecrypt,
            },
            AclSubjectEntry {
                pubkey: compute_signer.key(),
                permission: AclPermission::Compute,
            },
        ],
        false,
    )
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

pub fn nonce_key(
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
) -> [u8; 32] {
    zama_host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label)
}
