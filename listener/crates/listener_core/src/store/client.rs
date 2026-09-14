use crate::config::config::DatabaseConfig;
use sqlx::Postgres;
use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};
use std::str::FromStr;
use std::time::Duration;
use tracing::info;

pub struct PgClient {
    pool: PgPool,
}

/// Build connection options from `db_url`, applying `db_password` as an override.
///
/// sqlx percent-*decodes* the password it parses out of a URL but never encodes one,
/// so a password embedded in `db_url` must already be percent-encoded. Supplying it via
/// `db_password` skips URL parsing entirely and passes the secret through byte-for-byte.
fn connect_options(config: &DatabaseConfig) -> Result<PgConnectOptions, sqlx::Error> {
    let options = PgConnectOptions::from_str(&config.db_url)?;
    Ok(match &config.db_password {
        Some(password) => options.password(password),
        None => options,
    })
}

impl PgClient {
    /// Create from an existing pool (used by IAM auth path).
    #[cfg(feature = "iam-auth")]
    pub fn from_pool(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn new(config: &DatabaseConfig) -> Result<Self, sqlx::Error> {
        info!("Initializing database connection...");

        let pool = PgPoolOptions::new()
            .max_connections(config.pool.max_connections)
            .min_connections(config.pool.min_connections)
            .acquire_timeout(Duration::from_secs(config.pool.acquire_timeout_secs))
            .idle_timeout(Duration::from_secs(config.pool.idle_timeout_secs))
            .max_lifetime(Duration::from_secs(config.pool.max_lifetime_secs))
            .connect_with(connect_options(config)?)
            .await?;

        // Validate connection
        sqlx::query("SELECT 1").execute(&pool).await?;

        info!(
            "Database pool initialized: min={}, max={}",
            config.pool.min_connections, config.pool.max_connections
        );

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub fn get_pool(&self) -> PgPool {
        self.pool.clone()
    }

    pub async fn acquire(&self) -> Result<PoolConnection<Postgres>, sqlx::Error> {
        self.pool.acquire().await
    }

    pub async fn get_app_connection(&self) -> Result<PoolConnection<Postgres>, sqlx::Error> {
        self.pool.acquire().await
    }

    pub async fn close(&self) {
        self.pool.close().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::config::PoolConfig;
    use sqlx::ConnectOptions;

    fn config_with(db_url: &str, db_password: Option<&str>) -> DatabaseConfig {
        DatabaseConfig {
            db_url: db_url.to_string(),
            db_password: db_password.map(str::to_string),
            iam_auth: None,
            migration_max_attempts: 5,
            pool: PoolConfig::default(),
        }
    }

    /// A password passed via `db_password` must produce exactly the connection state you
    /// would get from a correctly percent-encoded `db_url`. The `/`, `?` and `#` cases
    /// cannot be expressed in a raw URL at all, and `pa%2Fss` is the silent-corruption case.
    #[test]
    fn db_password_matches_correctly_encoded_url() {
        for (raw, encoded) in [
            ("pa/ss", "pa%2Fss"),
            ("pa?ss", "pa%3Fss"),
            ("pa#ss", "pa%23ss"),
            ("pa%2Fss", "pa%252Fss"),
            ("pa@ss", "pa%40ss"),
            ("p:a s&s+", "p%3Aa%20s%26s%2B"),
        ] {
            let overridden =
                connect_options(&config_with("postgres://u@h:5432/d", Some(raw))).unwrap();
            let encoded_url = connect_options(&config_with(
                &format!("postgres://u:{encoded}@h:5432/d"),
                None,
            ))
            .unwrap();

            assert_eq!(
                overridden.to_url_lossy(),
                encoded_url.to_url_lossy(),
                "password {raw:?} did not round-trip"
            );
        }
    }

    /// Raw special characters in `db_url` are exactly the failure this override exists for.
    #[test]
    fn raw_special_chars_in_db_url_still_fail() {
        for raw_url in [
            "postgres://u:pa/ss@h:5432/d",
            "postgres://u:pa?ss@h:5432/d",
            "postgres://u:pa#ss@h:5432/d",
        ] {
            assert!(
                connect_options(&config_with(raw_url, None)).is_err(),
                "expected {raw_url} to be unparseable"
            );
        }
    }

    #[test]
    fn db_password_overrides_password_embedded_in_url() {
        let overridden = connect_options(&config_with(
            "postgres://u:from_url@h:5432/d",
            Some("from_env"),
        ))
        .unwrap();
        let expected =
            connect_options(&config_with("postgres://u:from_env@h:5432/d", None)).unwrap();

        assert_eq!(overridden.to_url_lossy(), expected.to_url_lossy());
    }

    #[test]
    fn db_url_is_untouched_when_no_override_is_set() {
        let options = connect_options(&config_with("postgres://u:s3cret@h:6543/d", None)).unwrap();

        assert_eq!(options.get_username(), "u");
        assert_eq!(options.get_host(), "h");
        assert_eq!(options.get_port(), 6543);
        assert_eq!(options.get_database(), Some("d"));
    }
}
