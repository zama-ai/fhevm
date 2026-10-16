//! Default-deny routing: exactly the configured routes, anything else is rejected before any body
//! byte is read.

use http::{Method, StatusCode};

/// A route the proxy forwards: one method on one exact path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Route {
    pub method: Method,
    pub path: String,
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
pub fn match_route<'a>(
    routes: &'a [Route],
    method: &Method,
    path: &str,
) -> Result<&'a Route, RouteError> {
    let route = routes
        .iter()
        .find(|route| route.path == path)
        .ok_or(RouteError::NotFound)?;
    if *method == route.method {
        Ok(route)
    } else {
        Err(RouteError::MethodNotAllowed(route.method.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn routes() -> Vec<Route> {
        [
            (Method::POST, "/v1/public-decrypt"),
            (Method::POST, "/v1/user-decrypt"),
            (Method::GET, "/version"),
        ]
        .map(|(method, path)| Route {
            method,
            path: path.to_owned(),
        })
        .to_vec()
    }

    #[test]
    fn matches_the_configured_routes() {
        let routes = routes();
        for route in &routes {
            assert_eq!(match_route(&routes, &route.method, &route.path), Ok(route));
        }
    }

    #[test]
    fn rejects_everything_else() {
        let routes = routes();
        assert_eq!(
            match_route(&routes, &Method::GET, "/"),
            Err(RouteError::NotFound)
        );
        assert_eq!(
            match_route(&routes, &Method::POST, "/v2/public-decrypt"),
            Err(RouteError::NotFound)
        );
        assert_eq!(
            match_route(&routes, &Method::POST, "/v1/public-decrypt/"),
            Err(RouteError::NotFound)
        );
        assert_eq!(
            match_route(&routes, &Method::GET, "/v1/public-decrypt"),
            Err(RouteError::MethodNotAllowed(Method::POST))
        );
        assert_eq!(
            match_route(&routes, &Method::POST, "/version"),
            Err(RouteError::MethodNotAllowed(Method::GET))
        );
    }
}
