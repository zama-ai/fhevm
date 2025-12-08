use crate::config::settings::RateLimitConfig;
use crate::http::AppResponse;
use axum::response::IntoResponse;
use rand::Rng;
use reqwest::Method;
use tower_governor::{
    governor::GovernorConfigBuilder, key_extractor::GlobalKeyExtractor, GovernorError,
    GovernorLayer,
};

/// Custom error handler for rate limiting that returns structured JSON responses.
pub fn create_rate_limit_error_handler(
    config: &RateLimitConfig,
) -> impl Fn(GovernorError) -> axum::response::Response + Clone {
    let base_retry_after_seconds = config.retry_after_seconds;
    let jitter_max_ms = config.jitter_max_ms;

    move |_err: GovernorError| {
        // Calculate retry-after with millisecond precision jitter
        let base_ms = base_retry_after_seconds * 1000;
        let total_ms = if jitter_max_ms > 0 {
            let jitter = rand::rng().random_range(0..=jitter_max_ms);
            base_ms + jitter
        } else {
            base_ms
        };

        // Convert to seconds for Retry-After header (rounded up)
        let retry_after_seconds = total_ms.div_ceil(1000); // Round up to nearest second

        AppResponse::<()>::rate_limited(
            "Too many requests at relayer HTTP server",
            &retry_after_seconds.to_string(), // Pass as seconds string for header
        )
        .into_response()
    }
}

/// Applies rate limiting layer to a router using the provided configuration.
/// This encapsulates all rate limiting setup logic in one place.
pub fn with_rate_limiting<S>(router: axum::Router<S>, config: &RateLimitConfig) -> axum::Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let ms_per_token = (1000 / config.requests_per_second) as u64;
    let governor_conf = GovernorConfigBuilder::default()
        .per_millisecond(ms_per_token)
        .burst_size(config.burst_size)
        .methods(vec![Method::POST])
        .key_extractor(GlobalKeyExtractor)
        .finish()
        .unwrap();

    let rate_limiting_layer =
        GovernorLayer::new(governor_conf).error_handler(create_rate_limit_error_handler(config));
    router.layer(rate_limiting_layer)
}
