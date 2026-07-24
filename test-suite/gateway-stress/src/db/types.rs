use alloy::primitives::U256;
use sqlx::{Row, postgres::PgRow, types::time::PrimitiveDateTime};

pub struct DecryptionRequestDbMetadata {
    pub id: U256,
    pub created_at: PrimitiveDateTime,
}

pub struct DecryptionResponseDbMetadata {
    pub id: U256,
    pub created_at: PrimitiveDateTime,
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
