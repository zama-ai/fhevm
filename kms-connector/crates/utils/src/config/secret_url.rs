use alloy::transports::http::reqwest::Url;
use anyhow::bail;
use serde::{Deserialize, Deserializer};
use sqlx::postgres::PgConnectOptions;
use std::{fmt, str::FromStr};

/// What replaces a secret in redacted text.
pub(crate) const REDACTED: &str = "<redacted>";

/// A Postgres connection URL, parsed into the options sqlx connects with.
///
/// [`Display`](fmt::Display) and [`Debug`] print the user, host, port and database it connects
/// to, without the password, e.g. `postgres://user@db:5432/kms`.
#[derive(Clone)]
pub struct DatabaseUrl {
    options: PgConnectOptions,
    /// The configured URL, only used to compare two of them.
    raw: String,
}

impl DatabaseUrl {
    /// Returns the options to connect with, password included.
    pub(crate) fn connect_options(&self) -> &PgConnectOptions {
        &self.options
    }
}

impl FromStr for DatabaseUrl {
    type Err = anyhow::Error;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(raw)?;
        if !matches!(url.scheme(), "postgres" | "postgresql") {
            bail!("expected a `postgres://` URL");
        }
        // An unencoded `/`, `?` or `#` in the password ends the authority early: the `@` before
        // the host then lands in the path, a query key or the fragment, and parts of the password
        // would be read as the host or the database. Query values may hold an `@`, e.g. a user.
        let query_keys = url
            .query()
            .into_iter()
            .flat_map(|query| query.split('&'))
            .map(|param| param.split_once('=').map_or(param, |(key, _)| key));
        if [url.path(), url.fragment().unwrap_or_default()]
            .into_iter()
            .chain(query_keys)
            .any(|part| part.contains('@'))
        {
            bail!(
                "found an `@` in the path, a query key or the fragment: percent-encode it, or the \
                 `/`, `?` or `#` in the password that put it there"
            );
        }
        Ok(Self {
            options: raw.parse()?,
            raw: raw.to_string(),
        })
    }
}

impl<'de> Deserialize<'de> for DatabaseUrl {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e| serde::de::Error::custom(format!("invalid DB URL: {e}")))
    }
}

impl PartialEq for DatabaseUrl {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl fmt::Display for DatabaseUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let options = &self.options;
        write!(f, "postgres://{}@", options.get_username())?;
        match options.get_socket() {
            Some(socket) => write!(f, "{}", socket.display())?,
            None => write!(f, "{}:{}", options.get_host(), options.get_port())?,
        }
        if let Some(database) = options.get_database() {
            write!(f, "/{database}")?;
        }
        Ok(())
    }
}

impl fmt::Debug for DatabaseUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("DatabaseUrl")
            .field(&format_args!("{self}"))
            .finish()
    }
}

/// A RPC node URL. Some providers put the API key in its userinfo, path or query.
///
/// The [`Display`](fmt::Display) and [`Debug`] implementations print the scheme, host and port,
/// and `<redacted>` in place of each other part the URL has, e.g.
/// `https://<redacted>@rpc.example.com/<redacted>?<redacted>`. A malformed URL is rejected at
/// deserialization without being echoed in the error.
#[derive(Clone, PartialEq)]
pub struct RpcUrl(Url);

impl RpcUrl {
    /// Returns the URL with its secrets, to connect to the node.
    pub(crate) fn as_url(&self) -> &Url {
        &self.0
    }
}

impl fmt::Display for RpcUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let url = &self.0;
        let Some(host) = url.host_str() else {
            return f.write_str(REDACTED);
        };
        write!(f, "{}://", url.scheme())?;
        if !url.username().is_empty() || url.password().is_some() {
            write!(f, "{REDACTED}@")?;
        }
        f.write_str(host)?;
        if let Some(port) = url.port() {
            write!(f, ":{port}")?;
        }
        f.write_str("/")?;
        if !matches!(url.path(), "" | "/") {
            f.write_str(REDACTED)?;
        }
        if url.query().is_some() {
            write!(f, "?{REDACTED}")?;
        }
        if url.fragment().is_some() {
            write!(f, "#{REDACTED}")?;
        }
        Ok(())
    }
}

impl fmt::Debug for RpcUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RpcUrl")
            .field(&format_args!("{self}"))
            .finish()
    }
}

impl From<Url> for RpcUrl {
    fn from(value: Url) -> Self {
        Self(value)
    }
}

impl FromStr for RpcUrl {
    type Err = <Url as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Url::parse(s).map(Self)
    }
}

impl<'de> Deserialize<'de> for RpcUrl {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e| serde::de::Error::custom(format!("invalid URL: {e}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn db_url_is_redacted() {
        let redacted = |db_url: &str| db_url.parse::<DatabaseUrl>().unwrap().to_string();
        assert_eq!(
            redacted("postgres://user:secret@db:5432/kms"),
            "postgres://user@db:5432/kms"
        );
        // The query parameters can hold the password, and override the rest.
        assert_eq!(
            redacted(
                "postgres://user@db:5432/kms?password=secret&host=other&port=6543&user=admin&dbname=prod"
            ),
            "postgres://admin@other:6543/prod"
        );
        assert_eq!(
            redacted("postgresql://user:pw@[::1]:5432/kms"),
            "postgres://user@[::1]:5432/kms"
        );
        assert_eq!(
            redacted("postgres:///kms?user=app&host=/var/run/postgresql"),
            "postgres://app@/var/run/postgresql/kms"
        );
        // An unencoded `@` alone is read right: the last `@` of the authority ends the userinfo.
        assert_eq!(
            redacted("postgres://user:p@ss@db:5432/kms"),
            "postgres://user@db:5432/kms"
        );
        // So is one in a query value.
        assert_eq!(
            redacted("postgres://db:5432/kms?user=app@server&application_name=kms@node1"),
            "postgres://app@server@db:5432/kms"
        );
    }

    #[test]
    fn misread_db_urls_are_rejected() {
        for db_url in [
            // Unencoded `/`, `?` and `#` in the password end the userinfo early, also when an `@`
            // before them makes the start of the password look like a host.
            "postgres://user:#secret@db/kms",
            "postgres://user:/secret@db/kms",
            "postgres://user:?secret@db/kms",
            "postgres://user:123#secret@db/kms",
            "postgres://user:12/secret@db/kms",
            "postgres://user:sec@ret/secret@db/kms",
            "postgres://user:sec@ret?secret@db/kms",
            "postgres://user:sec@ret#secret@db/kms",
            // libpq key/value strings, the first of which sqlx would read as a `db:` URL.
            "db:5432 user=admin password=secret",
            "host=db password=secret",
        ] {
            let err = db_url.parse::<DatabaseUrl>().unwrap_err().to_string();
            assert!(!err.contains("secret"), "{db_url}: {err}");
        }
    }

    #[test]
    fn db_url_debug_is_redacted() {
        let db_url: DatabaseUrl = "postgres://user:secret@db:5432/kms?password=secret"
            .parse()
            .unwrap();
        assert_eq!(
            format!("{db_url:?}"),
            "DatabaseUrl(postgres://user@db:5432/kms)"
        );
    }

    #[test]
    fn db_urls_compare_as_configured() {
        let db_url = |raw: &str| raw.parse::<DatabaseUrl>().unwrap();
        assert_eq!(
            db_url("postgres://user:pw@db:5432/kms"),
            db_url("postgres://user:pw@db:5432/kms")
        );
        assert_ne!(
            db_url("postgres://user:pw@db:5432/kms"),
            db_url("postgres://user:other@db:5432/kms")
        );
    }

    #[test]
    fn rpc_url_is_redacted() {
        let redacted = |rpc_url: &str| rpc_url.parse::<RpcUrl>().unwrap().to_string();
        assert_eq!(redacted("http://localhost:8545"), "http://localhost:8545/");
        assert_eq!(
            redacted("https://user:secret@rpc.example.com/v2/secret?key=secret#secret"),
            "https://<redacted>@rpc.example.com/<redacted>?<redacted>#<redacted>"
        );
        assert_eq!(
            redacted("https://:secret@rpc.example.com"),
            "https://<redacted>@rpc.example.com/"
        );
        assert_eq!(
            redacted("https://rpc.example.com:8443/secret/"),
            "https://rpc.example.com:8443/<redacted>"
        );
        assert_eq!(
            redacted("wss://[::1]/?apikey=secret"),
            "wss://[::1]/?<redacted>"
        );
        // No host to keep.
        assert_eq!(redacted("unix:/secret"), "<redacted>");
    }

    #[test]
    fn rpc_url_debug_is_redacted() {
        let rpc_url: RpcUrl = "https://rpc.example.com/v2/secret".parse().unwrap();
        assert_eq!(
            format!("{rpc_url:?}"),
            "RpcUrl(https://rpc.example.com/<redacted>)"
        );
        assert_eq!(rpc_url.as_url().path(), "/v2/secret");
    }

    #[test]
    fn invalid_rpc_url_is_not_echoed() {
        let err = serde_json::from_str::<RpcUrl>(r#""rpc.example.com/v2/secret""#).unwrap_err();
        let err = err.to_string();
        assert!(err.contains("invalid URL"), "{err}");
        assert!(!err.contains("secret"), "{err}");
    }
}
