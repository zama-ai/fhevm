use std::time::Duration;

use fhevm_engine_common::utils::DatabaseURL;

#[derive(Clone, Debug)]
pub struct ConfigSettings {
    /// GCS Postgres URL. coproc-mngr is hosted on the GCS instance.
    pub database_url: DatabaseURL,

    /// Pool size for the GCS DB connection.
    pub database_pool_size: u32,

    /// BCS Postgres URL. Currently only logged - opened in a future iteration
    /// during the SNAPSHOTTING phase to run `pg_dump` and dropped immediately
    /// after.
    pub bcs_database_url: Option<DatabaseURL>,

    /// Inbound Postgres NOTIFY channel that the (future) gw-listener fires
    /// after inserting into `upgrade_events`.
    pub upgrade_event_channel: String,

    /// Polling fallback for the inbound listener. If the LISTEN connection
    /// drops or no NOTIFYs arrive, the loop scans `upgrade_events` for
    /// unhandled rows on this interval.
    pub poll_interval: Duration,

    /// How long the readiness predicate (BCS settled at snapshotBlock) is
    /// retried before giving up and marking the proposal failed.
    pub readiness_timeout: Duration,

    /// Backoff between readiness checks while waiting for BCS to settle.
    pub readiness_poll_interval: Duration,

    /// Health-check HTTP port.
    pub health_check_port: u16,

    /// Prometheus metrics bind address. None disables the metrics server.
    pub metrics_addr: Option<String>,
}
