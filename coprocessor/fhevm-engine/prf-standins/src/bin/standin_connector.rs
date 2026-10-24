use alloy::{
    primitives::{Address, B256},
    signers::local::PrivateKeySigner,
};
use anyhow::{bail, Context};
use clap::{Parser, Subcommand};
use futures_util::future::join_all;
use prf_standins::{
    connector::{Connector, Coprocessor, ProbeOptions, Retrieval},
    load,
};
use reqwest::Method;
use std::{collections::BTreeMap, num::NonZeroUsize, sync::Arc, time::Duration};

/// Stand-in KMS Connector for the PRF endpoint spike. Prints one JSON line per result.
#[derive(Parser, Debug)]
struct Args {
    /// Base URL of each coprocessor's PRF endpoint, comma-separated.
    #[arg(long, env = "PRF_URLS", value_delimiter = ',')]
    urls: Vec<String>,
    /// Registered signer of each coprocessor, in the order of `urls`.
    #[arg(long, env = "PRF_SIGNERS", value_delimiter = ',')]
    signers: Vec<Address>,
    /// KMS tx-sender keys: one connector each.
    #[arg(
        long,
        env = "KMS_TX_SENDER_KEYS",
        value_delimiter = ',',
        hide_env_values = true
    )]
    keys: Vec<String>,
    #[arg(long, env = "PRF_ETHEREUM_CHAIN_ID")]
    chain_id: u64,
    #[arg(long, env = "PRF_PROTOCOL_CONFIG_ADDRESS")]
    protocol_config: Address,
    /// Coprocessor majority threshold. Defaults to N/2+1.
    #[arg(long)]
    threshold: Option<usize>,
    #[arg(long, global = true, value_parser = humantime::parse_duration, default_value = "30s")]
    deadline: Duration,
    #[arg(long, global = true, value_parser = humantime::parse_duration, default_value = "60s")]
    header_validity: Duration,
    #[arg(long, global = true, value_parser = humantime::parse_duration, default_value = "1s")]
    backoff: Duration,
    /// Fallback policy 2: when the GET target's attestation is in the quorum group, wait up to this
    /// long for its body before falling back. Unset: fall back as soon as quorum forms.
    #[arg(long, global = true, value_parser = humantime::parse_duration)]
    body_wait: Option<Duration>,
    /// QA row R7: count the first fallback GET as failed.
    #[arg(long, global = true)]
    fail_first_fallback: bool,
    /// The key that `flood`, `ask` and `probe` sign with, and the first key of `fetch`, so that
    /// rows do not share a budget.
    #[arg(long, global = true, default_value_t = 0)]
    key_index: usize,
    /// Sign with random keys, not registered tx-senders (QA row C2).
    #[arg(long, global = true)]
    random_keys: bool,
    /// Skip TLS verification. rustls refuses the preview's self-signed `CA:TRUE` proxy certificate
    /// as a server certificate (`CaUsedAsEndEntity`); integrity comes from the attestation quorum.
    #[arg(long)]
    accept_invalid_certs: bool,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Retrieves one label with every key at once.
    Fetch {
        #[command(flatten)]
        label: LabelArgs,
        #[arg(long, default_value_t = 64)]
        pad_length: u16,
        #[arg(long, default_value_t = 0)]
        prf: u16,
        /// Index of the coprocessor that gets the GET. Random when unset.
        #[arg(long)]
        get_from: Option<usize>,
        /// Start from headers that expired 10 minutes ago, as from a stale cache.
        #[arg(long)]
        stale_headers: bool,
        /// Use only `connectors` keys, from `--key-index`.
        #[arg(long)]
        connectors: Option<usize>,
        /// Retrieve this many labels one after the other, on the same connections. A new label each
        /// time with `--label-prefix`.
        #[arg(long, default_value_t = 1)]
        repeat: usize,
    },
    /// Sends `count` GETs of distinct labels to one coprocessor at once, with the first key, without
    /// retries. Prints the count of each status.
    Flood {
        #[arg(long)]
        target: usize,
        #[arg(long)]
        count: usize,
        #[arg(long)]
        label_prefix: String,
        #[arg(long, default_value_t = 2048)]
        pad_length: u16,
    },
    /// Sends one request to one coprocessor with the first key, with the retries of the retrieval,
    /// until `--deadline`. Prints the reply report, or `{"error": "deadline"}`.
    Ask {
        #[arg(long)]
        target: usize,
        #[command(flatten)]
        label: LabelArgs,
        /// One request at once for each length, for the same label.
        #[arg(long, value_delimiter = ',', default_value = "64")]
        pad_lengths: Vec<u16>,
        #[arg(long, default_value = "GET")]
        method: Method,
        /// Give up after this long and exit, which closes the socket. Prints the path, to ask again
        /// for the same label.
        #[arg(long, value_parser = humantime::parse_duration)]
        abort_after: Option<Duration>,
        /// Ask this many times on the same connections, `--interval` apart. A new label each time
        /// with `--label-prefix`.
        #[arg(long, default_value_t = 1)]
        repeat: usize,
        #[arg(long, value_parser = humantime::parse_duration, default_value = "0s")]
        interval: Duration,
    },
    /// Waits forever: keeps a pod up for `kubectl exec`, in an image without a shell.
    Idle,
    /// Users arrive over `--window`; for each, every connector from `--key-index` retrieves a new
    /// label. Prints one summary.
    Burst {
        #[arg(long)]
        users: usize,
        #[arg(long, value_parser = humantime::parse_duration)]
        window: Duration,
        #[arg(long, default_value_t = 1)]
        copies: usize,
        #[arg(long, default_value_t = 64)]
        pad_length: u16,
        /// Label first byte, 2 hex characters.
        #[arg(long, default_value = "00")]
        label_prefix: String,
        /// Use only `connectors` keys, from `--key-index`.
        #[arg(long)]
        connectors: Option<usize>,
        /// The first connector starts each label this much before the others.
        #[arg(long, value_parser = humantime::parse_duration, default_value = "0s")]
        lead: Duration,
        /// Ask this one coprocessor instead of retrieving.
        #[arg(long)]
        only: Option<usize>,
        /// Then retry each failed retrieval once, with this deadline.
        #[arg(long, value_parser = humantime::parse_duration)]
        retry_deadline: Option<Duration>,
    },
    /// `--concurrency` loops repeat signed requests for a label each (new labels with
    /// `--label-prefix`: the first request of each evaluates it), for `--duration`.
    Hammer {
        #[arg(long)]
        target: usize,
        #[arg(long)]
        label_prefix: String,
        #[arg(long, default_value_t = 2048)]
        pad_length: u16,
        #[arg(long, default_value = "GET")]
        method: Method,
        #[arg(long)]
        concurrency: usize,
        #[arg(long, value_parser = humantime::parse_duration)]
        duration: Duration,
    },
    /// Holds slow and idle TLS connections to one coprocessor's proxy.
    Slowloris {
        #[arg(long)]
        target: usize,
        #[arg(long)]
        slow: usize,
        #[arg(long)]
        idle: usize,
        #[arg(long, value_parser = humantime::parse_duration)]
        duration: Duration,
        #[arg(long, value_parser = humantime::parse_duration, default_value = "10s")]
        byte_every: Duration,
    },
    /// Prints the lines of a Prometheus page that start with `--prefix`.
    Metrics {
        #[arg(long)]
        url: String,
        #[arg(long, default_value = "")]
        prefix: String,
    },
    /// Sends one signed request with the first key, without retries.
    Probe {
        #[arg(long)]
        target: usize,
        /// The raw path and query.
        #[arg(long)]
        path: String,
        #[arg(long, default_value = "GET")]
        method: Method,
        /// `expires` relative to now, in seconds; negative for the past.
        #[arg(long, allow_hyphen_values = true)]
        expires_in: Option<i64>,
        /// Sign for this audience instead of the target's signer.
        #[arg(long)]
        audience: Option<Address>,
        /// Sign with a random key, not a registered tx-sender.
        #[arg(long)]
        random_key: bool,
        /// Send this body.
        #[arg(long)]
        body: Option<String>,
        /// Sign for this body's digest.
        #[arg(long, default_value = "")]
        signed_body: String,
    },
}

#[derive(clap::Args, Debug)]
#[group(required = true, multiple = false)]
struct LabelArgs {
    /// The label, 64 hex characters.
    #[arg(long)]
    label: Option<B256>,
    /// A random label that starts with this byte (2 hex characters), to select a pad-worker fault.
    #[arg(long)]
    label_prefix: Option<String>,
}

impl LabelArgs {
    fn resolve(&self) -> anyhow::Result<B256> {
        match (self.label, &self.label_prefix) {
            (Some(label), _) => Ok(label),
            (_, Some(prefix)) => label_with_prefix(prefix),
            (None, None) => unreachable!("clap requires one"),
        }
    }
}

fn label_with_prefix(prefix: &str) -> anyhow::Result<B256> {
    let byte = u8::from_str_radix(prefix, 16).context("label prefix")?;
    let mut label = B256::from(rand::random::<[u8; 32]>());
    label.0[0] = byte;
    Ok(label)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .with_max_level(tracing::Level::WARN)
        .init();
    let args = Args::parse();
    if args.urls.len() != args.signers.len() || args.urls.is_empty() {
        bail!("give one signer for each URL");
    }
    let coprocessors: Vec<Coprocessor> = args
        .urls
        .iter()
        .zip(&args.signers)
        .map(|(url, signer)| Coprocessor {
            url: url.trim_end_matches('/').to_owned(),
            signer: *signer,
        })
        .collect();
    let threshold = NonZeroUsize::new(args.threshold.unwrap_or(coprocessors.len() / 2 + 1))
        .context("threshold")?;
    let http = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(3))
        .danger_accept_invalid_certs(args.accept_invalid_certs)
        .build()?;
    let domain = fhevm_sig_auth::domain(args.chain_id, args.protocol_config);
    let connectors: Vec<Arc<Connector>> = args
        .keys
        .iter()
        .map(|key| -> anyhow::Result<_> {
            let key = if args.random_keys {
                PrivateKeySigner::random()
            } else {
                key.trim().parse::<PrivateKeySigner>()?
            };
            Ok(Arc::new(Connector {
                key,
                domain: domain.clone(),
                coprocessors: coprocessors.clone(),
                threshold,
                http: http.clone(),
                header_validity: args.header_validity,
                deadline: args.deadline,
                backoff: args.backoff,
                body_wait: args.body_wait,
                fail_first_fallback: args.fail_first_fallback,
            }))
        })
        .collect::<anyhow::Result<_>>()?;
    let first = connectors
        .get(args.key_index)
        .context("--key-index: no such key")?
        .clone();

    match args.command {
        Command::Fetch {
            label,
            pad_length,
            prf,
            get_from,
            stale_headers,
            connectors: count,
            repeat,
        } => {
            let from = &connectors[args.key_index.min(connectors.len())..];
            let chosen = &from[..count.unwrap_or(from.len()).min(from.len())];
            for _ in 0..repeat {
                let retrieval = Retrieval {
                    prf,
                    label: label.resolve()?,
                    pad_length,
                    get_from,
                    first_expires: stale_headers
                        .then(|| fhevm_sig_auth::unix_now().saturating_sub(600)),
                };
                let reports = join_all(chosen.iter().map(|c| c.retrieve(&retrieval))).await;
                for report in reports {
                    println!("{}", serde_json::to_string(&report)?);
                }
            }
        }
        Command::Flood {
            target,
            count,
            label_prefix,
            pad_length,
        } => {
            let paths = (0..count)
                .map(|_| {
                    label_with_prefix(&label_prefix).map(|label| {
                        format!("/v1/prf/0/labels/{}/{pad_length}", hex::encode(label))
                    })
                })
                .collect::<anyhow::Result<Vec<_>>>()?;
            let plain = ProbeOptions::default();
            let results = join_all(
                paths
                    .iter()
                    .map(|path| first.probe(target, Method::GET, path, &plain)),
            )
            .await;
            let mut statuses = BTreeMap::<String, usize>::new();
            let mut retry_after = None;
            for result in results {
                let key = match result {
                    Ok((status, error, after)) => {
                        retry_after = retry_after.or(after);
                        format!("{status} {}", error.map(|e| e.code).unwrap_or_default())
                    }
                    Err(err) => format!("error {err}"),
                };
                *statuses.entry(key.trim().to_owned()).or_default() += 1;
            }
            println!(
                "{}",
                serde_json::json!({"statuses": statuses, "retry_after": retry_after})
            );
        }
        Command::Ask {
            target,
            label,
            pad_lengths,
            method,
            abort_after,
            repeat,
            interval,
        } => {
            for round in 0..repeat {
                if round > 0 {
                    tokio::time::sleep(interval).await;
                }
                let label = hex::encode(label.resolve()?);
                let asks = pad_lengths.iter().map(|pad_length| {
                    let path = format!("/v1/prf/0/labels/{label}/{pad_length}");
                    ask_and_print(&first, target, method.clone(), path, abort_after)
                });
                for aborted in join_all(asks).await {
                    if aborted? {
                        // Exiting closes the socket.
                        return Ok(());
                    }
                }
            }
        }
        Command::Burst {
            users,
            window,
            copies,
            pad_length,
            label_prefix,
            connectors: count,
            lead,
            only,
            retry_deadline,
        } => {
            let from = &connectors[args.key_index.min(connectors.len())..];
            let chosen = &from[..count.unwrap_or(from.len()).min(from.len())];
            let burst = load::Burst {
                users,
                window,
                copies,
                pad_length,
                label_prefix: u8::from_str_radix(&label_prefix, 16).context("label prefix")?,
                lead,
                only,
                retry_deadline,
            };
            println!(
                "{}",
                serde_json::to_string(&load::burst(chosen, &burst).await)?
            );
        }
        Command::Hammer {
            target,
            label_prefix,
            pad_length,
            method,
            concurrency,
            duration,
        } => {
            let paths = (0..concurrency)
                .map(|_| {
                    label_with_prefix(&label_prefix).map(|label| {
                        format!("/v1/prf/0/labels/{}/{pad_length}", hex::encode(label))
                    })
                })
                .collect::<anyhow::Result<Vec<_>>>()?;
            let report = load::hammer(first, target, method, paths, duration).await;
            println!("{}", serde_json::to_string(&report)?);
        }
        Command::Slowloris {
            target,
            slow,
            idle,
            duration,
            byte_every,
        } => {
            let url = &first
                .coprocessors
                .get(target)
                .context("no such target")?
                .url;
            let report = load::slowloris(url, slow, idle, duration, byte_every).await?;
            println!("{}", serde_json::to_string(&report)?);
        }
        Command::Metrics { url, prefix } => {
            for line in load::metrics(&url, &prefix).await? {
                println!("{line}");
            }
        }
        Command::Idle => std::future::pending::<()>().await,
        Command::Probe {
            target,
            path,
            method,
            expires_in,
            audience,
            random_key,
            body,
            signed_body,
        } => {
            let options = ProbeOptions {
                expires: expires_in
                    .map(|secs| fhevm_sig_auth::unix_now().saturating_add_signed(secs)),
                audience,
                key: random_key.then(PrivateKeySigner::random),
                signed_body: signed_body.into_bytes(),
                body: body.map(String::into_bytes),
            };
            let started = std::time::Instant::now();
            let mut out = match first.probe(target, method, &path, &options).await {
                Ok((status, error, retry_after)) => {
                    serde_json::json!({"status": status, "error": error, "retry_after": retry_after})
                }
                Err(err) => serde_json::json!({"status": 0, "transport_error": format!("{err:#}")}),
            };
            out["elapsed_ms"] = (started.elapsed().as_millis() as u64).into();
            println!("{out}");
        }
    }
    Ok(())
}

/// Asks one coprocessor and prints the reply with its path and elapsed time. Returns true when
/// `abort_after` cut the request, and the caller then exits.
async fn ask_and_print(
    connector: &Connector,
    target: usize,
    method: Method,
    path: String,
    abort_after: Option<Duration>,
) -> anyhow::Result<bool> {
    let started = std::time::Instant::now();
    let asked = connector.ask_one(target, method, &path);
    let reply = match abort_after {
        Some(after) => match tokio::time::timeout(after, asked).await {
            Ok(reply) => reply,
            Err(_) => {
                println!(
                    "{}",
                    serde_json::json!({"aborted_after_ms": after.as_millis() as u64, "path": path})
                );
                return Ok(true);
            }
        },
        None => asked.await,
    };
    let mut out = match reply {
        Some(reply) => serde_json::to_value(&reply)?,
        None => serde_json::json!({"error": "deadline"}),
    };
    out["path"] = path.into();
    out["elapsed_ms"] = (started.elapsed().as_millis() as u64).into();
    println!("{out}");
    Ok(false)
}
