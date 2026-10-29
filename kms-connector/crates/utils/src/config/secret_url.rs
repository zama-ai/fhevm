use alloy::transports::http::reqwest::Url;
#[cfg(debug_assertions)]
use serde::Serialize;
use serde::{Deserialize, Deserializer};
use std::{fmt, str::FromStr};

/// What replaces a secret in redacted text.
pub(crate) const REDACTED: &str = "<redacted>";

/// A Postgres connection URL, which holds the DB password.
///
/// The [`Display`](fmt::Display) and [`Debug`] implementations print the URL without its
/// password and its fragment, and with `<redacted>` in place of its query parameters, which may
/// hold a password too, or override the host, port, user or database. They print `<redacted>`
/// for anything but a `postgres://` URL, e.g. a libpq key/value connection string, which may hold
/// a password anywhere, and for a URL whose userinfo was misread.
#[derive(Clone, Deserialize, PartialEq)]
#[cfg_attr(debug_assertions, derive(Serialize))]
#[serde(transparent)]
pub struct DatabaseUrl(String);

impl DatabaseUrl {
    /// Returns the URL with its password, to connect to the DB.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DatabaseUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Ok(mut url) = Url::parse(&self.0) else {
            return f.write_str(REDACTED);
        };
        let is_postgres = matches!(url.scheme(), "postgres" | "postgresql");
        // An unencoded `/`, `?` or `#` in the password ends the authority early: the `@` before
        // the host then lands in the path, query or fragment, and parts of the password are read
        // as the host, port, path, query or fragment.
        let userinfo_misread = [Some(url.path()), url.query(), url.fragment()]
            .into_iter()
            .flatten()
            .any(|part| part.contains('@'));
        // `set_password` fails for URLs without a host.
        let password_kept = url.password().is_some() && url.set_password(None).is_err();
        if !is_postgres || userinfo_misread || password_kept {
            return f.write_str(REDACTED);
        }
        let has_query = url.query().is_some();
        url.set_query(None);
        url.set_fragment(None);
        write!(f, "{url}")?;
        if has_query {
            write!(f, "?{REDACTED}")?;
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

impl From<&str> for DatabaseUrl {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

/// A RPC node URL. Some providers put the API key in its userinfo, path or query.
///
/// The [`Display`](fmt::Display) and [`Debug`] implementations print the scheme, host and port,
/// and `<redacted>` in place of each other part the URL has, e.g.
/// `https://<redacted>@rpc.example.com/<redacted>?<redacted>`. A malformed URL is rejected at
/// deserialization without being echoed in the error.
#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Serialize), serde(transparent))]
pub struct RpcUrl(Url);

impl RpcUrl {
    /// Returns the URL with its secrets, to connect to the node.
    pub fn as_url(&self) -> &Url {
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
        let redacted = |db_url: &str| DatabaseUrl::from(db_url).to_string();
        assert_eq!(
            redacted("postgres://user:secret@db:5432/kms"),
            "postgres://user@db:5432/kms"
        );
        // libpq also reads the password from the query parameters.
        assert_eq!(
            redacted("postgres://user@db/kms?sslmode=require&password=secret"),
            "postgres://user@db/kms?<redacted>"
        );
        assert_eq!(redacted("postgres://db/kms#frag"), "postgres://db/kms");
        assert_eq!(
            redacted("postgresql://user:pw@[::1]:5432/kms"),
            "postgresql://user@[::1]:5432/kms"
        );
        // Unix socket: no host in the URL.
        assert_eq!(
            redacted("postgres:///kms?host=/var/run/postgresql"),
            "postgres:///kms?<redacted>"
        );
        // An unencoded `@` alone is read right: the last `@` of the authority ends the userinfo.
        assert_eq!(
            redacted("postgres://user:p@ss@db/kms"),
            "postgres://user@db/kms"
        );
        // Unencoded `/`, `?` and `#` in the password end the userinfo early, also when an `@`
        // before them makes the start of the password look like a host.
        for db_url in [
            "postgres://user:#secret@db/kms",
            "postgres://user:/secret@db/kms",
            "postgres://user:?secret@db/kms",
            "postgres://user:123#secret@db/kms",
            "postgres://user:12/secret@db/kms",
            "postgres://user:sec@ret/secret@db/kms",
            "postgres://user:sec@ret?secret@db/kms",
            "postgres://user:sec@ret#secret@db/kms",
        ] {
            assert_eq!(redacted(db_url), "<redacted>", "{db_url}");
        }
        // A libpq key/value string parses as a URL with a `db:` scheme.
        assert_eq!(redacted("db:5432 user=admin password=secret"), "<redacted>");
        assert_eq!(redacted("host=db password=secret"), "<redacted>");
    }

    #[test]
    fn db_url_debug_is_redacted() {
        let db_url = DatabaseUrl::from("postgres://user:secret@db/kms?password=secret");
        assert_eq!(
            format!("{db_url:?}"),
            "DatabaseUrl(postgres://user@db/kms?<redacted>)"
        );
        assert_eq!(
            db_url.as_str(),
            "postgres://user:secret@db/kms?password=secret"
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
