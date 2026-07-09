use crate::core::Config;
use anyhow::anyhow;
use connector_utils::types::db::{
    ABORT_CRSGEN_REQUEST_NOTIFICATION, ABORT_KEYGEN_REQUEST_NOTIFICATION,
    CRSGEN_REQUEST_NOTIFICATION, EventType, KEYGEN_REQUEST_NOTIFICATION,
    NEW_KMS_CONTEXT_NOTIFICATION, NEW_KMS_EPOCH_NOTIFICATION, PREP_KEYGEN_REQUEST_NOTIFICATION,
    PUBLIC_DECRYPT_REQUEST_NOTIFICATION, USER_DECRYPT_REQUEST_NOTIFICATION,
};
use sqlx::{Pool, Postgres, postgres::PgListener};
use std::time::Duration;
use tokio::{
    select,
    sync::mpsc::Sender,
    time::{Interval, MissedTickBehavior, interval},
};
use tracing::{debug, error, info, warn};

pub struct DbEventNotifier {
    /// The entity collecting Postgres notifications.
    db_listener: PgListener,
    /// The channel used to send notifications to the `DbEventPicker`.
    notif_sender: Sender<EventType>,
    /// The timeout for notifying the `DbEventPicker` to poll fast events (decryption for ex).
    db_fast_event_polling: Duration,
    /// The timeout for notifying the `DbEventPicker` to poll long events (prep keygen for ex).
    db_long_event_polling: Duration,
}

impl DbEventNotifier {
    pub fn new(
        db_listener: PgListener,
        notif_sender: Sender<EventType>,
        db_fast_event_polling: Duration,
        db_long_event_polling: Duration,
    ) -> Self {
        Self {
            db_listener,
            notif_sender,
            db_fast_event_polling,
            db_long_event_polling,
        }
    }

    pub async fn connect(
        db_pool: Pool<Postgres>,
        notif_sender: Sender<EventType>,
        config: &Config,
    ) -> anyhow::Result<Self> {
        let db_listener = PgListener::connect_with(&db_pool)
            .await
            .map_err(|e| anyhow!("Failed to init Postgres Listener: {e}"))?;

        let mut event_notifier = DbEventNotifier::new(
            db_listener,
            notif_sender,
            config.db_fast_event_polling,
            config.db_long_event_polling,
        );
        event_notifier
            .listen()
            .await
            .map_err(|e| anyhow!("Failed to listen to events: {e}"))?;

        Ok(event_notifier)
    }

    async fn listen(&mut self) -> sqlx::Result<()> {
        self.db_listener
            .listen(PUBLIC_DECRYPT_REQUEST_NOTIFICATION)
            .await?;
        self.db_listener
            .listen(USER_DECRYPT_REQUEST_NOTIFICATION)
            .await?;
        self.db_listener
            .listen(PREP_KEYGEN_REQUEST_NOTIFICATION)
            .await?;
        self.db_listener.listen(KEYGEN_REQUEST_NOTIFICATION).await?;
        self.db_listener.listen(CRSGEN_REQUEST_NOTIFICATION).await?;
        self.db_listener
            .listen(ABORT_KEYGEN_REQUEST_NOTIFICATION)
            .await?;
        self.db_listener
            .listen(ABORT_CRSGEN_REQUEST_NOTIFICATION)
            .await?;
        self.db_listener
            .listen(NEW_KMS_CONTEXT_NOTIFICATION)
            .await?;
        self.db_listener.listen(NEW_KMS_EPOCH_NOTIFICATION).await?;
        Ok(())
    }

    pub async fn start(mut self) {
        use EventType::*;
        let mut fast = EventTicker::new(
            self.db_fast_event_polling,
            &[PublicDecryptionRequest, UserDecryptionRequest],
        );
        let mut long = EventTicker::new(
            self.db_long_event_polling,
            &[
                PrepKeygenRequest,
                KeygenRequest,
                CrsgenRequest,
                // TODO(dp): isn't this "fast"?
                AbortKeygenRequest,
                // TODO(dp): isn't this "fast"?
                AbortCrsgenRequest,
                // TODO(dp): isn't this "fast"?
                NewKmsContext,
                // TODO(dp): isn't this "fast"?
                NewKmsEpoch,
            ],
        );

        loop {
            let all_sent = select! {
                event_types = fast.tick() => self.notify_all(event_types).await,
                event_types = long.tick() => self.notify_all(event_types).await,
                result = self.db_listener.recv() => match result.map(EventType::try_from) {
                    Ok(Ok(notif)) => {
                        info!("Received Postgres notification: {}", notif.pg_notification());

                        if fast.contains(notif) {
                            fast.reset();
                        } else if long.contains(notif) {
                            long.reset();
                        }

                        self.notify_all(&[notif]).await
                    }
                    Ok(Err(e)) => {
                        warn!("Event notification parsing error: {e}");
                        continue;
                    }
                    Err(e) => {
                        warn!("Error while listening for Postgres notification: {e}");
                        continue;
                    }
                },
            };

            if !all_sent {
                break error!("Notification channel was closed!");
            }
        }
    }

    /// Sends every `kind` to the `DbEventPicker`. Returns `false` if the channel closed early.
    async fn notify_all(&self, kinds: &[EventType]) -> bool {
        for kind in kinds {
            if self.notif_sender.send(*kind).await.is_err() {
                // TODO(dp): or should we continue to send the rest?
                return false;
            }
        }
        true
    }
}

/// Wrapper of `tokio::time::Interval` that ticks into the `EventType`s it polls/resets for.
struct EventTicker {
    /// The interval at which to sweep this ticker's event types.
    ticker: Interval,

    /// Which event types this ticker is responsible for.
    kinds: &'static [EventType],
}

impl EventTicker {
    fn new(polling: Duration, kinds: &'static [EventType]) -> Self {
        let mut ticker = interval(polling);

        // We don't want to spam the `DbEventPicker` with notifications if the
        // `DbEventNotifier` was blocked by the channel being full, so we skip missed tick.
        ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);

        Self { ticker, kinds }
    }

    async fn tick(&mut self) -> &'static [EventType] {
        self.ticker.tick().await;
        debug!("polling triggered for {:?}", self.kinds);
        self.kinds
    }

    fn contains(&self, kind: EventType) -> bool {
        self.kinds.contains(&kind)
    }

    fn reset(&mut self) {
        self.ticker.reset();
    }
}
