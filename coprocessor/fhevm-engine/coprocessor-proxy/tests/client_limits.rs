//! End-to-end tests of the proxy's client limits: the connection limits, the first request head's
//! deadline and the connections it keeps.

mod common;

use common::{FakeRpc, KmsContext, StubPrfService, TestProxy, prf_path, read_one_response, sender};
use coprocessor_proxy::core::KEEPALIVE_REQUEST_LIMIT;
use std::{
    num::{NonZeroU32, NonZeroUsize},
    time::{Duration, Instant},
};

#[tokio::test]
async fn test_closes_client_connections_idle_between_requests() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let idle_timeout = Duration::from_secs(2);
    let t = TestProxy::start_with_config(StubPrfService::start().await, |config| {
        config.client_idle_timeout = idle_timeout;
    })
    .await;
    let mut stream = t.tls.connect(t.addr).await;

    // Two authenticated requests on the same connection.
    let request = t.signed_get_head(&prf_path(1), "").await;
    for _ in 0..2 {
        stream.write_all(request.as_bytes()).await.unwrap();
        let response = read_one_response(&mut stream).await;
        assert!(response.starts_with("HTTP/1.1 200"), "{response}");
    }

    let idle = Instant::now();
    let mut byte = [0u8; 1];
    let read = tokio::time::timeout(Duration::from_secs(10), stream.read(&mut byte)).await;
    // The server closes, without a TLS `close_notify` or with one.
    assert!(matches!(read, Ok(Ok(0) | Err(_))), "{read:?}");
    let closed_after = idle.elapsed();
    assert!(
        closed_after >= idle_timeout - Duration::from_millis(500)
            && closed_after < idle_timeout + Duration::from_secs(2),
        "{closed_after:?}"
    );
}

#[tokio::test]
async fn test_closes_a_client_connection_after_its_request_limit() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let t = TestProxy::start_with_config(StubPrfService::start().await, |config| {
        config.max_requests_per_signer = NonZeroU32::new(KEEPALIVE_REQUEST_LIMIT * 10).unwrap();
    })
    .await;
    let mut stream = t.tls.connect(t.addr).await;
    let request = t.signed_get_head(&prf_path(1), "").await;
    for served in 1..=KEEPALIVE_REQUEST_LIMIT {
        stream.write_all(request.as_bytes()).await.unwrap();
        let response = read_one_response(&mut stream).await;
        assert!(response.starts_with("HTTP/1.1 200"), "{served}: {response}");
        let closing = response.to_ascii_lowercase().contains("connection: close");
        assert_eq!(
            closing,
            served == KEEPALIVE_REQUEST_LIMIT,
            "{served}: {response}"
        );
    }
    let mut byte = [0u8; 1];
    let read = tokio::time::timeout(Duration::from_secs(5), stream.read(&mut byte)).await;
    assert!(matches!(read, Ok(Ok(0) | Err(_))), "{read:?}");
}

#[tokio::test]
async fn test_closes_a_connection_whose_request_head_misses_the_deadline() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let t = TestProxy::start_with_config(StubPrfService::start().await, |config| {
        config.request_head_timeout = Duration::from_secs(1);
    })
    .await;
    let stream = t.tls.connect(t.addr).await;
    let (mut reader, mut writer) = tokio::io::split(stream);
    let started = Instant::now();
    // One byte every 200 ms: Pingora alone waits up to 60 s for each read.
    let head = format!("GET / HTTP/1.1\r\nHost: {}\r\n\r\n", t.addr);
    let dripping = tokio::spawn(async move {
        for byte in head.as_bytes() {
            if writer.write_all(&[*byte]).await.is_err() {
                return;
            }
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    });

    let mut byte = [0u8; 1];
    let read = tokio::time::timeout(Duration::from_secs(5), reader.read(&mut byte)).await;
    assert!(matches!(read, Ok(Ok(0) | Err(_))), "{read:?}");
    let closed_after = started.elapsed();
    assert!(
        closed_after >= Duration::from_millis(900) && closed_after < Duration::from_secs(3),
        "{closed_after:?}"
    );
    dripping.abort();
}

#[tokio::test]
async fn test_serves_a_request_held_longer_than_the_head_deadline() {
    use tokio::io::AsyncWriteExt;

    let head_timeout = Duration::from_secs(1);
    let t = TestProxy::start_with_config(StubPrfService::start().await, |config| {
        config.request_head_timeout = head_timeout;
    })
    .await;
    let held = head_timeout * 3;
    let mut stream = t.tls.connect(t.addr).await;
    let delay = format!("X-Stub-Delay-Ms: {}\r\n", held.as_millis());
    let request = t.signed_get_head(&prf_path(1), &delay).await;
    stream.write_all(request.as_bytes()).await.unwrap();
    let response = tokio::time::timeout(
        held + Duration::from_secs(10),
        read_one_response(&mut stream),
    )
    .await
    .unwrap();
    assert!(response.starts_with("HTTP/1.1 200"), "{response}");
}

#[tokio::test]
async fn test_closes_the_connection_after_any_answer_before_authentication() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    // Kept, the connection would no longer have a head deadline.
    let t = TestProxy::start(StubPrfService::start().await).await;
    let addr = t.addr;
    let path = prf_path(1);
    for (request, status) in [
        (format!("GET / HTTP/1.1\r\nHost: {addr}\r\n\r\n"), "404"),
        (
            format!("POST {path} HTTP/1.1\r\nHost: {addr}\r\n\r\n"),
            "405",
        ),
        (
            format!("GET {path} HTTP/1.1\r\nHost: {addr}\r\nContent-Length: 1\r\n\r\nx"),
            "401",
        ),
        (
            format!("GET {path} HTTP/1.1\r\nHost: {addr}\r\n\r\n"),
            "401",
        ),
        (
            format!("GET {path} HTTP/1.1\r\nHost: {addr}\r\nAuthorization: Bearer x\r\n\r\n"),
            "401",
        ),
    ] {
        let mut stream = t.tls.connect(addr).await;
        stream.write_all(request.as_bytes()).await.unwrap();
        let mut response = Vec::new();
        let read =
            tokio::time::timeout(Duration::from_secs(5), stream.read_to_end(&mut response)).await;
        let response = String::from_utf8_lossy(&response);
        assert!(read.is_ok(), "still open after: {response}");
        assert!(
            response.starts_with(&format!("HTTP/1.1 {status}")),
            "{response}"
        );
    }
}

#[tokio::test]
async fn test_refuses_connections_before_tls_while_the_replica_holds_its_limit() {
    use tokio::io::AsyncWriteExt;

    let t = TestProxy::start_with_config(StubPrfService::start().await, |config| {
        config.max_connections = NonZeroUsize::new(2).unwrap();
    })
    .await;
    let request = t.signed_get_head(&prf_path(1), "").await;
    // An authenticated request on each, so the proxy holds both past their handshake.
    let mut held = Vec::new();
    for _ in 0..2 {
        let mut stream = connect_when_admitted(&t).await;
        stream.write_all(request.as_bytes()).await.unwrap();
        let response = read_one_response(&mut stream).await;
        assert!(response.starts_with("HTTP/1.1 200"), "{response}");
        held.push(stream);
    }

    // The proxy accepts the TCP connection, then drops it before the TLS handshake.
    for _ in 0..3 {
        assert!(t.tls.try_connect(t.addr).await.is_err());
    }
    let refused = refused_connections("max_connections");
    assert!(refused >= 3.0, "{refused}");

    // The held connections still serve, and closing one frees its place.
    let last = held.last_mut().unwrap();
    last.write_all(request.as_bytes()).await.unwrap();
    let response = read_one_response(last).await;
    assert!(response.starts_with("HTTP/1.1 200"), "{response}");
    held.pop();
    let mut stream = connect_when_admitted(&t).await;
    stream.write_all(request.as_bytes()).await.unwrap();
    let response = read_one_response(&mut stream).await;
    assert!(response.starts_with("HTTP/1.1 200"), "{response}");
}

#[tokio::test]
async fn test_caps_the_connections_waiting_for_their_first_head() {
    use tokio::io::AsyncReadExt;

    // 5 new connections per second and 1 s for the first head: at most 5 wait at once.
    let rpc = FakeRpc::start(vec![KmsContext::active(&[sender().address()])]).await;
    let t = TestProxy::launch(rpc, StubPrfService::start().await, |config| {
        config.max_new_connections_per_second = NonZeroU32::new(5).unwrap();
        config.request_head_timeout = Duration::from_secs(1);
    })
    .await;
    // The startup connection leaves the rate's 1 s window.
    tokio::time::sleep(Duration::from_secs(2)).await;
    let refused_before = refused_connections("first_heads");

    // Accepted within the rate, with their handshakes held back.
    let mut tcp = Vec::new();
    for _ in 0..12 {
        tcp.push(tokio::net::TcpStream::connect(t.addr).await.unwrap());
        tokio::time::sleep(Duration::from_millis(300)).await;
    }
    // The handshakes complete together; a connection the proxy keeps waits for its first head,
    // and stays open past 500 ms.
    let still_open = tcp.into_iter().map(|tcp| async {
        let Ok(mut stream) = t.tls.handshake(tcp).await else {
            return false;
        };
        let mut byte = [0u8; 1];
        let read = stream.read(&mut byte);
        tokio::time::timeout(Duration::from_millis(500), read)
            .await
            .is_err()
    });
    let waiting = futures_util::future::join_all(still_open)
        .await
        .into_iter()
        .filter(|&open| open)
        .count();
    assert_eq!(waiting, 5);
    assert_eq!(refused_connections("first_heads") - refused_before, 7.0);
}

/// The connections the proxies of this test binary refused for `reason`.
fn refused_connections(reason: &str) -> f64 {
    prometheus::gather()
        .iter()
        .filter(|family| family.name() == "coprocessor_proxy_connections_refused_counter")
        .flat_map(|family| family.get_metric())
        .filter(|metric| metric.get_label()[0].value() == reason)
        .map(|metric| metric.get_counter().value())
        .sum()
}

/// Opens a TLS connection once the proxy admits one: it counts a connection out only after it
/// noticed its close.
async fn connect_when_admitted(
    t: &TestProxy,
) -> tokio_rustls::client::TlsStream<tokio::net::TcpStream> {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        match t.tls.try_connect(t.addr).await {
            Ok(stream) => return stream,
            Err(e) => assert!(Instant::now() < deadline, "no connection admitted: {e}"),
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}
