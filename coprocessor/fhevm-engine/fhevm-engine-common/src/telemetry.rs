use crate::chain_id::ChainId;
use crate::utils::to_hex;
use bigdecimal::num_traits::ToPrimitive;
use opentelemetry::{trace::TraceContextExt, trace::TracerProvider, KeyValue};
use opentelemetry_sdk::{trace::SdkTracerProvider, Resource};
use prometheus::{register_histogram, Histogram};
use serde_json::Value;
use sqlx::PgConnection;
use std::fmt;
use std::{
    num::NonZeroUsize,
    str::FromStr,
    sync::{Arc, LazyLock, OnceLock},
};
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn, Subscriber};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_subscriber::{
    fmt::{format, FmtContext, FormatEvent, FormatFields},
    layer::SubscriberExt,
    registry::LookupSpan,
    util::SubscriberInitExt,
};

/// Calls provider shutdown exactly once when dropped.
pub struct TracerProviderGuard {
    tracer_provider: Option<SdkTracerProvider>,
}

impl TracerProviderGuard {
    fn new(trace_provider: SdkTracerProvider) -> Self {
        Self {
            tracer_provider: Some(trace_provider),
        }
    }

    fn shutdown_once(&mut self) {
        if let Some(provider) = self.tracer_provider.take() {
            if let Err(err) = provider.shutdown() {
                warn!(error = %err, "Failed to shutdown OTLP tracer provider");
            }
        }
    }
}

impl Drop for TracerProviderGuard {
    fn drop(&mut self) {
        self.shutdown_once();
    }
}

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

/// JSON log formatter that wraps the standard JSON formatter and injects
/// `trace_id` and `span_id` fields from the current OTel context into every
/// log event. This enables log→trace correlation in tools like Grafana/Loki.
///
/// When no OTel span is active (e.g., during init), the fields are omitted.
struct OtelJsonFormat {
    inner: format::Format<format::Json>,
}

impl OtelJsonFormat {
    fn new() -> Self {
        Self {
            inner: tracing_subscriber::fmt::format()
                .json()
                .with_target(false)
                .with_current_span(true)
                .with_span_list(false)
                .with_level(true),
        }
    }
}

/// Injects `trace_id` and `span_id` into a JSON log line.
///
/// Parses `buf` as a JSON object, inserts the two fields, and returns the
/// re-serialized string. Returns `None` if `buf` is not valid JSON.
///
/// Using serde_json (rather than raw string surgery) means the function is
/// correct regardless of how tracing_subscriber serializes the trailing bytes.
pub(crate) fn inject_otel_fields(buf: &str, trace_id: &str, span_id: &str) -> Option<String> {
    let mut map: serde_json::Map<String, Value> = serde_json::from_str(buf.trim_end()).ok()?;
    map.insert("trace_id".to_owned(), Value::String(trace_id.to_owned()));
    map.insert("span_id".to_owned(), Value::String(span_id.to_owned()));
    serde_json::to_string(&map).ok()
}

impl<S, N> FormatEvent<S, N> for OtelJsonFormat
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: format::Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> fmt::Result {
        let mut buf = String::new();
        self.inner
            .format_event(ctx, format::Writer::new(&mut buf), event)?;

        // Skip injection when the entered span differs from the event's
        // logical parent (e.g. `parent: &span` in async code) — injecting
        // the wrong span's context is worse than omitting it.
        let current_matches =
            ctx.parent_span().map(|s| s.id()) == ctx.lookup_current().map(|s| s.id());
        let otel_cx = tracing::Span::current().context();
        let otel_span = otel_cx.span();
        let sc = otel_span.span_context();

        if current_matches && sc.is_valid() {
            if let Some(injected) =
                inject_otel_fields(&buf, &sc.trace_id().to_string(), &sc.span_id().to_string())
            {
                return writeln!(writer, "{}", injected);
            }
        }

        write!(writer, "{}", buf)
    }
}

pub fn init_json_subscriber(
    log_level: tracing::Level,
    service_name: &str,
    tracer_name: &'static str,
) -> Result<Option<TracerProviderGuard>, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let level_filter = tracing_subscriber::filter::LevelFilter::from_level(log_level);
    let fmt_layer = tracing_subscriber::fmt::layer()
        .event_format(OtelJsonFormat::new())
        .fmt_fields(tracing_subscriber::fmt::format::JsonFields::new());
    let base = tracing_subscriber::registry()
        .with(level_filter)
        .with(fmt_layer);

    if service_name.is_empty() {
        base.try_init()?;
        return Ok(None);
    }

    let (tracer, trace_provider) = match setup_otel_with_tracer(service_name, tracer_name) {
        Ok(v) => v,
        Err(err) => {
            // Keep JSON logs even if OTLP export cannot be initialized.
            base.try_init()?;
            return Err(err);
        }
    };

    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);
    base.with(telemetry_layer).try_init()?;
    opentelemetry::global::set_tracer_provider(trace_provider.clone());
    Ok(Some(TracerProviderGuard::new(trace_provider)))
}

/// Initializes tracing with JSON logs and best-effort OTLP export.
///
/// Fallback here means "logs-only mode": if OTLP setup fails, we keep
/// JSON logging enabled and continue execution without an OTLP exporter.
/// It does not try alternate OTLP endpoints.
pub fn init_tracing_otel_with_logs_only_fallback(
    log_level: tracing::Level,
    service_name: &str,
    tracer_name: &'static str,
) -> Option<TracerProviderGuard> {
    match init_json_subscriber(log_level, service_name, tracer_name) {
        Ok(guard) => guard,
        Err(err) => {
            error!(error = %err, "Failed to setup OTLP");
            None
        }
    }
}

fn setup_otel_with_tracer(
    service_name: &str,
    tracer_name: &'static str,
) -> Result<
    (opentelemetry_sdk::trace::Tracer, SdkTracerProvider),
    Box<dyn std::error::Error + Send + Sync + 'static>,
> {
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

    let tracer = trace_provider.tracer(tracer_name);
    Ok((tracer, trace_provider))
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

pub fn set_current_span_error(error: &impl fmt::Display) {
    tracing::Span::current()
        .context()
        .span()
        .set_status(opentelemetry::trace::Status::Error {
            description: error.to_string().into(),
        });
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
        chain_id: ChainId,
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
            chain_id.as_i64(),
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
            txn_id = %to_hex(txn_id),
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
                    txn_id = %to_hex(txn_id),
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
    chain_id: ChainId,
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
        txn_id = %to_hex(transaction_id),
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use tracing_subscriber::fmt::MakeWriter;

    /// In-memory writer for capturing formatter output in tests.
    #[derive(Clone)]
    struct BufWriter(Arc<Mutex<Vec<u8>>>);

    impl BufWriter {
        fn new() -> (Self, Arc<Mutex<Vec<u8>>>) {
            let buf = Arc::new(Mutex::new(Vec::new()));
            (Self(buf.clone()), buf)
        }
    }

    impl std::io::Write for BufWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl<'a> MakeWriter<'a> for BufWriter {
        type Writer = BufWriter;
        fn make_writer(&'a self) -> Self::Writer {
            self.clone()
        }
    }

    fn test_subscriber(writer: BufWriter) -> impl tracing::Subscriber {
        tracing_subscriber::fmt::Subscriber::builder()
            .event_format(OtelJsonFormat::new())
            .fmt_fields(tracing_subscriber::fmt::format::JsonFields::new())
            .with_writer(writer)
            .with_max_level(tracing::Level::INFO)
            .finish()
    }

    #[test]
    fn otel_guard_shutdown_once_disarms_provider() {
        let provider = SdkTracerProvider::builder().build();
        let mut guard = TracerProviderGuard::new(provider);
        assert!(guard.tracer_provider.is_some());

        guard.shutdown_once();
        assert!(guard.tracer_provider.is_none());

        // A second shutdown is a no-op.
        guard.shutdown_once();
        assert!(guard.tracer_provider.is_none());
    }

    // --- inject_otel_fields / OtelJsonFormat tests ---

    #[test]
    fn inject_otel_fields_adds_trace_and_span_ids() {
        let buf = r#"{"level":"INFO","message":"test"}"#;
        let result = inject_otel_fields(buf, "abc123", "def456").unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["trace_id"], "abc123");
        assert_eq!(parsed["span_id"], "def456");
    }

    #[test]
    fn inject_otel_fields_preserves_existing_fields() {
        let buf = r#"{"level":"INFO","message":"hello"}"#;
        let result = inject_otel_fields(buf, "t", "s").unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["level"], "INFO");
        assert_eq!(parsed["message"], "hello");
    }

    #[test]
    fn inject_otel_fields_tolerates_trailing_newline() {
        // tracing_subscriber currently appends "}\n"; this must keep working even
        // if the inner formatter changes its trailing whitespace.
        let buf = "{\"level\":\"INFO\",\"message\":\"test\"}\n";
        let result = inject_otel_fields(buf, "t", "s").unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["trace_id"], "t");
    }

    #[test]
    fn inject_otel_fields_returns_none_for_invalid_json() {
        assert!(inject_otel_fields("not json", "t", "s").is_none());
        assert!(inject_otel_fields("", "t", "s").is_none());
    }

    /// Verifies that OtelJsonFormat does NOT inject trace_id/span_id when
    /// there is no active OTel span (i.e., the span context is invalid).
    #[test]
    fn otel_json_format_no_injection_without_active_span() {
        let (writer, buf) = BufWriter::new();
        let subscriber = test_subscriber(writer);

        tracing::subscriber::with_default(subscriber, || {
            tracing::info!("no span active");
        });

        let output = String::from_utf8(buf.lock().unwrap().clone()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(output.trim()).unwrap();
        assert!(
            parsed.get("trace_id").is_none(),
            "trace_id should be absent when no OTel span is active"
        );
        assert!(
            parsed.get("span_id").is_none(),
            "span_id should be absent when no OTel span is active"
        );
    }

    /// When an event uses `parent: &span` on a non-entered span, the entered
    /// span differs from the event's logical parent. The formatter should skip
    /// injection rather than emit the wrong span's context.
    #[test]
    fn otel_json_format_skips_injection_for_explicit_parent() {
        let (writer, buf) = BufWriter::new();
        let subscriber = test_subscriber(writer);

        tracing::subscriber::with_default(subscriber, || {
            let span = tracing::info_span!("not_entered");
            // Log with explicit parent — span is NOT entered, so
            // parent_span() != lookup_current() and injection is skipped.
            tracing::info!(parent: &span, "explicit parent");
        });

        let output = String::from_utf8(buf.lock().unwrap().clone()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(output.trim()).unwrap();
        assert!(
            parsed.get("trace_id").is_none(),
            "trace_id should be absent when parent span is not entered"
        );
        assert!(
            parsed.get("span_id").is_none(),
            "span_id should be absent when parent span is not entered"
        );
    }
}
