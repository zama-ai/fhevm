use crate::core::Config;
use anyhow::anyhow;
use sqlx::{
    Pool, Postgres,
    postgres::{PgListener, PgNotification},
};
use std::{
    fmt::{self, Display},
    str::FromStr,
    time::Duration,
};
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
    notif_sender: Sender<EventNotification>,
    /// The timeout for notifying the `DbEventPicker` to poll fast events (decryption for ex).
    db_fast_event_polling: Duration,
    /// The timeout for notifying the `DbEventPicker` to poll long events (prep keygen for ex).
    db_long_event_polling: Duration,
}

impl DbEventNotifier {
    pub fn new(
        db_listener: PgListener,
        notif_sender: Sender<EventNotification>,
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
        notif_sender: Sender<EventNotification>,
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
        self.db_listener.listen(PUBLIC_DECRYPT_NOTIFICATION).await?;
        self.db_listener.listen(USER_DECRYPT_NOTIFICATION).await?;
        self.db_listener.listen(PREP_KEYGEN_NOTIFICATION).await?;
        self.db_listener.listen(KEYGEN_NOTIFICATION).await?;
        self.db_listener.listen(CRSGEN_NOTIFICATION).await?;
        self.db_listener.listen(PRSS_INIT_NOTIFICATION).await?;
        self.db_listener
            .listen(KEY_RESHARE_SAME_SET_NOTIFICATION)
            .await
    }

    pub async fn start(mut self) {
        let db_fast_event_polling = self.db_fast_event_polling;
        let db_long_event_polling = self.db_long_event_polling;

        let mut public_decrypt_ticker =
            EventTicker::new(db_fast_event_polling, EventNotification::PublicDecryption);
        let mut user_decrypt_ticker =
            EventTicker::new(db_fast_event_polling, EventNotification::UserDecryption);
        let mut prep_keygen_ticker =
            EventTicker::new(db_long_event_polling, EventNotification::PrepKeygen);
        let mut keygen_ticker = EventTicker::new(db_long_event_polling, EventNotification::Keygen);
        let mut crsgen_ticker = EventTicker::new(db_long_event_polling, EventNotification::Crsgen);
        let mut prss_init_ticker =
            EventTicker::new(db_long_event_polling, EventNotification::PrssInit);
        let mut key_reshare_ticker =
            EventTicker::new(db_long_event_polling, EventNotification::KeyReshareSameSet);

        loop {
            let notification = select! {
                _ = public_decrypt_ticker.tick() => public_decrypt_ticker.deliver(),
                _ = user_decrypt_ticker.tick() => user_decrypt_ticker.deliver(),
                _ = prep_keygen_ticker.tick() => prep_keygen_ticker.deliver(),
                _ = keygen_ticker.tick() => keygen_ticker.deliver(),
                _ = crsgen_ticker.tick() => crsgen_ticker.deliver(),
                _ = prss_init_ticker.tick() => prss_init_ticker.deliver(),
                _ = key_reshare_ticker.tick() => key_reshare_ticker.deliver(),
                result = self.db_listener.recv() => match result.map(EventNotification::try_from) {
                    Ok(Ok(notif)) => {
                        info!("Received Postgres notification: {notif}");
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
                EventNotification::PublicDecryption => public_decrypt_ticker.reset(),
                EventNotification::UserDecryption => user_decrypt_ticker.reset(),
                EventNotification::PrepKeygen => prep_keygen_ticker.reset(),
                EventNotification::Keygen => keygen_ticker.reset(),
                EventNotification::Crsgen => crsgen_ticker.reset(),
                EventNotification::PrssInit => prss_init_ticker.reset(),
                EventNotification::KeyReshareSameSet => key_reshare_ticker.reset(),
            }

            if self.notif_sender.send(notification).await.is_err() {
                break error!("Notification channel was closed!");
            }
        }
    }
}

/// Enum representing the different types of events that can be notified to the `DbEventPicker`.
#[derive(Clone, Copy)]
pub enum EventNotification {
    PublicDecryption,
    UserDecryption,
    PrepKeygen,
    Keygen,
    Crsgen,
    PrssInit,
    KeyReshareSameSet,
}

impl TryFrom<PgNotification> for EventNotification {
    type Error = anyhow::Error;

    fn try_from(value: PgNotification) -> Result<Self, Self::Error> {
        value.channel().parse()
    }
}

impl FromStr for EventNotification {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            PUBLIC_DECRYPT_NOTIFICATION => Ok(Self::PublicDecryption),
            USER_DECRYPT_NOTIFICATION => Ok(Self::UserDecryption),
            PREP_KEYGEN_NOTIFICATION => Ok(Self::PrepKeygen),
            KEYGEN_NOTIFICATION => Ok(Self::Keygen),
            CRSGEN_NOTIFICATION => Ok(Self::Crsgen),
            PRSS_INIT_NOTIFICATION => Ok(Self::PrssInit),
            KEY_RESHARE_SAME_SET_NOTIFICATION => Ok(Self::KeyReshareSameSet),
            s => Err(anyhow!("Unknown notification channel: {s}")),
        }
    }
}

impl Display for EventNotification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventNotification::PublicDecryption => write!(f, "{PUBLIC_DECRYPT_NOTIFICATION}"),
            EventNotification::UserDecryption => write!(f, "{USER_DECRYPT_NOTIFICATION}"),
            EventNotification::PrepKeygen => write!(f, "{PREP_KEYGEN_NOTIFICATION}"),
            EventNotification::Keygen => write!(f, "{KEYGEN_NOTIFICATION}"),
            EventNotification::Crsgen => write!(f, "{CRSGEN_NOTIFICATION}"),
            EventNotification::PrssInit => write!(f, "{PRSS_INIT_NOTIFICATION}"),
            EventNotification::KeyReshareSameSet => {
                write!(f, "{KEY_RESHARE_SAME_SET_NOTIFICATION}")
            }
        }
    }
}

impl EventNotification {
    pub fn event_str(&self) -> &'static str {
        match self {
            EventNotification::PublicDecryption => "PublicDecryptionRequest",
            EventNotification::UserDecryption => "UserDecryptionRequest",
            EventNotification::PrepKeygen => "PrepKeygenRequest",
            EventNotification::Keygen => "KeygenRequest",
            EventNotification::Crsgen => "CrsgenRequest",
            EventNotification::PrssInit => "PrssInit",
            EventNotification::KeyReshareSameSet => "KeyReshareSameSet",
        }
    }
}
/// Wrapper of `tokio::time::Interval` that can deliver `EventNotification`.
struct EventTicker {
    /// The interval at which to check for new responses.
    ticker: Interval,

    /// The `EventNotification` kind to deliver.
    kind: EventNotification,
}

impl EventTicker {
    pub fn new(polling: Duration, kind: EventNotification) -> Self {
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

    pub fn deliver(&self) -> EventNotification {
        debug!("{} polling triggered", self.kind.event_str());
        self.kind
    }
}

// Postgres notifications
const PUBLIC_DECRYPT_NOTIFICATION: &str = "public_decryption_request_available";
const USER_DECRYPT_NOTIFICATION: &str = "user_decryption_request_available";
const PREP_KEYGEN_NOTIFICATION: &str = "prep_keygen_request_available";
const KEYGEN_NOTIFICATION: &str = "keygen_request_available";
const CRSGEN_NOTIFICATION: &str = "crsgen_request_available";
const PRSS_INIT_NOTIFICATION: &str = "prss_init_available";
const KEY_RESHARE_SAME_SET_NOTIFICATION: &str = "key_reshare_same_set_available";
