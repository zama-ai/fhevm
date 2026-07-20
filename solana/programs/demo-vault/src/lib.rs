//! Minimal public share-mint vault for the Solana FHEVM PoC.
//!
//! This is the public half of the confidential-vault design (see
//! `solana/docs/CONFIDENTIAL_VAULTS.md` and DD-042): tokens in, shares out,
//! share price computed on the fly as `assets / shares`, and yield delivered as
//! share-price appreciation (never rebasing). The confidential batcher fronts
//! this vault; its instruction interface deliberately mirrors Jupiter Earn's
//! (deposit/withdraw taking amounts, asset/share duality, a PDA authority that
//! owns the assets and the share mint) so the batcher can be retargeted to a
//! real venue later.
//!
//! It is standalone: plain SPL Token only, no dependency on `zama-host` or
//! `confidential-token`, and it emits no Zama host protocol events (the
//! coprocessor host-listener never ingests this program).

// Anchor macros generate framework-shaped code that trips rustc/Clippy checks.
#![allow(unexpected_cfgs)]
#![allow(
    clippy::diverging_sub_expression,
    clippy::too_many_arguments,
    clippy::result_large_err
)]

/// Shared constants, seed bytes, and virtual-offset parameters.
pub mod constants;
/// Program-specific errors returned by demo-vault instructions.
pub mod errors;
/// App-local events.
pub mod events;
/// Instruction account contexts and handlers.
pub mod instructions;
/// Vault account layout and share-price math.
pub mod state;

use anchor_lang::prelude::*;

/// Re-export constants for generated clients and tests.
pub use constants::*;
/// Re-export errors for generated clients and tests.
pub use errors::*;
/// Re-export events for generated clients and tests.
pub use events::*;
use instructions::*;
/// Re-export instruction account contexts for tests.
pub use instructions::{Deposit, Harvest, InitializeVault, Withdraw};
/// Re-export the vault layout and share-price helpers.
pub use state::*;

declare_id!("C6TBzPBWPJYY3fbsDV63nnT3s9vEjaWQAqdeAZpzowH7");

/// Anchor entrypoint module for the demo vault.
#[program]
pub mod demo_vault {
    use super::*;

    /// Creates the vault state account, its PDA-owned share mint, and its
    /// PDA-owned underlying token account. Permissionless; no admin role beyond
    /// this one-time setup.
    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        instructions::initialize_vault(ctx)
    }

    /// Deposits `amount` underlying assets and mints shares at the current price
    /// (rounded down). Permissionless.
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit(ctx, amount)
    }

    /// Burns `shares` and returns underlying assets at the current price
    /// (rounded down). Permissionless.
    pub fn withdraw(ctx: Context<Withdraw>, shares: u64) -> Result<()> {
        instructions::withdraw(ctx, shares)
    }

    /// Donates `amount` underlying to the vault without minting shares, raising
    /// the share price for existing holders. Permissionless (demo-only; a real
    /// vault gates this behind a keeper/strategy role).
    pub fn harvest(ctx: Context<Harvest>, amount: u64) -> Result<()> {
        instructions::harvest(ctx, amount)
    }
}
