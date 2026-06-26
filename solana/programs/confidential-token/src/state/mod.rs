//! Account layouts, PDA helpers, and token-domain labels.

pub mod burn_redemption;
pub mod burn_redemption_request;
pub mod confidential_mint;
pub mod confidential_token_account;
pub mod disclosure_request;
pub mod transfer_callback_settlement;
pub mod transfer_receiver_hook_call;

pub use burn_redemption::*;
pub use burn_redemption_request::*;
pub use confidential_mint::*;
pub use confidential_token_account::*;
pub use disclosure_request::*;
pub use transfer_callback_settlement::*;
pub use transfer_receiver_hook_call::*;

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

/// Returns the replay-marker PDA for a transfer callback settlement.
pub fn transfer_callback_settlement_address(mint: Pubkey, sent_handle: [u8; 32]) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"transfer-callback", mint.as_ref(), sent_handle.as_ref()],
        &crate::ID,
    )
}

/// Returns the one-shot marker PDA for a transfer receiver hook call.
pub fn transfer_receiver_hook_address(mint: Pubkey, sent_handle: [u8; 32]) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"transfer-hook", mint.as_ref(), sent_handle.as_ref()],
        &crate::ID,
    )
}

/// Returns the ZamaHost nonce key for a token balance field.
pub fn balance_nonce_key(acl_domain_key: Pubkey, app_account: Pubkey) -> [u8; 32] {
    nonce_key(acl_domain_key, app_account, balance_label())
}

/// Returns the ZamaHost nonce key for the encrypted total supply field.
pub fn total_supply_nonce_key(acl_domain_key: Pubkey, app_account: Pubkey) -> [u8; 32] {
    nonce_key(acl_domain_key, app_account, total_supply_label())
}

/// Returns the encrypted-value ACL lineage address for a token balance.
///
/// One lineage account per `(mint, token_account)` balance, reused across every
/// balance-handle rotation (encrypted-value ACL + MMR PoC, fhevm-internal#1569).
pub fn balance_value_acl_address(acl_domain_key: Pubkey, app_account: Pubkey) -> (Pubkey, u8) {
    zama_host::encrypted_value_acl_address(balance_nonce_key(acl_domain_key, app_account))
}

/// Returns the encrypted-value ACL lineage address for the encrypted total supply.
///
/// One lineage account per `(mint, total_supply_authority)`, reused across every
/// total-supply rotation (encrypted-value ACL + MMR PoC, fhevm-internal#1569).
pub fn total_supply_value_acl_address(acl_domain_key: Pubkey, app_account: Pubkey) -> (Pubkey, u8) {
    zama_host::encrypted_value_acl_address(total_supply_nonce_key(acl_domain_key, app_account))
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

/// Fixed encrypted value label for burn success bits.
pub fn burn_success_label() -> [u8; 32] {
    *b"burn_success____________________"
}

/// Fixed encrypted value label for transfer success bits.
pub fn transfer_success_label() -> [u8; 32] {
    *b"transfer_success________________"
}

/// Fixed encrypted value label for unchecked burn debit candidates.
pub fn burn_debit_candidate_label() -> [u8; 32] {
    *b"burn_debit_candidate____________"
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

/// Fixed encrypted value label for receiver callback success bits.
pub fn callback_success_label() -> [u8; 32] {
    *b"callback_success________________"
}

/// Fixed encrypted value label for callback-settlement zero constants.
pub fn callback_zero_label() -> [u8; 32] {
    *b"callback_zero___________________"
}

/// Fixed encrypted value label for callback refund balance checks.
pub fn callback_refund_success_label() -> [u8; 32] {
    *b"callback_refund_success_________"
}

/// Fixed encrypted value label for callback refund debit candidates.
pub fn callback_refund_debit_candidate_label() -> [u8; 32] {
    *b"callback_refund_debit_candidate_"
}

/// Fixed encrypted value label for callback actual refunds.
pub fn callback_refund_amount_label() -> [u8; 32] {
    *b"callback_refund_amount__________"
}

/// Fixed encrypted value label for final transfer amounts after callback refunds.
pub fn callback_final_transferred_label() -> [u8; 32] {
    *b"callback_final_transferred______"
}

/// Delegates nonce-key derivation to ZamaHost so app and host agree exactly.
pub fn nonce_key(
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
) -> [u8; 32] {
    zama_host::acl_nonce_key(acl_domain_key, app_account, encrypted_value_label)
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
        assert_eq!(
            TransferCallbackSettlement::SPACE,
            TransferCallbackSettlement::INIT_SPACE
        );
        assert_eq!(
            TransferReceiverHookCall::SPACE,
            TransferReceiverHookCall::INIT_SPACE
        );
    }
}
