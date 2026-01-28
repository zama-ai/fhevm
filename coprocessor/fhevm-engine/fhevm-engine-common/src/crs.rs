use anyhow::Result;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use tfhe::zk::CompactPkeCrs;

use crate::utils::safe_deserialize_key;

pub type CrsId = Vec<u8>;

#[derive(Clone)]
pub struct Crs {
    pub crs_id: CrsId,
    pub crs: CompactPkeCrs,
}

#[derive(Clone, Default)]
pub struct CrsCache {
    latest: Option<Arc<Crs>>,
}

impl CrsCache {
    pub async fn load(pool: &PgPool) -> Result<Self> {
        let row = sqlx::query("SELECT crs_id, crs FROM crs ORDER BY sequence_number DESC LIMIT 1")
            .fetch_optional(pool)
            .await?;

        let latest = row
            .map(|row| {
                Ok::<_, anyhow::Error>(Arc::new(Crs {
                    crs_id: row.try_get("crs_id")?,
                    crs: safe_deserialize_key(row.try_get("crs")?)?,
                }))
            })
            .transpose()?;

        Ok(Self { latest })
    }

    pub fn get_latest(&self) -> Option<&Crs> {
        self.latest.as_deref()
    }
}
