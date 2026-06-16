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

pub use confidential_token_receiver_sdk::{
    transfer_receiver_return_data, TransferReceiverReturn, TRANSFER_RECEIVER_RETURN_FIELD_COUNT,
    TRANSFER_RECEIVER_RETURN_LEN, TRANSFER_RECEIVER_RETURN_MAGIC,
};
/// Re-export constants for generated clients and tests.
pub use constants::*;
/// Re-export errors for generated clients and tests.
pub use errors::*;
/// Re-export events and instruction argument enums for generated clients and tests.
pub use events::*;
#[cfg(feature = "poc")]
pub use instructions::TestReceiverReturnCallback;
use instructions::*;
/// Re-export instruction account contexts for compatibility with existing tests.
pub use instructions::{
    CloseConsumedBurnRedemptionRequest, CloseConsumedDisclosureRequest,
    CloseExpiredBurnRedemptionRequest, CloseExpiredDisclosureRequest, ConfidentialBurn,
    ConfidentialCallTransferReceiver, ConfidentialFinalizeTransferCallback,
    ConfidentialPrepareTransferCallback, ConfidentialTransfer, CreateRandomAmount,
    DiscloseAmountSecp, DiscloseBalanceSecp, InitializeMint, InitializeTokenAccount,
    RedeemBurnedAmountSecp, RequestBurnRedemption, RequestDiscloseAmount, RequestDiscloseBalance,
    WrapUsdc,
};
/// Re-export account layouts and helper functions used by clients and tests.
pub use state::*;

declare_id!("9Sbhx7VF6vAGdYApPikwHgJ68Z367Au2tTyw9FpBxnAh");

/// Anchor entrypoint module for the confidential token PoC.
#[program]
pub mod confidential_token {
    use super::*;

    /// Initializes a confidential mint and records its host ACL domain.
    pub fn initialize_mint(ctx: Context<InitializeMint>) -> Result<()> {
        instructions::initialize_mint(ctx)
    }

    /// Initializes a token account and creates its initial confidential balance handle.
    pub fn initialize_token_account(
        ctx: Context<InitializeTokenAccount>,
        initial_balance: u64,
    ) -> Result<()> {
        instructions::initialize_token_account(ctx, initial_balance)
    }

    /// Creates a token-scoped random encrypted amount for transfer or burn flows.
    pub fn create_random_amount(
        ctx: Context<CreateRandomAmount>,
        amount_kind: ConfidentialAmountKind,
    ) -> Result<()> {
        instructions::create_random_amount(ctx, amount_kind)
    }

    /// Creates a token-scoped bounded random encrypted amount for transfer or burn flows.
    pub fn create_random_bounded_amount(
        ctx: Context<CreateRandomAmount>,
        amount_kind: ConfidentialAmountKind,
        upper_bound: [u8; 32],
    ) -> Result<()> {
        instructions::create_random_bounded_amount(ctx, amount_kind, upper_bound)
    }

    /// Escrows public USDC and rotates the confidential balance by `amount`.
    pub fn wrap_usdc(ctx: Context<WrapUsdc>, amount: u64) -> Result<()> {
        instructions::wrap_usdc(ctx, amount)
    }

    /// Burns an encrypted amount by rotating the account balance and encrypted total supply.
    pub fn confidential_burn(
        ctx: Context<ConfidentialBurn>,
        amount_handle: [u8; 32],
    ) -> Result<()> {
        instructions::confidential_burn(ctx, amount_handle)
    }

    /// Transfers an encrypted amount by rotating the sender and recipient balance handles.
    pub fn confidential_transfer(
        ctx: Context<ConfidentialTransfer>,
        amount_handle: [u8; 32],
    ) -> Result<()> {
        instructions::confidential_transfer(ctx, amount_handle)
    }

    /// Calls an arbitrary receiver hook and verifies its encrypted callback-success result.
    pub fn confidential_call_transfer_receiver<'info>(
        ctx: Context<'info, ConfidentialCallTransferReceiver<'info>>,
        sent_handle: [u8; 32],
        callback_success_handle: [u8; 32],
        receiver_instruction_data: Vec<u8>,
    ) -> Result<()> {
        instructions::confidential_call_transfer_receiver(
            ctx,
            sent_handle,
            callback_success_handle,
            receiver_instruction_data,
        )
    }

    /// Prepares receiver callback settlement by computing the encrypted refund.
    pub fn confidential_prepare_transfer_callback(
        ctx: Context<ConfidentialPrepareTransferCallback>,
        sent_handle: [u8; 32],
        callback_success_handle: [u8; 32],
    ) -> Result<()> {
        instructions::confidential_prepare_transfer_callback(
            ctx,
            sent_handle,
            callback_success_handle,
        )
    }

    /// Finalizes a prepared callback settlement by crediting refund and recording final transfer.
    pub fn confidential_finalize_transfer_callback(
        ctx: Context<ConfidentialFinalizeTransferCallback>,
    ) -> Result<()> {
        instructions::confidential_finalize_transfer_callback(ctx)
    }

    /// Test-only receiver endpoint that returns the supplied callback-success witness.
    #[cfg(feature = "poc")]
    pub fn test_receiver_return_callback(
        ctx: Context<TestReceiverReturnCallback>,
        mint: Pubkey,
        from_token_account: Pubkey,
        to_token_account: Pubkey,
        sent_handle: [u8; 32],
        sent_acl_record: Pubkey,
        callback_success_handle: [u8; 32],
        callback_success_acl_record: Pubkey,
    ) -> Result<()> {
        instructions::test_receiver_return_callback(
            ctx,
            mint,
            from_token_account,
            to_token_account,
            sent_handle,
            sent_acl_record,
            callback_success_handle,
            callback_success_acl_record,
        )
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

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_receiver_return() -> TransferReceiverReturn {
        TransferReceiverReturn {
            mint: Pubkey::new_unique(),
            from_token_account: Pubkey::new_unique(),
            to_token_account: Pubkey::new_unique(),
            sent_handle: [1; 32],
            sent_acl_record: Pubkey::new_unique(),
            callback_success_handle: [2; 32],
            callback_success_acl_record: Pubkey::new_unique(),
        }
    }

    #[test]
    fn transfer_receiver_return_round_trips() {
        let payload = sample_receiver_return();
        let encoded = payload.encode();

        assert_eq!(encoded.len(), TRANSFER_RECEIVER_RETURN_LEN);
        assert_eq!(encoded.len(), TransferReceiverReturn::LEN);
        assert_eq!(TransferReceiverReturn::decode(&encoded).unwrap(), payload);
    }

    #[test]
    fn transfer_receiver_return_compatibility_encoder_matches_struct_encoder() {
        let payload = sample_receiver_return();

        assert_eq!(
            transfer_receiver_return_data(
                payload.mint,
                payload.from_token_account,
                payload.to_token_account,
                payload.sent_handle,
                payload.sent_acl_record,
                payload.callback_success_handle,
                payload.callback_success_acl_record,
            ),
            payload.encode()
        );
    }

    #[test]
    fn transfer_receiver_return_rejects_wrong_magic_or_length() {
        let mut encoded = sample_receiver_return().encode();
        encoded[0] ^= 0xff;
        assert!(TransferReceiverReturn::decode(&encoded).is_err());

        let mut truncated = sample_receiver_return().encode();
        truncated.pop();
        assert!(TransferReceiverReturn::decode(&truncated).is_err());
    }
}
