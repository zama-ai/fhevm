//! Account layouts, PDA helpers, and token value labels.
//!
//! Public API surface: off-chain callers that have to derive a token PDA or name an encrypted value
//! the same way the program does — `runtime-tests`' Mollusk fixtures, and the demo dapp's TypeScript
//! derivations in `demo-dapp/src/vault/internal/`, which re-declare these labels as byte strings and
//! quote these function names as the source they must match. Exports here are that contract, so a
//! label with no on-chain use is not automatically dead — but one with no use anywhere is: the
//! `transfer_success` and `debit_candidate` labels were deleted once DD-019 stopped creating the
//! scratch PDAs they named.

pub mod confidential_mint;
pub mod confidential_token_account;
pub mod pending_burn;

pub use confidential_mint::*;
pub use confidential_token_account::*;
pub use pending_burn::*;

pub use crate::constants::*;

use anchor_lang::prelude::*;
use anchor_spl::associated_token::get_associated_token_address_with_program_id;
use zama_fhe::{AppScope, EncryptedValueId, EncryptedValueLabel};

/// The application one confidential mint is to the host: this program, scoped to the mint. HCU
/// metering, the deny list and every encrypted value's address key on it.
pub fn token_app(mint: Pubkey) -> AppScope {
    AppScope {
        program: crate::ID,
        scope: mint.to_bytes(),
    }
}

/// Returns the mint-scoped encrypted value account authority PDA for the encrypted total supply.
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

/// Returns the canonical token account used as the confidential mint's vault.
pub fn vault_token_account_address(
    mint: Pubkey,
    underlying_mint: Pubkey,
    token_program: Pubkey,
) -> Pubkey {
    get_associated_token_address_with_program_id(
        &vault_authority_address(mint).0,
        &underlying_mint,
        &token_program,
    )
}

/// Returns the single pending-burn PDA for a confidential token account.
pub fn pending_burn_address(mint: Pubkey, token_account: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[PENDING_BURN_SEED, mint.as_ref(), token_account.as_ref()],
        &crate::ID,
    )
}

/// The id of a token value: the mint's application, the controlling PDA, and the field label.
pub fn token_value_id(mint: Pubkey, authority: Pubkey, label: [u8; 32]) -> EncryptedValueId {
    EncryptedValueId::new(token_app(mint), authority, EncryptedValueLabel::new(label))
}

/// The id of a token account's balance, controlled by the token account PDA.
pub fn balance_encrypted_value_id(mint: Pubkey, token_account: Pubkey) -> EncryptedValueId {
    token_value_id(mint, token_account, encrypted_balance_label())
}

/// The id of a mint's encrypted total supply, controlled by the `total-supply` PDA.
pub fn total_supply_encrypted_value_id(mint: Pubkey) -> EncryptedValueId {
    token_value_id(
        mint,
        total_supply_authority_address(mint).0,
        encrypted_total_supply_label(),
    )
}

/// Returns the canonical `EncryptedValue` PDA for a token value, delegating key derivation to
/// ZamaHost so app and host agree exactly.
pub fn encrypted_value_address(mint: Pubkey, authority: Pubkey, label: [u8; 32]) -> (Pubkey, u8) {
    token_value_id(mint, authority, label).address_with_bump()
}

/// Fixed encrypted value label for confidential balances.
pub fn encrypted_balance_label() -> [u8; 32] {
    *b"balance_________________________"
}

/// Fixed encrypted value label for the encrypted total supply.
pub fn encrypted_total_supply_label() -> [u8; 32] {
    *b"total_supply____________________"
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

#[cfg(test)]
mod space_invariants {
    use super::*;

    /// Each manual `SPACE` must equal the `InitSpace`-derived body size, so a
    /// field added to a struct without updating `SPACE` fails fast here instead
    /// of corrupting account layouts in production.
    #[test]
    fn manual_space_matches_derived_init_space() {
        assert_eq!(PendingBurn::SPACE, PendingBurn::INIT_SPACE);
        assert_eq!(ConfidentialMint::SPACE, ConfidentialMint::INIT_SPACE);
        assert_eq!(
            ConfidentialTokenAccount::SPACE,
            ConfidentialTokenAccount::INIT_SPACE
        );
    }
}
