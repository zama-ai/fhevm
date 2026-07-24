//! Confidential batcher for the Solana FHEVM PoC — both directions of the
//! confidential-vault design (DD-042, `solana/docs/CONFIDENTIAL_VAULTS.md`).
//!
//! One program serves deposits and redemptions: each `Batcher` config is one
//! DIRECTION instance (the EVM design's two batcher deployments), wiring a
//! join confidential mint (what users batch in) and a payout confidential
//! mint (what claims pay) around one public `demo-vault`. Deposit batchers
//! join with confidential underlying and pay confidential shares; redeem
//! batchers join with confidential shares and pay confidential underlying.
//!
//! Users join a batch with encrypted amounts; the batch's own confidential
//! token account accumulates them while encrypted; dispatch burns the batch
//! total and the KMS certifies the one public number; settle moves that
//! number through the vault (deposit or withdraw), wraps what comes back into
//! the payout confidential mint, and records the batch's informational public
//! rate; claim pays each user the exact proportional floor
//! `encrypted(joined) x payout_received / total_joined` in confidential
//! payout tokens. Individual amounts stay encrypted end to end — only each
//! batch's total is ever revealed.
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
/// Account layouts, PDA helpers, encrypted-value labels, and the payout math.
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
/// Re-export account layouts, PDA helpers, and payout math.
pub use state::*;

declare_id!("Cr1Tyzov2Jq9AYVn5zLSLQdyd8CkZJLemHYkj6qDqFmG");

/// Anchor entrypoint module for the confidential batcher.
#[program]
pub mod confidential_batcher {
    use super::*;

    /// Creates a batcher config for one direction, wiring a join confidential
    /// mint, a payout confidential mint, and one public vault together.
    /// Deposit batchers join with confidential underlying and pay confidential
    /// shares; redeem batchers join with confidential shares and pay
    /// confidential underlying. Permissionless one-time setup; the batcher
    /// holds no admin role afterwards.
    pub fn initialize_batcher(
        ctx: Context<InitializeBatcher>,
        min_batch_age_slots: u64,
        direction: BatchDirection,
    ) -> Result<()> {
        instructions::initialize_batcher(ctx, min_batch_age_slots, direction)
    }

    /// Opens the next batch: creates the `Batch` account, its per-batch
    /// authority PDA, its own confidential join and payout token accounts,
    /// and its plain SPL accounts for settle's legs. Permissionless; requires
    /// the previous batch of the same batcher to have been dispatched (a
    /// batcher's batches never overlap while pending; the other direction's
    /// batcher is independent). `authority_funding_lamports` is moved from
    /// the payer to the batch authority PDA, which pays the rent the token
    /// CPIs charge to the account owner. Unspent funding stays on the PDA and
    /// is unrecoverable by design in this PoC (no sweep instruction).
    pub fn open_batch(ctx: Context<OpenBatch>, authority_funding_lamports: u64) -> Result<()> {
        instructions::open_batch(ctx, authority_funding_lamports)
    }

    /// Joins the pending batch with the batcher's join token: one user-signed
    /// transaction that CPIs the coprocessor-attested confidential transfer
    /// into the batch's own token account, then re-materializes the
    /// transferred amount into the user's joined encrypted value account (audience: user +
    /// batch authority) in the same transaction. Repeated joins accumulate.
    pub fn join<'info>(
        ctx: Context<'info, Join<'info>>,
        amount_attestation: zama_host::CoprocessorInputAttestation,
    ) -> Result<()> {
        instructions::join(ctx, amount_attestation)
    }

    /// Leaves the pending batch before dispatch: transfers the user's exact
    /// recorded amount back from the batch account (all-or-nothing) and
    /// resets the joined encrypted value account to zero. Always available while pending;
    /// there is no exit between dispatch and settle (fhevm-internal#1773).
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
    /// total: redeems the plain tokens, moves them through the vault
    /// (deposit for deposit batchers, withdraw for redeem batchers), wraps
    /// the received payout into confidential payout tokens, and records the
    /// batch's informational public rate. A zero-total batch is canceled
    /// instead. Permissionless.
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

    /// Claims a user's confidential payout from a settled batch: one MulDiv
    /// eval frame — the exact proportional floor
    /// `encrypted(joined) x payout_received / total_joined` — then a
    /// confidential transfer of the resulting handle to the user's payout
    /// account. Permissionless pull — anyone can trigger a user's claim.
    pub fn claim<'info>(ctx: Context<'info, Claim<'info>>) -> Result<()> {
        instructions::claim(ctx)
    }
}
