use crate::core::event_processor::ProcessingError;
use alloy::primitives::U256;
use anyhow::anyhow;
use sqlx::{Pool, Postgres};

pub trait ContextManager: Send + Sync {
    fn validate_context(
        &self,
        context_id: U256,
    ) -> impl Future<Output = Result<(), ProcessingError>> + Send;
}

#[derive(Clone)]
pub struct DbContextManager {
    db_pool: Pool<Postgres>,
}

impl ContextManager for DbContextManager {
    async fn validate_context(&self, context_id: U256) -> Result<(), ProcessingError> {
        let context_row = sqlx::query!(
            "SELECT is_valid FROM kms_context WHERE id = $1",
            context_id.as_le_slice()
        )
        .fetch_optional(&self.db_pool)
        .await
        .map_err(|e| {
            ProcessingError::Recoverable(anyhow!(
                "Query to check context #{context_id} failed: {e}"
            ))
        })?
        .map(|c| c.is_valid);

        match context_row {
            None => Err(ProcessingError::Recoverable(anyhow!(
                "Context #{context_id} not found in DB"
            ))),
            Some(false) => Err(ProcessingError::Irrecoverable(anyhow!(
                "Context #{context_id} is invalid"
            ))),
            Some(true) => Ok(()),
        }
    }
}

impl DbContextManager {
    pub fn new(db_pool: Pool<Postgres>) -> Self {
        Self { db_pool }
    }
}
