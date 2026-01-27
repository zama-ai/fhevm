use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CiphertextSideCar {
    pub key_id: String,
    pub tx_hash: String,
    pub handle: String,
    pub digest: String,
    pub digest_signature: String,
    pub digest_signer: String,
    pub created_at: String,
    pub format: String,
}

impl CiphertextSideCar {
    pub fn new(
        tx_hash: &[u8],
        handle: &[u8],
        digest: &[u8],
        digest_signature: &[u8],
        digest_signer: &str,
        format: &str,
    ) -> Self {
        let created_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            .to_string();
        Self {
            key_id: "TODO, we don't have key id yet only tenant".to_string(),
            tx_hash: hex::encode(tx_hash),
            handle: hex::encode(handle),
            digest: hex::encode(digest),
            digest_signature: hex::encode(digest_signature),
            digest_signer: digest_signer.to_owned(),
            created_at,
            format: format.to_owned(),
        }
    }

    pub fn to_json_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }
}
