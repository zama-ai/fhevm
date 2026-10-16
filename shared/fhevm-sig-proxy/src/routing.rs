//! Default-deny routing: the configured routes, anything else is rejected before any body byte is
//! read.
//!
//! A path template is literal segments and named parameters, e.g.
//! `/v1/prf/{prf}/labels/{label}/{padLength}`. A parameter matches one non-empty segment other
//! than `.` and `..`, so a path the proxy forwards names no other resource than its template's.
//! The endpoint parses and validates its own parameters.

use crate::{ProxyError, Upstream};
use anyhow::{Context, bail, ensure};
use http::Method;
use std::{collections::HashSet, time::Duration};

/// A route the proxy forwards.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Route {
    /// The methods the route accepts.
    pub methods: Vec<Method>,
    /// The path template, without query.
    pub path: String,
    /// The name of the [`Upstream`] the route is forwarded to.
    pub upstream: String,
    /// The maximum accepted size of a request body; `0` refuses any body.
    pub max_body_bytes: usize,
    /// How long the proxy waits for an endpoint response before answering
    /// [`crate::ProxyError::EndpointTimeout`].
    pub endpoint_response_timeout: Duration,
}

/// The routes, each parsed and resolved to the index of its upstream.
#[derive(Debug)]
pub(crate) struct RouteTable {
    routes: Vec<ParsedRoute>,
    /// A path of the first route accepting `GET`, with each parameter set to `0`. The TLS
    /// listener healthcheck requests it without credentials and expects `401`.
    self_check_path: String,
}

#[derive(Debug)]
pub(crate) struct ParsedRoute {
    pub route: Route,
    /// The index of the route's upstream in [`crate::ProxyConfig::upstreams`].
    pub upstream: usize,
    segments: Vec<Segment>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Segment {
    Literal(String),
    Parameter,
}

impl RouteTable {
    /// Parses the routes and resolves their upstreams. Every upstream must serve a route, and a
    /// route must accept `GET` for the TLS listener healthcheck.
    pub fn new(routes: &[Route], upstreams: &[Upstream]) -> anyhow::Result<Self> {
        ensure!(!routes.is_empty(), "no route configured");
        let mut names = HashSet::new();
        for upstream in upstreams {
            ensure!(
                names.insert(&upstream.name),
                "upstream {} is configured twice",
                upstream.name
            );
            ensure!(
                !upstream.endpoint_addresses.is_empty(),
                "upstream {} has no endpoint address",
                upstream.name
            );
            upstream
                .healthcheck_path
                .parse::<http::Uri>()
                .with_context(|| format!("upstream {}: healthcheck_path", upstream.name))?;
        }

        let mut paths = HashSet::new();
        let mut parsed = Vec::with_capacity(routes.len());
        for route in routes {
            let path = &route.path;
            ensure!(paths.insert(path), "route {path} is configured twice");
            ensure!(!route.methods.is_empty(), "route {path} has no method");
            let upstream = upstreams
                .iter()
                .position(|upstream| upstream.name == route.upstream)
                .with_context(|| format!("route {path}: unknown upstream {}", route.upstream))?;
            let segments = parse_template(path).with_context(|| format!("route {path}"))?;
            parsed.push(ParsedRoute {
                route: route.clone(),
                upstream,
                segments,
            });
        }
        for (index, upstream) in upstreams.iter().enumerate() {
            ensure!(
                parsed.iter().any(|route| route.upstream == index),
                "upstream {} serves no route",
                upstream.name
            );
        }
        let self_check_path = parsed
            .iter()
            .find(|route| route.route.methods.contains(&Method::GET))
            .context("no route accepts GET, as the TLS listener healthcheck needs")?
            .segments
            .iter()
            .map(|segment| match segment {
                Segment::Literal(literal) => format!("/{literal}"),
                Segment::Parameter => "/0".to_owned(),
            })
            .collect();
        Ok(Self {
            routes: parsed,
            self_check_path,
        })
    }

    pub fn self_check_path(&self) -> &str {
        &self.self_check_path
    }

    pub fn get(&self, index: usize) -> &ParsedRoute {
        &self.routes[index]
    }

    /// Matches a request path, without its query, and returns the index of the route found, or
    /// the [`ProxyError::NotFound`] or [`ProxyError::MethodNotAllowed`] to answer.
    pub fn find(&self, method: &Method, path: &str) -> Result<usize, ProxyError> {
        let mut error = ProxyError::NotFound;
        let Some(path) = path.strip_prefix('/') else {
            return Err(error);
        };
        for (index, route) in self.routes.iter().enumerate() {
            if !route.matches(path) {
                continue;
            }
            if route.route.methods.contains(method) {
                return Ok(index);
            }
            error = ProxyError::MethodNotAllowed(route.route.methods.clone());
        }
        Err(error)
    }
}

fn parse_template(path: &str) -> anyhow::Result<Vec<Segment>> {
    let Some(path) = path.strip_prefix('/') else {
        bail!("path must start with '/'");
    };
    path.split('/')
        .map(
            |segment| match segment.strip_prefix('{').and_then(|s| s.strip_suffix('}')) {
                Some(name)
                    if !name.is_empty()
                        && name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_') =>
                {
                    Ok(Segment::Parameter)
                }
                None if !segment.contains(['{', '}']) && names_a_resource(segment) => {
                    Ok(Segment::Literal(segment.to_owned()))
                }
                _ => bail!("invalid segment {segment:?}"),
            },
        )
        .collect()
}

/// Whether a path segment names a resource: it is neither empty nor the current or parent one.
fn names_a_resource(segment: &str) -> bool {
    !matches!(segment, "" | "." | "..")
}

impl ParsedRoute {
    fn matches(&self, path: &str) -> bool {
        let mut parts = path.split('/');
        self.segments.iter().all(|segment| {
            parts.next().is_some_and(|part| match segment {
                Segment::Literal(literal) => literal == part,
                Segment::Parameter => names_a_resource(part),
            })
        }) && parts.next().is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRF_ROUTE: &str = "/v1/prf/{prf}/labels/{label}/{padLength}";

    fn route(methods: &[Method], path: &str, upstream: &str) -> Route {
        Route {
            methods: methods.to_vec(),
            path: path.to_owned(),
            upstream: upstream.to_owned(),
            max_body_bytes: 0,
            endpoint_response_timeout: Duration::from_secs(1),
        }
    }

    fn upstream(name: &str) -> Upstream {
        Upstream {
            name: name.to_owned(),
            endpoint_addresses: vec!["127.0.0.1:1".parse().unwrap()],
            healthcheck_path: "/healthz".to_owned(),
        }
    }

    /// The connector's literal routes, and a route with parameters on a second upstream.
    fn table() -> RouteTable {
        RouteTable::new(
            &[
                route(&[Method::POST], "/v1/public-decrypt", "connector"),
                route(&[Method::POST], "/v1/user-decrypt", "connector"),
                route(&[Method::GET], "/version", "connector"),
                route(&[Method::GET, Method::HEAD], PRF_ROUTE, "prf"),
            ],
            &[upstream("connector"), upstream("prf")],
        )
        .unwrap()
    }

    fn label() -> String {
        "ab".repeat(32)
    }

    #[test]
    fn a_literal_path_matches_exactly() {
        let table = table();
        assert_eq!(table.find(&Method::POST, "/v1/public-decrypt"), Ok(0));
        assert_eq!(table.find(&Method::POST, "/v1/user-decrypt"), Ok(1));
        assert_eq!(table.find(&Method::GET, "/version"), Ok(2));
        for path in [
            "/",
            "/v2/public-decrypt",
            "/v1/public-decrypt/",
            "/v1//public-decrypt",
            "/v1/x/../public-decrypt",
            "/v1/./public-decrypt",
        ] {
            assert_eq!(
                table.find(&Method::POST, path),
                Err(ProxyError::NotFound),
                "{path}"
            );
        }
        assert_eq!(
            table.find(&Method::GET, "/v1/public-decrypt"),
            Err(ProxyError::MethodNotAllowed(vec![Method::POST]))
        );
    }

    #[test]
    fn a_parameter_matches_any_one_segment() {
        let table = table();
        let label = label();
        for method in [Method::GET, Method::HEAD] {
            let path = format!("/v1/prf/0/labels/{label}/2048");
            assert_eq!(table.find(&method, &path), Ok(3));
        }
        for path in [
            format!("/v1/prf/003/labels/{label}/64"),
            format!("/v1/prf/%33/labels/{}/64", label.to_uppercase()),
            "/v1/prf/a.b/labels/..x/...".to_owned(),
        ] {
            assert_eq!(table.find(&Method::GET, &path), Ok(3), "{path}");
        }
        assert_eq!(table.get(3).upstream, 1);
        assert_eq!(table.get(0).upstream, 0);
    }

    #[test]
    fn a_parameter_is_never_empty_or_a_dot_segment() {
        let table = table();
        let label = label();
        for path in [
            format!("/v1/prf//labels/{label}/64"),
            format!("/v1/prf/./labels/{label}/64"),
            "/v1/prf/3/labels/../64".to_owned(),
            format!("/v1/prf/3/labels/{label}/.."),
            format!("/v1/prf/3/labels/{label}/a/b"),
        ] {
            assert_eq!(
                table.find(&Method::GET, &path),
                Err(ProxyError::NotFound),
                "{path}"
            );
        }
    }

    #[test]
    fn other_paths_and_methods_are_refused() {
        let table = table();
        let path = format!("/v1/prf/3/labels/{}/64", label());
        assert_eq!(
            table.find(&Method::POST, &path),
            Err(ProxyError::MethodNotAllowed(vec![
                Method::GET,
                Method::HEAD
            ]))
        );
        for path in [
            "/v1/prf".to_owned(),
            format!("/v1/prf/3/labels/{}/64/", label()),
            format!("/v2/prf/3/labels/{}/64", label()),
            format!("v1/prf/3/labels/{}/64", label()),
        ] {
            assert_eq!(
                table.find(&Method::GET, &path),
                Err(ProxyError::NotFound),
                "{path}"
            );
        }
    }

    #[test]
    fn the_self_check_path_is_the_first_get_route_with_zero_parameters() {
        assert_eq!(table().self_check_path(), "/version");

        let with_parameters = RouteTable::new(
            &[
                route(&[Method::POST], "/v1/public-decrypt", "prf"),
                route(&[Method::GET, Method::HEAD], PRF_ROUTE, "prf"),
            ],
            &[upstream("prf")],
        )
        .unwrap();
        let path = with_parameters.self_check_path();
        assert_eq!(path, "/v1/prf/0/labels/0/0");
        assert_eq!(with_parameters.find(&Method::GET, path), Ok(1));

        let post_only = RouteTable::new(
            &[route(&[Method::POST], "/v1/public-decrypt", "prf")],
            &[upstream("prf")],
        );
        assert!(post_only.is_err());
    }

    #[test]
    fn a_template_has_named_parameters_and_no_dot_segment() {
        assert!(
            RouteTable::new(
                &[route(&[Method::GET], "/v1/{prf_id}", "prf")],
                &[upstream("prf")]
            )
            .is_ok()
        );
        for path in [
            "/v1/{}",
            "/v1/{prf:u16}",
            "/v1/{a-b}",
            "/v1/..",
            "/v1/./x",
            "/v1//x",
            "v1/x",
            "/v1/a{b}",
            "/",
        ] {
            let routes = [route(&[Method::GET], path, "prf")];
            assert!(
                RouteTable::new(&routes, &[upstream("prf")]).is_err(),
                "{path}"
            );
        }
    }

    #[test]
    fn the_routes_and_upstreams_must_be_consistent() {
        let get = |path: &str, upstream: &str| route(&[Method::GET], path, upstream);
        let error = |routes: &[Route], upstreams: &[Upstream]| {
            format!("{:#}", RouteTable::new(routes, upstreams).unwrap_err())
        };
        assert_eq!(error(&[], &[]), "no route configured");
        assert_eq!(
            error(&[get("/a", "x")], &[upstream("y")]),
            "route /a: unknown upstream x"
        );
        assert_eq!(
            error(&[get("/a", "x")], &[upstream("x"), upstream("y")]),
            "upstream y serves no route"
        );
        assert_eq!(
            error(&[get("/a", "x")], &[upstream("x"), upstream("x")]),
            "upstream x is configured twice"
        );
        assert_eq!(
            error(&[get("/a", "x"), get("/a", "x")], &[upstream("x")]),
            "route /a is configured twice"
        );
        assert_eq!(
            error(&[route(&[], "/a", "x")], &[upstream("x")]),
            "route /a has no method"
        );
        let no_address = Upstream {
            endpoint_addresses: vec![],
            ..upstream("x")
        };
        assert_eq!(
            error(&[get("/a", "x")], &[no_address]),
            "upstream x has no endpoint address"
        );
        let bad_healthcheck = Upstream {
            healthcheck_path: "/a b".to_owned(),
            ..upstream("x")
        };
        assert!(
            error(&[get("/a", "x")], &[bad_healthcheck])
                .starts_with("upstream x: healthcheck_path")
        );
    }
}
