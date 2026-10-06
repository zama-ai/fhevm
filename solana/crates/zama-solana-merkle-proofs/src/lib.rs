//! CBOR wire contract for the Solana Merkle proof HTTP service.

use serde::{Deserialize, Serialize};
use serde_bytes::ByteArray;

pub const MERKLE_PROOFS_PATH: &str = "/v1/solana/merkle-proofs";

/// Most leaves one request may ask for.
pub const MAX_LEAVES_PER_REQUEST: usize = 64;

/// Which leaf authorizes the decrypt.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum LeafQueryKind {
    /// A historical-access leaf: `key` was allowed on `handle`.
    Allowed,
    /// A public-decrypt leaf: `handle` was made public.
    Public,
}

/// One leaf to prove. Byte fields are 32-byte CBOR byte strings.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct LeafQuery {
    /// The encrypted store account whose MMR holds the leaf.
    #[serde(with = "serde_bytes")]
    #[cfg_attr(feature = "openapi", schema(value_type = String, format = Binary))]
    pub encrypted_store: [u8; 32],
    #[serde(with = "serde_bytes")]
    #[cfg_attr(feature = "openapi", schema(value_type = String, format = Binary))]
    pub handle: [u8; 32],
    pub kind: LeafQueryKind,
    /// The allowed key; required for `allowed`, absent for `public`.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "serde_bytes")]
    #[cfg_attr(feature = "openapi", schema(value_type = Option<String>, format = Binary))]
    pub key: Option<[u8; 32]>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct MerkleProofRequest {
    /// At most [`MAX_LEAVES_PER_REQUEST`] entries; answered in order.
    pub leaves: Vec<LeafQuery>,
}

/// The answer for one queried leaf, in request order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase", tag = "status")]
pub enum MerkleProofOutcome {
    /// The leaf is recorded. `leafCount` is the history the record had sealed
    /// when it built the proof; the caller verifies the path against the
    /// on-chain account's peaks and retries when the record is behind the
    /// chain (`leafCount` smaller).
    #[serde(rename_all = "camelCase")]
    Found {
        leaf_index: u64,
        leaf_count: u64,
        /// Authentication path from the leaf to its peak, 32-byte byte strings.
        #[cfg_attr(feature = "openapi", schema(value_type = Vec<String>))]
        #[serde(
            serialize_with = "encode_siblings",
            deserialize_with = "decode_siblings"
        )]
        siblings: Vec<[u8; 32]>,
    },
    /// The account is recorded but no such leaf is, at `leafCount` leaves. Either
    /// it was never sealed or the record has not reached the block that sealed it.
    #[serde(rename_all = "camelCase")]
    NotFound { leaf_count: u64 },
    /// The record never saw this account.
    UnknownAccount,
    /// This record is known to disagree with the chain for this leaf: the store
    /// check quarantined the store, or the leaf's commitment or path does not
    /// match the recorded peaks. The caller asks another coprocessor.
    Inconsistent,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct MerkleProofResponse {
    pub proofs: Vec<MerkleProofOutcome>,
}

/// Error codes, in the vocabulary of the Direct HTTP Decryption Endpoint RFC.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    /// Malformed body, missing key, too many leaves.
    Malformed,
    /// The request signature is missing, malformed or expired, or its signer is
    /// not the tx-sender of a node in a live KMS context.
    SenderAuthenticationFailed,
    /// The leaf record could not be read or is inconsistent, or the KMS
    /// tx-sender set is not read yet; retry later.
    UpstreamTransient,
    /// The signer is over its leaves per second or holds as many signed
    /// requests as it may, or no database connection freed in time; ask another
    /// coprocessor or retry later.
    RateLimited,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub code: ErrorCode,
    pub message: String,
    pub retryable: bool,
}

impl ErrorCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Malformed => "malformed",
            Self::SenderAuthenticationFailed => "sender_authentication_failed",
            Self::UpstreamTransient => "upstream_transient",
            Self::RateLimited => "rate_limited",
        }
    }
}

fn encode_siblings<S: serde::Serializer>(
    siblings: &[[u8; 32]],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.collect_seq(siblings.iter().map(|sibling| ByteArray::new(*sibling)))
}

fn decode_siblings<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Vec<[u8; 32]>, D::Error> {
    let siblings = Vec::<ByteArray<32>>::deserialize(deserializer)?;
    if siblings.len() > zama_solana_acl::MAX_MMR_PEAKS {
        return Err(serde::de::Error::custom("too many proof siblings"));
    }
    Ok(siblings.into_iter().map(ByteArray::into_array).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proof_decoding_bounds_the_siblings() {
        for count in [
            zama_solana_acl::MAX_MMR_PEAKS,
            zama_solana_acl::MAX_MMR_PEAKS + 1,
        ] {
            let proof = MerkleProofOutcome::Found {
                leaf_index: 0,
                leaf_count: 1,
                siblings: vec![[0; 32]; count],
            };
            let mut body = Vec::new();
            ciborium::into_writer(&proof, &mut body).unwrap();
            let decoded = ciborium::from_reader::<MerkleProofOutcome, _>(body.as_slice());
            if count == zama_solana_acl::MAX_MMR_PEAKS {
                assert_eq!(decoded.unwrap(), proof);
            } else {
                assert!(decoded
                    .unwrap_err()
                    .to_string()
                    .contains("too many proof siblings"));
            }
        }
    }
}
