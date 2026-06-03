use crate::core::{
    solana_acl::{
        SolanaKmsExtraDataV0, SolanaNativeRequestError, SolanaNativeRequestLimits,
        decode_solana_kms_extra_data_v0, encode_solana_kms_extra_data_v0,
        solana_native_extra_data_hash, solana_native_request_hash, solana_native_update_ascii,
    },
    solana_flow::{
        SolanaNativeLiveRequestReleaseV0, SolanaNativeResponseRouteV0,
        SolanaNativeVerifiedResponsePublicationV0,
    },
    solana_request::{
        SolanaNativeAccountWitnessV0, SolanaNativeParsedRequestV0, SolanaNativeRequestParseError,
        parse_solana_native_request_v0,
    },
    solana_response::{
        SolanaKmsResponseCertificateV0, SolanaKmsResponsePayloadV0,
        solana_native_kms_response_hash, solana_native_response_body_hash,
    },
};
use connector_utils::{monitoring::otlp::PropagationContext, types::db::OperationStatus};
use sha3::{Digest, Keccak256};
use sqlx::{
    Pool, Postgres, Row,
    postgres::{PgListener, PgNotification, PgRow},
    types::chrono::{DateTime, Utc},
};
use std::{fmt::Display, str::FromStr, time::Duration};
use thiserror::Error;
use tokio::{
    select,
    sync::mpsc::{self, Receiver, Sender},
    time::{Instant, Interval, MissedTickBehavior, interval},
};
use tracing::{debug, error, info, warn};

pub const SOLANA_NATIVE_DECRYPTION_REQUEST_NOTIFICATION_V0: &str =
    "solana_native_decryption_request_v0_available";
pub const SOLANA_NATIVE_DECRYPTION_RESPONSE_NOTIFICATION_V0: &str =
    "solana_native_decryption_response_v0_available";
pub const SOLANA_NATIVE_RESPONSE_PAYLOAD_DB_LAYOUT_V0: u8 = 0;
pub const SOLANA_NATIVE_RESPONSE_CERTIFICATE_DB_LAYOUT_V0: u8 = 0;

#[derive(Clone, Debug)]
pub struct DbSolanaNativeDecryptionStore {
    db_pool: Pool<Postgres>,
}

pub struct DbSolanaNativeDecryptionPicker {
    store: DbSolanaNativeDecryptionStore,
    notif_receiver: Receiver<SolanaNativeDecryptionNotificationV0>,
    batch_size: u8,
}

pub struct DbSolanaNativeDecryptionNotifier {
    db_listener: PgListener,
    notif_sender: Sender<SolanaNativeDecryptionNotificationV0>,
    db_polling: Duration,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SolanaNativeDecryptionNotificationV0 {
    Request,
    Response,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SolanaNativeDecryptionWorkItemV0 {
    Requests(Vec<SolanaNativeDbRequestV0>),
    Responses(Vec<SolanaNativeDbResponseV0>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaNativeDbRequestV0 {
    pub request_hash: [u8; 32],
    pub host_chain_id: u64,
    pub solana_cluster_id: [u8; 32],
    pub kms_context_id: [u8; 32],
    pub request_mode: u8,
    pub response_context: Vec<u8>,
    pub request_bytes: Vec<u8>,
    pub already_sent: bool,
    pub error_counter: i16,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaNativeDbResponseV0 {
    pub route: SolanaNativeResponseRouteV0,
    pub response_hash: [u8; 32],
    pub response_payload: Vec<u8>,
    pub raw_response_body: Vec<u8>,
    pub certificate: Vec<u8>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Error)]
pub enum SolanaNativeDecryptionStoreError {
    #[error("native Solana request parsing failed: {0}")]
    Parse(#[from] SolanaNativeRequestParseError),
    #[error("native Solana request validation failed: {0}")]
    Request(#[from] SolanaNativeRequestError),
    #[error("native Solana response has too many certificate signatures")]
    TooManyCertificateSignatures,
    #[error("native Solana verified response publication is internally inconsistent")]
    ResponsePublicationMismatch,
    #[error("stored native Solana DB value is malformed: {0}")]
    MalformedStoredValue(&'static str),
    #[error("native Solana DB serialization failed: {0}")]
    Serialization(String),
    #[error("database error while storing native Solana decryption data: {0}")]
    Database(#[from] sqlx::Error),
}

impl DbSolanaNativeDecryptionStore {
    pub fn new(db_pool: Pool<Postgres>) -> Self {
        Self { db_pool }
    }

    pub async fn insert_pending_request_bytes_v0(
        &self,
        request_bytes: &[u8],
        limits: SolanaNativeRequestLimits,
        otlp_context: &PropagationContext,
    ) -> Result<SolanaNativeDbRequestV0, SolanaNativeDecryptionStoreError> {
        let request = SolanaNativeDbRequestV0::from_request_bytes(request_bytes, limits)?;
        self.insert_pending_request_v0(&request, otlp_context)
            .await?;
        Ok(request)
    }

    pub async fn insert_pending_request_v0(
        &self,
        request: &SolanaNativeDbRequestV0,
        otlp_context: &PropagationContext,
    ) -> Result<(), SolanaNativeDecryptionStoreError> {
        sqlx::query(
            "\
            INSERT INTO solana_native_decryption_requests_v0 (
                request_hash,
                host_chain_id,
                solana_cluster_id,
                kms_context_id,
                request_mode,
                response_context,
                request_bytes,
                otlp_context
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT DO NOTHING\
            ",
        )
        .bind(request.request_hash.as_slice())
        .bind(request.host_chain_id.to_le_bytes().as_slice())
        .bind(request.solana_cluster_id.as_slice())
        .bind(request.kms_context_id.as_slice())
        .bind(i16::from(request.request_mode))
        .bind(request.response_context.as_slice())
        .bind(request.request_bytes.as_slice())
        .bind(serialize_otlp_context(otlp_context)?)
        .execute(&self.db_pool)
        .await?;
        Ok(())
    }

    pub async fn pick_pending_requests_v0(
        &self,
        limit: u8,
    ) -> Result<Vec<SolanaNativeDbRequestV0>, SolanaNativeDecryptionStoreError> {
        sqlx::query(
            "\
            UPDATE solana_native_decryption_requests_v0
            SET status = 'under_process'
            FROM (
                SELECT request_hash
                FROM solana_native_decryption_requests_v0
                WHERE status = 'pending'
                ORDER BY updated_at ASC
                LIMIT $1 FOR UPDATE SKIP LOCKED
            ) AS req
            WHERE solana_native_decryption_requests_v0.request_hash = req.request_hash
            RETURNING
                req.request_hash,
                host_chain_id,
                solana_cluster_id,
                kms_context_id,
                request_mode,
                response_context,
                request_bytes,
                already_sent,
                error_counter,
                created_at\
            ",
        )
        .bind(i16::from(limit))
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(SolanaNativeDbRequestV0::from_row)
        .collect()
    }

    pub async fn pick_pending_responses_v0(
        &self,
        limit: u8,
    ) -> Result<Vec<SolanaNativeDbResponseV0>, SolanaNativeDecryptionStoreError> {
        sqlx::query(
            "\
            UPDATE solana_native_decryption_responses_v0
            SET status = 'under_process'
            FROM (
                SELECT request_hash
                FROM solana_native_decryption_responses_v0
                WHERE status = 'pending'
                ORDER BY updated_at ASC
                LIMIT $1 FOR UPDATE SKIP LOCKED
            ) AS resp
            WHERE solana_native_decryption_responses_v0.request_hash = resp.request_hash
            RETURNING
                resp.request_hash,
                host_chain_id,
                solana_cluster_id,
                kms_context_id,
                request_mode,
                response_kind,
                response_context,
                response_hash,
                response_payload,
                raw_response_body,
                certificate,
                created_at\
            ",
        )
        .bind(i16::from(limit))
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(SolanaNativeDbResponseV0::from_row)
        .collect()
    }

    pub async fn mark_request_status_v0(
        &self,
        request_hash: [u8; 32],
        status: OperationStatus,
        error_counter: i16,
    ) -> Result<(), SolanaNativeDecryptionStoreError> {
        sqlx::query(
            "\
            UPDATE solana_native_decryption_requests_v0
            SET status = $1, error_counter = $2
            WHERE request_hash = $3\
            ",
        )
        .bind(status)
        .bind(error_counter)
        .bind(request_hash.as_slice())
        .execute(&self.db_pool)
        .await?;
        Ok(())
    }

    pub async fn mark_response_status_v0(
        &self,
        request_hash: [u8; 32],
        status: OperationStatus,
    ) -> Result<(), SolanaNativeDecryptionStoreError> {
        sqlx::query(
            "\
            UPDATE solana_native_decryption_responses_v0
            SET status = $1
            WHERE request_hash = $2\
            ",
        )
        .bind(status)
        .bind(request_hash.as_slice())
        .execute(&self.db_pool)
        .await?;
        Ok(())
    }

    pub async fn mark_request_sent_for_release_v0(
        &self,
        release: &SolanaNativeLiveRequestReleaseV0,
    ) -> Result<(), SolanaNativeDecryptionStoreError> {
        let observed_slot = release.final_admission.observed_slot.to_le_bytes();
        let account_witnesses_hash =
            solana_native_account_witnesses_hash_v0(release.account_witnesses());
        sqlx::query(
            "\
            UPDATE solana_native_decryption_requests_v0
            SET
                already_sent = TRUE,
                observed_slot = $2,
                observed_commitment_level = $3,
                account_witnesses_hash = $4
            WHERE request_hash = $1\
            ",
        )
        .bind(
            release
                .final_admission
                .admitted
                .accepted
                .request_hash
                .as_slice(),
        )
        .bind(observed_slot.as_slice())
        .bind(i16::from(release.final_admission.observed_commitment_level))
        .bind(account_witnesses_hash.as_slice())
        .execute(&self.db_pool)
        .await?;
        Ok(())
    }

    pub async fn publish_verified_response_v0(
        &self,
        publication: &SolanaNativeVerifiedResponsePublicationV0,
        otlp_context: &PropagationContext,
    ) -> Result<SolanaNativeDbResponseV0, SolanaNativeDecryptionStoreError> {
        let response = SolanaNativeDbResponseV0::from_publication(publication)?;
        sqlx::query(
            "\
            INSERT INTO solana_native_decryption_responses_v0 (
                request_hash,
                host_chain_id,
                solana_cluster_id,
                kms_context_id,
                request_mode,
                response_kind,
                response_context,
                response_hash,
                response_payload,
                raw_response_body,
                certificate,
                otlp_context
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT DO NOTHING\
            ",
        )
        .bind(response.route.request_hash.as_slice())
        .bind(response.route.host_chain_id.to_le_bytes().as_slice())
        .bind(response.route.solana_cluster_id.as_slice())
        .bind(response.route.kms_context_id.as_slice())
        .bind(i16::from(response.route.request_mode))
        .bind(i16::from(response.route.response_kind))
        .bind(response.route.response_context.as_slice())
        .bind(response.response_hash.as_slice())
        .bind(response.response_payload.as_slice())
        .bind(response.raw_response_body.as_slice())
        .bind(response.certificate.as_slice())
        .bind(serialize_otlp_context(otlp_context)?)
        .execute(&self.db_pool)
        .await?;
        Ok(response)
    }
}

impl DbSolanaNativeDecryptionPicker {
    pub fn new(
        store: DbSolanaNativeDecryptionStore,
        notif_receiver: Receiver<SolanaNativeDecryptionNotificationV0>,
        batch_size: u8,
    ) -> Self {
        Self {
            store,
            notif_receiver,
            batch_size,
        }
    }

    pub async fn connect(
        db_pool: Pool<Postgres>,
        batch_size: u8,
        polling: Duration,
        task_limit: usize,
    ) -> anyhow::Result<Self> {
        let (notif_sender, notif_receiver) = mpsc::channel(task_limit);
        let notifier =
            DbSolanaNativeDecryptionNotifier::connect(db_pool.clone(), notif_sender, polling)
                .await?;
        tokio::spawn(notifier.start());

        Ok(Self::new(
            DbSolanaNativeDecryptionStore::new(db_pool),
            notif_receiver,
            batch_size,
        ))
    }

    pub async fn pick_work_items(&mut self) -> anyhow::Result<SolanaNativeDecryptionWorkItemV0> {
        loop {
            let Some(notification) = self.notif_receiver.recv().await else {
                anyhow::bail!("native Solana decryption notification channel was closed");
            };

            match self.pick_notified_work_items(notification).await {
                Err(e) => {
                    warn!("Error while picking native Solana decryption work items: {e}");
                    continue;
                }
                Ok(Some(work_items)) => return Ok(work_items),
                Ok(None) => {
                    debug!("Native Solana decryption work items have already been picked");
                    continue;
                }
            }
        }
    }

    async fn pick_notified_work_items(
        &self,
        notification: SolanaNativeDecryptionNotificationV0,
    ) -> Result<Option<SolanaNativeDecryptionWorkItemV0>, SolanaNativeDecryptionStoreError> {
        match notification {
            SolanaNativeDecryptionNotificationV0::Request => {
                let requests = self.store.pick_pending_requests_v0(self.batch_size).await?;
                if requests.is_empty() {
                    Ok(None)
                } else {
                    info!("Picked {} native Solana requests", requests.len());
                    Ok(Some(SolanaNativeDecryptionWorkItemV0::Requests(requests)))
                }
            }
            SolanaNativeDecryptionNotificationV0::Response => {
                let responses = self
                    .store
                    .pick_pending_responses_v0(self.batch_size)
                    .await?;
                if responses.is_empty() {
                    Ok(None)
                } else {
                    info!("Picked {} native Solana responses", responses.len());
                    Ok(Some(SolanaNativeDecryptionWorkItemV0::Responses(responses)))
                }
            }
        }
    }
}

impl DbSolanaNativeDecryptionNotifier {
    pub fn new(
        db_listener: PgListener,
        notif_sender: Sender<SolanaNativeDecryptionNotificationV0>,
        db_polling: Duration,
    ) -> Self {
        Self {
            db_listener,
            notif_sender,
            db_polling,
        }
    }

    pub async fn connect(
        db_pool: Pool<Postgres>,
        notif_sender: Sender<SolanaNativeDecryptionNotificationV0>,
        db_polling: Duration,
    ) -> anyhow::Result<Self> {
        let db_listener = PgListener::connect_with(&db_pool)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to init native Solana Postgres listener: {e}"))?;
        let mut notifier = Self::new(db_listener, notif_sender, db_polling);
        notifier.listen().await.map_err(|e| {
            anyhow::anyhow!("Failed to listen to native Solana decryption notifications: {e}")
        })?;
        Ok(notifier)
    }

    async fn listen(&mut self) -> sqlx::Result<()> {
        self.db_listener
            .listen(SOLANA_NATIVE_DECRYPTION_REQUEST_NOTIFICATION_V0)
            .await?;
        self.db_listener
            .listen(SOLANA_NATIVE_DECRYPTION_RESPONSE_NOTIFICATION_V0)
            .await?;
        Ok(())
    }

    pub async fn start(mut self) {
        let mut request_ticker = SolanaNativeDecryptionTicker::new(
            self.db_polling,
            SolanaNativeDecryptionNotificationV0::Request,
        );
        let mut response_ticker = SolanaNativeDecryptionTicker::new(
            self.db_polling,
            SolanaNativeDecryptionNotificationV0::Response,
        );

        loop {
            let notification = select! {
                _ = request_ticker.tick() => request_ticker.deliver(),
                _ = response_ticker.tick() => response_ticker.deliver(),
                result = self.db_listener.recv() => match result.map(SolanaNativeDecryptionNotificationV0::try_from) {
                    Err(e) => {
                        warn!("Error while listening for native Solana Postgres notification: {e}");
                        continue;
                    }
                    Ok(Err(e)) => {
                        warn!("Native Solana notification parsing error: {e}");
                        continue;
                    }
                    Ok(Ok(notification)) => {
                        info!(
                            "Received native Solana Postgres notification: {}",
                            notification.pg_notification()
                        );
                        match notification {
                            SolanaNativeDecryptionNotificationV0::Request => request_ticker.reset(),
                            SolanaNativeDecryptionNotificationV0::Response => response_ticker.reset(),
                        }
                        notification
                    }
                },
            };

            if self.notif_sender.send(notification).await.is_err() {
                break error!("Native Solana decryption notification channel was closed");
            }
        }
    }
}

struct SolanaNativeDecryptionTicker {
    ticker: Interval,
    kind: SolanaNativeDecryptionNotificationV0,
}

impl SolanaNativeDecryptionTicker {
    fn new(polling: Duration, kind: SolanaNativeDecryptionNotificationV0) -> Self {
        let mut ticker = interval(polling);
        ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);
        Self { ticker, kind }
    }

    async fn tick(&mut self) -> Instant {
        self.ticker.tick().await
    }

    fn reset(&mut self) {
        self.ticker.reset();
    }

    fn deliver(&self) -> SolanaNativeDecryptionNotificationV0 {
        debug!("{} polling triggered", self.kind);
        self.kind
    }
}

impl SolanaNativeDecryptionNotificationV0 {
    pub fn pg_notification(&self) -> &'static str {
        match self {
            Self::Request => SOLANA_NATIVE_DECRYPTION_REQUEST_NOTIFICATION_V0,
            Self::Response => SOLANA_NATIVE_DECRYPTION_RESPONSE_NOTIFICATION_V0,
        }
    }
}

impl TryFrom<PgNotification> for SolanaNativeDecryptionNotificationV0 {
    type Error = anyhow::Error;

    fn try_from(value: PgNotification) -> Result<Self, Self::Error> {
        value.channel().parse()
    }
}

impl FromStr for SolanaNativeDecryptionNotificationV0 {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            SOLANA_NATIVE_DECRYPTION_REQUEST_NOTIFICATION_V0 => Ok(Self::Request),
            SOLANA_NATIVE_DECRYPTION_RESPONSE_NOTIFICATION_V0 => Ok(Self::Response),
            other => Err(anyhow::anyhow!(
                "unknown native Solana decryption notification channel: {other}"
            )),
        }
    }
}

impl Display for SolanaNativeDecryptionNotificationV0 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Request => formatter.write_str("SolanaNativeDecryptionRequestV0"),
            Self::Response => formatter.write_str("SolanaNativeDecryptionResponseV0"),
        }
    }
}

impl SolanaNativeDbRequestV0 {
    pub fn from_request_bytes(
        request_bytes: &[u8],
        limits: SolanaNativeRequestLimits,
    ) -> Result<Self, SolanaNativeDecryptionStoreError> {
        let parsed_request = parse_solana_native_request_v0(request_bytes, limits)?;
        Self::from_parsed_request(&parsed_request, request_bytes.to_vec(), limits)
    }

    pub fn from_parsed_request(
        parsed_request: &SolanaNativeParsedRequestV0,
        request_bytes: Vec<u8>,
        limits: SolanaNativeRequestLimits,
    ) -> Result<Self, SolanaNativeDecryptionStoreError> {
        let extra_data = decode_solana_kms_extra_data_v0(&parsed_request.raw_extra_data, limits)?;
        Ok(Self {
            request_hash: solana_native_request_hash(&parsed_request.payload),
            host_chain_id: parsed_request.payload.host_chain_id,
            solana_cluster_id: parsed_request.payload.solana_cluster_id,
            kms_context_id: parsed_request.payload.kms_context_id,
            request_mode: parsed_request.payload.request_mode,
            response_context: extra_data.response_context,
            request_bytes,
            already_sent: false,
            error_counter: 0,
            created_at: None,
        })
    }

    fn from_row(row: &PgRow) -> Result<Self, SolanaNativeDecryptionStoreError> {
        Ok(Self {
            request_hash: bytes32(row.try_get::<Vec<u8>, _>("request_hash")?, "request_hash")?,
            host_chain_id: u64::from_le_bytes(bytes8(
                row.try_get::<Vec<u8>, _>("host_chain_id")?,
                "host_chain_id",
            )?),
            solana_cluster_id: bytes32(
                row.try_get::<Vec<u8>, _>("solana_cluster_id")?,
                "solana_cluster_id",
            )?,
            kms_context_id: bytes32(
                row.try_get::<Vec<u8>, _>("kms_context_id")?,
                "kms_context_id",
            )?,
            request_mode: u8_from_i16(row.try_get::<i16, _>("request_mode")?, "request_mode")?,
            response_context: row.try_get("response_context")?,
            request_bytes: row.try_get("request_bytes")?,
            already_sent: row.try_get("already_sent")?,
            error_counter: row.try_get("error_counter")?,
            created_at: Some(row.try_get("created_at")?),
        })
    }
}

impl SolanaNativeDbResponseV0 {
    pub fn from_publication(
        publication: &SolanaNativeVerifiedResponsePublicationV0,
    ) -> Result<Self, SolanaNativeDecryptionStoreError> {
        verify_response_publication_for_storage_v0(publication)?;
        Ok(Self {
            route: publication.route.clone(),
            response_hash: publication.verified.response_hash,
            response_payload: encode_solana_response_payload_for_db_v0(
                &publication.response_payload,
            ),
            raw_response_body: publication.raw_response_body.clone(),
            certificate: encode_solana_response_certificate_for_db_v0(&publication.certificate)?,
            created_at: None,
        })
    }

    fn from_row(row: &PgRow) -> Result<Self, SolanaNativeDecryptionStoreError> {
        Ok(Self {
            route: SolanaNativeResponseRouteV0 {
                host_chain_id: u64::from_le_bytes(bytes8(
                    row.try_get::<Vec<u8>, _>("host_chain_id")?,
                    "host_chain_id",
                )?),
                solana_cluster_id: bytes32(
                    row.try_get::<Vec<u8>, _>("solana_cluster_id")?,
                    "solana_cluster_id",
                )?,
                kms_context_id: bytes32(
                    row.try_get::<Vec<u8>, _>("kms_context_id")?,
                    "kms_context_id",
                )?,
                request_hash: bytes32(row.try_get::<Vec<u8>, _>("request_hash")?, "request_hash")?,
                request_mode: u8_from_i16(row.try_get::<i16, _>("request_mode")?, "request_mode")?,
                response_kind: u8_from_i16(
                    row.try_get::<i16, _>("response_kind")?,
                    "response_kind",
                )?,
                response_context: row.try_get("response_context")?,
            },
            response_hash: bytes32(row.try_get::<Vec<u8>, _>("response_hash")?, "response_hash")?,
            response_payload: row.try_get("response_payload")?,
            raw_response_body: row.try_get("raw_response_body")?,
            certificate: row.try_get("certificate")?,
            created_at: Some(row.try_get("created_at")?),
        })
    }
}

fn verify_response_publication_for_storage_v0(
    publication: &SolanaNativeVerifiedResponsePublicationV0,
) -> Result<(), SolanaNativeDecryptionStoreError> {
    let payload = &publication.response_payload;
    let route = &publication.route;
    if publication.verified.response_hash != solana_native_kms_response_hash(payload)
        || publication.raw_response_body.len() > u32::MAX as usize
        || payload.response_body_len as usize != publication.raw_response_body.len()
        || payload.response_body_hash
            != solana_native_response_body_hash(&publication.raw_response_body)
        || payload.extra_data_hash
            != solana_native_extra_data_hash(&encode_solana_kms_extra_data_v0(
                &SolanaKmsExtraDataV0 {
                    kms_context_id: route.kms_context_id,
                    response_context: route.response_context.clone(),
                },
            ))
        || publication.certificate.kms_context_id != payload.kms_context_id
        || route.host_chain_id != payload.host_chain_id
        || route.solana_cluster_id != payload.solana_cluster_id
        || route.kms_context_id != payload.kms_context_id
        || route.request_hash != payload.request_hash
        || route.request_mode != payload.request_mode
        || route.response_kind != payload.response_kind
    {
        return Err(SolanaNativeDecryptionStoreError::ResponsePublicationMismatch);
    }
    Ok(())
}

pub fn solana_native_account_witnesses_hash_v0(
    account_witnesses: &[SolanaNativeAccountWitnessV0],
) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    solana_native_update_ascii(&mut hasher, "zama-solana-account-witnesses-v0");
    hasher.update((account_witnesses.len() as u32).to_le_bytes());
    for witness in account_witnesses {
        hasher.update(witness.account_key);
        hasher.update(witness.owner);
        hasher.update([u8::from(witness.executable)]);
        hasher.update((witness.data.len() as u32).to_le_bytes());
        hasher.update(&witness.data);
    }
    hasher.finalize().into()
}

pub fn encode_solana_response_payload_for_db_v0(payload: &SolanaKmsResponsePayloadV0) -> Vec<u8> {
    let mut output = Vec::with_capacity(311);
    output.push(SOLANA_NATIVE_RESPONSE_PAYLOAD_DB_LAYOUT_V0);
    output.extend_from_slice(&payload.domain_separator);
    output.extend_from_slice(&payload.host_chain_id.to_le_bytes());
    output.extend_from_slice(&payload.config_version.to_le_bytes());
    output.extend_from_slice(&payload.solana_cluster_id);
    output.extend_from_slice(&payload.kms_context_id);
    output.extend_from_slice(&payload.request_hash);
    output.push(payload.request_mode);
    output.push(payload.response_kind);
    output.extend_from_slice(&payload.nonce);
    output.extend_from_slice(&payload.entries_hash);
    output.extend_from_slice(&payload.extra_data_hash);
    output.extend_from_slice(&payload.user_reencryption_pubkey_hash);
    output.extend_from_slice(&payload.response_body_len.to_le_bytes());
    output.extend_from_slice(&payload.response_body_hash);
    output
}

pub fn encode_solana_response_certificate_for_db_v0(
    certificate: &SolanaKmsResponseCertificateV0,
) -> Result<Vec<u8>, SolanaNativeDecryptionStoreError> {
    let signature_count = u16::try_from(certificate.signatures.len())
        .map_err(|_| SolanaNativeDecryptionStoreError::TooManyCertificateSignatures)?;
    let mut output = Vec::with_capacity(69 + usize::from(signature_count) * 96);
    output.push(SOLANA_NATIVE_RESPONSE_CERTIFICATE_DB_LAYOUT_V0);
    output.extend_from_slice(&certificate.kms_context_id);
    output.extend_from_slice(&certificate.signer_set_hash);
    output.extend_from_slice(&certificate.threshold.to_le_bytes());
    output.extend_from_slice(&signature_count.to_le_bytes());
    for signature in &certificate.signatures {
        output.extend_from_slice(&signature.signer_pubkey);
        output.extend_from_slice(&signature.signature);
    }
    Ok(output)
}

fn serialize_otlp_context(
    otlp_context: &PropagationContext,
) -> Result<Vec<u8>, SolanaNativeDecryptionStoreError> {
    bc2wrap::serialize(otlp_context)
        .map_err(|e| SolanaNativeDecryptionStoreError::Serialization(e.to_string()))
}

fn bytes32(
    value: Vec<u8>,
    field: &'static str,
) -> Result<[u8; 32], SolanaNativeDecryptionStoreError> {
    value
        .try_into()
        .map_err(|_| SolanaNativeDecryptionStoreError::MalformedStoredValue(field))
}

fn bytes8(
    value: Vec<u8>,
    field: &'static str,
) -> Result<[u8; 8], SolanaNativeDecryptionStoreError> {
    value
        .try_into()
        .map_err(|_| SolanaNativeDecryptionStoreError::MalformedStoredValue(field))
}

fn u8_from_i16(value: i16, field: &'static str) -> Result<u8, SolanaNativeDecryptionStoreError> {
    u8::try_from(value).map_err(|_| SolanaNativeDecryptionStoreError::MalformedStoredValue(field))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        solana_acl::{
            SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED, SOLANA_NATIVE_SUPPORTED_MATERIAL_SOURCE_MODE,
            SolanaKmsExtraDataV0, SolanaUserDecryptionPayloadV0, encode_solana_kms_extra_data_v0,
            solana_native_domain_separator, solana_native_extra_data_hash,
            solana_native_reencryption_pubkey_hash,
        },
        solana_response::{
            KmsResponseSignatureV0, SOLANA_NATIVE_RESPONSE_KIND_DIRECT_SCOPED,
            SolanaKmsVerifiedResponseV0,
        },
    };

    fn payload_fixture(raw_extra_data: &[u8]) -> SolanaUserDecryptionPayloadV0 {
        let reencryption_key = b"reencryption-key".to_vec();
        SolanaUserDecryptionPayloadV0 {
            domain_separator: solana_native_domain_separator(900, [9; 32], [42; 32], [8; 32]),
            host_chain_id: 900,
            config_version: 3,
            solana_cluster_id: [9; 32],
            kms_context_id: [8; 32],
            user_reencryption_pubkey_len: reencryption_key.len() as u32,
            user_reencryption_pubkey_hash: solana_native_reencryption_pubkey_hash(
                &reencryption_key,
            ),
            request_signer_pubkey: [7; 32],
            acl_program_id: [42; 32],
            request_mode: SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED,
            material_source_mode: SOLANA_NATIVE_SUPPORTED_MATERIAL_SOURCE_MODE,
            commitment_level: 1,
            min_context_slot: 500,
            expiration_slot: 520,
            nonce: [77; 32],
            extra_data_hash: solana_native_extra_data_hash(raw_extra_data),
            entries_hash: [99; 32],
        }
    }

    fn response_publication_fixture() -> SolanaNativeVerifiedResponsePublicationV0 {
        let raw_response_body = b"response-body".to_vec();
        let response_context = b"route".to_vec();
        let extra_data = encode_solana_kms_extra_data_v0(&SolanaKmsExtraDataV0 {
            kms_context_id: [8; 32],
            response_context: response_context.clone(),
        });
        let response_payload = SolanaKmsResponsePayloadV0 {
            domain_separator: [1; 32],
            host_chain_id: 900,
            config_version: 3,
            solana_cluster_id: [9; 32],
            kms_context_id: [8; 32],
            request_hash: [55; 32],
            request_mode: SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED,
            response_kind: SOLANA_NATIVE_RESPONSE_KIND_DIRECT_SCOPED,
            nonce: [77; 32],
            entries_hash: [99; 32],
            extra_data_hash: solana_native_extra_data_hash(&extra_data),
            user_reencryption_pubkey_hash: [11; 32],
            response_body_len: raw_response_body.len() as u32,
            response_body_hash: solana_native_response_body_hash(&raw_response_body),
        };
        let response_hash = solana_native_kms_response_hash(&response_payload);
        SolanaNativeVerifiedResponsePublicationV0 {
            route: SolanaNativeResponseRouteV0 {
                host_chain_id: response_payload.host_chain_id,
                solana_cluster_id: response_payload.solana_cluster_id,
                kms_context_id: response_payload.kms_context_id,
                request_hash: response_payload.request_hash,
                request_mode: response_payload.request_mode,
                response_kind: response_payload.response_kind,
                response_context,
            },
            verified: SolanaKmsVerifiedResponseV0 { response_hash },
            response_payload,
            raw_response_body,
            certificate: SolanaKmsResponseCertificateV0 {
                kms_context_id: [8; 32],
                signer_set_hash: [9; 32],
                threshold: 1,
                signatures: vec![KmsResponseSignatureV0 {
                    signer_pubkey: [1; 32],
                    signature: [2; 64],
                }],
            },
        }
    }

    #[test]
    fn db_request_route_helper_derives_fields_from_parsed_request() {
        let raw_extra_data = encode_solana_kms_extra_data_v0(&SolanaKmsExtraDataV0 {
            kms_context_id: [8; 32],
            response_context: b"client-route".to_vec(),
        });
        let parsed_request = SolanaNativeParsedRequestV0 {
            payload: payload_fixture(&raw_extra_data),
            entries: Vec::new(),
            raw_extra_data,
            user_reencryption_public_key: Vec::new(),
            request_signature: Vec::new(),
        };

        let item = SolanaNativeDbRequestV0::from_parsed_request(
            &parsed_request,
            b"request".to_vec(),
            SolanaNativeRequestLimits::default(),
        )
        .unwrap();

        assert_eq!(
            item.request_hash,
            solana_native_request_hash(&parsed_request.payload)
        );
        assert_eq!(item.host_chain_id, 900);
        assert_eq!(item.solana_cluster_id, [9; 32]);
        assert_eq!(item.kms_context_id, [8; 32]);
        assert_eq!(item.request_mode, SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED);
        assert_eq!(item.response_context, b"client-route".to_vec());
        assert_eq!(item.request_bytes, b"request".to_vec());
        assert!(!item.already_sent);
    }

    #[test]
    fn db_response_from_publication_binds_verified_material() {
        let publication = response_publication_fixture();
        let item = SolanaNativeDbResponseV0::from_publication(&publication).unwrap();

        assert_eq!(item.route, publication.route);
        assert_eq!(item.response_hash, publication.verified.response_hash);
        assert_eq!(
            item.response_payload,
            encode_solana_response_payload_for_db_v0(&publication.response_payload)
        );
        assert_eq!(item.raw_response_body, publication.raw_response_body);
        assert_eq!(
            item.certificate,
            encode_solana_response_certificate_for_db_v0(&publication.certificate).unwrap()
        );
    }

    #[test]
    fn db_response_from_publication_rejects_inconsistent_material() {
        let mut bad_hash = response_publication_fixture();
        bad_hash.verified.response_hash = [44; 32];
        assert!(matches!(
            SolanaNativeDbResponseV0::from_publication(&bad_hash),
            Err(SolanaNativeDecryptionStoreError::ResponsePublicationMismatch)
        ));

        let mut bad_body = response_publication_fixture();
        bad_body.raw_response_body.push(0);
        assert!(matches!(
            SolanaNativeDbResponseV0::from_publication(&bad_body),
            Err(SolanaNativeDecryptionStoreError::ResponsePublicationMismatch)
        ));

        let mut bad_route = response_publication_fixture();
        bad_route.route.request_hash = [45; 32];
        assert!(matches!(
            SolanaNativeDbResponseV0::from_publication(&bad_route),
            Err(SolanaNativeDecryptionStoreError::ResponsePublicationMismatch)
        ));

        let mut bad_route_context = response_publication_fixture();
        bad_route_context.route.response_context = b"wrong-route".to_vec();
        assert!(matches!(
            SolanaNativeDbResponseV0::from_publication(&bad_route_context),
            Err(SolanaNativeDecryptionStoreError::ResponsePublicationMismatch)
        ));

        let mut bad_certificate = response_publication_fixture();
        bad_certificate.certificate.kms_context_id = [46; 32];
        assert!(matches!(
            SolanaNativeDbResponseV0::from_publication(&bad_certificate),
            Err(SolanaNativeDecryptionStoreError::ResponsePublicationMismatch)
        ));
    }

    #[test]
    fn account_witness_hash_binds_order_keys_owner_data_and_executable() {
        let witnesses = vec![
            SolanaNativeAccountWitnessV0 {
                account_key: [1; 32],
                owner: [2; 32],
                executable: false,
                data: vec![3, 4],
            },
            SolanaNativeAccountWitnessV0 {
                account_key: [5; 32],
                owner: [6; 32],
                executable: false,
                data: vec![7],
            },
        ];
        let hash = solana_native_account_witnesses_hash_v0(&witnesses);
        let mut changed = witnesses.clone();
        changed.reverse();

        assert_ne!(hash, solana_native_account_witnesses_hash_v0(&changed));
        changed.reverse();
        changed[0].account_key = [8; 32];
        assert_ne!(hash, solana_native_account_witnesses_hash_v0(&changed));
        changed[0].account_key = witnesses[0].account_key;
        changed[0].owner = [9; 32];
        assert_ne!(hash, solana_native_account_witnesses_hash_v0(&changed));
        changed[0].owner = witnesses[0].owner;
        changed[0].executable = true;
        assert_ne!(hash, solana_native_account_witnesses_hash_v0(&changed));
        changed[0].executable = false;
        changed[0].data.push(9);
        assert_ne!(hash, solana_native_account_witnesses_hash_v0(&changed));
    }

    #[test]
    fn response_payload_db_encoding_is_fixed_width() {
        let body = b"body".to_vec();
        let payload = SolanaKmsResponsePayloadV0 {
            domain_separator: [1; 32],
            host_chain_id: 900,
            config_version: 3,
            solana_cluster_id: [9; 32],
            kms_context_id: [8; 32],
            request_hash: [55; 32],
            request_mode: SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED,
            response_kind: SOLANA_NATIVE_RESPONSE_KIND_DIRECT_SCOPED,
            nonce: [77; 32],
            entries_hash: [99; 32],
            extra_data_hash: [10; 32],
            user_reencryption_pubkey_hash: [11; 32],
            response_body_len: body.len() as u32,
            response_body_hash: solana_native_response_body_hash(&body),
        };
        let encoded = encode_solana_response_payload_for_db_v0(&payload);

        assert_eq!(encoded.len(), 311);
        assert_eq!(encoded[0], SOLANA_NATIVE_RESPONSE_PAYLOAD_DB_LAYOUT_V0);
        assert_eq!(&encoded[1..33], &[1; 32]);
        assert_eq!(&encoded[33..41], &900_u64.to_le_bytes());
        assert_eq!(&encoded[41..49], &3_u64.to_le_bytes());
        assert_eq!(&encoded[49..81], &[9; 32]);
        assert_eq!(&encoded[81..113], &[8; 32]);
        assert_eq!(&encoded[113..145], &[55; 32]);
        assert_eq!(encoded[145], SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED);
        assert_eq!(encoded[146], SOLANA_NATIVE_RESPONSE_KIND_DIRECT_SCOPED);
        assert_eq!(&encoded[147..179], &[77; 32]);
        assert_eq!(&encoded[179..211], &[99; 32]);
        assert_eq!(&encoded[211..243], &[10; 32]);
        assert_eq!(&encoded[243..275], &[11; 32]);
        assert_eq!(&encoded[275..279], &(body.len() as u32).to_le_bytes());
        assert_eq!(&encoded[279..311], &solana_native_response_body_hash(&body));
    }

    #[test]
    fn response_certificate_db_encoding_binds_all_signatures() {
        let certificate = SolanaKmsResponseCertificateV0 {
            kms_context_id: [8; 32],
            signer_set_hash: [9; 32],
            threshold: 2,
            signatures: vec![
                KmsResponseSignatureV0 {
                    signer_pubkey: [1; 32],
                    signature: [2; 64],
                },
                KmsResponseSignatureV0 {
                    signer_pubkey: [3; 32],
                    signature: [4; 64],
                },
            ],
        };
        let encoded = encode_solana_response_certificate_for_db_v0(&certificate).unwrap();

        assert_eq!(encoded.len(), 69 + 2 * 96);
        assert_eq!(encoded[0], SOLANA_NATIVE_RESPONSE_CERTIFICATE_DB_LAYOUT_V0);
        assert_eq!(&encoded[1..33], &[8; 32]);
        assert_eq!(&encoded[33..65], &[9; 32]);
        assert_eq!(&encoded[65..67], &2_u16.to_le_bytes());
        assert_eq!(&encoded[67..69], &2_u16.to_le_bytes());
        assert_eq!(&encoded[69..101], &[1; 32]);
        assert_eq!(&encoded[101..165], &[2; 64]);
        assert_eq!(&encoded[165..197], &[3; 32]);
        assert_eq!(&encoded[197..261], &[4; 64]);
    }

    #[test]
    fn response_migration_constrains_payload_and_certificate_layouts() {
        let migration = include_str!(
            "../../../../connector-db/migrations/20260526100000_solana_native_decryption_flow.sql"
        );
        let (request_table, response_and_after) = migration
            .split_once("CREATE TABLE IF NOT EXISTS solana_native_decryption_responses_v0")
            .expect("migration should contain the native response table");
        let (response_table, _) = response_and_after
            .split_once("CREATE INDEX IF NOT EXISTS idx_solana_native_decryption_responses_v0_status_updated_at")
            .expect("migration should index the native response table");

        assert!(!request_table.contains(
            "FOREIGN KEY (request_hash)\n        REFERENCES solana_native_decryption_requests_v0(request_hash)"
        ));
        assert!(
            response_table.contains(
                "FOREIGN KEY (request_hash)\n        REFERENCES solana_native_decryption_requests_v0(request_hash)"
            )
        );
        assert!(migration.contains("CHECK (octet_length(response_payload) = 311)"));
        assert!(
            migration
                .contains("CHECK (substring(response_payload FROM 1 FOR 1) = decode('00', 'hex'))")
        );
        assert!(migration.contains("CHECK (octet_length(certificate) >= 69)"));
        assert!(migration.contains("CHECK ((octet_length(certificate) - 69) % 96 = 0)"));
        assert!(
            migration.contains(
                "69 + 96 * (get_byte(certificate, 67) + get_byte(certificate, 68) * 256)"
            )
        );
        assert!(
            migration.contains("CHECK (substring(certificate FROM 1 FOR 1) = decode('00', 'hex'))")
        );
    }

    #[test]
    fn native_notification_round_trips_channel_names() {
        assert_eq!(
            SOLANA_NATIVE_DECRYPTION_REQUEST_NOTIFICATION_V0
                .parse::<SolanaNativeDecryptionNotificationV0>()
                .unwrap(),
            SolanaNativeDecryptionNotificationV0::Request
        );
        assert_eq!(
            SOLANA_NATIVE_DECRYPTION_RESPONSE_NOTIFICATION_V0
                .parse::<SolanaNativeDecryptionNotificationV0>()
                .unwrap(),
            SolanaNativeDecryptionNotificationV0::Response
        );
        assert_eq!(
            SolanaNativeDecryptionNotificationV0::Request.pg_notification(),
            SOLANA_NATIVE_DECRYPTION_REQUEST_NOTIFICATION_V0
        );
        assert_eq!(
            SolanaNativeDecryptionNotificationV0::Response.to_string(),
            "SolanaNativeDecryptionResponseV0"
        );
        assert!(
            "unknown"
                .parse::<SolanaNativeDecryptionNotificationV0>()
                .is_err()
        );
    }
}
