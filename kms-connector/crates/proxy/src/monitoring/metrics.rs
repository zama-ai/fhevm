use connector_utils::monitoring::http::{HttpMetrics, ROUTES, UNMATCHED_ROUTE};
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
    for route in ROUTES.into_iter().filter(|&route| route != UNMATCHED_ROUTE) {
        HTTP_METRICS
            .requests
            .with_label_values(&[route, "200", "upstream"]);
    }
}
