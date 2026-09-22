//! Ciphertext material agreed on by an attestation consensus.

use crate::{CiphertextAttestation, CiphertextFormat};
use alloy_primitives::{B256, U256};

/// The ciphertext material a consensus group agreed on.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConsensusMaterial {
    pub key_id: U256,
    pub ciphertext_digest: B256,
    pub sns_ciphertext_digest: B256,
    pub format: CiphertextFormat,
}

impl From<&CiphertextAttestation> for ConsensusMaterial {
    fn from(att: &CiphertextAttestation) -> Self {
        Self {
            key_id: att.key_id,
            ciphertext_digest: att.ciphertext_digest,
            sns_ciphertext_digest: att.sns_ciphertext_digest,
            format: att.format,
        }
    }
}
