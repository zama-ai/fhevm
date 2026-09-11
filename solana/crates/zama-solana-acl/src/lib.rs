//! Shared encrypted-state ACL core for the Zama Solana port (RFC 035).
//!
//! The single source of truth — used identically by the on-chain `zama-host` program, the
//! coprocessor's leaf indexer, and the off-chain KMS connector — for the `EncryptedStore`
//! account layout, its PDA seeds, its Merkle Mountain Range history, the leaf commitments, and
//! the decrypt-authorization rules. Sharing this crate makes the host↔KMS lockstep type-level
//! instead of a convention checked by tests.
//!
//! Deliberately solana-version-agnostic (pure `borsh` + hashing, pubkeys as raw `[u8; 32]`) so
//! the on-chain programs and the connector can share it. PDA derivation stays on each side; this
//! crate provides the exact seed list through [`EncryptedStore::seeds`].
//!
//! Hashing: leaf commitments and MMR nodes use keccak256, the hash the EVM ACL (RFC 034) uses,
//! so the two chains share one leaf encoding and one proof verifier. Anchor account
//! discriminators stay sha256 because Anchor defines them that way.
//!
//! Public API surface: the verifier side. The KMS connector, the coprocessor indexer, and any
//! third-party verifier reconstruct and check `EncryptedStore` state through these exports, so a
//! predicate with no caller in this repository is still doing real work for them.

#[cfg(not(target_os = "solana"))]
use sha2::{Digest as _, Sha256};
#[cfg(not(target_os = "solana"))]
use sha3::Keccak256;

pub mod encrypted_store;
pub use encrypted_store::{
    decode_encrypted_store, encrypted_store_discriminator, EncryptedSlot, EncryptedStore,
    ENCRYPTED_STORE_SEED, MAX_STORE_SLOTS,
};

pub mod delegation;
pub use delegation::{
    decode_user_decryption_delegation, UserDecryptionDelegationRecord, DELEGATION_SEED,
    USER_DECRYPTION_DELEGATION_DISCRIMINATOR, WILDCARD_AUTHORITY,
};
pub mod host_config;
pub use host_config::{
    decode_host_config, encode_host_config, HostConfigRecord, HOST_CONFIG_DISCRIMINATOR,
    HOST_CONFIG_SEED,
};
pub mod history;
pub use history::{
    build_proof_from_events, build_verified_proof_from_events, reconstruct,
    ReconstructedStoreHistory, StoreHistoryError, StoreHistoryEvent,
};

pub mod mmr;
pub use mmr::{
    mmr_append, mmr_build_proof, mmr_leaf_node, mmr_node, mmr_peaks_from_leaves, mmr_verify,
    MmrProof, MAX_MMR_PEAKS,
};

const HISTORICAL_ACCESS_LEAF_PREFIX: &[u8] = b"ZAMA_HIST_ACCESS_LEAF_V1";
const PUBLIC_DECRYPT_LEAF_PREFIX: &[u8] = b"ZAMA_PUBLIC_DECRYPT_LEAF_V1";

/// Errors from the shared ACL/MMR layer. Each side maps these to its own error type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AclError {
    BadDiscriminator,
    BadAccountData,
    MmrInconsistent,
    MmrPeakCapacityExceeded,
    HistoricalProofInvalid,
    PublicDecryptProofInvalid,
}

#[cfg(not(target_os = "solana"))]
pub(crate) fn sha256(parts: &[&[u8]]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update(part);
    }
    hasher.finalize().into()
}

#[cfg(target_os = "solana")]
pub(crate) fn sha256(parts: &[&[u8]]) -> [u8; 32] {
    solana_sha256_hasher::hashv(parts).to_bytes()
}

#[cfg(not(target_os = "solana"))]
pub(crate) fn keccak256(parts: &[&[u8]]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    for part in parts {
        hasher.update(part);
    }
    hasher.finalize().into()
}

#[cfg(target_os = "solana")]
pub(crate) fn keccak256(parts: &[&[u8]]) -> [u8; 32] {
    solana_keccak_hasher::hashv(parts).to_bytes()
}

/// Commitment for `HistoricalAccessLeaf { encrypted_store_account, leaf_index, handle, key }`:
/// one `allow` of `key` on `handle`. `leaf_index` is bound in so a leaf cannot be replayed at a
/// different position.
pub fn historical_access_leaf_commitment(
    encrypted_store_account: [u8; 32],
    leaf_index: u64,
    handle: [u8; 32],
    key: [u8; 32],
) -> [u8; 32] {
    keccak256(&[
        HISTORICAL_ACCESS_LEAF_PREFIX,
        &encrypted_store_account,
        &leaf_index.to_be_bytes(),
        &handle,
        &key,
    ])
}

/// Commitment for `PublicDecryptLeaf { encrypted_store_account, leaf_index, handle }`.
pub fn public_decrypt_leaf_commitment(
    encrypted_store_account: [u8; 32],
    leaf_index: u64,
    handle: [u8; 32],
) -> [u8; 32] {
    keccak256(&[
        PUBLIC_DECRYPT_LEAF_PREFIX,
        &encrypted_store_account,
        &leaf_index.to_be_bytes(),
        &handle,
    ])
}

/// User decrypt under an encrypted store: a valid historical-access proof for the exact handle
/// and key. The handle may be absent from the state's current slots; the retained MMR is the
/// authority for historical decryption.
pub fn authorize_state_historical(
    encrypted_store: [u8; 32],
    state: &EncryptedStore,
    handle: [u8; 32],
    key: [u8; 32],
    proof: &MmrProof,
) -> Result<(), AclError> {
    let commitment =
        historical_access_leaf_commitment(encrypted_store, proof.leaf_index, handle, key);
    verify_leaf(
        &state.peaks,
        state.leaf_count,
        commitment,
        proof,
        AclError::HistoricalProofInvalid,
    )
}

/// Exact public decrypt under an encrypted store. Current slot contents do not participate:
/// public access is bound to the exact historical handle by the proven leaf.
pub fn authorize_state_public(
    encrypted_store: [u8; 32],
    state: &EncryptedStore,
    handle: [u8; 32],
    proof: &MmrProof,
) -> Result<(), AclError> {
    let commitment = public_decrypt_leaf_commitment(encrypted_store, proof.leaf_index, handle);
    verify_leaf(
        &state.peaks,
        state.leaf_count,
        commitment,
        proof,
        AclError::PublicDecryptProofInvalid,
    )
}

fn verify_leaf(
    peaks: &[[u8; 32]],
    leaf_count: u64,
    commitment: [u8; 32],
    proof: &MmrProof,
    invalid: AclError,
) -> Result<(), AclError> {
    if mmr_verify(peaks, leaf_count, commitment, proof) {
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

    /// Doc-sync guard for `docs/MMR_ACL_MVP.md` "Resource Bounds And Liveness": the
    /// stranding-impossible argument quotes these exact numbers, so a change here must update
    /// that section in the same PR.
    #[test]
    fn resource_bounds_match_liveness_doc() {
        assert_eq!(MAX_MMR_PEAKS, 64, "MMR_ACL_MVP.md liveness section");
        // account_size = 181 + 32·peaks; max = 2229 bytes, forever.
        assert_eq!(EncryptedStore::account_size(0, 0), 121);
        assert_eq!(
            EncryptedStore::account_size(MAX_STORE_SLOTS, MAX_MMR_PEAKS),
            4217,
            "max account must stay « Solana's 10240-byte realloc cap"
        );
    }

    /// A test encrypted store that maintains its own leaf list so it can build proofs,
    /// mirroring the coprocessor indexer.
    #[derive(Default)]
    struct StoreHistory {
        value: EncryptedStore,
        account: [u8; 32],
        leaves: Vec<[u8; 32]>,
    }

    impl StoreHistory {
        fn new(handle: [u8; 32]) -> Self {
            Self {
                value: EncryptedStore {
                    slots: vec![EncryptedSlot {
                        key: [0; 32],
                        handle,
                    }],
                    ..Default::default()
                },
                account: h(0xAC),
                leaves: Vec::new(),
            }
        }

        fn append(&mut self, commitment: [u8; 32]) {
            mmr_append(
                &mut self.value.peaks,
                &mut self.value.leaf_count,
                commitment,
            )
            .unwrap();
            self.leaves.push(commitment);
        }

        /// Mirrors a write: the handle is overwritten, then one leaf per allowed key on it.
        fn update(&mut self, new_handle: [u8; 32], allows: &[[u8; 32]]) {
            self.value.slots[0].handle = new_handle;
            for key in allows {
                let idx = self.value.leaf_count;
                self.append(historical_access_leaf_commitment(
                    self.account,
                    idx,
                    new_handle,
                    *key,
                ));
            }
        }

        fn make_public(&mut self) {
            let idx = self.value.leaf_count;
            let c = public_decrypt_leaf_commitment(self.account, idx, self.value.slots[0].handle);
            self.append(c);
        }

        fn proof(&self, i: u64) -> MmrProof {
            mmr_build_proof(&self.leaves, i).unwrap()
        }
    }

    #[test]
    fn a_leaf_authorizes_its_key_on_its_handle_only() {
        let owner = h(1);
        let mut l = StoreHistory::new(h(10));
        l.update(h(10), &[owner]);
        l.update(h(11), &[owner]);
        let old = l.proof(0);
        let new = l.proof(1);
        assert!(authorize_state_historical(l.account, &l.value, h(10), owner, &old).is_ok());
        assert!(authorize_state_historical(l.account, &l.value, h(11), owner, &new).is_ok());
        assert!(authorize_state_historical(l.account, &l.value, h(10), h(2), &old).is_err());
        assert!(authorize_state_historical(l.account, &l.value, h(11), owner, &old).is_err());
    }

    #[test]
    fn exact_public_no_roll_forward() {
        let mut l = StoreHistory::new(h(10));
        l.make_public();
        l.update(h(11), &[]);
        let proof = l.proof(0);
        assert!(authorize_state_public(l.account, &l.value, h(10), &proof).is_ok());
        assert_eq!(
            authorize_state_public(l.account, &l.value, h(11), &proof),
            Err(AclError::PublicDecryptProofInvalid)
        );
    }

    #[test]
    fn store_history_authorizes_exact_handles_without_a_current_slot_match() {
        let account = h(0xac);
        let old_handle = h(10);
        let current_handle = h(11);
        let key = h(1);
        let leaves = vec![
            historical_access_leaf_commitment(account, 0, old_handle, key),
            public_decrypt_leaf_commitment(account, 1, old_handle),
        ];
        let state = EncryptedStore {
            slots: vec![EncryptedSlot {
                key: h(20),
                handle: current_handle,
            }],
            leaf_count: leaves.len() as u64,
            peaks: mmr_peaks_from_leaves(&leaves),
            ..Default::default()
        };

        assert!(authorize_state_historical(
            account,
            &state,
            old_handle,
            key,
            &mmr_build_proof(&leaves, 0).unwrap()
        )
        .is_ok());
        assert!(authorize_state_public(
            account,
            &state,
            old_handle,
            &mmr_build_proof(&leaves, 1).unwrap()
        )
        .is_ok());
        assert_eq!(state.get(&h(20)), Some(current_handle));
        assert!(authorize_state_historical(
            account,
            &state,
            current_handle,
            key,
            &mmr_build_proof(&leaves, 0).unwrap()
        )
        .is_err());
    }

    #[test]
    fn a_value_nobody_was_allowed_on_cannot_be_decrypted() {
        let l = StoreHistory::new(h(10));
        assert_eq!(l.value.leaf_count, 0);
        let empty = MmrProof::default();
        assert!(authorize_state_historical(l.account, &l.value, h(10), h(1), &empty).is_err());
        assert!(authorize_state_public(l.account, &l.value, h(10), &empty).is_err());
    }

    #[test]
    fn on_chain_account_decoder_reads_layout() {
        let mut l = StoreHistory::new(h(10));
        l.value.program = h(0x50);
        l.value.authority = h(0x51);
        l.value.scope = h(0x52);
        l.value.slots[0].key = h(0x53);
        l.value.bump = 254;
        l.update(h(10), &[h(1), h(2)]);
        l.make_public();
        let mut data = encrypted_store_discriminator().to_vec();
        data.extend_from_slice(&borsh::to_vec(&l.value).unwrap());
        assert_eq!(
            data.len(),
            EncryptedStore::account_size(l.value.slots.len(), l.value.peaks.len())
        );

        let decoded = decode_encrypted_store(&data).unwrap();
        assert_eq!(decoded, l.value);
        assert_eq!(
            decoded.seeds(),
            [
                ENCRYPTED_STORE_SEED,
                &h(0x50)[..],
                &h(0x51)[..],
                &h(0x52)[..]
            ]
        );
        data[0] ^= 1;
        assert_eq!(
            decode_encrypted_store(&data),
            Err(AclError::BadDiscriminator)
        );
    }
}
