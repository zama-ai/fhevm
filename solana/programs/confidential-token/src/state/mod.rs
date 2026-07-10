//! Account layouts, PDA helpers, and token-domain labels.

pub mod burn_redemption;
pub mod burn_redemption_request;
pub mod confidential_mint;
pub mod confidential_token_account;
pub mod disclosure_request;

pub use burn_redemption::*;
pub use burn_redemption_request::*;
pub use confidential_mint::*;
pub use confidential_token_account::*;
pub use disclosure_request::*;

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

/// Returns the mint-scoped HCU authority PDA — the identity the host's HCU block cap meters
/// (and the admin trusts) for this mint's FHE evals.
pub fn hcu_authority_address(mint: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"hcu-authority", mint.as_ref()], &crate::ID)
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
pub fn balance_encrypted_value_address(
    acl_domain_key: Pubkey,
    app_account: Pubkey,
) -> (Pubkey, u8) {
    encrypted_value_address(acl_domain_key, app_account, balance_label())
}

/// Returns the canonical `EncryptedValue` PDA for the encrypted total supply field.
pub fn total_supply_encrypted_value_address(
    acl_domain_key: Pubkey,
    app_account: Pubkey,
) -> (Pubkey, u8) {
    encrypted_value_address(acl_domain_key, app_account, total_supply_label())
}

/// Returns the canonical `EncryptedValue` PDA for an arbitrary label, delegating
/// key derivation to ZamaHost so app and host agree exactly.
pub fn encrypted_value_address(
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
) -> (Pubkey, u8) {
    zama_host::encrypted_value_address(value_key(
        acl_domain_key,
        app_account,
        encrypted_value_label,
    ))
}

/// Fixed encrypted value label for confidential balances.
pub fn balance_label() -> [u8; 32] {
    *b"balance_________________________"
}

/// Fixed encrypted value label for the encrypted total supply.
pub fn total_supply_label() -> [u8; 32] {
    *b"total_supply____________________"
}

/// Fixed encrypted value label for public wrap amounts.
pub fn wrap_amount_label() -> [u8; 32] {
    *b"wrap_amount_____________________"
}

/// Fixed encrypted value label for externally verified burn amounts.
pub fn burn_amount_label() -> [u8; 32] {
    *b"burn_amount_____________________"
}

/// Fixed encrypted value label for externally verified transfer amounts.
pub fn transfer_amount_label() -> [u8; 32] {
    *b"transfer_amount_________________"
}

/// Fixed encrypted value label for transfer success bits.
pub fn transfer_success_label() -> [u8; 32] {
    *b"transfer_success________________"
}

/// Fixed encrypted value label for unchecked debit candidates.
pub fn debit_candidate_label() -> [u8; 32] {
    *b"debit_candidate_________________"
}

/// Fixed encrypted value label for the all-or-zero burned amount.
pub fn burned_amount_label() -> [u8; 32] {
    *b"burned_amount___________________"
}

/// Fixed encrypted value label for the all-or-zero transferred amount.
pub fn transferred_amount_label() -> [u8; 32] {
    *b"transferred_amount______________"
}

/// Delegates value-key derivation to the shared ACL crate so app and host agree exactly.
pub fn value_key(
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
) -> [u8; 32] {
    zama_solana_acl::derive_value_key(
        acl_domain_key.to_bytes(),
        app_account.to_bytes(),
        encrypted_value_label,
    )
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
        assert_eq!(
            BurnRedemptionRequest::SPACE,
            BurnRedemptionRequest::INIT_SPACE
        );
        assert_eq!(ConfidentialMint::SPACE, ConfidentialMint::INIT_SPACE);
        assert_eq!(
            ConfidentialTokenAccount::SPACE,
            ConfidentialTokenAccount::INIT_SPACE
        );
        assert_eq!(DisclosureRequest::SPACE, DisclosureRequest::INIT_SPACE);
    }
}
