use crate::orchestrator::traits::{Event, EventHandler};
use serde::{de, Deserialize, Deserializer};
use serde_json::Value;
use std::sync::Mutex;
use tokio::sync::oneshot;
use validator::ValidationError;

pub struct OnceHandler<T> {
    tx: Mutex<Option<oneshot::Sender<T>>>,
}

impl<T> OnceHandler<T> {
    pub fn new() -> (Self, oneshot::Receiver<T>) {
        let (tx, rx) = oneshot::channel();
        (
            Self {
                tx: Mutex::new(Some(tx)),
            },
            rx,
        )
    }
}

#[async_trait::async_trait]
impl<E> EventHandler<E> for OnceHandler<E>
where
    E: Event + Send + Sync + 'static,
{
    async fn handle_event(&self, event: E) {
        let mut lock = self.tx.lock().unwrap();
        if let Some(tx) = lock.take() {
            let _ = tx.send(event);
        }
    }
}

pub fn de_string_or_number<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s,
        Value::Number(num) => format!("{num}"),
        _ => return Err(de::Error::custom("wrong type")),
    })
}

// Custom validation function for a standard Ethereum-style blockchain address.
// It must start with "0x", be 42 characters long, and contain hex characters.
pub fn validate_blockchain_address(address: &str) -> Result<(), ValidationError> {
    if !address.starts_with("0x") {
        return Err(ValidationError::new("must_start_with_0x"));
    }
    if address.len() != 42 {
        return Err(ValidationError::new("invalid_length"));
    }
    // The `hex` crate robustly checks if the string slice (after "0x") is valid hex.
    if hex::decode(&address[2..]).is_err() {
        return Err(ValidationError::new("invalid_hex_characters"));
    }
    Ok(())
}

// Custom validation function for a hex string that must NOT have a "0x" prefix.
pub fn validate_hex_string_no_prefix(hex_str: &str) -> Result<(), ValidationError> {
    if hex_str.starts_with("0x") {
        return Err(ValidationError::new("must_not_start_with_0x"));
    }
    if hex::decode(hex_str).is_err() {
        return Err(ValidationError::new("invalid_hex_characters"));
    }
    Ok(())
}
