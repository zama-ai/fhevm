//! Shared encrypted-value ACL core for the Zama Solana PoC (fhevm-internal#1569).
//!
//! The single source of truth — used identically by the on-chain `zama-host`
//! program and the off-chain KMS connector — for the encrypted-value ACL account
//! layout, its Merkle Mountain Range history, the leaf commitments, the value-key
//! derivation, and the decrypt-authorization rules. Sharing this crate makes the
//! host↔KMS lockstep type-level instead of a convention checked by tests.
//!
//! Deliberately solana-version-agnostic (pure `borsh` + `sha3`/`sha2`, pubkeys as
//! raw `[u8; 32]`) so the on-chain programs (solana 3.x) and the connector
//! (solana 2.x) can share it. PDA derivation stays on each side; this crate
//! provides the seed and the `value_key`.

use sha2::{Digest as _, Sha256};

pub mod lineage;
pub use lineage::{
    build_proof_from_events, build_verified_proof_from_events, reconstruct, LineageError,
    LineageEvent, ReconstructedLineage,
};

pub mod mmr;
pub use mmr::{
    mmr_append, mmr_build_proof, mmr_leaf_node, mmr_node, mmr_peaks_from_leaves, mmr_verify,
    MmrProof,
};

use sha3::Keccak256;

/// PDA seed for an encrypted-value ACL lineage: `[ENCRYPTED_VALUE_ACL_SEED, value_key]`.
pub const ENCRYPTED_VALUE_ACL_SEED: &[u8] = b"encrypted-value-acl";
/// Upper bound on durable subjects, for write-side validation.
pub const MAX_ENCRYPTED_VALUE_SUBJECTS: usize = 8;

const HISTORICAL_ACCESS_LEAF_PREFIX: &[u8] = b"ZAMA_HIST_ACCESS_LEAF_V1";
const PUBLIC_DECRYPT_LEAF_PREFIX: &[u8] = b"ZAMA_PUBLIC_DECRYPT_LEAF_V1";

/// Errors from the shared ACL/MMR layer. Each side maps these to its own error type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AclError {
    BadDiscriminator,
    BadAccountData,
    MmrInconsistent,
    SubjectCapacityExceeded,
    HandleMismatch,
    SubjectMissing,
    HistoricalProofInvalid,
    PublicDecryptProofInvalid,
}

/// Current authorization state and compact history for one encrypted-value lineage.
///
/// One account per lineage, reused across every rotation. `peaks`/`subjects` grow
/// with use (the on-chain account is `realloc`-grown), so a current-only lineage
/// (`leaf_count == 0`) stays tiny and pays nothing for history.
#[derive(borsh::BorshSerialize, borsh::BorshDeserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct EncryptedValueAcl {
    /// App-level ACL domain, such as a confidential token mint.
    pub acl_domain_key: [u8; 32],
    /// App-owned account whose encrypted field this lineage represents.
    pub app_account: [u8; 32],
    /// Domain-separated encrypted field label inside `app_account`.
    pub encrypted_value_label: [u8; 32],
    /// Current encrypted value identifier (the live handle).
    pub current_handle: [u8; 32],
    /// Current durable subjects (binary membership; no role flags).
    pub subjects: Vec<[u8; 32]>,
    /// Number of MMR leaves appended; `0` means no history.
    pub leaf_count: u64,
    /// MMR peaks, oldest mountain first (`popcount(leaf_count)` entries).
    pub peaks: Vec<[u8; 32]>,
    /// PDA bump.
    pub bump: u8,
}

impl EncryptedValueAcl {
    /// The lineage's value key — its PDA seed. Derived, never stored.
    pub fn value_key(&self) -> [u8; 32] {
        acl_nonce_key(
            self.acl_domain_key,
            self.app_account,
            self.encrypted_value_label,
        )
    }

    /// Whether `subject` is a current durable member.
    pub fn is_subject(&self, subject: [u8; 32]) -> bool {
        self.subjects.contains(&subject)
    }

    /// Full on-chain account size (8-byte discriminator + borsh body) for a lineage
    /// with `subjects_len` subjects and `peaks_len` peaks. Used to `init`/`realloc`.
    pub fn account_size(subjects_len: usize, peaks_len: usize) -> usize {
        // disc + (domain+app+label+handle) + subjects(vec) + leaf_count + peaks(vec) + bump
        8 + (32 * 4) + (4 + 32 * subjects_len) + 8 + (4 + 32 * peaks_len) + 1
    }
}

/// The Anchor-style 8-byte account discriminator, `sha256("account:EncryptedValueAcl")[..8]`.
pub fn encrypted_value_acl_discriminator() -> [u8; 8] {
    let digest = Sha256::digest(b"account:EncryptedValueAcl");
    let mut disc = [0u8; 8];
    disc.copy_from_slice(&digest[..8]);
    disc
}

/// Decodes an `EncryptedValueAcl` from raw account data (discriminator + borsh body).
///
/// Reads exactly the struct's bytes and ignores any trailing capacity: a Solana
/// account buffer presented to a program (especially one passed through a CPI)
/// carries realloc headroom past `data_len`, so a strict whole-slice decode would
/// spuriously reject a valid account. The 8-byte discriminator still pins the type.
pub fn decode_account(data: &[u8]) -> Result<EncryptedValueAcl, AclError> {
    if data.len() < 8 || data[..8] != encrypted_value_acl_discriminator() {
        return Err(AclError::BadDiscriminator);
    }
    let mut body = &data[8..];
    <EncryptedValueAcl as borsh::BorshDeserialize>::deserialize(&mut body)
        .map_err(|_| AclError::BadAccountData)
}

/// Encodes an `EncryptedValueAcl` to raw account data (discriminator + borsh body).
pub fn encode_account(acl: &EncryptedValueAcl) -> Result<Vec<u8>, AclError> {
    let mut data = encrypted_value_acl_discriminator().to_vec();
    let body = borsh::to_vec(acl).map_err(|_| AclError::BadAccountData)?;
    data.extend_from_slice(&body);
    Ok(data)
}

/// The app-controlled value key for one encrypted field — the lineage's PDA seed.
/// Contains app metadata, not the opaque handle, so the address is predeclarable.
pub fn acl_nonce_key(
    acl_domain_key: [u8; 32],
    app_account: [u8; 32],
    encrypted_value_label: [u8; 32],
) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b"zama-acl-nonce-key-v1");
    hasher.update(acl_domain_key);
    hasher.update(app_account);
    hasher.update(encrypted_value_label);
    hasher.finalize().into()
}

fn keccak(parts: &[&[u8]]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    for part in parts {
        hasher.update(part);
    }
    hasher.finalize().into()
}

/// Commitment for `HistoricalAccessLeaf { acl_account, leaf_index, encrypted_value, subject }`.
/// `leaf_index` is bound in so a leaf cannot be replayed at a different position.
pub fn historical_access_leaf_commitment(
    acl_account: [u8; 32],
    leaf_index: u64,
    encrypted_value: [u8; 32],
    subject: [u8; 32],
) -> [u8; 32] {
    keccak(&[
        HISTORICAL_ACCESS_LEAF_PREFIX,
        &acl_account,
        &leaf_index.to_be_bytes(),
        &encrypted_value,
        &subject,
    ])
}

/// Commitment for `PublicDecryptLeaf { acl_account, leaf_index, encrypted_value }`.
pub fn public_decrypt_leaf_commitment(
    acl_account: [u8; 32],
    leaf_index: u64,
    encrypted_value: [u8; 32],
) -> [u8; 32] {
    keccak(&[
        PUBLIC_DECRYPT_LEAF_PREFIX,
        &acl_account,
        &leaf_index.to_be_bytes(),
        &encrypted_value,
    ])
}

/// Current decrypt: `handle` is the live handle and `subject` is a current member. No proof.
pub fn authorize_current(
    acl: &EncryptedValueAcl,
    handle: [u8; 32],
    subject: [u8; 32],
) -> Result<(), AclError> {
    if acl.current_handle != handle {
        return Err(AclError::HandleMismatch);
    }
    if !acl.is_subject(subject) {
        return Err(AclError::SubjectMissing);
    }
    Ok(())
}

/// Historical decrypt: a valid historical-access MMR proof is the authorization;
/// the subject is bound into the proven leaf, so it survives membership changes.
/// A current-only lineage (`leaf_count == 0`) has no provable history, so this fails.
pub fn authorize_historical(
    acl_account: [u8; 32],
    acl: &EncryptedValueAcl,
    encrypted_value: [u8; 32],
    subject: [u8; 32],
    proof: &MmrProof,
) -> Result<(), AclError> {
    let commitment =
        historical_access_leaf_commitment(acl_account, proof.leaf_index, encrypted_value, subject);
    verify_leaf(acl, commitment, proof, AclError::HistoricalProofInvalid)
}

/// Exact public decrypt: a valid public-decrypt MMR proof for the exact handle.
/// There is no live public flag, so a proof for one handle never authorizes a later one.
pub fn authorize_public(
    acl_account: [u8; 32],
    acl: &EncryptedValueAcl,
    encrypted_value: [u8; 32],
    proof: &MmrProof,
) -> Result<(), AclError> {
    let commitment = public_decrypt_leaf_commitment(acl_account, proof.leaf_index, encrypted_value);
    verify_leaf(acl, commitment, proof, AclError::PublicDecryptProofInvalid)
}

fn verify_leaf(
    acl: &EncryptedValueAcl,
    commitment: [u8; 32],
    proof: &MmrProof,
    invalid: AclError,
) -> Result<(), AclError> {
    if mmr_verify(&acl.peaks, acl.leaf_count, commitment, proof) {
        Ok(())
    } else {
        Err(invalid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn h(tag: u8) -> [u8; 32] {
        [tag; 32]
    }

    /// A test lineage that maintains its own leaf list so it can build proofs,
    /// mirroring an off-chain proof service.
    #[derive(Default)]
    struct Lineage {
        acl: EncryptedValueAcl,
        account: [u8; 32],
        leaves: Vec<[u8; 32]>,
    }

    impl Lineage {
        fn new(handle: [u8; 32], subjects: &[[u8; 32]]) -> Self {
            let account = h(0xAC);
            Self {
                acl: EncryptedValueAcl {
                    current_handle: handle,
                    subjects: subjects.to_vec(),
                    ..Default::default()
                },
                account,
                leaves: Vec::new(),
            }
        }

        fn append(&mut self, commitment: [u8; 32]) {
            mmr_append(&mut self.acl.peaks, &mut self.acl.leaf_count, commitment).unwrap();
            self.leaves.push(commitment);
        }

        fn rotate(&mut self, new_handle: [u8; 32]) {
            let old = self.acl.current_handle;
            for i in 0..self.acl.subjects.len() {
                let idx = self.acl.leaf_count;
                let c =
                    historical_access_leaf_commitment(self.account, idx, old, self.acl.subjects[i]);
                self.append(c);
            }
            self.acl.current_handle = new_handle;
        }

        fn mark_public(&mut self) {
            let idx = self.acl.leaf_count;
            let c = public_decrypt_leaf_commitment(self.account, idx, self.acl.current_handle);
            self.append(c);
        }

        fn proof(&self, i: u64) -> MmrProof {
            mmr_build_proof(&self.leaves, i).unwrap()
        }
    }

    #[test]
    fn current_and_rejections() {
        let owner = h(1);
        let l = Lineage::new(h(10), &[owner]);
        assert!(authorize_current(&l.acl, h(10), owner).is_ok());
        assert_eq!(
            authorize_current(&l.acl, h(10), h(2)),
            Err(AclError::SubjectMissing)
        );
        assert_eq!(
            authorize_current(&l.acl, h(99), owner),
            Err(AclError::HandleMismatch)
        );
    }

    #[test]
    fn post_rotation_then_historical_proof() {
        let owner = h(1);
        let mut l = Lineage::new(h(10), &[owner]);
        l.rotate(h(11));
        assert_eq!(
            authorize_current(&l.acl, h(10), owner),
            Err(AclError::HandleMismatch)
        );
        assert!(authorize_current(&l.acl, h(11), owner).is_ok());
        let proof = l.proof(0);
        assert!(authorize_historical(l.account, &l.acl, h(10), owner, &proof).is_ok());
        assert!(authorize_historical(l.account, &l.acl, h(10), h(2), &proof).is_err());
        assert!(authorize_historical(l.account, &l.acl, h(99), owner, &proof).is_err());
    }

    #[test]
    fn exact_public_no_roll_forward() {
        let owner = h(1);
        let mut l = Lineage::new(h(10), &[owner]);
        l.mark_public();
        l.rotate(h(11));
        let proof = l.proof(0);
        assert!(authorize_public(l.account, &l.acl, h(10), &proof).is_ok());
        assert_eq!(
            authorize_public(l.account, &l.acl, h(11), &proof),
            Err(AclError::PublicDecryptProofInvalid)
        );
    }

    #[test]
    fn no_history_rejects_proofs_but_current_works() {
        let owner = h(1);
        let l = Lineage::new(h(10), &[owner]);
        assert_eq!(l.acl.leaf_count, 0);
        assert!(authorize_current(&l.acl, h(10), owner).is_ok());
        let empty = MmrProof::default();
        assert!(authorize_historical(l.account, &l.acl, h(10), owner, &empty).is_err());
        assert!(authorize_public(l.account, &l.acl, h(10), &empty).is_err());
    }

    #[test]
    fn account_round_trips_through_shared_codec() {
        let mut l = Lineage::new(h(10), &[h(1), h(2)]);
        l.rotate(h(11));
        let data = encode_account(&l.acl).unwrap();
        assert_eq!(
            data.len(),
            EncryptedValueAcl::account_size(l.acl.subjects.len(), l.acl.peaks.len())
        );
        assert_eq!(decode_account(&data).unwrap(), l.acl);
        let mut bad = data.clone();
        bad[0] ^= 0xff;
        assert_eq!(decode_account(&bad), Err(AclError::BadDiscriminator));
    }
}
