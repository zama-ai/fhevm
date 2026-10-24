use alloy::signers::local::PrivateKeySigner;
use clap::Parser;
use prf_standins::{
    faults::Faults,
    pad_worker::{Cost, PadWorker, PadWorkerConfig},
};
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tokio::signal::unix::{signal, SignalKind};
use tokio_util::sync::CancellationToken;

/// Stand-in pad-worker for the PRF endpoint spike.
#[derive(Parser, Debug)]
struct Args {
    #[arg(long, env = "DATABASE_URL")]
    database_url: String,

    /// The coprocessor signing key that signs the attestations.
    #[arg(long, env = "PAD_WORKER_PRIVATE_KEY", hide_env_values = true)]
    private_key: String,

    /// `<label first byte hex>:<fault>`, comma-separated. Faults: `delay=<duration>`,
    /// `wrong_digest`, `unsigned`, `foreign_signer`, `crash_once`, `fail`.
    #[arg(long, env = "PAD_WORKER_FAULTS", default_value = "")]
    faults: Faults,

    /// Rows evaluated at once: the stand-in `PRF_MAX_CONCURRENT`.
    #[arg(long, env = "PAD_WORKER_CONCURRENCY", default_value_t = 16)]
    concurrency: usize,

    #[arg(long, value_parser = humantime::parse_duration, default_value = "10s")]
    claim_timeout: Duration,

    #[arg(long, default_value_t = 3)]
    max_attempts: i32,

    #[arg(long, value_parser = humantime::parse_duration, default_value = "1s")]
    poll_interval: Duration,

    /// Bytes of each PRF block in the stand-in ciphertexts. Same value at every party.
    #[arg(long, env = "PAD_WORKER_BLOCK_BYTES", default_value_t = 32)]
    block_bytes: usize,

    /// Evaluation time of a label without a fault: base + per_bit x padLength.
    #[arg(long, env = "PAD_WORKER_COST_BASE", value_parser = humantime::parse_duration, default_value = "0s")]
    cost_base: Duration,

    #[arg(long, env = "PAD_WORKER_COST_PER_BIT", value_parser = humantime::parse_duration, default_value = "0s")]
    cost_per_bit: Duration,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_ansi(false)
        .init();
    let args = Args::parse();
    let signer: PrivateKeySigner = args.private_key.trim().parse()?;
    let pool = PgPoolOptions::new()
        .max_connections(args.concurrency as u32 + 4)
        .connect(&args.database_url)
        .await?;
    let cancel = CancellationToken::new();
    let mut sigterm = signal(SignalKind::terminate())?;
    let on_signal = cancel.clone();
    tokio::spawn(async move {
        tokio::select! {
            _ = sigterm.recv() => {}
            _ = tokio::signal::ctrl_c() => {}
        }
        on_signal.cancel();
    });
    PadWorker::new(
        pool,
        signer,
        PadWorkerConfig {
            faults: args.faults,
            concurrency: args.concurrency,
            claim_timeout: args.claim_timeout,
            max_attempts: args.max_attempts,
            poll_interval: args.poll_interval,
            block_bytes: args.block_bytes,
            cost: Cost {
                base: args.cost_base,
                per_bit: args.cost_per_bit,
            },
        },
    )
    .run(cancel)
    .await
}
