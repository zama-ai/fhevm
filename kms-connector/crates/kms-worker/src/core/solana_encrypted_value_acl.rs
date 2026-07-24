//! KMS-side `EncryptedValue` ACL verification (RFC-024 MMR rewrite).
//!
//! The MMR, leaf commitments, and authorization rules come from the shared
//! `zama_solana_acl` crate — the same code the on-chain program runs — so the KMS
//! and the host cannot drift on those. The account layout now matches the shared
//! crate directly: `subjects` is the complete allowed set.

use solana_pubkey::Pubkey;

use zama_solana_acl::{
    AclError, ENCRYPTED_VALUE_SEED, EncryptedValue, MmrProof, authorize_current,
    authorize_historical, authorize_public, decode_on_chain_account,
};

use super::solana_acl::{
    HandleBytes, SolanaAclVerificationError, SolanaAclVerifier, SolanaPubkeyBytes,
};

/// KMS-local decoded encrypted value account.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecodedEncryptedValueAcl {
    pub acl: EncryptedValue,
}

impl DecodedEncryptedValueAcl {
    #[cfg(test)]
    fn from_acl(acl: EncryptedValue) -> Self {
        Self { acl }
    }
}

/// Canonical encrypted value account PDA for a value key under `host_program_id`.
pub fn encrypted_value_acl_address(
    host_program_id: SolanaPubkeyBytes,
    value_key: [u8; 32],
) -> (SolanaPubkeyBytes, u8) {
    let program_id = Pubkey::new_from_array(host_program_id);
    let (address, bump) =
        Pubkey::find_program_address(&[ENCRYPTED_VALUE_SEED, value_key.as_ref()], &program_id);
    (address.to_bytes(), bump)
}

/// Decodes a fetched `EncryptedValue` encrypted value account.
pub fn decode_encrypted_value_acl(
    data: &[u8],
) -> Result<DecodedEncryptedValueAcl, SolanaAclVerificationError> {
    Ok(DecodedEncryptedValueAcl {
        acl: decode_on_chain_account(data).map_err(map_acl_error)?,
    })
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
        AclError::MmrInconsistent | AclError::MmrPeakCapacityExceeded => {
            SolanaAclVerificationError::MmrStateInconsistent
        }
        AclError::BadDiscriminator
        | AclError::BadAccountData
        | AclError::SubjectCapacityExceeded => SolanaAclVerificationError::InvalidAccountData,
    }
}

/// The fetched encrypted value account plus the handle a request wants to decrypt against it. Groups the
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

    /// Current decrypt: live handle + membership, within the request's domain scope. An empty
    /// domain scope is permissive. Reads the account fetched at `confirmed` commitment — never a
    /// snapshot.
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
        if !allowed_acl_domain_keys.is_empty()
            && !allowed_acl_domain_keys.contains(&acl.acl_domain_key)
        {
            return Err(SolanaAclVerificationError::DomainNotAllowed);
        }
        authorize_current(acl, handle, subject).map_err(map_acl_error)?;
        Ok(())
    }

    /// Historical decrypt: a valid historical-access MMR proof against the live confirmed peaks
    /// (the account passed in, read fresh — not a cached/snapshotted proof-time state). An empty
    /// domain scope is permissive.
    pub fn verify_historical_user_decrypt(
        &self,
        target: EncryptedValueTarget<'_>,
        subject: SolanaPubkeyBytes,
        allowed_acl_domain_keys: &[SolanaPubkeyBytes],
        proof: &MmrProof,
    ) -> Result<(), SolanaAclVerificationError> {
        self.verify_canonical(target.account_key, target.owner, target.acl)?;
        if !allowed_acl_domain_keys.is_empty()
            && !allowed_acl_domain_keys.contains(&target.acl.acl_domain_key)
        {
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
    /// live confirmed peaks. There is no live "is_public" flag — public-ness is only provable via
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
    const LABEL: [u8; 32] = *b"balance_________________________";

    fn h(tag: u8) -> HandleBytes {
        [tag; 32]
    }

    /// A encrypted value account whose account bytes and proofs are produced by the shared crate.
    struct EncryptedValueAccount {
        acl: EncryptedValue,
        account: SolanaPubkeyBytes,
        leaves: Vec<[u8; 32]>,
    }

    fn encrypted_value_account(
        handle: HandleBytes,
        subjects: &[SolanaPubkeyBytes],
    ) -> EncryptedValueAccount {
        let value_key = derive_value_key(DOMAIN, APP, LABEL);
        let (account, bump) = encrypted_value_acl_address(HOST, value_key);
        EncryptedValueAccount {
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

    impl EncryptedValueAccount {
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

    fn decoded(l: &EncryptedValueAccount) -> DecodedEncryptedValueAcl {
        DecodedEncryptedValueAcl::from_acl(l.acl.clone())
    }

    /// Encodes a encrypted value account using the on-chain layout exactly as `zama-host` writes it.
    fn encode_on_chain(acl: &EncryptedValue) -> Vec<u8> {
        let mut data = zama_solana_acl::encrypted_value_discriminator().to_vec();
        data.extend_from_slice(&borsh::to_vec(acl).unwrap());
        data
    }

    #[test]
    fn current_and_rejections() {
        let l = encrypted_value_account(h(10), &[OWNER]);
        let decoded = decoded(&l);
        let v = verifier();
        assert!(
            v.verify_current_user_decrypt(l.account, HOST, &decoded, h(10), OWNER, &[DOMAIN])
                .is_ok()
        );
        assert!(
            v.verify_current_user_decrypt(l.account, HOST, &decoded, h(10), OWNER, &[])
                .is_ok()
        );
        assert_eq!(
            v.verify_current_user_decrypt(l.account, HOST, &decoded, h(10), STRANGER, &[])
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
    fn current_requires_membership() {
        let l = encrypted_value_account(h(10), &[STRANGER]);
        let decoded_stranger = decoded(&l);
        assert_eq!(
            verifier()
                .verify_current_user_decrypt(
                    l.account,
                    HOST,
                    &decoded_stranger,
                    h(10),
                    OWNER,
                    &[DOMAIN],
                )
                .unwrap_err(),
            SolanaAclVerificationError::EncryptedValueSubjectMissing
        );

        let l = encrypted_value_account(h(10), &[OWNER]);
        let decoded = decoded(&l);
        verifier()
            .verify_current_user_decrypt(l.account, HOST, &decoded, h(10), OWNER, &[DOMAIN])
            .expect("member subject must decrypt the current handle");
    }

    #[test]
    fn rejects_non_canonical_acl_account() {
        let l = encrypted_value_account(h(10), &[OWNER]);
        let decoded = decoded(&l);
        let wrong_account: SolanaPubkeyBytes = [0xab; 32];
        assert_ne!(wrong_account, l.account);
        assert_eq!(
            verifier()
                .verify_current_user_decrypt(wrong_account, HOST, &decoded, h(10), OWNER, &[])
                .unwrap_err(),
            SolanaAclVerificationError::NonCanonicalEncryptedValueAcl
        );
    }

    #[test]
    fn rejects_bump_mismatch() {
        let mut l = encrypted_value_account(h(10), &[OWNER]);
        l.acl.bump ^= 1;
        let decoded = decoded(&l);
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
        let mut l = encrypted_value_account(h(10), &subjects);
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
        let mut l = encrypted_value_account(h(10), &[OWNER]);
        l.rotate(h(11));
        let v = verifier();
        let decoded = decoded(&l);
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
            v.verify_historical_user_decrypt(target(h(10)), OWNER, &[], &proof)
                .is_ok()
        );
        assert_eq!(
            v.verify_historical_user_decrypt(target(h(10)), OWNER, &[[9; 32]], &proof)
                .unwrap_err(),
            SolanaAclVerificationError::DomainNotAllowed
        );
        assert_eq!(
            v.verify_historical_user_decrypt(target(h(10)), STRANGER, &[], &proof)
                .unwrap_err(),
            SolanaAclVerificationError::HistoricalAccessProofInvalid
        );
        assert_eq!(
            v.verify_historical_user_decrypt(target(h(99)), OWNER, &[], &proof)
                .unwrap_err(),
            SolanaAclVerificationError::HistoricalAccessProofInvalid
        );
    }

    #[test]
    fn historical_proof_path_uses_sealed_subject_not_current_membership() {
        let mut l = encrypted_value_account(h(10), &[OWNER]);
        l.rotate(h(11));
        l.acl.subjects = vec![STRANGER];
        let data = encode_on_chain(&l.acl);
        let decoded = decode_encrypted_value_acl(&data).unwrap();

        assert_eq!(
            verifier()
                .verify_current_user_decrypt(l.account, HOST, &decoded, h(11), OWNER, &[DOMAIN])
                .unwrap_err(),
            SolanaAclVerificationError::EncryptedValueSubjectMissing
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
        let mut l = encrypted_value_account(h(10), &[OWNER]);
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

    /// The load-bearing test for this module: decoding the real on-chain layout
    /// must recover exactly the same ACL/MMR state as the shared crate's
    /// in-memory value.
    #[test]
    fn decodes_real_on_chain_layout_and_authorizes_end_to_end() {
        let mut l = encrypted_value_account(h(10), &[OWNER]);
        l.rotate(h(11));
        let data = encode_on_chain(&l.acl);

        let decoded = decode_encrypted_value_acl(&data).unwrap();
        assert_eq!(
            decoded.acl, l.acl,
            "decoded ACL/MMR state must match the shared in-memory value"
        );

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
