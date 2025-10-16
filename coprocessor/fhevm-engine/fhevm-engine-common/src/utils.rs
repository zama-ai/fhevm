use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use serde::{de::DeserializeOwned, Serialize};
use tfhe::{named::Named, prelude::ParameterSetConformant, Unversionize, Versionize};

use sqlx::postgres::PgConnectOptions;
use std::fmt;
use std::str::FromStr;

use crate::types::FhevmError;

pub const SAFE_SER_DESER_LIMIT: u64 = 1024 * 1024 * 16;
pub const SAFE_SER_DESER_KEY_LIMIT: u64 = 1024 * 1024 * 1024 * 2;
pub const SAFE_SER_DESER_SNS_KEY_LIMIT: u64 = 1024 * 1024 * 1024 * 2;

pub fn safe_serialize<T: Serialize + Named + Versionize>(object: &T) -> Vec<u8> {
    let mut out = vec![];
    tfhe::safe_serialization::safe_serialize(object, &mut out, SAFE_SER_DESER_LIMIT)
        .expect("safe serialize succeeds");
    out
}

pub fn safe_deserialize<T: DeserializeOwned + Named + Unversionize>(
    input: &[u8],
) -> Result<T, FhevmError> {
    tfhe::safe_serialization::safe_deserialize(input, SAFE_SER_DESER_LIMIT)
        .map_err(|e| FhevmError::DeserializationError(e.into()))
}

pub fn safe_deserialize_conformant<
    T: DeserializeOwned + Named + Unversionize + ParameterSetConformant,
>(
    input: &[u8],
    parameter_set: &T::ParameterSet,
) -> Result<T, FhevmError> {
    tfhe::safe_serialization::safe_deserialize_conformant(
        input,
        SAFE_SER_DESER_LIMIT,
        parameter_set,
    )
    .map_err(|e| FhevmError::DeserializationError(e.into()))
}

pub fn safe_serialize_key<T: Serialize + Named + Versionize>(object: &T) -> Vec<u8> {
    let mut out = vec![];
    tfhe::safe_serialization::safe_serialize(object, &mut out, SAFE_SER_DESER_KEY_LIMIT)
        .expect("safe serialize succeeds");
    out
}

pub fn safe_deserialize_key<T: DeserializeOwned + Named + Unversionize>(
    input: &[u8],
) -> Result<T, FhevmError> {
    tfhe::safe_serialization::safe_deserialize(input, SAFE_SER_DESER_KEY_LIMIT)
        .map_err(|e| FhevmError::DeserializationError(e.into()))
}

pub fn safe_deserialize_sns_key<T: DeserializeOwned + Named + Unversionize>(
    input: &[u8],
) -> Result<T, FhevmError> {
    tfhe::safe_serialization::safe_deserialize(input, SAFE_SER_DESER_SNS_KEY_LIMIT)
        .map_err(|e| FhevmError::DeserializationError(e.into()))
}

// Print first 4 and last 4 bytes of a blob as hex
pub fn compact_hex(blob: &[u8]) -> String {
    const OFFSET: usize = 8;
    match blob.len() {
        0 => String::from("0x"),
        len if len <= 2 * OFFSET => format!("0x{}", hex::encode(blob)),
        _ => {
            let hex_str = hex::encode(blob);
            format!(
                "0x{}...{}",
                &hex_str[..OFFSET],
                &hex_str[hex_str.len() - OFFSET..]
            )
        }
    }
}

#[derive(Clone, Debug)]
pub struct HeartBeat {
    timestamp_origin: std::time::Instant,
    timestamp: Arc<AtomicU64>,
}
impl HeartBeat {
    pub fn new() -> Self {
        Self {
            timestamp_origin: std::time::Instant::now(),
            timestamp: Arc::new(AtomicU64::new(0)),
        }
    }

    fn now_timestamp(&self) -> u64 {
        self.timestamp_origin.elapsed().as_secs()
    }

    pub fn update(&self) {
        let now = self.now_timestamp();
        self.timestamp.store(now, Ordering::Relaxed);
    }

    pub fn is_recent(&self, freshness: &Duration) -> bool {
        let elapsed = self.now_timestamp() - self.timestamp.load(Ordering::Relaxed);
        elapsed <= freshness.as_secs()
    }
}

impl Default for HeartBeat {
    fn default() -> Self {
        Self::new()
    }
}
/// Simple wrapper around Database URL string to provide
/// url constraints and masking functionality.
#[derive(Clone)]
pub struct DatabaseURL(String);

impl From<&str> for DatabaseURL {
    fn from(s: &str) -> Self {
        let url: Self = Self(s.to_owned());
        url.parse().expect("DatabaseURL is valid");
        url
    }
}
impl From<String> for DatabaseURL {
    fn from(s: String) -> Self {
        let url: Self = Self(s);
        url.parse().expect("DatabaseURL is valid");
        url
    }
}

impl Default for DatabaseURL {
    fn default() -> Self {
        let url = std::env::var("DATABASE_URL")
            .unwrap_or("postgres://postgres:postgres@localhost:5432/coprocessor".to_owned());

        let app_name = Self::default_app_name();
        Self::new_with_app_name(&url, &app_name)
    }
}

impl DatabaseURL {
    /// Create a new DatabaseURL, appending application_name if not present
    /// If the base URL already contains an application_name, it will be preserved.
    ///
    /// application_name is useful for identifying the source of DB conns
    pub fn new_with_app_name(base: &str, app_name: &str) -> Self {
        // Append application_name if not present
        let mut url = base.to_owned();
        if !url.contains("application_name=") {
            if url.contains('?') {
                url.push_str(&format!("&application_name={}", app_name));
            } else {
                url.push_str(&format!("?application_name={}", app_name));
            }
        }
        let url: Self = Self(url);
        let _ = url.parse().expect("DatabaseURL is valid");
        url
    }

    /// Get default app name from the executable name
    fn default_app_name() -> String {
        std::env::args()
            .next()
            .and_then(|path| {
                std::path::Path::new(&path)
                    .file_name()
                    .map(|s| s.to_string_lossy().into_owned())
            })
            .unwrap_or_else(|| "unknown".to_string())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    fn mask_password(options: &PgConnectOptions) -> String {
        let new_url = format!(
            "postgres://{}:{}@{}:{}/{}?application_name={}",
            options.get_username(),
            "*****",
            options.get_host(),
            options.get_port(),
            options.get_database().unwrap_or_default(),
            options.get_application_name().unwrap_or_default()
        );
        new_url
    }

    pub fn parse(&self) -> Result<PgConnectOptions, sqlx::Error> {
        PgConnectOptions::from_str(self.as_str())
    }
}

impl fmt::Display for DatabaseURL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match PgConnectOptions::from_str(self.as_str()) {
            Ok(options) => {
                write!(f, "{:?}", Self::mask_password(&options))
            }
            Err(_) => write!(f, "Invalid DatabaseURL"),
        }
    }
}

impl fmt::Debug for DatabaseURL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match PgConnectOptions::from_str(self.as_str()) {
            Ok(options) => {
                write!(f, "{:?}", options.password("*****"))
            }
            Err(_) => write!(f, "Invalid DatabaseURL"),
        }
    }
}
impl FromStr for DatabaseURL {
    type Err = sqlx::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let _ = PgConnectOptions::from_str(s)?;
        Ok(Self(s.to_owned()))
    }
}
