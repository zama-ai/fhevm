use bigdecimal::num_traits::ToPrimitive;
use futures::lock::Mutex;
use opentelemetry::{
    global::{BoxedSpan, BoxedTracer, ObjectSafeSpan},
    trace::{SpanBuilder, Status, TraceContextExt, Tracer},
    Context, KeyValue,
};
use opentelemetry_sdk::{trace::SdkTracerProvider, Resource};
use prometheus::{register_histogram, Histogram};
use rand::rand_core::block;
use sqlx::PgConnection;
use std::{
    collections::HashSet,
    sync::{Arc, LazyLock},
    time::SystemTime,
};
use tracing::{info, warn};

use crate::utils::compact_hex;
pub const TXN_ID_ATTR_KEY: &str = "txn_id";
pub const GLOBAL_LATENCY_METRIC_NAME: &str = "coprocessor_txn_latency_seconds";

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

/// Insert a transaction in transactions table
pub async fn record_txn_created(
    pool: &sqlx::PgPool,
    txn_id: &[u8],
    src_id: i32,
    block_number: u64,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO transactions (id, src_id, created_at, block_number) VALUES ($1, $2, NOW(), $3)
        ON CONFLICT (id) DO NOTHING
        "#,
        txn_id,
        src_id,
        block_number as i64
    )
    .execute(pool)
    .await?;

    // Do some GC
    sqlx::query!(
        r#"
        DELETE FROM transactions
        WHERE created_at < NOW() - INTERVAL '1 day'
        "#,
    )
    .execute(pool)
    .await?;
    Ok(())
}

use once_cell::sync::Lazy;
// create hashset of txn ids to avoid duplicate processing. Use lazy static to create a static variable
// TODO: This should be LRU cache
pub static TXN_IDS: Lazy<Mutex<HashSet<Vec<u8>>>> = Lazy::new(|| Mutex::new(HashSet::new()));

pub async fn record_txn_completed(
    pool: &sqlx::PgPool,
    txn_id: &[u8],
    histogram: &prometheus::Histogram,
) -> Result<Option<f64>, sqlx::Error> {
    // check if txn is completed

    let mut set = TXN_IDS.lock().await;
    if set.contains(txn_id) {
        warn!(
            "Transaction {:?} already completed, skipping",
            compact_hex(txn_id)
        );
        return Ok(None);
    }
    set.insert(txn_id.to_vec());

    // begin db txn
    let mut trx: sqlx::Transaction<'static, sqlx::Postgres> = pool.begin().await?;

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
        WHERE id = $1
        "#,
        txn_id
    )
    .execute(trx.as_mut())
    .await?;

    let res = get_transaction_latency(trx.as_mut(), txn_id).await?;

    if let Some(latency) = res {
        if latency > 0.0 {
            let latency_sec = latency / 1000.0;
            info!(
                "Transaction {:?} latency: {} seconds",
                compact_hex(txn_id),
                latency_sec
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

pub(crate) static L1_TXN_LATENCY_HISTOGRAM: LazyLock<Histogram> = LazyLock::new(|| {
    let mut buckets = Vec::new();
    let mut v = 0.25;
    while v <= 2000.0 {
        buckets.push(v);
        v += 0.25;
    }

    register_histogram!(
        GLOBAL_LATENCY_METRIC_NAME,
        "L1 transaction latencies in seconds",
        buckets
    )
    .unwrap()
});

pub async fn record_transaction_completed(
    pool: &sqlx::PgPool,
    l1_transaction_id: &Vec<u8>,
) -> Result<(), sqlx::Error> {
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
        l1_transaction_id
    )
    .fetch_one(pool)
    .await
    .unwrap()
    .all_ok
    .unwrap_or(false);

    if transaction_completed {
        record_txn_completed(&pool, &l1_transaction_id, &L1_TXN_LATENCY_HISTOGRAM)
            .await
            .unwrap();
    }

    Ok(())
}
