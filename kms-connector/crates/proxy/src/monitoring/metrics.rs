use connector_utils::monitoring::http::HttpMetrics;
use kms_connector_api::{PUBLIC_DECRYPTION_ROUTE, USER_DECRYPTION_ROUTE, VERSION_ROUTE};
use std::sync::LazyLock;

/// Proxy request counts by route, status, and code, request durations by route, and active
/// requests whose headers have been parsed.
pub static HTTP_METRICS: LazyLock<HttpMetrics> =
    LazyLock::new(|| HttpMetrics::register("kms_connector_proxy"));

/// Registers proxy metrics and creates zero-valued success counters for every API route.
///
/// Called at proxy startup so metrics appear in scrapes before any requests arrive.
/// `LazyLock` lets us construct and register each global metric once at runtime;
/// forcing it here makes registration happen at startup rather than on the first request.
pub fn init() {
    for route in [
        PUBLIC_DECRYPTION_ROUTE,
        USER_DECRYPTION_ROUTE,
        VERSION_ROUTE,
    ] {
        HTTP_METRICS
            .requests
            .with_label_values(&[route, "200", "upstream"]);
    }
}
