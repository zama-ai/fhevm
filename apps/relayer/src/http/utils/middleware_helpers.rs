use crate::config::settings::RateLimitConfig;
use crate::http::middleware::create_rate_limit_error_handler;
use reqwest::Method;
use tower_governor::{
    governor::GovernorConfigBuilder, key_extractor::GlobalKeyExtractor, GovernorLayer,
};

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
