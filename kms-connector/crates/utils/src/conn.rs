use crate::{
    config::KmsWallet,
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
        IntoBoxTransport, TransportFut,
        http::{Client, Http, reqwest::Url},
    },
};
use anyhow::anyhow;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::{num::NonZeroUsize, sync::Once, time::Duration};
use tower::{
    limit::{GlobalConcurrencyLimitLayer, concurrency::future::ResponseFuture},
    util::MapFutureLayer,
};
use tracing::{info, warn};

/// The number of connection retry to connect to the database or to a RPC node.
pub const CONNECTION_RETRY_NUMBER: usize = 5;

/// The delay between two connection attempts.
pub const CONNECTION_RETRY_DELAY: Duration = Duration::from_secs(2);

/// Tries to establish the connection with Postgres database.
pub async fn connect_to_db(db_url: &str, db_pool_size: u32) -> anyhow::Result<Pool<Postgres>> {
    for i in 1..=CONNECTION_RETRY_NUMBER {
        info!("Attempting connection to DB... ({i}/{CONNECTION_RETRY_NUMBER})");

        let options = PgPoolOptions::new().max_connections(db_pool_size);
        match options.connect(db_url).await {
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

/// The ceilings of a RPC connection: how many calls it may have in flight, and how long any single
/// one of them may take.
#[derive(Clone, Copy, Debug)]
struct RpcCallBounds {
    max_concurrent_calls: NonZeroUsize,
    call_timeout: Duration,
}

/// Tries to establish the connection with a RPC node.
pub async fn connect_to_rpc_node(
    rpc_node_url: Url,
    chain_id: u64,
) -> anyhow::Result<DefaultProvider> {
    connect_to_rpc_node_inner(rpc_node_url, None, || {
        ProviderBuilder::new().with_chain_id(chain_id)
    })
    .await
}

/// Tries to establish the connection with a RPC node, bounding the calls it may have in flight and
/// the duration of each of them.
pub async fn connect_to_rpc_node_with_bounds(
    rpc_node_url: Url,
    chain_id: u64,
    max_concurrent_calls: NonZeroUsize,
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
    rpc_node_url: Url,
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
    rpc_node_url: Url,
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

    let client = match bounds {
        Some(bounds) => {
            let http_client = Client::builder()
                .timeout(bounds.call_timeout)
                .build()
                .map_err(|e| anyhow!("Failed to create the RPC HTTP client: {e}"))?;
            let transport = Http::with_client(http_client, rpc_node_url.clone());
            let is_local = transport.guess_local();
            bounded_rpc_client(transport, is_local, bounds.max_concurrent_calls)
        }
        None => {
            let transport = Http::new(rpc_node_url.clone());
            let is_local = transport.guess_local();
            ClientBuilder::default().transport(transport, is_local)
        }
    };

    let provider = provider_builder_new().connect_client(client);
    info!(
        "Connected to RPC node successfully ({})",
        rpc_node_url.host_str().unwrap_or("unexpected URL format")
    );

    Ok(provider)
}

static INSTALL_CRYPTO_PROVIDER_ONCE: Once = Once::new();

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::net::black_hole_server;
    use alloy::{
        primitives::U64,
        providers::Provider,
        transports::mock::{Asserter, MockTransport},
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
    async fn bounded_calls_end_on_their_deadline() {
        const CALL_TIMEOUT: Duration = Duration::from_millis(200);

        let (url, accept_loop) = black_hole_server().await;
        let url = Url::parse(&url).unwrap();
        let provider = connect_to_rpc_node_with_bounds(url, 1, NonZeroUsize::MIN, CALL_TIMEOUT)
            .await
            .expect("the bounded provider should have been built");

        // Without the deadline, the silent node would leave this call pending forever.
        let started = std::time::Instant::now();
        let call = tokio::time::timeout(Duration::from_secs(5), provider.get_block_number())
            .await
            .expect("the call should have ended on its own deadline");
        assert!(call.is_err(), "{call:?}");

        // And the deadline is what ended it, rather than the call failing on the spot.
        assert!(started.elapsed() >= CALL_TIMEOUT, "{:?}", started.elapsed());

        accept_loop.abort();
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
}
