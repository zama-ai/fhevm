use crate::core::solana_acl::{
    SOLANA_NATIVE_ED25519_SIGNATURE_LEN, SOLANA_NATIVE_REQUEST_MODE_DELEGATED_SCOPED,
    SOLANA_NATIVE_REQUEST_MODE_DELEGATED_WILDCARD_SCOPED, SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED,
    SOLANA_NATIVE_REQUEST_MODE_PUBLIC, SolanaNativeAcceptedRequestV0, SolanaNativeReplayKeyV0,
    SolanaPubkeyBytes, SolanaUserDecryptionPayloadV0, solana_native_request_hash,
    solana_native_signature_message, solana_native_update_ascii,
};
use sha3::{Digest, Keccak256};
use thiserror::Error;

pub const SOLANA_NATIVE_RESPONSE_KIND_DIRECT_SCOPED: u8 = 0;
pub const SOLANA_NATIVE_RESPONSE_KIND_DELEGATED_SCOPED: u8 = 1;
pub const SOLANA_NATIVE_RESPONSE_KIND_DELEGATED_WILDCARD_SCOPED: u8 = 2;
pub const SOLANA_NATIVE_RESPONSE_KIND_PUBLIC: u8 = 3;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaKmsResponsePayloadV0 {
    pub domain_separator: [u8; 32],
    pub host_chain_id: u64,
    pub config_version: u64,
    pub solana_cluster_id: [u8; 32],
    pub kms_context_id: [u8; 32],
    pub request_hash: [u8; 32],
    pub request_mode: u8,
    pub response_kind: u8,
    pub nonce: [u8; 32],
    pub entries_hash: [u8; 32],
    pub extra_data_hash: [u8; 32],
    pub user_reencryption_pubkey_hash: [u8; 32],
    pub response_body_len: u32,
    pub response_body_hash: [u8; 32],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KmsResponseSignatureV0 {
    pub signer_pubkey: SolanaPubkeyBytes,
    pub signature: [u8; SOLANA_NATIVE_ED25519_SIGNATURE_LEN],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaKmsResponseCertificateV0 {
    pub kms_context_id: [u8; 32],
    pub signer_set_hash: [u8; 32],
    pub threshold: u16,
    pub signatures: Vec<KmsResponseSignatureV0>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaKmsResponseVerificationConfigV0 {
    pub kms_context_id: [u8; 32],
    pub signer_set_hash: [u8; 32],
    pub threshold: u16,
    pub signer_pubkeys: Vec<SolanaPubkeyBytes>,
    pub max_signers: u16,
    pub max_signatures: u16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaKmsVerifiedResponseV0 {
    pub response_hash: [u8; 32],
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SolanaKmsResponseVerificationError {
    #[error("native Solana KMS response signer set is invalid")]
    InvalidSignerSet,
    #[error("native Solana KMS response signer-set hash is invalid")]
    SignerSetHashMismatch,
    #[error("native Solana KMS response threshold is invalid")]
    InvalidThreshold,
    #[error("native Solana KMS response certificate context does not match the request")]
    CertificateContextMismatch,
    #[error("native Solana KMS response certificate threshold does not match the release")]
    CertificateThresholdMismatch,
    #[error("native Solana KMS response certificate signer-set hash does not match the release")]
    CertificateSignerSetHashMismatch,
    #[error("native Solana KMS response certificate has too many signatures")]
    TooManySignatures,
    #[error("native Solana KMS response certificate has too few valid signatures")]
    SignatureThresholdNotReached,
    #[error("native Solana KMS response signatures are not strictly sorted by signer pubkey")]
    SignaturesNotSorted,
    #[error("native Solana KMS response signature references an unknown signer")]
    UnknownSigner,
    #[error("native Solana KMS response signature is invalid")]
    InvalidSignature,
    #[error("native Solana KMS response kind is invalid for the accepted request")]
    InvalidResponseKind,
    #[error("native Solana KMS response payload does not match the accepted request")]
    ResponseBindingMismatch,
    #[error("native Solana KMS response body hash or length is invalid")]
    ResponseBodyMismatch,
}

pub fn verify_solana_kms_response_v0(
    config: &SolanaKmsResponseVerificationConfigV0,
    accepted: &SolanaNativeAcceptedRequestV0,
    request_payload: &SolanaUserDecryptionPayloadV0,
    response_payload: &SolanaKmsResponsePayloadV0,
    raw_response_body: &[u8],
    certificate: &SolanaKmsResponseCertificateV0,
) -> Result<SolanaKmsVerifiedResponseV0, SolanaKmsResponseVerificationError> {
    validate_response_config(config)?;
    if config.kms_context_id != request_payload.kms_context_id {
        return Err(SolanaKmsResponseVerificationError::CertificateContextMismatch);
    }
    verify_response_binding(accepted, request_payload, response_payload)?;
    verify_response_body(response_payload, raw_response_body)?;

    let response_hash = solana_native_kms_response_hash(response_payload);
    verify_response_certificate(config, response_hash, certificate)?;

    Ok(SolanaKmsVerifiedResponseV0 { response_hash })
}

pub fn solana_native_response_body_hash(raw_response_body: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    solana_native_update_ascii(&mut hasher, "zama-solana-kms-response-body-v0");
    hasher.update((raw_response_body.len() as u32).to_le_bytes());
    hasher.update(raw_response_body);
    hasher.finalize().into()
}

pub fn solana_native_kms_response_hash(payload: &SolanaKmsResponsePayloadV0) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    solana_native_update_ascii(&mut hasher, "zama-solana-kms-response-v0");
    hasher.update(payload.domain_separator);
    hasher.update(payload.host_chain_id.to_le_bytes());
    hasher.update(payload.config_version.to_le_bytes());
    hasher.update(payload.solana_cluster_id);
    hasher.update(payload.kms_context_id);
    hasher.update(payload.request_hash);
    hasher.update([payload.request_mode]);
    hasher.update([payload.response_kind]);
    hasher.update(payload.nonce);
    hasher.update(payload.entries_hash);
    hasher.update(payload.extra_data_hash);
    hasher.update(payload.user_reencryption_pubkey_hash);
    hasher.update(payload.response_body_len.to_le_bytes());
    hasher.update(payload.response_body_hash);
    hasher.finalize().into()
}

pub fn solana_native_kms_response_signature_message(response_hash: [u8; 32]) -> Vec<u8> {
    solana_native_signature_message("zama-solana-kms-response-signature-v0", response_hash)
}

pub fn solana_native_kms_response_signer_set_hash(
    kms_context_id: [u8; 32],
    threshold: u16,
    signer_pubkeys: &[SolanaPubkeyBytes],
) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    solana_native_update_ascii(&mut hasher, "zama-solana-kms-response-signer-set-v0");
    hasher.update(kms_context_id);
    hasher.update(threshold.to_le_bytes());
    hasher.update((signer_pubkeys.len() as u32).to_le_bytes());
    for signer in signer_pubkeys {
        hasher.update(signer);
    }
    hasher.finalize().into()
}

fn validate_response_config(
    config: &SolanaKmsResponseVerificationConfigV0,
) -> Result<(), SolanaKmsResponseVerificationError> {
    if config.kms_context_id == [0; 32]
        || config.signer_pubkeys.is_empty()
        || config.signer_pubkeys.len() > usize::from(config.max_signers)
        || config.signer_pubkeys.len() > usize::from(u16::MAX)
    {
        return Err(SolanaKmsResponseVerificationError::InvalidSignerSet);
    }
    if config.threshold == 0
        || config.threshold as usize > config.signer_pubkeys.len()
        || config.max_signatures < config.threshold
    {
        return Err(SolanaKmsResponseVerificationError::InvalidThreshold);
    }
    if !is_strictly_sorted_nonzero(&config.signer_pubkeys) {
        return Err(SolanaKmsResponseVerificationError::InvalidSignerSet);
    }
    let signer_set_hash = solana_native_kms_response_signer_set_hash(
        config.kms_context_id,
        config.threshold,
        &config.signer_pubkeys,
    );
    if signer_set_hash != config.signer_set_hash {
        return Err(SolanaKmsResponseVerificationError::SignerSetHashMismatch);
    }
    Ok(())
}

fn verify_response_binding(
    accepted: &SolanaNativeAcceptedRequestV0,
    request_payload: &SolanaUserDecryptionPayloadV0,
    response_payload: &SolanaKmsResponsePayloadV0,
) -> Result<(), SolanaKmsResponseVerificationError> {
    let expected_response_kind = response_kind_for_request_mode(request_payload.request_mode)?;
    if response_payload.response_kind != expected_response_kind {
        return Err(SolanaKmsResponseVerificationError::InvalidResponseKind);
    }
    if accepted.request_hash != solana_native_request_hash(request_payload) {
        return Err(SolanaKmsResponseVerificationError::ResponseBindingMismatch);
    }
    verify_accepted_replay_key(accepted, request_payload)?;
    if response_payload.domain_separator != request_payload.domain_separator
        || response_payload.host_chain_id != request_payload.host_chain_id
        || response_payload.config_version != request_payload.config_version
        || response_payload.solana_cluster_id != request_payload.solana_cluster_id
        || response_payload.kms_context_id != request_payload.kms_context_id
        || response_payload.request_hash != accepted.request_hash
        || response_payload.request_mode != request_payload.request_mode
        || response_payload.nonce != request_payload.nonce
        || response_payload.entries_hash != request_payload.entries_hash
        || response_payload.extra_data_hash != request_payload.extra_data_hash
        || response_payload.user_reencryption_pubkey_hash
            != request_payload.user_reencryption_pubkey_hash
    {
        return Err(SolanaKmsResponseVerificationError::ResponseBindingMismatch);
    }
    Ok(())
}

fn verify_accepted_replay_key(
    accepted: &SolanaNativeAcceptedRequestV0,
    request_payload: &SolanaUserDecryptionPayloadV0,
) -> Result<(), SolanaKmsResponseVerificationError> {
    if request_payload.request_mode == SOLANA_NATIVE_REQUEST_MODE_PUBLIC {
        return if accepted.replay_key.is_none() {
            Ok(())
        } else {
            Err(SolanaKmsResponseVerificationError::ResponseBindingMismatch)
        };
    }

    let Some(replay_key) = &accepted.replay_key else {
        return Err(SolanaKmsResponseVerificationError::ResponseBindingMismatch);
    };
    let expected_replay_key = SolanaNativeReplayKeyV0 {
        host_chain_id: request_payload.host_chain_id,
        solana_cluster_id: request_payload.solana_cluster_id,
        kms_context_id: request_payload.kms_context_id,
        request_signer_pubkey: request_payload.request_signer_pubkey,
        nonce: request_payload.nonce,
    };
    if replay_key != &expected_replay_key {
        return Err(SolanaKmsResponseVerificationError::ResponseBindingMismatch);
    }
    Ok(())
}

fn response_kind_for_request_mode(
    request_mode: u8,
) -> Result<u8, SolanaKmsResponseVerificationError> {
    match request_mode {
        SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED => Ok(SOLANA_NATIVE_RESPONSE_KIND_DIRECT_SCOPED),
        SOLANA_NATIVE_REQUEST_MODE_DELEGATED_SCOPED => {
            Ok(SOLANA_NATIVE_RESPONSE_KIND_DELEGATED_SCOPED)
        }
        SOLANA_NATIVE_REQUEST_MODE_DELEGATED_WILDCARD_SCOPED => {
            Ok(SOLANA_NATIVE_RESPONSE_KIND_DELEGATED_WILDCARD_SCOPED)
        }
        SOLANA_NATIVE_REQUEST_MODE_PUBLIC => Ok(SOLANA_NATIVE_RESPONSE_KIND_PUBLIC),
        _ => Err(SolanaKmsResponseVerificationError::InvalidResponseKind),
    }
}

fn verify_response_body(
    response_payload: &SolanaKmsResponsePayloadV0,
    raw_response_body: &[u8],
) -> Result<(), SolanaKmsResponseVerificationError> {
    if raw_response_body.len() > u32::MAX as usize
        || response_payload.response_body_len as usize != raw_response_body.len()
        || response_payload.response_body_hash
            != solana_native_response_body_hash(raw_response_body)
    {
        return Err(SolanaKmsResponseVerificationError::ResponseBodyMismatch);
    }
    Ok(())
}

fn verify_response_certificate(
    config: &SolanaKmsResponseVerificationConfigV0,
    response_hash: [u8; 32],
    certificate: &SolanaKmsResponseCertificateV0,
) -> Result<(), SolanaKmsResponseVerificationError> {
    if certificate.kms_context_id != config.kms_context_id {
        return Err(SolanaKmsResponseVerificationError::CertificateContextMismatch);
    }
    if certificate.threshold != config.threshold {
        return Err(SolanaKmsResponseVerificationError::CertificateThresholdMismatch);
    }
    if certificate.signer_set_hash != config.signer_set_hash {
        return Err(SolanaKmsResponseVerificationError::CertificateSignerSetHashMismatch);
    }
    if certificate.signatures.len() > usize::from(config.max_signatures) {
        return Err(SolanaKmsResponseVerificationError::TooManySignatures);
    }
    if certificate.signatures.len() < usize::from(config.threshold) {
        return Err(SolanaKmsResponseVerificationError::SignatureThresholdNotReached);
    }
    let signer_pubkeys = certificate
        .signatures
        .iter()
        .map(|signature| signature.signer_pubkey)
        .collect::<Vec<_>>();
    if !is_strictly_sorted_nonzero(&signer_pubkeys) {
        return Err(SolanaKmsResponseVerificationError::SignaturesNotSorted);
    }

    let message = solana_native_kms_response_signature_message(response_hash);
    let mut valid_signature_count = 0usize;
    for signature in &certificate.signatures {
        if config
            .signer_pubkeys
            .binary_search(&signature.signer_pubkey)
            .is_err()
        {
            return Err(SolanaKmsResponseVerificationError::UnknownSigner);
        }
        ring::signature::UnparsedPublicKey::new(&ring::signature::ED25519, signature.signer_pubkey)
            .verify(&message, &signature.signature)
            .map_err(|_| SolanaKmsResponseVerificationError::InvalidSignature)?;
        valid_signature_count += 1;
    }

    if valid_signature_count < usize::from(config.threshold) {
        return Err(SolanaKmsResponseVerificationError::SignatureThresholdNotReached);
    }
    Ok(())
}

fn is_strictly_sorted_nonzero(pubkeys: &[SolanaPubkeyBytes]) -> bool {
    if pubkeys.iter().any(|pubkey| *pubkey == [0; 32]) {
        return false;
    }
    pubkeys.windows(2).all(|window| window[0] < window[1])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::solana_acl::{
        SOLANA_NATIVE_SUPPORTED_MATERIAL_SOURCE_MODE, solana_native_domain_separator,
        solana_native_request_hash,
    };
    use ring::signature::KeyPair;

    fn key_pair(seed: u8) -> ring::signature::Ed25519KeyPair {
        ring::signature::Ed25519KeyPair::from_seed_unchecked(&[seed; 32]).unwrap()
    }

    fn sorted_key_pairs() -> Vec<ring::signature::Ed25519KeyPair> {
        let mut key_pairs = vec![key_pair(1), key_pair(2), key_pair(3)];
        key_pairs.sort_by_key(|key_pair| key_pair.public_key().as_ref().to_vec());
        key_pairs
    }

    fn signer_pubkey(key_pair: &ring::signature::Ed25519KeyPair) -> SolanaPubkeyBytes {
        key_pair.public_key().as_ref().try_into().unwrap()
    }

    fn hex32(value: &str) -> [u8; 32] {
        assert_eq!(value.len(), 64);
        let mut output = [0u8; 32];
        for (index, byte) in output.iter_mut().enumerate() {
            *byte = u8::from_str_radix(&value[index * 2..index * 2 + 2], 16).unwrap();
        }
        output
    }

    fn hex_bytes(value: &str) -> Vec<u8> {
        assert_eq!(value.len() % 2, 0);
        value
            .as_bytes()
            .chunks_exact(2)
            .map(|pair| {
                let hex = std::str::from_utf8(pair).unwrap();
                u8::from_str_radix(hex, 16).unwrap()
            })
            .collect()
    }

    fn request_payload(request_mode: u8) -> SolanaUserDecryptionPayloadV0 {
        let request_signer_pubkey = if request_mode == SOLANA_NATIVE_REQUEST_MODE_PUBLIC {
            [0; 32]
        } else {
            [7; 32]
        };
        let nonce = if request_mode == SOLANA_NATIVE_REQUEST_MODE_PUBLIC {
            [0; 32]
        } else {
            [8; 32]
        };
        let user_reencryption_pubkey_hash = if request_mode == SOLANA_NATIVE_REQUEST_MODE_PUBLIC {
            [0; 32]
        } else {
            [9; 32]
        };
        SolanaUserDecryptionPayloadV0 {
            domain_separator: solana_native_domain_separator(900, [2; 32], [3; 32], [5; 32]),
            host_chain_id: 900,
            config_version: 4,
            solana_cluster_id: [2; 32],
            kms_context_id: [5; 32],
            user_reencryption_pubkey_len: if request_mode == SOLANA_NATIVE_REQUEST_MODE_PUBLIC {
                0
            } else {
                32
            },
            user_reencryption_pubkey_hash,
            request_signer_pubkey,
            acl_program_id: [3; 32],
            request_mode,
            material_source_mode: SOLANA_NATIVE_SUPPORTED_MATERIAL_SOURCE_MODE,
            commitment_level: 1,
            min_context_slot: 10,
            expiration_slot: 20,
            nonce,
            extra_data_hash: [10; 32],
            entries_hash: [11; 32],
        }
    }

    fn accepted_request(payload: &SolanaUserDecryptionPayloadV0) -> SolanaNativeAcceptedRequestV0 {
        SolanaNativeAcceptedRequestV0 {
            request_hash: solana_native_request_hash(payload),
            replay_key: if payload.request_mode == SOLANA_NATIVE_REQUEST_MODE_PUBLIC {
                None
            } else {
                Some(SolanaNativeReplayKeyV0 {
                    host_chain_id: payload.host_chain_id,
                    solana_cluster_id: payload.solana_cluster_id,
                    kms_context_id: payload.kms_context_id,
                    request_signer_pubkey: payload.request_signer_pubkey,
                    nonce: payload.nonce,
                })
            },
        }
    }

    fn response_payload(
        payload: &SolanaUserDecryptionPayloadV0,
        accepted: &SolanaNativeAcceptedRequestV0,
        raw_response_body: &[u8],
    ) -> SolanaKmsResponsePayloadV0 {
        SolanaKmsResponsePayloadV0 {
            domain_separator: payload.domain_separator,
            host_chain_id: payload.host_chain_id,
            config_version: payload.config_version,
            solana_cluster_id: payload.solana_cluster_id,
            kms_context_id: payload.kms_context_id,
            request_hash: accepted.request_hash,
            request_mode: payload.request_mode,
            response_kind: response_kind_for_request_mode(payload.request_mode).unwrap(),
            nonce: payload.nonce,
            entries_hash: payload.entries_hash,
            extra_data_hash: payload.extra_data_hash,
            user_reencryption_pubkey_hash: payload.user_reencryption_pubkey_hash,
            response_body_len: raw_response_body.len() as u32,
            response_body_hash: solana_native_response_body_hash(raw_response_body),
        }
    }

    fn verification_config(
        key_pairs: &[ring::signature::Ed25519KeyPair],
        kms_context_id: [u8; 32],
        threshold: u16,
    ) -> SolanaKmsResponseVerificationConfigV0 {
        let signer_pubkeys = key_pairs.iter().map(signer_pubkey).collect::<Vec<_>>();
        let signer_set_hash =
            solana_native_kms_response_signer_set_hash(kms_context_id, threshold, &signer_pubkeys);
        SolanaKmsResponseVerificationConfigV0 {
            kms_context_id,
            signer_set_hash,
            threshold,
            signer_pubkeys,
            max_signers: 3,
            max_signatures: 3,
        }
    }

    fn certificate(
        key_pairs: &[ring::signature::Ed25519KeyPair],
        config: &SolanaKmsResponseVerificationConfigV0,
        payload: &SolanaKmsResponsePayloadV0,
        signature_count: usize,
    ) -> SolanaKmsResponseCertificateV0 {
        let response_hash = solana_native_kms_response_hash(payload);
        let message = solana_native_kms_response_signature_message(response_hash);
        let signatures = key_pairs
            .iter()
            .take(signature_count)
            .map(|key_pair| {
                let signature = key_pair.sign(&message);
                KmsResponseSignatureV0 {
                    signer_pubkey: signer_pubkey(key_pair),
                    signature: signature.as_ref().try_into().unwrap(),
                }
            })
            .collect();
        SolanaKmsResponseCertificateV0 {
            kms_context_id: config.kms_context_id,
            signer_set_hash: config.signer_set_hash,
            threshold: config.threshold,
            signatures,
        }
    }

    #[test]
    fn native_v0_response_hash_helpers_match_spec_vectors() {
        let raw_response_body = b"response";
        let response_body_hash = solana_native_response_body_hash(raw_response_body);
        assert_eq!(
            response_body_hash,
            hex32("e7a09401e6cc0ccfa240e427f1ff71c70c6013bc8f30a877e566c5c8f821bef4")
        );

        let response_payload = SolanaKmsResponsePayloadV0 {
            domain_separator: solana_native_domain_separator(900, [9; 32], [42; 32], [8; 32]),
            host_chain_id: 900,
            config_version: 3,
            solana_cluster_id: [9; 32],
            kms_context_id: [8; 32],
            request_hash: hex32("b435c1d350e62d5b945dc8acba09cdf4d300aaf9e2073b4fe53cee3f56d704b2"),
            request_mode: SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED,
            response_kind: SOLANA_NATIVE_RESPONSE_KIND_DIRECT_SCOPED,
            nonce: [6; 32],
            entries_hash: hex32("17af075816b9871ecaa47d89d5bdc192e6bedfa1842aa49cef988f59e2eb60c4"),
            extra_data_hash: hex32(
                "ae6b7cb429d69f3b06bd367b8362c44e9b5be702a2788a5136925333dddeb1c9",
            ),
            user_reencryption_pubkey_hash: hex32(
                "b912b7cb96960cad5718c858e4bf6cb7176cbcf96807607561037061a38567fc",
            ),
            response_body_len: raw_response_body.len() as u32,
            response_body_hash,
        };
        let response_hash = solana_native_kms_response_hash(&response_payload);
        assert_eq!(
            response_hash,
            hex32("dde127f9c12082471a059168c1cef81f9739842ab0ef273f8a2163aa6665aa1b")
        );
        assert_eq!(
            solana_native_kms_response_signature_message(response_hash),
            hex_bytes(
                "25007a616d612d736f6c616e612d6b6d732d726573706f6e73652d7369676e61747572652d7630dde127f9c12082471a059168c1cef81f9739842ab0ef273f8a2163aa6665aa1b"
            )
        );
        assert_eq!(
            solana_native_kms_response_signer_set_hash([8; 32], 1, &[[3; 32], [4; 32]]),
            hex32("d79c8613441849c1801664d3d5b0b4f0bfb1aec1a770573f8b03f9fc7aa843c6")
        );
    }

    #[test]
    fn verifies_threshold_response_certificate() {
        let payload = request_payload(SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED);
        let accepted = accepted_request(&payload);
        let raw_response_body = b"reencrypted-share";
        let response = response_payload(&payload, &accepted, raw_response_body);
        let key_pairs = sorted_key_pairs();
        let config = verification_config(&key_pairs, payload.kms_context_id, 2);
        let certificate = certificate(&key_pairs, &config, &response, 2);

        let verified = verify_solana_kms_response_v0(
            &config,
            &accepted,
            &payload,
            &response,
            raw_response_body,
            &certificate,
        )
        .unwrap();

        assert_eq!(
            verified.response_hash,
            solana_native_kms_response_hash(&response)
        );
    }

    #[test]
    fn rejects_response_config_context_mismatch_with_request() {
        let payload = request_payload(SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED);
        let accepted = accepted_request(&payload);
        let raw_response_body = b"reencrypted-share";
        let response = response_payload(&payload, &accepted, raw_response_body);
        let key_pairs = sorted_key_pairs();
        let config = verification_config(&key_pairs, [6; 32], 2);
        let certificate = certificate(&key_pairs, &config, &response, 2);

        assert_eq!(
            verify_solana_kms_response_v0(
                &config,
                &accepted,
                &payload,
                &response,
                raw_response_body,
                &certificate,
            ),
            Err(SolanaKmsResponseVerificationError::CertificateContextMismatch)
        );
    }

    #[test]
    fn rejects_response_body_mismatch() {
        let payload = request_payload(SOLANA_NATIVE_REQUEST_MODE_PUBLIC);
        let accepted = accepted_request(&payload);
        let response = response_payload(&payload, &accepted, b"cleartext");
        let key_pairs = sorted_key_pairs();
        let config = verification_config(&key_pairs, payload.kms_context_id, 2);
        let certificate = certificate(&key_pairs, &config, &response, 2);

        assert_eq!(
            verify_solana_kms_response_v0(
                &config,
                &accepted,
                &payload,
                &response,
                b"different",
                &certificate,
            ),
            Err(SolanaKmsResponseVerificationError::ResponseBodyMismatch)
        );
    }

    #[test]
    fn rejects_response_bound_to_a_different_request() {
        let payload = request_payload(SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED);
        let accepted = accepted_request(&payload);
        let raw_response_body = b"reencrypted-share";
        let mut response = response_payload(&payload, &accepted, raw_response_body);
        response.request_hash = [99; 32];
        let key_pairs = sorted_key_pairs();
        let config = verification_config(&key_pairs, payload.kms_context_id, 2);
        let certificate = certificate(&key_pairs, &config, &response, 2);

        assert_eq!(
            verify_solana_kms_response_v0(
                &config,
                &accepted,
                &payload,
                &response,
                raw_response_body,
                &certificate,
            ),
            Err(SolanaKmsResponseVerificationError::ResponseBindingMismatch)
        );
    }

    #[test]
    fn rejects_accepted_request_hash_not_derived_from_payload() {
        let payload = request_payload(SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED);
        let mut accepted = accepted_request(&payload);
        accepted.request_hash = [99; 32];
        let raw_response_body = b"reencrypted-share";
        let response = response_payload(&payload, &accepted, raw_response_body);
        let key_pairs = sorted_key_pairs();
        let config = verification_config(&key_pairs, payload.kms_context_id, 2);
        let certificate = certificate(&key_pairs, &config, &response, 2);

        assert_eq!(
            verify_solana_kms_response_v0(
                &config,
                &accepted,
                &payload,
                &response,
                raw_response_body,
                &certificate,
            ),
            Err(SolanaKmsResponseVerificationError::ResponseBindingMismatch)
        );
    }

    #[test]
    fn rejects_public_accepted_request_with_replay_key() {
        let payload = request_payload(SOLANA_NATIVE_REQUEST_MODE_PUBLIC);
        let mut accepted = accepted_request(&payload);
        accepted.replay_key = Some(SolanaNativeReplayKeyV0 {
            host_chain_id: payload.host_chain_id,
            solana_cluster_id: payload.solana_cluster_id,
            kms_context_id: payload.kms_context_id,
            request_signer_pubkey: payload.request_signer_pubkey,
            nonce: payload.nonce,
        });
        let raw_response_body = b"cleartext";
        let response = response_payload(&payload, &accepted, raw_response_body);
        let key_pairs = sorted_key_pairs();
        let config = verification_config(&key_pairs, payload.kms_context_id, 2);
        let certificate = certificate(&key_pairs, &config, &response, 2);

        assert_eq!(
            verify_solana_kms_response_v0(
                &config,
                &accepted,
                &payload,
                &response,
                raw_response_body,
                &certificate,
            ),
            Err(SolanaKmsResponseVerificationError::ResponseBindingMismatch)
        );
    }

    #[test]
    fn rejects_signed_accepted_request_without_replay_key() {
        let payload = request_payload(SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED);
        let mut accepted = accepted_request(&payload);
        accepted.replay_key = None;
        let raw_response_body = b"reencrypted-share";
        let response = response_payload(&payload, &accepted, raw_response_body);
        let key_pairs = sorted_key_pairs();
        let config = verification_config(&key_pairs, payload.kms_context_id, 2);
        let certificate = certificate(&key_pairs, &config, &response, 2);

        assert_eq!(
            verify_solana_kms_response_v0(
                &config,
                &accepted,
                &payload,
                &response,
                raw_response_body,
                &certificate,
            ),
            Err(SolanaKmsResponseVerificationError::ResponseBindingMismatch)
        );
    }

    #[test]
    fn rejects_signed_accepted_replay_key_not_bound_to_payload() {
        let payload = request_payload(SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED);
        let mut accepted = accepted_request(&payload);
        accepted.replay_key.as_mut().unwrap().nonce = [44; 32];
        let raw_response_body = b"reencrypted-share";
        let response = response_payload(&payload, &accepted, raw_response_body);
        let key_pairs = sorted_key_pairs();
        let config = verification_config(&key_pairs, payload.kms_context_id, 2);
        let certificate = certificate(&key_pairs, &config, &response, 2);

        assert_eq!(
            verify_solana_kms_response_v0(
                &config,
                &accepted,
                &payload,
                &response,
                raw_response_body,
                &certificate,
            ),
            Err(SolanaKmsResponseVerificationError::ResponseBindingMismatch)
        );
    }

    #[test]
    fn rejects_below_threshold_certificate() {
        let payload = request_payload(SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED);
        let accepted = accepted_request(&payload);
        let raw_response_body = b"reencrypted-share";
        let response = response_payload(&payload, &accepted, raw_response_body);
        let key_pairs = sorted_key_pairs();
        let config = verification_config(&key_pairs, payload.kms_context_id, 2);
        let certificate = certificate(&key_pairs, &config, &response, 1);

        assert_eq!(
            verify_solana_kms_response_v0(
                &config,
                &accepted,
                &payload,
                &response,
                raw_response_body,
                &certificate,
            ),
            Err(SolanaKmsResponseVerificationError::SignatureThresholdNotReached)
        );
    }

    #[test]
    fn rejects_unknown_response_signer() {
        let payload = request_payload(SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED);
        let accepted = accepted_request(&payload);
        let raw_response_body = b"reencrypted-share";
        let response = response_payload(&payload, &accepted, raw_response_body);
        let key_pairs = sorted_key_pairs();
        let config = verification_config(&key_pairs, payload.kms_context_id, 2);
        let unknown_key_pair = key_pair(4);
        let mut certificate = certificate(&key_pairs, &config, &response, 1);
        let response_hash = solana_native_kms_response_hash(&response);
        let message = solana_native_kms_response_signature_message(response_hash);
        certificate.signatures.push(KmsResponseSignatureV0 {
            signer_pubkey: signer_pubkey(&unknown_key_pair),
            signature: unknown_key_pair.sign(&message).as_ref().try_into().unwrap(),
        });
        certificate
            .signatures
            .sort_by_key(|signature| signature.signer_pubkey);

        assert_eq!(
            verify_solana_kms_response_v0(
                &config,
                &accepted,
                &payload,
                &response,
                raw_response_body,
                &certificate,
            ),
            Err(SolanaKmsResponseVerificationError::UnknownSigner)
        );
    }

    #[test]
    fn rejects_tampered_response_signature() {
        let payload = request_payload(SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED);
        let accepted = accepted_request(&payload);
        let raw_response_body = b"reencrypted-share";
        let response = response_payload(&payload, &accepted, raw_response_body);
        let key_pairs = sorted_key_pairs();
        let config = verification_config(&key_pairs, payload.kms_context_id, 2);
        let mut certificate = certificate(&key_pairs, &config, &response, 2);
        certificate.signatures[0].signature[0] ^= 0xff;

        assert_eq!(
            verify_solana_kms_response_v0(
                &config,
                &accepted,
                &payload,
                &response,
                raw_response_body,
                &certificate,
            ),
            Err(SolanaKmsResponseVerificationError::InvalidSignature)
        );
    }
}
