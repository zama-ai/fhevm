use crate::orchestrator::traits::{Event, EventHandler};
use async_trait::async_trait;
use std::sync::Mutex;
use tokio::sync::oneshot;

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
