//! Threshold verifier-set account data.

use super::*;

/// Fixed-size threshold verifier set for host-facing signature proofs.
#[account]
pub struct VerifierSet {
    /// Admin that created and may disable this set.
    pub admin: Pubkey,
    /// Protocol purpose for this set.
    pub kind: u8,
    /// Scope that disambiguates sets of the same kind.
    ///
    /// Input sets are scoped to the host config PDA. Token disclosure and
    /// redemption sets are scoped to the confidential mint they certify.
    pub scope: Pubkey,
    /// Monotonic version chosen by the admin for rotation.
    pub version: u64,
    /// Number of distinct signer signatures required.
    pub threshold: u8,
    /// Number of active entries in `signers`.
    pub signer_count: u8,
    /// Fixed signer list. Entries after `signer_count` must be default keys.
    pub signers: [Pubkey; MAX_VERIFIER_SET_SIGNERS],
    /// Active/disabled state.
    pub state: u8,
    /// Slot in which the set was created.
    pub created_slot: u64,
    /// Slot in which the set was last updated.
    pub updated_slot: u64,
    /// PDA bump for the canonical verifier-set address.
    pub bump: u8,
}

impl VerifierSet {
    pub const SPACE: usize =
        32 + 1 + 32 + 8 + 1 + 1 + (32 * MAX_VERIFIER_SET_SIGNERS) + 1 + 8 + 8 + 1;

    pub fn signer_slice(&self) -> &[Pubkey] {
        &self.signers[..self.signer_count as usize]
    }

    pub fn is_active(&self) -> bool {
        self.state == VERIFIER_SET_STATE_ACTIVE
    }

    pub fn contains_signer(&self, signer: Pubkey) -> bool {
        self.signer_slice().iter().any(|entry| *entry == signer)
    }

    pub fn validate_shape(&self) -> bool {
        verifier_set_fields_are_valid(
            self.kind,
            self.threshold,
            self.signer_count,
            &self.signers,
            self.state,
        )
    }
}

pub fn verifier_set_fields_are_valid(
    kind: u8,
    threshold: u8,
    signer_count: u8,
    signers: &[Pubkey; MAX_VERIFIER_SET_SIGNERS],
    state: u8,
) -> bool {
    if !verifier_set_kind_is_valid(kind) {
        return false;
    }
    if state != VERIFIER_SET_STATE_ACTIVE && state != VERIFIER_SET_STATE_DISABLED {
        return false;
    }
    let signer_count = signer_count as usize;
    if signer_count == 0 || signer_count > MAX_VERIFIER_SET_SIGNERS {
        return false;
    }
    if threshold == 0 || threshold as usize > signer_count {
        return false;
    }
    for index in 0..signer_count {
        let signer = signers[index];
        if signer == Pubkey::default() {
            return false;
        }
        if signers[..index].iter().any(|previous| *previous == signer) {
            return false;
        }
    }
    signers[signer_count..]
        .iter()
        .all(|signer| *signer == Pubkey::default())
}

pub fn verifier_set_kind_is_valid(kind: u8) -> bool {
    matches!(
        kind,
        VERIFIER_SET_KIND_INPUT
            | VERIFIER_SET_KIND_TOKEN_DISCLOSURE
            | VERIFIER_SET_KIND_TOKEN_REDEMPTION
    )
}

pub fn verifier_set_scope_is_valid_for_kind(kind: u8, scope: Pubkey, host_config: Pubkey) -> bool {
    match kind {
        VERIFIER_SET_KIND_INPUT => scope == host_config,
        VERIFIER_SET_KIND_TOKEN_DISCLOSURE | VERIFIER_SET_KIND_TOKEN_REDEMPTION => {
            scope != Pubkey::default() && scope != host_config
        }
        _ => false,
    }
}

pub fn verifier_set_rotates_host_input(kind: u8) -> bool {
    kind == VERIFIER_SET_KIND_INPUT
}
