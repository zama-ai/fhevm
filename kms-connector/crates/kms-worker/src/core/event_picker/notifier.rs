use crate::core::Config;
use anyhow::anyhow;
use connector_utils::types::db::{
    ABORT_CRSGEN_REQUEST_NOTIFICATION, ABORT_KEYGEN_REQUEST_NOTIFICATION,
    CRSGEN_REQUEST_NOTIFICATION, EventType, KEYGEN_REQUEST_NOTIFICATION,
    NEW_KMS_CONTEXT_NOTIFICATION, NEW_KMS_EPOCH_NOTIFICATION, PREP_KEYGEN_REQUEST_NOTIFICATION,
    PUBLIC_DECRYPT_REQUEST_NOTIFICATION, USER_DECRYPT_REQUEST_NOTIFICATION,
};
use futures::future::select_all;
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

    fn ticker(&self, kind: EventType) -> EventTicker {
        use EventType::*;
        let polling = match kind {
            PublicDecryptionRequest => self.db_fast_event_polling,
            UserDecryptionRequest => self.db_fast_event_polling,
            PrepKeygenRequest => self.db_long_event_polling,
            KeygenRequest => self.db_long_event_polling,
            CrsgenRequest => self.db_long_event_polling,
            AbortKeygenRequest => self.db_long_event_polling,
            AbortCrsgenRequest => self.db_long_event_polling,
            NewKmsContext => self.db_long_event_polling,
            NewKmsEpoch => self.db_long_event_polling,
        };
        EventTicker::new(polling, kind)
    }

    pub async fn start(mut self) {
        use EventType::*;
        let mut tickers = [
            self.ticker(PublicDecryptionRequest),
            self.ticker(UserDecryptionRequest),
            self.ticker(PrepKeygenRequest),
            self.ticker(KeygenRequest),
            self.ticker(CrsgenRequest),
            self.ticker(AbortKeygenRequest),
            self.ticker(AbortCrsgenRequest),
            self.ticker(NewKmsContext),
            self.ticker(NewKmsEpoch),
        ];

        loop {
            let notification = select! {
                (kind, _idx, _rest) = select_all(tickers.iter_mut().map(|t| Box::pin(t.tick()))) => kind,
                result = self.db_listener.recv() => match result.map(EventType::try_from) {
                    Ok(Ok(notif)) => {
                        info!("Received Postgres notification: {}", notif.pg_notification());
                        notif
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

            if let Some(ticker) = tickers.iter_mut().find(|t| t.kind == notification) {
                ticker.reset();
            } else {
                warn!("Notification from unknown event type: {notification:?}");
            }

            if self.notif_sender.send(notification).await.is_err() {
                break error!("Notification channel was closed!");
            }
        }
    }
}

/// Wrapper of `tokio::time::Interval` that ticks into an `EventType` notification.
struct EventTicker {
    /// The interval at which to check for new responses.
    ticker: Interval,

    /// The `EventType` kind of notification this ticker represents.
    kind: EventType,
}

impl EventTicker {
    pub fn new(polling: Duration, kind: EventType) -> Self {
        let mut ticker = interval(polling);

        // We don't want to spam the `DbEventPicker` with notifications if the
        // `DbEventNotifier` was blocked by the channel being full, so we skip missed tick.
        ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);

        Self { ticker, kind }
    }

    pub async fn tick(&mut self) -> EventType {
        self.ticker.tick().await;
        debug!("{} polling triggered", self.kind);
        self.kind
    }

    pub fn reset(&mut self) {
        self.ticker.reset();
    }
}
