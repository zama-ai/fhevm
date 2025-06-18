use crate::orchestrator::traits::{Event, EventHandler};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::Value;
use std::sync::Mutex;
use tokio::sync::oneshot;

use serde::{de, Deserializer};

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

#[async_trait]
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
        Value::Number(num) => format!("{}", num),
        _ => return Err(de::Error::custom("wrong type")),
    })
}
