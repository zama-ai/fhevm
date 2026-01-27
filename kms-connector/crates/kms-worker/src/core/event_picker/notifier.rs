use crate::core::Config;
use anyhow::anyhow;
use connector_utils::types::db::{
    CRSGEN_REQUEST_NOTIFICATION, EventType, KEY_RESHARE_SAME_SET_NOTIFICATION,
    KEYGEN_REQUEST_NOTIFICATION, PREP_KEYGEN_REQUEST_NOTIFICATION, PRSS_INIT_NOTIFICATION,
    PUBLIC_DECRYPT_REQUEST_NOTIFICATION, USER_DECRYPT_REQUEST_NOTIFICATION,
};
use sqlx::{Pool, Postgres, postgres::PgListener};
use std::time::Duration;
use tokio::{
    select,
    sync::mpsc::Sender,
    time::{Instant, Interval, MissedTickBehavior, interval},
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
        self.db_listener.listen(PRSS_INIT_NOTIFICATION).await?;
        self.db_listener
            .listen(KEY_RESHARE_SAME_SET_NOTIFICATION)
            .await
    }

    pub async fn start(mut self) {
        let db_fast_event_polling = self.db_fast_event_polling;
        let db_long_event_polling = self.db_long_event_polling;

        let mut public_decrypt_ticker =
            EventTicker::new(db_fast_event_polling, EventType::PublicDecryptionRequest);
        let mut user_decrypt_ticker =
            EventTicker::new(db_fast_event_polling, EventType::UserDecryptionRequest);
        let mut prep_keygen_ticker =
            EventTicker::new(db_long_event_polling, EventType::PrepKeygenRequest);
        let mut keygen_ticker = EventTicker::new(db_long_event_polling, EventType::KeygenRequest);
        let mut crsgen_ticker = EventTicker::new(db_long_event_polling, EventType::CrsgenRequest);
        let mut prss_init_ticker = EventTicker::new(db_long_event_polling, EventType::PrssInit);
        let mut key_reshare_ticker =
            EventTicker::new(db_long_event_polling, EventType::KeyReshareSameSet);

        loop {
            let notification = select! {
                _ = public_decrypt_ticker.tick() => public_decrypt_ticker.deliver(),
                _ = user_decrypt_ticker.tick() => user_decrypt_ticker.deliver(),
                _ = prep_keygen_ticker.tick() => prep_keygen_ticker.deliver(),
                _ = keygen_ticker.tick() => keygen_ticker.deliver(),
                _ = crsgen_ticker.tick() => crsgen_ticker.deliver(),
                _ = prss_init_ticker.tick() => prss_init_ticker.deliver(),
                _ = key_reshare_ticker.tick() => key_reshare_ticker.deliver(),
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

            match notification {
                EventType::PublicDecryptionRequest => public_decrypt_ticker.reset(),
                EventType::UserDecryptionRequest => user_decrypt_ticker.reset(),
                EventType::PrepKeygenRequest => prep_keygen_ticker.reset(),
                EventType::KeygenRequest => keygen_ticker.reset(),
                EventType::CrsgenRequest => crsgen_ticker.reset(),
                EventType::PrssInit => prss_init_ticker.reset(),
                EventType::KeyReshareSameSet => key_reshare_ticker.reset(),
            }

            if self.notif_sender.send(notification).await.is_err() {
                break error!("Notification channel was closed!");
            }
        }
    }
}

/// Wrapper of `tokio::time::Interval` that can deliver `EventType` notification.
struct EventTicker {
    /// The interval at which to check for new responses.
    ticker: Interval,

    /// The `EventType` kind of notification to deliver.
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

    pub async fn tick(&mut self) -> Instant {
        self.ticker.tick().await
    }

    pub fn reset(&mut self) {
        self.ticker.reset();
    }

    pub fn deliver(&self) -> EventType {
        debug!("{} polling triggered", self.kind);
        self.kind
    }
}
