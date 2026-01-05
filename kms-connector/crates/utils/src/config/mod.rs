pub mod contract;
mod deserialize;
mod error;
mod wallet;

pub use contract::ContractConfig;
pub use deserialize::DeserializeConfig;
pub use error::{Error, Result};
use sqlx::postgres::types::PgInterval;
pub use wallet::{AwsKmsConfig, KmsWallet};

use serde::{Deserializer, Serializer};
use std::time::Duration;

pub fn serialize_pg_interval<S>(
    interval: &PgInterval,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // When deserialized from `Duration`, which is our case, the `months` and `days` fields of
    // the `PgInterval` are set to 0, so we just need to use the `microseconds` field.
    // https://docs.rs/sqlx-postgres/0.8.6/src/sqlx_postgres/types/interval.rs.html#204
    let duration = Duration::from_micros(interval.microseconds as u64);
    humantime_serde::serialize(&duration, serializer)
}

pub fn deserialize_pg_interval<'de, D>(deserializer: D) -> std::result::Result<PgInterval, D::Error>
where
    D: Deserializer<'de>,
{
    humantime_serde::deserialize(deserializer).and_then(|d: Duration| {
        PgInterval::try_from(d).map_err(|e| serde::de::Error::custom(e.to_string()))
    })
}

pub fn default_database_pool_size() -> u32 {
    16
}
