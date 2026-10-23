//! Postgres listener waking up the connections waiting for an HTTP-sourced response row.

use crate::core::{WaiterRegistry, db::id_from_notification_payload};
use anyhow::anyhow;
use sqlx::{
    Pool, Postgres,
    postgres::{PgListener, PgNotification},
};
use std::sync::Arc;
use tracing::{debug, error, info, warn};

pub const HTTP_PUBLIC_DECRYPT_NOTIFICATION: &str = "http_public_decryption_response_available";
pub const HTTP_USER_DECRYPT_NOTIFICATION: &str = "http_user_decryption_response_available";

/// Listens to the `http_*_decryption_response_available` channels and wakes up the matching
/// waiters.
pub struct ResponseListener {
    db_listener: PgListener,
    waiter_registry: Arc<WaiterRegistry>,
}

impl ResponseListener {
    /// Connects a dedicated listener connection and subscribes to both HTTP response channels.
    pub async fn connect(
        db_pool: &Pool<Postgres>,
        waiter_registry: Arc<WaiterRegistry>,
    ) -> anyhow::Result<Self> {
        let mut db_listener = PgListener::connect_with(db_pool)
            .await
            .map_err(|e| anyhow!("Failed to init Postgres Listener: {e}"))?;
        db_listener
            .listen_all([
                HTTP_PUBLIC_DECRYPT_NOTIFICATION,
                HTTP_USER_DECRYPT_NOTIFICATION,
            ])
            .await
            .map_err(|e| anyhow!("Failed to listen to HTTP responses: {e}"))?;

        Ok(Self {
            db_listener,
            waiter_registry,
        })
    }

    /// Runs the listener until a listening error occurs.
    pub async fn start(mut self) {
        info!("Starting ResponseListener");
        loop {
            match self.db_listener.try_recv().await {
                Ok(Some(notification)) => self.handle_notif(notification),
                // `sqlx` already reconnected and re-issued the `LISTEN`s. Notifications emitted
                // during the gap are lost, so the in-flight waiters are failed fast.
                Ok(None) => {
                    warn!("Postgres listener connection was lost and re-established");
                    self.fail_in_flight_waiters();
                }
                Err(e) => {
                    error!("Error while listening for Postgres notifications: {e}");
                    self.fail_in_flight_waiters();
                    return info!("ResponseListener stopped");
                }
            }
        }
    }

    /// Drops the registered waiters, failing their requests with a retryable error.
    fn fail_in_flight_waiters(&self) {
        let in_flight = self.waiter_registry.len();
        if in_flight > 0 {
            warn!(
                "Failing the waiters of {in_flight} in-flight decryption(s): their response may have been missed"
            );
            self.waiter_registry.clear();
        }
    }

    fn handle_notif(&self, notification: PgNotification) {
        let id = match id_from_notification_payload(notification.payload()) {
            Ok(id) => id,
            Err(e) => return error!("Ignoring malformed response notification: {e}"),
        };
        debug!(decryption_id = %id, channel = notification.channel(), "Response notification received");

        if self.waiter_registry.wake(&id) {
            info!(decryption_id = %id, "Waiting connection(s) woken up");
        } else {
            // Another endpoint replica's request, or a client that already went away.
            debug!(decryption_id = %id, "Nobody waits for this response");
        }
    }
}
