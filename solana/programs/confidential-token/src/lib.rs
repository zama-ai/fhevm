//! Confidential token app used by the Solana FHEVM PoC.
//!
//! This program demonstrates how an app can keep token-specific semantics locally while
//! delegating FHE handle creation, compute ACL checks, and protocol event emission to
//! `zama-host`. The crate root mirrors `zama-host`: account state, events, errors,
//! and instruction handlers live in focused modules, while the Anchor entrypoint
//! module only delegates into `instructions`.

// Anchor macros generate framework-shaped code that trips rustc/Clippy checks.
#![allow(unexpected_cfgs)]
#![allow(clippy::diverging_sub_expression, clippy::too_many_arguments)]

/// Shared constants, seed bytes, and fixed protocol sizes.
pub mod constants;
/// Program-specific errors returned by confidential-token instructions.
pub mod errors;
/// App-local events and instruction argument enums.
pub mod events;
mod fhe;
/// Instruction account contexts and handlers.
pub mod instructions;
/// Account layouts, PDA helpers, and token-domain labels.
pub mod state;

use anchor_lang::prelude::*;

/// Re-export constants for generated clients and tests.
pub use constants::*;
/// Re-export errors for generated clients and tests.
pub use errors::*;
/// Re-export events and instruction argument enums for generated clients and tests.
pub use events::*;
use instructions::*;
/// Re-export instruction account contexts for compatibility with existing tests.
pub use instructions::{
    AllowTokenAccountSubjects, ConfidentialBurn, ConfidentialBurnFromValue, ConfidentialTransfer,
    ConfidentialTransferFromValue, DiscloseSecp, InitializeMint, InitializeTokenAccount,
    RecoverPendingBurn, RedeemBurnedAmount, RemoveTokenAccountSubject, WrapUsdc,
};
/// Re-export account layouts and helper functions used by clients and tests.
pub use state::*;

declare_id!("pS2gMMq6PNZKpjxiANeoN5XxJgwaFsUR6xaJkpUHcDg");

/// Anchor entrypoint module for the confidential token PoC.
#[program]
pub mod confidential_token {
    use super::*;

    /// Initializes a confidential mint and records its host ACL domain.
    pub fn initialize_mint<'info>(ctx: Context<'info, InitializeMint<'info>>) -> Result<()> {
        instructions::initialize_mint(ctx)
    }

    /// Initializes a token account and creates its zero confidential balance handle.
    pub fn initialize_token_account<'info>(
        ctx: Context<'info, InitializeTokenAccount<'info>>,
    ) -> Result<()> {
        instructions::initialize_token_account(ctx)
    }

    /// Escrows public USDC and updates the confidential balance by `amount`.
    pub fn wrap_usdc<'info>(ctx: Context<'info, WrapUsdc<'info>>, amount: u64) -> Result<()> {
        instructions::wrap_usdc(ctx, amount)
    }

    /// Grants subjects on a token-account-scoped encrypted value by CPI to host
    /// `allow_subjects`, signing as the token-account PDA (`EncryptedValue.account`).
    /// Owner-authorized; auditors remain decrypt-only subjects (fhevm-internal#1862 #13).
    pub fn allow_token_account_subjects<'info>(
        ctx: Context<'info, AllowTokenAccountSubjects<'info>>,
        subjects: Vec<Pubkey>,
    ) -> Result<()> {
        instructions::allow_token_account_subjects(ctx, subjects)
    }

    /// Removes one subject from a token-account-scoped encrypted value by CPI to host
    /// `remove_subject`, signing as the token-account PDA (`EncryptedValue.account`).
    pub fn remove_token_account_subject<'info>(
        ctx: Context<'info, RemoveTokenAccountSubject<'info>>,
        subject: Pubkey,
    ) -> Result<()> {
        instructions::remove_token_account_subject(ctx, subject)
    }

    /// Burns an encrypted amount by updating the account balance and encrypted total supply.
    /// `burn_id` is a client-supplied unique id known before the tx; it seeds the `PendingBurn` lane
    /// PDA so the account can appear in metas before `fhe_execute` derives `burned_handle`.
    pub fn confidential_burn<'info>(
        ctx: Context<'info, ConfidentialBurn<'info>>,
        amount_attestation: zama_host::CoprocessorInputAttestation,
        burn_id: [u8; 32],
    ) -> Result<()> {
        instructions::confidential_burn(ctx, amount_attestation, burn_id)
    }

    /// Burns an encrypted amount taken from an existing on-chain `EncryptedValue` (a computed or
    /// received handle) instead of a freshly attested client-side encryption — the burn-side analog
    /// of `confidential_transfer_from_value` (fhevm-internal#1755). The batcher uses this to burn a
    /// execution's computed encrypted total, then requests the KMS burn certificate. The signing owner
    /// must be in the amount value's subject set (the token spend gate); the amount is spent
    /// read-only, and the burned-amount output is created publicly decryptable exactly as in
    /// `confidential_burn`, so `redeem_burned_amount` consumes it unchanged. `burn_id` seeds the
    /// pending-burn lane (batcher passes `batch.key().to_bytes()`).
    pub fn confidential_burn_from_value<'info>(
        ctx: Context<'info, ConfidentialBurnFromValue<'info>>,
        burn_id: [u8; 32],
    ) -> Result<()> {
        instructions::confidential_burn_from_value(ctx, burn_id)
    }

    /// Transfers an encrypted amount by updating the sender and recipient balance handles.
    pub fn confidential_transfer<'info>(
        ctx: Context<'info, ConfidentialTransfer<'info>>,
        amount_attestation: zama_host::CoprocessorInputAttestation,
    ) -> Result<()> {
        instructions::confidential_transfer(ctx, amount_attestation)
    }

    /// Transfers an encrypted amount taken from an existing on-chain `EncryptedValue` (a computed or
    /// received handle) instead of a freshly attested client-side encryption — the path that lets a
    /// contract be the sender of a computed amount (fhevm-internal#1680). The signing owner must be
    /// in the amount value's subject set (the token spend gate); the amount is spent read-only.
    pub fn confidential_transfer_from_value<'info>(
        ctx: Context<'info, ConfidentialTransferFromValue<'info>>,
    ) -> Result<()> {
        instructions::confidential_transfer_from_value(ctx)
    }

    /// Consumes a KMS public-decrypt certificate through the stateless host verifier and emits a
    /// token-scoped disclosed event. See `instructions::disclose_secp` for the act-once semantics
    /// (idempotent by design — no on-chain replay marker).
    pub fn disclose_secp(
        ctx: Context<DiscloseSecp>,
        handle: [u8; 32],
        cleartext: [u8; 32],
        signatures: Vec<[u8; 65]>,
        extra_data: Vec<u8>,
        proof: zama_host::instructions::MmrInclusionProof,
    ) -> Result<()> {
        instructions::disclose_secp(ctx, handle, cleartext, signatures, extra_data, proof)
    }

    /// Redeems a KMS-certified burned amount from the SPL vault through the stateless host verifier.
    /// Verifies the KMS `PublicDecryptVerification` certificate against the context the cert names
    /// (any live, non-destroyed context, EVM-parity rotation grace) plus an exact-handle MMR
    /// public-decrypt proof, then pays out `cleartext_amount` and closes the per-`burn_id`
    /// `PendingBurn` lane opened at burn time. See `instructions::redeem_burned_amount`.
    pub fn redeem_burned_amount(
        ctx: Context<RedeemBurnedAmount>,
        burn_id: [u8; 32],
        burned_handle: [u8; 32],
        cleartext_amount: u64,
        signatures: Vec<[u8; 65]>,
        extra_data: Vec<u8>,
        proof: zama_host::instructions::MmrInclusionProof,
    ) -> Result<()> {
        instructions::redeem_burned_amount(
            ctx,
            burn_id,
            burned_handle,
            cleartext_amount,
            signatures,
            extra_data,
            proof,
        )
    }

    /// Recovers a tip-only pending burn by FHE-crediting the burned amount back onto confidential
    /// balance and encrypted total supply, then closing the `PendingBurn` lane. Requires the burned
    /// handle to still be the shared `burned_amount` EncryptedValue's `current_handle`; superseded
    /// burns must claim via `redeem_burned_amount` instead. See `instructions::recover_pending_burn`.
    pub fn recover_pending_burn<'info>(
        ctx: Context<'info, RecoverPendingBurn<'info>>,
    ) -> Result<()> {
        instructions::recover_pending_burn(ctx)
    }
}
