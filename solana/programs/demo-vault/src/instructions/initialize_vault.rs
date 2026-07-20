//! Creates a vault: its state account, the PDA-owned share mint, and the
//! PDA-owned underlying token account.

use super::*;

/// Accounts for initializing a vault.
#[derive(Accounts)]
pub struct InitializeVault<'info> {
    /// Rent payer and the account that submits the initialization.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Vault state account, created here. A fresh keypair signs its own creation.
    #[account(init, payer = payer, space = 8 + Vault::SPACE)]
    pub vault: Box<Account<'info, Vault>>,
    /// Underlying SPL mint the vault accepts.
    pub underlying_mint: Box<Account<'info, Mint>>,
    /// CHECK: PDA that owns the vault token account and the share mint. Only a
    /// signing authority, never deserialized.
    #[account(seeds = [VAULT_AUTHORITY_SEED, vault.key().as_ref()], bump)]
    pub vault_authority: UncheckedAccount<'info>,
    /// Share mint created here, with the same decimals as the underlying and the
    /// vault authority PDA as its mint authority.
    #[account(
        init,
        payer = payer,
        seeds = [SHARE_MINT_SEED, vault.key().as_ref()],
        bump,
        mint::decimals = underlying_mint.decimals,
        mint::authority = vault_authority,
    )]
    pub share_mint: Box<Account<'info, Mint>>,
    /// Vault token account created here, owned by the vault authority PDA.
    #[account(
        init,
        payer = payer,
        seeds = [VAULT_TOKEN_ACCOUNT_SEED, vault.key().as_ref()],
        bump,
        token::mint = underlying_mint,
        token::authority = vault_authority,
    )]
    pub vault_token_account: Box<Account<'info, TokenAccount>>,
    /// SPL token program.
    pub token_program: Program<'info, Token>,
    /// System program used for account creation.
    pub system_program: Program<'info, System>,
}

/// Records vault state and emits [`VaultInitialized`]. The share mint and vault
/// token account are created by the Anchor `init` constraints above.
pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
    let vault_authority = ctx.accounts.vault_authority.key();
    let underlying_mint = ctx.accounts.underlying_mint.key();
    let share_mint = ctx.accounts.share_mint.key();
    let vault_token_account = ctx.accounts.vault_token_account.key();

    let vault = &mut ctx.accounts.vault;
    vault.underlying_mint = underlying_mint;
    vault.share_mint = share_mint;
    vault.vault_token_account = vault_token_account;
    vault.authority_bump = ctx.bumps.vault_authority;

    emit!(VaultInitialized {
        version: APP_EVENT_VERSION,
        vault: vault.key(),
        underlying_mint,
        share_mint,
        vault_token_account,
        vault_authority,
    });
    Ok(())
}
