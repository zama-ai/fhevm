//! Ciphertext-specific consensus: what a round is about and what the Coprocessors agree on.

use crate::{CiphertextAttestation, CiphertextFormat, consensus::ConsensusRound};
use alloy_primitives::{B256, U256};

/// A handle in a given Coprocessor context: the subject of a ciphertext consensus round.
///
/// Both fields are bound by the signature but absent from the wire form. The verifier rebuilds
/// them from the S3 lookup path.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CiphertextRef {
    pub handle: B256,
    pub coprocessor_context_id: U256,
}

impl CiphertextRef {
    pub fn new(handle: B256, coprocessor_context_id: U256) -> Self {
        Self {
            handle,
            coprocessor_context_id,
        }
    }
}

impl std::fmt::Display for CiphertextRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "handle {}", self.handle)
    }
}

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

impl ConsensusRound<CiphertextAttestation> {
    /// The handle this round is about.
    pub fn handle(&self) -> B256 {
        self.subject.handle
    }
}
