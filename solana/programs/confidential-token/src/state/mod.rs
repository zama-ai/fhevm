//! Account layouts, PDA helpers, and token-domain labels.
//!
//! Public API surface: off-chain callers that have to derive a token PDA or name an encrypted value
//! the same way the program does — `runtime-tests`' Mollusk fixtures, and the demo dapp's TypeScript
//! derivations in `demo-dapp/src/vault/internal/`, which re-declare these labels as byte strings and
//! quote these function names as the source they must match. Exports here are that contract, so a
//! label with no on-chain use is not automatically dead — but one with no use anywhere is: the
//! `transfer_success` and `debit_candidate` labels were deleted once DD-019 stopped creating the
//! scratch PDAs they named.

pub mod burn_redemption;
pub mod confidential_mint;
pub mod confidential_token_account;

pub use burn_redemption::*;
pub use confidential_mint::*;
pub use confidential_token_account::*;

pub use crate::constants::*;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::get_associated_token_address_with_program_id, token as spl_token,
};
use zama_host;

/// Returns the compute signer PDA for a confidential mint.
pub fn compute_signer_address(mint: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"fhe-compute", mint.as_ref()], &crate::ID)
}

/// Returns the mint-scoped app authority PDA for encrypted total supply.
pub fn total_supply_authority_address(mint: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"total-supply", mint.as_ref()], &crate::ID)
}

/// Returns the canonical confidential token account PDA for one owner and mint.
pub fn token_account_address(mint: Pubkey, owner: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"token-account", mint.as_ref(), owner.as_ref()],
        &crate::ID,
    )
}

/// Returns the PDA that owns the confidential mint's underlying-token vault.
pub fn vault_authority_address(mint: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"vault-authority", mint.as_ref()], &crate::ID)
}

/// Returns the canonical SPL token account used as the confidential mint's vault.
pub fn vault_token_account_address(mint: Pubkey, underlying_mint: Pubkey) -> Pubkey {
    get_associated_token_address_with_program_id(
        &vault_authority_address(mint).0,
        &underlying_mint,
        &spl_token::ID,
    )
}

/// Returns the replay-marker PDA for a redeemed burned amount handle.
pub fn burn_redemption_address(mint: Pubkey, burned_handle: [u8; 32]) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"burn-redemption", mint.as_ref(), burned_handle.as_ref()],
        &crate::ID,
    )
}

/// Returns the canonical `EncryptedValue` PDA for a token balance field.
pub fn balance_encrypted_value_address(domain: Pubkey, account: Pubkey) -> (Pubkey, u8) {
    encrypted_value_address(domain, account, encrypted_balance_label())
}

/// Returns the canonical `EncryptedValue` PDA for the encrypted total supply field.
pub fn total_supply_encrypted_value_address(domain: Pubkey, account: Pubkey) -> (Pubkey, u8) {
    encrypted_value_address(domain, account, encrypted_total_supply_label())
}

/// Returns the canonical `EncryptedValue` PDA for an arbitrary label, delegating
/// key derivation to ZamaHost so app and host agree exactly.
pub fn encrypted_value_address(domain: Pubkey, account: Pubkey, label: [u8; 32]) -> (Pubkey, u8) {
    zama_host::encrypted_value_address(encrypted_value_id_bytes(domain, account, label))
}

/// Fixed encrypted value label for confidential balances.
pub fn encrypted_balance_label() -> [u8; 32] {
    *b"balance_________________________"
}

/// Fixed encrypted value label for the encrypted total supply.
pub fn encrypted_total_supply_label() -> [u8; 32] {
    *b"total_supply____________________"
}

/// Fixed encrypted value label for public wrap amounts.
pub fn encrypted_wrap_amount_label() -> [u8; 32] {
    *b"wrap_amount_____________________"
}

/// Fixed encrypted value label for externally verified transfer amounts.
pub fn encrypted_transfer_amount_label() -> [u8; 32] {
    *b"transfer_amount_________________"
}

/// Fixed encrypted value label for the all-or-zero burned amount.
pub fn encrypted_burned_amount_label() -> [u8; 32] {
    *b"burned_amount___________________"
}

/// Fixed encrypted value label for the all-or-zero transferred amount.
pub fn encrypted_transferred_amount_label() -> [u8; 32] {
    *b"transferred_amount______________"
}

/// Delegates encrypted-value-ID derivation to the shared ACL crate so app and host agree exactly.
fn encrypted_value_id_bytes(domain: Pubkey, account: Pubkey, label: [u8; 32]) -> [u8; 32] {
    zama_solana_acl::derive_encrypted_value_id(domain.to_bytes(), account.to_bytes(), label)
}

#[cfg(test)]
mod space_invariants {
    use super::*;

    /// Each manual `SPACE` must equal the `InitSpace`-derived body size, so a
    /// field added to a struct without updating `SPACE` fails fast here instead
    /// of corrupting account layouts in production.
    #[test]
    fn manual_space_matches_derived_init_space() {
        assert_eq!(BurnRedemption::SPACE, BurnRedemption::INIT_SPACE);
        assert_eq!(ConfidentialMint::SPACE, ConfidentialMint::INIT_SPACE);
        assert_eq!(
            ConfidentialTokenAccount::SPACE,
            ConfidentialTokenAccount::INIT_SPACE
        );
    }
}
