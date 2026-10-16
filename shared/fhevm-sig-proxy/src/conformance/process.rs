//! A proxy binary run as a child process, configured by its own package for one test.

use super::{
    FakeRpc, KmsContext, Stub, TestTls, authorization, free_addr, sender, wait_for_listener,
};
use crate::{ErrorBody, ProxyError};
use alloy::transports::http::reqwest::{self, Client, Url};
use http::{Method, StatusCode};
use std::{
    io::{BufRead, BufReader, Read},
    net::SocketAddr,
    process::{Child, Command, Stdio},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

/// How long a proxy waits for its endpoint's response before answering `504`.
pub const ENDPOINT_RESPONSE_TIMEOUT: Duration = Duration::from_secs(1);
/// How often a proxy probes its endpoint.
pub const ENDPOINT_HEALTHCHECK_FREQUENCY: Duration = Duration::from_millis(200);
/// How often a proxy reading `ProtocolConfig` reads its signers again.
pub const SIGNER_REFRESH_INTERVAL: Duration = Duration::from_millis(200);

/// What a deployment's package tells the suite about its proxy binary.
pub struct Profile {
    /// The binary, `env!("CARGO_BIN_EXE_<name>")`; it is run with `start`.
    pub binary: &'static str,
    /// Configures the child, which starts with an empty environment, from the [`Launch`]. The
    /// allowed signer is [`sender`], the audience [`AUDIENCE`](super::AUDIENCE), the domain
    /// ([`CHAIN_ID`](super::CHAIN_ID), [`PROTOCOL_CONFIG`](super::PROTOCOL_CONFIG)), and the
    /// timings are this module's constants.
    pub configure: fn(&Launch, &mut Command),
    /// The route the suite sends signed requests on.
    pub route: SignedRoute,
    /// Where the proxy serves `/metrics`.
    pub metrics: MetricsPort,
    /// Whether the endpoint receives the client's `Authorization` header.
    pub forwards_authorization: bool,
    /// The deployment's own renderer of the errors the proxy answers itself.
    pub error_body: fn(&ProxyError) -> ErrorBody,
}

/// A route of the deployment and the requests the suite derives from it.
pub struct SignedRoute {
    pub method: Method,
    /// A concrete path of the route.
    pub path: &'static str,
    /// The body of a valid request.
    pub body: &'static [u8],
    /// The route template, the `route` label of the request counter.
    pub template: &'static str,
    /// A method the route refuses, and the `Allow` value of the `405`.
    pub other_method: Method,
    pub allow: &'static str,
    /// The path in a parameter encoding the route refuses, if it has parameters.
    pub non_canonical_path: Option<&'static str>,
}

pub enum MetricsPort {
    /// On the port of `/healthz`.
    Health,
    /// On a port of its own.
    Own,
}

/// The addresses and files one proxy process is given.
pub struct Launch {
    /// The TLS listener.
    pub bind: SocketAddr,
    /// The server of `/healthz`.
    pub health: SocketAddr,
    /// The server of `/metrics`: the health server for [`MetricsPort::Health`].
    pub metrics: SocketAddr,
    /// The certificate, and the directory any other file of the launch goes in.
    pub tls: TestTls,
    /// The only endpoint.
    pub endpoint: SocketAddr,
    /// The Ethereum RPC serving `ProtocolConfig`.
    pub rpc_url: Url,
}

/// The world a proxy process starts in.
pub struct Setup {
    /// Whether the endpoint answers; when not, nothing listens on its address.
    pub endpoint_up: bool,
    /// The KMS contexts of the fake `ProtocolConfig`.
    pub contexts: Vec<KmsContext>,
    /// Whether the fake RPC fails every call.
    pub rpc_failing: bool,
}

impl Default for Setup {
    /// A reachable endpoint, and [`sender`] the tx-sender of the only Active KMS context.
    fn default() -> Self {
        Self {
            endpoint_up: true,
            contexts: vec![KmsContext {
                active: true,
                tx_senders: vec![sender().address()],
            }],
            rpc_failing: false,
        }
    }
}

/// A proxy binary running in front of a [`Stub`], killed on drop.
pub struct ProxyProcess {
    pub stub: Stub,
    pub rpc: FakeRpc,
    pub client: Client,
    pub launch: Launch,
    child: Child,
    output: Arc<Mutex<String>>,
}

impl ProxyProcess {
    /// Starts the binary and waits for its TLS listener and its health server.
    pub async fn start(profile: &Profile, setup: Setup) -> Self {
        let stub = Stub::start().await;
        let rpc = FakeRpc::start(setup.contexts).await;
        rpc.set_failing(setup.rpc_failing);
        let health = free_addr();
        let launch = Launch {
            bind: free_addr(),
            health,
            metrics: match profile.metrics {
                MetricsPort::Health => health,
                MetricsPort::Own => free_addr(),
            },
            tls: TestTls::generate(),
            endpoint: if setup.endpoint_up {
                stub.addr
            } else {
                free_addr()
            },
            rpc_url: rpc.url.clone(),
        };

        let mut command = Command::new(profile.binary);
        command
            .arg("start")
            .env_clear()
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        (profile.configure)(&launch, &mut command);
        let mut child = command.spawn().unwrap();
        let output = Arc::new(Mutex::new(String::new()));
        capture(child.stdout.take().unwrap(), output.clone());
        capture(child.stderr.take().unwrap(), output.clone());
        let mut proxy = Self {
            stub,
            rpc,
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .tls_danger_accept_invalid_certs(true) // self-signed test certificate
                .build()
                .unwrap(),
            launch,
            child,
            output,
        };
        for addr in [proxy.launch.bind, proxy.launch.health] {
            let exited = tokio::select! {
                _ = wait_for_listener(addr) => None,
                status = proxy.exited() => Some(status),
            };
            if let Some(status) = exited {
                panic!("{} exited ({status}):\n{}", profile.binary, proxy.output());
            }
        }
        proxy
    }

    async fn exited(&mut self) -> std::process::ExitStatus {
        loop {
            if let Some(status) = self.child.try_wait().unwrap() {
                return status;
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }

    pub fn url(&self, path: &str) -> String {
        format!("https://{}{path}", self.launch.bind)
    }

    /// A request of the route with its valid body, signed by [`sender`].
    pub async fn signed(&self, route: &SignedRoute) -> reqwest::Response {
        let authorization = authorization(route.path, route.body).await;
        self.send(
            route.method.clone(),
            route.path,
            Some(&authorization),
            route.body,
        )
        .await
    }

    pub async fn send(
        &self,
        method: Method,
        path: &str,
        authorization: Option<&str>,
        body: &[u8],
    ) -> reqwest::Response {
        let mut request = self.client.request(method, self.url(path));
        if let Some(authorization) = authorization {
            request = request.header("authorization", authorization);
        }
        request.body(body.to_vec()).send().await.unwrap()
    }

    pub async fn healthz(&self) -> StatusCode {
        let url = format!("http://{}/healthz", self.launch.health);
        self.client.get(url).send().await.unwrap().status()
    }

    /// Polls `/healthz` until it answers `status`.
    pub async fn wait_for_healthz(&self, status: StatusCode) {
        let deadline = Instant::now() + Duration::from_secs(10);
        while self.healthz().await != status {
            assert!(
                Instant::now() < deadline,
                "/healthz never answered {status}:\n{}",
                self.output()
            );
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }

    /// The Prometheus text of `/metrics`.
    pub async fn metrics(&self) -> String {
        let url = format!("http://{}/metrics", self.launch.metrics);
        self.client
            .get(url)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
    }

    /// What the process wrote so far, stdout and stderr interleaved, without terminal colours.
    pub fn output(&self) -> String {
        strip_ansi(&self.output.lock().unwrap())
    }
}

impl Drop for ProxyProcess {
    fn drop(&mut self) {
        self.child.kill().ok();
        self.child.wait().ok();
    }
}

fn capture(stream: impl Read + Send + 'static, output: Arc<Mutex<String>>) {
    std::thread::spawn(move || {
        for line in BufReader::new(stream).lines() {
            let Ok(line) = line else { return };
            let mut output = output.lock().unwrap();
            output.push_str(&line);
            output.push('\n');
        }
    });
}

/// Removes the `ESC [ ... letter` sequences a terminal logger colours its fields with.
fn strip_ansi(text: &str) -> String {
    let mut plain = String::with_capacity(text.len());
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        if c == '\u{1b}' {
            for c in chars.by_ref() {
                if c.is_ascii_alphabetic() {
                    break;
                }
            }
        } else {
            plain.push(c);
        }
    }
    plain
}
