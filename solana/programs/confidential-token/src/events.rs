//! App-local events and instruction argument enums for confidential-token.

use anchor_lang::prelude::*;

/// App-local balance history event.
///
/// This event is for frontend/app indexers. The generic coprocessor listener
/// consumes ZamaHost protocol events instead.
#[event]
pub struct BalanceHandleUpdatedEvent {
    /// Event schema version.
    pub version: u8,
    /// Confidential mint.
    pub mint: Pubkey,
    /// Token account owner.
    pub owner: Pubkey,
    /// Confidential token account.
    pub token_account: Pubkey,
    /// Previous balance handle.
    pub old_handle: [u8; 32],
    /// Previous ZamaHost ACL record.
    pub old_encrypted_value: Pubkey,
    /// New balance handle.
    pub new_handle: [u8; 32],
    /// New ZamaHost ACL record.
    pub new_encrypted_value: Pubkey,
    /// Reason this balance pointer changed.
    pub reason: BalanceHandleUpdateReason,
}

/// Reason code for [`BalanceHandleUpdatedEvent`].
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum BalanceHandleUpdateReason {
    /// Initial account creation.
    Initialize,
    /// Public USDC was wrapped into this account.
    Wrap,
    /// Transfer debit from this account.
    TransferDebit,
    /// Transfer credit to this account.
    TransferCredit,
    /// Confidential burn debit from this account.
    BurnDebit,
    /// Pending burn cancelled back onto this account.
    CancelBurn,
}

/// App-local total-supply history event.
///
/// This mirrors ERC7984's encrypted `_totalSupply` pointer at the Solana mint
/// level. The generic coprocessor listener consumes ZamaHost protocol events;
/// this event is for token-aware indexers.
#[event]
pub struct TotalSupplyHandleUpdatedEvent {
    /// Event schema version.
    pub version: u8,
    /// Confidential mint.
    pub mint: Pubkey,
    /// Previous total-supply handle.
    pub old_handle: [u8; 32],
    /// Previous ZamaHost ACL record.
    pub old_encrypted_value: Pubkey,
    /// New total-supply handle.
    pub new_handle: [u8; 32],
    /// New ZamaHost ACL record.
    pub new_encrypted_value: Pubkey,
    /// Reason this total-supply pointer changed.
    pub reason: TotalSupplyUpdateReason,
}

/// Reason code for [`TotalSupplyHandleUpdatedEvent`].
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TotalSupplyUpdateReason {
    /// Initial mint creation.
    Initialize,
    /// Public USDC was wrapped into confidential supply.
    Wrap,
    /// Confidential supply was burned.
    Burn,
    /// Pending burn cancelled back onto confidential supply.
    CancelBurn,
}

/// Emitted when `disclose_secp` publishes a KMS-certified cleartext for a token-scoped handle.
///
/// This is the single app-level disclosure event after the `DisclosureRequest` lifecycle was
/// dissolved (fhevm-internal#1704): it covers both former balance and amount disclosures. It carries
/// no request witness or `request_hash` — there is no per-request PDA anymore. The request side is
/// the owner or mint authority sealing a public-decrypt leaf through the token wrapper that signs
/// the Host `make_handle_public` CPI as the field's encrypted value account authority.
#[event]
pub struct HandleDisclosedEvent {
    /// Event schema version.
    pub version: u8,
    /// Confidential mint whose ACL domain scopes the disclosed encrypted value account.
    pub mint: Pubkey,
    /// Disclosed handle, proven public by the host verifier.
    pub handle: [u8; 32],
    /// ZamaHost `EncryptedValue` encrypted value account the handle belongs to.
    pub encrypted_value: Pubkey,
    /// Token state field whose canonical authority and label were validated.
    pub kind: DisclosedValueKind,
    /// Encrypted value account authority bound to `kind`.
    pub encrypted_value_account_authority: Pubkey,
    /// Encrypted value label bound to `kind`.
    pub encrypted_value_label: [u8; 32],
    /// KMS-certified cleartext amount (low 64 bits of the certified `uint256`).
    pub cleartext_amount: u64,
}

/// Token state field disclosed by [`HandleDisclosedEvent`].
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisclosedValueKind {
    /// Confidential token-account balance.
    Balance,
    /// Amount produced by a confidential transfer.
    TransferredAmount,
    /// Amount produced by a confidential burn.
    BurnedAmount,
    /// Mint encrypted total supply.
    TotalSupply,
}

/// Emitted when a KMS-certified burned amount is redeemed from the vault.
///
/// After the `BurnRedemptionRequest` witness lifecycle was dissolved (fhevm-internal#1763), redeem is
/// a single instruction with no request witness, so this event no longer carries a `request` PDA or
/// `request_hash`.
#[event]
pub struct BurnRedeemedEvent {
    /// Event schema version.
    pub version: u8,
    /// Confidential mint.
    pub mint: Pubkey,
    /// Token account owner.
    pub owner: Pubkey,
    /// Confidential token account that produced the burned amount.
    pub token_account: Pubkey,
    /// Burned amount handle proven by KMS.
    pub burned_handle: [u8; 32],
    /// ACL record for `burned_handle`.
    pub burned_encrypted_value: Pubkey,
    /// Underlying token destination account.
    pub destination_usdc: Pubkey,
    /// KMS-certified cleartext amount released from the vault.
    pub cleartext_amount: u64,
}

/// Emitted when `cancel_pending_burn` re-credits a pending burn into confidential balance.
#[event]
pub struct PendingBurnCancelledEvent {
    /// Event schema version.
    pub version: u8,
    /// Confidential mint.
    pub mint: Pubkey,
    /// Token account owner (rent destination of the closed pending-burn account).
    pub owner: Pubkey,
    /// Confidential token account that produced the burned amount.
    pub token_account: Pubkey,
    /// Burned amount handle that was cancelled (must be the current handle).
    pub burned_handle: [u8; 32],
    /// Shared `burned_amount` EncryptedValue account for the token account.
    pub burned_encrypted_value: Pubkey,
}

/// Emitted when a confidential burn computes the all-or-zero burned amount.
#[event]
pub struct ConfidentialBurnEvent {
    /// Event schema version.
    pub version: u8,
    /// Confidential mint.
    pub mint: Pubkey,
    /// Token account owner.
    pub owner: Pubkey,
    /// Token account whose balance was debited.
    pub token_account: Pubkey,
    /// Encrypted amount actually burned.
    pub burned_handle: [u8; 32],
    /// ZamaHost ACL record for `burned_handle`.
    pub burned_encrypted_value: Pubkey,
}

/// Emitted when a confidential transfer computes the all-or-zero moved amount.
#[event]
pub struct ConfidentialTransferEvent {
    /// Event schema version.
    pub version: u8,
    /// Confidential mint.
    pub mint: Pubkey,
    /// Sender token account owner.
    pub from_owner: Pubkey,
    /// Sender confidential token account.
    pub from_token_account: Pubkey,
    /// Recipient token account owner.
    pub to_owner: Pubkey,
    /// Recipient confidential token account.
    pub to_token_account: Pubkey,
    /// Encrypted amount actually transferred.
    pub transferred_handle: [u8; 32],
    /// ZamaHost ACL record for `transferred_handle`.
    pub transferred_encrypted_value: Pubkey,
}
