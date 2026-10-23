//! HTTP request tracing and metrics shared by the proxy and endpoint.

use kms_connector_api::{PUBLIC_DECRYPTION_ROUTE, USER_DECRYPTION_ROUTE, VERSION_ROUTE};
use opentelemetry::{
    Context,
    propagation::{Extractor, Injector, TextMapPropagator},
};
use opentelemetry_sdk::propagation::TraceContextPropagator;
use prometheus::{
    HistogramOpts, HistogramVec, IntCounterVec, IntGauge, register_histogram_vec,
    register_int_counter_vec, register_int_gauge,
};
use std::time::{Duration, Instant};
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

/// `route` label of requests matching none of the API routes.
pub const UNMATCHED_ROUTE: &str = "unmatched";
/// W3C trace context headers.
pub const TRACEPARENT: &str = "traceparent";
pub const TRACESTATE: &str = "tracestate";
/// Histogram bucket upper bounds in seconds, covering immediate responses up to the proxy's
/// default 60-second endpoint response timeout (the endpoint's default decryption timeout is 30
/// seconds).
pub const DURATION_BUCKETS: &[f64] = &[
    0.01, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0, 20.0, 30.0, 45.0, 60.0,
];

/// The `route` label of a request path: the API route it matches, or [`UNMATCHED_ROUTE`].
pub fn route_label(path: &str) -> &'static str {
    match path {
        PUBLIC_DECRYPTION_ROUTE => PUBLIC_DECRYPTION_ROUTE,
        USER_DECRYPTION_ROUTE => USER_DECRYPTION_ROUTE,
        VERSION_ROUTE => VERSION_ROUTE,
        _ => UNMATCHED_ROUTE,
    }
}

/// The trace context headers of a request: borrowed when extracted, owned when injected.
// TODO: This can be replaced by `opentelemetry_http`'s `HeaderExtractor`/`HeaderInjector` once actix-web moves to
// `http` 1.x
#[derive(Default)]
struct TraceHeaders<S> {
    traceparent: Option<S>,
    tracestate: Option<S>,
}

impl<S: AsRef<str>> Extractor for TraceHeaders<S> {
    fn get(&self, key: &str) -> Option<&str> {
        match key {
            TRACEPARENT => self.traceparent.as_ref().map(|value| value.as_ref()),
            TRACESTATE => self.tracestate.as_ref().map(|value| value.as_ref()),
            _ => None,
        }
    }

    fn keys(&self) -> Vec<&str> {
        [
            (TRACEPARENT, &self.traceparent),
            (TRACESTATE, &self.tracestate),
        ]
        .into_iter()
        .filter_map(|(key, value)| value.as_ref().map(|_| key))
        .collect()
    }
}

impl Injector for TraceHeaders<String> {
    fn set(&mut self, key: &str, value: String) {
        match key {
            TRACEPARENT => self.traceparent = Some(value),
            // The propagator emits an empty tracestate for a fresh root; omit that header.
            TRACESTATE if !value.is_empty() => self.tracestate = Some(value),
            _ => {}
        }
    }
}

/// Extracts the W3C trace context of a request, reading its headers with `header`.
///
/// The OpenTelemetry propagator validates the header values and ignores invalid trace context.
pub fn extract_context<'h>(header: impl Fn(&'static str) -> Option<&'h str>) -> Context {
    let headers = TraceHeaders {
        traceparent: header(TRACEPARENT),
        tracestate: header(TRACESTATE),
    };
    TraceContextPropagator::new().extract(&headers)
}

/// The W3C trace context headers propagating `span`; none for a disabled span.
///
/// Only W3C trace context is injected; never caller baggage.
pub fn trace_headers(span: &Span) -> impl Iterator<Item = (&'static str, String)> {
    let mut headers = TraceHeaders::<String>::default();
    TraceContextPropagator::new().inject_context(&span.context(), &mut headers);
    [
        (TRACEPARENT, headers.traceparent),
        (TRACESTATE, headers.tracestate),
    ]
    .into_iter()
    .filter_map(|(name, value)| Some((name, value?)))
}

/// Request counters, duration histograms and in-flight gauges shared by the proxy and endpoint.
///
/// [`RequestObservation`] updates them on completion or cancellation.
/// Request counts have `route`, `status`, and `code` labels. Durations are grouped by route.
pub struct HttpMetrics {
    pub requests: IntCounterVec,
    pub duration: HistogramVec,
    pub in_flight: IntGauge,
}

impl HttpMetrics {
    /// Registers request counts, durations and in-flight requests under the service's metric
    /// prefix.
    pub fn register(prefix: &str) -> Self {
        let requests_name = format!("{prefix}_http_requests_counter");
        let requests = register_int_counter_vec!(
            &requests_name,
            "Finished HTTP requests, including cancellations; status 0 means no response",
            &["route", "status", "code"]
        )
        .unwrap_or_else(|e| panic!("Failed to register {requests_name} metric: {e}"));
        let duration_name = format!("{prefix}_http_request_latency_seconds");
        let duration = register_histogram_vec!(
            HistogramOpts::new(&duration_name, "HTTP request duration in seconds")
                .buckets(DURATION_BUCKETS.to_vec()),
            &["route"]
        )
        .unwrap_or_else(|e| panic!("Failed to register {duration_name} metric: {e}"));
        let in_flight_name = format!("{prefix}_http_requests_in_flight");
        let in_flight =
            register_int_gauge!(&in_flight_name, "Active HTTP requests after header parsing")
                .unwrap_or_else(|e| panic!("Failed to register {in_flight_name} metric: {e}"));
        for route in [
            PUBLIC_DECRYPTION_ROUTE,
            USER_DECRYPTION_ROUTE,
            VERSION_ROUTE,
            UNMATCHED_ROUTE,
        ] {
            duration.with_label_values(&[route]);
        }
        Self {
            requests,
            duration,
            in_flight,
        }
    }

    /// Starts observing an active request; dropping the observation records its count and
    /// elapsed time.
    pub fn start(&'static self, route: &'static str) -> RequestObservation {
        self.in_flight.inc();
        RequestObservation {
            metrics: self,
            route,
            started_at: Instant::now(),
            status: 0,
            code: "cancelled",
        }
    }
}

/// Counts a request as in flight, then records its count and duration sample when dropped,
/// including on cancellation.
///
/// Cancellation or panic unwinding before [`Self::finish`] is counted as `cancelled`.
/// A process abort or forced termination skips `Drop`, so recording is not guaranteed.
pub struct RequestObservation {
    metrics: &'static HttpMetrics,
    pub route: &'static str,
    started_at: Instant,
    status: u16,
    code: &'static str,
}

impl RequestObservation {
    /// Time elapsed since the request started.
    pub fn elapsed(&self) -> Duration {
        self.started_at.elapsed()
    }

    /// Sets the final status and code and records them on the request's tracing span.
    ///
    /// Called by the proxy's `ProxyHttp::logging` callback when proxying ends, and by the
    /// endpoint's `observe_request` middleware when processing returns a response or an error.
    /// The count and duration are recorded on drop. If cancellation or panic prevents this call,
    /// the observation keeps status `0` and code `cancelled`.
    pub fn finish(&mut self, status: u16, code: &'static str, span: &Span) {
        self.status = status;
        self.code = code;
        span.record("http.response.status_code", status);
        span.record("code", code);
    }
}

impl Drop for RequestObservation {
    fn drop(&mut self) {
        self.metrics.in_flight.dec();
        self.metrics
            .requests
            .with_label_values(&[self.route, &self.status.to_string(), self.code])
            .inc();
        self.metrics
            .duration
            .with_label_values(&[self.route])
            .observe(self.elapsed().as_secs_f64());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use opentelemetry::trace::{TraceContextExt, TracerProvider};
    use opentelemetry_sdk::trace::SdkTracerProvider;
    use std::collections::HashMap;
    use tracing_subscriber::layer::SubscriberExt;

    #[test]
    // Headers injected from a span extract back to that span's context. Checks: disabled spans inject nothing, an empty
    // `tracestate` is omitted, and invalid headers extract nothing.
    fn trace_headers_round_trip() {
        let provider = SdkTracerProvider::builder().build();
        let subscriber = tracing_subscriber::registry()
            .with(tracing_opentelemetry::layer().with_tracer(provider.tracer("http-test")));
        tracing::subscriber::with_default(subscriber, || {
            let span = tracing::info_span!("request");
            let headers: HashMap<_, _> = trace_headers(&span).collect();
            assert_eq!(headers.len(), 1, "only a traceparent: {headers:?}");
            let extracted = extract_context(|name| headers.get(name).map(String::as_str));
            let (extracted, original) = (
                extracted.span().span_context().clone(),
                span.context().span().span_context().clone(),
            );
            assert_eq!(extracted.trace_id(), original.trace_id());
            assert_eq!(extracted.span_id(), original.span_id());

            assert_eq!(trace_headers(&Span::none()).count(), 0);
            let invalid = extract_context(|_| Some("invalid"));
            assert!(!invalid.span().span_context().is_valid());
        });
    }

    #[test]
    fn requests_are_recorded_once_on_drop() {
        static METRICS: std::sync::LazyLock<HttpMetrics> =
            std::sync::LazyLock::new(|| HttpMetrics::register("http_test"));
        let observation = METRICS.start(PUBLIC_DECRYPTION_ROUTE);
        assert_eq!(METRICS.in_flight.get(), 1);
        drop(observation);
        assert_eq!(METRICS.in_flight.get(), 0);
        assert_eq!(
            METRICS
                .requests
                .with_label_values(&[PUBLIC_DECRYPTION_ROUTE, "0", "cancelled"])
                .get(),
            1
        );
        let mut completed = METRICS.start(PUBLIC_DECRYPTION_ROUTE);
        completed.finish(401, "sender_authentication_failed", &Span::none());
        drop(completed);
        assert_eq!(
            METRICS
                .requests
                .with_label_values(&[
                    PUBLIC_DECRYPTION_ROUTE,
                    "401",
                    "sender_authentication_failed"
                ])
                .get(),
            1
        );
        assert_eq!(
            METRICS
                .duration
                .with_label_values(&[PUBLIC_DECRYPTION_ROUTE])
                .get_sample_count(),
            2
        );
        assert_eq!(route_label("/attacker/controlled/path"), UNMATCHED_ROUTE);
    }
}
