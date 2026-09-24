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
            &sdk_version,
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
    V3,
}

impl HttpApiVersion {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpApiVersion::V2 => "v2",
            HttpApiVersion::V3 => "v3",
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

fn extract_sdk_info(headers: &HeaderMap) -> (&'static str, String) {
    let sdk_name = headers
        .get("zama-sdk-name")
        .and_then(|v| v.to_str().ok())
        .and_then(|name| match name {
            "@zama-fhe/relayer-sdk" => Some("@zama-fhe/relayer-sdk"),
            "@fhevm/sdk" => Some("@fhevm/sdk"),
            _ => None,
        })
        .unwrap_or("unknown");

    let sdk_version = headers
        .get("zama-sdk-version")
        .and_then(|v| v.to_str().ok())
        .and_then(parse_major_minor)
        .unwrap_or_else(|| "unknown".to_string());

    (sdk_name, sdk_version)
}

/// Extract major.minor only (e.g., "0.4.0-alpha.4" -> "0.4").
/// Bounded to major in 0..=9 and minor in 0..=99 to avoid unbounded label cardinality.
fn parse_major_minor(version: &str) -> Option<String> {
    let mut parts = version.split('.');
    let major = parts.next()?;
    let minor = parts.next()?;
    if !major.bytes().all(|b| b.is_ascii_digit()) || !minor.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let major: u8 = major.parse().ok()?;
    let minor: u8 = minor.parse().ok()?;
    if major > 9 || minor > 99 {
        return None;
    }
    // Re-format from parsed numbers so e.g. "00.04" and "0.4" map to the same label
    Some(format!("{major}.{minor}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_major_minor_accepts_bounded_versions() {
        assert_eq!(parse_major_minor("0.4.0-alpha.4").as_deref(), Some("0.4"));
        assert_eq!(parse_major_minor("0.15.0-0").as_deref(), Some("0.15"));
        assert_eq!(parse_major_minor("9.99").as_deref(), Some("9.99"));
        assert_eq!(parse_major_minor("00.04.1").as_deref(), Some("0.4"));
    }

    #[test]
    fn parse_major_minor_rejects_out_of_range_or_malformed() {
        assert_eq!(parse_major_minor("10.0.0"), None);
        assert_eq!(parse_major_minor("0.100.0"), None);
        assert_eq!(parse_major_minor("+1.2"), None);
        assert_eq!(parse_major_minor("1"), None);
        assert_eq!(parse_major_minor("1.x"), None);
        assert_eq!(parse_major_minor(".1"), None);
        assert_eq!(parse_major_minor("99999999999.1"), None);
    }

    #[test]
    fn extract_sdk_info_from_headers() {
        let mut headers = HeaderMap::new();
        headers.insert("ZAMA-SDK-NAME", "@fhevm/sdk".parse().unwrap());
        headers.insert("zama-sdk-version", "0.15.0-0".parse().unwrap());
        assert_eq!(
            extract_sdk_info(&headers),
            ("@fhevm/sdk", "0.15".to_string())
        );

        let mut headers = HeaderMap::new();
        headers.insert("zama-sdk-name", "evil".parse().unwrap());
        headers.insert("zama-sdk-version", "42.0".parse().unwrap());
        assert_eq!(
            extract_sdk_info(&headers),
            ("unknown", "unknown".to_string())
        );
    }
}
