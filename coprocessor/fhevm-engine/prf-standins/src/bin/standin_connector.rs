use alloy::{
    primitives::{Address, B256},
    signers::local::PrivateKeySigner,
};
use anyhow::{bail, Context};
use clap::{Parser, Subcommand};
use futures_util::future::join_all;
use prf_standins::connector::{Connector, Coprocessor, Retrieval};
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
        /// Use only the first `connectors` keys.
        #[arg(long)]
        connectors: Option<usize>,
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
        #[arg(long, default_value_t = 64)]
        pad_length: u16,
        #[arg(long, default_value = "GET")]
        method: Method,
    },
    /// Waits forever: keeps a pod up for `kubectl exec`, in an image without a shell.
    Idle,
    /// Sends one signed request with the first key, without retries.
    Probe {
        #[arg(long)]
        target: usize,
        /// The raw path and query.
        #[arg(long)]
        path: String,
        #[arg(long, default_value = "GET")]
        method: Method,
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
    fn resolve(self) -> anyhow::Result<B256> {
        match (self.label, self.label_prefix) {
            (Some(label), _) => Ok(label),
            (_, Some(prefix)) => label_with_prefix(&prefix),
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
            Ok(Arc::new(Connector {
                key: key.trim().parse::<PrivateKeySigner>()?,
                domain: domain.clone(),
                coprocessors: coprocessors.clone(),
                threshold,
                http: http.clone(),
                header_validity: args.header_validity,
                deadline: args.deadline,
                backoff: args.backoff,
            }))
        })
        .collect::<anyhow::Result<_>>()?;
    let first = connectors.first().context("give at least one key")?.clone();

    match args.command {
        Command::Fetch {
            label,
            pad_length,
            prf,
            get_from,
            stale_headers,
            connectors: count,
        } => {
            let retrieval = Retrieval {
                prf,
                label: label.resolve()?,
                pad_length,
                get_from,
                first_expires: stale_headers
                    .then(|| fhevm_sig_auth::unix_now().saturating_sub(600)),
            };
            let chosen = &connectors[..count.unwrap_or(connectors.len()).min(connectors.len())];
            let reports = join_all(chosen.iter().map(|c| c.retrieve(&retrieval))).await;
            for report in reports {
                println!("{}", serde_json::to_string(&report)?);
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
            let results = join_all(
                paths
                    .iter()
                    .map(|path| first.probe(target, Method::GET, path, None)),
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
            pad_length,
            method,
        } => {
            let path = format!(
                "/v1/prf/0/labels/{}/{pad_length}",
                hex::encode(label.resolve()?)
            );
            match first.ask_one(target, method, &path).await {
                Some(reply) => println!("{}", serde_json::to_string(&reply)?),
                None => println!("{}", serde_json::json!({"error": "deadline", "path": path})),
            }
        }
        Command::Idle => std::future::pending::<()>().await,
        Command::Probe {
            target,
            path,
            method,
        } => {
            let (status, error, retry_after) = first.probe(target, method, &path, None).await?;
            println!(
                "{}",
                serde_json::json!({"status": status, "error": error, "retry_after": retry_after})
            );
        }
    }
    Ok(())
}
