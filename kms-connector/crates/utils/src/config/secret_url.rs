use crate::conn::REDACTED;
use alloy::transports::http::reqwest::Url;
use serde::Deserialize;
#[cfg(debug_assertions)]
use serde::Serialize;
use std::fmt;

/// A Postgres connection URL, which holds the DB password.
///
/// The [`Display`](fmt::Display) and [`Debug`] implementations print the URL without its
/// password, its query parameters, which may hold one too, and its fragment. They print
/// `<redacted>` for anything but a `postgres://` URL, e.g. a libpq key/value connection string,
/// which may hold a password anywhere, and for a URL whose userinfo was misread.
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
        url.set_query(None);
        url.set_fragment(None);
        write!(f, "{url}")
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
            "postgres://user@db/kms"
        );
        assert_eq!(redacted("postgres://db/kms#frag"), "postgres://db/kms");
        assert_eq!(
            redacted("postgresql://user:pw@[::1]:5432/kms"),
            "postgresql://user@[::1]:5432/kms"
        );
        // Unix socket: no host in the URL.
        assert_eq!(
            redacted("postgres:///kms?host=/var/run/postgresql"),
            "postgres:///kms"
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
        assert_eq!(format!("{db_url:?}"), "DatabaseUrl(postgres://user@db/kms)");
        assert_eq!(
            db_url.as_str(),
            "postgres://user:secret@db/kms?password=secret"
        );
    }
}
