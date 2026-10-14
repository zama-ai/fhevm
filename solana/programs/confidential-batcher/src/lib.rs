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
//! number through the vault (deposit or withdraw) and wraps what comes back
//! into the payout confidential mint; claim pays each user the exact
//! proportional floor `encrypted(joined) x payout_received / total_joined` in
//! confidential payout tokens. Individual amounts stay encrypted end to end — only each
//! batch's total is ever revealed.
//!
//! This program evolves the earlier `confidential-deposit-app` reference: the
//! app-driven join (one user signature propagating through the transfer CPI)
//! is kept, and the rest of the batch lifecycle is built around it.
//!
//! Host levers. Every execution runs as one application: the token's executions as
//! `(confidential-token, mint)`, the batcher's own as `(confidential-batcher, batch)`. For each
//! application an instruction runs as, it takes an optional HCU block meter and trust record
//! (`<application>_hcu_block_meter`, `<application>_hcu_trusted_app_record`), which a client
//! supplies while the host's per-application block cap binds. While the host's deny list is on,
//! the remaining accounts are the deny records of the applications each execution touches, one
//! per application, execution by execution; each instruction documents its order.

// Anchor macros generate framework-shaped code that trips rustc/Clippy checks.
#![allow(unexpected_cfgs)]
#![allow(clippy::diverging_sub_expression, clippy::too_many_arguments)]

/// Shared constants and PDA seed bytes.
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
pub use instructions::{
    BatcherAdmin, CancelDispatch, Claim, Dispatch, InitializeBatcher, Join, Quit, SetAdmin, Settle,
};
/// Re-export account layouts, PDA helpers, and payout math.
pub use state::*;

// Written by build.rs from solana/environments/<PROGRAM_ENVIRONMENT>.json (DD-053).
include!(concat!(env!("OUT_DIR"), "/program_id.rs"));

#[cfg(feature = "admin-sweep")]
// Anchor's IDL parser does not resolve #[path] modules.
mod preview_cleanup {
    include!("../../preview_cleanup.rs");
}
#[cfg(feature = "admin-sweep")]
use preview_cleanup::*;

/// Anchor entrypoint module for the confidential batcher.
#[program]
pub mod confidential_batcher {
    use super::*;

    /// Preview reset: recover program-owned rent after recovering external accounts.
    #[cfg(feature = "admin-sweep")]
    pub fn close_owned_accounts<'info>(ctx: Context<'info, PreviewAdmin<'info>>) -> Result<()> {
        preview_cleanup::close_owned_accounts(ctx)
    }

    /// Preview reset: burn disposable tokens and recover PDA-owned token account rent.
    #[cfg(feature = "admin-sweep")]
    pub fn preview_close_token(ctx: Context<PreviewCloseToken>, seeds: Vec<Vec<u8>>) -> Result<()> {
        preview_cleanup::close_token(ctx, seeds)
    }

    /// Preview reset: return unused PDA funding to the deployer.
    #[cfg(feature = "admin-sweep")]
    pub fn preview_drain(ctx: Context<PreviewDrain>, seeds: Vec<Vec<u8>>) -> Result<()> {
        preview_cleanup::drain(ctx, seeds)
    }

    /// Creates a batcher config for one direction, wiring a join confidential
    /// mint, a payout confidential mint, and one public vault together.
    /// Deposit batchers join with confidential underlying and pay confidential
    /// shares; redeem batchers join with confidential shares and pay
    /// confidential underlying. Permissionless one-time setup. `admin` may later pause the
    /// batcher, set the policy that batches opened from then on copy, and hand the role on.
    ///
    /// Also opens batch 0: the `Batch` account, its per-batch authority PDA, its own confidential
    /// join and payout token accounts unless they already exist, and its plain SPL accounts for
    /// settle's phases. `authority_funding_lamports` is moved from the payer to the batch authority
    /// PDA, which pays the rent the token CPIs charge to the account owner. Unspent funding stays on
    /// the PDA until the batch is finished and `reclaim_batch_authority` returns it.
    /// Deny records: `(token, join mint)`, then `(token, payout mint)`.
    pub fn initialize_batcher<'info>(
        ctx: Context<'info, InitializeBatcher<'info>>,
        min_batch_age_secs: u64,
        settle_deadline_secs: u64,
        direction: BatchDirection,
        authority_funding_lamports: u64,
        admin: Pubkey,
    ) -> Result<()> {
        instructions::initialize_batcher(
            ctx,
            min_batch_age_secs,
            settle_deadline_secs,
            direction,
            authority_funding_lamports,
            admin,
        )
    }

    /// Pauses the batcher (admin-only): joins, dispatches and settles of a nonzero total are
    /// refused until `unpause`. Refuses when already paused.
    pub fn pause(ctx: Context<BatcherAdmin>) -> Result<()> {
        instructions::pause(ctx)
    }

    /// Unpauses the batcher (admin-only). Refuses when not paused.
    pub fn unpause(ctx: Context<BatcherAdmin>) -> Result<()> {
        instructions::unpause(ctx)
    }

    /// Sets the minimum batch age that batches opened from now on copy (admin-only), at most
    /// `MAX_MIN_BATCH_AGE_SECS`. Works while paused.
    pub fn set_min_batch_age(ctx: Context<BatcherAdmin>, min_batch_age_secs: u64) -> Result<()> {
        instructions::set_min_batch_age(ctx, min_batch_age_secs)
    }

    /// Sets the settle deadline that batches opened from now on copy (admin-only), at most
    /// `MAX_SETTLE_DEADLINE_SECS`. With zero, a batch cannot settle and anyone may cancel it once
    /// dispatched. Works while paused.
    pub fn set_settle_deadline(
        ctx: Context<BatcherAdmin>,
        settle_deadline_secs: u64,
    ) -> Result<()> {
        instructions::set_settle_deadline(ctx, settle_deadline_secs)
    }

    /// Hands the admin role to `new_admin`. The current admin signs; a new keypair co-signs; a
    /// new PDA does not, provided it already exists as a program-owned account.
    pub fn set_admin(ctx: Context<SetAdmin>) -> Result<()> {
        instructions::set_admin(ctx)
    }

    /// Joins the pending batch with the batcher's join token (refused while paused): one user-signed
    /// transaction that CPIs the coprocessor-attested confidential transfer
    /// into the batch's token account. The token returns the transferred handle and grants
    /// the JoinRecord Store access through transient store. The batcher adds it to the user's
    /// contribution slot (decryptable by the user). Repeated joins accumulate.
    /// Deny records: `(token, join mint)`, then `(batcher, batch)`.
    pub fn join<'info>(
        ctx: Context<'info, Join<'info>>,
        amount_attestation: zama_host::CoprocessorInputAttestation,
    ) -> Result<()> {
        instructions::join(ctx, amount_attestation)
    }

    /// Leaves a pending batch before dispatch, or a refunding batch after dispatch cancellation:
    /// transfers the user's exact
    /// recorded amount back from the batch account (all-or-nothing) and
    /// resets the joined encrypted store to zero. It also closes the join record, returning its
    /// rent to the user. The user signs a pending batch's quit; anyone may run a refunding batch's
    /// quit for the user.
    /// Deny records: `(token, join mint)` and `(batcher, batch)` for the refund, then
    /// `(batcher, batch)` again for the reset.
    pub fn quit<'info>(ctx: Context<'info, Quit<'info>>) -> Result<()> {
        instructions::quit(ctx)
    }

    /// Dispatches the batch once it is old enough: burns the batch account's
    /// full encrypted balance via `confidential_burn_from_value` and records
    /// the created-public burned handle the KMS will certify. Permissionless; refused while paused.
    /// In the same instruction it opens the next batch, as `initialize_batcher` opens batch 0, with
    /// `authority_funding_lamports` moved from the payer to the next batch's authority PDA.
    /// Deny records: `(token, join mint)` for the burn, then `(token, join mint)` and
    /// `(token, payout mint)` for the next batch's token accounts.
    pub fn dispatch<'info>(
        ctx: Context<'info, Dispatch<'info>>,
        authority_funding_lamports: u64,
    ) -> Result<()> {
        instructions::dispatch(ctx, authority_funding_lamports)
    }

    /// Cancels a dispatched burn when settlement cannot complete. Anyone may call it once the batch's
    /// settle deadline has passed, and only then. The burned total is restored to the batch token account
    /// and encrypted total supply, and the batch becomes refund-only so users can retrieve their
    /// recorded joins through `quit`.
    /// Deny records: `(token, join mint)`.
    pub fn cancel_dispatch<'info>(
        ctx: Context<'info, CancelDispatch<'info>>,
        authority_funding_lamports: u64,
    ) -> Result<()> {
        instructions::cancel_dispatch(ctx, authority_funding_lamports)
    }

    /// Settles a dispatched batch with the KMS certificate for its burned
    /// total: redeems the plain tokens, moves them through the vault
    /// (deposit for deposit batchers, withdraw for redeem batchers), and wraps
    /// the received payout into confidential payout tokens. A zero-total batch
    /// is canceled instead, and a deposit worth zero vault shares is wrapped back and opens
    /// refunds. Permissionless, before the batch's settle deadline; refused while paused unless
    /// the total is zero.
    /// Deny records: `(token, payout mint)` then `(token, join mint)` for the zero-shares refund, or
    /// none for a zero total, which runs no execution.
    pub fn settle<'info>(
        ctx: Context<'info, Settle<'info>>,
        cleartext_total: u64,
        signatures: Vec<[u8; 65]>,
        extra_data: Vec<u8>,
        authority_funding_lamports: u64,
    ) -> Result<()> {
        instructions::settle(
            ctx,
            cleartext_total,
            signatures,
            extra_data,
            authority_funding_lamports,
        )
    }

    /// Claims a user's confidential payout from a settled batch: one MulDiv
    /// batch — the exact proportional floor
    /// `encrypted(joined) x payout_received / total_joined` — then a
    /// confidential transfer of the resulting handle to the user's payout
    /// account. Permissionless pull — anyone can trigger a user's claim.
    /// Deny records: `(batcher, batch)`, then `(token, payout mint)`.
    pub fn claim<'info>(ctx: Context<'info, Claim<'info>>) -> Result<()> {
        instructions::claim(ctx)
    }

    /// Returns a finished batch's (settled, canceled or refunding) unspent
    /// authority funding to the join mint's wrapper authority, the operator
    /// role that funds batches. Claims and quits pay their own rent, so the
    /// authority needs no lamports after this point.
    pub fn reclaim_batch_authority(ctx: Context<ReclaimBatchAuthority>) -> Result<()> {
        instructions::reclaim_batch_authority(ctx)
    }

    /// Closes the user's spent join record (payout claimed, or batch
    /// canceled), returning its rent to the user. User-signed.
    pub fn close_join_record(ctx: Context<CloseJoinRecord>) -> Result<()> {
        instructions::close_join_record(ctx)
    }
}

#[cfg(test)]
mod pda_vectors {
    include!("../../../test-fixtures/pda/pda_vectors.rs");
}

#[cfg(test)]
mod pda_golden;
