//! `v1` request and response DTOs.
//!
//! The request bodies are declared with `sol!` for the EIP-712 hash implementation used to
//! compute the `decryption_id`.
//!
//! EVM `v1` shapes take no new fields. The Solana bodies share the EVM routes: `attestationType`
//! names the user decryption scheme, and a public decryption body is the Solana one when its
//! handles name a Solana chain.

use alloy::{
    primitives::{B256, Bytes, Keccak256},
    sol,
    sol_types::{Eip712Domain, SolStruct},
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// EIP-712 domain for the `v1` decryption interface.
pub const DECRYPTION_EIP712_DOMAIN: Eip712Domain = Eip712Domain {
    name: Some(Cow::Borrowed("kms-connector-decryption")),
    version: Some(Cow::Borrowed("1")),
    chain_id: None,
    verifying_contract: None,
    salt: None,
};

/// The maximum `durationSeconds` of a user decryption's validity window (365 days). Mirrors
/// `MAX_USER_DECRYPT_DURATION_SECONDS` of the `Decryption` gateway contract.
pub const MAX_USER_DECRYPTION_DURATION_SECONDS: u64 = 365 * 24 * 60 * 60;

/// Domain tag of the pad label preimage.
pub const PAD_LABEL_DOMAIN_TAG: &[u8; 8] = b"FHE_padl";

/// The only witness format currently defined.
pub const PAD_WITNESS_VERSION: u8 = 0x00;

/// Upper bound on the number of bits of a pad stream a request can release.
pub const MAX_PAD_LENGTH: u16 = 2048;

/// Upper bound on the number of `decryptors` of a witness.
pub const MAX_PAD_DECRYPTORS: u8 = 16;

sol! {
    /// `POST v1/public-decrypt` request body: the protocol inputs of a public decryption.
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct PublicDecryptionRequest {
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

    /// The protocol inputs of a user decryption, i.e. the `payload` of a [`UserDecryptionRequest`].
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct UserDecryptionPayload {
        HandleEntry[] handles;
        address userAddress;
        bytes publicKey;
        address[] allowedContracts;
        RequestValidity requestValidity;
        bytes extraData;
    }

    /// `POST v1/user-decrypt` request body: the user's signature over the protocol inputs of a
    /// user decryption, tagged with the scheme it was produced with.
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct UserDecryptionRequest {
        string attestationType;
        UserDecryptionPayload payload;
        /// The user's signature, in the scheme named by `attestationType`.
        bytes signature;
    }

    /// The per-pad fields of a label witness.
    ///
    /// The shared fields are carried once by the [`PadDecryptionPayload`].
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct PadWitness {
        address submitter;
        address consumer;
        /// Unix seconds after which the pad of the label can no longer be decrypted.
        uint64 expirationTimestamp;
        address[] decryptors;
        bytes16 nonce;
    }

    /// One requested pad: its label, the witness authenticating it, and the number of bits of
    /// its pad stream to release.
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct PadEntry {
        bytes32 label;
        PadWitness witness;
        uint16 padLength;
    }

    /// The protocol inputs of a pad decryption, i.e. the `payload` of a [`PadDecryptionRequest`].
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct PadDecryptionPayload {
        uint8 witnessVersion;
        uint16 prfId;
        uint64 chainId;
        PadEntry[] pads;
        address userAddress;
        bytes publicKey;
        address[] allowedContracts;
        RequestValidity requestValidity;
        bytes extraData;
    }

    /// `POST v1/pad-decrypt` request body.
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct PadDecryptionRequest {
        string attestationType;
        PadDecryptionPayload payload;
        /// The user's signature, in the scheme named by `attestationType`.
        bytes signature;
    }
}

sol! {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct SolanaHandleEntry {
        bytes32 handle;
        bytes32 ownerAddress;
        bytes32 encryptedStore;
    }

    /// Solana permit fields and the ordered ciphertext entries authorized under that permit.
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct SolanaUserDecryptionPayload {
        SolanaHandleEntry[] handles;
        bytes32 userAddress;
        bytes publicKey;
        bytes[] allowedScopes;
        RequestValidity requestValidity;
        bytes32 verifyingProgramId;
        bytes extraData;
    }

    /// The Solana body of `POST v1/public-decrypt`: each handle names the encrypted store its
    /// public-decrypt leaf is proven against, as a store is not derivable from a handle.
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct SolanaPublicDecryptionRequest {
        bytes32[] ctHandles;
        bytes extraData;
        bytes32[] encryptedStores;
    }

    /// The Solana body of `POST v1/user-decrypt`. The signature covers the canonical permit;
    /// the HTTP request ID additionally binds every ordered handle entry.
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct SolanaUserDecryptionRequest {
        string attestationType;
        SolanaUserDecryptionPayload payload;
        bytes signature;
    }
}

impl SolanaUserDecryptionRequest {
    pub fn id(&self) -> B256 {
        self.eip712_signing_hash(&DECRYPTION_EIP712_DOMAIN)
    }
}

impl SolanaPublicDecryptionRequest {
    /// Derives the content-derived `decryption_id`: the EIP-712 signing hash of the body, whose
    /// type hash differs from the EVM body's.
    pub fn id(&self) -> B256 {
        self.eip712_signing_hash(&DECRYPTION_EIP712_DOMAIN)
    }
}

impl PublicDecryptionRequest {
    /// Derives the content-derived `decryption_id`: the EIP-712 signing hash of the body.
    pub fn id(&self) -> B256 {
        self.eip712_signing_hash(&DECRYPTION_EIP712_DOMAIN)
    }
}

impl UserDecryptionRequest {
    /// Derives the content-derived `decryption_id`: the EIP-712 signing hash of the body.
    pub fn id(&self) -> B256 {
        self.eip712_signing_hash(&DECRYPTION_EIP712_DOMAIN)
    }
}

impl PadDecryptionRequest {
    /// Derives the content-derived `decryption_id`: the EIP-712 signing hash of the body.
    pub fn id(&self) -> B256 {
        self.eip712_signing_hash(&DECRYPTION_EIP712_DOMAIN)
    }
}

impl PadWitness {
    /// Derives the label of this witness under the given shared witness fields:
    ///
    /// `keccak256("FHE_padl" ‖ witnessVersion ‖ prfId ‖ chainId ‖ submitter ‖ consumer ‖
    /// expirationTimestamp ‖ decryptorCount ‖ decryptors ‖ nonce)`
    ///
    /// Integers use fixed-width big-endian encoding: `witnessVersion` and `decryptorCount` are
    /// 1 byte, `prfId` is 2 bytes, and `chainId` and `expirationTimestamp` are 8 bytes each.
    ///
    /// Computes the [`PAD_WITNESS_VERSION`] layout: checking `witness_version` and that the
    /// decryptor count is at most [`MAX_PAD_DECRYPTORS`] is up to the caller.
    ///
    /// # Panics
    ///
    /// If there are more than 255 decryptors.
    pub fn label(&self, witness_version: u8, prf_id: u16, chain_id: u64) -> B256 {
        let mut hasher = Keccak256::new();
        hasher.update(PAD_LABEL_DOMAIN_TAG);
        hasher.update([witness_version]);
        hasher.update(prf_id.to_be_bytes());
        hasher.update(chain_id.to_be_bytes());
        hasher.update(self.submitter);
        hasher.update(self.consumer);
        hasher.update(self.expirationTimestamp.to_be_bytes());
        hasher.update([u8::try_from(self.decryptors.len())
            .expect("checked against MAX_PAD_DECRYPTORS by the caller")]);
        for decryptor in &self.decryptors {
            hasher.update(decryptor);
        }
        hasher.update(self.nonce);
        hasher.finalize()
    }
}

impl PadDecryptionPayload {
    /// Derives the label of `witness` under the shared witness fields of this payload.
    pub fn label_of(&self, witness: &PadWitness) -> B256 {
        witness.label(self.witnessVersion, self.prfId, self.chainId)
    }
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

/// `200` body of `v1/pad-decrypt`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PadDecryptionResponse {
    /// The content-derived decryption id.
    pub decryption_id: B256,
    /// The node's pad shares, encrypted under the request's `publicKey`.
    pub encrypted_payload: Bytes,
    /// The KMS node's signature over `decryptionId` and `encryptedPayload`.
    pub signature: Bytes,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AttestationType;
    use alloy::primitives::{Address, FixedBytes};

    /// A v2 `extraData` (version, context id, epoch id).
    fn extra_data_v2() -> Vec<u8> {
        [vec![0x02], vec![0x07; 32], vec![0x08; 32]].concat()
    }

    fn public_request() -> PublicDecryptionRequest {
        PublicDecryptionRequest {
            ctHandles: vec![B256::repeat_byte(0x11), B256::repeat_byte(0x22)],
            extraData: Bytes::from(extra_data_v2()),
        }
    }

    fn user_request() -> UserDecryptionRequest {
        UserDecryptionRequest {
            attestationType: AttestationType::Eip712UnifiedUserDecryptV1.to_string(),
            payload: UserDecryptionPayload {
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
                extraData: Bytes::from(extra_data_v2()),
            },
            signature: Bytes::from(vec![0x66; 65]),
        }
    }

    fn pad_request() -> PadDecryptionRequest {
        PadDecryptionRequest {
            attestationType: AttestationType::Eip712UnifiedUserDecryptV1.to_string(),
            payload: PadDecryptionPayload {
                witnessVersion: 0,
                prfId: 0,
                chainId: 31337,
                pads: vec![PadEntry {
                    label: B256::repeat_byte(0xbb),
                    witness: PadWitness {
                        submitter: Address::repeat_byte(0x55),
                        consumer: Address::repeat_byte(0x33),
                        expirationTimestamp: 1_770_600_000,
                        decryptors: vec![],
                        nonce: FixedBytes::repeat_byte(0x0a),
                    },
                    padLength: 64,
                }],
                userAddress: Address::repeat_byte(0x55),
                publicKey: Bytes::from(vec![0x20, 0x00, 0x20, 0x00]),
                allowedContracts: vec![Address::repeat_byte(0x33)],
                requestValidity: RequestValidity {
                    startTimestamp: 1_770_000_000,
                    durationSeconds: 300,
                },
                extraData: Bytes::from(extra_data_v2()),
            },
            signature: Bytes::from(vec![0x66; 65]),
        }
    }

    #[test]
    fn solana_id_binds_scheme_signature_and_ordered_entries() {
        let request = SolanaUserDecryptionRequest {
            attestationType: "solana-srfc38-user-decrypt-v1".into(),
            payload: SolanaUserDecryptionPayload {
                handles: vec![SolanaHandleEntry {
                    handle: B256::repeat_byte(1),
                    ownerAddress: B256::repeat_byte(2),
                    encryptedStore: B256::repeat_byte(3),
                }],
                userAddress: B256::ZERO,
                publicKey: Bytes::new(),
                allowedScopes: vec![],
                requestValidity: RequestValidity {
                    startTimestamp: 1,
                    durationSeconds: 60,
                },
                verifyingProgramId: B256::ZERO,
                extraData: Bytes::new(),
            },
            signature: vec![4; 64].into(),
        };
        let id = request.id();
        // Independently encoded EIP-712 words, hashed with Foundry cast keccak.
        assert_eq!(
            id,
            "0x04e1a57fac578e2d2a40d1b8e596c2c142dc4507f41328586e524bd5cb8f8da1"
                .parse::<B256>()
                .unwrap()
        );
        let roundtrip: SolanaUserDecryptionRequest =
            serde_json::from_str(&serde_json::to_string(&request).unwrap()).unwrap();
        assert_eq!(id, roundtrip.id());
        let mut other = request.clone();
        other.attestationType.push('2');
        assert_ne!(id, other.id());
        let mut other = request.clone();
        other.signature = vec![5; 64].into();
        assert_ne!(id, other.id());
        let mut other = request.clone();
        other.payload.handles[0].ownerAddress = B256::repeat_byte(6);
        assert_ne!(id, other.id());
        let mut other = request.clone();
        other.payload.handles[0].encryptedStore = B256::repeat_byte(6);
        assert_ne!(id, other.id());
        let mut other = request.clone();
        other.payload.handles[0].handle = B256::repeat_byte(6);
        assert_ne!(id, other.id());
        let mut other = request.clone();
        other.payload.handles.push(other.payload.handles[0].clone());
        assert_ne!(id, other.id());
        other.payload.handles[1].handle = B256::repeat_byte(9);
        let ordered = other.id();
        other.payload.handles.swap(0, 1);
        assert_ne!(ordered, other.id());
        let mut json = serde_json::to_value(&request).unwrap();
        json["payload"]["authority"] = serde_json::json!("invented");
        assert!(serde_json::from_value::<SolanaUserDecryptionRequest>(json).is_err());
    }

    /// Both public bodies answer from one table keyed by id, which their EIP-712 type names keep
    /// apart: the same handles and `extraData` are two requests, even with no store.
    #[test]
    fn solana_public_id_binds_stores_and_is_not_an_evm_id() {
        let request = SolanaPublicDecryptionRequest {
            ctHandles: vec![B256::repeat_byte(0x11)],
            extraData: Bytes::from(vec![0x00]),
            encryptedStores: vec![B256::repeat_byte(0x33)],
        };
        let id = request.id();
        let mut other = request.clone();
        other.encryptedStores[0] = B256::repeat_byte(0x34);
        assert_ne!(id, other.id());
        let evm = PublicDecryptionRequest {
            ctHandles: request.ctHandles.clone(),
            extraData: request.extraData.clone(),
        };
        let storeless = SolanaPublicDecryptionRequest {
            encryptedStores: vec![],
            ..request.clone()
        };
        assert_ne!(storeless.id(), evm.id());
    }

    #[test]
    fn unknown_fields_are_rejected() {
        let json = r#"{
            "ctHandles": ["0x1111111111111111111111111111111111111111111111111111111111111111"],
            "extraData": "0x0207000000000000000000000000000000000000000000000000000000000000010800000000000000000000000000000000000000000000000000000000000001",
            "sneaky": true
        }"#;
        assert!(serde_json::from_str::<PublicDecryptionRequest>(json).is_err());

        // Both the envelope and the payload deny unknown fields.
        let mut json = serde_json::to_value(user_request()).unwrap();
        json["sneaky"] = serde_json::Value::Bool(true);
        assert!(serde_json::from_value::<UserDecryptionRequest>(json).is_err());
        let mut json = serde_json::to_value(user_request()).unwrap();
        json["payload"]["sneaky"] = serde_json::Value::Bool(true);
        assert!(serde_json::from_value::<UserDecryptionRequest>(json).is_err());

        let mut json = serde_json::to_value(pad_request()).unwrap();
        json["sneaky"] = serde_json::Value::Bool(true);
        assert!(serde_json::from_value::<PadDecryptionRequest>(json).is_err());
        let mut json = serde_json::to_value(pad_request()).unwrap();
        json["payload"]["sneaky"] = serde_json::Value::Bool(true);
        assert!(serde_json::from_value::<PadDecryptionRequest>(json).is_err());
        let mut json = serde_json::to_value(pad_request()).unwrap();
        json["payload"]["pads"][0]["sneaky"] = serde_json::Value::Bool(true);
        assert!(serde_json::from_value::<PadDecryptionRequest>(json).is_err());
        let mut json = serde_json::to_value(pad_request()).unwrap();
        json["payload"]["pads"][0]["witness"]["sneaky"] = serde_json::Value::Bool(true);
        assert!(serde_json::from_value::<PadDecryptionRequest>(json).is_err());
    }

    #[test]
    fn attestation_type_is_not_enforced_by_serde() {
        // Deserialization accepts any string: the endpoint answers a dedicated error code for
        // unsupported schemes, which requires a parsed body.
        let mut request = user_request();
        request.attestationType = "random_attestation_type".to_owned();
        let json = serde_json::to_string(&request).unwrap();
        let back: UserDecryptionRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(back.attestationType, "random_attestation_type");
    }

    #[test]
    fn id_is_deterministic_and_input_sensitive() {
        let public = public_request();
        assert_eq!(public.id(), public.id());

        let mut other = public.clone();
        other.ctHandles[0] = B256::repeat_byte(0x12);
        assert_ne!(public.id(), other.id());

        let user = user_request();
        assert_eq!(user.id(), user.id());
        let mut other = user.clone();
        other.payload.requestValidity.durationSeconds += 1;
        assert_ne!(user.id(), other.id());

        // The discriminant is part of the id: the same payload and signature under another
        // scheme is another request.
        let mut other = user.clone();
        other.attestationType = "random_attestation_type".to_owned();
        assert_ne!(user.id(), other.id());

        // The unsigned pad fields are part of the id.
        let pad = pad_request();
        assert_eq!(pad.id(), pad.id());
        let mut other = pad.clone();
        other.payload.pads[0].padLength += 1;
        assert_ne!(pad.id(), other.id());
        let mut other = pad.clone();
        other.payload.pads[0].witness.nonce = FixedBytes::repeat_byte(0x0b);
        assert_ne!(pad.id(), other.id());
    }
}

// Checks the `decryption_id` EIP-712 digests and the pad labels against the known-good values
// committed in `tests/vectors.json`, generated independently by `tests/vectors.py`.
#[cfg(test)]
mod vector_tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase", deny_unknown_fields)]
    struct Vectors {
        public_decryption: Vec<DecryptionVector<PublicDecryptionRequest>>,
        user_decryption: Vec<DecryptionVector<UserDecryptionRequest>>,
        solana_public_decryption: Vec<DecryptionVector<SolanaPublicDecryptionRequest>>,
        solana_user_decryption: Vec<DecryptionVector<SolanaUserDecryptionRequest>>,
        pad_decryption: Vec<DecryptionVector<PadDecryptionRequest>>,
        pad_label: Vec<LabelVector>,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase", deny_unknown_fields)]
    struct LabelVector {
        name: String,
        witness_version: u8,
        prf_id: u16,
        chain_id: u64,
        witness: PadWitness,
        label: B256,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase", deny_unknown_fields)]
    struct DecryptionVector<R> {
        name: String,
        request: R,
        decryption_id: B256,
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
    fn solana_decryption_id_vectors() {
        let vs = vectors();
        for v in vs.solana_public_decryption {
            assert_eq!(v.request.id(), v.decryption_id, "{}", v.name);
        }
        for v in vs.solana_user_decryption {
            assert_eq!(v.request.id(), v.decryption_id, "{}", v.name);
        }
    }

    #[test]
    fn pad_decryption_id_vectors() {
        for v in vectors().pad_decryption {
            assert_eq!(v.request.id(), v.decryption_id, "{}", v.name);
        }
    }

    #[test]
    fn pad_label_vectors() {
        for v in vectors().pad_label {
            let label = v.witness.label(v.witness_version, v.prf_id, v.chain_id);
            assert_eq!(label, v.label, "{}", v.name);
        }
        // The labels embedded in the request vectors are valid too.
        for v in vectors().pad_decryption {
            let payload = &v.request.payload;
            for pad in &payload.pads {
                assert_eq!(payload.label_of(&pad.witness), pad.label, "{}", v.name);
            }
        }
    }

    #[test]
    fn requests_serialize_to_vectors() {
        // The id vectors only exercise deserialization; this pins the serialize direction
        // (field casing, hex encodings) against the raw JSON of the vectors.
        fn check<R: Serialize>(vectors: Vec<DecryptionVector<R>>, raw: &serde_json::Value) {
            for (v, raw) in vectors.iter().zip(raw.as_array().unwrap()) {
                assert_eq!(
                    serde_json::to_value(&v.request).unwrap(),
                    raw["request"],
                    "{}",
                    v.name
                );
            }
        }
        let raw: serde_json::Value =
            serde_json::from_str(include_str!("../tests/vectors.json")).unwrap();
        let vs = vectors();
        check(vs.public_decryption, &raw["publicDecryption"]);
        check(vs.user_decryption, &raw["userDecryption"]);
        check(vs.pad_decryption, &raw["padDecryption"]);
        check(vs.solana_public_decryption, &raw["solanaPublicDecryption"]);
        check(vs.solana_user_decryption, &raw["solanaUserDecryption"]);
    }
}
