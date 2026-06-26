//! KMS-side encrypted-value ACL verification.
//!
//! The layout, MMR, leaf commitments, and authorization rules come from the
//! shared `zama_solana_acl` crate — the same code the on-chain program runs — so
//! the two cannot drift. The KMS fetches the single live lineage account
//! (deterministic PDA, finalized commitment), decodes it with the shared codec,
//! and verifies the request-carried proof against the peaks. No history indexing.

use solana_pubkey::Pubkey;

use zama_solana_acl::{
    AclError, ENCRYPTED_VALUE_ACL_SEED, EncryptedValueAcl, MmrProof, authorize_current,
    authorize_historical, authorize_public, decode_account,
};

use super::solana_acl::{
    HandleBytes, SolanaAclVerificationError, SolanaAclVerifier, SolanaPubkeyBytes,
};

/// Canonical lineage PDA for a value key under `host_program_id`.
pub fn encrypted_value_acl_address(
    host_program_id: SolanaPubkeyBytes,
    value_key: [u8; 32],
) -> (SolanaPubkeyBytes, u8) {
    let program_id = Pubkey::new_from_array(host_program_id);
    let (address, bump) =
        Pubkey::find_program_address(&[ENCRYPTED_VALUE_ACL_SEED, value_key.as_ref()], &program_id);
    (address.to_bytes(), bump)
}

/// Decodes a fetched lineage account with the shared codec.
pub fn decode_encrypted_value_acl(
    data: &[u8],
) -> Result<EncryptedValueAcl, SolanaAclVerificationError> {
    decode_account(data).map_err(map_acl_error)
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

/// The fetched lineage account plus the handle a request wants to decrypt against it.
/// Groups the inputs common to the historical and public MMR-proof paths so the verifier
/// methods stay below the argument-count lint without a suppression.
pub struct EncryptedValueTarget<'a> {
    pub account_key: SolanaPubkeyBytes,
    pub owner: SolanaPubkeyBytes,
    pub acl: &'a EncryptedValueAcl,
    pub encrypted_value: HandleBytes,
}

impl SolanaAclVerifier {
    /// Owner + canonical-PDA checks shared by every encrypted-value path.
    fn verify_canonical(
        &self,
        account_key: SolanaPubkeyBytes,
        owner: SolanaPubkeyBytes,
        acl: &EncryptedValueAcl,
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

    /// Current decrypt: live handle + membership, within the request's domain scope.
    pub fn verify_current_user_decrypt(
        &self,
        account_key: SolanaPubkeyBytes,
        owner: SolanaPubkeyBytes,
        acl: &EncryptedValueAcl,
        handle: HandleBytes,
        subject: SolanaPubkeyBytes,
        allowed_acl_domain_keys: &[SolanaPubkeyBytes],
    ) -> Result<(), SolanaAclVerificationError> {
        self.verify_canonical(account_key, owner, acl)?;
        if !allowed_acl_domain_keys.contains(&acl.acl_domain_key) {
            return Err(SolanaAclVerificationError::DomainNotAllowed);
        }
        authorize_current(acl, handle, subject).map_err(map_acl_error)
    }

    /// Historical decrypt: a valid historical-access MMR proof carried on the request.
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

    /// Exact public decrypt: a valid public-decrypt MMR proof for the exact handle.
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
        MAX_ENCRYPTED_VALUE_SUBJECTS, acl_nonce_key, encode_account,
        historical_access_leaf_commitment, mmr_append, mmr_build_proof,
        public_decrypt_leaf_commitment,
    };

    const HOST: SolanaPubkeyBytes = [42; 32];
    const DOMAIN: SolanaPubkeyBytes = [1; 32];
    const APP: SolanaPubkeyBytes = [2; 32];
    const OWNER: SolanaPubkeyBytes = [3; 32];
    const STRANGER: SolanaPubkeyBytes = [4; 32];
    const LABEL: [u8; 32] = *b"balance_________________________";

    fn h(tag: u8) -> HandleBytes {
        [tag; 32]
    }

    /// A lineage whose account bytes and proofs are produced by the shared crate.
    struct Lineage {
        acl: EncryptedValueAcl,
        account: SolanaPubkeyBytes,
        leaves: Vec<[u8; 32]>,
    }

    fn lineage(handle: HandleBytes, subjects: &[SolanaPubkeyBytes]) -> Lineage {
        let value_key = acl_nonce_key(DOMAIN, APP, LABEL);
        let (account, bump) = encrypted_value_acl_address(HOST, value_key);
        Lineage {
            acl: EncryptedValueAcl {
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

    #[test]
    fn current_and_rejections() {
        let l = lineage(h(10), &[OWNER]);
        let v = verifier();
        assert!(
            v.verify_current_user_decrypt(l.account, HOST, &l.acl, h(10), OWNER, &[DOMAIN])
                .is_ok()
        );
        assert_eq!(
            v.verify_current_user_decrypt(l.account, HOST, &l.acl, h(10), STRANGER, &[DOMAIN])
                .unwrap_err(),
            SolanaAclVerificationError::EncryptedValueSubjectMissing
        );
        assert_eq!(
            v.verify_current_user_decrypt(l.account, HOST, &l.acl, h(10), OWNER, &[[9; 32]])
                .unwrap_err(),
            SolanaAclVerificationError::DomainNotAllowed
        );
        assert_eq!(
            v.verify_current_user_decrypt(l.account, [7; 32], &l.acl, h(10), OWNER, &[DOMAIN])
                .unwrap_err(),
            SolanaAclVerificationError::InvalidAccountOwner
        );
    }

    #[test]
    fn rejects_non_canonical_acl_account() {
        // A correctly-shaped lineage presented under a NON-canonical account key (not the PDA
        // derived from its own value_key) is rejected before any authorization.
        let l = lineage(h(10), &[OWNER]);
        let wrong_account: SolanaPubkeyBytes = [0xab; 32];
        assert_ne!(wrong_account, l.account);
        assert_eq!(
            verifier()
                .verify_current_user_decrypt(wrong_account, HOST, &l.acl, h(10), OWNER, &[DOMAIN])
                .unwrap_err(),
            SolanaAclVerificationError::NonCanonicalEncryptedValueAcl
        );
    }

    #[test]
    fn rejects_bump_mismatch() {
        // The account key is canonical but the stored bump differs from the derived bump — the
        // account is not the canonical PDA it claims to be.
        let mut l = lineage(h(10), &[OWNER]);
        l.acl.bump ^= 1;
        assert_eq!(
            verifier()
                .verify_current_user_decrypt(l.account, HOST, &l.acl, h(10), OWNER, &[DOMAIN])
                .unwrap_err(),
            SolanaAclVerificationError::EncryptedValueAclBumpMismatch
        );
    }

    #[test]
    fn max_subjects_historical_proof_round_trip() {
        // A lineage at the subject cap: one rotation appends one historical leaf per subject, so an
        // MMR proof for the last subject's leaf must still verify against the live peaks.
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
        assert_eq!(
            v.verify_current_user_decrypt(l.account, HOST, &l.acl, h(10), OWNER, &[DOMAIN])
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

    #[test]
    fn decodes_shared_account_and_authorizes_end_to_end() {
        let mut l = lineage(h(10), &[OWNER]);
        l.rotate(h(11));
        let data = encode_account(&l.acl).unwrap();
        let decoded = decode_encrypted_value_acl(&data).unwrap();
        assert_eq!(decoded, l.acl);
        let proof = l.proof(0);
        let target = EncryptedValueTarget {
            account_key: l.account,
            owner: HOST,
            acl: &decoded,
            encrypted_value: h(10),
        };
        assert!(
            verifier()
                .verify_historical_user_decrypt(target, OWNER, &[DOMAIN], &proof)
                .is_ok()
        );
    }
}
