//! Per-handle outcome taxonomy for the off-chain ciphertext-attestation verifier.
//!
//! A [`Result<(), AttestationError>`] is the verdict for a single handle:
//! `Ok(())` means the off-chain path fully corroborated the on-chain tuple, so
//! every off-chain value equals the input the caller already holds — there is
//! nothing to echo back. Every other outcome is a classified
//! [`AttestationError`].

use alloy::primitives::B256;

/// Why the off-chain path failed to corroborate a single on-chain handle.
#[derive(Debug, thiserror::Error)]
pub enum AttestationError {
    /// Not enough valid, in-registry signers agreed on a single tuple to meet
    /// the Coprocessor majority threshold.
    #[error("consensus unreachable: {valid_signers} valid signer(s), threshold {threshold}")]
    ConsensusUnreachable {
        valid_signers: usize,
        threshold: usize,
    },

    /// Consensus was reached but the ciphertext could not be downloaded from any
    /// Coprocessor bucket to verify its digest.
    #[error("ciphertext unavailable after {buckets_attempted} bucket attempt(s)")]
    CiphertextUnavailable { buckets_attempted: usize },

    /// The downloaded ciphertext bytes do not hash to the attested digest — an
    /// internally-inconsistent attestation (corruption), not on-chain divergence.
    #[error("ciphertext digest mismatch: attested {attested}, computed {computed}")]
    CiphertextDigestMismatch { attested: B256, computed: B256 },

    /// The off-chain consensus tuple diverges from the on-chain
    /// `SnsCiphertextMaterial` — the single most important signal shadow mode
    /// exists to surface.
    #[error("on-chain tuple mismatch on `{field}`: onchain {onchain}, attested {attested}")]
    OnchainTupleMismatch {
        field: &'static str,
        onchain: String,
        attested: String,
    },
}
