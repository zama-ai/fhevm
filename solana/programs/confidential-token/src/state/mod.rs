//! Account layouts, PDA helpers, and token value labels.
//!
//! Public API surface: off-chain callers that have to derive a token PDA or name an encrypted value
//! the same way the program does — `runtime-tests`' Mollusk fixtures, and the demo dapp's TypeScript
//! derivations in `demo-dapp/src/vault/internal/`, which re-declare these labels as byte strings and
//! quote these function names as the source they must match. Exports here are that contract, so a
//! label with no on-chain use is not automatically dead — but one with no use anywhere is: the
//! `transfer_success` and `debit_candidate` labels were deleted once DD-019 stopped creating the
//! transient store PDAs they named.

pub mod confidential_mint;
pub mod confidential_token_account;
pub mod pending_burn;

pub use confidential_mint::*;
pub use confidential_token_account::*;
pub use pending_burn::*;

pub use crate::constants::*;

use anchor_lang::prelude::*;
use anchor_spl::associated_token::get_associated_token_address_with_program_id;
use zama_fhe::{AppScope, StateId};

/// The application one confidential mint is to the host: this program, scoped to the mint. HCU
/// metering, the deny list and every encrypted value's address key on it.
pub fn token_app(mint: Pubkey) -> AppScope {
    AppScope {
        program: crate::ID,
        scope: mint.to_bytes(),
    }
}

/// Returns the mint-scoped encrypted State authority PDA for the encrypted total supply.
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
pub fn token_slot(mint: Pubkey, authority: Pubkey, key: [u8; 32]) -> (StateId, [u8; 32]) {
    (StateId::new(crate::ID, authority, mint.to_bytes()), key)
}

pub fn balance_slot(mint: Pubkey, token_account: Pubkey) -> (StateId, [u8; 32]) {
    token_slot(mint, token_account, balance_key())
}

pub fn total_supply_slot(mint: Pubkey) -> (StateId, [u8; 32]) {
    token_slot(
        mint,
        total_supply_authority_address(mint).0,
        total_supply_key(),
    )
}

pub fn encrypted_state_address(mint: Pubkey, authority: Pubkey) -> (Pubkey, u8) {
    zama_host::encrypted_state_address(crate::ID, authority, mint.to_bytes())
}

/// Fixed encrypted value label for confidential balances.
pub fn balance_key() -> [u8; 32] {
    *b"balance_________________________"
}

/// Fixed encrypted value label for the encrypted total supply.
pub fn total_supply_key() -> [u8; 32] {
    *b"total_supply____________________"
}

/// Fixed encrypted value label for externally verified transfer amounts.
pub fn transfer_input_key() -> [u8; 32] {
    *b"transfer_amount_________________"
}

/// Slot holding the all-or-zero burn result until redemption or cancellation.
pub fn burned_amount_key() -> [u8; 32] {
    *b"burned_amount___________________"
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
