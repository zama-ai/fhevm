//! Shared encrypted-value ACL core for the Zama Solana port (RFC 035).
//!
//! The single source of truth — used identically by the on-chain `zama-host` program, the
//! coprocessor's leaf indexer, and the off-chain KMS connector — for the `EncryptedValue`
//! account layout, its PDA seeds, its Merkle Mountain Range history, the leaf commitments, and
//! the decrypt-authorization rules. Sharing this crate makes the host↔KMS lockstep type-level
//! instead of a convention checked by tests.
//!
//! Deliberately solana-version-agnostic (pure `borsh` + hashing, pubkeys as raw `[u8; 32]`) so
//! the on-chain programs and the connector can share it. PDA derivation stays on each side; this
//! crate provides the exact seed list through [`encrypted_value_seeds`].
//!
//! Hashing: leaf commitments and MMR nodes use keccak256, the hash the EVM ACL (RFC 034) uses,
//! so the two chains share one leaf encoding and one proof verifier. Anchor account
//! discriminators stay sha256 because Anchor defines them that way.
//!
//! Public API surface: the verifier side. The KMS connector, the coprocessor indexer, and any
//! third-party verifier reconstruct and check `EncryptedValue` state through these exports, so a
//! predicate with no caller in this repository is still doing real work for them.

#[cfg(not(target_os = "solana"))]
use sha2::{Digest as _, Sha256};
#[cfg(not(target_os = "solana"))]
use sha3::Keccak256;

pub mod delegation;
pub use delegation::{
    decode_user_decryption_delegation, UserDecryptionDelegationRecord, DELEGATION_SEED,
    USER_DECRYPTION_DELEGATION_DISCRIMINATOR, WILDCARD_ENCRYPTED_VALUE_ACCOUNT_AUTHORITY,
};
pub mod host_config;
pub use host_config::{
    decode_host_config, encode_host_config, HostConfigRecord, HOST_CONFIG_DISCRIMINATOR,
    HOST_CONFIG_SEED,
};
pub mod encrypted_value_account;
pub use encrypted_value_account::{
    build_proof_from_events, build_verified_proof_from_events, reconstruct,
    EncryptedValueAccountError, EncryptedValueAccountEvent, ReconstructedEncryptedValueAccount,
};

pub mod mmr;
pub use mmr::{
    mmr_append, mmr_build_proof, mmr_leaf_node, mmr_node, mmr_peaks_from_leaves, mmr_verify,
    MmrProof, MAX_MMR_PEAKS,
};

/// First PDA seed of an encrypted value account; the full list is [`encrypted_value_seeds`].
pub const ENCRYPTED_VALUE_SEED: &[u8] = b"encrypted-value";

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

/// One persistent encrypted value: who owns it, which handle is current, and the peaks of the
/// MMR that fingerprints every decrypt permission ever sealed on it.
///
/// Compute on the value is authorized by the authority's signature and nothing else; decrypt is
/// authorized off-chain by a leaf proof against `peaks`. The account stores nothing about who
/// may decrypt. It is `realloc`-grown by one peak at a time and never shrunk.
#[derive(borsh::BorshSerialize, borsh::BorshDeserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct EncryptedValue {
    /// The application program this value belongs to. Verified by the host on every write:
    /// `encrypted_value_account_authority` must be a PDA of `program`, so only that program can
    /// sign for it. Never taken on the caller's word.
    pub program: [u8; 32],
    /// The account that controls this encrypted value: a PDA of `program` that must sign to
    /// create it or update its handle. For a token balance this is the token account itself.
    pub encrypted_value_account_authority: [u8; 32],
    /// Program-declared scope inside `program`'s namespace — the mint for the token program.
    /// Trustworthy only as the pair `(program, scope)`: another program cannot forge the
    /// `program` half. The pair is the application identity for HCU metering, the deny list and
    /// permit scoping.
    pub scope: [u8; 32],
    /// Which of the authority's values this is.
    pub label: [u8; 32],
    /// Current encrypted value identifier (the live handle).
    pub current_handle: [u8; 32],
    /// Number of MMR leaves appended; `0` means no history.
    pub leaf_count: u64,
    /// MMR peaks, oldest mountain first (`popcount(leaf_count)` entries).
    pub peaks: Vec<[u8; 32]>,
    /// PDA bump.
    pub bump: u8,
}

impl EncryptedValue {
    /// The account's PDA seeds, recomputed from its own fields. A verifier derives the address
    /// from these and refuses a well-formed account found anywhere else.
    pub fn seeds(&self) -> [&[u8]; 5] {
        encrypted_value_seeds(
            &self.program,
            &self.encrypted_value_account_authority,
            &self.scope,
            &self.label,
        )
    }

    /// Full on-chain account size (8-byte discriminator + borsh body) with `peaks_len` peaks.
    /// Used to `init`/`realloc`.
    pub fn account_size(peaks_len: usize) -> usize {
        // disc + (program+authority+scope+label+handle) + leaf_count + peaks(vec) + bump
        8 + (32 * 5) + 8 + (4 + 32 * peaks_len) + 1
    }
}

/// The exact PDA seed list of an encrypted value account: a constant tag then four fixed-width
/// fields, in this order, with no intermediate hash — so there is exactly one way to build it
/// and two seed lists can never concatenate to the same bytes.
pub fn encrypted_value_seeds<'a>(
    program: &'a [u8; 32],
    encrypted_value_account_authority: &'a [u8; 32],
    scope: &'a [u8; 32],
    label: &'a [u8; 32],
) -> [&'a [u8]; 5] {
    [
        ENCRYPTED_VALUE_SEED,
        program,
        encrypted_value_account_authority,
        scope,
        label,
    ]
}

/// The Anchor-style 8-byte account discriminator, `sha256("account:EncryptedValue")[..8]`.
pub fn encrypted_value_discriminator() -> [u8; 8] {
    let digest = sha256(&[b"account:EncryptedValue"]);
    let mut disc = [0u8; 8];
    disc.copy_from_slice(&digest[..8]);
    disc
}

/// Decodes `zama-host`'s real on-chain account layout: discriminator, then the borsh body of
/// [`EncryptedValue`], which is the on-chain layout by construction (the program's own pin test
/// serializes with Anchor and decodes with this function).
pub fn decode_on_chain_account(data: &[u8]) -> Result<EncryptedValue, AclError> {
    if data.len() < 8 || data[..8] != encrypted_value_discriminator() {
        return Err(AclError::BadDiscriminator);
    }
    let mut body = &data[8..];
    <EncryptedValue as borsh::BorshDeserialize>::deserialize(&mut body)
        .map_err(|_| AclError::BadAccountData)
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

/// Commitment for `HistoricalAccessLeaf { encrypted_value_account, leaf_index, handle, key }`:
/// one `allow` of `key` on `handle`. `leaf_index` is bound in so a leaf cannot be replayed at a
/// different position.
pub fn historical_access_leaf_commitment(
    encrypted_value_account: [u8; 32],
    leaf_index: u64,
    handle: [u8; 32],
    key: [u8; 32],
) -> [u8; 32] {
    keccak256(&[
        HISTORICAL_ACCESS_LEAF_PREFIX,
        &encrypted_value_account,
        &leaf_index.to_be_bytes(),
        &handle,
        &key,
    ])
}

/// Commitment for `PublicDecryptLeaf { encrypted_value_account, leaf_index, handle }`.
pub fn public_decrypt_leaf_commitment(
    encrypted_value_account: [u8; 32],
    leaf_index: u64,
    handle: [u8; 32],
) -> [u8; 32] {
    keccak256(&[
        PUBLIC_DECRYPT_LEAF_PREFIX,
        &encrypted_value_account,
        &leaf_index.to_be_bytes(),
        &handle,
    ])
}

/// User decrypt: a valid historical-access MMR proof is the authorization, for the current
/// handle and for an old one alike. The key is bound into the proven leaf. A value with no
/// leaves (`leaf_count == 0`) has nobody allowed on it, so this fails.
pub fn authorize_historical(
    encrypted_value_account: [u8; 32],
    value: &EncryptedValue,
    handle: [u8; 32],
    key: [u8; 32],
    proof: &MmrProof,
) -> Result<(), AclError> {
    let commitment =
        historical_access_leaf_commitment(encrypted_value_account, proof.leaf_index, handle, key);
    verify_leaf(value, commitment, proof, AclError::HistoricalProofInvalid)
}

/// Exact public decrypt: a valid public-decrypt MMR proof for the exact handle.
/// There is no live public flag, so a proof for one handle never authorizes a later one.
pub fn authorize_public(
    encrypted_value_account: [u8; 32],
    value: &EncryptedValue,
    handle: [u8; 32],
    proof: &MmrProof,
) -> Result<(), AclError> {
    let commitment =
        public_decrypt_leaf_commitment(encrypted_value_account, proof.leaf_index, handle);
    verify_leaf(
        value,
        commitment,
        proof,
        AclError::PublicDecryptProofInvalid,
    )
}

fn verify_leaf(
    value: &EncryptedValue,
    commitment: [u8; 32],
    proof: &MmrProof,
    invalid: AclError,
) -> Result<(), AclError> {
    if mmr_verify(&value.peaks, value.leaf_count, commitment, proof) {
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
        assert_eq!(EncryptedValue::account_size(0), 181);
        assert_eq!(
            EncryptedValue::account_size(MAX_MMR_PEAKS),
            2229,
            "max account must stay « Solana's 10240-byte realloc cap"
        );
    }

    /// A test encrypted value account that maintains its own leaf list so it can build proofs,
    /// mirroring the coprocessor indexer.
    #[derive(Default)]
    struct EncryptedValueAccount {
        value: EncryptedValue,
        account: [u8; 32],
        leaves: Vec<[u8; 32]>,
    }

    impl EncryptedValueAccount {
        fn new(handle: [u8; 32]) -> Self {
            Self {
                value: EncryptedValue {
                    current_handle: handle,
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
            self.value.current_handle = new_handle;
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
            let c = public_decrypt_leaf_commitment(self.account, idx, self.value.current_handle);
            self.append(c);
        }

        fn proof(&self, i: u64) -> MmrProof {
            mmr_build_proof(&self.leaves, i).unwrap()
        }
    }

    #[test]
    fn a_leaf_authorizes_its_key_on_its_handle_only() {
        let owner = h(1);
        let mut l = EncryptedValueAccount::new(h(10));
        l.update(h(10), &[owner]);
        l.update(h(11), &[owner]);
        let old = l.proof(0);
        let new = l.proof(1);
        assert!(authorize_historical(l.account, &l.value, h(10), owner, &old).is_ok());
        assert!(authorize_historical(l.account, &l.value, h(11), owner, &new).is_ok());
        assert!(authorize_historical(l.account, &l.value, h(10), h(2), &old).is_err());
        assert!(authorize_historical(l.account, &l.value, h(11), owner, &old).is_err());
    }

    #[test]
    fn exact_public_no_roll_forward() {
        let mut l = EncryptedValueAccount::new(h(10));
        l.make_public();
        l.update(h(11), &[]);
        let proof = l.proof(0);
        assert!(authorize_public(l.account, &l.value, h(10), &proof).is_ok());
        assert_eq!(
            authorize_public(l.account, &l.value, h(11), &proof),
            Err(AclError::PublicDecryptProofInvalid)
        );
    }

    #[test]
    fn a_value_nobody_was_allowed_on_cannot_be_decrypted() {
        let l = EncryptedValueAccount::new(h(10));
        assert_eq!(l.value.leaf_count, 0);
        let empty = MmrProof::default();
        assert!(authorize_historical(l.account, &l.value, h(10), h(1), &empty).is_err());
        assert!(authorize_public(l.account, &l.value, h(10), &empty).is_err());
    }

    #[test]
    fn on_chain_account_decoder_reads_layout() {
        let mut l = EncryptedValueAccount::new(h(10));
        l.value.program = h(0x50);
        l.value.encrypted_value_account_authority = h(0x51);
        l.value.scope = h(0x52);
        l.value.label = h(0x53);
        l.value.bump = 254;
        l.update(h(10), &[h(1), h(2)]);
        l.make_public();
        let mut data = encrypted_value_discriminator().to_vec();
        data.extend_from_slice(&borsh::to_vec(&l.value).unwrap());
        assert_eq!(
            data.len(),
            EncryptedValue::account_size(l.value.peaks.len())
        );

        let decoded = decode_on_chain_account(&data).unwrap();
        assert_eq!(decoded, l.value);
        assert_eq!(
            decoded.seeds(),
            [
                ENCRYPTED_VALUE_SEED,
                &h(0x50)[..],
                &h(0x51)[..],
                &h(0x52)[..],
                &h(0x53)[..]
            ]
        );
        data[0] ^= 1;
        assert_eq!(
            decode_on_chain_account(&data),
            Err(AclError::BadDiscriminator)
        );
    }
}
