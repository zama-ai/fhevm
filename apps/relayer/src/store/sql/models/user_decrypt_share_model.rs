use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

/// Represents a row in the `user_decrypt_share` table.
#[derive(Debug, FromRow, Clone, Serialize, Deserialize)]
pub struct UserDecryptShare {
    pub id: i32,
    pub gw_reference_id: Vec<u8>,
    pub share_index: i32,
    pub share: String,
    pub kms_signature: String,
    pub extra_data: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
