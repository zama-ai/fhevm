//! KMS-side `EncryptedValue` ACL verification (RFC-024 MMR rewrite).
//!
//! The MMR, leaf commitments, and authorization rules come from the shared
//! `zama_solana_acl` crate — the same code the on-chain program runs — so the KMS
//! and the host cannot drift on those. The account **layout** includes host-only
//! role bytes, so this module decodes the real on-chain shape before projecting it
//! to the shared type. See "WHY A LOCAL DECODE ROUTINE" below.
//!
//! ## WHY A LOCAL DECODE ROUTINE (read before touching this file)
//!
//! `zama_solana_acl::EncryptedValue` (the shared crate's wire type) is:
//! `acl_domain_key, app_account, encrypted_value_label, current_handle, subjects,
//! leaf_count, peaks, bump`.
//!
//! The **actual on-chain account**, `zama-host`'s `state::encrypted_value::EncryptedValue`
//! (an Anchor `#[account]`), interleaves one extra Borsh field, `subject_roles:
//! Vec<u8>`, immediately after `subjects` and before `leaf_count`:
//! `acl_domain_key, app_account, encrypted_value_label, current_handle, subjects,
//! subject_roles, leaf_count, peaks, bump`.
//!
//! Borsh has no field tags — it is decoded strictly positionally — so this module
//! locally decodes the REAL on-chain byte layout (mirroring `zama-host`'s own
//! `EncryptedValue::to_shared`,
//! which performs the identical projection on-chain), preserves `subject_roles`
//! alongside the projected `zama_solana_acl::EncryptedValue`, and locally enforces
//! the host's current-decrypt USE-role policy before/after delegating to the shared
//! crate's role-less authorization helpers. Historical/public MMR authorization
//! still stays entirely in the shared crate because the leaf commitments already
//! encode the authorized subject/handle history.

use borsh::BorshDeserialize;
use solana_pubkey::Pubkey;

use zama_solana_acl::{
    AclError, ENCRYPTED_VALUE_SEED, EncryptedValue, MmrProof, authorize_current,
    authorize_historical, authorize_public, encrypted_value_discriminator,
};

use super::solana_acl::{
    HandleBytes, SolanaAclVerificationError, SolanaAclVerifier, SolanaPubkeyBytes,
};

/// Mirrors `ACL_ROLE_USE` from `solana/programs/zama-host/src/constants.rs`.
const ACL_ROLE_USE: u8 = 0x01;

/// Byte-exact mirror of `zama-host`'s on-chain `EncryptedValue` account body (i.e. its Borsh
/// layout, discriminator excluded) — see the module doc for why this must NOT be
/// `zama_solana_acl::EncryptedValue`. Kept `pub(crate)` — only [`decode_encrypted_value_acl`]
/// should ever construct one; everything downstream uses the shared crate's projected type.
#[derive(BorshDeserialize, borsh::BorshSerialize, Clone, Debug, PartialEq, Eq)]
struct OnChainEncryptedValue {
    acl_domain_key: [u8; 32],
    app_account: [u8; 32],
    encrypted_value_label: [u8; 32],
    current_handle: [u8; 32],
    subjects: Vec<[u8; 32]>,
    /// Role flags parallel to `subjects`. Host-program-only policy, preserved next to the
    /// projected shared ACL/MMR type; see the module doc.
    subject_roles: Vec<u8>,
    leaf_count: u64,
    peaks: Vec<[u8; 32]>,
    bump: u8,
}

/// KMS-local decoded lineage: the shared ACL/MMR state plus host-only role bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecodedEncryptedValueAcl {
    pub acl: EncryptedValue,
    pub subject_roles: Vec<u8>,
}

impl OnChainEncryptedValue {
    /// Projects to the shared crate's wire type while preserving host-only roles next to it.
    fn to_decoded(&self) -> Result<DecodedEncryptedValueAcl, SolanaAclVerificationError> {
        if self.subject_roles.len() != self.subjects.len() {
            return Err(SolanaAclVerificationError::InvalidAccountData);
        }
        Ok(DecodedEncryptedValueAcl {
            acl: EncryptedValue {
                acl_domain_key: self.acl_domain_key,
                app_account: self.app_account,
                encrypted_value_label: self.encrypted_value_label,
                current_handle: self.current_handle,
                subjects: self.subjects.clone(),
                leaf_count: self.leaf_count,
                peaks: self.peaks.clone(),
                bump: self.bump,
            },
            subject_roles: self.subject_roles.clone(),
        })
    }
}

impl DecodedEncryptedValueAcl {
    #[cfg(test)]
    fn from_parts(acl: EncryptedValue, subject_roles: Vec<u8>) -> Self {
        Self { acl, subject_roles }
    }

    fn subject_role(&self, subject: SolanaPubkeyBytes) -> Option<u8> {
        self.acl
            .subjects
            .iter()
            .position(|candidate| *candidate == subject)
            .and_then(|index| self.subject_roles.get(index).copied())
    }
}

/// Canonical lineage PDA for a value key under `host_program_id`.
pub fn encrypted_value_acl_address(
    host_program_id: SolanaPubkeyBytes,
    value_key: [u8; 32],
) -> (SolanaPubkeyBytes, u8) {
    let program_id = Pubkey::new_from_array(host_program_id);
    let (address, bump) =
        Pubkey::find_program_address(&[ENCRYPTED_VALUE_SEED, value_key.as_ref()], &program_id);
    (address.to_bytes(), bump)
}

/// Decodes a fetched `EncryptedValue` lineage account using the REAL on-chain layout (see the
/// module doc), then projects to the shared crate's `EncryptedValue` for authorization.
pub fn decode_encrypted_value_acl(
    data: &[u8],
) -> Result<DecodedEncryptedValueAcl, SolanaAclVerificationError> {
    if data.len() < 8 || data[..8] != encrypted_value_discriminator() {
        return Err(map_acl_error(AclError::BadDiscriminator));
    }
    let mut body = &data[8..];
    let decoded = OnChainEncryptedValue::deserialize(&mut body)
        .map_err(|_| map_acl_error(AclError::BadAccountData))?;
    decoded.to_decoded()
}

fn map_acl_error(error: AclError) -> SolanaAclVerificationError {
    match error {
        AclError::HandleMismatch => SolanaAclVerificationError::EncryptedValueHandleMismatch,
        AclError::SubjectMissing => SolanaAclVerificationError::EncryptedValueSubjectMissing,
        AclError::HistoricalProofInvalid => {
            SolanaAclVerificationError::HistoricalAccessProofInvalid
        }
        AclError::PublicDecryptProofInvalid => {
            SolanaAclVerificationError::PublicDecryptProofInvalid
        }
        AclError::MmrInconsistent => SolanaAclVerificationError::MmrStateInconsistent,
        AclError::BadDiscriminator
        | AclError::BadAccountData
        | AclError::SubjectCapacityExceeded => SolanaAclVerificationError::InvalidAccountData,
    }
}

/// The fetched lineage account plus the handle a request wants to decrypt against it. Groups the
/// inputs common to the historical and public MMR-proof paths so the verifier methods stay below
/// the argument-count lint without a suppression.
pub struct EncryptedValueTarget<'a> {
    pub account_key: SolanaPubkeyBytes,
    pub owner: SolanaPubkeyBytes,
    pub acl: &'a EncryptedValue,
    pub encrypted_value: HandleBytes,
}

impl SolanaAclVerifier {
    /// Owner + canonical-PDA checks shared by every encrypted-value path.
    fn verify_canonical(
        &self,
        account_key: SolanaPubkeyBytes,
        owner: SolanaPubkeyBytes,
        acl: &EncryptedValue,
    ) -> Result<(), SolanaAclVerificationError> {
        if owner != self.host_program_id {
            return Err(SolanaAclVerificationError::InvalidAccountOwner);
        }
        let value_key = acl.value_key();
        let (expected, bump) = encrypted_value_acl_address(self.host_program_id, value_key);
        if account_key != expected {
            return Err(SolanaAclVerificationError::NonCanonicalEncryptedValueAcl);
        }
        if acl.bump != bump {
            return Err(SolanaAclVerificationError::EncryptedValueAclBumpMismatch);
        }
        Ok(())
    }

    /// Current decrypt: live handle + membership, within the request's domain scope. Reads the
    /// account fetched at `finalized` commitment — never a snapshot.
    pub fn verify_current_user_decrypt(
        &self,
        account_key: SolanaPubkeyBytes,
        owner: SolanaPubkeyBytes,
        decoded: &DecodedEncryptedValueAcl,
        handle: HandleBytes,
        subject: SolanaPubkeyBytes,
        allowed_acl_domain_keys: &[SolanaPubkeyBytes],
    ) -> Result<(), SolanaAclVerificationError> {
        let acl = &decoded.acl;
        self.verify_canonical(account_key, owner, acl)?;
        if !allowed_acl_domain_keys.contains(&acl.acl_domain_key) {
            return Err(SolanaAclVerificationError::DomainNotAllowed);
        }
        authorize_current(acl, handle, subject).map_err(map_acl_error)?;
        let role = decoded
            .subject_role(subject)
            .ok_or(SolanaAclVerificationError::EncryptedValueSubjectMissing)?;
        if role & ACL_ROLE_USE != ACL_ROLE_USE {
            return Err(SolanaAclVerificationError::EncryptedValueSubjectUseRoleMissing);
        }
        Ok(())
    }

    /// Historical decrypt: a valid historical-access MMR proof against the LIVE finalized peaks
    /// (the account passed in, read fresh — not a cached/snapshotted proof-time state).
    pub fn verify_historical_user_decrypt(
        &self,
        target: EncryptedValueTarget<'_>,
        subject: SolanaPubkeyBytes,
        allowed_acl_domain_keys: &[SolanaPubkeyBytes],
        proof: &MmrProof,
    ) -> Result<(), SolanaAclVerificationError> {
        self.verify_canonical(target.account_key, target.owner, target.acl)?;
        if !allowed_acl_domain_keys.contains(&target.acl.acl_domain_key) {
            return Err(SolanaAclVerificationError::DomainNotAllowed);
        }
        authorize_historical(
            target.account_key,
            target.acl,
            target.encrypted_value,
            subject,
            proof,
        )
        .map_err(map_acl_error)
    }

    /// Exact public decrypt: a valid public-decrypt MMR proof for the exact handle, against the
    /// LIVE finalized peaks. There is no live "is_public" flag — public-ness is only provable via
    /// a `PublicDecryptLeaf` MMR leaf.
    pub fn verify_public_decrypt_exact(
        &self,
        target: EncryptedValueTarget<'_>,
        proof: &MmrProof,
    ) -> Result<(), SolanaAclVerificationError> {
        self.verify_canonical(target.account_key, target.owner, target.acl)?;
        authorize_public(
            target.account_key,
            target.acl,
            target.encrypted_value,
            proof,
        )
        .map_err(map_acl_error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zama_solana_acl::{
        MAX_ENCRYPTED_VALUE_SUBJECTS, derive_value_key, historical_access_leaf_commitment,
        mmr_append, mmr_build_proof, public_decrypt_leaf_commitment,
    };

    const HOST: SolanaPubkeyBytes = [42; 32];
    const DOMAIN: SolanaPubkeyBytes = [1; 32];
    const APP: SolanaPubkeyBytes = [2; 32];
    const OWNER: SolanaPubkeyBytes = [3; 32];
    const STRANGER: SolanaPubkeyBytes = [4; 32];
    const ACL_ROLE_GRANT: u8 = 0x02;
    const LABEL: [u8; 32] = *b"balance_________________________";

    fn h(tag: u8) -> HandleBytes {
        [tag; 32]
    }

    /// A lineage whose account bytes and proofs are produced by the shared crate.
    struct Lineage {
        acl: EncryptedValue,
        account: SolanaPubkeyBytes,
        leaves: Vec<[u8; 32]>,
    }

    fn lineage(handle: HandleBytes, subjects: &[SolanaPubkeyBytes]) -> Lineage {
        let value_key = derive_value_key(DOMAIN, APP, LABEL);
        let (account, bump) = encrypted_value_acl_address(HOST, value_key);
        Lineage {
            acl: EncryptedValue {
                acl_domain_key: DOMAIN,
                app_account: APP,
                encrypted_value_label: LABEL,
                current_handle: handle,
                subjects: subjects.to_vec(),
                leaf_count: 0,
                peaks: Vec::new(),
                bump,
            },
            account,
            leaves: Vec::new(),
        }
    }

    impl Lineage {
        fn append(&mut self, commitment: [u8; 32]) {
            mmr_append(&mut self.acl.peaks, &mut self.acl.leaf_count, commitment).unwrap();
            self.leaves.push(commitment);
        }
        fn rotate(&mut self, new_handle: HandleBytes) {
            let old = self.acl.current_handle;
            for i in 0..self.acl.subjects.len() {
                let idx = self.acl.leaf_count;
                self.append(historical_access_leaf_commitment(
                    self.account,
                    idx,
                    old,
                    self.acl.subjects[i],
                ));
            }
            self.acl.current_handle = new_handle;
        }
        fn mark_public(&mut self) {
            let idx = self.acl.leaf_count;
            self.append(public_decrypt_leaf_commitment(
                self.account,
                idx,
                self.acl.current_handle,
            ));
        }
        fn proof(&self, i: u64) -> MmrProof {
            mmr_build_proof(&self.leaves, i).unwrap()
        }
    }

    fn verifier() -> SolanaAclVerifier {
        SolanaAclVerifier::new(HOST)
    }

    fn with_roles(l: &Lineage, subject_roles: Vec<u8>) -> DecodedEncryptedValueAcl {
        DecodedEncryptedValueAcl::from_parts(l.acl.clone(), subject_roles)
    }

    fn use_roles(l: &Lineage) -> DecodedEncryptedValueAcl {
        with_roles(l, vec![ACL_ROLE_USE; l.acl.subjects.len()])
    }

    /// Encodes a lineage using the REAL on-chain layout (with `subject_roles`), exactly as
    /// `zama-host` would write it. Exercises the local decode routine this module exists for.
    fn encode_on_chain(acl: &EncryptedValue, subject_roles: Vec<u8>) -> Vec<u8> {
        let on_chain = OnChainEncryptedValue {
            acl_domain_key: acl.acl_domain_key,
            app_account: acl.app_account,
            encrypted_value_label: acl.encrypted_value_label,
            current_handle: acl.current_handle,
            subjects: acl.subjects.clone(),
            subject_roles,
            leaf_count: acl.leaf_count,
            peaks: acl.peaks.clone(),
            bump: acl.bump,
        };
        let mut data = encrypted_value_discriminator().to_vec();
        data.extend_from_slice(&borsh::to_vec(&on_chain).unwrap());
        data
    }

    #[test]
    fn current_and_rejections() {
        let l = lineage(h(10), &[OWNER]);
        let decoded = use_roles(&l);
        let v = verifier();
        assert!(
            v.verify_current_user_decrypt(l.account, HOST, &decoded, h(10), OWNER, &[DOMAIN])
                .is_ok()
        );
        assert_eq!(
            v.verify_current_user_decrypt(l.account, HOST, &decoded, h(10), STRANGER, &[DOMAIN])
                .unwrap_err(),
            SolanaAclVerificationError::EncryptedValueSubjectMissing
        );
        assert_eq!(
            v.verify_current_user_decrypt(l.account, HOST, &decoded, h(10), OWNER, &[[9; 32]])
                .unwrap_err(),
            SolanaAclVerificationError::DomainNotAllowed
        );
        assert_eq!(
            v.verify_current_user_decrypt(l.account, [7; 32], &decoded, h(10), OWNER, &[DOMAIN])
                .unwrap_err(),
            SolanaAclVerificationError::InvalidAccountOwner
        );
    }

    #[test]
    fn current_requires_use_role() {
        let l = lineage(h(10), &[OWNER]);
        let grant_only = with_roles(&l, vec![ACL_ROLE_GRANT]);
        assert_eq!(
            verifier()
                .verify_current_user_decrypt(l.account, HOST, &grant_only, h(10), OWNER, &[DOMAIN])
                .unwrap_err(),
            SolanaAclVerificationError::EncryptedValueSubjectUseRoleMissing
        );

        let use_subject = use_roles(&l);
        verifier()
            .verify_current_user_decrypt(l.account, HOST, &use_subject, h(10), OWNER, &[DOMAIN])
            .expect("USE-role subject must decrypt the current handle");
    }

    #[test]
    fn rejects_non_canonical_acl_account() {
        let l = lineage(h(10), &[OWNER]);
        let decoded = use_roles(&l);
        let wrong_account: SolanaPubkeyBytes = [0xab; 32];
        assert_ne!(wrong_account, l.account);
        assert_eq!(
            verifier()
                .verify_current_user_decrypt(wrong_account, HOST, &decoded, h(10), OWNER, &[DOMAIN])
                .unwrap_err(),
            SolanaAclVerificationError::NonCanonicalEncryptedValueAcl
        );
    }

    #[test]
    fn rejects_bump_mismatch() {
        let mut l = lineage(h(10), &[OWNER]);
        l.acl.bump ^= 1;
        let decoded = use_roles(&l);
        assert_eq!(
            verifier()
                .verify_current_user_decrypt(l.account, HOST, &decoded, h(10), OWNER, &[DOMAIN])
                .unwrap_err(),
            SolanaAclVerificationError::EncryptedValueAclBumpMismatch
        );
    }

    #[test]
    fn max_subjects_historical_proof_round_trip() {
        let subjects: Vec<SolanaPubkeyBytes> = (0..MAX_ENCRYPTED_VALUE_SUBJECTS as u8)
            .map(|i| [i + 1; 32])
            .collect();
        let mut l = lineage(h(10), &subjects);
        l.rotate(h(11));
        assert_eq!(l.acl.leaf_count, MAX_ENCRYPTED_VALUE_SUBJECTS as u64);

        let last = MAX_ENCRYPTED_VALUE_SUBJECTS - 1;
        let target = EncryptedValueTarget {
            account_key: l.account,
            owner: HOST,
            acl: &l.acl,
            encrypted_value: h(10),
        };
        verifier()
            .verify_historical_user_decrypt(
                target,
                subjects[last],
                &[DOMAIN],
                &l.proof(last as u64),
            )
            .expect("historical proof for the cap-th subject must verify");
    }

    #[test]
    fn post_rotation_then_historical_proof() {
        let mut l = lineage(h(10), &[OWNER]);
        l.rotate(h(11));
        let v = verifier();
        let decoded = use_roles(&l);
        assert_eq!(
            v.verify_current_user_decrypt(l.account, HOST, &decoded, h(10), OWNER, &[DOMAIN])
                .unwrap_err(),
            SolanaAclVerificationError::EncryptedValueHandleMismatch
        );
        let proof = l.proof(0);
        let target = |handle| EncryptedValueTarget {
            account_key: l.account,
            owner: HOST,
            acl: &l.acl,
            encrypted_value: handle,
        };
        assert!(
            v.verify_historical_user_decrypt(target(h(10)), OWNER, &[DOMAIN], &proof)
                .is_ok()
        );
        assert!(
            v.verify_historical_user_decrypt(target(h(10)), STRANGER, &[DOMAIN], &proof)
                .is_err()
        );
        assert!(
            v.verify_historical_user_decrypt(target(h(99)), OWNER, &[DOMAIN], &proof)
                .is_err()
        );
    }

    #[test]
    fn historical_proof_path_stays_roleless() {
        let mut l = lineage(h(10), &[OWNER]);
        l.rotate(h(11));
        let data = encode_on_chain(&l.acl, vec![ACL_ROLE_GRANT]);
        let decoded = decode_encrypted_value_acl(&data).unwrap();

        assert_eq!(
            verifier()
                .verify_current_user_decrypt(l.account, HOST, &decoded, h(11), OWNER, &[DOMAIN])
                .unwrap_err(),
            SolanaAclVerificationError::EncryptedValueSubjectUseRoleMissing
        );

        let target = EncryptedValueTarget {
            account_key: l.account,
            owner: HOST,
            acl: &decoded.acl,
            encrypted_value: h(10),
        };
        verifier()
            .verify_historical_user_decrypt(target, OWNER, &[DOMAIN], &l.proof(0))
            .expect("historical proof verification must stay unchanged");
    }

    #[test]
    fn exact_public_no_roll_forward() {
        let mut l = lineage(h(10), &[OWNER]);
        l.mark_public();
        l.rotate(h(11));
        let v = verifier();
        let proof = l.proof(0);
        let target = |handle| EncryptedValueTarget {
            account_key: l.account,
            owner: HOST,
            acl: &l.acl,
            encrypted_value: handle,
        };
        assert!(v.verify_public_decrypt_exact(target(h(10)), &proof).is_ok());
        assert_eq!(
            v.verify_public_decrypt_exact(target(h(11)), &proof)
                .unwrap_err(),
            SolanaAclVerificationError::PublicDecryptProofInvalid
        );
    }

    /// The load-bearing test for this module: decoding the REAL on-chain layout (with
    /// `subject_roles` inserted between `subjects` and `leaf_count`) must recover exactly the
    /// same ACL/MMR state as the shared crate's in-memory value.
    #[test]
    fn decodes_real_on_chain_layout_with_subject_roles_and_authorizes_end_to_end() {
        let mut l = lineage(h(10), &[OWNER]);
        l.rotate(h(11));
        let subject_roles = vec![0x01u8; l.acl.subjects.len()];
        let data = encode_on_chain(&l.acl, subject_roles);

        let decoded = decode_encrypted_value_acl(&data).unwrap();
        assert_eq!(
            decoded.acl, l.acl,
            "decoded ACL/MMR state must match the shared in-memory value"
        );
        assert_eq!(decoded.subject_roles, vec![ACL_ROLE_USE]);

        let proof = l.proof(0);
        let target = EncryptedValueTarget {
            account_key: l.account,
            owner: HOST,
            acl: &decoded.acl,
            encrypted_value: h(10),
        };
        assert!(
            verifier()
                .verify_historical_user_decrypt(target, OWNER, &[DOMAIN], &proof)
                .is_ok()
        );
    }
}
