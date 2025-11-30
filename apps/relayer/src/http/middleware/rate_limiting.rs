use crate::config::settings::RateLimitConfig;
use crate::http::AppResponse;
use axum::response::IntoResponse;
use chrono::Utc;
use rand::Rng;
use tower_governor::GovernorError;

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

        // Generate RFC 7231 timestamp indicating when client should retry
        // Uses absolute timestamp instead of relative seconds for cache-safety
        let retry_time = Utc::now() + chrono::Duration::milliseconds(total_ms as i64);
        let retry_after_timestamp = retry_time.format("%a, %d %b %Y %H:%M:%S GMT").to_string();

        AppResponse::<()>::rate_limited(
            "Too many requests at relayer HTTP server",
            &retry_after_timestamp,
        )
        .into_response()
    }
}
