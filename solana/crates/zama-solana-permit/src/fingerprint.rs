//! Transport-key fingerprint.
//!
//! The canonical text commits to a 32-byte SHAKE-256 digest of the transport key
//! rather than to the key itself, because a full key does not fit a hardware
//! wallet's clear-signing budget. The digest is plain SHAKE-256 with no
//! domain-separation tag: it is a wallet-surface object, and only collision
//! resistance is required of it.
//!
//! The fingerprint is always recomputed from the full key that traveled as a typed
//! field. It is never accepted as an input — a verifier that trusted a supplied
//! fingerprint would let an attacker attach their own transport key to somebody
//! else's signed permit and have the result sealed to them.

use sha3::{
    digest::{ExtendableOutput, Update, XofReader},
    Shake256,
};

use crate::types::TransportKey;

/// Length of the fingerprint the canonical text carries.
pub const FINGERPRINT_LEN: usize = 32;

/// Recomputes the fingerprint the canonical text commits to.
pub fn transport_key_fingerprint(key: &TransportKey) -> [u8; FINGERPRINT_LEN] {
    let mut hasher = Shake256::default();
    // The whole key, and nothing else: no domain-separation tag and no length prefix, so
    // any implementation with a SHAKE-256 primitive reproduces this from the key alone.
    hasher.update(key.as_bytes());
    let mut fingerprint = [0u8; FINGERPRINT_LEN];
    hasher.finalize_xof().read(&mut fingerprint);
    fingerprint
}
