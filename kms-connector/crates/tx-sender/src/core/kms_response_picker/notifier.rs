use crate::core::Config;
use anyhow::anyhow;
use sqlx::{
    Pool, Postgres,
    postgres::{PgListener, PgNotification},
};
use std::{str::FromStr, time::Duration};
use tokio::{
    select,
    sync::mpsc::Sender,
    time::{Instant, Interval, MissedTickBehavior, interval},
};
use tracing::{debug, error, info, warn};

pub struct DbKmsResponseNotifier {
    /// The entity collecting Postgres notifications.
    db_listener: PgListener,

    /// The channel used to send notifications to the `DbKmsResponsePicker`.
    notif_sender: Sender<KmsResponseNotification>,

    /// The timeout for notifying the `DbKmsResponsePicker` to poll responses.
    db_polling: Duration,
}

impl DbKmsResponseNotifier {
    pub fn new(
        db_listener: PgListener,
        notif_sender: Sender<KmsResponseNotification>,
        db_polling: Duration,
    ) -> Self {
        Self {
            db_listener,
            notif_sender,
            db_polling,
        }
    }

    /// Connects to the database.
    pub async fn connect(
        db_pool: Pool<Postgres>,
        notif_sender: Sender<KmsResponseNotification>,
        config: &Config,
    ) -> anyhow::Result<Self> {
        let db_listener = PgListener::connect_with(&db_pool)
            .await
            .map_err(|e| anyhow!("Failed to init Postgres Listener: {e}"))?;

        let mut kms_response_notifier =
            DbKmsResponseNotifier::new(db_listener, notif_sender, config.database_polling_timeout);
        kms_response_notifier
            .listen()
            .await
            .map_err(|e| anyhow!("Failed to listen to responses: {e}"))?;

        Ok(kms_response_notifier)
    }

    /// Starts listening to notifications from the database.
    async fn listen(&mut self) -> sqlx::Result<()> {
        self.db_listener.listen(PUBLIC_DECRYPT_NOTIFICATION).await?;
        self.db_listener.listen(USER_DECRYPT_NOTIFICATION).await?;
        self.db_listener.listen(PREP_KEYGEN_NOTIFICATION).await?;
        self.db_listener.listen(KEYGEN_NOTIFICATION).await?;
        self.db_listener.listen(CRSGEN_NOTIFICATION).await
    }

    /// Starts the `DbKmsResponseNotifier`.
    ///
    /// It will send a notification to the `DbKmsResponsePicker` each time:
    /// * a notification from the DB is received
    /// * the polling timeout for a specific response kind is reached
    pub async fn start(mut self) {
        let mut public_decrypt_ticker =
            KmsResponseTicker::new(self.db_polling, KmsResponseNotification::PublicDecryption);
        let mut user_decrypt_ticker =
            KmsResponseTicker::new(self.db_polling, KmsResponseNotification::UserDecryption);
        let mut prep_keygen_ticker =
            KmsResponseTicker::new(self.db_polling, KmsResponseNotification::PrepKeygen);
        let mut keygen_ticker =
            KmsResponseTicker::new(self.db_polling, KmsResponseNotification::Keygen);
        let mut crsgen_ticker =
            KmsResponseTicker::new(self.db_polling, KmsResponseNotification::Crsgen);

        loop {
            let notification = select! {
                _ = public_decrypt_ticker.tick() => public_decrypt_ticker.deliver(),
                _ = user_decrypt_ticker.tick() => user_decrypt_ticker.deliver(),
                _ = prep_keygen_ticker.tick() => prep_keygen_ticker.deliver(),
                _ = keygen_ticker.tick() => keygen_ticker.deliver(),
                _ = crsgen_ticker.tick() => crsgen_ticker.deliver(),
                result = self.db_listener.recv() => match result.map(KmsResponseNotification::try_from) {
                    Err(e) => {
                        warn!("Error while listening for Postgres notification: {e}");
                        continue;
                    }
                    Ok(Err(e)) => {
                        warn!("Response notification parsing error: {e}");
                        continue;
                    }
                    Ok(Ok(notif)) => {
                        info!("Received Postgres notification: {}", notif.pg_notification());

                        match notif {
                            KmsResponseNotification::PublicDecryption => public_decrypt_ticker.reset(),
                            KmsResponseNotification::UserDecryption => user_decrypt_ticker.reset(),
                            KmsResponseNotification::PrepKeygen => prep_keygen_ticker.reset(),
                            KmsResponseNotification::Keygen => keygen_ticker.reset(),
                            KmsResponseNotification::Crsgen => crsgen_ticker.reset(),
                        }

                        notif
                    }
                },
            };

            if self.notif_sender.send(notification).await.is_err() {
                break error!("Notification channel was closed!");
            }
        }
    }
}

/// Enum representing the different types of responses that can be notified to the
/// `DbKmsResponsePicker`.
#[derive(Clone, Copy)]
pub enum KmsResponseNotification {
    PublicDecryption,
    UserDecryption,
    PrepKeygen,
    Keygen,
    Crsgen,
}

impl TryFrom<PgNotification> for KmsResponseNotification {
    type Error = anyhow::Error;

    fn try_from(value: PgNotification) -> Result<Self, Self::Error> {
        value.channel().parse()
    }
}

impl FromStr for KmsResponseNotification {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            PUBLIC_DECRYPT_NOTIFICATION => Ok(Self::PublicDecryption),
            USER_DECRYPT_NOTIFICATION => Ok(Self::UserDecryption),
            PREP_KEYGEN_NOTIFICATION => Ok(Self::PrepKeygen),
            KEYGEN_NOTIFICATION => Ok(Self::Keygen),
            CRSGEN_NOTIFICATION => Ok(Self::Crsgen),
            s => Err(anyhow!("Unknown notification channel: {s}")),
        }
    }
}

impl KmsResponseNotification {
    pub fn pg_notification(&self) -> &'static str {
        match self {
            KmsResponseNotification::PublicDecryption => PUBLIC_DECRYPT_NOTIFICATION,
            KmsResponseNotification::UserDecryption => USER_DECRYPT_NOTIFICATION,
            KmsResponseNotification::PrepKeygen => PREP_KEYGEN_NOTIFICATION,
            KmsResponseNotification::Keygen => KEYGEN_NOTIFICATION,
            KmsResponseNotification::Crsgen => CRSGEN_NOTIFICATION,
        }
    }

    pub fn response_str(&self) -> &'static str {
        match self {
            KmsResponseNotification::PublicDecryption => "PublicDecryptionResponse",
            KmsResponseNotification::UserDecryption => "UserDecryptionResponse",
            KmsResponseNotification::PrepKeygen => "PrepKeygenResponse",
            KmsResponseNotification::Keygen => "KeygenResponse",
            KmsResponseNotification::Crsgen => "CrsgenResponse",
        }
    }
}

/// Wrapper of `tokio::time::Interval` that can deliver `KmsResponseNotification`.
struct KmsResponseTicker {
    /// The interval at which to check for new responses.
    ticker: Interval,

    /// The `KmsResponseNotification` kind to deliver.
    kind: KmsResponseNotification,
}

impl KmsResponseTicker {
    pub fn new(polling: Duration, kind: KmsResponseNotification) -> Self {
        let mut ticker = interval(polling);

        // We don't want to spam the `DbKmsResponsePicker` with notifications if the
        // `DbKmsResponseNotifier` was blocked by the channel being full, so we skip missed tick.
        ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);

        Self { ticker, kind }
    }

    pub async fn tick(&mut self) -> Instant {
        self.ticker.tick().await
    }

    pub fn reset(&mut self) {
        self.ticker.reset();
    }

    pub fn deliver(&self) -> KmsResponseNotification {
        debug!("{} polling triggered", self.kind.response_str());
        self.kind
    }
}

// Postgres notifications for KMS Core's responses
const PUBLIC_DECRYPT_NOTIFICATION: &str = "public_decryption_response_available";
const USER_DECRYPT_NOTIFICATION: &str = "user_decryption_response_available";
const PREP_KEYGEN_NOTIFICATION: &str = "prep_keygen_response_available";
const KEYGEN_NOTIFICATION: &str = "keygen_response_available";
const CRSGEN_NOTIFICATION: &str = "crsgen_response_available";
