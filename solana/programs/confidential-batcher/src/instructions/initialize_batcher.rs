//! Creates the batcher config wiring the two confidential mints and the vault
//! for one direction.

use super::*;

/// Accounts for initializing a batcher.
#[derive(Accounts)]
pub struct InitializeBatcher<'info> {
    /// Rent payer and the account that submits the initialization.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Batcher config account, created here. A fresh keypair signs its own
    /// creation (the demo-vault `Vault` pattern).
    #[account(init, payer = payer, space = 8 + Batcher::SPACE)]
    pub batcher: Box<Account<'info, Batcher>>,
    /// Confidential mint users join batches with.
    pub join_confidential_mint: Box<Account<'info, ct::ConfidentialMint>>,
    /// Confidential mint claims pay out in.
    pub payout_confidential_mint: Box<Account<'info, ct::ConfidentialMint>>,
    /// Public vault the batcher fronts.
    pub vault: Box<Account<'info, demo_vault::Vault>>,
    /// System program used for account creation.
    pub system_program: Program<'info, System>,
}

/// Validates the direction's mint/vault wiring and records the batcher config.
pub fn initialize_batcher(
    ctx: Context<InitializeBatcher>,
    min_batch_age_slots: u64,
    direction: BatchDirection,
) -> Result<()> {
    // The join mint must wrap what the batch total goes INTO the vault as,
    // and the payout mint what comes back OUT.
    let (join_vault_mint, payout_vault_mint) = match direction {
        BatchDirection::Deposit => (
            ctx.accounts.vault.underlying_mint,
            ctx.accounts.vault.share_mint,
        ),
        BatchDirection::Redeem => (
            ctx.accounts.vault.share_mint,
            ctx.accounts.vault.underlying_mint,
        ),
    };
    require_keys_eq!(
        ctx.accounts.join_confidential_mint.underlying_mint,
        join_vault_mint,
        BatcherError::JoinMintVaultMismatch
    );
    require_keys_eq!(
        ctx.accounts.payout_confidential_mint.underlying_mint,
        payout_vault_mint,
        BatcherError::PayoutMintVaultMismatch
    );

    let batcher = &mut ctx.accounts.batcher;
    batcher.direction = direction;
    batcher.join_confidential_mint = ctx.accounts.join_confidential_mint.key();
    batcher.payout_confidential_mint = ctx.accounts.payout_confidential_mint.key();
    batcher.vault = ctx.accounts.vault.key();
    batcher.min_batch_age_slots = min_batch_age_slots;
    batcher.next_batch_index = 0;

    emit!(BatcherInitialized {
        version: APP_EVENT_VERSION,
        batcher: batcher.key(),
        direction,
        join_confidential_mint: batcher.join_confidential_mint,
        payout_confidential_mint: batcher.payout_confidential_mint,
        vault: batcher.vault,
        min_batch_age_slots,
    });
    Ok(())
}
