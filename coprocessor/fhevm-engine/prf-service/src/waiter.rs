//! Wakes the requests that wait for a pending row of `pads`.

use crate::{
    pads::{read, PadState},
    path::PrfPath,
};
use alloy::primitives::B256;
use anyhow::{bail, Context};
use sqlx::{postgres::PgListener, PgPool};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;
use tracing::{debug, warn};

/// A trigger on `pads` sends it for a complete or failed row.
pub const PAD_READY: &str = "pad_ready";

type RowKey = (u16, B256, u16);

/// Wakes the requests of one pod when their row is complete or failed.
pub struct PadWaiter {
    pool: PgPool,
    poll_interval: Duration,
    waiting: Mutex<HashMap<RowKey, Arc<Notify>>>,
}

impl PadWaiter {
    pub fn new(pool: PgPool, poll_interval: Duration) -> Self {
        Self {
            pool,
            poll_interval,
            waiting: Mutex::default(),
        }
    }

    /// Waits until the row of `path` is complete or failed.
    pub async fn wait(&self, path: &PrfPath) -> anyhow::Result<PadState> {
        let waiting = self.register((path.prf, path.label, path.pad_length));
        loop {
            let notified = waiting.notify.notified();
            tokio::pin!(notified);
            // Before the read, so a wake-up during the read is not lost.
            notified.as_mut().enable();
            match read(&self.pool, path).await? {
                Some(PadState::Pending) => notified.await,
                Some(state) => return Ok(state),
                None => bail!("no row for {path:?}"),
            }
        }
    }

    /// Listens for `pad_ready` until `cancel`. On an error, the process must exit.
    pub async fn run(&self, cancel: CancellationToken) -> anyhow::Result<()> {
        let mut listener = PgListener::connect_with(&self.pool).await?;
        listener.listen(PAD_READY).await?;
        let mut poll = tokio::time::interval(self.poll_interval);
        loop {
            tokio::select! {
                _ = cancel.cancelled() => return Ok(()),
                notification = listener.try_recv() => match notification? {
                    Some(notification) => match parse_payload(notification.payload()) {
                        Some(key) => self.wake(&key),
                        None => debug!(payload = notification.payload(), "ignored a bad {PAD_READY} payload"),
                    },
                    // sqlx connected again. Read the rows: notifications of the gap are lost.
                    None => {
                        warn!("the {PAD_READY} connection was lost and restored");
                        if let Err(err) = self.wake_finished().await {
                            warn!(error = %err, "pad poll failed");
                        }
                    }
                },
                // Postgres can lose a notification. A failed poll does not stop the waiter.
                _ = poll.tick() => {
                    if let Err(err) = self.wake_finished().await {
                        warn!(error = %err, "pad poll failed");
                    }
                }
            }
        }
    }

    fn register(&self, key: RowKey) -> Waiting<'_> {
        let notify = self
            .waiting
            .lock()
            .expect("waiting lock")
            .entry(key)
            .or_default()
            .clone();
        Waiting {
            waiter: self,
            key,
            notify,
        }
    }

    fn wake(&self, key: &RowKey) {
        if let Some(notify) = self.waiting.lock().expect("waiting lock").get(key) {
            notify.notify_waiters();
        }
    }

    async fn wake_finished(&self) -> anyhow::Result<()> {
        let mut prfs = Vec::new();
        let mut labels = Vec::new();
        let mut pad_lengths = Vec::new();
        for (prf, label, pad_length) in self.waiting.lock().expect("waiting lock").keys() {
            prfs.push(i32::from(*prf));
            labels.push(label.to_vec());
            pad_lengths.push(i32::from(*pad_length));
        }
        if prfs.is_empty() {
            return Ok(());
        }
        let finished = sqlx::query!(
            "SELECT p.prf_instance, p.label, p.pad_length
               FROM UNNEST($1::INTEGER[], $2::BYTEA[], $3::INTEGER[]) AS w (prf_instance, label, pad_length)
               JOIN pads p USING (prf_instance, label, pad_length)
              WHERE p.signature IS NOT NULL OR p.is_error",
            &prfs,
            &labels,
            &pad_lengths,
        )
        .fetch_all(&self.pool)
        .await?;
        for row in finished {
            let prf = u16::try_from(row.prf_instance).context("prf_instance")?;
            let label = B256::try_from(row.label.as_slice()).context("label")?;
            let pad_length = u16::try_from(row.pad_length).context("pad_length")?;
            self.wake(&(prf, label, pad_length));
        }
        Ok(())
    }
}

/// A request in the wait list. On drop, it removes its row from the list if no other request waits.
struct Waiting<'a> {
    waiter: &'a PadWaiter,
    key: RowKey,
    notify: Arc<Notify>,
}

impl Drop for Waiting<'_> {
    fn drop(&mut self) {
        let mut waiting = self.waiter.waiting.lock().expect("waiting lock");
        // Only the list and this request hold the Notify: no other request waits.
        if Arc::strong_count(&self.notify) == 2 {
            waiting.remove(&self.key);
        }
    }
}

fn parse_payload(payload: &str) -> Option<RowKey> {
    let mut parts = payload.split(':');
    let key = (
        parts.next()?.parse().ok()?,
        parts.next()?.parse().ok()?,
        parts.next()?.parse().ok()?,
    );
    parts.next().is_none().then_some(key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;

    #[tokio::test]
    async fn wait_list_removes_a_row_after_its_last_request() {
        let pool = PgPoolOptions::new()
            .connect_lazy("postgres://localhost/unused")
            .unwrap();
        let waiter = PadWaiter::new(pool, Duration::from_secs(1));
        let key = (0, B256::repeat_byte(1), 64);
        let first = waiter.register(key);
        let second = waiter.register(key);
        assert_eq!(waiter.waiting.lock().unwrap().len(), 1);
        drop(first);
        assert_eq!(waiter.waiting.lock().unwrap().len(), 1);
        drop(second);
        assert!(waiter.waiting.lock().unwrap().is_empty());
    }

    #[test]
    fn parses_only_prf_label_and_pad_length() {
        let label = B256::repeat_byte(0xab);
        assert_eq!(
            parse_payload(&format!("7:{label:x}:64")),
            Some((7, label, 64))
        );
        assert_eq!(parse_payload(&format!("7:{label:x}")), None);
        assert_eq!(parse_payload(&format!("7:{label:x}:64:1")), None);
        assert_eq!(parse_payload(&format!("70000:{label:x}:64")), None);
        assert_eq!(parse_payload("7:ab:64"), None);
    }
}
