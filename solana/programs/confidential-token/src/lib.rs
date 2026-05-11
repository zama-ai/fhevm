use anchor_lang::prelude::*;
use anchor_spl::token::{self as spl_token, Mint as SplMint, Token, TokenAccount, TransferChecked};
use zama_host::{
    self, cpi,
    cpi::accounts::{AssertAclRecord, BindAclRecord, EmitProtocolEvent},
    program::ZamaHost,
    AclPermission, FheBinaryOpCode,
};

declare_id!("5GKzUSfqBSNjoVW83w3xPtTnAe84srZcDTBstpSoBCR4");

#[program]
pub mod confidential_token {
    use super::*;

    pub fn initialize_mint(ctx: Context<InitializeMint>, fhe_authority: Pubkey) -> Result<()> {
        let mint = &mut ctx.accounts.mint;
        mint.authority = ctx.accounts.authority.key();
        mint.fhe_authority = fhe_authority;
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
        token_account.next_acl_nonce = 1;
        token_account.bump = ctx.bumps.token_account;
        Ok(())
    }

    pub fn authorize_balance_acl(
        ctx: Context<AuthorizeBalanceAcl>,
        subject: Pubkey,
        permission: AclPermission,
    ) -> Result<()> {
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
        let acl_nonce = current_acl_nonce(token_account)?;
        bind_token_account_acl(
            &ctx.accounts.owner,
            &ctx.accounts.token_account,
            ctx.accounts.acl_record.to_account_info(),
            &ctx.accounts.zama_event_authority,
            &ctx.accounts.zama_program,
            &ctx.accounts.system_program,
            acl_nonce,
            token_account.balance_handle,
            subject,
            permission,
        )
    }

    pub fn wrap_usdc(
        ctx: Context<WrapUsdc>,
        amount: u64,
        amount_handle: [u8; 32],
        new_balance_handle: [u8; 32],
    ) -> Result<()> {
        let mint = &ctx.accounts.mint;
        let token_account_key = ctx.accounts.token_account.key();
        let token_account = &ctx.accounts.token_account;
        let nonce = token_account.next_acl_nonce;
        let fhe_authority = mint.fhe_authority;

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
            fhe_authority,
            amount_to_plaintext(amount),
            5,
            amount_handle,
        )?;
        assert_compute_acl(
            &ctx.accounts.zama_program,
            ctx.accounts.current_compute_acl.to_account_info(),
            token_account_key,
            token_account.balance_handle,
            fhe_authority,
        )?;
        emit_binary_op(
            &ctx.accounts.zama_event_authority,
            &ctx.accounts.zama_program,
            FheBinaryOpCode::Add,
            fhe_authority,
            token_account.balance_handle,
            amount_handle,
            new_balance_handle,
        )?;
        bind_token_account_acl(
            &ctx.accounts.owner,
            &ctx.accounts.token_account,
            ctx.accounts.owner_output_acl.to_account_info(),
            &ctx.accounts.zama_event_authority,
            &ctx.accounts.zama_program,
            &ctx.accounts.system_program,
            nonce,
            new_balance_handle,
            token_account.owner,
            AclPermission::UserDecrypt,
        )?;
        bind_token_account_acl(
            &ctx.accounts.owner,
            &ctx.accounts.token_account,
            ctx.accounts.compute_output_acl.to_account_info(),
            &ctx.accounts.zama_event_authority,
            &ctx.accounts.zama_program,
            &ctx.accounts.system_program,
            nonce,
            new_balance_handle,
            fhe_authority,
            AclPermission::Compute,
        )?;

        let token_account = &mut ctx.accounts.token_account;
        token_account.balance_handle = new_balance_handle;
        token_account.next_acl_nonce = nonce
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
        let from_key = ctx.accounts.from_account.key();
        let to_key = ctx.accounts.to_account.key();
        let from = &ctx.accounts.from_account;
        let to = &ctx.accounts.to_account;
        let from_nonce = from.next_acl_nonce;
        let to_nonce = to.next_acl_nonce;
        let fhe_authority = mint.fhe_authority;

        require_keys_eq!(
            from.owner,
            ctx.accounts.owner.key(),
            ConfidentialTokenError::OwnerMismatch
        );
        require_keys_eq!(from.mint, mint.key(), ConfidentialTokenError::MintMismatch);
        require_keys_eq!(to.mint, mint.key(), ConfidentialTokenError::MintMismatch);

        assert_compute_acl(
            &ctx.accounts.zama_program,
            ctx.accounts.from_current_compute_acl.to_account_info(),
            from_key,
            from.balance_handle,
            fhe_authority,
        )?;
        assert_compute_acl(
            &ctx.accounts.zama_program,
            ctx.accounts.to_current_compute_acl.to_account_info(),
            to_key,
            to.balance_handle,
            fhe_authority,
        )?;

        emit_binary_op(
            &ctx.accounts.zama_event_authority,
            &ctx.accounts.zama_program,
            FheBinaryOpCode::Sub,
            fhe_authority,
            from.balance_handle,
            amount_handle,
            new_from_handle,
        )?;
        emit_binary_op(
            &ctx.accounts.zama_event_authority,
            &ctx.accounts.zama_program,
            FheBinaryOpCode::Add,
            fhe_authority,
            to.balance_handle,
            amount_handle,
            new_to_handle,
        )?;

        bind_token_account_acl(
            &ctx.accounts.owner,
            &ctx.accounts.from_account,
            ctx.accounts.from_owner_output_acl.to_account_info(),
            &ctx.accounts.zama_event_authority,
            &ctx.accounts.zama_program,
            &ctx.accounts.system_program,
            from_nonce,
            new_from_handle,
            from.owner,
            AclPermission::UserDecrypt,
        )?;
        bind_token_account_acl(
            &ctx.accounts.owner,
            &ctx.accounts.from_account,
            ctx.accounts.from_compute_output_acl.to_account_info(),
            &ctx.accounts.zama_event_authority,
            &ctx.accounts.zama_program,
            &ctx.accounts.system_program,
            from_nonce,
            new_from_handle,
            fhe_authority,
            AclPermission::Compute,
        )?;
        bind_token_account_acl(
            &ctx.accounts.owner,
            &ctx.accounts.to_account,
            ctx.accounts.to_owner_output_acl.to_account_info(),
            &ctx.accounts.zama_event_authority,
            &ctx.accounts.zama_program,
            &ctx.accounts.system_program,
            to_nonce,
            new_to_handle,
            to.owner,
            AclPermission::UserDecrypt,
        )?;
        bind_token_account_acl(
            &ctx.accounts.owner,
            &ctx.accounts.to_account,
            ctx.accounts.to_compute_output_acl.to_account_info(),
            &ctx.accounts.zama_event_authority,
            &ctx.accounts.zama_program,
            &ctx.accounts.system_program,
            to_nonce,
            new_to_handle,
            fhe_authority,
            AclPermission::Compute,
        )?;

        let from = &mut ctx.accounts.from_account;
        from.balance_handle = new_from_handle;
        from.next_acl_nonce = from_nonce
            .checked_add(1)
            .ok_or(ConfidentialTokenError::AclNonceOverflow)?;

        let to = &mut ctx.accounts.to_account;
        to.balance_handle = new_to_handle;
        to.next_acl_nonce = to_nonce
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
    pub current_compute_acl: Account<'info, zama_host::AclRecord>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub owner_output_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub compute_output_acl: UncheckedAccount<'info>,
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
    pub from_current_compute_acl: Account<'info, zama_host::AclRecord>,
    pub to_current_compute_acl: Account<'info, zama_host::AclRecord>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub from_owner_output_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub from_compute_output_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub to_owner_output_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub to_compute_output_acl: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    pub zama_program: Program<'info, ZamaHost>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ConfidentialMint {
    pub authority: Pubkey,
    pub fhe_authority: Pubkey,
    pub underlying_mint: Pubkey,
    pub decimals: u8,
}

impl ConfidentialMint {
    pub const SPACE: usize = 32 + 32 + 32 + 1;
}

#[account]
pub struct ConfidentialTokenAccount {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub balance_handle: [u8; 32],
    pub next_acl_nonce: u64,
    pub bump: u8,
}

impl ConfidentialTokenAccount {
    pub const SPACE: usize = 32 + 32 + 32 + 8 + 1;
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
}

fn current_acl_nonce(token_account: &ConfidentialTokenAccount) -> Result<u64> {
    token_account
        .next_acl_nonce
        .checked_sub(1)
        .ok_or(ConfidentialTokenError::AclNonceOverflow.into())
}

fn assert_compute_acl<'info>(
    zama_program: &Program<'info, ZamaHost>,
    acl_record: AccountInfo<'info>,
    scope: Pubkey,
    handle: [u8; 32],
    subject: Pubkey,
) -> Result<()> {
    cpi::assert_acl_record(
        CpiContext::new(
            zama_program.to_account_info(),
            AssertAclRecord { acl_record },
        ),
        scope,
        handle,
        subject,
        AclPermission::Compute,
    )
}

fn emit_binary_op<'info>(
    zama_event_authority: &UncheckedAccount<'info>,
    zama_program: &Program<'info, ZamaHost>,
    op: FheBinaryOpCode,
    subject: Pubkey,
    lhs: [u8; 32],
    rhs: [u8; 32],
    result: [u8; 32],
) -> Result<()> {
    cpi::fhe_binary_op(
        CpiContext::new(
            zama_program.to_account_info(),
            EmitProtocolEvent {
                event_authority: zama_event_authority.to_account_info(),
                program: zama_program.to_account_info(),
            },
        ),
        op,
        subject,
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
    token_account: &Account<'info, ConfidentialTokenAccount>,
    acl_record: AccountInfo<'info>,
    zama_event_authority: &UncheckedAccount<'info>,
    zama_program: &Program<'info, ZamaHost>,
    system_program: &Program<'info, System>,
    acl_nonce: u64,
    handle: [u8; 32],
    subject: Pubkey,
    permission: AclPermission,
) -> Result<()> {
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
                scope_authority: token_account.to_account_info(),
                acl_record,
                system_program: system_program.to_account_info(),
                event_authority: zama_event_authority.to_account_info(),
                program: zama_program.to_account_info(),
            },
            signer_seeds,
        ),
        acl_nonce,
        token_account.key(),
        handle,
        subject,
        permission,
    )
}
