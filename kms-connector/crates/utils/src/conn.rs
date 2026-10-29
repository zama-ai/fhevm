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
    rpc::{
        client::{ClientBuilder, RpcClient},
        json_rpc::{ErrorPayload, Response, ResponsePacket, ResponsePayload},
    },
    transports::{
        IntoBoxTransport, RpcError, TransportError, TransportErrorKind, TransportFut,
        http::{
            Client, Http,
            reqwest::{self, Url},
        },
    },
};
use anyhow::anyhow;
use percent_encoding::percent_decode_str;
use serde_json::value::RawValue;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::{
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
        match options.connect(db_url.as_str()).await {
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
    let url_secrets = UrlSecrets::new(rpc_node_url.as_url());
    let transport = ServiceBuilder::new()
        .map_future(move |call| redact_errors(call, url_secrets.clone()))
        .service(transport);
    let client = match bounds.and_then(|bounds| bounds.max_concurrent_calls) {
        Some(max_concurrent_calls) => bounded_rpc_client(transport, is_local, max_concurrent_calls),
        None => ClientBuilder::default().transport(transport, is_local),
    };

    let provider = provider_builder_new().connect_client(client);
    info!("Connected to RPC node successfully ({rpc_node_url})");

    Ok(provider)
}

/// What replaces a secret in redacted text.
pub(crate) const REDACTED: &str = "<redacted>";

/// The parts of a RPC node URL that may hold an API key: the userinfo, the path segments and the
/// query values, raw and percent-decoded.
///
/// They are replaced in the error messages and bodies the node sends back, in case it echoes the
/// request URL.
#[derive(Clone)]
struct UrlSecrets(Arc<[String]>);

impl UrlSecrets {
    fn new(url: &Url) -> Self {
        // `Url` returns these parts percent-encoded.
        let userinfo_and_path_segments = [url.username(), url.password().unwrap_or_default()]
            .into_iter()
            .chain(url.path_segments().into_iter().flatten())
            .flat_map(|part| {
                let decoded = percent_decode_str(part).decode_utf8().ok();
                std::iter::once(part.to_string()).chain(decoded.map(|decoded| decoded.into_owned()))
            });
        let raw_query_values = url
            .query()
            .into_iter()
            .flat_map(|query| query.split('&'))
            .filter_map(|param| param.split_once('=').map(|(_, value)| value.to_string()));
        let decoded_query_values = url.query_pairs().map(|(_, value)| value.into_owned());
        let mut secrets: Vec<String> = userinfo_and_path_segments
            .chain(raw_query_values)
            .chain(decoded_query_values)
            .filter(|part| !part.is_empty())
            .collect();
        // Longest first, so that a part containing another one is replaced as a whole.
        secrets.sort_by_key(|secret| std::cmp::Reverse(secret.len()));
        secrets.dedup();
        Self(secrets.into())
    }

    /// Replaces the secrets in `text` where they stand as whole tokens, i.e. not inside a longer
    /// run of ASCII letters and digits. Short parts such as a `v2` path segment are then replaced
    /// without touching other words, or hex data such as the revert data of a JSON-RPC error.
    fn redact(&self, text: &str) -> String {
        self.0
            .iter()
            .fold(text.to_string(), |text, secret| redact_token(&text, secret))
    }
}

/// Replaces each occurrence of `token` in `text` that is not inside a longer run of ASCII letters
/// and digits.
///
/// The letter or digit that ends an escape, such as the `n` of a JSON `\n` or the `F` of a
/// percent-encoded `%2F`, does not count as part of such a run.
fn redact_token(text: &str, token: &str) -> String {
    let is_word = |c: char| c.is_ascii_alphanumeric();
    let mut redacted = String::with_capacity(text.len());
    let (mut copied, mut from) = (0, 0);
    while let Some(offset) = text[from..].find(token) {
        let start = from + offset;
        let end = start + token.len();
        let before = &text[..start];
        let bounded = (!before.chars().next_back().is_some_and(is_word)
            || ends_with_escape(before))
            && !text[end..].chars().next().is_some_and(is_word);
        if bounded {
            redacted.push_str(&text[copied..start]);
            redacted.push_str(REDACTED);
            (copied, from) = (end, end);
        } else {
            // A token-bounded occurrence may still start inside this one.
            from = start + text[start..].chars().next().map_or(1, char::len_utf8);
        }
    }
    redacted.push_str(&text[copied..]);
    redacted
}

/// Whether `text` ends with an escape whose last character is a letter or digit: a JSON escape
/// such as `\n` or `\u0020`, or a percent-encoded byte such as `%2F`.
fn ends_with_escape(text: &str) -> bool {
    match text.as_bytes() {
        [.., b'\\', b'b' | b'f' | b'n' | b'r' | b't'] => true,
        [.., b'%', high, low] => high.is_ascii_hexdigit() && low.is_ascii_hexdigit(),
        [.., b'\\', b'u', a, b, c, d] => [a, b, c, d].iter().all(|digit| digit.is_ascii_hexdigit()),
        _ => false,
    }
}

/// Strips the RPC node URL, which may contain an API key, from the errors of a call, and from the
/// JSON-RPC error responses that alloy turns into errors later on.
fn redact_errors(call: TransportFut<'static>, url_secrets: UrlSecrets) -> TransportFut<'static> {
    Box::pin(async move {
        match call.await {
            Ok(ResponsePacket::Single(response)) => Ok(ResponsePacket::Single(redact_failure(
                response,
                &url_secrets,
            ))),
            Ok(ResponsePacket::Batch(responses)) => Ok(ResponsePacket::Batch(
                responses
                    .into_iter()
                    .map(|response| redact_failure(response, &url_secrets))
                    .collect(),
            )),
            Err(err) => Err(redact_error(err, &url_secrets)),
        }
    })
}

/// Redacts the message and data of a JSON-RPC error response.
fn redact_failure(mut response: Response, url_secrets: &UrlSecrets) -> Response {
    if let ResponsePayload::Failure(payload) = &mut response.payload {
        redact_error_payload(payload, url_secrets);
    }
    response
}

/// Redacts the message and data of a JSON-RPC error.
///
/// The data is only rewritten if it holds a secret, so revert data is left as is. If the
/// redacted data is no longer valid JSON, it is dropped rather than kept with the secret.
fn redact_error_payload(payload: &mut ErrorPayload, url_secrets: &UrlSecrets) {
    let message = url_secrets.redact(&payload.message);
    if message != payload.message {
        payload.message = message.into();
    }
    if let Some(data) = &payload.data {
        let redacted = url_secrets.redact(data.get());
        if redacted != data.get() {
            payload.data = RawValue::from_string(redacted).ok();
        }
    }
}

/// Strips the URL that `reqwest` errors carry and print in their `Display` and `Debug` output,
/// and redacts the response bodies and JSON-RPC errors kept by the other errors.
fn redact_error(err: TransportError, url_secrets: &UrlSecrets) -> TransportError {
    match err {
        RpcError::Transport(TransportErrorKind::Custom(source)) => {
            match source.downcast::<reqwest::Error>() {
                Ok(reqwest_err) => TransportErrorKind::custom(reqwest_err.without_url()),
                Err(source) => RpcError::Transport(TransportErrorKind::Custom(source)),
            }
        }
        RpcError::Transport(TransportErrorKind::HttpError(http_err)) => {
            TransportErrorKind::http_error(http_err.status, url_secrets.redact(&http_err.body))
        }
        RpcError::DeserError { err, text } => RpcError::DeserError {
            err: redact_serde_error(err, url_secrets),
            text: url_secrets.redact(&text),
        },
        // alloy reads a body that is only the error object of a JSON-RPC response as an error
        // response.
        RpcError::ErrorResp(mut payload) => {
            redact_error_payload(&mut payload, url_secrets);
            RpcError::ErrorResp(payload)
        }
        err => err,
    }
}

/// Redacts the message of a serde error, which quotes the value it could not read, e.g. a
/// JSON-RPC `error` sent as a string rather than an object.
///
/// The error is only rebuilt if its message holds a secret.
fn redact_serde_error(err: serde_json::Error, url_secrets: &UrlSecrets) -> serde_json::Error {
    let message = err.to_string();
    let redacted = url_secrets.redact(&message);
    if redacted == message {
        err
    } else {
        // The message already ends with the line and column of the error.
        serde::de::Error::custom(redacted)
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

        assert_no_api_key(&provider.get_block_number().await.unwrap_err());
    }

    #[tokio::test]
    async fn echoed_api_keys_are_redacted() {
        let json_rpc_error = |data: String| {
            format!(
                r#"{{"jsonrpc":"2.0","id":0,"error":{{"code":-32600,"message":"invalid key {API_KEY}","data":"{data}"}}}}"#
            )
        };
        let cases = [
            // alloy keeps the body of an HTTP error...
            ("404 Not Found", format!("Cannot POST /v2/{API_KEY}")),
            ("401 Unauthorized", json_rpc_error(format!("/v2/{API_KEY}"))),
            // ... and of a response that is not JSON-RPC...
            ("200 OK", format!("Cannot POST /v2/{API_KEY}")),
            // ... and turns a JSON-RPC error response into an error...
            ("200 OK", json_rpc_error(format!("/v2/{API_KEY}"))),
            // ... keeping its data as raw JSON, where an escape may come right before the key...
            (
                "200 OK",
                json_rpc_error(format!(r"invalid key:\n{API_KEY}")),
            ),
            // ... and does the same with a body that is only the error object of a response.
            (
                "200 OK",
                format!(r#"{{"code":-32600,"message":"invalid key {API_KEY}"}}"#),
            ),
            // The error of a response that alloy cannot read quotes the part it could not read.
            (
                "200 OK",
                format!(r#"{{"jsonrpc":"2.0","id":0,"error":"invalid key {API_KEY}"}}"#),
            ),
        ];
        for (status, body) in cases {
            let (url, server) = answering_server(status, body).await;
            let url = Url::parse(&format!("{url}/v2/{API_KEY}")).unwrap();
            let provider = connect_to_rpc_node(url.into(), 1).await.unwrap();

            let err = provider.get_block_number().await.unwrap_err();
            let (display, debug) = (format!("{err}"), format!("{err:?}"));
            assert!(display.contains(REDACTED), "{status}: {display}");
            assert!(!display.contains(API_KEY), "{status}: {display}");
            assert!(!debug.contains(API_KEY), "{status}: {debug}");

            server.abort();
        }
    }

    #[tokio::test]
    async fn revert_data_is_kept() {
        // `Error(string)` revert data, with a URL path segment found inside it.
        let revert_data = "0x08c379a0";
        let body = format!(
            r#"{{"jsonrpc":"2.0","id":0,"error":{{"code":3,"message":"execution reverted: {API_KEY}","data":"{revert_data}"}}}}"#
        );
        let (url, server) = answering_server("200 OK", body).await;
        let url = Url::parse(&format!("{url}/v2/{API_KEY}/c379")).unwrap();
        let provider = connect_to_rpc_node(url.into(), 1).await.unwrap();

        let err = provider.get_block_number().await.unwrap_err();
        let error_resp = err.as_error_resp().expect("a JSON-RPC error response");
        assert_eq!(error_resp.message, "execution reverted: <redacted>");
        assert_eq!(
            error_resp.as_revert_data().unwrap().to_string(),
            revert_data
        );

        server.abort();
    }

    #[test]
    fn url_secrets_are_redacted_as_whole_tokens() {
        let url = Url::parse("https://user:pw@rpc.example.com/v2/k3y?apikey=a%2Bb").unwrap();
        let secrets = UrlSecrets::new(&url);
        let redacted = |text| secrets.redact(text);

        assert_eq!(
            redacted("Cannot POST /v2/k3y?apikey=a%2Bb"),
            "Cannot POST /<redacted>/<redacted>?apikey=<redacted>"
        );
        // The query value is also redacted percent-decoded, and the userinfo as well.
        assert_eq!(
            redacted("key a+b of user:pw"),
            "key <redacted> of <redacted>:<redacted>"
        );
        // Not inside longer words or hex data.
        assert_eq!(redacted("v20 k3yz 0xk3y"), "v20 k3yz 0xk3y");
        // But right after an escape, in JSON or percent-encoded text.
        assert_eq!(
            redacted(r#""key:\nk3y" "key\u0020k3y" Cannot POST %2Fv2%2Fk3y"#),
            r#""key:\n<redacted>" "key\u0020<redacted>" Cannot POST %2F<redacted>%2F<redacted>"#
        );
        // The userinfo and path segments are also redacted percent-decoded.
        let url = Url::parse("https://us%2Ber:p%40ss@rpc.example.com/v2/k%2B3y").unwrap();
        assert_eq!(
            UrlSecrets::new(&url).redact("key k+3y of us+er:p@ss"),
            "key <redacted> of <redacted>:<redacted>"
        );
        // An occurrence that is not a token does not hide a following one that is.
        let secrets = UrlSecrets::new(&Url::parse("https://rpc.example.com/ab-ab").unwrap());
        assert_eq!(secrets.redact("xab-ab-ab"), "xab-<redacted>");
    }

    /// Starts a server answering every request with `status` and `body`.
    ///
    /// Returns the URL of the server, and the handle of its accept loop to abort.
    async fn answering_server(
        status: &'static str,
        body: String,
    ) -> (String, tokio::task::JoinHandle<()>) {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let accept_loop = tokio::spawn(async move {
            while let Ok((mut stream, _)) = listener.accept().await {
                let mut request = [0; 4096];
                // The request fits in one read: it is only read for the client to send it.
                let read = stream.read(&mut request).await.unwrap();
                assert!(
                    read > 0,
                    "the client closed the connection without a request"
                );
                let response = format!(
                    "HTTP/1.1 {status}\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
                    body.len()
                );
                stream.write_all(response.as_bytes()).await.unwrap();
            }
        });
        (url, accept_loop)
    }

    /// An API key, put in the URL path as some RPC providers require.
    const API_KEY: &str = "secret-api-key";

    /// Asserts that the error of a failed HTTP call does not contain `API_KEY`.
    fn assert_no_api_key(err: &TransportError) {
        let (display, debug) = (format!("{err}"), format!("{err:?}"));
        // Guards against an error that would not come from the HTTP client at all.
        assert!(display.contains("error sending request"), "{display}");
        assert!(!display.contains(API_KEY), "{display}");
        assert!(!debug.contains(API_KEY), "{debug}");
    }
}
