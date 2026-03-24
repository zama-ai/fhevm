use axum::http::HeaderMap;
use axum::response::IntoResponse;
use prometheus::{
    register_counter_vec_with_registry, register_histogram_vec_with_registry, CounterVec,
    HistogramOpts, HistogramVec, Opts, Registry,
};
use reqwest::StatusCode;
use std::future::Future;
use std::sync::OnceLock;
use tokio::time::Instant;

use crate::config::settings::HttpMetricsConfig;

#[derive(Debug)]
struct HttpMetrics {
    requests_total: CounterVec,
    responses_total: CounterVec,
    request_duration_seconds: HistogramVec,
}

static HTTP_METRICS: OnceLock<HttpMetrics> = OnceLock::new();

/// Initialize HTTP metrics. Call this once at startup with the Prometheus registry.
pub fn init_http_metrics(registry: &Registry, config: &HttpMetricsConfig) {
    HTTP_METRICS.get_or_init(|| HttpMetrics {
        requests_total: register_counter_vec_with_registry!(
            Opts::new("relayer_http_requests_total", "Count of HTTP requests"),
            &[
                "endpoint",
                "method",
                "version",
                "relayer_sdk_name",
                "relayer_sdk_version"
            ],
            registry
        )
        .unwrap(),
        responses_total: register_counter_vec_with_registry!(
            Opts::new("relayer_http_responses_total", "Count of HTTP responses"),
            &["endpoint", "method", "version", "status"],
            registry
        )
        .unwrap(),
        request_duration_seconds: register_histogram_vec_with_registry!(
            HistogramOpts::new(
                "relayer_http_request_duration_seconds",
                "Histogram of HTTP request durations (seconds)"
            )
            .buckets(config.histogram_buckets.clone()),
            &["endpoint", "method", "version", "status"],
            registry
        )
        .unwrap(),
    });
}

/// Increment the HTTP requests_total metric.
pub fn requests_total(
    endpoint: HttpEndpoint,
    method: HttpMethod,
    version: HttpApiVersion,
    headers: HeaderMap,
) {
    let (sdk_name, sdk_version) = extract_sdk_info(&headers);
    let metrics = HTTP_METRICS.get().expect("HTTP metrics not initialized");
    metrics
        .requests_total
        .with_label_values(&[
            endpoint.as_str(),
            method.as_str(),
            version.as_str(),
            sdk_name,
            sdk_version,
        ])
        .inc();
}

/// Increment the HTTP responses_total metric.
pub fn responses_total(
    endpoint: HttpEndpoint,
    method: HttpMethod,
    version: HttpApiVersion,
    status_code: StatusCode,
) {
    let metrics = HTTP_METRICS.get().expect("HTTP metrics not initialized");
    metrics
        .responses_total
        .with_label_values(&[
            endpoint.as_str(),
            method.as_str(),
            version.as_str(),
            status_code.as_str(), // e.g., "200", "400", "429", "500"
        ])
        .inc();
}

/// Observe the HTTP request duration in seconds.
pub fn request_duration_seconds(
    endpoint: HttpEndpoint,
    method: HttpMethod,
    version: HttpApiVersion,
    status_code: StatusCode,
    duration: f64,
) {
    let metrics = HTTP_METRICS.get().expect("HTTP metrics not initialized");
    metrics
        .request_duration_seconds
        .with_label_values(&[
            endpoint.as_str(),
            method.as_str(),
            version.as_str(),
            status_code.as_str(),
        ])
        .observe(duration);
}

/// HTTP endpoints handled by the relayer.
#[derive(Debug, Clone, Copy)]
pub enum HttpEndpoint {
    InputProof,
    PublicDecrypt,
    UserDecrypt,
    DelegatedUserDecrypt,
    KeyUrl,
    Unknown,
}

// TODO: Add a tag with version (v2 only for now) to support new routes.
impl HttpEndpoint {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpEndpoint::InputProof => "/input-proof",
            HttpEndpoint::PublicDecrypt => "/public-decrypt",
            HttpEndpoint::UserDecrypt => "/user-decrypt",
            HttpEndpoint::DelegatedUserDecrypt => "/delegated-user-decrypt",
            HttpEndpoint::KeyUrl => "/keyurl",
            HttpEndpoint::Unknown => "unknown",
        }
    }
}

/// API Version tag.
#[derive(Debug, Clone, Copy)]
pub enum HttpApiVersion {
    V2,
}

impl HttpApiVersion {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpApiVersion::V2 => "v2",
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
    version: HttpApiVersion,
    headers: HeaderMap,
    fut: Fut,
) -> impl IntoResponse
where
    Fut: Future<Output = R>,
    R: IntoResponse,
{
    requests_total(endpoint, method, version, headers);

    let start = Instant::now();

    let response = fut.await.into_response();

    let status_code = response.status();

    responses_total(endpoint, method, version, status_code);
    request_duration_seconds(
        endpoint,
        method,
        version,
        status_code,
        start.elapsed().as_secs_f64(),
    );

    response
}

fn extract_sdk_info(headers: &HeaderMap) -> (&'static str, &'static str) {
    let sdk_name = headers
        .get("zama-sdk-name")
        .and_then(|v| v.to_str().ok())
        .and_then(|name| {
            if name == "@zama-fhe/relayer-sdk" {
                Some("@zama-fhe/relayer-sdk")
            } else {
                None
            }
        })
        .unwrap_or("unknown");

    let sdk_version = headers
        .get("zama-sdk-version")
        .and_then(|v| v.to_str().ok())
        .and_then(|version| {
            // Extract major.minor only (e.g., "0.4.0-alpha.4" -> "0.4")
            let parts: Vec<&str> = version.split('.').collect();
            if parts.len() >= 2 {
                if parts[0].parse::<u32>().is_ok() && parts[1].parse::<u32>().is_ok() {
                    // Map to known versions to avoid unbounded cardinality
                    match (parts[0], parts[1]) {
                        ("0", "4") => Some("0.4"),
                        ("0", "5") => Some("0.5"),
                        // Add more known versions as needed
                        _ => None,
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap_or("unknown");

    (sdk_name, sdk_version)
}
