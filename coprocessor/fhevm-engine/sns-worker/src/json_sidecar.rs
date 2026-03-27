use serde::{Deserialize, Serialize};

use fhevm_engine_common::types::CoproSigner;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// A structure representing metadata for a ciphertext stored externally (side car pattern).
///
/// This structure contains cryptographic metadata and identifiers necessary to verify
/// and retrieve ciphertext data that is stored outside of the main blockchain storage.
///
/// # Fields
///
/// * `key_id` - Identifier for the encryption key used to encrypt the ciphertext
/// * `tx_hash` - Transaction hash associated with this ciphertext
/// * `handle` - Unique handle/identifier for referencing this ciphertext
/// * `digest` - Cryptographic digest (hash) of the ciphertext content
/// * `signed_tuple` - Digital signature of tuple (handle, digest, key_id)
/// * `signed_digest` - Digital signature of the digest alone
/// * `signer` - Address or identifier of the entity that signed the tuple
/// * `created_at` - Timestamp indicating when this sidecar entry was created
/// * `format` - Format specification of the ciphertext (e.g., serialization format, version)
pub struct CiphertextSideCar {
    pub key_id: String,
    pub tx_hash: String,
    pub handle: String,
    pub digest: String,
    pub signed_tuple: String,
    pub signed_digest: String,
    pub signer: String,
    pub created_at: String,
    pub format: String,
}

pub struct SignedTuple {
    pub handle: String,
    pub digest: String,
    pub key_id: String,
}

impl SignedTuple {
    pub fn to_be_signed(&self) -> String {
        format!(
            "Handle:{},Digest:{},KeyId:{},",
            self.handle, self.digest, self.key_id
        )
    }
}

impl CiphertextSideCar {
    /// Creates a new `CiphertextSideCar` instance with the provided metadata.
    /// The input byte slices are expected to be raw binary data, which will be hex-encoded for storage in the JSON structure.
    pub async fn new(
        key_id: &[u8],
        tx_hash: &[u8],
        handle: &[u8],
        digest: &[u8],
        format: &str,
        signer: &CoproSigner,
    ) -> anyhow::Result<Self> {
        let signed_digest = signer.sign_message(digest).await?.as_bytes();
        let key_id = hex::encode(key_id);
        let tx_hash = hex::encode(tx_hash);
        let handle = hex::encode(handle);
        let digest = hex::encode(digest);
        let signed_tuple = SignedTuple {
            handle: handle.clone(),
            digest: digest.clone(),
            key_id: key_id.clone(),
        };
        let signed_tuple = String::into_bytes(signed_tuple.to_be_signed());
        let digest_signer = signer.address().to_string();
        let signed_tuple = signer.sign_message(&signed_tuple).await?.as_bytes();
        let created_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            .to_string();
        Ok(Self {
            key_id,
            tx_hash,
            handle,
            digest,
            signed_tuple: hex::encode(signed_tuple),
            signed_digest: hex::encode(signed_digest),
            signer: digest_signer.to_owned(),
            created_at,
            format: format.to_owned(),
        })
    }

    pub fn to_json_bytes(&self) -> anyhow::Result<Vec<u8>> {
        serde_json::to_vec(self)
            .map_err(|e| anyhow::anyhow!("Failed to serialize sidecar to JSON: {}", e))
    }
}
