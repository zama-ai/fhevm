use kms_worker::core::{Config, event_processor::KmsClient};
use std::{
    net::SocketAddr,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use tonic::transport::{Channel, Server, server::TcpIncoming};
use tonic_health::pb::{HealthCheckRequest, health_client::HealthClient};

/// Upper bound for a request sent on a black-holed connection to fail.
///
/// Keepalive should fail the request after ~15s (10s ping interval + 5s ping timeout). Without
/// keepalive, the request hangs forever.
const BLACK_HOLE_DETECTION_TIMEOUT: Duration = Duration::from_secs(30);

/// TCP proxy that can silently drop all traffic while keeping its connections open.
///
/// Reproduces a KMS Core pod disappearing without closing its connections (e.g. node preemption):
/// the TCP layer stays alive, but nothing reaches the other side anymore.
struct BlackHoleProxy {
    addr: SocketAddr,
    frozen: Arc<AtomicBool>,
}

impl BlackHoleProxy {
    async fn start(upstream: SocketAddr) -> anyhow::Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;
        let frozen = Arc::new(AtomicBool::new(false));

        let proxy_frozen = frozen.clone();
        tokio::spawn(async move {
            while let Ok((client, _)) = listener.accept().await {
                let Ok(server) = TcpStream::connect(upstream).await else {
                    continue;
                };
                let (client_read, client_write) = client.into_split();
                let (server_read, server_write) = server.into_split();
                tokio::spawn(forward(client_read, server_write, proxy_frozen.clone()));
                tokio::spawn(forward(server_read, client_write, proxy_frozen.clone()));
            }
        });

        Ok(Self { addr, frozen })
    }

    fn url(&self) -> String {
        format!("http://{}", self.addr)
    }

    fn set_frozen(&self, frozen: bool) {
        self.frozen.store(frozen, Ordering::SeqCst);
    }
}

/// Forwards bytes from `reader` to `writer`, discarding them while `frozen` is set.
async fn forward(
    mut reader: impl AsyncReadExt + Unpin,
    mut writer: impl AsyncWriteExt + Unpin,
    frozen: Arc<AtomicBool>,
) {
    let mut buf = [0u8; 8192];
    loop {
        let n = match reader.read(&mut buf).await {
            Ok(0) | Err(_) => return,
            Ok(n) => n,
        };
        if !frozen.load(Ordering::SeqCst) && writer.write_all(&buf[..n]).await.is_err() {
            return;
        }
    }
}

/// Starts a GRPC server only serving the health service, as a stand-in for the KMS Core.
async fn start_grpc_server() -> anyhow::Result<SocketAddr> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;
    let (_, health_service) = tonic_health::server::health_reporter();
    tokio::spawn(
        Server::builder()
            .add_service(health_service)
            .serve_with_incoming(TcpIncoming::from(listener)),
    );
    Ok(addr)
}

async fn check(client: &mut HealthClient<Channel>) -> Result<(), tonic::Status> {
    client
        .check(HealthCheckRequest::default())
        .await
        .map(|_| ())
}

#[tokio::test]
async fn test_black_holed_connection_is_detected() -> anyhow::Result<()> {
    let server_addr = start_grpc_server().await?;
    let proxy = BlackHoleProxy::start(server_addr).await?;

    let channel = KmsClient::connect_single_shard(0, &proxy.url(), &Config::default()).await?;
    let mut client = HealthClient::new(channel);
    check(&mut client).await?;

    // The KMS Core "disappears": the request must fail instead of hanging
    proxy.set_frozen(true);
    let result = tokio::time::timeout(BLACK_HOLE_DETECTION_TIMEOUT, check(&mut client)).await;
    match result {
        Err(_) => anyhow::bail!(
            "Request on a black-holed connection still pending after {BLACK_HOLE_DETECTION_TIMEOUT:?}"
        ),
        Ok(Ok(())) => anyhow::bail!("Request on a black-holed connection unexpectedly succeeded"),
        Ok(Err(status)) => println!("Black-holed request failed as expected: {status}"),
    }

    // The KMS Core is "back": the channel must reconnect on its own
    proxy.set_frozen(false);
    let mut last_error = None;
    for _ in 0..10 {
        match tokio::time::timeout(Duration::from_secs(5), check(&mut client)).await {
            Ok(Ok(())) => return Ok(()),
            Ok(Err(status)) => last_error = Some(status.to_string()),
            Err(_) => last_error = Some("request timed out".to_string()),
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
    anyhow::bail!("Channel did not reconnect after unfreezing: {last_error:?}")
}
