//! PRF-specific consensus: what a round is about and what the Coprocessors agree on.

use alloy_primitives::B256;

/// A PRF evaluation: the subject of a PRF output consensus round.
///
/// Both fields are bound by the signature but absent from the wire form. The verifier rebuilds
/// them from the request path.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PrfOutputRef {
    pub prf_id: u16,
    pub label: B256,
}

impl PrfOutputRef {
    pub fn new(prf_id: u16, label: B256) -> Self {
        Self { prf_id, label }
    }
}

impl std::fmt::Display for PrfOutputRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "prf {} label {}", self.prf_id, self.label)
    }
}

/// The PRF output material a consensus group agreed on: the digest of the PRF output.
pub type PrfMaterial = B256;
