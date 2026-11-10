use alloy::primitives::U256;
use connector_utils::types::{KmsGrpcRequest, u256_to_request_id};
use kms_grpc::{
    kms::v1::PublicDecryptionRequest, kms_service::v1::core_service_endpoint_server::SERVICE_NAME,
};
use kms_worker::core::{
    Config,
    event_processor::{KmsClient, KmsPollTarget},
};
use std::{
    convert::Infallible,
    future::{Future, pending},
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    task::{Context, Poll},
    time::Duration,
};
use tonic::{
    Code,
    body::Body,
    codegen::{Service, http},
    server::NamedService,
    transport::{Channel, Server, server::TcpIncoming},
};

/// A responsive HTTP/2 connection whose RPC handlers never reply.
#[derive(Clone)]
struct StalledCore {
    calls: Arc<AtomicUsize>,
}

impl NamedService for StalledCore {
    const NAME: &'static str = SERVICE_NAME;
}

impl Service<http::Request<Body>> for StalledCore {
    type Response = http::Response<Body>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: http::Request<Body>) -> Self::Future {
        self.calls.fetch_add(1, Ordering::SeqCst);
        Box::pin(pending())
    }
}

async fn start_core() -> (Channel, Arc<AtomicUsize>, tokio::task::JoinHandle<()>) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("http://{}", listener.local_addr().unwrap());
    let calls = Arc::new(AtomicUsize::new(0));
    let service = StalledCore {
        calls: calls.clone(),
    };
    let server = tokio::spawn(async move {
        Server::builder()
            .add_service(service)
            .serve_with_incoming(TcpIncoming::from(listener))
            .await
            .unwrap();
    });
    let config = Config {
        kms_req_timeout: Duration::from_millis(100),
        ..Config::default()
    };
    let channel = KmsClient::connect_single_shard(0, &endpoint, &config)
        .await
        .unwrap();
    (channel, calls, server)
}

// Even a dummy method unknown to KmsClient inherits the channel timeout.
#[tokio::test]
async fn dummy_method_inherits_channel_timeout() {
    let (mut channel, calls, server) = start_core().await;
    // Call the transport directly: no generated method or per-call timeout configuration.
    let request = http::Request::post(format!("/{SERVICE_NAME}/Dummy"))
        .header("content-type", "application/grpc")
        .body(Body::empty())
        .unwrap();
    // Watchdog only: a broken channel timeout should fail the test rather than hang it.
    let result = tokio::time::timeout(Duration::from_secs(3), async {
        std::future::poll_fn(|cx| channel.poll_ready(cx)).await?;
        channel.call(request).await
    })
    .await
    .expect("channel timeout did not fire");
    server.abort();
    let status = tonic::Status::from_error(Box::new(result.unwrap_err()));
    assert_eq!(status.code(), Code::Cancelled);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

// Submissions and result polls stop with an error after exhausting their timeout retries.
#[rstest::rstest]
#[case::submission(false)]
#[case::polling(true)]
#[tokio::test]
async fn calls_stop_after_timeout_retry_limit(#[case] polling: bool) {
    let (channel, calls, server) = start_core().await;
    let client = KmsClient::new(vec![channel], 2);
    let (errors, failed) = tokio::time::timeout(Duration::from_secs(5), async {
        let id = u256_to_request_id(U256::from(1));
        if polling {
            let (errors, result) = client
                .poll_result(KmsPollTarget::PublicDecryption(id))
                .await;
            (errors, result.is_err())
        } else {
            let request = KmsGrpcRequest::PublicDecryption(PublicDecryptionRequest {
                request_id: Some(id),
                ..Default::default()
            });
            let (errors, result) = client.send_request(&request).await;
            (errors, result.is_err())
        }
    })
    .await
    .expect("RPC retries did not finish");
    server.abort();
    assert!(failed);
    assert_eq!(errors, 2);
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}
