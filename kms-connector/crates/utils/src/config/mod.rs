pub mod contract;
mod deserialize;
mod error;
mod wallet;

pub use contract::ContractConfig;
pub use deserialize::DeserializeConfig;
pub use error::{Error, Result};
pub use wallet::{AwsKmsConfig, KmsWallet, TestingPrivateKey};

use serde::{
    Deserialize, Deserializer, Serializer,
    de::{IntoDeserializer, SeqAccess, Visitor, value::SeqAccessDeserializer},
};
use sqlx::postgres::types::PgInterval;
use std::{fmt, marker::PhantomData, time::Duration};

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

/// Deserializes a `Vec<T>` field from either a single scalar `T` or a sequence of `T`.
///
/// Workaround for a config-rs limitation (see https://github.com/rust-cli/config-rs/issues/120)
/// where a scalar value is not automatically coerced into a single-element list.
///
/// Implemented via the `Visitor` trait (https://serde.rs/impl-deserialize.html#the-visitor-trait).
pub fn deserialize_one_or_many<'de, D, T>(deserializer: D) -> std::result::Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    struct OneOrManyVisitor<T>(PhantomData<T>);

    impl<'de, T> Visitor<'de> for OneOrManyVisitor<T>
    where
        T: Deserialize<'de>,
    {
        type Value = Vec<T>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a single value or a sequence of values")
        }

        fn visit_seq<A>(self, seq: A) -> std::result::Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            Vec::<T>::deserialize(SeqAccessDeserializer::new(seq))
        }

        fn visit_bool<E>(self, v: bool) -> std::result::Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            T::deserialize(v.into_deserializer()).map(|value| vec![value])
        }

        fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            T::deserialize(v.into_deserializer()).map(|value| vec![value])
        }

        fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            T::deserialize(v.into_deserializer()).map(|value| vec![value])
        }

        fn visit_f64<E>(self, v: f64) -> std::result::Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            T::deserialize(v.into_deserializer()).map(|value| vec![value])
        }

        fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            T::deserialize(v.into_deserializer()).map(|value| vec![value])
        }

        fn visit_string<E>(self, v: String) -> std::result::Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            T::deserialize(v.into_deserializer()).map(|value| vec![value])
        }
    }

    deserializer.deserialize_any(OneOrManyVisitor(PhantomData))
}
