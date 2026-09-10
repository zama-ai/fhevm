//! Default-deny routing of the interface: exactly three routes, anything else is rejected before
//! any body byte is read.

use http::{Method, StatusCode};
use kms_connector_api::{PUBLIC_DECRYPTION_ROUTE, USER_DECRYPTION_ROUTE, VERSION_ROUTE};

/// The routes the proxy forwards.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Route {
    PublicDecrypt,
    UserDecrypt,
    Version,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RouteError {
    /// Unknown path: `404`.
    NotFound,
    /// Known path, wrong method: `405`, with the method the route actually expects.
    MethodNotAllowed(Method),
}

impl RouteError {
    pub fn http_status(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::MethodNotAllowed(_) => StatusCode::METHOD_NOT_ALLOWED,
        }
    }

    /// The value of the `Allow` header RFC 9110 §15.5.6 requires on a `405` response.
    pub fn allow_header(&self) -> Option<&Method> {
        match self {
            Self::NotFound => None,
            Self::MethodNotAllowed(expected) => Some(expected),
        }
    }
}

/// Matches a request against the routes.
pub fn match_route(method: &Method, path: &str) -> Result<Route, RouteError> {
    let (route, expected) = match path {
        PUBLIC_DECRYPTION_ROUTE => (Route::PublicDecrypt, Method::POST),
        USER_DECRYPTION_ROUTE => (Route::UserDecrypt, Method::POST),
        VERSION_ROUTE => (Route::Version, Method::GET),
        _ => return Err(RouteError::NotFound),
    };
    if *method == expected {
        Ok(route)
    } else {
        Err(RouteError::MethodNotAllowed(expected))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_the_three_v1_routes() {
        assert_eq!(
            match_route(&Method::POST, PUBLIC_DECRYPTION_ROUTE),
            Ok(Route::PublicDecrypt)
        );
        assert_eq!(
            match_route(&Method::POST, USER_DECRYPTION_ROUTE),
            Ok(Route::UserDecrypt)
        );
        assert_eq!(match_route(&Method::GET, VERSION_ROUTE), Ok(Route::Version));
    }

    #[test]
    fn rejects_everything_else() {
        assert_eq!(match_route(&Method::GET, "/"), Err(RouteError::NotFound));
        assert_eq!(
            match_route(&Method::POST, "/v2/public-decrypt"),
            Err(RouteError::NotFound)
        );
        assert_eq!(
            match_route(&Method::POST, "/v1/public-decrypt/"),
            Err(RouteError::NotFound)
        );
        assert_eq!(
            match_route(&Method::GET, PUBLIC_DECRYPTION_ROUTE),
            Err(RouteError::MethodNotAllowed(Method::POST))
        );
        assert_eq!(
            match_route(&Method::POST, VERSION_ROUTE),
            Err(RouteError::MethodNotAllowed(Method::GET))
        );
    }
}
