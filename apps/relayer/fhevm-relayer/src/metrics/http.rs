use std::future::Future;

use axum::response::IntoResponse;
use http::StatusCode;
use once_cell::sync::OnceCell;
use prometheus::{
    register_counter_vec_with_registry, register_histogram_vec_with_registry, CounterVec,
    HistogramOpts, HistogramVec, Opts, Registry,
};
use tokio::time::Instant;

use crate::config::settings::HttpMetricsConfig;

#[derive(Debug)]
struct HttpMetrics {
    requests_total: CounterVec,
    responses_total: CounterVec,
    request_duration_seconds: HistogramVec,
}

static HTTP_METRICS: OnceCell<HttpMetrics> = OnceCell::new();

/// Initialize HTTP metrics. Call this once at startup with the Prometheus registry.
pub fn init_http_metrics(registry: &Registry, config: &HttpMetricsConfig) {
    let requests_total = register_counter_vec_with_registry!(
        Opts::new("relayer_http_requests_total", "Count of HTTP requests"),
        &["endpoint", "method"],
        registry
    )
    .unwrap();

    let responses_total = register_counter_vec_with_registry!(
        Opts::new("relayer_http_responses_total", "Count of HTTP responses"),
        &["endpoint", "method", "status"],
        registry
    )
    .unwrap();

    let buckets = &config.histogram_buckets;

    let request_duration_seconds = register_histogram_vec_with_registry!(
        HistogramOpts::new(
            "relayer_http_request_duration_seconds",
            "Histogram of HTTP request durations (seconds)"
        )
        .buckets(buckets.clone()),
        &["endpoint", "method", "status"],
        registry
    )
    .unwrap();

    HTTP_METRICS
        .set(HttpMetrics {
            requests_total,
            responses_total,
            request_duration_seconds,
        })
        .expect("HTTP metrics already initialized");
}

/// Increment the HTTP requests_total metric.
pub fn requests_total(endpoint: HttpEndpoint, method: HttpMethod) {
    let metrics = HTTP_METRICS.get().expect("HTTP metrics not initialized");
    metrics
        .requests_total
        .with_label_values(&[endpoint.as_str(), method.as_str()])
        .inc();
}

/// Increment the HTTP responses_total metric.
pub fn responses_total(endpoint: HttpEndpoint, method: HttpMethod, status_code: StatusCode) {
    let metrics = HTTP_METRICS.get().expect("HTTP metrics not initialized");
    metrics
        .responses_total
        .with_label_values(&[endpoint.as_str(), method.as_str(), status_code.as_str()])
        .inc();
}

/// Observe the HTTP request duration in seconds.
pub fn request_duration_seconds(
    endpoint: HttpEndpoint,
    method: HttpMethod,
    status_code: StatusCode,
    duration: f64,
) {
    let metrics = HTTP_METRICS.get().expect("HTTP metrics not initialized");
    metrics
        .request_duration_seconds
        .with_label_values(&[endpoint.as_str(), method.as_str(), status_code.as_str()])
        .observe(duration);
}

/// HTTP endpoints handled by the relayer.
#[derive(Debug, Clone, Copy)]
pub enum HttpEndpoint {
    InputProof,
    PublicDecrypt,
    UserDecrypt,
    KeyUrl,
    Unknown,
}

impl HttpEndpoint {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpEndpoint::InputProof => "/input-proof",
            HttpEndpoint::PublicDecrypt => "/public-decrypt",
            HttpEndpoint::UserDecrypt => "/user-decrypt",
            HttpEndpoint::KeyUrl => "/keyurl",
            HttpEndpoint::Unknown => "unknown",
        }
    }
}

/// HTTP methods.
#[derive(Debug, Clone, Copy)]
pub enum HttpMethod {
    Get,
    Post,
    Unknown,
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Unknown => "UNKNOWN",
        }
    }
}

/// HTTP response/request status.
#[derive(Debug, Clone, Copy)]
pub enum HttpStatus {
    Success,
    Error,
    Unknown,
}

impl HttpStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpStatus::Success => "success",
            HttpStatus::Error => "error",
            HttpStatus::Unknown => "unknown",
        }
    }
}

// Helper for HTTP metrics instrumentation
pub async fn with_http_metrics<Fut, R>(
    endpoint: HttpEndpoint,
    method: HttpMethod,
    fut: Fut,
) -> impl IntoResponse
where
    Fut: Future<Output = R>,
    R: IntoResponse,
{
    requests_total(endpoint, method);

    let start = Instant::now();

    let response = fut.await.into_response();

    let status_code = response.status();
    responses_total(endpoint, method, status_code);
    request_duration_seconds(endpoint, method, status_code, start.elapsed().as_secs_f64());
    response
}
