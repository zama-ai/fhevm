//! Load tools of the PRF QA campaign (Run B): bursts of users, a rogue that repeats one stored
//! output, slow or idle TLS connections, and a metrics read. Each prints one summary, not one line
//! per request.

use crate::connector::{Connector, ReplyReport, Retrieval};
use alloy::primitives::B256;
use reqwest::Method;
use serde::Serialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    task::JoinSet,
    time::Instant,
};

#[derive(Debug, Default, Serialize)]
pub struct Percentiles {
    pub p50: u64,
    pub p95: u64,
    pub p99: u64,
    pub max: u64,
}

impl Percentiles {
    fn of(mut values: Vec<u64>) -> Self {
        values.sort_unstable();
        let at = |q: f64| {
            values
                .get(((values.len() as f64 * q).ceil() as usize).saturating_sub(1))
                .copied()
                .unwrap_or_default()
        };
        Self {
            p50: at(0.50),
            p95: at(0.95),
            p99: at(0.99),
            max: values.last().copied().unwrap_or_default(),
        }
    }
}

/// The highest count of open file descriptors of this process while `running`, sampled every
/// 250 ms, to tell a saturated load generator apart.
fn sample_fds(running: Arc<AtomicBool>) -> tokio::task::JoinHandle<usize> {
    tokio::spawn(async move {
        let mut max = 0;
        while running.load(Ordering::Relaxed) {
            let open = std::fs::read_dir("/proc/self/fd")
                .or_else(|_| std::fs::read_dir("/dev/fd"))
                .map(|dir| dir.count())
                .unwrap_or_default();
            max = max.max(open);
            tokio::time::sleep(Duration::from_millis(250)).await;
        }
        max
    })
}

/// Adds the answers of one request loop: its final status, then each retried answer.
fn count_statuses(statuses: &mut BTreeMap<String, usize>, reply: &ReplyReport) {
    let last = match (reply.status, &reply.code) {
        (Some(status), Some(code)) => format!("{status} {code}"),
        (Some(status), None) => status.to_string(),
        (None, code) => code.clone().unwrap_or_else(|| "connection_error".into()),
    };
    *statuses.entry(last).or_default() += 1;
    for retried in &reply.retried {
        *statuses.entry(retried.clone()).or_default() += 1;
    }
}

pub struct Burst {
    pub users: usize,
    /// Users arrive evenly over this window.
    pub window: Duration,
    /// Concurrent requests of each connector for each label, as for several decryptors.
    pub copies: usize,
    pub pad_length: u16,
    pub label_prefix: u8,
    /// The first connector starts each label this much before the others.
    pub lead: Duration,
    /// Ask this one coprocessor instead of retrieving from all of them.
    pub only: Option<usize>,
    /// Once every retrieval returned, retry each failed one once with this deadline: a label that
    /// missed the deadline must be served from the stored output.
    pub retry_deadline: Option<Duration>,
}

#[derive(Debug, Default, Serialize)]
pub struct BurstReport {
    pub users: usize,
    pub requests: usize,
    pub ok: usize,
    /// Failed retrievals as `<label hex>:<connector>`, to retry once the queue drained.
    pub failed: Vec<String>,
    pub failed_labels: usize,
    pub latency_ms: Percentiles,
    /// How late each request started against its schedule: a high p99 means the load generator
    /// is saturated.
    pub lag_ms: Percentiles,
    /// Final answers and retried answers, by `<status> <code>`.
    pub statuses: BTreeMap<String, usize>,
    pub fallback_rate: f64,
    pub bytes_per_label: usize,
    /// From the first scheduled request to the last answer.
    pub drain_ms: u64,
    pub max_fds: usize,
    pub retry: Option<RetryReport>,
}

struct Outcome {
    label: B256,
    connector: usize,
    ok: bool,
    elapsed_ms: u64,
    lag_ms: u64,
    fallback: bool,
    bytes: usize,
    replies: Vec<ReplyReport>,
}

/// Users arrive over the window; for each one, every connector retrieves a new label (or asks one
/// coprocessor), `copies` times at once.
pub async fn burst(connectors: &[Arc<Connector>], burst: &Burst) -> BurstReport {
    let running = Arc::new(AtomicBool::new(true));
    let fds = sample_fds(running.clone());
    let start = Instant::now();
    let mut tasks = JoinSet::new();
    for user in 0..burst.users {
        let mut label = B256::from(rand::random::<[u8; 32]>());
        label.0[0] = burst.label_prefix;
        let arrival = start + burst.window.mul_f64(user as f64 / burst.users as f64);
        for (k, connector) in connectors.iter().enumerate() {
            let scheduled = if k == 0 {
                arrival
            } else {
                arrival + burst.lead
            };
            for _ in 0..burst.copies {
                let (connector, pad_length, only) =
                    (connector.clone(), burst.pad_length, burst.only);
                tasks.spawn(async move {
                    tokio::time::sleep_until(scheduled).await;
                    let lag_ms = scheduled.elapsed().as_millis() as u64;
                    let begun = Instant::now();
                    let retrieval = Retrieval {
                        prf: 0,
                        label,
                        pad_length,
                        get_from: None,
                        first_expires: None,
                    };
                    let mut outcome = Outcome {
                        label,
                        connector: k,
                        ok: false,
                        elapsed_ms: 0,
                        lag_ms,
                        fallback: false,
                        bytes: 0,
                        replies: Vec::new(),
                    };
                    match only {
                        Some(i) => {
                            if let Some(reply) =
                                connector.ask_one(i, Method::GET, &retrieval.path()).await
                            {
                                outcome.ok = reply.status == Some(200);
                                outcome.replies.push(reply);
                            }
                        }
                        None => {
                            let report = connector.retrieve(&retrieval).await;
                            outcome.ok = report.ok;
                            outcome.fallback = report.fallback;
                            outcome.bytes = report.bytes;
                            outcome.replies = report.replies;
                        }
                    }
                    outcome.elapsed_ms = begun.elapsed().as_millis() as u64;
                    outcome
                });
            }
        }
    }
    let mut outcomes = Vec::new();
    while let Some(outcome) = tasks.join_next().await {
        outcomes.extend(outcome.ok());
    }
    let drain_ms = start.elapsed().as_millis() as u64;
    running.store(false, Ordering::Relaxed);

    let mut report = BurstReport {
        users: burst.users,
        requests: outcomes.len(),
        drain_ms,
        max_fds: fds.await.unwrap_or_default(),
        ..Default::default()
    };
    let mut failed_labels = BTreeSet::new();
    let (mut fallbacks, mut bytes) = (0, 0);
    for outcome in &outcomes {
        if outcome.ok {
            report.ok += 1;
        } else {
            report.failed.push(format!(
                "{}:{}",
                hex::encode(outcome.label),
                outcome.connector
            ));
            failed_labels.insert(outcome.label);
        }
        fallbacks += usize::from(outcome.fallback);
        bytes += outcome.bytes;
        for reply in &outcome.replies {
            count_statuses(&mut report.statuses, reply);
        }
        if burst.only.is_some() && outcome.replies.is_empty() {
            *report.statuses.entry("deadline".into()).or_default() += 1;
        }
    }
    report.failed_labels = failed_labels.len();
    report.fallback_rate = fallbacks as f64 / outcomes.len().max(1) as f64;
    report.bytes_per_label = bytes / burst.users.max(1);
    report.latency_ms = Percentiles::of(outcomes.iter().map(|o| o.elapsed_ms).collect());
    report.lag_ms = Percentiles::of(outcomes.iter().map(|o| o.lag_ms).collect());
    if let (Some(deadline), None, false) =
        (burst.retry_deadline, burst.only, report.failed.is_empty())
    {
        let patient: Vec<_> = connectors
            .iter()
            .map(|c| {
                Arc::new(Connector {
                    deadline,
                    ..(**c).clone()
                })
            })
            .collect();
        report.retry = Some(retry(&patient, &report.failed, burst.pad_length).await);
    }
    report
}

#[derive(Debug, Default, Serialize)]
pub struct RetryReport {
    pub retried: usize,
    pub ok: usize,
    pub failed: Vec<String>,
    pub latency_ms: Percentiles,
}

/// Retries each `<label hex>:<connector>` once, 512 at a time.
async fn retry(connectors: &[Arc<Connector>], items: &[String], pad_length: u16) -> RetryReport {
    let permits = Arc::new(tokio::sync::Semaphore::new(512));
    let mut tasks = JoinSet::new();
    for item in items {
        let Some((label, k)) = item.split_once(':') else {
            continue;
        };
        let (Ok(label), Some(connector)) = (
            label.parse::<B256>(),
            k.parse::<usize>().ok().and_then(|k| connectors.get(k)),
        ) else {
            continue;
        };
        let (connector, permits, item) = (connector.clone(), permits.clone(), item.clone());
        tasks.spawn(async move {
            let _permit = permits.acquire_owned().await;
            let report = connector
                .retrieve(&Retrieval {
                    prf: 0,
                    label,
                    pad_length,
                    get_from: None,
                    first_expires: None,
                })
                .await;
            (item, report.ok, report.elapsed_ms)
        });
    }
    let (mut ok, mut failed, mut latencies) = (0, Vec::new(), Vec::new());
    while let Some(Ok((item, success, elapsed))) = tasks.join_next().await {
        latencies.push(elapsed);
        if success {
            ok += 1;
        } else {
            failed.push(item);
        }
    }
    RetryReport {
        retried: latencies.len(),
        ok,
        failed,
        latency_ms: Percentiles::of(latencies),
    }
}

#[derive(Debug, Serialize)]
pub struct HammerReport {
    pub requests: usize,
    pub req_per_s: f64,
    pub mb_per_s: f64,
    pub statuses: BTreeMap<String, usize>,
    pub latency_ms: Percentiles,
    pub max_fds: usize,
}

/// One loop per path sends signed requests for it to one coprocessor, without retries, for
/// `duration`. Prf-service takes one request per (connector, label) at a time, so each loop needs
/// its own label.
pub async fn hammer(
    connector: Arc<Connector>,
    i: usize,
    method: Method,
    paths: Vec<String>,
    duration: Duration,
) -> HammerReport {
    let running = Arc::new(AtomicBool::new(true));
    let fds = sample_fds(running.clone());
    let started = Instant::now();
    let until = started + duration;
    let bytes = Arc::new(AtomicUsize::new(0));
    let mut tasks = JoinSet::new();
    for path in paths {
        let (connector, method, bytes) = (connector.clone(), method.clone(), bytes.clone());
        tasks.spawn(async move {
            let (mut statuses, mut latencies) = (BTreeMap::<String, usize>::new(), Vec::new());
            while Instant::now() < until {
                let sent = Instant::now();
                let key = match connector.send(i, method.clone(), &path).await {
                    Ok(response) => {
                        let status = response.status().as_u16();
                        let body = response.bytes().await.map(|b| b.len()).unwrap_or_default();
                        bytes.fetch_add(body, Ordering::Relaxed);
                        status.to_string()
                    }
                    Err(_) => "connection_error".to_owned(),
                };
                latencies.push(sent.elapsed().as_millis() as u64);
                *statuses.entry(key).or_default() += 1;
            }
            (statuses, latencies)
        });
    }
    let (mut statuses, mut latencies) = (BTreeMap::new(), Vec::new());
    while let Some(Ok((counts, times))) = tasks.join_next().await {
        for (key, n) in counts {
            *statuses.entry(key).or_default() += n;
        }
        latencies.extend(times);
    }
    running.store(false, Ordering::Relaxed);
    let seconds = started.elapsed().as_secs_f64();
    HammerReport {
        requests: latencies.len(),
        req_per_s: latencies.len() as f64 / seconds,
        mb_per_s: bytes.load(Ordering::Relaxed) as f64 / seconds / 1e6,
        statuses,
        latency_ms: Percentiles::of(latencies),
        max_fds: fds.await.unwrap_or_default(),
    }
}

#[derive(Debug, Default, Serialize)]
pub struct SlowlorisReport {
    pub opened: usize,
    pub failed_to_open: usize,
    /// Connections that send one header byte every `byte_every` and are still open at the end.
    pub slow_open_at_end: usize,
    /// Connections idle after the TLS handshake and still open at the end.
    pub idle_open_at_end: usize,
    /// For the connections the server closed: seconds from open to close.
    pub closed_after_s: Percentiles,
    pub max_fds: usize,
}

/// Opens `slow` TLS connections that send a request head one byte every `byte_every`, and `idle`
/// TLS connections that send nothing, and keeps them for `duration`.
pub async fn slowloris(
    url: &str,
    slow: usize,
    idle: usize,
    duration: Duration,
    byte_every: Duration,
) -> anyhow::Result<SlowlorisReport> {
    let url = reqwest::Url::parse(url)?;
    let host = url.host_str().unwrap_or_default().to_owned();
    let port = url.port_or_known_default().unwrap_or(443);
    let connector = tokio_rustls::TlsConnector::from(Arc::new(accept_any_certificate()?));
    let server_name = rustls::pki_types::ServerName::try_from(host.clone())?;
    let running = Arc::new(AtomicBool::new(true));
    let fds = sample_fds(running.clone());
    let until = Instant::now() + duration;
    let mut tasks = JoinSet::new();
    for n in 0..slow + idle {
        let (connector, server_name, address) = (
            connector.clone(),
            server_name.clone(),
            format!("{host}:{port}"),
        );
        let is_slow = n < slow;
        tasks.spawn(async move {
            let opened = Instant::now();
            let Ok(Ok(tcp)) =
                tokio::time::timeout(Duration::from_secs(10), TcpStream::connect(&address)).await
            else {
                return (is_slow, None);
            };
            let Ok(Ok(tls)) =
                tokio::time::timeout(Duration::from_secs(30), connector.connect(server_name, tcp))
                    .await
            else {
                return (is_slow, None);
            };
            let (mut reader, mut writer) = tokio::io::split(tls);
            let mut head = b"GET /v1/prf/0/labels/00/64 HTTP/1.1\r\nHost: x\r\nX-Slow: ".to_vec();
            if !is_slow {
                head.clear();
            }
            let mut next = 0;
            let mut buf = [0u8; 256];
            loop {
                if Instant::now() >= until {
                    return (is_slow, Some(None));
                }
                let tick = tokio::time::sleep(byte_every.min(until - Instant::now()));
                tokio::select! {
                    _ = reader.read(&mut buf) => {
                        return (is_slow, Some(Some(opened.elapsed().as_secs())));
                    }
                    _ = tick, if is_slow => {
                        let byte = if next < head.len() { head[next] } else { b'a' };
                        next += 1;
                        if writer.write_all(&[byte]).await.is_err() {
                            return (is_slow, Some(Some(opened.elapsed().as_secs())));
                        }
                    }
                    _ = tokio::time::sleep_until(until), if !is_slow => {}
                }
            }
        });
    }
    let mut report = SlowlorisReport::default();
    let mut closed = Vec::new();
    while let Some(Ok((is_slow, result))) = tasks.join_next().await {
        match result {
            None => report.failed_to_open += 1,
            Some(closed_after) => {
                report.opened += 1;
                match closed_after {
                    Some(seconds) => closed.push(seconds),
                    None if is_slow => report.slow_open_at_end += 1,
                    None => report.idle_open_at_end += 1,
                }
            }
        }
    }
    running.store(false, Ordering::Relaxed);
    report.closed_after_s = Percentiles::of(closed);
    report.max_fds = fds.await.unwrap_or_default();
    Ok(report)
}

/// A TLS client that accepts any server certificate: the preview's proxies use a self-signed
/// `CA:TRUE` certificate, and this client only holds connections open.
fn accept_any_certificate() -> anyhow::Result<rustls::ClientConfig> {
    let provider = Arc::new(rustls::crypto::aws_lc_rs::default_provider());
    Ok(
        rustls::ClientConfig::builder_with_provider(provider.clone())
            .with_safe_default_protocol_versions()?
            .dangerous()
            .with_custom_certificate_verifier(Arc::new(AcceptAnyCertificate(provider)))
            .with_no_client_auth(),
    )
}

#[derive(Debug)]
struct AcceptAnyCertificate(Arc<rustls::crypto::CryptoProvider>);

impl rustls::client::danger::ServerCertVerifier for AcceptAnyCertificate {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::pki_types::CertificateDer<'_>,
        _intermediates: &[rustls::pki_types::CertificateDer<'_>],
        _server_name: &rustls::pki_types::ServerName<'_>,
        _ocsp_response: &[u8],
        _now: rustls::pki_types::UnixTime,
    ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::danger::ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &rustls::pki_types::CertificateDer<'_>,
        dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls12_signature(
            message,
            cert,
            dss,
            &self.0.signature_verification_algorithms,
        )
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &rustls::pki_types::CertificateDer<'_>,
        dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls13_signature(
            message,
            cert,
            dss,
            &self.0.signature_verification_algorithms,
        )
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        self.0.signature_verification_algorithms.supported_schemes()
    }
}

/// The lines of a Prometheus text page that start with `prefix`.
pub async fn metrics(url: &str, prefix: &str) -> anyhow::Result<Vec<String>> {
    let page = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;
    Ok(page
        .lines()
        .filter(|line| line.starts_with(prefix))
        .map(str::to_owned)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percentiles_pick_ranks() {
        let p = Percentiles::of((1..=100).collect());
        assert_eq!((p.p50, p.p95, p.p99, p.max), (50, 95, 99, 100));
        assert_eq!(Percentiles::of(Vec::new()).max, 0);
    }

    /// K5 against a local TLS listener that never answers: every connection opens and stays open.
    #[tokio::test]
    async fn slowloris_holds_connections() {
        let certified = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
        let key = rustls::pki_types::PrivateKeyDer::try_from(certified.signing_key.serialize_der())
            .unwrap();
        let server = rustls::ServerConfig::builder_with_provider(Arc::new(
            rustls::crypto::aws_lc_rs::default_provider(),
        ))
        .with_safe_default_protocol_versions()
        .unwrap()
        .with_no_client_auth()
        .with_single_cert(vec![certified.cert.der().clone()], key)
        .unwrap();
        let acceptor = tokio_rustls::TlsAcceptor::from(Arc::new(server));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        tokio::spawn(async move {
            loop {
                let (tcp, _) = listener.accept().await.unwrap();
                let acceptor = acceptor.clone();
                tokio::spawn(async move {
                    if let Ok(mut tls) = acceptor.accept(tcp).await {
                        let mut sink = Vec::new();
                        let _ = tls.read_to_end(&mut sink).await;
                    }
                });
            }
        });
        let report = slowloris(
            &format!("https://localhost:{port}"),
            3,
            2,
            Duration::from_millis(1500),
            Duration::from_millis(200),
        )
        .await
        .unwrap();
        assert_eq!(
            (
                report.opened,
                report.slow_open_at_end,
                report.idle_open_at_end
            ),
            (5, 3, 2),
            "{report:?}"
        );
    }
}
