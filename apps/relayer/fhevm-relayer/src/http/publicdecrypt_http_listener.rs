use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicDecryptErrorResponseJson {
    pub message: String,
}
