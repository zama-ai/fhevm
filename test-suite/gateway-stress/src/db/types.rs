use alloy::primitives::U256;
use sqlx::{Row, postgres::PgRow, types::time::OffsetDateTime};

pub struct DecryptionRequestDbMetadata {
    pub id: U256,
    pub created_at: OffsetDateTime,
}

pub struct DecryptionResponseDbMetadata {
    pub id: U256,
    pub created_at: OffsetDateTime,
    pub handle_batch_size: usize,
}

impl From<PgRow> for DecryptionRequestDbMetadata {
    fn from(row: PgRow) -> Self {
        Self {
            id: U256::from_le_slice(row.get("decryption_id")),
            created_at: row.get("created_at"),
        }
    }
}
