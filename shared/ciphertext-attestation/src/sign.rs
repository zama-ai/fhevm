//! Canonical encoding, signing, and verification for ciphertext attestations.
//!
//! The signed payload reproduces `keccak256(abi.encodePacked(...))` from
//! RFC-023. All fields are fixed-width, so direct byte concatenation is
//! byte-identical to packed ABI encoding while staying auditable in a handful
//! of lines.
//!
//! Signing uses raw prehash (`Signer::sign_hash`) over the keccak of the
//! canonical bytes. The `bytes8("FHEVMCTA")` domain tag inside the payload
//! provides the domain separation.

use crate::{
    AttestationError, CiphertextAttestation, CiphertextAttestationPayload, DOMAIN_TAG, Version,
};
use alloy_primitives::{B256, Signature, U256};
use alloy_signer::Signer;
use sha3::{Digest, Keccak256};

/// V1 canonical-bytes length:
/// `bytes8 + uint8 + bytes32 + uint256*2 + bytes32*2 + uint8 = 170`.
const V1_PAYLOAD_LEN: usize = 8 + 1 + 32 + 32 + 32 + 32 + 32 + 1;

impl CiphertextAttestationPayload {
    /// Canonical `abi.encodePacked`-equivalent bytes for this payload. The
    /// exact message that gets keccak'd and signed.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        match self.version {
            Version::V1 => {
                let mut out = Vec::with_capacity(V1_PAYLOAD_LEN);
                out.extend_from_slice(&DOMAIN_TAG);
                out.push(self.version as u8);
                out.extend_from_slice(self.handle.as_slice());
                out.extend_from_slice(&self.key_id.to_be_bytes::<32>());
                out.extend_from_slice(&self.coprocessor_context_id.to_be_bytes::<32>());
                out.extend_from_slice(self.ciphertext_digest.as_slice());
                out.extend_from_slice(self.sns_ciphertext_digest.as_slice());
                out.push(self.format as u8);
                out
            }
        }
    }

    /// Keccak-256 of [`Self::canonical_bytes`] — the prehash the signer signs.
    pub fn canonical_digest(&self) -> B256 {
        keccak_b256(&self.canonical_bytes())
    }

    /// Consume the payload and produce a signed [`CiphertextAttestation`].
    /// `handle` and `coprocessor_context_id` are bound by the signature but
    /// stripped from the resulting wire form.
    pub async fn sign<S: Signer + Sync>(
        self,
        signer: &S,
    ) -> Result<CiphertextAttestation, AttestationError> {
        let sig = signer.sign_hash(&self.canonical_digest()).await?;
        Ok(CiphertextAttestation {
            version: self.version,
            key_id: self.key_id,
            ciphertext_digest: self.ciphertext_digest,
            sns_ciphertext_digest: self.sns_ciphertext_digest,
            format: self.format,
            signer: signer.address(),
            signature: sig.as_bytes().to_vec(),
        })
    }
}

impl CiphertextAttestation {
    /// Verify this attestation. `handle` and `coprocessor_context_id` are
    /// supplied by the caller from the S3 lookup path; both are bound by the
    /// signature, so any mismatch surfaces as [`AttestationError::SignerMismatch`].
    ///
    /// Membership and threshold/quorum checks against `self.signer` are the
    /// caller's responsibility.
    pub fn verify(
        &self,
        handle: B256,
        coprocessor_context_id: U256,
    ) -> Result<(), AttestationError> {
        let payload = CiphertextAttestationPayload {
            version: self.version,
            handle,
            key_id: self.key_id,
            coprocessor_context_id,
            ciphertext_digest: self.ciphertext_digest,
            sns_ciphertext_digest: self.sns_ciphertext_digest,
            format: self.format,
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
                expected: self.signer,
            });
        }
        Ok(())
    }
}

fn keccak_b256(bytes: &[u8]) -> B256 {
    B256::from_slice(Keccak256::digest(bytes).as_slice())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CiphertextFormat;
    use alloy_primitives::{address, b256, uint};
    use alloy_signer_local::PrivateKeySigner;

    const VERSION: Version = Version::V1;
    const HANDLE: B256 = b256!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    const KEY_ID: U256 = uint!(0xdeadbeef_U256);
    const CTX: U256 = U256::ZERO;
    const CT_DIGEST: B256 =
        b256!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb");
    const SNS_DIGEST: B256 =
        b256!("cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc");
    const FORMAT: CiphertextFormat = CiphertextFormat::UncompressedOnCpu;

    fn sample_payload() -> CiphertextAttestationPayload {
        CiphertextAttestationPayload::new(
            VERSION, HANDLE, KEY_ID, CTX, CT_DIGEST, SNS_DIGEST, FORMAT,
        )
    }

    #[test]
    fn v1_canonical_bytes_length() {
        assert_eq!(sample_payload().canonical_bytes().len(), V1_PAYLOAD_LEN);
    }

    async fn signed(signer: &PrivateKeySigner) -> CiphertextAttestation {
        sample_payload().sign(signer).await.unwrap()
    }

    #[tokio::test]
    async fn sign_then_verify_round_trip() {
        let signer = PrivateKeySigner::random();
        let att = signed(&signer).await;
        assert_eq!(att.signer, signer.address());
        att.verify(HANDLE, CTX).unwrap();
    }

    #[tokio::test]
    async fn fixed_key_signature_vector_is_pinned() {
        let signer: PrivateKeySigner =
            "0000000000000000000000000000000000000000000000000000000000000001"
                .parse()
                .unwrap();
        let att = signed(&signer).await;

        assert_eq!(
            att.signer,
            address!("7e5f4552091a69125d5dfcb7b8c2659029395bdf")
        );
        assert_eq!(
            hex::encode(&att.signature),
            "47c7923968439491f88d9e80997e07c22fd7e8c454da87b25933f0ff522d49792513dd8fe4c5851af5a8dfb12c9d00cd140f9937094e591d53ede75ae6adbf0a1c"
        );
        att.verify(HANDLE, CTX).unwrap();
    }

    #[tokio::test]
    async fn rejects_flipped_ciphertext_digest() {
        let signer = PrivateKeySigner::random();
        let mut att = signed(&signer).await;
        let mut bytes = att.ciphertext_digest.0;
        bytes[0] ^= 0x01;
        att.ciphertext_digest = B256::from(bytes);
        let err = att.verify(HANDLE, CTX).unwrap_err();
        assert!(matches!(err, AttestationError::SignerMismatch { .. }));
    }

    #[tokio::test]
    async fn rejects_wrong_handle() {
        let signer = PrivateKeySigner::random();
        let att = signed(&signer).await;
        let wrong = b256!("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
        let err = att.verify(wrong, CTX).unwrap_err();
        assert!(matches!(err, AttestationError::SignerMismatch { .. }));
    }

    #[tokio::test]
    async fn rejects_wrong_coprocessor_context_id() {
        let signer = PrivateKeySigner::random();
        let att = signed(&signer).await;
        let err = att.verify(HANDLE, U256::ONE).unwrap_err();
        assert!(matches!(err, AttestationError::SignerMismatch { .. }));
    }

    #[tokio::test]
    async fn rejects_replaced_signer_field() {
        let signer = PrivateKeySigner::random();
        let other = PrivateKeySigner::random();
        let mut att = signed(&signer).await;
        att.signer = other.address();
        let err = att.verify(HANDLE, CTX).unwrap_err();
        assert!(matches!(err, AttestationError::SignerMismatch { .. }));
    }

    #[tokio::test]
    async fn rejects_replaced_key_id() {
        let signer = PrivateKeySigner::random();
        let mut att = signed(&signer).await;
        att.key_id = KEY_ID + U256::ONE;
        let err = att.verify(HANDLE, CTX).unwrap_err();
        assert!(matches!(err, AttestationError::SignerMismatch { .. }));
    }

    #[tokio::test]
    async fn rejects_replaced_sns_digest() {
        let signer = PrivateKeySigner::random();
        let mut att = signed(&signer).await;
        let mut bytes = att.sns_ciphertext_digest.0;
        bytes[0] ^= 0x01;
        att.sns_ciphertext_digest = B256::from(bytes);
        let err = att.verify(HANDLE, CTX).unwrap_err();
        assert!(matches!(err, AttestationError::SignerMismatch { .. }));
    }

    #[tokio::test]
    async fn rejects_replaced_format() {
        let signer = PrivateKeySigner::random();
        let mut att = signed(&signer).await;
        att.format = CiphertextFormat::CompressedOnCpu;
        let err = att.verify(HANDLE, CTX).unwrap_err();
        assert!(matches!(err, AttestationError::SignerMismatch { .. }));
    }

    #[tokio::test]
    async fn rejects_bad_signature_length() {
        let signer = PrivateKeySigner::random();
        let mut att = signed(&signer).await;
        att.signature.truncate(60);
        let err = att.verify(HANDLE, CTX).unwrap_err();
        assert!(matches!(err, AttestationError::MalformedSignature { .. }));
    }

    /// Pins the V1 wire encoding against hand-checked hex. Any drift in field
    /// order, endianness, domain tag, or hash function breaks this test loudly.
    #[test]
    fn v1_canonical_bytes_and_digest_pinned() {
        let payload = CiphertextAttestationPayload::new(
            Version::V1,
            b256!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
            uint!(7_U256),
            U256::ZERO,
            b256!("1111111111111111111111111111111111111111111111111111111111111111"),
            b256!("2222222222222222222222222222222222222222222222222222222222222222"),
            CiphertextFormat::UncompressedOnCpu,
        );
        assert_eq!(
            hex::encode(payload.canonical_bytes()),
            "464845564d43544101aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00000000000000000000000000000000000000000000000000000000000000070000000000000000000000000000000000000000000000000000000000000000111111111111111111111111111111111111111111111111111111111111111122222222222222222222222222222222222222222222222222222222222222220a",
        );
        assert_eq!(
            hex::encode(payload.canonical_digest().as_slice()),
            "97f99a874a16f680d5c6b60b4cca7356877a78ce59f49872ad21030ebe6e0dd8",
        );
    }
}
