//! Donates underlying assets to the vault without minting shares — the demo's
//! simulated yield. The donation raises the share price for existing holders.

use super::*;

/// Accounts for a harvest (donation).
#[derive(Accounts)]
pub struct Harvest<'info> {
    /// Donor and transfer authority over `donor_underlying`.
    #[account(mut)]
    pub donor: Signer<'info>,
    /// Vault receiving the donation.
    #[account(
        has_one = underlying_mint @ DemoVaultError::UnderlyingMintMismatch,
        has_one = vault_token_account @ DemoVaultError::VaultTokenAccountMismatch,
    )]
    pub vault: Box<Account<'info, Vault>>,
    /// Underlying SPL mint, read for its decimals.
    pub underlying_mint: Box<Account<'info, Mint>>,
    /// Donor's underlying token account (source).
    #[account(
        mut,
        constraint = donor_underlying.mint == underlying_mint.key() @ DemoVaultError::MintMismatch,
        constraint = donor_underlying.owner == donor.key() @ DemoVaultError::OwnerMismatch,
    )]
    pub donor_underlying: Box<Account<'info, TokenAccount>>,
    /// Vault token account whose balance is increased by the donation.
    #[account(mut)]
    pub vault_token_account: Box<Account<'info, TokenAccount>>,
    /// SPL token program.
    pub token_program: Program<'info, Token>,
}

/// Moves `amount` underlying into the vault without minting shares.
pub fn harvest(ctx: Context<Harvest>, amount: u64) -> Result<()> {
    require!(amount > 0, DemoVaultError::ZeroAmount);

    spl_token::transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.key(),
            TransferChecked {
                from: ctx.accounts.donor_underlying.to_account_info(),
                mint: ctx.accounts.underlying_mint.to_account_info(),
                to: ctx.accounts.vault_token_account.to_account_info(),
                authority: ctx.accounts.donor.to_account_info(),
            },
        ),
        amount,
        ctx.accounts.underlying_mint.decimals,
    )?;

    emit!(Harvested {
        version: APP_EVENT_VERSION,
        vault: ctx.accounts.vault.key(),
        donor: ctx.accounts.donor.key(),
        assets: amount,
    });
    Ok(())
}
