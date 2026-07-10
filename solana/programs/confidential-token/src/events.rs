//! App-local events and instruction argument enums for confidential-token.

use anchor_lang::prelude::*;

use crate::state::{burn_amount_label, transfer_amount_label};

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
    /// Receiver callback settlement debited a best-effort refund.
    TransferCallbackRefundDebit,
    /// Receiver callback settlement credited a best-effort refund.
    TransferCallbackRefundCredit,
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
}

/// Token-scoped amount purpose used for amount-handle birth.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfidentialAmountKind {
    /// Amount intended for confidential transfers.
    Transfer,
    /// Amount intended for confidential burns.
    Burn,
}

impl ConfidentialAmountKind {
    // Only reached by the `poc`-gated create_random_amount helpers.
    #[cfg_attr(not(feature = "poc"), allow(dead_code))]
    pub(crate) fn encrypted_value_label(self) -> [u8; 32] {
        match self {
            ConfidentialAmountKind::Transfer => transfer_amount_label(),
            ConfidentialAmountKind::Burn => burn_amount_label(),
        }
    }
}

/// Emitted when the token program creates a token-scoped random amount.
#[event]
pub struct RandomAmountCreatedEvent {
    /// Event schema version.
    pub version: u8,
    /// Confidential mint.
    pub mint: Pubkey,
    /// Token account owner.
    pub owner: Pubkey,
    /// Confidential token account.
    pub token_account: Pubkey,
    /// Whether this amount is intended for transfer or burn.
    pub amount_kind: ConfidentialAmountKind,
    /// Newly created amount handle.
    pub handle: [u8; 32],
    /// ZamaHost ACL record initialized for the amount.
    pub encrypted_value: Pubkey,
}

/// Emitted when the owner requests public disclosure of the current balance.
#[event]
pub struct BalanceDisclosureRequestedEvent {
    /// Event schema version.
    pub version: u8,
    /// Confidential mint.
    pub mint: Pubkey,
    /// Token account owner.
    pub owner: Pubkey,
    /// Confidential token account.
    pub token_account: Pubkey,
    /// Publicly decryptable balance handle.
    pub handle: [u8; 32],
    /// ZamaHost ACL record updated by the request.
    pub encrypted_value: Pubkey,
    /// Account-backed request witness.
    pub request: Pubkey,
    /// Canonical request hash stored in the witness.
    pub request_hash: [u8; 32],
    /// KMS context id the response cert must verify against.
    pub kms_context_id: u64,
    /// Last slot in which this request can be consumed.
    pub expires_slot: u64,
}

/// Emitted when a requester asks to publicly disclose a token-scoped amount.
#[event]
pub struct AmountDisclosureRequestedEvent {
    /// Event schema version.
    pub version: u8,
    /// Confidential mint.
    pub mint: Pubkey,
    /// Requester authorized on the amount ACL.
    pub requester: Pubkey,
    /// Publicly decryptable amount handle.
    pub handle: [u8; 32],
    /// ZamaHost ACL record updated by the request.
    pub encrypted_value: Pubkey,
    /// Account-backed request witness.
    pub request: Pubkey,
    /// Canonical request hash stored in the witness.
    pub request_hash: [u8; 32],
    /// KMS context id the response cert must verify against.
    pub kms_context_id: u64,
    /// Last slot in which this request can be consumed.
    pub expires_slot: u64,
}

/// Emitted when a KMS certificate discloses the current balance cleartext.
#[event]
pub struct BalanceDisclosedEvent {
    /// Event schema version.
    pub version: u8,
    /// Confidential mint.
    pub mint: Pubkey,
    /// Token account owner.
    pub owner: Pubkey,
    /// Confidential token account.
    pub token_account: Pubkey,
    /// Disclosed balance handle.
    pub handle: [u8; 32],
    /// Consumed request witness.
    pub request: Pubkey,
    /// Canonical request hash stored in the witness.
    pub request_hash: [u8; 32],
    /// KMS-certified cleartext amount.
    pub cleartext_amount: u64,
}

/// Emitted when a KMS certificate discloses a token-scoped amount cleartext.
#[event]
pub struct AmountDisclosedEvent {
    /// Event schema version.
    pub version: u8,
    /// Confidential mint.
    pub mint: Pubkey,
    /// Disclosed encrypted amount handle.
    pub handle: [u8; 32],
    /// Consumed request witness.
    pub request: Pubkey,
    /// Canonical request hash stored in the witness.
    pub request_hash: [u8; 32],
    /// KMS-certified cleartext amount.
    pub cleartext_amount: u64,
}

/// Emitted when a holder requests redemption of a burned amount.
#[event]
pub struct BurnRedemptionRequestedEvent {
    /// Event schema version.
    pub version: u8,
    /// Confidential mint.
    pub mint: Pubkey,
    /// Token account owner.
    pub owner: Pubkey,
    /// Confidential token account that produced the burned amount.
    pub token_account: Pubkey,
    /// Burned amount handle.
    pub burned_handle: [u8; 32],
    /// ACL record for `burned_handle`.
    pub burned_encrypted_value: Pubkey,
    /// Underlying token destination owner.
    pub destination_owner: Pubkey,
    /// Underlying token destination account.
    pub destination_account: Pubkey,
    /// Account-backed request witness.
    pub request: Pubkey,
    /// Canonical request hash stored in the witness.
    pub request_hash: [u8; 32],
    /// KMS context id the redemption cert must verify against.
    pub kms_context_id: u64,
    /// Last slot in which this request can be consumed.
    pub expires_slot: u64,
}

/// Emitted when a KMS-certified burned amount is redeemed from the vault.
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
    /// Consumed request witness.
    pub request: Pubkey,
    /// Canonical request hash stored in the witness.
    pub request_hash: [u8; 32],
    /// KMS-certified cleartext amount released from the vault.
    pub cleartext_amount: u64,
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
