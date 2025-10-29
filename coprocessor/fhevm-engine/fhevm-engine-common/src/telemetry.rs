use crate::utils::compact_hex;
use bigdecimal::num_traits::ToPrimitive;
use opentelemetry::{
    global::{BoxedSpan, BoxedTracer, ObjectSafeSpan},
    trace::{SpanBuilder, Status, TraceContextExt, Tracer},
    Context, KeyValue,
};
use opentelemetry_sdk::{trace::SdkTracerProvider, Resource};
use prometheus::{register_histogram, Histogram};
use sqlx::PgConnection;
use std::fmt;
use std::{
    num::NonZeroUsize,
    str::FromStr,
    sync::{Arc, LazyLock, OnceLock},
    time::SystemTime,
};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

pub const TXN_ID_ATTR_KEY: &str = "txn_id";

pub static HOST_TXN_LATENCY_CONFIG: OnceLock<MetricsConfig> = OnceLock::new();
pub(crate) static HOST_TXN_LATENCY_HISTOGRAM: LazyLock<Histogram> = LazyLock::new(|| {
    register_histogram(
        HOST_TXN_LATENCY_CONFIG.get(),
        "coprocessor_host_txn_latency_seconds",
        "Host transaction latencies in seconds",
    )
});

pub static ZKPROOF_TXN_LATENCY_CONFIG: OnceLock<MetricsConfig> = OnceLock::new();
pub(crate) static ZKPROOF_TXN_LATENCY_HISTOGRAM: LazyLock<Histogram> = LazyLock::new(|| {
    register_histogram(
        ZKPROOF_TXN_LATENCY_CONFIG.get(),
        "coprocessor_zkproof_txn_latency_seconds",
        "ZKProof transaction latencies in seconds",
    )
});

pub fn setup_otlp(
    service_name: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let otlp_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .build()?;

    let resource = Resource::builder_empty()
        .with_attributes(vec![KeyValue::new(
            opentelemetry_semantic_conventions::resource::SERVICE_NAME.to_string(),
            service_name.to_string(),
        )])
        .build();

    let trace_provider = SdkTracerProvider::builder()
        .with_resource(resource)
        .with_batch_exporter(otlp_exporter)
        .build();

    opentelemetry::global::set_tracer_provider(trace_provider);

    Ok(())
}

#[derive(Clone)]
pub struct OtelTracer {
    ctx: opentelemetry::Context,
    tracer: Arc<BoxedTracer>,
}

impl OtelTracer {
    pub fn child_span(&self, name: &'static str) -> BoxedSpan {
        self.tracer.start_with_context(name, &self.ctx)
    }

    /// Sets attribute to the root span
    pub fn set_attribute(&self, key: &str, value: String) {
        self.ctx
            .span()
            .set_attribute(KeyValue::new(key.to_owned(), value));
    }

    /// Consumes and ends the tracer with status Ok
    pub fn end(self) {
        self.ctx.span().set_status(Status::Ok);
        self.ctx.span().end();
    }
}

#[derive(Debug, PartialEq)]
struct Handle(Vec<u8>);
#[derive(Debug, PartialEq)]
struct Transaction(Vec<u8>);

pub fn tracer_with_handle(
    span_name: &'static str,
    handle: Vec<u8>,
    transaction_id: &Option<Vec<u8>>,
) -> OtelTracer {
    let tracer = opentelemetry::global::tracer(format!("tracer_{}", span_name));
    let mut span = tracer.start(span_name);

    if !handle.is_empty() {
        let handle = compact_hex(&handle)
            .get(0..10)
            .unwrap_or_default()
            .to_owned();

        span.set_attribute(KeyValue::new("handle", handle));
    }

    if let Some(transaction_id) = transaction_id {
        set_txn_id(&mut span, transaction_id);
    }

    // Add handle and transaction_id to the context
    // so that they can be retrieved in the application code, e.g. for logging
    let mut ctx = Context::default().with_span(span);
    ctx = ctx.with_value(Handle(handle.clone()));
    ctx = ctx.with_value(Transaction(transaction_id.clone().unwrap_or_default()));

    OtelTracer {
        ctx,
        tracer: Arc::new(tracer),
    }
}

// Sets the txn_id attribute to the span
// The txn_id is a shortened version of the transaction_id (first 10 characters of the hex representation)
pub fn set_txn_id(span: &mut BoxedSpan, transaction_id: &[u8]) {
    let txn_id_short = compact_hex(transaction_id)
        .get(0..10)
        .unwrap_or_default()
        .to_owned();

    span.set_attribute(KeyValue::new(TXN_ID_ATTR_KEY, txn_id_short));
}

/// Create a new span with start and end time
pub fn tracer_with_start_time(span_name: &'static str, start_time: SystemTime) -> OtelTracer {
    let tracer = opentelemetry::global::tracer(span_name);
    let root_span = tracer.build(SpanBuilder::from_name(span_name).with_start_time(start_time));
    let ctx = opentelemetry::Context::default().with_span(root_span);
    OtelTracer {
        ctx,
        tracer: Arc::new(tracer),
    }
}

pub fn tracer(span_name: &'static str, transaction_id: &Option<Vec<u8>>) -> OtelTracer {
    tracer_with_handle(span_name, vec![], transaction_id)
}

pub fn attribute(span: &mut BoxedSpan, key: &str, value: String) {
    span.set_attribute(KeyValue::new(key.to_owned(), value));
}

/// Ends span with status Ok
pub fn end_span(mut span: BoxedSpan) {
    span.set_status(Status::Ok);
    span.end();
}

pub fn end_span_with_timestamp(mut span: BoxedSpan, timestamp: SystemTime) {
    span.set_status(Status::Ok);
    span.end_with_timestamp(timestamp);
}

/// Ends span with status Error with description
pub fn end_span_with_err(mut span: BoxedSpan, desc: String) {
    span.set_status(Status::Error {
        description: desc.into(),
    });
    span.end();
}

#[derive(Clone, Copy, Debug)]
pub struct MetricsConfig {
    bucket_start: f64,
    bucket_end: f64,
    bucket_step: f64,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        MetricsConfig {
            bucket_start: 0.01,
            bucket_end: 10.0,
            bucket_step: 0.01,
        }
    }
}

impl FromStr for MetricsConfig {
    type Err = String;
    /// Expected format: "start:end:step", e.g. "0.0:10.0:0.5"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 3 {
            return Err("Expected format: <start>:<end>:<step>".to_string());
        }

        let bucket_start = parts[0]
            .parse::<f64>()
            .map_err(|_| "Invalid start value".to_string())?;
        let bucket_end = parts[1]
            .parse::<f64>()
            .map_err(|_| "Invalid end value".to_string())?;
        let bucket_step = parts[2]
            .parse::<f64>()
            .map_err(|_| "Invalid step value".to_string())?;

        Ok(Self {
            bucket_start,
            bucket_end,
            bucket_step,
        })
    }
}

impl fmt::Display for MetricsConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            self.bucket_start, self.bucket_end, self.bucket_step
        )
    }
}

pub fn gen_linear_buckets(conf: &MetricsConfig) -> Vec<f64> {
    let mut buckets = vec![];
    let mut current = conf.bucket_start;
    while current <= conf.bucket_end {
        buckets.push(current);
        current += conf.bucket_step;
    }
    buckets
}

/// Registers histogram to global prometheus registry
pub fn register_histogram(config: Option<&MetricsConfig>, name: &str, desc: &str) -> Histogram {
    let config = config.copied().unwrap_or_default();
    register_histogram!(name, desc, gen_linear_buckets(&config))
        .unwrap_or_else(|_| panic!("Failed to register latency histogram: {}", name))
}

pub(crate) static TXN_METRICS_MANAGER: LazyLock<TransactionMetrics> =
    LazyLock::new(|| TransactionMetrics::new(NonZeroUsize::new(100).unwrap()));

pub struct TransactionMetrics {
    created_txns_cache: Arc<RwLock<lru::LruCache<Vec<u8>, ()>>>,
    completed_txns_cache: Arc<RwLock<lru::LruCache<Vec<u8>, ()>>>,
    last_cleanup: RwLock<std::time::Instant>,
}

impl TransactionMetrics {
    pub fn new(capacity: NonZeroUsize) -> Self {
        Self {
            created_txns_cache: Arc::new(RwLock::new(lru::LruCache::new(capacity))),
            completed_txns_cache: Arc::new(RwLock::new(lru::LruCache::new(capacity))),
            last_cleanup: RwLock::new(std::time::Instant::now()),
        }
    }

    /// Returns true if the transaction is new (not seen before), false otherwise
    async fn is_new_transaction(&self, txn_hash: &[u8]) -> bool {
        let mut cache = self.created_txns_cache.write().await;
        if cache.contains(txn_hash) {
            false
        } else {
            cache.put(txn_hash.to_vec(), ());
            true
        }
    }

    /// Returns true if the transaction is new (not seen before), false otherwise
    async fn is_transaction_completed(&self, txn_hash: &[u8]) -> bool {
        let mut cache = self.completed_txns_cache.write().await;
        if cache.contains(txn_hash) {
            true
        } else {
            cache.put(txn_hash.to_vec(), ());
            false
        }
    }

    /// Marks a transaction as started
    /// Returns true if the transaction was newly started, false if it was already started
    pub async fn begin_transaction(
        &self,
        pool: &sqlx::PgPool,
        chain_id: i64,
        txn_id: &[u8],
        block_number: u64,
    ) -> Result<bool, sqlx::Error> {
        // Reduce DB writes by checking in-memory cache first
        if !self.is_new_transaction(txn_id).await {
            return Ok(false);
        }

        sqlx::query!(
        r#"
            INSERT INTO transactions (id, chain_id, created_at, block_number) VALUES ($1, $2, NOW(), $3)
            ON CONFLICT (id) DO NOTHING
        "#,
            txn_id,
            chain_id,
            block_number as i64
        )
        .execute(pool)
        .await?;

        // clean up old transactions on regular basis
        self.clean_up_transactions(pool).await;

        Ok(true)
    }

    async fn clean_up_transactions(&self, pool: &sqlx::PgPool) {
        let last_cleanup = self.last_cleanup.read().await.elapsed().as_secs();
        if last_cleanup < 60 * 60 {
            return;
        }
        let mut last_cleanup_write = self.last_cleanup.write().await;
        info!("Cleaning up old transactions");

        // Clean up old transactions
        // Completed transactions older than 1 day and incomplete transactions older than 7 days
        if let Err(err) = sqlx::query!(
            r#"
                DELETE FROM transactions
                WHERE (completed_at IS NOT NULL
                  AND created_at < NOW() - INTERVAL '1 day') OR (completed_at IS NULL
                  AND created_at < NOW() - INTERVAL '7 day')
            "#,
        )
        .execute(pool)
        .await
        {
            warn!(%err, "Failed to clean up old transactions");
            return;
        }

        info!("Cleaning up old transactions is done");

        *last_cleanup_write = std::time::Instant::now();
    }

    /// Marks a transaction as completed
    pub async fn end_transaction(
        &self,
        pool: &sqlx::PgPool,
        txn_id: &[u8],
        histogram: &prometheus::Histogram,
    ) -> Result<Option<f64>, sqlx::Error> {
        debug!(
            txn_id = %compact_hex(txn_id),
            "Marking transaction as completed, recording latency"
        );

        // Reduce DB writes by checking in-memory cache first
        if self.is_transaction_completed(txn_id).await {
            return Ok(None);
        }

        let mut trx = pool.begin().await?;

        // Lock the row to prevent duplicated histogram.observe calls
        let existing = sqlx::query!(
            r#"
            SELECT *
            FROM transactions
            WHERE id = $1 AND completed_at IS NOT NULL
            FOR UPDATE SKIP LOCKED
        "#,
            txn_id
        )
        .fetch_optional(trx.as_mut())
        .await?;

        if existing.is_some() {
            return Ok(None);
        }

        sqlx::query!(
            r#"
                UPDATE transactions
                SET completed_at = NOW()
                WHERE id = $1 AND completed_at IS NULL
            "#,
            txn_id
        )
        .execute(trx.as_mut())
        .await?;

        let res = Self::get_transaction_latency(trx.as_mut(), txn_id).await?;

        if let Some(latency) = res {
            if latency > 0.0 {
                let latency_sec = latency / 1000.0;
                info!(
                    txn_id = %compact_hex(txn_id),
                    latency_sec,
                    target = "latency",
                    "Transaction latency recorded"
                );
                histogram.observe(latency_sec);
            }
        }

        trx.commit().await?;
        Ok(res)
    }

    async fn get_transaction_latency(
        trx: &mut PgConnection,
        txn_id: &[u8],
    ) -> Result<Option<f64>, sqlx::Error> {
        let record = sqlx::query!(
            r#"
            SELECT EXTRACT(EPOCH FROM (completed_at - created_at)) * 1000 AS latency_ms
            FROM transactions
            WHERE id = $1 AND completed_at IS NOT NULL
        "#,
            txn_id
        )
        .fetch_optional(trx)
        .await?;

        Ok(record.and_then(|r| r.latency_ms.map(|v| v.to_f64().unwrap_or_default())))
    }
}

/// Marks a transaction as started using the global transaction manager
pub async fn try_begin_transaction(
    pool: &sqlx::PgPool,
    chain_id: i64,
    transaction_id: &[u8],
    block_number: u64,
) {
    if let Err(e) = TXN_METRICS_MANAGER
        .begin_transaction(pool, chain_id, transaction_id, block_number)
        .await
    {
        warn!(%e, "Failed to begin transaction");
    }
}

// Checks if all operations of the transaction are completed, and if so,
// records the transaction as completed.
// This function is idempotent and can be called multiple times safely
//
// The checks are relevant to L1 transactions only
pub async fn try_end_l1_transaction(
    pool: &sqlx::PgPool,
    transaction_id: &[u8],
) -> Result<(), sqlx::Error> {
    debug!(
        txn_id = %compact_hex(transaction_id),
        "Checking if L1 transaction can be ended"
    );

    let transaction_completed = sqlx::query!(
        "
            WITH
                cipher_all AS (
                SELECT COALESCE(BOOL_AND(COALESCE(txn_is_sent, false)), false) AS v
                FROM ciphertext_digest
                WHERE transaction_id = $1
            ),
            allowed_handles_all AS (
                SELECT COALESCE(BOOL_AND(COALESCE(txn_is_sent, false)), false) AS v
                FROM allowed_handles
                WHERE transaction_id = $1
            ),
            pbs_all AS (
                SELECT COALESCE(BOOL_AND(COALESCE(is_completed, false)), false) AS v
                FROM pbs_computations
                WHERE transaction_id = $1
            )
            SELECT (cipher_all.v AND allowed_handles_all.v AND pbs_all.v) AS all_ok
            FROM cipher_all, allowed_handles_all, pbs_all",
        transaction_id
    )
    .fetch_one(pool)
    .await
    .unwrap()
    .all_ok
    .unwrap_or(false);

    if transaction_completed {
        if let Err(e) = TXN_METRICS_MANAGER
            .end_transaction(pool, transaction_id, &HOST_TXN_LATENCY_HISTOGRAM)
            .await
        {
            warn!(%e, "Failed to end transaction");
        }
    }

    Ok(())
}

// Records the end of an zkproof transaction unconditionally.
// This function is idempotent and can be called multiple times safely
pub async fn try_end_zkproof_transaction(
    pool: &sqlx::PgPool,
    transaction_id: &[u8],
) -> Result<(), sqlx::Error> {
    if let Err(e) = TXN_METRICS_MANAGER
        .end_transaction(pool, transaction_id, &ZKPROOF_TXN_LATENCY_HISTOGRAM)
        .await
    {
        warn!(%e, "Failed to end transaction");
    }

    Ok(())
}
