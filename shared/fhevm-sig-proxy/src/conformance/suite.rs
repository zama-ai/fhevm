//! The criteria every deployment's proxy binary meets, one `pub async fn` each, run by
//! [`conformance_tests!`](crate::conformance_tests) and
//! [`protocol_config_signer_tests!`](crate::protocol_config_signer_tests).

use super::{
    AUDIENCE, Canned, ENDPOINT_RESPONSE_TIMEOUT, KmsContext, Profile, ProxyProcess,
    SIGNER_REFRESH_INTERVAL, Setup, authorization, sender, signed_by, stranger,
};
use crate::ProxyError;
use alloy::{primitives::Address, transports::http::reqwest};
use bytes::Bytes;
use fhevm_sig_auth::{AuthError, unix_now};
use http::{
    Method, StatusCode,
    header::{ALLOW, CONTENT_TYPE, WWW_AUTHENTICATE},
};
use std::time::{Duration, Instant};

/// Expands to one `#[tokio::test]` per criterion every deployment meets, for the [`Profile`]
/// expression given.
#[macro_export]
macro_rules! conformance_tests {
    ($profile:expr) => {
        $crate::conformance_tests!(@each $profile;
            an_accepted_request_reaches_the_endpoint_unchanged,
            a_failed_authentication_never_reaches_the_endpoint,
            a_routing_error_never_reaches_the_endpoint,
            a_dot_segment_never_reaches_the_endpoint,
            the_endpoint_rate_limit_reaches_the_client_unchanged,
            an_unreachable_endpoint_is_a_502_and_not_ready,
            a_slow_endpoint_is_a_504,
            client_forwarding_headers_never_reach_the_endpoint,
            each_request_is_logged_and_counted,
        );
    };
    (@each $profile:expr; $($criterion:ident,)*) => {
        $(
            #[tokio::test(flavor = "multi_thread")]
            async fn $criterion() {
                $crate::conformance::suite::$criterion(&$profile).await;
            }
        )*
    };
}

/// Expands to one `#[tokio::test]` per criterion of a deployment reading its signers from
/// `ProtocolConfig`, for the [`Profile`] expression given.
#[macro_export]
macro_rules! protocol_config_signer_tests {
    ($profile:expr) => {
        $crate::conformance_tests!(@each $profile;
            not_ready_and_refusing_before_the_first_signer_read,
            a_removed_signer_is_refused_after_a_refresh,
            an_rpc_failure_keeps_the_last_signers,
        );
    };
}

async fn started(profile: &Profile) -> ProxyProcess {
    let proxy = ProxyProcess::start(profile, Setup::default()).await;
    proxy.wait_for_healthz(StatusCode::OK).await;
    proxy
}

/// Asserts `response` is the deployment's rendering of `error`: its status and body, a
/// `Content-Type` exactly when there is a body, `WWW-Authenticate` exactly on a `401`, and `Allow`
/// on a `405`.
async fn assert_error(profile: &Profile, response: reqwest::Response, error: ProxyError) {
    let expected = (profile.error_body)(&error);
    let headers = response.headers();
    assert_eq!(response.status(), expected.status, "{error:?}");
    assert_eq!(
        headers.get(CONTENT_TYPE).map(|v| v.to_str().unwrap()),
        (!expected.json.is_empty()).then_some("application/json"),
        "{error:?}"
    );
    assert_eq!(
        headers.contains_key(WWW_AUTHENTICATE),
        expected.status == StatusCode::UNAUTHORIZED,
        "{error:?}"
    );
    if let ProxyError::MethodNotAllowed(methods) = &error {
        let methods: Vec<_> = methods.iter().map(Method::as_str).collect();
        assert_eq!(headers[ALLOW], methods.join(", "), "{error:?}");
    }
    assert_eq!(response.bytes().await.unwrap(), expected.json, "{error:?}");
}

/// The endpoint receives the method, target and body unchanged, `Authorization` only where the
/// deployment forwards it, and the client receives the endpoint's response unchanged.
pub async fn an_accepted_request_reaches_the_endpoint_unchanged(profile: &Profile) {
    let proxy = started(profile).await;
    let body = Bytes::from(vec![b'p'; 4096]);
    proxy.stub.answer(Canned {
        status: StatusCode::OK,
        headers: vec![("x-endpoint", "kept")],
        body: body.clone(),
    });
    let route = &profile.route;
    let authorization = authorization(route.path, route.body).await;

    let response = proxy
        .send(
            route.method.clone(),
            route.path,
            Some(&authorization),
            route.body,
        )
        .await;

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()["x-endpoint"], "kept");
    assert_eq!(response.bytes().await.unwrap(), body);
    let requests = proxy.stub.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, route.method);
    assert_eq!(requests[0].target, route.path);
    assert_eq!(requests[0].body, route.body);
    let forwarded = requests[0].headers.get("authorization");
    match profile.forwards_authorization {
        true => assert_eq!(forwarded.unwrap(), authorization.as_str()),
        false => assert_eq!(forwarded, None),
    }
}

/// Each failed `FhevmSig` check is a `401` with `WWW-Authenticate`, and no whole request reaches
/// the endpoint; a check failing before the body is read sends it nothing at all.
pub async fn a_failed_authentication_never_reaches_the_endpoint(profile: &Profile) {
    let proxy = started(profile).await;
    let route = &profile.route;
    let (path, body) = (route.path, route.body);
    let send = |authorization: Option<String>, path: String, body: &'static [u8]| {
        let proxy = &proxy;
        async move {
            proxy
                .send(route.method.clone(), &path, authorization.as_deref(), body)
                .await
        }
    };
    let expired = signed_by(&sender(), AUDIENCE, path, body, unix_now() - 3600).await;
    let stranger_signed = signed_by(&stranger(), AUDIENCE, path, body, unix_now() + 60).await;
    let with_query = format!("{path}?x=1");

    for (response, error) in [
        (send(None, path.into(), body).await, AuthError::Missing),
        (
            send(Some(expired), path.into(), body).await,
            AuthError::Expired,
        ),
        (
            send(Some(stranger_signed), path.into(), body).await,
            AuthError::UnknownSigner(stranger().address()),
        ),
        (
            send(Some(authorization(path, body).await), with_query, body).await,
            AuthError::QueryString,
        ),
    ] {
        assert_error(profile, response, ProxyError::Auth(error)).await;
    }
    for authorization in [
        signed_by(
            &sender(),
            Address::repeat_byte(0xa2),
            path,
            body,
            unix_now() + 60,
        )
        .await,
        "FhevmSig not-a-header".to_owned(),
    ] {
        let response = send(Some(authorization), path.into(), body).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        assert!(response.headers().contains_key(WWW_AUTHENTICATE));
    }
    assert_eq!(proxy.stub.heads(), Vec::<String>::new());

    // A body other than the signed one: refused once the body is read, so the endpoint may have
    // received the head but never a whole request.
    let other_body = [body, b"x"].concat();
    let response = send(
        Some(authorization(path, &other_body).await),
        path.into(),
        body,
    )
    .await;
    assert_error(profile, response, ProxyError::Auth(AuthError::BodyMismatch)).await;
    assert_eq!(proxy.stub.requests().len(), 0);
}

/// An unknown path is a `404`, a refused method a `405` with `Allow`, each rendered by the
/// deployment, and neither reaches the endpoint.
pub async fn a_routing_error_never_reaches_the_endpoint(profile: &Profile) {
    let proxy = started(profile).await;
    let route = &profile.route;

    let unknown = "/not-a-route";
    let response = proxy
        .send(
            route.method.clone(),
            unknown,
            Some(&authorization(unknown, route.body).await),
            route.body,
        )
        .await;
    assert_error(profile, response, ProxyError::NotFound).await;

    let response = proxy
        .send(
            route.other_method.clone(),
            route.path,
            Some(&authorization(route.path, b"").await),
            b"",
        )
        .await;
    assert_error(
        profile,
        response,
        ProxyError::MethodNotAllowed(route.methods.to_vec()),
    )
    .await;

    assert_eq!(proxy.stub.heads(), Vec::<String>::new());
}

/// A signed request with a `.` or `..` path segment, anywhere and as any parameter, is the
/// deployment's `404` and never reaches the endpoint, which could resolve it to another resource
/// than its route's.
pub async fn a_dot_segment_never_reaches_the_endpoint(profile: &Profile) {
    let proxy = started(profile).await;
    let route = &profile.route;
    let segments: Vec<&str> = route.path.split('/').collect();
    let mut paths = vec![format!("{}/x/..", route.path)];
    for (index, template) in route.template.split('/').enumerate() {
        if template.starts_with('{') {
            for dots in [".", ".."] {
                let mut path = segments.clone();
                path[index] = dots;
                paths.push(path.join("/"));
            }
        }
    }

    let expected = (profile.error_body)(&ProxyError::NotFound);
    for path in &paths {
        let authorization = authorization(path, route.body).await;
        let response = proxy
            .send_raw(&route.method, path, &authorization, route.body)
            .await;
        assert_eq!(response, (expected.status, expected.json.clone()), "{path}");
    }
    assert_eq!(proxy.stub.heads(), Vec::<String>::new());
}

/// The endpoint's `429`, its `Retry-After` and body reach the client unchanged.
pub async fn the_endpoint_rate_limit_reaches_the_client_unchanged(profile: &Profile) {
    let proxy = started(profile).await;
    let body = Bytes::from_static(br#"{"code":"rate_limited"}"#);
    proxy.stub.answer(Canned {
        status: StatusCode::TOO_MANY_REQUESTS,
        headers: vec![("retry-after", "7")],
        body: body.clone(),
    });

    let response = proxy.signed(&profile.route).await;

    assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(response.headers()["retry-after"], "7");
    assert_eq!(response.bytes().await.unwrap(), body);
}

/// With its only endpoint unreachable, the proxy is not ready and answers a signed request `502`.
pub async fn an_unreachable_endpoint_is_a_502_and_not_ready(profile: &Profile) {
    let setup = Setup {
        endpoint_up: false,
        ..Setup::default()
    };
    let proxy = ProxyProcess::start(profile, setup).await;
    proxy
        .wait_for_healthz(StatusCode::SERVICE_UNAVAILABLE)
        .await;

    let response = proxy.signed(&profile.route).await;

    assert_error(profile, response, ProxyError::EndpointUnavailable).await;
}

/// An endpoint answering after the route's response timeout is a `504`.
pub async fn a_slow_endpoint_is_a_504(profile: &Profile) {
    let proxy = started(profile).await;
    proxy.stub.hold(ENDPOINT_RESPONSE_TIMEOUT * 2);

    let started_at = Instant::now();
    let response = proxy.signed(&profile.route).await;

    assert!(started_at.elapsed() < ENDPOINT_RESPONSE_TIMEOUT * 2);
    assert_error(profile, response, ProxyError::EndpointTimeout).await;
}

/// The endpoint receives the forwarding headers the proxy sets, never the client's.
pub async fn client_forwarding_headers_never_reach_the_endpoint(profile: &Profile) {
    let proxy = started(profile).await;
    let route = &profile.route;
    let mut request = proxy
        .client
        .request(route.method.clone(), proxy.url(route.path))
        .header("authorization", authorization(route.path, route.body).await)
        .body(route.body);
    for (name, value) in [
        ("x-forwarded-for", "203.0.113.7"),
        ("x-forwarded-proto", "http"),
        ("x-forwarded-host", "evil.example"),
        ("forwarded", "for=203.0.113.7"),
        ("via", "1.1 evil"),
    ] {
        request = request.header(name, value);
    }

    assert_eq!(request.send().await.unwrap().status(), StatusCode::OK);

    let headers = &proxy.stub.requests()[0].headers;
    let values = |name| -> Vec<_> { headers.get_all(name).iter().collect() };
    assert_eq!(values("x-forwarded-for"), ["127.0.0.1"]);
    assert_eq!(values("x-forwarded-proto"), ["https"]);
    for name in ["x-forwarded-host", "forwarded", "via"] {
        assert_eq!(values(name), Vec::<&http::HeaderValue>::new(), "{name}");
    }
}

/// Each request has an access log line and is counted in `fhevm_sig_proxy_requests_total` by
/// route template and status.
pub async fn each_request_is_logged_and_counted(profile: &Profile) {
    let proxy = started(profile).await;
    let route = &profile.route;
    // From here on nothing else reaches the TLS listener: the self-check runs on `/healthz` only.
    let counted = |metrics: &str| {
        [(route.template, 200), (route.template, 401), ("none", 404)]
            .map(|(route, status)| requests_total(metrics, route, status))
    };
    let before = counted(&proxy.metrics().await);

    let unknown = "/not-a-route";
    let authorization = authorization(route.path, route.body).await;
    let statuses = [
        proxy
            .send(
                route.method.clone(),
                route.path,
                Some(&authorization),
                route.body,
            )
            .await
            .status(),
        proxy
            .send(route.method.clone(), route.path, None, route.body)
            .await
            .status(),
        proxy
            .send(route.method.clone(), unknown, None, route.body)
            .await
            .status(),
    ];
    assert_eq!(statuses, [200, 401, 404]);

    // The count and the line are written once the response is sent.
    let expected = before.map(|count| count + 1);
    eventually(|| async { counted(&proxy.metrics().await) == expected }).await;
    let sender = sender().address().to_string();
    for (path, template, status, signer) in [
        (route.path, route.template, 200, sender.as_str()),
        (route.path, route.template, 401, ""),
        (unknown, "none", 404, ""),
    ] {
        eventually(|| async {
            proxy.output().lines().any(|line| {
                line.contains("Proxied request")
                    && has_field(line, "path", path)
                    && has_field(line, "route", template)
                    && has_field(line, "status", &status.to_string())
                    && has_field(line, "sender", signer)
            })
        })
        .await;
    }
    assert!(!proxy.output().contains(&authorization));
}

/// Whether a log line, in text (`name=value`, `name="value"`) or JSON, has the field `name` equal
/// to `value`.
fn has_field(line: &str, name: &str, value: &str) -> bool {
    [
        format!(" {name}={value} "),
        format!(" {name}=\"{value}\""),
        format!("\"{name}\":{value},"),
        format!("\"{name}\":\"{value}\""),
    ]
    .iter()
    .any(|field| format!("{line} ").contains(field.as_str()))
}

/// Before `ProtocolConfig` was first read the proxy is not ready and refuses every signer; it
/// is ready once a read succeeds.
pub async fn not_ready_and_refusing_before_the_first_signer_read(profile: &Profile) {
    let setup = Setup {
        rpc_failing: true,
        ..Setup::default()
    };
    let proxy = ProxyProcess::start(profile, setup).await;
    assert_eq!(proxy.healthz().await, StatusCode::SERVICE_UNAVAILABLE);

    let response = proxy.signed(&profile.route).await;
    assert_error(
        profile,
        response,
        ProxyError::Auth(AuthError::UnknownSigner(sender().address())),
    )
    .await;
    assert_eq!(proxy.stub.heads(), Vec::<String>::new());

    proxy.rpc.set_failing(false);
    proxy.wait_for_healthz(StatusCode::OK).await;
    assert_eq!(proxy.signed(&profile.route).await.status(), StatusCode::OK);
}

/// A tx-sender whose KMS context is no longer Active is refused once the signers are read again,
/// and the tx-senders of the new Active context are accepted.
pub async fn a_removed_signer_is_refused_after_a_refresh(profile: &Profile) {
    let proxy = started(profile).await;
    let route = &profile.route;
    assert_eq!(proxy.signed(route).await.status(), StatusCode::OK);

    proxy.rpc.set_contexts(vec![
        KmsContext {
            active: false,
            tx_senders: vec![sender().address()],
        },
        KmsContext {
            active: true,
            tx_senders: vec![stranger().address()],
        },
    ]);

    eventually(|| async { proxy.signed(route).await.status() == StatusCode::UNAUTHORIZED }).await;
    let stranger_signed = signed_by(
        &stranger(),
        AUDIENCE,
        route.path,
        route.body,
        unix_now() + 60,
    )
    .await;
    let response = proxy
        .send(
            route.method.clone(),
            route.path,
            Some(&stranger_signed),
            route.body,
        )
        .await;
    assert_eq!(response.status(), StatusCode::OK);
}

/// When the RPC fails, the proxy keeps the signers it last read and stays ready.
pub async fn an_rpc_failure_keeps_the_last_signers(profile: &Profile) {
    let proxy = started(profile).await;

    proxy.rpc.set_failing(true);
    tokio::time::sleep(SIGNER_REFRESH_INTERVAL * 5).await;

    assert_eq!(proxy.healthz().await, StatusCode::OK);
    assert_eq!(proxy.signed(&profile.route).await.status(), StatusCode::OK);
}

/// The value of `fhevm_sig_proxy_requests_total{route, status}` in Prometheus text, `0` when the
/// series does not exist yet.
fn requests_total(metrics: &str, route: &str, status: u16) -> u64 {
    let labels = [format!("route=\"{route}\""), format!("status=\"{status}\"")];
    metrics
        .lines()
        .filter(|line| line.starts_with("fhevm_sig_proxy_requests_total{"))
        .find(|line| labels.iter().all(|label| line.contains(label.as_str())))
        .map_or(0, |line| line.rsplit(' ').next().unwrap().parse().unwrap())
}

/// Polls `condition` until it holds, for up to 5 s.
async fn eventually<F: Future<Output = bool>>(condition: impl Fn() -> F) {
    let deadline = Instant::now() + Duration::from_secs(5);
    while !condition().await {
        assert!(Instant::now() < deadline, "condition never held");
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}
