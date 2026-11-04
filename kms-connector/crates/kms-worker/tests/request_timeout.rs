use alloy::primitives::U256;
use connector_utils::types::{KmsGrpcRequest, u256_to_request_id};
use kms_grpc::{
    kms::v1::{KeyGenRequest, PublicDecryptionRequest, UserDecryptionRequest},
    kms_service::v1::{
        core_service_endpoint_client::CoreServiceEndpointClient,
        core_service_endpoint_server::SERVICE_NAME,
    },
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

/// A responsive HTTP/2 connection whose RPC handler never replies on the first attempt.
#[derive(Clone)]
struct StalledCore {
    calls: Arc<AtomicUsize>,
    reply_on_retry: bool,
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
        let attempt = self.calls.fetch_add(1, Ordering::SeqCst);
        let reply = self.reply_on_retry && attempt > 0;
        Box::pin(async move {
            if reply {
                Ok(tonic::Status::already_exists("accepted on first attempt").into_http())
            } else {
                pending().await
            }
        })
    }
}

async fn start_core(
    reply_on_retry: bool,
) -> (Channel, Arc<AtomicUsize>, tokio::task::JoinHandle<()>) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("http://{}", listener.local_addr().unwrap());
    let calls = Arc::new(AtomicUsize::new(0));
    let service = StalledCore {
        calls: calls.clone(),
        reply_on_retry,
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

// A stalled RPC on a live connection hits the endpoint timeout and returns Cancelled.
#[tokio::test]
async fn endpoint_timeout_returns_cancelled() {
    let (channel, calls, server) = start_core(false).await;
    let mut client = CoreServiceEndpointClient::new(channel);
    let result = tokio::time::timeout(
        Duration::from_secs(3),
        client.get_public_decryption_result(u256_to_request_id(U256::from(1))),
    )
    .await
    .expect("endpoint timeout did not fire");
    server.abort();
    assert_eq!(result.unwrap_err().code(), Code::Cancelled);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

// Decryption and keygen submissions retry a timeout and treat AlreadyExists as success.
#[rstest::rstest]
#[case::public_decryption(0)]
#[case::user_decryption(1)]
#[case::key_generation(2)]
#[tokio::test]
async fn submissions_retry_timeout_then_accept_already_exists(#[case] kind: u8) {
    let (channel, calls, server) = start_core(true).await;
    let client = KmsClient::new(vec![channel], 2);
    let id = Some(u256_to_request_id(U256::from(1)));
    let request = match kind {
        0 => KmsGrpcRequest::PublicDecryption(PublicDecryptionRequest {
            request_id: id,
            ..Default::default()
        }),
        1 => KmsGrpcRequest::UserDecryption(UserDecryptionRequest {
            request_id: id,
            ..Default::default()
        }),
        _ => KmsGrpcRequest::Keygen(KeyGenRequest {
            preproc_id: id,
            ..Default::default()
        }),
    };
    let (errors, result) =
        tokio::time::timeout(Duration::from_secs(5), client.send_request(&request))
            .await
            .expect("submission retries did not finish");
    server.abort();
    assert!(result.is_ok(), "{result:?}");
    assert_eq!(errors, 1);
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}

// Submissions and result polls stop with an error after exhausting their timeout retries.
#[rstest::rstest]
#[case::submission(false)]
#[case::polling(true)]
#[tokio::test]
async fn calls_stop_after_timeout_retry_limit(#[case] polling: bool) {
    let (channel, calls, server) = start_core(false).await;
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
