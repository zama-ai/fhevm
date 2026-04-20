use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Represents a row in the `filters` table
#[derive(Debug, Clone)]
pub struct Filter {
    pub id: Uuid,
    pub chain_id: i64,
    pub consumer_id: String,
    pub from: Option<String>,
    pub to: Option<String>,
    pub log_address: Option<String>,
    pub created_at: DateTime<Utc>,
}
