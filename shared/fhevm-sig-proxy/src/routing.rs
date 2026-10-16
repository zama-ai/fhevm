//! Default-deny routing: the configured routes, anything else is rejected before any body byte is
//! read.
//!
//! A path template is literal segments and typed parameters, e.g.
//! `/v1/prf/{prf:u16}/labels/{label:hex32}/{padLength:u16}`. A parameter accepts exactly one
//! encoding, so each resource has one URL, the one its `FhevmSig` signs: `u16` is decimal without
//! leading zeros, `hex32` is 64 lowercase hexadecimal characters without `0x`.

use crate::Upstream;
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

/// Why a request matches no route.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum RouteError {
    /// Unknown path: `404`.
    NotFound,
    /// Known path, wrong method: `405`, with the methods the route accepts.
    MethodNotAllowed(Vec<Method>),
    /// Known path with a parameter not in its canonical encoding.
    NonCanonical,
}

/// The routes, each parsed and resolved to the index of its upstream.
#[derive(Debug)]
pub(crate) struct RouteTable(Vec<ParsedRoute>);

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
    U16,
    Hex32,
}

impl RouteTable {
    /// Parses the routes and resolves their upstreams. Every upstream must serve a route.
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
        Ok(Self(parsed))
    }

    pub fn get(&self, index: usize) -> &ParsedRoute {
        &self.0[index]
    }

    /// Matches a request path, without its query, and returns the index of the route found.
    pub fn find(&self, method: &Method, path: &str) -> Result<usize, RouteError> {
        let mut error = RouteError::NotFound;
        let Some(path) = path.strip_prefix('/') else {
            return Err(error);
        };
        for (index, route) in self.0.iter().enumerate() {
            match route.shape(path) {
                Shape::Other => {}
                Shape::NonCanonical => {
                    if error == RouteError::NotFound {
                        error = RouteError::NonCanonical;
                    }
                }
                Shape::Canonical if route.route.methods.contains(method) => return Ok(index),
                Shape::Canonical => {
                    error = RouteError::MethodNotAllowed(route.route.methods.clone())
                }
            }
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
                Some(param) => match param.split_once(':').map(|(_, kind)| kind) {
                    Some("u16") => Ok(Segment::U16),
                    Some("hex32") => Ok(Segment::Hex32),
                    _ => bail!("parameter {{{param}}} needs a type, u16 or hex32"),
                },
                None if segment.contains(['{', '}']) || segment.is_empty() => {
                    bail!("invalid segment {segment:?}")
                }
                None => Ok(Segment::Literal(segment.to_owned())),
            },
        )
        .collect()
}

enum Shape {
    Other,
    NonCanonical,
    Canonical,
}

impl ParsedRoute {
    fn shape(&self, path: &str) -> Shape {
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() != self.segments.len() {
            return Shape::Other;
        }
        let mut canonical = true;
        for (part, segment) in parts.iter().zip(&self.segments) {
            match segment {
                Segment::Literal(literal) if literal != part => return Shape::Other,
                Segment::Literal(_) => {}
                Segment::U16 => canonical &= is_canonical_u16(part),
                Segment::Hex32 => canonical &= is_canonical_hex32(part),
            }
        }
        if canonical {
            Shape::Canonical
        } else {
            Shape::NonCanonical
        }
    }
}

fn is_canonical_u16(part: &str) -> bool {
    part.bytes().all(|b| b.is_ascii_digit())
        && (part == "0" || !part.starts_with('0'))
        && part.parse::<u16>().is_ok()
}

fn is_canonical_hex32(part: &str) -> bool {
    part.len() == 64 && part.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRF_ROUTE: &str = "/v1/prf/{prf:u16}/labels/{label:hex32}/{padLength:u16}";

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

    /// The connector's literal routes, and a typed route on a second upstream.
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
        ] {
            assert_eq!(
                table.find(&Method::POST, path),
                Err(RouteError::NotFound),
                "{path}"
            );
        }
        assert_eq!(
            table.find(&Method::GET, "/v1/public-decrypt"),
            Err(RouteError::MethodNotAllowed(vec![Method::POST]))
        );
    }

    #[test]
    fn a_canonical_path_matches_its_route_and_upstream() {
        let table = table();
        for method in [Method::GET, Method::HEAD] {
            let path = format!("/v1/prf/0/labels/{}/2048", label());
            assert_eq!(table.find(&method, &path), Ok(3));
        }
        let path = format!("/v1/prf/65535/labels/{}/1", label());
        assert_eq!(table.find(&Method::GET, &path), Ok(3));
        assert_eq!(table.get(3).upstream, 1);
        assert_eq!(table.get(0).upstream, 0);
    }

    #[test]
    fn each_parameter_has_one_encoding() {
        let table = table();
        let label = label();
        for path in [
            format!("/v1/prf/003/labels/{label}/64"),
            format!("/v1/prf/65536/labels/{label}/64"),
            format!("/v1/prf/%33/labels/{label}/64"),
            format!("/v1/prf/+3/labels/{label}/64"),
            format!("/v1/prf/3/labels/{}/64", label.to_uppercase()),
            format!("/v1/prf/3/labels/0x{}/64", &label[2..]),
            format!("/v1/prf/3/labels/{}/64", &label[2..]),
            format!("/v1/prf/3/labels/{label}/064"),
        ] {
            assert_eq!(
                table.find(&Method::GET, &path),
                Err(RouteError::NonCanonical),
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
            Err(RouteError::MethodNotAllowed(vec![
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
                Err(RouteError::NotFound),
                "{path}"
            );
        }
    }

    #[test]
    fn a_template_needs_typed_parameters() {
        for path in [
            "/v1/{prf}",
            "/v1/{prf:u32}",
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
    }
}
