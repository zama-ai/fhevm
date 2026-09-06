//! `v1` request and response DTOs.
//!
//! The request bodies are declared with `sol!` for the EIP-712 hash implementation used to
//! compute the `decryption_id`.
//!
//! Interface evolution happens via new versioned routes, not by adding fields to `v1` shapes.

use alloy::{
    primitives::{B256, Bytes},
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

    /// `POST v1/user-decrypt` request body: the protocol inputs of a user decryption
    /// (unified shape per RFC 016).
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct UserDecryptionRequest {
        HandleEntry[] handles;
        address userAddress;
        bytes publicKey;
        address[] allowedContracts;
        RequestValidity requestValidity;
        /// The user's EIP-712 signature.
        bytes signature;
        bytes extraData;
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::Address;

    fn public_request() -> PublicDecryptionRequest {
        PublicDecryptionRequest {
            ctHandles: vec![B256::repeat_byte(0x11), B256::repeat_byte(0x22)],
            extraData: Bytes::from(vec![0x00]),
        }
    }

    fn user_request() -> UserDecryptionRequest {
        UserDecryptionRequest {
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
    fn unknown_fields_are_rejected() {
        let json = r#"{
            "ctHandles": ["0x1111111111111111111111111111111111111111111111111111111111111111"],
            "extraData": "0x00",
            "sneaky": true
        }"#;
        assert!(serde_json::from_str::<PublicDecryptionRequest>(json).is_err());
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
        other.requestValidity.durationSeconds += 1;
        assert_ne!(user.id(), other.id());
    }
}

// Checks the `decryption_id` EIP-712 digests against the known-good values committed in
// `tests/vectors.json`.
#[cfg(test)]
mod vector_tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase", deny_unknown_fields)]
    struct Vectors {
        public_decryption: Vec<DecryptionVector<PublicDecryptionRequest>>,
        user_decryption: Vec<DecryptionVector<UserDecryptionRequest>>,
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
}
