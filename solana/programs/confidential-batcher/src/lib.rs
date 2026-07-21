//! Confidential batcher for the Solana FHEVM PoC — the deposit path of the
//! confidential-vault design (DD-042, `solana/docs/CONFIDENTIAL_VAULTS.md`).
//!
//! Users join a batch with encrypted amounts; the batch's own confidential
//! token account accumulates them while encrypted; dispatch burns the batch
//! total and the KMS certifies the one public number; settle deposits that
//! number into the public `demo-vault`, wraps the received shares into
//! confidential shares, and freezes the batch's public share rate; claim pays
//! each user `encrypted(deposit) x rate / RATE_SCALE` in confidential shares.
//! Individual amounts stay encrypted end to end — only each batch's total is
//! ever revealed.
//!
//! This program evolves the earlier `confidential-deposit-app` reference: the
//! app-driven join (one user signature propagating through the transfer CPI)
//! is kept, and the rest of the batch lifecycle is built around it.

// Anchor macros generate framework-shaped code that trips rustc/Clippy checks.
#![allow(unexpected_cfgs)]
#![allow(clippy::diverging_sub_expression, clippy::too_many_arguments)]

/// Shared constants, PDA seed bytes, and the fixed rate scale.
pub mod constants;
/// Program-specific errors returned by confidential-batcher instructions.
pub mod errors;
/// App-local events.
pub mod events;
mod fhe;
/// Instruction account contexts and handlers.
pub mod instructions;
/// Account layouts, PDA helpers, encrypted-value labels, and the rate math.
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
pub use instructions::{Claim, Dispatch, InitializeBatcher, Join, OpenBatch, Quit, Settle};
/// Re-export account layouts, PDA helpers, and rate math.
pub use state::*;

declare_id!("415fK9iaJJzMHbwkGD4pWgAmDkMvp7Wbd9TTPUtyRX1g");

/// Anchor entrypoint module for the confidential batcher.
#[program]
pub mod confidential_batcher {
    use super::*;

    /// Creates the batcher config wiring one deposit confidential mint, one
    /// confidential-shares mint, and one public vault together. Permissionless
    /// one-time setup; the batcher holds no admin role afterwards.
    pub fn initialize_batcher(
        ctx: Context<InitializeBatcher>,
        min_batch_age_slots: u64,
    ) -> Result<()> {
        instructions::initialize_batcher(ctx, min_batch_age_slots)
    }

    /// Opens the next batch: creates the `Batch` account, its per-batch
    /// authority PDA, its own confidential deposit and shares token accounts,
    /// and its plain SPL underlying/share accounts. Permissionless; requires
    /// the previous batch to have been dispatched (batches never overlap while
    /// pending). `authority_funding_lamports` is moved from the payer to the
    /// batch authority PDA, which pays the rent the token CPIs charge to the
    /// account owner. Unspent funding stays on the PDA and is unrecoverable
    /// by design in this PoC (no sweep instruction).
    pub fn open_batch(ctx: Context<OpenBatch>, authority_funding_lamports: u64) -> Result<()> {
        instructions::open_batch(ctx, authority_funding_lamports)
    }

    /// Joins the pending batch: one user-signed transaction that CPIs the
    /// coprocessor-attested confidential transfer into the batch's own token
    /// account, then re-materializes the transferred amount into the user's
    /// batch deposit lineage (audience: user + batch authority) in the same
    /// transaction. Repeated joins accumulate.
    pub fn join<'info>(
        ctx: Context<'info, Join<'info>>,
        amount_attestation: zama_host::CoprocessorInputAttestation,
    ) -> Result<()> {
        instructions::join(ctx, amount_attestation)
    }

    /// Leaves the pending batch before dispatch: transfers the user's exact
    /// recorded deposit back from the batch account (all-or-nothing) and
    /// resets the deposit lineage to zero. Always available while pending.
    pub fn quit<'info>(ctx: Context<'info, Quit<'info>>) -> Result<()> {
        instructions::quit(ctx)
    }

    /// Dispatches the batch once it is old enough: burns the batch account's
    /// full encrypted balance via `confidential_burn_from_value` and records
    /// the born-public burned handle the KMS will certify. Permissionless.
    pub fn dispatch(ctx: Context<Dispatch>) -> Result<()> {
        instructions::dispatch(ctx)
    }

    /// Settles a dispatched batch with the KMS certificate for its burned
    /// total: redeems the plain tokens, deposits them into the public vault,
    /// wraps the received shares into confidential shares, and freezes the
    /// batch's public share rate. A zero-total batch is canceled instead.
    /// Permissionless.
    pub fn settle(
        ctx: Context<Settle>,
        cleartext_total: u64,
        signatures: Vec<[u8; 65]>,
        extra_data: Vec<u8>,
        proof: zama_host::instructions::MmrInclusionProof,
        authority_funding_lamports: u64,
    ) -> Result<()> {
        instructions::settle(
            ctx,
            cleartext_total,
            signatures,
            extra_data,
            proof,
            authority_funding_lamports,
        )
    }

    /// Claims a user's confidential shares from a settled batch: one MulDiv
    /// eval frame (`encrypted(deposit) x rate / RATE_SCALE`) then a
    /// confidential transfer of the resulting handle to the user's shares
    /// account. Permissionless pull — anyone can trigger a user's claim.
    pub fn claim<'info>(ctx: Context<'info, Claim<'info>>) -> Result<()> {
        instructions::claim(ctx)
    }
}
