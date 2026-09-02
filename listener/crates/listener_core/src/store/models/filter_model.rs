use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Maps to PostgreSQL enum type `filter_type`
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "filter_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FilterType {
    Live,
    Final,
}

impl From<primitives::event::FilterType> for FilterType {
    fn from(filter_type: primitives::event::FilterType) -> Self {
        match filter_type {
            primitives::event::FilterType::Live => Self::Live,
            primitives::event::FilterType::Final => Self::Final,
        }
    }
}

/// Represents a row in the `filters` table
#[derive(Debug, Clone)]
pub struct Filter {
    pub id: Uuid,
    pub chain_id: i64,
    pub consumer_id: String,
    pub from: Option<String>,
    pub to: Option<String>,
    pub log_address: Option<String>,
    pub filter_type: FilterType,
    pub created_at: DateTime<Utc>,
}
