//! On-chain account data for a KMS context.
//!
//! Solana mirror of `host-contracts` `ProtocolConfig` KMS contexts: a versioned,
//! id'd set of KMS node signer addresses plus per-operation thresholds, synced from
//! the canonical gateway (`KMSVerifier.NewContextSet` -> `ProtocolConfig.NewKmsContext`).
//! On-chain we keep only what cert verification needs — the EVM `signerAddress` of
//! each node and the four `KmsThresholds` — not the off-chain ip/storage metadata.

use super::*;

/// Per-operation signature thresholds for a KMS context (mirrors `KmsThresholds`).
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct KmsThresholds {
    /// Threshold for public-decrypt (disclose/redeem) certificates.
    pub public_decryption: u8,
    /// Threshold for user-decrypt certificates.
    pub user_decryption: u8,
    /// Threshold for key generation (KMS-internal; stored for fidelity).
    pub kms_gen: u8,
    /// Threshold for MPC (KMS-internal; stored for fidelity).
    pub mpc: u8,
}

/// A KMS context: the signer set + thresholds active under a `context_id`.
#[account]
pub struct KmsContext {
    /// Monotonic context id (mirrors `kmsContextId`).
    pub context_id: u64,
    /// KMS node signer EVM addresses authorized to sign certs in this context.
    pub signers: Vec<[u8; 20]>,
    /// Per-operation signature thresholds.
    pub thresholds: KmsThresholds,
    /// True once destroyed (mirrors `destroyKmsContext`); destroyed contexts reject.
    pub destroyed: bool,
    /// PDA bump for `PDA("kms-context", context_id)`.
    pub bump: u8,
}

impl KmsContext {
    /// Upper bound on KMS nodes per context (bounds account size).
    pub const MAX_SIGNERS: usize = 16;
    pub const SPACE: usize = 8 + (4 + Self::MAX_SIGNERS * 20) + 4 + 1 + 1;
}
