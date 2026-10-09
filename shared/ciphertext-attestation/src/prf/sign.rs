//! Canonical encoding, signing, and verification for PRF output attestations.
//!
//! The signed payload reproduces `keccak256(abi.encodePacked(...))` from
//! RFC-038. All fields are fixed-width, so direct byte concatenation is
//! byte-identical to packed ABI encoding.
//!
//! Signing uses raw prehash (`Signer::sign_hash`) over the keccak of the
//! canonical bytes. The `bytes8("FHEVMPRF")` domain tag inside the payload
//! separates it from RFC-023 ciphertext attestations.

use crate::{
    AttestationError, PrfMaterial, PrfOutputAttestation, PrfOutputAttestationPayload, PrfOutputRef,
    consensus::Attestation,
    keccak_b256,
    prf::{PRF_DOMAIN_TAG, Version},
};
use alloy_primitives::{Address, B256, Signature};
use alloy_signer::Signer;

/// V1 canonical-bytes length: `bytes8 + uint8 + uint16 + bytes32 + bytes32 = 75`.
const V1_PAYLOAD_LEN: usize = 8 + 1 + 2 + 32 + 32;

impl PrfOutputAttestationPayload {
    /// Canonical `abi.encodePacked`-equivalent bytes for this payload. The
    /// exact message that gets keccak'd and signed.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        match self.version {
            Version::V1 => {
                let mut out = Vec::with_capacity(V1_PAYLOAD_LEN);
                out.extend_from_slice(&PRF_DOMAIN_TAG);
                out.push(self.version as u8);
                out.extend_from_slice(&self.prf_id.to_be_bytes());
                out.extend_from_slice(self.label.as_slice());
                out.extend_from_slice(self.digest.as_slice());
                out
            }
        }
    }

    /// Keccak-256 of [`Self::canonical_bytes`]: the prehash the signer signs.
    pub fn canonical_digest(&self) -> B256 {
        keccak_b256(&self.canonical_bytes())
    }

    /// Consume the payload and produce a signed [`PrfOutputAttestation`].
    /// `prf_id` and `label` are bound by the signature but stripped from the
    /// resulting wire form.
    pub async fn sign<S: Signer + Sync>(
        self,
        signer: &S,
    ) -> Result<PrfOutputAttestation, AttestationError> {
        let sig = signer.sign_hash(&self.canonical_digest()).await?;
        Ok(PrfOutputAttestation {
            version: self.version,
            digest: self.digest,
            signer: signer.address(),
            signature: sig.as_bytes().to_vec(),
        })
    }
}

impl Attestation for PrfOutputAttestation {
    type Subject = PrfOutputRef;
    type Material = PrfMaterial;

    /// Verifies that this attestation was signed by `expected_signer` over the subject's `prf_id`
    /// and `label`.
    fn verify(
        &self,
        subject: &PrfOutputRef,
        expected_signer: Address,
    ) -> Result<(), AttestationError> {
        if self.signer != expected_signer {
            return Err(AttestationError::UnexpectedSigner {
                claimed: self.signer,
                expected: expected_signer,
            });
        }
        let payload = PrfOutputAttestationPayload {
            version: self.version,
            prf_id: subject.prf_id,
            label: subject.label,
            digest: self.digest,
        };
        let digest = payload.canonical_digest();

        let sig = Signature::try_from(self.signature.as_slice())
            .map_err(|e| AttestationError::MalformedSignature(e.to_string()))?;
        let recovered = sig
            .recover_address_from_prehash(&digest)
            .map_err(|e| AttestationError::Recovery(e.to_string()))?;
        if recovered != self.signer {
            return Err(AttestationError::SignerMismatch {
                recovered,
                claimed: self.signer,
            });
        }
        Ok(())
    }

    fn material(&self) -> PrfMaterial {
        self.digest
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::b256;
    use alloy_signer_local::PrivateKeySigner;

    const PRF_ID: u16 = 3;
    const LABEL: B256 = b256!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    const DIGEST: B256 = b256!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb");

    fn sample_payload() -> PrfOutputAttestationPayload {
        PrfOutputAttestationPayload::new(Version::V1, PRF_ID, LABEL, DIGEST)
    }

    async fn signed(signer: &PrivateKeySigner) -> PrfOutputAttestation {
        sample_payload().sign(signer).await.unwrap()
    }

    fn subject() -> PrfOutputRef {
        PrfOutputRef::new(PRF_ID, LABEL)
    }

    #[test]
    fn v1_canonical_bytes_length() {
        assert_eq!(sample_payload().canonical_bytes().len(), V1_PAYLOAD_LEN);
    }

    #[tokio::test]
    async fn sign_then_verify_round_trip() {
        let signer = PrivateKeySigner::random();
        let att = signed(&signer).await;
        assert_eq!(att.signer, signer.address());
        assert_eq!(att.material(), DIGEST);
        att.verify(&subject(), signer.address()).unwrap();
    }

    #[tokio::test]
    async fn rejects_flipped_digest() {
        let signer = PrivateKeySigner::random();
        let mut att = signed(&signer).await;
        let mut bytes = att.digest.0;
        bytes[0] ^= 0x01;
        att.digest = B256::from(bytes);
        let err = att.verify(&subject(), signer.address()).unwrap_err();
        assert!(matches!(err, AttestationError::SignerMismatch { .. }));
    }

    #[tokio::test]
    async fn rejects_wrong_label() {
        let signer = PrivateKeySigner::random();
        let att = signed(&signer).await;
        let wrong = PrfOutputRef::new(PRF_ID, B256::repeat_byte(0xFF));
        let err = att.verify(&wrong, signer.address()).unwrap_err();
        assert!(matches!(err, AttestationError::SignerMismatch { .. }));
    }

    #[tokio::test]
    async fn rejects_wrong_prf_id() {
        let signer = PrivateKeySigner::random();
        let att = signed(&signer).await;
        let wrong = PrfOutputRef::new(PRF_ID + 1, LABEL);
        let err = att.verify(&wrong, signer.address()).unwrap_err();
        assert!(matches!(err, AttestationError::SignerMismatch { .. }));
    }

    #[tokio::test]
    async fn rejects_replaced_signer() {
        // The claimed signer is the expected one, but the signature recovers to someone else.
        let signer = PrivateKeySigner::random();
        let impostor = PrivateKeySigner::random();
        let mut att = signed(&impostor).await;
        att.signer = signer.address();
        let err = att.verify(&subject(), signer.address()).unwrap_err();
        assert!(matches!(err, AttestationError::SignerMismatch { .. }));
    }

    #[tokio::test]
    async fn rejects_unexpected_signer() {
        let signer = PrivateKeySigner::random();
        let att = signed(&signer).await;
        let err = att
            .verify(&subject(), PrivateKeySigner::random().address())
            .unwrap_err();
        assert!(matches!(err, AttestationError::UnexpectedSigner { .. }));
    }

    #[tokio::test]
    async fn rejects_bad_signature_length() {
        let signer = PrivateKeySigner::random();
        let mut att = signed(&signer).await;
        att.signature.truncate(60);
        let err = att.verify(&subject(), signer.address()).unwrap_err();
        assert!(matches!(err, AttestationError::MalformedSignature { .. }));
    }

    /// Smoke test of the PRF wiring into the generic round. The counting rules themselves are
    /// tested in `crate::consensus`.
    #[tokio::test]
    async fn consensus_round_on_prf_output() {
        let [s1, s2, s3] = std::array::from_fn(|_| PrivateKeySigner::random());
        let entries = [&s1, &s2, &s3].map(|s| crate::CoprocessorEntry {
            tx_sender: s.address(),
            signer: s.address(),
            bucket: format!("http://bucket-{}", s.address()),
        });
        let mut round = crate::ConsensusRound::<PrfOutputAttestation>::open(
            subject(),
            entries,
            std::num::NonZeroUsize::new(2).unwrap(),
        );

        assert!(
            round
                .record_attestation(s1.address(), &signed(&s1).await)
                .is_none()
        );

        let other_label = PrfOutputAttestationPayload::new(Version::V1, PRF_ID, B256::ZERO, DIGEST)
            .sign(&s2)
            .await
            .unwrap();
        assert!(
            round
                .record_attestation(s2.address(), &other_label)
                .is_none()
        );
        assert_eq!(round.rejected(), vec![s2.address()]);

        match round.record_attestation(s3.address(), &signed(&s3).await) {
            Some(Ok(resolved)) => {
                assert_eq!(resolved.material, DIGEST);
                let winners: Vec<_> = resolved.winners.iter().map(|e| e.signer).collect();
                assert_eq!(winners, vec![s1.address(), s3.address()]);
            }
            other => panic!("expected a reached consensus, got {other:?}"),
        }
    }

    /// Pins the V1 wire encoding against hand-checked hex. Any drift in field
    /// order, endianness, domain tag, or hash function breaks this test loudly.
    #[test]
    fn v1_canonical_bytes_and_digest_pinned() {
        let payload = PrfOutputAttestationPayload::new(
            Version::V1,
            3,
            b256!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
            b256!("1111111111111111111111111111111111111111111111111111111111111111"),
        );
        assert_eq!(
            hex::encode(payload.canonical_bytes()),
            "464845564d505246010003aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa1111111111111111111111111111111111111111111111111111111111111111",
        );
        assert_eq!(
            hex::encode(payload.canonical_digest().as_slice()),
            "03be33dec652f2c59acc73d37a1eebd06eabda43c832f89ecd7df43adcdb7301",
        );
    }
}
