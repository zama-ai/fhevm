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
    AllowTokenAccountSubjects, AllowTotalSupplySubjects, CancelPendingBurn, ConfidentialBurn,
    ConfidentialBurnFromValue, ConfidentialTransfer, ConfidentialTransferFromValue, DiscloseSecp,
    InitializeMint, InitializeTokenAccount, MakeTokenAccountHandlePublic,
    MakeTotalSupplyHandlePublic, RedeemBurnedAmount, RemoveTokenAccountSubject,
    RemoveTotalSupplySubject, WrapUsdc,
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
    /// `allow_subjects`, signing as the token-account PDA
    /// (`EncryptedValue.encrypted_value_account_authority`).
    /// Owner-authorized; auditors remain decrypt-only subjects (fhevm-internal#1862 #13).
    pub fn allow_token_account_subjects<'info>(
        ctx: Context<'info, AllowTokenAccountSubjects<'info>>,
        subjects: Vec<Pubkey>,
    ) -> Result<()> {
        instructions::allow_token_account_subjects(ctx, subjects)
    }

    /// Removes one subject from a token-account-scoped encrypted value by CPI to host
    /// `remove_subject`, signing as the token-account PDA
    /// (`EncryptedValue.encrypted_value_account_authority`).
    pub fn remove_token_account_subject<'info>(
        ctx: Context<'info, RemoveTokenAccountSubject<'info>>,
        subject: Pubkey,
    ) -> Result<()> {
        instructions::remove_token_account_subject(ctx, subject)
    }

    /// Grants decrypt subjects on the encrypted total supply. The existing mint authority
    /// authorizes the operation; the total-supply PDA signs the host CPI.
    pub fn allow_total_supply_subjects<'info>(
        ctx: Context<'info, AllowTotalSupplySubjects<'info>>,
        subjects: Vec<Pubkey>,
    ) -> Result<()> {
        instructions::allow_total_supply_subjects(ctx, subjects)
    }

    /// Removes one decrypt subject from the encrypted total supply. The existing mint authority
    /// authorizes the operation; the total-supply PDA signs the host CPI.
    pub fn remove_total_supply_subject(
        ctx: Context<RemoveTotalSupplySubject>,
        subject: Pubkey,
    ) -> Result<()> {
        instructions::remove_total_supply_subject(ctx, subject)
    }

    /// Seals one token-account state handle publicly. The owner authorizes the request and the
    /// token-account PDA signs as encrypted value account authority.
    pub fn make_token_account_handle_public(
        ctx: Context<MakeTokenAccountHandlePublic>,
        kind: DisclosedValueKind,
        handle: [u8; 32],
    ) -> Result<()> {
        instructions::make_token_account_handle_public(ctx, kind, handle)
    }

    /// Seals encrypted total supply publicly. The mint authority authorizes the request and the
    /// total-supply PDA signs as encrypted value account authority.
    pub fn make_total_supply_handle_public(
        ctx: Context<MakeTotalSupplyHandlePublic>,
        handle: [u8; 32],
    ) -> Result<()> {
        instructions::make_total_supply_handle_public(ctx, handle)
    }

    /// Burns an encrypted amount by updating the account balance and encrypted total supply.
    /// Exactly one burn may be pending for a token account; redeem or cancel it before burning again.
    pub fn confidential_burn<'info>(
        ctx: Context<'info, ConfidentialBurn<'info>>,
        amount_attestation: zama_host::CoprocessorInputAttestation,
    ) -> Result<()> {
        instructions::confidential_burn(ctx, amount_attestation)
    }

    /// Burns an encrypted amount taken from an existing on-chain `EncryptedValue` (a computed or
    /// received handle) instead of a freshly attested client-side encryption — the burn-side analog
    /// of `confidential_transfer_from_value` (fhevm-internal#1755). The batcher uses this to burn an
    /// execution's computed encrypted total, then requests the KMS burn certificate. The signing owner
    /// must be in the amount value's subject set (the token spend gate); the amount is spent
    /// read-only, and the burned-amount output is created publicly decryptable exactly as in
    /// `confidential_burn`, so `redeem_burned_amount` consumes it unchanged.
    pub fn confidential_burn_from_value<'info>(
        ctx: Context<'info, ConfidentialBurnFromValue<'info>>,
    ) -> Result<()> {
        instructions::confidential_burn_from_value(ctx)
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
        kind: DisclosedValueKind,
        handle: [u8; 32],
        cleartext: [u8; 32],
        signatures: Vec<[u8; 65]>,
        extra_data: Vec<u8>,
        proof: zama_host::instructions::MmrInclusionProof,
    ) -> Result<()> {
        instructions::disclose_secp(ctx, kind, handle, cleartext, signatures, extra_data, proof)
    }

    /// Redeems a KMS-certified burned amount from the SPL vault through the stateless host verifier.
    /// Verifies the KMS `PublicDecryptVerification` certificate against the context the cert names
    /// (any live, non-destroyed context, EVM-parity rotation grace) plus an exact-handle MMR
    /// public-decrypt proof, then pays out `cleartext_amount` and closes the `PendingBurn`
    /// account opened at burn time. See `instructions::redeem_burned_amount`.
    pub fn redeem_burned_amount(
        ctx: Context<RedeemBurnedAmount>,
        burned_handle: [u8; 32],
        cleartext_amount: u64,
        signatures: Vec<[u8; 65]>,
        extra_data: Vec<u8>,
        proof: zama_host::instructions::MmrInclusionProof,
    ) -> Result<()> {
        instructions::redeem_burned_amount(
            ctx,
            burned_handle,
            cleartext_amount,
            signatures,
            extra_data,
            proof,
        )
    }

    /// Cancels a pending burn by FHE-crediting the burned amount back onto confidential balance and
    /// encrypted total supply, then closing the `PendingBurn` account.
    pub fn cancel_pending_burn<'info>(ctx: Context<'info, CancelPendingBurn<'info>>) -> Result<()> {
        instructions::cancel_pending_burn(ctx)
    }
}
