use std::str::FromStr;
use std::time::{Duration, SystemTime};

use aws_config::{BehaviorVersion, Region};
use aws_credential_types::{provider::ProvideCredentials, Credentials};
use aws_sigv4::{
    http_request::{self, SignableBody, SignableRequest, SignatureLocation, SigningSettings},
    sign::v4,
};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use sqlx::{Pool, Postgres};
use thiserror::Error;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};
use url::Url;

use crate::utils::DatabaseURL;

const DATABASE_IAM_AUTH_ENABLED: &str = "DATABASE_IAM_AUTH_ENABLED";
const DATABASE_IAM_REGION: &str = "DATABASE_IAM_REGION";
const DATABASE_SSL_ROOT_CERT_PATH: &str = "DATABASE_SSL_ROOT_CERT_PATH";
const DATABASE_URL_ENV: &str = "DATABASE_URL";

const IAM_TOKEN_TTL: Duration = Duration::from_secs(15 * 60);
const IAM_REFRESH_MARGIN: Duration = Duration::from_secs(2 * 60);
const IAM_REFRESH_RETRY_DELAY: Duration = Duration::from_secs(30);

#[derive(Debug, Error)]
pub enum DatabaseConnectionError {
    #[error(transparent)]
    InvalidDatabaseUrl(#[from] sqlx::Error),

    #[error("DATABASE_IAM_AUTH_ENABLED is enabled but DATABASE_URL is missing a host")]
    MissingHost,

    #[error("DATABASE_IAM_AUTH_ENABLED is enabled but DATABASE_URL is missing a username")]
    MissingUsername,

    #[error(
        "DATABASE_IAM_AUTH_ENABLED is enabled but DATABASE_SSL_ROOT_CERT_PATH is not configured"
    )]
    MissingSslRootCertPath,

    #[error("DATABASE_IAM_AUTH_ENABLED is enabled but DATABASE_URL is not configured")]
    MissingDatabaseUrl,

    #[error(
        "DATABASE_IAM_REGION is not set and no AWS region could be resolved from the environment"
    )]
    MissingAwsRegion,

    #[error("failed to resolve AWS credentials for IAM database auth: {0}")]
    AwsCredentials(String),

    #[error("failed to generate an RDS IAM authentication token: {0}")]
    TokenGeneration(String),

    #[error("failed to render a database URL with a generated IAM token: {0}")]
    UrlRendering(String),
}

#[derive(Default)]
pub struct PoolRefreshHandle(Option<CancellationToken>);

impl PoolRefreshHandle {
    fn new(cancel_token: CancellationToken) -> Self {
        Self(Some(cancel_token))
    }

    pub fn cancel(&mut self) {
        if let Some(token) = self.0.take() {
            token.cancel();
        }
    }
}

impl Drop for PoolRefreshHandle {
    fn drop(&mut self) {
        self.cancel();
    }
}

pub async fn connect_options_for_database_url(
    database_url: &DatabaseURL,
) -> Result<PgConnectOptions, DatabaseConnectionError> {
    connect_options_for_database_url_with(database_url, identity_connect_options).await
}

pub async fn connect_pool_with_options(
    database_url: &DatabaseURL,
    pool_options: PgPoolOptions,
    refresh_parent: Option<&CancellationToken>,
) -> Result<(Pool<Postgres>, PoolRefreshHandle), sqlx::Error> {
    connect_pool_with_options_and_connect_options(
        database_url,
        pool_options,
        refresh_parent,
        identity_connect_options,
    )
    .await
}

pub async fn connect_pool_with_options_and_connect_options(
    database_url: &DatabaseURL,
    pool_options: PgPoolOptions,
    refresh_parent: Option<&CancellationToken>,
    connect_options_transform: fn(PgConnectOptions) -> PgConnectOptions,
) -> Result<(Pool<Postgres>, PoolRefreshHandle), sqlx::Error> {
    let auth = DatabaseAuth::from_env(database_url);
    let connect_options = match &auth {
        DatabaseAuth::Static => database_url
            .parse()
            .map(connect_options_transform)
            .map_err(|err| sqlx::Error::Configuration(Box::new(err)))?,
        DatabaseAuth::AwsIam(config) => config
            .connect_options(connect_options_transform)
            .await
            .map_err(|err| sqlx::Error::Configuration(Box::new(err)))?,
    };

    let pool = pool_options.connect_with(connect_options).await?;
    let refresh_handle =
        auth.spawn_pool_refresh_task(pool.clone(), refresh_parent, connect_options_transform);

    Ok((pool, refresh_handle))
}

async fn connect_options_for_database_url_with(
    database_url: &DatabaseURL,
    connect_options_transform: fn(PgConnectOptions) -> PgConnectOptions,
) -> Result<PgConnectOptions, DatabaseConnectionError> {
    match DatabaseAuth::from_env(database_url) {
        DatabaseAuth::Static => database_url
            .parse()
            .map(connect_options_transform)
            .map_err(Into::into),
        DatabaseAuth::AwsIam(config) => config.connect_options(connect_options_transform).await,
    }
}

pub async fn resolve_runtime_database_url(
    database_url: &DatabaseURL,
) -> Result<String, DatabaseConnectionError> {
    match DatabaseAuth::from_env(database_url) {
        DatabaseAuth::Static => Ok(database_url.as_str().to_owned()),
        DatabaseAuth::AwsIam(config) => config.render_database_url().await,
    }
}

pub fn resolve_database_url_from_option(
    database_url: Option<DatabaseURL>,
) -> Result<DatabaseURL, DatabaseConnectionError> {
    match database_url {
        Some(database_url) => Ok(database_url),
        None => match std::env::var(DATABASE_URL_ENV) {
            Ok(database_url) => DatabaseURL::from_str(&database_url).map_err(Into::into),
            Err(_) if env_flag_enabled(DATABASE_IAM_AUTH_ENABLED) => {
                Err(DatabaseConnectionError::MissingDatabaseUrl)
            }
            Err(_) => Ok(DatabaseURL::default()),
        },
    }
}

enum DatabaseAuth {
    Static,
    AwsIam(AwsIamConfig),
}

impl DatabaseAuth {
    fn from_env(database_url: &DatabaseURL) -> Self {
        if env_flag_enabled(DATABASE_IAM_AUTH_ENABLED) {
            Self::AwsIam(AwsIamConfig::from_env(database_url.clone()))
        } else {
            Self::Static
        }
    }

    fn spawn_pool_refresh_task(
        &self,
        pool: Pool<Postgres>,
        refresh_parent: Option<&CancellationToken>,
        connect_options_transform: fn(PgConnectOptions) -> PgConnectOptions,
    ) -> PoolRefreshHandle {
        let Self::AwsIam(config) = self else {
            return PoolRefreshHandle::default();
        };

        let cancel_token = refresh_parent
            .map(CancellationToken::child_token)
            .unwrap_or_else(CancellationToken::new);
        let task_cancel_token = cancel_token.clone();
        let config = config.clone();

        tokio::spawn(async move {
            config
                .refresh_pool_connect_options(pool, task_cancel_token, connect_options_transform)
                .await;
        });

        PoolRefreshHandle::new(cancel_token)
    }
}

#[derive(Clone)]
struct AwsIamConfig {
    database_url: DatabaseURL,
    region_override: Option<String>,
    ssl_root_cert_path: Option<String>,
}

impl AwsIamConfig {
    fn from_env(database_url: DatabaseURL) -> Self {
        Self {
            database_url,
            region_override: std::env::var(DATABASE_IAM_REGION)
                .ok()
                .map(|value| value.trim().to_owned())
                .filter(|value| !value.is_empty()),
            ssl_root_cert_path: std::env::var(DATABASE_SSL_ROOT_CERT_PATH)
                .ok()
                .map(|value| value.trim().to_owned())
                .filter(|value| !value.is_empty()),
        }
    }

    async fn connect_options(
        &self,
        connect_options_transform: fn(PgConnectOptions) -> PgConnectOptions,
    ) -> Result<PgConnectOptions, DatabaseConnectionError> {
        let ssl_root_cert_path = self
            .ssl_root_cert_path
            .as_deref()
            .ok_or(DatabaseConnectionError::MissingSslRootCertPath)?;
        let context = self.build_auth_context().await?;
        let auth_token = generate_rds_iam_token(
            &context.host,
            context.port,
            &context.username,
            &context.region,
            context.credentials,
            SystemTime::now(),
        )?;

        let mut options = self.database_url.parse()?;
        options = options
            .password(&auth_token)
            .ssl_mode(PgSslMode::VerifyFull)
            .ssl_root_cert(ssl_root_cert_path);

        Ok(connect_options_transform(options))
    }

    async fn render_database_url(&self) -> Result<String, DatabaseConnectionError> {
        let ssl_root_cert_path = self
            .ssl_root_cert_path
            .as_deref()
            .ok_or(DatabaseConnectionError::MissingSslRootCertPath)?;
        let context = self.build_auth_context().await?;
        let auth_token = generate_rds_iam_token(
            &context.host,
            context.port,
            &context.username,
            &context.region,
            context.credentials,
            SystemTime::now(),
        )?;

        render_database_url_with_auth_token(
            self.database_url.as_str(),
            &auth_token,
            Some(ssl_root_cert_path),
        )
    }

    async fn refresh_pool_connect_options(
        &self,
        pool: Pool<Postgres>,
        cancel_token: CancellationToken,
        connect_options_transform: fn(PgConnectOptions) -> PgConnectOptions,
    ) {
        let refresh_interval = IAM_TOKEN_TTL
            .checked_sub(IAM_REFRESH_MARGIN)
            .unwrap_or(IAM_TOKEN_TTL);

        loop {
            tokio::select! {
                _ = cancel_token.cancelled() => {
                    return;
                }
                _ = tokio::time::sleep(refresh_interval) => {}
            }

            loop {
                match self.connect_options(connect_options_transform).await {
                    Ok(connect_options) => {
                        pool.set_connect_options(connect_options);
                        info!(database_url = %self.database_url, "Refreshed PostgreSQL IAM auth token");
                        break;
                    }
                    Err(err) => {
                        warn!(
                            error = %err,
                            database_url = %self.database_url,
                            "Failed to refresh PostgreSQL IAM auth token; retrying"
                        );
                        tokio::select! {
                            _ = cancel_token.cancelled() => {
                                return;
                            }
                            _ = tokio::time::sleep(IAM_REFRESH_RETRY_DELAY) => {}
                        }
                    }
                }
            }
        }
    }

    async fn build_auth_context(&self) -> Result<AwsAuthContext, DatabaseConnectionError> {
        let connect_options = self.database_url.parse()?;
        let host = connect_options.get_host().to_owned();
        if host.is_empty() {
            return Err(DatabaseConnectionError::MissingHost);
        }

        let username = connect_options.get_username().to_owned();
        if username.is_empty() {
            return Err(DatabaseConnectionError::MissingUsername);
        }

        let mut loader = aws_config::defaults(BehaviorVersion::latest());
        if let Some(region) = &self.region_override {
            loader = loader.region(Region::new(region.clone()));
        }
        let shared_config = loader.load().await;

        let region = shared_config
            .region()
            .map(|region| region.as_ref().to_owned())
            .ok_or(DatabaseConnectionError::MissingAwsRegion)?;

        let credentials = shared_config
            .credentials_provider()
            .ok_or_else(|| {
                DatabaseConnectionError::AwsCredentials(
                    "no AWS credentials provider is configured".to_owned(),
                )
            })?
            .provide_credentials()
            .await
            .map_err(|err| DatabaseConnectionError::AwsCredentials(err.to_string()))?;

        Ok(AwsAuthContext {
            host,
            port: connect_options.get_port(),
            username,
            region,
            credentials,
        })
    }
}

struct AwsAuthContext {
    host: String,
    port: u16,
    username: String,
    region: String,
    credentials: Credentials,
}

fn env_flag_enabled(name: &str) -> bool {
    std::env::var(name)
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false)
}

fn identity_connect_options(options: PgConnectOptions) -> PgConnectOptions {
    options
}

fn apply_iam_ssl_settings_to_url(url: &mut Url, ssl_root_cert_path: Option<&str>) {
    let existing_pairs: Vec<(String, String)> = url
        .query_pairs()
        .filter(|(name, _)| name != "sslmode" && name != "sslrootcert")
        .map(|(name, value)| (name.into_owned(), value.into_owned()))
        .collect();

    url.set_query(None);
    {
        let mut pairs = url.query_pairs_mut();
        for (name, value) in existing_pairs {
            pairs.append_pair(&name, &value);
        }
        pairs.append_pair("sslmode", "verify-full");
        if let Some(path) = ssl_root_cert_path {
            pairs.append_pair("sslrootcert", path);
        }
    }
}

fn render_database_url_with_auth_token(
    database_url: &str,
    auth_token: &str,
    ssl_root_cert_path: Option<&str>,
) -> Result<String, DatabaseConnectionError> {
    let mut url = Url::parse(database_url)
        .map_err(|err| DatabaseConnectionError::UrlRendering(err.to_string()))?;
    url.set_password(Some(auth_token)).map_err(|_| {
        DatabaseConnectionError::UrlRendering("database URL cannot accept a password".to_owned())
    })?;
    apply_iam_ssl_settings_to_url(&mut url, ssl_root_cert_path);
    Ok(url.to_string())
}

fn generate_rds_iam_token(
    db_hostname: &str,
    port: u16,
    db_username: &str,
    region: &str,
    credentials: Credentials,
    timestamp: SystemTime,
) -> Result<String, DatabaseConnectionError> {
    let mut signing_settings = SigningSettings::default();
    signing_settings.expires_in = Some(IAM_TOKEN_TTL);
    signing_settings.signature_location = SignatureLocation::QueryParams;

    let identity = credentials.into();
    let signing_params = v4::SigningParams::builder()
        .identity(&identity)
        .region(region)
        .name("rds-db")
        .time(timestamp)
        .settings(signing_settings)
        .build()
        .map_err(|err| DatabaseConnectionError::TokenGeneration(err.to_string()))?;

    let url = format!("https://{db_hostname}:{port}/?Action=connect&DBUser={db_username}");
    let signable_request =
        SignableRequest::new("GET", &url, std::iter::empty(), SignableBody::Bytes(&[]))
            .map_err(|err| DatabaseConnectionError::TokenGeneration(err.to_string()))?;

    let (signing_instructions, _signature) =
        http_request::sign(signable_request, &signing_params.into())
            .map_err(|err| DatabaseConnectionError::TokenGeneration(err.to_string()))?
            .into_parts();

    let mut signed_url = Url::parse(&url)
        .map_err(|err| DatabaseConnectionError::TokenGeneration(err.to_string()))?;
    for (name, value) in signing_instructions.params() {
        signed_url.query_pairs_mut().append_pair(name, &value);
    }

    Ok(signed_url.to_string().split_off("https://".len()))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::time::{Duration, UNIX_EPOCH};

    use aws_credential_types::Credentials;

    use super::{
        apply_iam_ssl_settings_to_url, env_flag_enabled, generate_rds_iam_token,
        render_database_url_with_auth_token,
    };

    #[test]
    fn enables_iam_ssl_query_parameters() {
        let mut url = url::Url::parse(
            "postgresql://coprocessor@db.example/coprocessor?application_name=tfhe-worker",
        )
        .unwrap();

        apply_iam_ssl_settings_to_url(&mut url, Some("/etc/ssl/custom.pem"));

        assert_eq!(
            url.as_str(),
            "postgresql://coprocessor@db.example/coprocessor?application_name=tfhe-worker&sslmode=verify-full&sslrootcert=%2Fetc%2Fssl%2Fcustom.pem"
        );
    }

    #[test]
    fn replaces_existing_ssl_query_parameters() {
        let mut url = url::Url::parse(
            "postgresql://coprocessor@db.example/coprocessor?sslmode=disable&sslrootcert=old.pem",
        )
        .unwrap();

        apply_iam_ssl_settings_to_url(&mut url, None);

        assert_eq!(
            url.as_str(),
            "postgresql://coprocessor@db.example/coprocessor?sslmode=verify-full"
        );
    }

    #[test]
    fn parses_env_flags_case_insensitively() {
        std::env::set_var("TEST_DATABASE_FLAG", "YeS");
        assert!(env_flag_enabled("TEST_DATABASE_FLAG"));
        std::env::remove_var("TEST_DATABASE_FLAG");
    }

    #[test]
    fn renders_database_url_with_token_and_tls_settings() {
        let rendered = render_database_url_with_auth_token(
            "postgresql://coprocessor@db.example/coprocessor?application_name=tfhe-worker",
            "db.example:5432/?Action=connect&DBUser=coprocessor&X-Amz-Signature=abc123",
            Some("/etc/ssl/custom.pem"),
        )
        .unwrap();
        let url = url::Url::parse(&rendered).unwrap();
        let params: HashMap<_, _> = url.query_pairs().into_owned().collect();

        assert_eq!(url.username(), "coprocessor");
        assert!(rendered.contains(
            "coprocessor:db.example%3A5432%2F%3FAction%3Dconnect&DBUser%3Dcoprocessor&X-Amz-Signature%3Dabc123@db.example"
        ));
        assert_eq!(
            params.get("application_name"),
            Some(&"tfhe-worker".to_owned())
        );
        assert_eq!(params.get("sslmode"), Some(&"verify-full".to_owned()));
        assert_eq!(
            params.get("sslrootcert"),
            Some(&"/etc/ssl/custom.pem".to_owned())
        );
    }

    #[test]
    fn generates_rds_iam_token_with_expected_query_parameters() {
        let credentials = Credentials::new(
            "AKIDEXAMPLE",
            "secret",
            Some("session-token".to_owned()),
            None,
            "test",
        );

        let token = generate_rds_iam_token(
            "db.example",
            5432,
            "coprocessor",
            "eu-west-2",
            credentials,
            UNIX_EPOCH + Duration::from_secs(1_700_000_000),
        )
        .unwrap();

        assert!(token.starts_with("db.example:5432/?Action=connect&DBUser=coprocessor"));

        let token_url = url::Url::parse(&format!("https://{token}")).unwrap();
        let params: HashMap<_, _> = token_url.query_pairs().into_owned().collect();

        assert_eq!(params.get("Action"), Some(&"connect".to_owned()));
        assert_eq!(params.get("DBUser"), Some(&"coprocessor".to_owned()));
        assert_eq!(
            params.get("X-Amz-Algorithm"),
            Some(&"AWS4-HMAC-SHA256".to_owned())
        );
        assert_eq!(params.get("X-Amz-Expires"), Some(&"900".to_owned()));
        assert_eq!(
            params.get("X-Amz-Security-Token"),
            Some(&"session-token".to_owned())
        );
        assert!(params
            .get("X-Amz-Credential")
            .is_some_and(|value| value.contains("/eu-west-2/rds-db/aws4_request")));
        assert!(params.contains_key("X-Amz-Signature"));
    }
}
