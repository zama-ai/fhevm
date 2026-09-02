use clap::Parser;
use fhevm_engine_common::drift_revert::WatcherTimeouts;
use fhevm_engine_common::telemetry::MetricsConfig;
use fhevm_engine_common::utils::DatabaseURL;
use tracing::Level;
use uuid::Uuid;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Run the background worker
    #[arg(long)]
    pub run_bg_worker: bool,

    /// Polling interval for the background worker to fetch jobs
    #[arg(long, default_value_t = 1000)]
    pub worker_polling_interval_ms: u64,

    /// Polling interval (ms) for the confidential bridge worker to associate
    /// bridged handles. Minimum 10ms: a smaller value would busy-spin the idle
    /// loop with empty readiness queries.
    #[arg(long, value_parser = clap::value_parser!(u64).range(10..), default_value_t = 1000)]
    pub bridge_polling_interval_ms: u64,

    /// Max bridged-handle pairs the confidential bridge worker associates per transaction
    #[arg(long, value_parser = clap::value_parser!(i64).range(1..), default_value_t = 128)]
    pub bridge_associate_batch_size: i64,

    /// Generate fhe keys and exit
    #[arg(long)]
    pub generate_fhe_keys: bool,

    /// Work items batch size
    #[arg(long, value_parser = parse_positive_i32, default_value_t = 100)]
    pub work_items_batch_size: i32,

    /// Number of dependence chains to fetch per worker
    #[arg(long, value_parser = parse_positive_i32, default_value_t = 20)]
    pub dependence_chains_per_batch: i32,

    /// Acquire and execute independent dependence chains in one worker
    /// schedule. This is enabled by default so DB acquisition and boundary
    /// preparation can overlap FHE execution; set
    /// `FHEVM_DCID_BATCH_EXECUTION=false` only when explicitly investigating
    /// single-DCID scheduling.
    #[arg(long, env = "FHEVM_DCID_BATCH_EXECUTION", default_value_t = true)]
    pub dcid_batch_execution: bool,

    /// Share each bounded work window across acquired DCIDs, so one large
    /// chain cannot monopolize the worker batch while unrelated ready chains
    /// wait. Enabled by default: it is the fairness mitigation for the
    /// head-of-line blocking that `--dcid-batch-execution` introduces, and
    /// shipping the two apart leaves the blocking without its remedy.
    ///
    /// NOTE: this also changes the unit of `--work-items-batch-size`. The
    /// adaptive window counts (dependence chain, transaction) groups; the
    /// non-adaptive one counts computation rows, with no residual row cap. A
    /// value tuned for rows therefore admits roughly
    /// `work_items_batch_size * average rows per transaction` rows here, so
    /// the two flags must be retuned together.
    #[arg(
        long,
        env = "FHEVM_DCID_ADAPTIVE_BATCH_EXECUTION",
        default_value_t = true
    )]
    pub dcid_adaptive_batch_execution: bool,

    /// How many times a RETRYABLE stamp may be re-applied to the same
    /// computation in one lane pass before the row is DEMOTED to the slow
    /// lane.
    ///
    /// Demotion is not condemnation. The row keeps its retryable stamp and
    /// stays pending; it merely stops consuming fast-lane batch slots, and
    /// its chain is allowed to retire so dependents discharge. The slow
    /// sweep re-arms the chain at `SchedulePriority::Slow` and resets this
    /// count, so a transient failure heals on a later pass and a permanent
    /// one costs a bounded trickle instead of a verdict.
    ///
    /// Small on purpose: the old value of 20 existed because exhausting the
    /// budget used to promote the row to TERMINAL, which condemns the whole
    /// downstream cone. Nothing is terminalised for running out of attempts
    /// any more, so there is no reason to be generous.
    #[arg(
        long,
        env = "FHEVM_COMPUTATION_RETRY_DEMOTE_THRESHOLD",
        value_parser = parse_positive_i16,
        default_value_t = 3
    )]
    pub computation_retry_demote_threshold: i16,

    /// Key cache size
    #[arg(long, default_value_t = 32, alias = "tenant-key-cache-size")]
    pub key_cache_size: usize,

    /// Coprocessor FHE processing threads
    #[arg(long, default_value_t = 32)]
    pub coprocessor_fhe_threads: usize,

    /// Maximum time a GPU operation may wait for memory capacity before it
    /// fails (and its batch retries) instead of spinning while holding
    /// resources. CPU builds ignore this setting.
    ///
    /// The effective value is capped at a fraction of `--dcid-ttl-sec`,
    /// because the wait is a blocking loop inside batch execution and the
    /// lease is only renewed between worker cycles: a wait longer than the
    /// lease guarantees the lease lapses mid-batch and another worker
    /// recomputes the chain. This default is chosen to sit UNDER that cap at
    /// the default TTL, so the cap's warning means "this deployment is
    /// misconfigured" rather than firing on every startup.
    #[arg(
        long,
        env = "FHEVM_GPU_MEMORY_RESERVATION_TIMEOUT_MS",
        default_value_t = 20_000
    )]
    pub gpu_memory_reservation_timeout_ms: u64,

    /// Maximum number of concurrent CUDA streams allocated per visible GPU.
    /// CPU builds ignore this setting. Single-stream execution serializes
    /// partition dispatch and measurably regresses block-scoped workloads
    /// versus unbounded main; 16 is the measured plateau on H100.
    #[arg(long, env = "FHEVM_GPU_STREAMS_PER_DEVICE", value_parser = parse_nonzero_usize, default_value_t = 16)]
    pub gpu_streams_per_device: usize,

    /// Tokio Async IO threads
    #[arg(long, default_value_t = 4)]
    pub tokio_threads: usize,

    /// Postgres pool max connections
    #[arg(long, default_value_t = 10)]
    pub pg_pool_max_connections: u32,

    /// Prometheus metrics server address
    #[arg(long, default_value = "0.0.0.0:9100")]
    pub metrics_addr: Option<String>,

    /// Postgres database url. If unspecified DATABASE_URL environment variable is used
    #[arg(long)]
    pub database_url: Option<DatabaseURL>,

    /// tfhe-worker service name in OTLP traces
    #[arg(long, env = "OTEL_SERVICE_NAME", default_value = "tfhe-worker")]
    pub service_name: String,

    /// Worker/replica ID for this worker instance
    /// If not provided, a random UUID will be generated
    /// Used to identify the worker in the dependence_chain table
    #[arg(long, value_parser = clap::value_parser!(Uuid))]
    pub worker_id: Option<Uuid>,

    /// Time-to-live in seconds for dependence chain locks
    /// Defaults to 30 seconds if not provided
    #[arg(long, value_parser = clap::value_parser!(u32), default_value_t = 30)]
    pub dcid_ttl_sec: u32,

    /// If set to true, disable dependence chain ID locking mechanism
    /// Enabling this may lead to multiple workers processing the same dependence chain simultaneously
    /// Useful for fallbacking to non-locking behavior in case of issues with the locking mechanism
    #[arg(long, value_parser = clap::value_parser!(bool), default_value_t = false)]
    pub disable_dcid_locking: bool,

    /// Time slice in seconds for processing each dependence chain
    /// If a worker exceeds this time while processing a dependence chain,
    /// it will release the lock and allow other workers to acquire it
    #[arg(long, default_value_t = 90)]
    pub dcid_timeslice_sec: u32,

    /// Time-to-live in seconds for processed dependence chains
    /// Processed dependence chains older than this TTL will be deleted during idle time
    #[arg(long, default_value_t = 48*60*60)] // Keep dcid not older than 48 hours
    pub processed_dcid_ttl_sec: u32,

    /// Interval in seconds for cleaning up expired dependence chain locks
    #[arg(long, default_value_t = 3600)]
    pub dcid_cleanup_interval_sec: u32,

    /// Maximum number of worker cycles allowed without progress on a
    /// dependence chain
    #[arg(long, value_parser = clap::value_parser!(u32), default_value_t = 2)]
    pub dcid_max_no_progress_cycles: u32,

    /// Number of no-progress DCID releases before ignoring dependence counter
    #[arg(long, value_parser = clap::value_parser!(u32), default_value_t = 100)]
    pub dcid_ignore_dependency_count_threshold: u32,

    /// Minimum age (seconds) of an unowned, dependency-gated dependence
    /// chain before the idle-time repair acquisition may pick it up. The
    /// repair path only takes chains whose gate is provably stale (every
    /// producer chain processed or gone, count never decremented); this age
    /// gate additionally keeps it from racing a listener transaction
    /// mid-arm.
    #[arg(long, default_value_t = 300.0)]
    pub dcid_stale_gate_age_secs: f64,

    /// Log level for the application
    #[arg(
        long,
        value_parser = clap::value_parser!(Level),
        default_value_t = Level::INFO)]
    pub log_level: Level,

    #[arg(long, default_value_t = 8080)]
    pub health_check_port: u16,

    /// Prometheus metrics: coprocessor_rerand_batch_latency_seconds
    #[arg(long, default_value = "0.1:5.0:0.01", value_parser = clap::value_parser!(MetricsConfig))]
    pub metric_rerand_batch_latency: MetricsConfig,

    /// Prometheus metrics: coprocessor_fhe_batch_latency_seconds
    #[arg(long, default_value = "0.2:5.0:0.05", value_parser = clap::value_parser!(MetricsConfig))]
    pub metric_fhe_batch_latency: MetricsConfig,

    /// Liveness override budget for one in-flight worker batch, in seconds:
    /// a batch older than this stops keeping the pod alive, so a genuinely
    /// wedged execution is eventually restarted while per-op progress ticks
    /// keep legitimate long batches healthy.
    #[arg(long, env = "TFHE_WORKER_MAX_BATCH_TTL_SECS", default_value_t = 300)]
    pub max_batch_ttl_secs: u64,

    /// Print the compiled-in coprocessor stack version and exit.
    #[arg(long)]
    pub stack_version: bool,

    /// Not exposed via CLI — `#[arg(skip)]` initializes the field to `WatcherTimeouts::default()`
    /// on `Args::parse()`.
    #[arg(skip)]
    pub drift_revert_watcher_timeouts: WatcherTimeouts,
}

fn parse_positive_i32(value: &str) -> Result<i32, String> {
    let parsed = value
        .parse::<i32>()
        .map_err(|error| format!("must be a positive integer: {error}"))?;
    if parsed < 1 {
        Err("must be at least 1".to_owned())
    } else {
        Ok(parsed)
    }
}

/// Selection requires `error_retry_count < threshold` and a re-arm resets the
/// count to zero, so a non-positive threshold excludes a retryable row the
/// moment it is stamped and keeps excluding it after every sweep — the row is
/// demoted forever and the sweep churns it. Reject the configuration at
/// startup rather than let it silently strand work.
fn parse_positive_i16(value: &str) -> Result<i16, String> {
    let parsed = value
        .parse::<i16>()
        .map_err(|error| format!("must be a positive integer: {error}"))?;
    if parsed < 1 {
        Err("must be at least 1".to_owned())
    } else {
        Ok(parsed)
    }
}

fn parse_nonzero_usize(value: &str) -> Result<usize, String> {
    let parsed = value
        .parse::<usize>()
        .map_err(|error| format!("must be a positive integer: {error}"))?;
    if parsed == 0 {
        Err("must be at least 1".to_owned())
    } else {
        Ok(parsed)
    }
}

pub fn parse_args() -> Args {
    fhevm_engine_common::handle_stack_version_flag();
    let args = Args::parse();
    // Set global configs from args
    let _ = scheduler::RERAND_LATENCY_BATCH_HISTOGRAM_CONF.set(args.metric_rerand_batch_latency);
    let _ = scheduler::FHE_BATCH_LATENCY_HISTOGRAM_CONF.set(args.metric_fhe_batch_latency);
    args
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn dcid_batch_execution_defaults_to_enabled() {
        let command = Args::command();
        let argument = command
            .get_arguments()
            .find(|argument| argument.get_id() == "dcid_batch_execution")
            .expect("dcid batch execution argument is present");
        assert_eq!(argument.get_default_values(), ["true"]);
    }

    #[test]
    fn dcid_adaptive_batch_execution_defaults_to_enabled() {
        let command = Args::command();
        let argument = command
            .get_arguments()
            .find(|argument| argument.get_id() == "dcid_adaptive_batch_execution")
            .expect("adaptive dcid batch execution argument is present");
        assert_eq!(argument.get_default_values(), ["true"]);
    }

    /// The pairing is the point: batching creates the head-of-line blocking
    /// that the adaptive window exists to mitigate, so shipping one without
    /// the other is the configuration this test exists to prevent.
    #[test]
    fn batching_and_its_fairness_mitigation_ship_together() {
        let command = Args::command();
        let default_of = |id: &str| {
            command
                .get_arguments()
                .find(|argument| argument.get_id() == id)
                .unwrap_or_else(|| panic!("{id} argument is present"))
                .get_default_values()
                .to_vec()
        };
        assert_eq!(
            default_of("dcid_batch_execution"),
            default_of("dcid_adaptive_batch_execution")
        );
    }

    /// The chart is the infra team's only view of what a worker runs with, so
    /// a flag or a default that exists only in this file is a hidden default.
    /// Every argument must appear in the `tfheWorker.extraArgs` documentation
    /// block of `charts/coprocessor/values.yaml`, spelled the way it would be
    /// passed:
    ///
    ///   * an argument that takes a value appears as `--flag=<default>`, with
    ///     the compiled default, or `--flag=<...>`/`<url>`/`<uuid>` when it has
    ///     none;
    ///   * a presence-only switch appears bare, because clap REJECTS
    ///     `--switch=false` and the worker exits on the unknown value. Writing
    ///     `--switch=<default>` in the chart would document a line that cannot
    ///     be pasted into `extraArgs`.
    ///
    /// Skipped when the chart is not on disk, so the crate stays testable from
    /// a source tarball or a build context that excludes `charts/`.
    #[test]
    fn chart_documents_every_cli_flag_and_default() {
        let chart = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../charts/coprocessor/values.yaml");
        let Ok(values) = std::fs::read_to_string(&chart) else {
            eprintln!("skipping: {} not present", chart.display());
            return;
        };
        // Only the tfhe_worker block: other components document flags of the
        // same name (sns_worker also has --work-items-batch-size).
        let block = values
            .split_once("\ntfheWorker:")
            .expect("chart has a tfheWorker section")
            .1;
        let block = block
            .split_once("\n  extraArgs:")
            .expect("tfheWorker documents extraArgs")
            .0;

        let command = Args::command();
        let mut missing = Vec::new();
        for argument in command.get_arguments() {
            let id = argument.get_id().as_str();
            if id == "help" || id == "version" {
                continue;
            }
            let long = argument
                .get_long()
                .unwrap_or_else(|| panic!("{id} is a long argument"));
            let presence_only = matches!(
                argument.get_action(),
                clap::ArgAction::SetTrue | clap::ArgAction::SetFalse
            );
            let defaults = argument.get_default_values();

            if presence_only {
                // Bare mention, and never an `=` form the CLI would reject.
                let bare = format!("--{long}\n");
                let padded = format!("--{long} ");
                if !block.contains(&bare) && !block.contains(&padded) {
                    missing.push(format!("--{long} (presence-only switch, not documented)"));
                } else if block.contains(&format!("--{long}=")) {
                    missing.push(format!(
                        "--{long} is documented in `=value` form, which clap rejects"
                    ));
                }
                continue;
            }

            let expected = match defaults.first() {
                Some(default) => format!("--{long}={}", default.to_string_lossy()),
                // No compiled default: any documented placeholder will do, as
                // long as the flag is shown taking a value.
                None => format!("--{long}=<"),
            };
            if !block.contains(&expected) {
                missing.push(format!("{expected} (not documented with this default)"));
            }
        }

        assert!(
            missing.is_empty(),
            "charts/coprocessor/values.yaml tfheWorker.extraArgs is out of sync \
             with the tfhe_worker CLI; hidden defaults:\n  {}",
            missing.join("\n  ")
        );
    }

    /// The GPU reservation wait is capped at a fraction of the DCID lease at
    /// startup. A default above that cap is unreachable: it would be rewritten
    /// on every GPU worker boot, so the advertised default would describe a
    /// configuration nothing ever runs and the cap's warning would carry no
    /// signal. Keep the default inside the cap the shipped TTL implies.
    #[test]
    fn gpu_reservation_timeout_default_is_reachable_under_the_default_lease() {
        let args = Args::parse_from(["tfhe_worker"]);
        let cap_ms = (f64::from(args.dcid_ttl_sec)
            * f64::from(crate::tfhe_worker::GPU_RESERVATION_LEASE_FRACTION)
            * 1000.0) as u64;
        assert!(
            args.gpu_memory_reservation_timeout_ms <= cap_ms,
            "default --gpu-memory-reservation-timeout-ms ({}) exceeds the \
             {cap_ms} ms cap implied by --dcid-ttl-sec ({})",
            args.gpu_memory_reservation_timeout_ms,
            args.dcid_ttl_sec
        );
    }

    /// A non-positive demotion threshold makes `error_retry_count < threshold`
    /// unsatisfiable, so a retryable row leaves the work window the moment it
    /// is stamped and stays out after a sweep resets its count to zero. That
    /// is indistinguishable from losing the work, so it is refused at startup.
    #[test]
    fn demotion_threshold_rejects_non_positive_values() {
        for value in ["0", "-1"] {
            assert!(
                Args::try_parse_from([
                    "tfhe_worker",
                    &format!("--computation-retry-demote-threshold={value}"),
                ])
                .is_err(),
                "--computation-retry-demote-threshold={value} must be rejected"
            );
        }
        let args = Args::parse_from(["tfhe_worker", "--computation-retry-demote-threshold=1"]);
        assert_eq!(args.computation_retry_demote_threshold, 1);
    }
}
