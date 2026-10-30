use crate::{
    config::{DatabaseUrl, KmsWallet, RpcUrl},
    provider::{FillersWithoutNonceManagement, NonceManagedProvider},
};
use alloy::{
    network::EthereumWallet,
    providers::{
        Identity, ProviderBuilder, ProviderLayer, RootProvider,
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, TxFiller,
            WalletFiller,
        },
    },
    rpc::client::{ClientBuilder, RpcClient},
    transports::{
        IntoBoxTransport, RpcError, TransportErrorKind, TransportFut,
        http::{Client, Http, reqwest},
    },
};
use anyhow::anyhow;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::{
    fmt,
    num::NonZeroUsize,
    sync::{Arc, Once},
    time::Duration,
};
use tower::{
    ServiceBuilder,
    limit::{GlobalConcurrencyLimitLayer, concurrency::future::ResponseFuture},
    util::MapFutureLayer,
};
use tracing::{info, warn};

/// The number of connection retry to connect to the database or to a RPC node.
pub const CONNECTION_RETRY_NUMBER: usize = 5;

/// The delay between two connection attempts.
pub const CONNECTION_RETRY_DELAY: Duration = Duration::from_secs(2);

/// Tries to establish the connection with Postgres database.
pub async fn connect_to_db(
    db_url: &DatabaseUrl,
    db_pool_size: u32,
) -> anyhow::Result<Pool<Postgres>> {
    for i in 1..=CONNECTION_RETRY_NUMBER {
        info!("Attempting connection to DB... ({i}/{CONNECTION_RETRY_NUMBER})");

        let options = PgPoolOptions::new().max_connections(db_pool_size);
        match options.connect_with(db_url.connect_options().clone()).await {
            Ok(db_pool) => {
                info!("Connected to Postgres database successfully");
                return Ok(db_pool);
            }
            Err(e) => warn!("DB connection attempt #{i} failed: {e}"),
        }

        if i != CONNECTION_RETRY_NUMBER {
            tokio::time::sleep(CONNECTION_RETRY_DELAY).await;
        }
    }
    Err(anyhow!("Could not connect to Postgres DB at url {db_url}"))
}

/// The default `Filler`s for an `alloy::Provider`.
type DefaultFillers = JoinFill<
    Identity,
    JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
>;

/// The default `alloy::Provider` used to interact with the Gateway/Host chain.
pub type DefaultProvider = FillProvider<JoinFill<DefaultFillers, ChainIdFiller>, RootProvider>;

/// The default `alloy::Provider` used to interact with the Gateway/Host chain using a wallet.
pub type WalletProvider = NonceManagedProvider<FillProvider<WalletProviderFillers, RootProvider>>;
pub type WalletProviderFillers = JoinFill<
    JoinFill<JoinFill<Identity, ChainIdFiller>, FillersWithoutNonceManagement>,
    WalletFiller<EthereumWallet>,
>;

/// The ceilings of a RPC connection: how long any single call may take, and optionally how many
/// calls it may have in flight.
#[derive(Clone, Copy, Debug)]
struct RpcCallBounds {
    max_concurrent_calls: Option<NonZeroUsize>,
    call_timeout: Duration,
}

/// Tries to establish the connection with a RPC node.
pub async fn connect_to_rpc_node(
    rpc_node_url: RpcUrl,
    chain_id: u64,
) -> anyhow::Result<DefaultProvider> {
    connect_to_rpc_node_inner(rpc_node_url, None, || {
        ProviderBuilder::new().with_chain_id(chain_id)
    })
    .await
}

/// Tries to establish the connection with a RPC node, bounding the duration of each call and, if
/// `max_concurrent_calls` is set, the calls it may have in flight.
pub async fn connect_to_rpc_node_with_bounds(
    rpc_node_url: RpcUrl,
    chain_id: u64,
    max_concurrent_calls: Option<NonZeroUsize>,
    call_timeout: Duration,
) -> anyhow::Result<DefaultProvider> {
    let bounds = RpcCallBounds {
        max_concurrent_calls,
        call_timeout,
    };
    connect_to_rpc_node_inner(rpc_node_url, Some(bounds), || {
        ProviderBuilder::new().with_chain_id(chain_id)
    })
    .await
}

/// Tries to establish the connection with a RPC node, with a `WalletFiller`.
pub async fn connect_to_rpc_node_with_wallet(
    rpc_node_url: RpcUrl,
    chain_id: u64,
    wallet: KmsWallet,
) -> anyhow::Result<WalletProvider> {
    let provider = connect_to_rpc_node_inner(rpc_node_url, None, || {
        ProviderBuilder::new()
            .disable_recommended_fillers()
            .with_chain_id(chain_id)
            .filler(FillersWithoutNonceManagement::default())
            .wallet(wallet.clone())
    })
    .await?;
    Ok(NonceManagedProvider::new(provider, wallet.address()))
}

/// An `RpcClient` which never has more than `max_concurrent_calls` JSON-RPC calls in flight.
fn bounded_rpc_client<T: IntoBoxTransport>(
    transport: T,
    is_local: bool,
    max_concurrent_calls: NonZeroUsize,
) -> RpcClient {
    ClientBuilder::default()
        // Added first, so outermost: `ConcurrencyLimit` has a future type of its own, which has to
        // be boxed back into the `TransportFut` that alloy's `Transport` bound requires.
        .layer(MapFutureLayer::new(
            |fut: ResponseFuture<TransportFut<'static>>| Box::pin(fut) as TransportFut<'static>,
        ))
        .layer(GlobalConcurrencyLimitLayer::new(max_concurrent_calls.get()))
        .transport(transport, is_local)
}

/// Tries to establish the connection with a RPC node.
async fn connect_to_rpc_node_inner<L, F>(
    rpc_node_url: RpcUrl,
    bounds: Option<RpcCallBounds>,
    provider_builder_new: impl Fn() -> ProviderBuilder<L, F>,
) -> anyhow::Result<F::Provider>
where
    L: ProviderLayer<RootProvider>,
    F: ProviderLayer<L::Provider> + TxFiller,
{
    INSTALL_CRYPTO_PROVIDER_ONCE.call_once(|| {
        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .map_err(|e| anyhow!("Failed to install AWS-LC crypto provider: {e:?}"))
            .unwrap()
    });

    let rpc_node_url = Arc::new(rpc_node_url);
    let transport = match bounds {
        Some(bounds) => {
            let http_client = Client::builder()
                .timeout(bounds.call_timeout)
                .build()
                .map_err(|e| anyhow!("Failed to create the RPC HTTP client: {e}"))?;
            Http::with_client(http_client, rpc_node_url.as_url().clone())
        }
        None => Http::new(rpc_node_url.as_url().clone()),
    };
    let is_local = transport.guess_local();
    let call_rpc_url = Arc::clone(&rpc_node_url);
    let transport = ServiceBuilder::new()
        .map_future(move |call| redact_errors(call, Arc::clone(&call_rpc_url)))
        .service(transport);
    let client = match bounds.and_then(|bounds| bounds.max_concurrent_calls) {
        Some(max_concurrent_calls) => bounded_rpc_client(transport, is_local, max_concurrent_calls),
        None => ClientBuilder::default().transport(transport, is_local),
    };

    let provider = provider_builder_new().connect_client(client);
    info!("Connected to RPC node successfully ({rpc_node_url})");

    Ok(provider)
}

/// Strips the URL that a `reqwest` error carries and prints, which may contain an API key, and
/// names the node by its redacted URL instead.
fn redact_errors(call: TransportFut<'static>, rpc_url: Arc<RpcUrl>) -> TransportFut<'static> {
    Box::pin(async move {
        call.await.map_err(|err| match err {
            RpcError::Transport(TransportErrorKind::Custom(source)) => {
                match source.downcast::<reqwest::Error>() {
                    Ok(source) => TransportErrorKind::custom(RpcRequestError {
                        rpc_url,
                        source: source.without_url(),
                    }),
                    Err(source) => RpcError::Transport(TransportErrorKind::Custom(source)),
                }
            }
            err => err,
        })
    })
}

/// A failed HTTP request to a RPC node.
#[derive(Debug)]
struct RpcRequestError {
    rpc_url: Arc<RpcUrl>,
    source: reqwest::Error,
}

impl fmt::Display for RpcRequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "request to {} failed: {}", self.rpc_url, self.source)
    }
}

impl std::error::Error for RpcRequestError {
    /// Returns the cause of the reqwest error, whose own message is already part of this one.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.source()
    }
}

static INSTALL_CRYPTO_PROVIDER_ONCE: Once = Once::new();

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::net::black_hole_server;
    use alloy::{
        primitives::U64,
        providers::Provider,
        transports::{
            TransportError,
            http::reqwest::Url,
            mock::{Asserter, MockTransport},
        },
    };
    use futures::future::join_all;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    const CALLS: usize = 8;

    /// Highest number of calls ever in flight at the same time, and the current one.
    #[derive(Default)]
    struct InFlightRecorder {
        current: AtomicUsize,
        max: AtomicUsize,
    }

    /// A mocked node answering `CALLS` block numbers, slowly enough for the calls to overlap, and
    /// recording how many of them are in flight.
    fn slow_mocked_node() -> (impl IntoBoxTransport, Arc<InFlightRecorder>) {
        let asserter = Asserter::new();
        for _ in 0..CALLS {
            asserter.push_success(&U64::from(1));
        }

        let in_flight = Arc::new(InFlightRecorder::default());
        let recorder = in_flight.clone();
        let transport = tower::ServiceBuilder::new()
            .map_future(move |fut: TransportFut<'static>| {
                let recorder = recorder.clone();
                Box::pin(async move {
                    // `fetch_add` returns the previous value, so `+ 1` is needed to get the
                    // number of calls in flight including this one.
                    let current = recorder.current.fetch_add(1, Ordering::SeqCst) + 1;
                    recorder.max.fetch_max(current, Ordering::SeqCst);
                    tokio::time::sleep(Duration::from_millis(50)).await;
                    let response = fut.await;
                    recorder.current.fetch_sub(1, Ordering::SeqCst);
                    response
                }) as TransportFut<'static>
            })
            .service(MockTransport::new(asserter));

        (transport, in_flight)
    }

    #[tokio::test]
    async fn concurrency_limit_bounds_the_calls_in_flight() {
        const MAX_IN_FLIGHT: usize = 2;

        let (transport, in_flight) = slow_mocked_node();
        let provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_client(bounded_rpc_client(
                transport,
                true,
                NonZeroUsize::new(MAX_IN_FLIGHT).unwrap(),
            ));

        let calls_res = join_all((0..CALLS).map(|_| provider.get_block_number())).await;
        assert!(calls_res.iter().all(|r| r.is_ok()), "{calls_res:?}");

        assert_eq!(in_flight.max.load(Ordering::SeqCst), MAX_IN_FLIGHT);
        assert_eq!(in_flight.current.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn calls_end_on_their_deadline() {
        const CALL_TIMEOUT: Duration = Duration::from_millis(200);

        // With and without a limit on the calls in flight.
        for max_concurrent_calls in [Some(NonZeroUsize::MIN), None] {
            let (url, accept_loop) = black_hole_server().await;
            let url = Url::parse(&format!("{url}/v2/{API_KEY}")).unwrap();
            let provider =
                connect_to_rpc_node_with_bounds(url.into(), 1, max_concurrent_calls, CALL_TIMEOUT)
                    .await
                    .expect("the bounded provider should have been built");

            // Without the deadline, the silent node would leave this call pending forever.
            let started = std::time::Instant::now();
            let call = tokio::time::timeout(Duration::from_secs(5), provider.get_block_number())
                .await
                .expect("the call should have ended on its own deadline");
            let err = call.unwrap_err();
            assert_no_api_key(&err);

            // And the deadline is what ended it, rather than the call failing on the spot.
            assert!(started.elapsed() >= CALL_TIMEOUT, "{:?}", started.elapsed());

            // The listeners log errors with their sources (`{:#}`), which say the call timed out.
            let logged = format!("{:#}", anyhow::Error::from(err));
            assert!(logged.contains("timed out"), "{logged}");

            accept_loop.abort();
        }
    }

    /// Guards the test above against a mocked node that would serialize the calls by itself.
    #[tokio::test]
    async fn unbounded_client_runs_every_call_at_once() {
        let (transport, in_flight) = slow_mocked_node();
        let provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_client(ClientBuilder::default().transport(transport, true));

        let calls_res = join_all((0..CALLS).map(|_| provider.get_block_number())).await;
        assert!(calls_res.iter().all(|r| r.is_ok()), "{calls_res:?}");

        assert_eq!(in_flight.max.load(Ordering::SeqCst), CALLS);
    }

    #[tokio::test]
    async fn refused_calls_do_not_log_the_api_key() {
        // Binding then dropping a listener leaves a local port that refuses connections.
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        drop(listener);

        let url = Url::parse(&format!("http://{address}/v2/{API_KEY}")).unwrap();
        let provider = connect_to_rpc_node(url.into(), 1)
            .await
            .expect("the provider should have been built");

        // Bounded, as a parallel test could bind the freed port to a server that never answers.
        let call = tokio::time::timeout(Duration::from_secs(5), provider.get_block_number())
            .await
            .expect("the refused call should have ended");
        assert_no_api_key(&call.unwrap_err());
    }

    /// An API key, put in the URL path as some RPC providers require.
    const API_KEY: &str = "secret-api-key";

    /// Asserts that the error of a failed HTTP call names the node by its redacted URL, and does
    /// not contain `API_KEY`.
    fn assert_no_api_key(err: &TransportError) {
        let chain = std::iter::successors(Some(err as &dyn std::error::Error), |err| err.source())
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(": ");
        let (display, debug) = (err.to_string(), format!("{err:?}"));
        assert!(display.contains("/<redacted> failed"), "{display}");
        // The message says why the call failed, which also guards against an error that would not
        // come from the HTTP client at all. alloy prints a custom transport error and also returns
        // it as its source, so the chain repeats the message, but never the reason on its own.
        assert!(display.contains("error sending request"), "{display}");
        assert_eq!(
            chain.matches("error sending request").count(),
            chain.matches("/<redacted> failed").count(),
            "{chain}"
        );
        assert!(!chain.contains(API_KEY), "{chain}");
        assert!(!debug.contains(API_KEY), "{debug}");
    }
}
