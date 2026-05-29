//! Account layouts, constants, PDA helpers, and token-domain labels.

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::get_associated_token_address_with_program_id, token as spl_token,
};
use zama_host;

pub(crate) const BALANCE_FHE_TYPE: u8 = 5;
pub(crate) const APP_EVENT_VERSION: u8 = 0;
pub const CALLBACK_SETTLEMENT_PREPARED: u8 = 1;
pub const CALLBACK_SETTLEMENT_FINALIZED: u8 = 2;
pub(crate) const DISCLOSURE_PROOF_DOMAIN_SEPARATOR: &[u8] =
    b"zama-confidential-token-disclosure-v1";
pub(crate) const ED25519_SIGNATURE_OFFSETS_SERIALIZED_SIZE: usize = 14;
pub(crate) const ED25519_SIGNATURE_OFFSETS_START: usize = 2;
pub(crate) const ED25519_PUBKEY_SERIALIZED_SIZE: usize = 32;
pub(crate) const ED25519_SIGNATURE_SERIALIZED_SIZE: usize = 64;
pub(crate) const ED25519_PROGRAM_ID: Pubkey =
    anchor_lang::pubkey!("Ed25519SigVerify111111111111111111111111111");

/// Confidential mint state for the token PoC.
#[account]
pub struct ConfidentialMint {
    /// Admin/authority that created the mint.
    pub authority: Pubkey,
    /// ACL domain key, currently equal to the mint pubkey.
    pub acl_domain_key: Pubkey,
    /// Program-controlled compute signer PDA.
    pub compute_signer: Pubkey,
    /// Underlying SPL mint wrapped by this confidential mint.
    pub underlying_mint: Pubkey,
    /// Ed25519 authority accepted for KMS disclosure response certificates.
    pub kms_verifier_authority: Pubkey,
    /// Decimal precision inherited from the underlying mint.
    pub decimals: u8,
    /// Current encrypted total-supply handle.
    pub total_supply_handle: [u8; 32],
    /// Current ZamaHost ACL record for `total_supply_handle`.
    pub total_supply_acl_record: Pubkey,
    /// Next nonce sequence to use for a total-supply ACL record.
    pub next_total_supply_nonce_sequence: u64,
}

impl ConfidentialMint {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32 + 32 + 32 + 32 + 32 + 1 + 32 + 32 + 8;
}

/// Confidential token account state.
#[account]
pub struct ConfidentialTokenAccount {
    /// Token account owner.
    pub owner: Pubkey,
    /// Confidential mint this account belongs to.
    pub mint: Pubkey,
    /// Current confidential balance handle.
    pub balance_handle: [u8; 32],
    /// Current ZamaHost ACL record for `balance_handle`.
    pub balance_acl_record: Pubkey,
    /// Next nonce sequence to use for a balance ACL record.
    pub next_balance_nonce_sequence: u64,
    /// Next nonce sequence to use for owner-scoped random amount ACL records.
    pub next_amount_nonce_sequence: u64,
    /// PDA bump for the token account.
    pub bump: u8,
}

/// Operator authorization for one confidential token account.
#[account]
pub struct ConfidentialOperator {
    /// Token account whose balance may be transferred by the operator.
    pub token_account: Pubkey,
    /// Token account owner that created the authorization.
    pub owner: Pubkey,
    /// Operator signer allowed until `expiration_slot`.
    pub operator: Pubkey,
    /// Last slot in which the operator remains active. Zero revokes the row.
    pub expiration_slot: u64,
    /// PDA bump for `(token_account, operator)`.
    pub bump: u8,
}

impl ConfidentialOperator {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32 + 32 + 32 + 8 + 1;
}

/// Replay marker for a redeemed burned amount handle.
#[account]
pub struct BurnRedemption {
    /// Confidential mint whose vault paid the redemption.
    pub mint: Pubkey,
    /// Token owner that redeemed the burned amount.
    pub owner: Pubkey,
    /// Token account that produced the burned amount.
    pub token_account: Pubkey,
    /// Burned amount handle proven by KMS.
    pub burned_handle: [u8; 32],
    /// ACL record for `burned_handle`.
    pub burned_acl_record: Pubkey,
    /// KMS-certified cleartext amount released from the vault.
    pub cleartext_amount: u64,
    /// PDA bump for `(mint, burned_handle)`.
    pub bump: u8,
}

impl BurnRedemption {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32 + 32 + 32 + 32 + 32 + 8 + 1;
}

/// One-shot marker for a receiver hook call.
#[account]
pub struct TransferReceiverHookCall {
    /// Confidential mint whose transfer reached the receiver hook.
    pub mint: Pubkey,
    /// Original sender token account.
    pub from_token_account: Pubkey,
    /// Original recipient token account.
    pub to_token_account: Pubkey,
    /// Prior transfer's encrypted all-or-zero sent handle.
    pub sent_handle: [u8; 32],
    /// ACL record for `sent_handle`.
    pub sent_acl_record: Pubkey,
    /// Encrypted receiver callback success bit.
    pub callback_success_handle: [u8; 32],
    /// ACL record for `callback_success_handle`.
    pub callback_success_acl_record: Pubkey,
    /// Receiver hook program that returned the callback witness.
    pub receiver_program: Pubkey,
    /// Sender or active operator that invoked the hook.
    pub caller: Pubkey,
    /// PDA bump for `(mint, sent_handle)`.
    pub bump: u8,
}

impl TransferReceiverHookCall {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = (32 * 9) + 1;
}

/// Replay marker for a transfer callback settlement.
#[account]
pub struct TransferCallbackSettlement {
    /// Confidential mint whose transfer was settled.
    pub mint: Pubkey,
    /// Original sender owner.
    pub from_owner: Pubkey,
    /// Original sender token account.
    pub from_token_account: Pubkey,
    /// Original recipient owner.
    pub to_owner: Pubkey,
    /// Original recipient token account.
    pub to_token_account: Pubkey,
    /// Prior transfer's encrypted all-or-zero sent handle.
    pub sent_handle: [u8; 32],
    /// ACL record for `sent_handle`.
    pub sent_acl_record: Pubkey,
    /// Encrypted receiver callback success bit.
    pub callback_success_handle: [u8; 32],
    /// ACL record for `callback_success_handle`.
    pub callback_success_acl_record: Pubkey,
    /// Encrypted refund requested by the callback result.
    pub requested_refund_handle: [u8; 32],
    /// ACL record for `requested_refund_handle`.
    pub requested_refund_acl_record: Pubkey,
    /// Encrypted amount actually refunded.
    pub refund_handle: [u8; 32],
    /// ACL record for `refund_handle`.
    pub refund_acl_record: Pubkey,
    /// Recipient balance handle after the prepared refund debit.
    pub to_balance_handle: [u8; 32],
    /// ACL record for `to_balance_handle`.
    pub to_balance_acl_record: Pubkey,
    /// Sender balance handle after the finalized refund credit.
    pub from_balance_handle: [u8; 32],
    /// ACL record for `from_balance_handle`.
    pub from_balance_acl_record: Pubkey,
    /// Encrypted amount that remains transferred after refund.
    pub transferred_handle: [u8; 32],
    /// ACL record for `transferred_handle`.
    pub transferred_acl_record: Pubkey,
    /// Settlement lifecycle status.
    pub status: u8,
    /// PDA bump for `(mint, sent_handle)`.
    pub bump: u8,
}

impl TransferCallbackSettlement {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = (32 * 19) + 2;
}

impl ConfidentialTokenAccount {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32 + 32 + 32 + 32 + 8 + 8 + 1;
}

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

/// Returns the operator authorization PDA for one token account and operator.
pub fn operator_record_address(token_account: Pubkey, operator: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"operator", token_account.as_ref(), operator.as_ref()],
        &crate::ID,
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

/// Fixed encrypted value label for callback-requested refunds.
pub fn callback_refund_request_label() -> [u8; 32] {
    *b"callback_refund_request_________"
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
