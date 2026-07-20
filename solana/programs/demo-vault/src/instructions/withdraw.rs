//! Burns shares and returns underlying assets at the current on-the-fly price.

use super::*;

/// Accounts for a withdraw.
#[derive(Accounts)]
pub struct Withdraw<'info> {
    /// Share owner and burn authority over `owner_shares`.
    #[account(mut)]
    pub owner: Signer<'info>,
    /// Vault whose share price the withdraw is measured against.
    #[account(
        has_one = underlying_mint @ DemoVaultError::UnderlyingMintMismatch,
        has_one = share_mint @ DemoVaultError::ShareMintMismatch,
        has_one = vault_token_account @ DemoVaultError::VaultTokenAccountMismatch,
    )]
    pub vault: Box<Account<'info, Vault>>,
    /// CHECK: PDA authority that owns the vault token account; validated by seeds + stored bump.
    #[account(seeds = [VAULT_AUTHORITY_SEED, vault.key().as_ref()], bump = vault.authority_bump)]
    pub vault_authority: UncheckedAccount<'info>,
    /// Underlying SPL mint, read for its decimals.
    pub underlying_mint: Box<Account<'info, Mint>>,
    /// Share mint, whose supply is the total shares outstanding.
    #[account(mut)]
    pub share_mint: Box<Account<'info, Mint>>,
    /// Owner's share token account (burn source).
    #[account(
        mut,
        constraint = owner_shares.mint == share_mint.key() @ DemoVaultError::MintMismatch,
        constraint = owner_shares.owner == owner.key() @ DemoVaultError::OwnerMismatch,
    )]
    pub owner_shares: Box<Account<'info, TokenAccount>>,
    /// Vault token account whose balance is the total assets under management.
    #[account(mut)]
    pub vault_token_account: Box<Account<'info, TokenAccount>>,
    /// Owner's underlying token account (destination for returned assets).
    #[account(
        mut,
        constraint = owner_underlying.mint == underlying_mint.key() @ DemoVaultError::MintMismatch,
    )]
    pub owner_underlying: Box<Account<'info, TokenAccount>>,
    /// SPL token program.
    pub token_program: Program<'info, Token>,
}

/// Burns `shares` and returns the corresponding underlying assets.
pub fn withdraw(ctx: Context<Withdraw>, shares: u64) -> Result<()> {
    require!(shares > 0, DemoVaultError::ZeroAmount);
    require!(
        ctx.accounts.owner_shares.amount >= shares,
        DemoVaultError::InsufficientShares
    );

    // Price against the balances *before* this withdraw burns or moves anything.
    let total_assets = ctx.accounts.vault_token_account.amount;
    let total_shares = ctx.accounts.share_mint.supply;
    let assets = shares_to_assets(shares, total_assets, total_shares)?;
    require!(assets > 0, DemoVaultError::ZeroAssets);

    // Burn the owner's shares under their own signature.
    spl_token::burn(
        CpiContext::new(
            ctx.accounts.token_program.key(),
            Burn {
                mint: ctx.accounts.share_mint.to_account_info(),
                from: ctx.accounts.owner_shares.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ),
        shares,
    )?;

    // Pay out the underlying under the vault authority PDA.
    let vault_key = ctx.accounts.vault.key();
    let authority_bump = [ctx.accounts.vault.authority_bump];
    let authority_seeds: &[&[u8]] = &[VAULT_AUTHORITY_SEED, vault_key.as_ref(), &authority_bump];
    spl_token::transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.key(),
            TransferChecked {
                from: ctx.accounts.vault_token_account.to_account_info(),
                mint: ctx.accounts.underlying_mint.to_account_info(),
                to: ctx.accounts.owner_underlying.to_account_info(),
                authority: ctx.accounts.vault_authority.to_account_info(),
            },
            &[authority_seeds],
        ),
        assets,
        ctx.accounts.underlying_mint.decimals,
    )?;

    emit!(Withdrawn {
        version: APP_EVENT_VERSION,
        vault: vault_key,
        owner: ctx.accounts.owner.key(),
        shares,
        assets,
    });
    Ok(())
}
