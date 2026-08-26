//! `v1` request and response DTOs.
//!
//! The payload structs are declared with `sol!` for EIP-712 hash implementation used to compute
//! the `decryption_id`.
//!
//! Interface evolution happens via new versioned routes, not by adding fields to `v1` shapes.

use alloy::{
    primitives::{Address, B256, Bytes, SignatureError},
    signers::{Signature, Signer, SignerSync},
    sol,
    sol_types::{Eip712Domain, SolStruct},
};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    time::{SystemTime, UNIX_EPOCH},
};

/// EIP-712 domain for the `v1` decryption interface.
pub const DECRYPTION_EIP712_DOMAIN: Eip712Domain = Eip712Domain {
    name: Some(Cow::Borrowed("kms-connector-decryption")),
    version: Some(Cow::Borrowed("1")),
    chain_id: None,
    verifying_contract: None,
    salt: None,
};

/// `POST v1/public-decrypt` request body.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PublicDecryptionRequest {
    pub payload: PublicDecryptionPayload,
    /// Sender's signature over (payload_hash, timestamp).
    pub sender_signature: Bytes,
    /// Timestamp at which the request was made by the client (Unix seconds).
    pub timestamp: u64,
}

/// `POST v1/user-decrypt` request body.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UserDecryptionRequest {
    pub payload: UserDecryptionPayload,
    /// Sender's signature over (payload_hash, timestamp).
    pub sender_signature: Bytes,
    /// Timestamp at which the request was made by the client (Unix seconds).
    pub timestamp: u64,
}

/// `200` body of `v1/public-decrypt`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicDecryptionResponse {
    /// The content-derived decryption id.
    pub decryption_id: B256,
    /// ABI-encoded plaintexts, as published on-chain on Gateway.
    pub decrypted_result: Bytes,
    /// The KMS node's external signature over the result.
    pub signature: Bytes,
    pub extra_data: Bytes,
}

/// `200` body of `v1/user-decrypt`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDecryptionResponse {
    /// The content-derived decryption id.
    pub decryption_id: B256,
    /// This node's signcrypted decryption share(s), as published on-chain on Gateway.
    pub user_decrypted_shares: Bytes,
    /// The KMS node's external signature over the shares.
    pub signature: Bytes,
    pub extra_data: Bytes,
}

sol! {
    /// The inputs of a public decryption.
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct PublicDecryptionPayload {
        bytes32[] ctHandles;
        bytes extraData;
    }

    /// One requested ciphertext handle with its owning contract and owner.
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct HandleEntry {
        bytes32 handle;
        address contractAddress;
        address ownerAddress;
    }

    /// The user-signed validity window of a user-decryption request. Timestamps are Unix seconds.
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct RequestValidity {
        uint64 startTimestamp;
        uint64 durationSeconds;
    }

    /// The id-bound protocol inputs of a user decryption.
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct UserDecryptionPayload {
        HandleEntry[] handles;
        address userAddress;
        bytes publicKey;
        address[] allowedContracts;
        RequestValidity requestValidity;
        bytes signature;
        bytes extraData;
    }

    /// The claim `senderSignature` signs: "I submit payload `payloadHash` at `timestamp`".
    struct RequestSubmission {
        bytes32 payloadHash;
        uint64 timestamp;
    }
}

impl PublicDecryptionPayload {
    /// Computes the EIP-712 hash of the `PublicDecryptionPayload`.
    pub fn hash(&self) -> B256 {
        self.eip712_signing_hash(&DECRYPTION_EIP712_DOMAIN)
    }

    pub async fn sign<S: Signer>(
        self,
        signer: &S,
    ) -> alloy::signers::Result<PublicDecryptionRequest> {
        let timestamp = utc_now_secs();
        self.sign_at(signer, timestamp).await
    }

    pub async fn sign_at<S: Signer>(
        self,
        signer: &S,
        timestamp: u64,
    ) -> alloy::signers::Result<PublicDecryptionRequest> {
        let request_digest = request_digest(self.hash(), timestamp);
        let signature = signer.sign_hash(&request_digest).await?;

        Ok(PublicDecryptionRequest {
            payload: self,
            sender_signature: signature.as_bytes().into(),
            timestamp,
        })
    }

    pub fn sign_sync<S: SignerSync>(
        self,
        signer: &S,
    ) -> alloy::signers::Result<PublicDecryptionRequest> {
        let timestamp = utc_now_secs();
        self.sign_sync_at(signer, timestamp)
    }

    pub fn sign_sync_at<S: SignerSync>(
        self,
        signer: &S,
        timestamp: u64,
    ) -> alloy::signers::Result<PublicDecryptionRequest> {
        let request_digest = request_digest(self.hash(), timestamp);
        let signature = signer.sign_hash_sync(&request_digest)?;

        Ok(PublicDecryptionRequest {
            payload: self,
            sender_signature: signature.as_bytes().into(),
            timestamp,
        })
    }
}

impl PublicDecryptionRequest {
    /// Derives the `decryption_id` of the `PublicDecryptionRequest`.
    pub fn id(&self) -> B256 {
        self.payload.hash()
    }

    /// Recovers the signer's address from the `sender_signature` and the request digest.
    pub fn recover_signer_address(&self) -> Result<Address, SignatureError> {
        let request_digest = request_digest(self.payload.hash(), self.timestamp);
        Signature::from_raw(&self.sender_signature)?.recover_address_from_prehash(&request_digest)
    }
}

impl UserDecryptionPayload {
    /// Computes the EIP-712 hash of the `UserDecryptionPayload`.
    pub fn hash(&self) -> B256 {
        self.eip712_signing_hash(&DECRYPTION_EIP712_DOMAIN)
    }

    pub async fn sign<S: Signer>(
        self,
        signer: &S,
    ) -> alloy::signers::Result<UserDecryptionRequest> {
        let timestamp = utc_now_secs();
        self.sign_at(signer, timestamp).await
    }

    pub async fn sign_at<S: Signer>(
        self,
        signer: &S,
        timestamp: u64,
    ) -> alloy::signers::Result<UserDecryptionRequest> {
        let request_digest = request_digest(self.hash(), timestamp);
        let signature = signer.sign_hash(&request_digest).await?;

        Ok(UserDecryptionRequest {
            payload: self,
            sender_signature: signature.as_bytes().into(),
            timestamp,
        })
    }

    pub fn sign_sync<S: SignerSync>(
        self,
        signer: &S,
    ) -> alloy::signers::Result<UserDecryptionRequest> {
        let timestamp = utc_now_secs();
        self.sign_sync_at(signer, timestamp)
    }

    pub fn sign_sync_at<S: SignerSync>(
        self,
        signer: &S,
        timestamp: u64,
    ) -> alloy::signers::Result<UserDecryptionRequest> {
        let request_digest = request_digest(self.hash(), timestamp);
        let signature = signer.sign_hash_sync(&request_digest)?;

        Ok(UserDecryptionRequest {
            payload: self,
            sender_signature: signature.as_bytes().into(),
            timestamp,
        })
    }
}

impl UserDecryptionRequest {
    /// Derives the `decryption_id` of the `UserDecryptionRequest`.
    pub fn id(&self) -> B256 {
        self.payload.hash()
    }

    /// Recovers the signer's address from the `sender_signature` and the request digest.
    pub fn recover_signer_address(&self) -> Result<Address, SignatureError> {
        let request_digest = request_digest(self.payload.hash(), self.timestamp);
        Signature::from_raw(&self.sender_signature)?.recover_address_from_prehash(&request_digest)
    }
}

/// The EIP-712 digest the sender signs: the signing hash of [`RequestSubmission`].
fn request_digest(payload_hash: B256, timestamp: u64) -> B256 {
    RequestSubmission {
        payloadHash: payload_hash,
        timestamp,
    }
    .eip712_signing_hash(&DECRYPTION_EIP712_DOMAIN)
}

/// Returns the current UTC time in seconds since the Unix epoch.
fn utc_now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Clock may have gone backwards")
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{primitives::Address, signers::local::PrivateKeySigner};

    fn public_payload() -> PublicDecryptionPayload {
        PublicDecryptionPayload {
            ctHandles: vec![B256::repeat_byte(0x11), B256::repeat_byte(0x22)],
            extraData: Bytes::from(vec![0x00]),
        }
    }

    fn user_payload() -> UserDecryptionPayload {
        UserDecryptionPayload {
            handles: vec![HandleEntry {
                handle: B256::repeat_byte(0xaa),
                contractAddress: Address::repeat_byte(0x33),
                ownerAddress: Address::repeat_byte(0x44),
            }],
            userAddress: Address::repeat_byte(0x55),
            publicKey: Bytes::from(vec![0x20, 0x00, 0x20, 0x00]),
            allowedContracts: vec![Address::repeat_byte(0x33)],
            requestValidity: RequestValidity {
                startTimestamp: 1_770_000_000,
                durationSeconds: 300,
            },
            signature: Bytes::from(vec![0x66; 65]),
            extraData: Bytes::from(vec![0x00]),
        }
    }

    #[test]
    fn unknown_payload_fields_are_rejected() {
        let json = r#"{
            "payload": {
                "ctHandles": ["0x1111111111111111111111111111111111111111111111111111111111111111"],
                "extraData": "0x00",
                "sneaky": true
            },
            "senderSignature": "0x",
            "timestamp": 0
        }"#;
        assert!(serde_json::from_str::<PublicDecryptionRequest>(json).is_err());
    }

    #[test]
    fn id_is_deterministic_and_input_sensitive() {
        let public = PublicDecryptionRequest {
            payload: public_payload(),
            sender_signature: Bytes::from(vec![0x01; 65]),
            timestamp: 1_770_000_000,
        };
        assert_eq!(public.id(), public.id());

        let mut other = public.clone();
        other.payload.ctHandles[0] = B256::repeat_byte(0x12);
        assert_ne!(public.id(), other.id());

        let user = UserDecryptionRequest {
            payload: user_payload(),
            sender_signature: Bytes::from(vec![0x01; 65]),
            timestamp: 1_770_000_000,
        };
        assert_eq!(user.id(), user.id());
        let mut other = user.clone();
        other.payload.requestValidity.durationSeconds += 1;
        assert_ne!(user.id(), other.id());
    }

    #[test]
    fn id_ignores_request_timestamp_and_sender_signature() {
        // A retry re-signs the same payload with a fresh timestamp: same id.
        let a = PublicDecryptionRequest {
            payload: public_payload(),
            sender_signature: Bytes::from(vec![0x01; 65]),
            timestamp: 1_770_000_000,
        };
        let b = PublicDecryptionRequest {
            payload: public_payload(),
            sender_signature: Bytes::from(vec![0x02; 65]),
            timestamp: 1_770_009_999,
        };
        assert_eq!(a.id(), b.id());
    }

    #[tokio::test]
    async fn public_sender_signature_round_trip() {
        let signer = PrivateKeySigner::random();
        let payload = public_payload();
        let timestamp = utc_now_secs();

        let request = payload.clone().sign_at(&signer, timestamp).await.unwrap();
        let request_sync = payload.clone().sign_sync_at(&signer, timestamp).unwrap();
        assert_eq!(request, request_sync);

        let recovered = request.recover_signer_address().unwrap();
        assert_eq!(recovered, signer.address());

        // A different timestamp generates a different signature.
        let other = payload.sign_at(&signer, timestamp + 1).await.unwrap();
        assert_ne!(request.sender_signature, other.sender_signature);
    }

    #[tokio::test]
    async fn user_sender_signature_round_trip() {
        let signer = PrivateKeySigner::random();
        let payload = user_payload();
        let timestamp = utc_now_secs();

        let request = payload.clone().sign_at(&signer, timestamp).await.unwrap();
        let request_sync = payload.clone().sign_sync_at(&signer, timestamp).unwrap();
        assert_eq!(request, request_sync);

        let recovered = request.recover_signer_address().unwrap();
        assert_eq!(recovered, signer.address());

        // A different timestamp generates a different signature.
        let other = payload.sign_at(&signer, timestamp + 1).await.unwrap();
        assert_ne!(request.sender_signature, other.sender_signature);
    }
}

// Checks the EIP-712 digests (`decryption_id` and the request-submission digest) against
// the known-good values committed in `tests/vectors.json`.
#[cfg(test)]
mod vector_tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase", deny_unknown_fields)]
    struct Vectors {
        public_decryption: Vec<DecryptionVector<PublicDecryptionRequest>>,
        user_decryption: Vec<DecryptionVector<UserDecryptionRequest>>,
        request_submission: Vec<RequestSubmissionVector>,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase", deny_unknown_fields)]
    struct DecryptionVector<R> {
        name: String,
        request: R,
        decryption_id: B256,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase", deny_unknown_fields)]
    struct RequestSubmissionVector {
        name: String,
        payload_hash: B256,
        timestamp: u64,
        digest: B256,
    }

    fn vectors() -> Vectors {
        serde_json::from_str(include_str!("../tests/vectors.json")).unwrap()
    }

    #[test]
    fn public_decryption_id_vectors() {
        for v in vectors().public_decryption {
            assert_eq!(v.request.id(), v.decryption_id, "{}", v.name);
        }
    }

    #[test]
    fn user_decryption_id_vectors() {
        for v in vectors().user_decryption {
            assert_eq!(v.request.id(), v.decryption_id, "{}", v.name);
        }
    }

    #[test]
    fn requests_round_trip_through_serde() {
        // The id vectors only exercise deserialization; this pins the serialize direction
        // (field casing, hex encodings) by re-parsing what we emit.
        let vs = vectors();
        for v in vs.public_decryption {
            let json = serde_json::to_string(&v.request).unwrap();
            let back: PublicDecryptionRequest = serde_json::from_str(&json).unwrap();
            assert_eq!(back, v.request, "{}", v.name);
        }
        for v in vs.user_decryption {
            let json = serde_json::to_string(&v.request).unwrap();
            let back: UserDecryptionRequest = serde_json::from_str(&json).unwrap();
            assert_eq!(back, v.request, "{}", v.name);
        }
    }

    #[test]
    fn submission_digest_vectors() {
        for v in vectors().request_submission {
            assert_eq!(
                request_digest(v.payload_hash, v.timestamp),
                v.digest,
                "{}",
                v.name
            );
        }
    }
}
