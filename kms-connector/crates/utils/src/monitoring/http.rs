//! HTTP request tracing and metrics shared by the proxy and endpoint.

use kms_connector_api::{PUBLIC_DECRYPTION_ROUTE, USER_DECRYPTION_ROUTE, VERSION_ROUTE};
use opentelemetry::{
    Context,
    propagation::{Extractor, Injector, TextMapPropagator},
    trace::TraceContextExt,
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
/// `route` label values: the API routes, then [`UNMATCHED_ROUTE`].
pub const ROUTES: [&str; 4] = ["public_decrypt", "user_decrypt", "version", UNMATCHED_ROUTE];
/// W3C trace context headers.
pub const TRACEPARENT: &str = "traceparent";
pub const TRACESTATE: &str = "tracestate";
/// Histogram bucket upper bounds in seconds, covering immediate responses up to the proxy's
/// default 60-second endpoint response timeout (the endpoint's default decryption timeout is 30
/// seconds).
pub const DURATION_BUCKETS: &[f64] = &[
    0.01, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0, 20.0, 30.0, 45.0, 60.0,
];

pub fn route_label(path: &str) -> &'static str {
    match path {
        PUBLIC_DECRYPTION_ROUTE => ROUTES[0],
        USER_DECRYPTION_ROUTE => ROUTES[1],
        VERSION_ROUTE => ROUTES[2],
        _ => UNMATCHED_ROUTE,
    }
}

/// The trace context headers of a request: borrowed when extracted, owned when injected.
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

/// The root span of a request, outside any current trace.
fn request_span(service: &'static str, method: &str, path: &str) -> Span {
    // Probes traverse both services. Never create or export a request span for them.
    if method == "GET" && path == VERSION_ROUTE {
        return Span::none();
    }
    tracing::info_span!(parent: None, "http_request", service,
        http.route = route_label(path), http.request.method = method,
        http.response.status_code = tracing::field::Empty, code = tracing::field::Empty)
}

/// Starts a new trace for a request received by the proxy.
///
/// A valid trace supplied by the client is recorded as a related trace. The proxy decides
/// whether to record its own trace independently of the client's sampling flag.
/// Returns a disabled span for `GET /v1/version` health checks.
pub fn proxy_span(method: &str, path: &str, incoming: Context) -> Span {
    let span = request_span("proxy", method, path);
    let remote = incoming.span().span_context().clone();
    if remote.is_valid() {
        span.add_link(remote);
    }
    span
}

/// Creates an endpoint request span with the proxy's span as its parent.
///
/// This puts the endpoint's work in the same trace as the proxy request.
/// Returns a disabled span for `GET /v1/version` so health checks do not produce traces.
pub fn endpoint_span(method: &str, path: &str, incoming: Context) -> Span {
    let span = request_span("endpoint", method, path);
    span.set_parent(incoming);
    span
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
        let requests = register_int_counter_vec!(
            format!("{prefix}_http_requests_counter"),
            "Finished HTTP requests, including cancellations; status 0 means no response",
            &["route", "status", "code"]
        )
        .expect("register HTTP request counter");
        let duration = register_histogram_vec!(
            HistogramOpts::new(
                format!("{prefix}_http_request_duration_seconds"),
                "HTTP request duration in seconds"
            )
            .buckets(DURATION_BUCKETS.to_vec()),
            &["route"]
        )
        .expect("register HTTP duration histogram");
        let in_flight = register_int_gauge!(
            format!("{prefix}_http_requests_in_flight"),
            "Active HTTP requests after header parsing"
        )
        .expect("register HTTP in-flight gauge");
        for route in ROUTES {
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
    use crate::monitoring::otlp::PropagationContext;
    use opentelemetry::trace::{SpanId, TracerProvider};
    use opentelemetry_sdk::trace::{InMemorySpanExporter, SdkTracerProvider};
    use std::collections::HashMap;
    use tracing_subscriber::layer::SubscriberExt;

    #[test]
    // The proxy starts a new trace linked to the client's trace; the endpoint and worker continue it.
    fn request_trace_propagation() {
        let exporter = InMemorySpanExporter::default();
        let provider = SdkTracerProvider::builder()
            .with_simple_exporter(exporter.clone())
            .build();
        let subscriber = tracing_subscriber::registry()
            .with(tracing_opentelemetry::layer().with_tracer(provider.tracer("http-test")));
        // `PropagationContext`, persisted with the request row, uses the global propagator.
        opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());
        tracing::subscriber::with_default(subscriber, || {
            // An unsampled client trace must not stop the proxy from recording its own.
            let client = extract_context(|name| {
                (name == TRACEPARENT)
                    .then_some("00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-00")
            });
            let proxy = proxy_span("POST", PUBLIC_DECRYPTION_ROUTE, client.clone());
            let local = proxy.context();
            assert!(local.span().span_context().is_sampled());
            assert_ne!(
                local.span().span_context().trace_id(),
                client.span().span_context().trace_id()
            );

            let headers: HashMap<_, _> = trace_headers(&proxy).collect();
            assert_eq!(headers.len(), 1, "only a traceparent: {headers:?}");
            let endpoint = endpoint_span(
                "POST",
                PUBLIC_DECRYPTION_ROUTE,
                extract_context(|name| headers.get(name).map(String::as_str)),
            );
            let handler =
                endpoint.in_scope(|| tracing::info_span!("public_decrypt", decryption_id = "test"));
            let persisted =
                bc2wrap::serialize(&PropagationContext::inject(&handler.context())).unwrap();
            let restored: PropagationContext = bc2wrap::deserialize_slice(&persisted).unwrap();
            let worker = tracing::info_span!(parent: None, "handle_event");
            worker.set_parent(restored.extract());
            assert_eq!(
                worker.context().span().span_context().trace_id(),
                local.span().span_context().trace_id()
            );
            drop((worker, handler, endpoint, proxy));

            assert!(proxy_span("GET", VERSION_ROUTE, Context::new()).is_disabled());
            assert!(endpoint_span("GET", VERSION_ROUTE, Context::new()).is_disabled());
            assert_eq!(trace_headers(&Span::none()).count(), 0);
            let first = proxy_span(
                "POST",
                PUBLIC_DECRYPTION_ROUTE,
                extract_context(|_| Some("invalid")),
            );
            let second =
                first.in_scope(|| proxy_span("POST", USER_DECRYPTION_ROUTE, Context::new()));
            assert_ne!(
                first.context().span().span_context().trace_id(),
                second.context().span().span_context().trace_id()
            );
        });
        provider.force_flush().unwrap();
        let spans = exporter.get_finished_spans().unwrap();
        assert_eq!(spans.len(), 6); // One four-span chain and two independent roots; no probes.
        let [worker, handler, endpoint, proxy] = &spans[..4] else {
            unreachable!()
        };
        assert_eq!(worker.parent_span_id, handler.span_context.span_id());
        assert_eq!(handler.parent_span_id, endpoint.span_context.span_id());
        assert_eq!(endpoint.parent_span_id, proxy.span_context.span_id());
        assert_eq!(proxy.parent_span_id, SpanId::INVALID);
        assert_eq!(proxy.links.links.len(), 1);
    }

    #[test]
    fn requests_are_recorded_once_on_drop() {
        static METRICS: std::sync::LazyLock<HttpMetrics> =
            std::sync::LazyLock::new(|| HttpMetrics::register("http_test"));
        let observation = METRICS.start("public_decrypt");
        assert_eq!(METRICS.in_flight.get(), 1);
        drop(observation);
        assert_eq!(METRICS.in_flight.get(), 0);
        assert_eq!(
            METRICS
                .requests
                .with_label_values(&["public_decrypt", "0", "cancelled"])
                .get(),
            1
        );
        let mut completed = METRICS.start("public_decrypt");
        completed.finish(401, "sender_authentication_failed", &Span::none());
        drop(completed);
        assert_eq!(
            METRICS
                .requests
                .with_label_values(&["public_decrypt", "401", "sender_authentication_failed"])
                .get(),
            1
        );
        assert_eq!(
            METRICS
                .duration
                .with_label_values(&["public_decrypt"])
                .get_sample_count(),
            2
        );
        assert_eq!(route_label("/attacker/controlled/path"), UNMATCHED_ROUTE);
    }
}
