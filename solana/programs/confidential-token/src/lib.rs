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
/// The random-amount PoC helper account context (gated behind `poc`, see below).
#[cfg(feature = "poc")]
pub use instructions::CreateRandomAmount;
use instructions::*;
/// Re-export instruction account contexts for compatibility with existing tests.
pub use instructions::{
    CloseConsumedBurnRedemptionRequest, CloseConsumedDisclosureRequest,
    CloseExpiredBurnRedemptionRequest, CloseExpiredDisclosureRequest, ConfidentialBurn,
    ConfidentialTransfer, DiscloseAmountSecp, DiscloseBalanceSecp, InitializeMint,
    InitializeTokenAccount, RedeemBurnedAmountSecp, RequestBurnRedemption, RequestDiscloseAmount,
    RequestDiscloseBalance, WrapUsdc,
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

    /// Initializes a token account and creates its initial confidential balance handle.
    pub fn initialize_token_account<'info>(
        ctx: Context<'info, InitializeTokenAccount<'info>>,
        initial_balance: u64,
    ) -> Result<()> {
        instructions::initialize_token_account(ctx, initial_balance)
    }

    /// Creates a token-scoped random encrypted amount. Vestigial PoC/demo helper: production
    /// transfers and burns take a coprocessor-attested external amount (fromExternal), not a random
    /// handle. Gated behind the `poc` feature so it never ships in the production IDL.
    #[cfg(feature = "poc")]
    pub fn create_random_amount<'info>(
        ctx: Context<'info, CreateRandomAmount<'info>>,
        amount_kind: ConfidentialAmountKind,
    ) -> Result<()> {
        instructions::create_random_amount(ctx, amount_kind)
    }

    /// Creates a token-scoped bounded random encrypted amount. Vestigial PoC/demo helper gated
    /// behind the `poc` feature with `create_random_amount`.
    #[cfg(feature = "poc")]
    pub fn create_random_bounded_amount<'info>(
        ctx: Context<'info, CreateRandomAmount<'info>>,
        amount_kind: ConfidentialAmountKind,
        upper_bound: [u8; 32],
    ) -> Result<()> {
        instructions::create_random_bounded_amount(ctx, amount_kind, upper_bound)
    }

    /// Escrows public USDC and rotates the confidential balance by `amount`.
    pub fn wrap_usdc<'info>(ctx: Context<'info, WrapUsdc<'info>>, amount: u64) -> Result<()> {
        instructions::wrap_usdc(ctx, amount)
    }

    /// Burns an encrypted amount by rotating the account balance and encrypted total supply.
    pub fn confidential_burn<'info>(
        ctx: Context<'info, ConfidentialBurn<'info>>,
        amount_attestation: zama_host::CoprocessorInputAttestation,
    ) -> Result<()> {
        instructions::confidential_burn(ctx, amount_attestation)
    }

    /// Transfers an encrypted amount by rotating the sender and recipient balance handles.
    pub fn confidential_transfer<'info>(
        ctx: Context<'info, ConfidentialTransfer<'info>>,
        amount_attestation: zama_host::CoprocessorInputAttestation,
    ) -> Result<()> {
        instructions::confidential_transfer(ctx, amount_attestation)
    }

    /// Requests public disclosure for the current confidential balance handle.
    pub fn request_disclose_balance(
        ctx: Context<RequestDiscloseBalance>,
        request_nonce: [u8; 32],
        expires_slot: u64,
    ) -> Result<()> {
        instructions::request_disclose_balance(ctx, request_nonce, expires_slot)
    }

    /// Requests public disclosure for any token-scoped encrypted amount handle.
    pub fn request_disclose_amount(
        ctx: Context<RequestDiscloseAmount>,
        amount_handle: [u8; 32],
        request_nonce: [u8; 32],
        expires_slot: u64,
    ) -> Result<()> {
        instructions::request_disclose_amount(ctx, amount_handle, request_nonce, expires_slot)
    }

    /// Requests KMS certification for redeeming a burned encrypted amount.
    pub fn request_burn_redemption(
        ctx: Context<RequestBurnRedemption>,
        burned_handle: [u8; 32],
        request_nonce: [u8; 32],
        expires_slot: u64,
    ) -> Result<()> {
        instructions::request_burn_redemption(ctx, burned_handle, request_nonce, expires_slot)
    }

    /// Gateway-compatible balance disclosure: verifies the KMS `PublicDecryptVerification`
    /// EIP-712 certificate on-chain via secp256k1_recover against the HostConfig KMS signer.
    pub fn disclose_balance_secp(
        ctx: Context<DiscloseBalanceSecp>,
        cleartext_amount: u64,
        signatures: Vec<[u8; 65]>,
        extra_data: Vec<u8>,
    ) -> Result<()> {
        instructions::disclose_balance_secp(ctx, cleartext_amount, signatures, extra_data)
    }

    /// Gateway-compatible amount disclosure: verifies the KMS `PublicDecryptVerification`
    /// EIP-712 certificate on-chain via secp256k1_recover against the HostConfig KMS signer.
    pub fn disclose_amount_secp(
        ctx: Context<DiscloseAmountSecp>,
        amount_handle: [u8; 32],
        cleartext_amount: u64,
        signatures: Vec<[u8; 65]>,
        extra_data: Vec<u8>,
    ) -> Result<()> {
        instructions::disclose_amount_secp(
            ctx,
            amount_handle,
            cleartext_amount,
            signatures,
            extra_data,
        )
    }

    /// Gateway-compatible redemption: verifies the KMS `PublicDecryptVerification`
    /// EIP-712 certificate on-chain via secp256k1_recover against the active KMS context.
    pub fn redeem_burned_amount_secp(
        ctx: Context<RedeemBurnedAmountSecp>,
        burned_handle: [u8; 32],
        cleartext_amount: u64,
        signatures: Vec<[u8; 65]>,
        extra_data: Vec<u8>,
    ) -> Result<()> {
        instructions::redeem_burned_amount_secp(
            ctx,
            burned_handle,
            cleartext_amount,
            signatures,
            extra_data,
        )
    }

    /// Closes a consumed disclosure request witness.
    pub fn close_consumed_disclosure_request(
        ctx: Context<CloseConsumedDisclosureRequest>,
    ) -> Result<()> {
        instructions::close_consumed_disclosure_request(ctx)
    }

    /// Closes a consumed burn-redemption request witness.
    pub fn close_consumed_burn_redemption_request(
        ctx: Context<CloseConsumedBurnRedemptionRequest>,
    ) -> Result<()> {
        instructions::close_consumed_burn_redemption_request(ctx)
    }

    /// Closes an expired, unconsumed disclosure request witness.
    pub fn close_expired_disclosure_request(
        ctx: Context<CloseExpiredDisclosureRequest>,
    ) -> Result<()> {
        instructions::close_expired_disclosure_request(ctx)
    }

    /// Closes an expired, unconsumed burn-redemption request witness.
    pub fn close_expired_burn_redemption_request(
        ctx: Context<CloseExpiredBurnRedemptionRequest>,
    ) -> Result<()> {
        instructions::close_expired_burn_redemption_request(ctx)
    }
}
