//! Deposits underlying assets and mints shares at the current on-the-fly price.

use super::*;

/// Accounts for a deposit.
#[derive(Accounts)]
pub struct Deposit<'info> {
    /// Depositor and transfer authority over `depositor_underlying`.
    #[account(mut)]
    pub depositor: Signer<'info>,
    /// Vault whose share price the deposit is measured against.
    #[account(
        has_one = underlying_mint @ DemoVaultError::UnderlyingMintMismatch,
        has_one = share_mint @ DemoVaultError::ShareMintMismatch,
        has_one = vault_token_account @ DemoVaultError::VaultTokenAccountMismatch,
    )]
    pub vault: Box<Account<'info, Vault>>,
    /// CHECK: PDA authority that mints shares; validated by seeds + stored bump.
    #[account(seeds = [VAULT_AUTHORITY_SEED, vault.key().as_ref()], bump = vault.authority_bump)]
    pub vault_authority: UncheckedAccount<'info>,
    /// Underlying SPL mint, read for its decimals.
    pub underlying_mint: Box<Account<'info, Mint>>,
    /// Share mint, whose supply is the total shares outstanding.
    #[account(mut)]
    pub share_mint: Box<Account<'info, Mint>>,
    /// Depositor's underlying token account (source).
    #[account(
        mut,
        constraint = depositor_underlying.mint == underlying_mint.key() @ DemoVaultError::MintMismatch,
        constraint = depositor_underlying.owner == depositor.key() @ DemoVaultError::OwnerMismatch,
    )]
    pub depositor_underlying: Box<Account<'info, TokenAccount>>,
    /// Vault token account whose balance is the total assets under management.
    #[account(mut)]
    pub vault_token_account: Box<Account<'info, TokenAccount>>,
    /// Depositor's share token account (destination for minted shares).
    #[account(
        mut,
        constraint = depositor_shares.mint == share_mint.key() @ DemoVaultError::MintMismatch,
    )]
    pub depositor_shares: Box<Account<'info, TokenAccount>>,
    /// SPL token program.
    pub token_program: Program<'info, Token>,
}

/// Pulls `amount` underlying in and mints the corresponding shares.
pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    require!(amount > 0, DemoVaultError::ZeroAmount);

    // Price against the balances *before* this deposit moves any tokens.
    let total_assets = ctx.accounts.vault_token_account.amount;
    let total_shares = ctx.accounts.share_mint.supply;
    let shares = assets_to_shares(amount, total_assets, total_shares)?;
    require!(shares > 0, DemoVaultError::ZeroShares);

    // Pull the underlying in under the depositor's own signature.
    spl_token::transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.key(),
            TransferChecked {
                from: ctx.accounts.depositor_underlying.to_account_info(),
                mint: ctx.accounts.underlying_mint.to_account_info(),
                to: ctx.accounts.vault_token_account.to_account_info(),
                authority: ctx.accounts.depositor.to_account_info(),
            },
        ),
        amount,
        ctx.accounts.underlying_mint.decimals,
    )?;

    // Mint shares under the vault authority PDA.
    let vault_key = ctx.accounts.vault.key();
    let authority_bump = [ctx.accounts.vault.authority_bump];
    let authority_seeds: &[&[u8]] = &[VAULT_AUTHORITY_SEED, vault_key.as_ref(), &authority_bump];
    spl_token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.key(),
            MintTo {
                mint: ctx.accounts.share_mint.to_account_info(),
                to: ctx.accounts.depositor_shares.to_account_info(),
                authority: ctx.accounts.vault_authority.to_account_info(),
            },
            &[authority_seeds],
        ),
        shares,
    )?;

    emit!(Deposited {
        version: APP_EVENT_VERSION,
        vault: vault_key,
        depositor: ctx.accounts.depositor.key(),
        assets: amount,
        shares,
    });
    Ok(())
}
