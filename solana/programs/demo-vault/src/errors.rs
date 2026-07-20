//! Program-specific errors returned by demo-vault instructions.

use anchor_lang::prelude::*;

/// Errors returned by the demo-vault PoC.
#[error_code]
pub enum DemoVaultError {
    /// A deposit, withdraw, or harvest was called with a zero amount.
    #[msg("amount must be greater than zero")]
    ZeroAmount,
    /// A deposit rounded down to zero shares (deposit too small for the current price).
    #[msg("deposit is too small to mint any shares")]
    ZeroShares,
    /// A withdraw rounded down to zero underlying assets (redeem too small for the current price).
    #[msg("redeem is too small to return any assets")]
    ZeroAssets,
    /// A withdraw requested more shares than the owner holds.
    #[msg("insufficient share balance for this withdraw")]
    InsufficientShares,
    /// A share-price computation overflowed the u128 intermediate or the u64 result.
    #[msg("share-price arithmetic overflowed")]
    MathOverflow,
    /// The supplied underlying mint did not match the vault's recorded underlying mint.
    #[msg("underlying mint does not match the vault")]
    UnderlyingMintMismatch,
    /// The supplied share mint did not match the vault's recorded share mint.
    #[msg("share mint does not match the vault")]
    ShareMintMismatch,
    /// The supplied vault token account did not match the vault's recorded token account.
    #[msg("vault token account does not match the vault")]
    VaultTokenAccountMismatch,
    /// A user token account was not owned by the expected signer.
    #[msg("token account owner does not match signer")]
    OwnerMismatch,
    /// A user token account was for the wrong mint.
    #[msg("token account mint does not match")]
    MintMismatch,
}
