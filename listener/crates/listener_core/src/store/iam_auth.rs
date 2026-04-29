//! IAM role-based RDS authentication.
//!
//! Generates short-lived SigV4 tokens used as PostgreSQL passwords,
//! and refreshes them in a background task before they expire.
//! Connection details (host, port, user, dbname) are parsed from `db_url`.

use crate::config::config::DatabaseConfig;
use anyhow::Context;
use aws_credential_types::provider::ProvideCredentials;
use aws_sigv4::http_request::{SignableBody, SignableRequest, SigningSettings, sign};
use aws_sigv4::sign::v4;
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions, PgSslMode};
use std::time::{Duration, SystemTime};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

/// Refresh every 10 min (token valid for 15 min, 5 min safety buffer).
const TOKEN_REFRESH_INTERVAL: Duration = Duration::from_secs(600);

/// Force connection recycle at 14 min (before 15-min token expiry).
const IAM_MAX_LIFETIME: Duration = Duration::from_secs(840);

/// Validated parameters for IAM-authenticated RDS connections.
///
/// Parsed from `db_url` at startup (`api-parse-dont-validate`).
/// Invalid config fails immediately, not at first token refresh.
#[derive(Debug, Clone)]
struct ConnectParameters {
    host: String,
    port: u16,
    username: String,
    database: String,
    /// `None` = rely on system/rustls trust store, `Some(path)` = custom CA PEM.
    ssl_ca_path: Option<String>,
}

impl ConnectParameters {
    /// Parse connection parameters from a PostgreSQL URL.
    ///
    /// # Errors
    ///
    /// Returns error if the URL is malformed or missing required fields
    /// (host, username, database).
    fn from_db_url(db_url: &str, ssl_ca_path: Option<String>) -> anyhow::Result<Self> {
        let url = url::Url::parse(db_url).context("invalid db_url")?;

        let host = url
            .host_str()
            .filter(|h| !h.is_empty())
            .context("db_url missing host")?
            .to_string();

        let username = {
            let u = url.username();
            anyhow::ensure!(!u.is_empty(), "db_url missing username");
            u.to_string()
        };

        let database = {
            let path = url.path().trim_start_matches('/');
            anyhow::ensure!(!path.is_empty(), "db_url missing database name");
            path.to_string()
        };

        let port = url.port().unwrap_or(5432);

        Ok(Self {
            host,
            port,
            username,
            database,
            ssl_ca_path,
        })
    }

    /// Generate a fresh IAM token and build `PgConnectOptions` with SSL.
    ///
    /// By default, relies on the system/rustls trust store (which includes
    /// Amazon Root CAs). If `ssl_ca_path` is set, uses that PEM file instead
    /// (needed for legacy `rds-ca-2019` or custom CAs).
    ///
    /// # Errors
    ///
    /// Returns error if AWS credentials are unavailable or SigV4 signing fails.
    async fn connect_options(&self) -> anyhow::Result<PgConnectOptions> {
        let token = generate_rds_iam_token(&self.host, self.port, &self.username)
            .await
            .context("IAM token generation failed")?;

        let mut opts = PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .password(&token)
            .database(&self.database)
            .ssl_mode(PgSslMode::VerifyFull);

        if let Some(path) = &self.ssl_ca_path {
            opts = opts.ssl_root_cert(path);
        }

        Ok(opts)
    }
}

/// Create an IAM-authenticated connection pool and spawn the background
/// token refresh task. Returns the pool directly — caller wraps in `PgClient`.
///
/// Connection details (host, port, user, dbname) are parsed from `db_config.db_url`.
/// The password in `db_url` is ignored — replaced by a short-lived IAM token.
///
/// # Errors
///
/// Returns error if URL parsing, initial token generation, or connection fails.
pub async fn connect_iam(
    db_config: &DatabaseConfig,
    cancel: CancellationToken,
) -> anyhow::Result<PgPool> {
    let ssl_ca_path = db_config
        .iam_auth
        .as_ref()
        .and_then(|c| c.ssl_ca_path.clone()); // clone: own the path before ConnectParameters takes ownership

    let params = ConnectParameters::from_db_url(&db_config.db_url, ssl_ca_path)?;

    let opts = params
        .connect_options()
        .await
        .context("initial IAM connection failed")?;

    let pool = PgPoolOptions::new()
        .max_connections(db_config.pool.max_connections)
        .min_connections(db_config.pool.min_connections)
        .acquire_timeout(Duration::from_secs(db_config.pool.acquire_timeout_secs))
        .idle_timeout(Duration::from_secs(db_config.pool.idle_timeout_secs))
        // Override pool.max_lifetime_secs: IAM tokens expire at 15 min,
        // so connections must recycle before that. 14 min is the hard ceiling.
        .max_lifetime(IAM_MAX_LIFETIME)
        .connect_with(opts)
        .await
        .context("failed to connect to RDS with IAM token")?;

    sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .context("IAM-authenticated connection validation failed")?;

    info!(
        host = %params.host,
        "Database pool initialized (IAM): min={}, max={}, max_lifetime={}s",
        db_config.pool.min_connections,
        db_config.pool.max_connections,
        IAM_MAX_LIFETIME.as_secs()
    );

    let refresh_pool = pool.clone(); // clone: PgPool is Arc-backed; refresh task needs its own handle
    tokio::spawn(token_refresh_loop(params, refresh_pool, cancel));

    Ok(pool)
}

async fn token_refresh_loop(params: ConnectParameters, pool: PgPool, cancel: CancellationToken) {
    loop {
        tokio::select! {
            biased;
            _ = cancel.cancelled() => {
                info!("IAM token refresh loop shutting down");
                break;
            }
            _ = tokio::time::sleep(TOKEN_REFRESH_INTERVAL) => {
                match params.connect_options().await {
                    Ok(opts) => {
                        pool.set_connect_options(opts);
                        info!("IAM RDS auth token refreshed for new connections");
                    }
                    Err(e) => {
                        error!(
                            error = %e,
                            "IAM token refresh failed — existing connections still valid"
                        );
                    }
                }
            }
        }
    }
}

/// Generate an IAM RDS auth token (SigV4 pre-signed URL).
///
/// The token is used as the PostgreSQL password. It expires in 15 minutes
/// but existing connections authenticated with it remain valid.
///
/// # Errors
///
/// Returns error if AWS credentials are unavailable (e.g., IRSA not configured,
/// metadata service unreachable) or if SigV4 signing fails.
async fn generate_rds_iam_token(
    db_hostname: &str,
    port: u16,
    db_username: &str,
) -> anyhow::Result<String> {
    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;

    let credentials = config
        .credentials_provider()
        .context("no AWS credentials provider found — is IRSA configured?")?
        .provide_credentials()
        .await
        .context("failed to load AWS credentials")?;

    let identity = credentials.into();
    let region = config
        .region()
        .context("AWS region not configured")?
        .to_string();

    let mut signing_settings = SigningSettings::default();
    signing_settings.expires_in = Some(Duration::from_secs(900));
    signing_settings.signature_location = aws_sigv4::http_request::SignatureLocation::QueryParams;

    let signing_params = v4::SigningParams::builder()
        .identity(&identity)
        .region(&region)
        .name("rds-db")
        .time(SystemTime::now())
        .settings(signing_settings)
        .build()
        .context("failed to build SigV4 signing params")?;

    let url = format!("https://{db_hostname}:{port}/?Action=connect&DBUser={db_username}");

    let signable_request =
        SignableRequest::new("GET", &url, std::iter::empty(), SignableBody::Bytes(&[]))
            .context("failed to create signable request")?;

    let (signing_instructions, _signature) = sign(signable_request, &signing_params.into())
        .context("SigV4 signing failed")?
        .into_parts();

    let mut url = url::Url::parse(&url).context("failed to parse token URL")?;
    for (name, value) in signing_instructions.params() {
        url.query_pairs_mut().append_pair(name, value);
    }

    // Strip "https://" — the token is the signed URL without scheme
    Ok(url.to_string().split_off("https://".len()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_standard_postgres_url() {
        let params = ConnectParameters::from_db_url(
            "postgres://listener_iam@my-db.rds.amazonaws.com:5432/listener",
            None,
        )
        .unwrap();

        assert_eq!(params.host, "my-db.rds.amazonaws.com");
        assert_eq!(params.port, 5432);
        assert_eq!(params.username, "listener_iam");
        assert_eq!(params.database, "listener");
        assert!(params.ssl_ca_path.is_none());
    }

    #[test]
    fn parses_url_with_password_ignores_it() {
        let params =
            ConnectParameters::from_db_url("postgres://user:secret@host:5432/mydb", None).unwrap();

        // Password is ignored — IAM token replaces it
        assert_eq!(params.username, "user");
        assert_eq!(params.host, "host");
        assert_eq!(params.database, "mydb");
    }

    #[test]
    fn default_port_when_omitted() {
        let params = ConnectParameters::from_db_url("postgres://user@host/db", None).unwrap();
        assert_eq!(params.port, 5432);
    }

    #[test]
    fn missing_host_rejected() {
        let result = ConnectParameters::from_db_url("postgres:///db", None);
        assert!(result.is_err());
    }

    #[test]
    fn missing_username_rejected() {
        let result = ConnectParameters::from_db_url("postgres://host/db", None);
        assert!(result.is_err());
    }

    #[test]
    fn missing_database_rejected() {
        let result = ConnectParameters::from_db_url("postgres://user@host", None);
        assert!(result.is_err());
    }

    #[test]
    fn ssl_ca_path_preserved() {
        let params = ConnectParameters::from_db_url(
            "postgres://user@host/db",
            Some("/etc/ssl/rds/ca.pem".into()),
        )
        .unwrap();

        assert_eq!(params.ssl_ca_path.as_deref(), Some("/etc/ssl/rds/ca.pem"));
    }

    #[test]
    fn invalid_url_rejected() {
        let result = ConnectParameters::from_db_url("not-a-url", None);
        assert!(result.is_err());
    }
}
