//! Default-deny routing of the interface: only the routes of [`ROUTES`], anything else is rejected
//! before any prf-service connection.
//!
//! The proxy checks the shape of a path only. The service behind a route checks the encoding of
//! its path parameters.

use http::Method;

/// The routes the proxy forwards.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Route {
    /// RFC-038 PRF outputs, served by prf-service.
    Prf,
}

impl Route {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Prf => "prf",
        }
    }
}

struct RouteEntry {
    route: Route,
    /// The path, where a `{name}` segment matches exactly one non-empty segment.
    pattern: &'static str,
    /// The accepted methods.
    methods: &'static [Method],
}

const ROUTES: [RouteEntry; 1] = [RouteEntry {
    route: Route::Prf,
    pattern: "/v1/prf/{prf}/labels/{label}/{padLength}",
    methods: &[Method::GET, Method::HEAD],
}];

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RouteError {
    /// Unknown path: `404`.
    NotFound,
    /// Known path, wrong method: `405`, with the methods the route accepts.
    MethodNotAllowed(&'static [Method]),
}

impl RouteError {
    /// The value of the `Allow` header RFC 9110 §15.5.6 requires on a `405` response.
    pub fn allow_header(&self) -> Option<String> {
        match self {
            Self::NotFound => None,
            Self::MethodNotAllowed(methods) => Some(
                methods
                    .iter()
                    .map(Method::as_str)
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
        }
    }
}

/// Matches a request against the routes. `path` excludes the query string.
pub fn match_route(method: &Method, path: &str) -> Result<Route, RouteError> {
    let entry = ROUTES
        .iter()
        .find(|entry| matches_pattern(entry.pattern, path))
        .ok_or(RouteError::NotFound)?;
    if entry.methods.contains(method) {
        Ok(entry.route)
    } else {
        Err(RouteError::MethodNotAllowed(entry.methods))
    }
}

fn matches_pattern(pattern: &str, path: &str) -> bool {
    let mut segments = path.split('/');
    let all_match = pattern.split('/').all(|expected| match segments.next() {
        Some(segment) if expected.starts_with('{') => !matches!(segment, "" | "." | ".."),
        Some(segment) => segment == expected,
        None => false,
    });
    all_match && segments.next().is_none()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRF_PATH: &str =
        "/v1/prf/3/labels/00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff/64";

    #[test]
    fn matches_the_prf_route_for_get_and_head() {
        for method in [Method::GET, Method::HEAD] {
            assert_eq!(match_route(&method, PRF_PATH), Ok(Route::Prf));
        }
        // The encoding of a parameter is the service's to check.
        assert_eq!(
            match_route(&Method::GET, "/v1/prf/03/labels/0xAB/%36%34"),
            Ok(Route::Prf)
        );
    }

    #[test]
    fn rejects_everything_else() {
        for path in [
            "/",
            "",
            "/v2/prf/3/labels/ab/64",
            "/v1/prf/3/labels/ab",
            "/v1/prf/3/labels/ab/64/",
            "//v1/prf/3/labels/ab/64",
            "/v1/prf/3/labels/ab/64/..",
            "/v1/prf/3/./labels/ab/64",
            "/v1/prf//labels/ab/64",
            "/v1/prf/./labels/ab/64",
            "/v1/prf/../labels/ab/64",
            "/v1/prf/3/labels/../64",
            "/v1/prf/3/labels/ab/.",
            "/v1/PRF/3/labels/ab/64",
        ] {
            assert_eq!(
                match_route(&Method::GET, path),
                Err(RouteError::NotFound),
                "{path}"
            );
        }
        for method in [Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS] {
            assert_eq!(
                match_route(&method, PRF_PATH),
                Err(RouteError::MethodNotAllowed(&[Method::GET, Method::HEAD]))
            );
        }
    }
}
