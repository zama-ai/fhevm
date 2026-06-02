//! Consensus evaluation over a fetched attestation set.
//!
//! Implemented in a later step: validate each attestation's signature via
//! [`ciphertext_attestation::CiphertextAttestation::verify`], filter by signer
//! membership in the [`super::registry::CoprocessorRegistry`], group by the
//! `(keyId, ciphertextDigest, snsCiphertextDigest, format)` tuple, and evaluate
//! the majority threshold — returning the winning group or
//! [`super::error::AttestationError::ConsensusUnreachable`].
