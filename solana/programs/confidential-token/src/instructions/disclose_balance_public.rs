//! Owner-authorized public disclosure of a confidential balance.
//!
//! Records an exact public-decrypt MMR leaf on the owner's balance lineage,
//! marking its current handle publicly decryptable. This is the on-chain trigger
//! the KMS public-decrypt path consumes: the indexer picks up the public-decrypt
//! leaf, the SDK builds a public-decrypt proof against it, and the KMS authorizes
//! a public decrypt of the disclosed handle. Encrypted-value ACL + MMR PoC
//! (fhevm-internal#1569).

use super::*;

/// Accounts for publicly disclosing a confidential balance.
#[derive(Accounts)]
pub struct DiscloseBalancePublic<'info> {
    /// Token owner authorizing disclosure of their own balance, and CPI payer for
    /// any lineage-account growth the appended leaf requires.
    #[account(mut)]
    pub owner: Signer<'info>,
    /// The owner's confidential token account; its balance lineage is disclosed.
    #[account(
        mut,
        seeds = [b"token-account", token_account.mint.as_ref(), owner.key().as_ref()],
        bump = token_account.bump,
    )]
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// CHECK: validated as the canonical balance lineage PDA inside the CPI helper.
    #[account(mut)]
    pub balance_value_acl: UncheckedAccount<'info>,
    pub zama_program: Program<'info, ZamaHost>,
    pub system_program: Program<'info, System>,
}

pub fn disclose_balance_public(ctx: Context<DiscloseBalancePublic>) -> Result<()> {
    let token_account = ctx.accounts.token_account.as_ref();
    let mint = token_account.mint;
    mark_value_acl_public(
        &LineageCpi {
            zama_program: ctx.accounts.zama_program.to_account_info(),
            encrypted_value_acl: ctx.accounts.balance_value_acl.to_account_info(),
            payer: ctx.accounts.owner.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        },
        LineageAuthority::balance(token_account),
        mint,
    )
}
