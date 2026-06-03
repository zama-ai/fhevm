//! On-chain account data for `TransientSession`.

use super::*;

/// Host-owned one-shot transient compute capability account.
///
/// This is not durable ACL state and must not be accepted for KMS/public/user
/// decrypt witnesses. It is a short-lived explicit account witness for
/// same-slot compute authorization across instructions or CPIs.
#[account]
pub struct TransientSession {
    /// Caller-chosen session nonce used in the PDA.
    pub session_nonce: [u8; 32],
    /// Authority that may append, seal, consume, and close before expiry.
    pub authority: Pubkey,
    /// Account that receives lamports when the session is closed.
    pub refund_recipient: Pubkey,
    /// Subject allowed by capabilities in this session.
    pub compute_subject: Pubkey,
    /// Slot in which the session was created.
    pub created_slot: u64,
    /// Last slot in which sealed capabilities may be consumed.
    pub expires_slot: u64,
    /// Session state. See `TRANSIENT_SESSION_STATE_*`.
    pub state: u8,
    /// Caller-selected capacity, currently required to be `1`.
    pub max_entries: u8,
    /// Capability entries.
    pub entries: Vec<TransientCapability>,
    /// PDA bump for this session.
    pub bump: u8,
}

impl TransientSession {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32
        + 32
        + 32
        + 32
        + 8
        + 8
        + 1
        + 1
        + 4
        + (MAX_TRANSIENT_CAPABILITIES * TransientCapability::SPACE)
        + 1;
}
