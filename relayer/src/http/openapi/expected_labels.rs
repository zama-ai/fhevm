//! Per-endpoint error label catalog and sync tests.
//!
//! This module is the single source of truth for which error labels each
//! endpoint *should* produce. The `ExampleInjector` modifier auto-generates
//! named examples from `ERROR_MATRIX`, and the tests here verify they stay
//! in sync with `all_error_labels()` and the generated OpenAPI spec.

// ── Error label definitions ───────────────────────────────────────────────

/// Complete definition of an error label for OpenAPI documentation.
///
/// Each label maps to exactly one HTTP status code and carries metadata
/// used by [`ErrorLabelEnricher`](crate::http::middleware::openapi::ErrorLabelEnricher)
/// to auto-generate the error reference table.
pub struct ErrorLabelDef {
    pub label: &'static str,
    pub http_status: &'static str,
    pub retryable: bool,
    pub action: &'static str,
}

/// All error labels with their metadata.
///
/// Adding a new label? Add it here *and* in [`ERROR_MATRIX`] below.
pub const ERROR_LABEL_DEFS: &[ErrorLabelDef] = &[
    // 400 — client errors (fix and retry)
    ErrorLabelDef {
        label: "malformed_json",
        http_status: "400",
        retryable: false,
        action: "Fix JSON syntax",
    },
    ErrorLabelDef {
        label: "missing_fields",
        http_status: "400",
        retryable: false,
        action: "Add required fields (see `details[]`)",
    },
    ErrorLabelDef {
        label: "validation_failed",
        http_status: "400",
        retryable: false,
        action: "Fix field values (see `details[]`)",
    },
    ErrorLabelDef {
        label: "request_error",
        http_status: "400",
        retryable: false,
        action: "Fix request body",
    },
    ErrorLabelDef {
        label: "host_chain_id_not_supported",
        http_status: "400",
        retryable: false,
        action: "Use supported chain ID",
    },
    ErrorLabelDef {
        label: "not_allowed_on_host_acl",
        http_status: "400",
        retryable: false,
        action: "Contact operator to allowlist contract",
    },
    // 404
    ErrorLabelDef {
        label: "not_found",
        http_status: "404",
        retryable: false,
        action: "Check job ID validity",
    },
    // 429
    ErrorLabelDef {
        label: "rate_limited",
        http_status: "429",
        retryable: true,
        action: "Wait `Retry-After` seconds",
    },
    // 500
    ErrorLabelDef {
        label: "internal_server_error",
        http_status: "500",
        retryable: false,
        action: "Contact operator with `requestId`",
    },
    ErrorLabelDef {
        label: "host_acl_failed",
        http_status: "500",
        retryable: true,
        action: "Retry; if persistent, contact operator",
    },
    // 503 — transient / infrastructure
    ErrorLabelDef {
        label: "readiness_check_timed_out",
        http_status: "503",
        retryable: true,
        action: "Retry with backoff",
    },
    ErrorLabelDef {
        label: "response_timed_out",
        http_status: "503",
        retryable: true,
        action: "Retry with backoff",
    },
    ErrorLabelDef {
        label: "protocol_paused",
        http_status: "503",
        retryable: true,
        action: "Wait for unpause",
    },
    ErrorLabelDef {
        label: "gateway_not_reachable",
        http_status: "503",
        retryable: true,
        action: "Retry with backoff",
    },
    ErrorLabelDef {
        label: "insufficient_balance",
        http_status: "503",
        retryable: false,
        action: "Contact operator",
    },
    ErrorLabelDef {
        label: "insufficient_allowance",
        http_status: "503",
        retryable: false,
        action: "Contact operator",
    },
];

/// Look up a label definition by name.
pub fn label_def(label: &str) -> Option<&'static ErrorLabelDef> {
    ERROR_LABEL_DEFS.iter().find(|d| d.label == label)
}

/// Metadata for an error label: `(http_status, retryable, sdk_action)`.
///
/// Delegates to [`ERROR_LABEL_DEFS`]. Returns a default for unknown labels.
pub fn label_metadata(label: &str) -> (&'static str, bool, &'static str) {
    match label_def(label) {
        Some(def) => (def.http_status, def.retryable, def.action),
        None => ("500", false, "Unknown error"),
    }
}

// ── Endpoint classification helpers ─────────────────────────────────────────

/// Classify an endpoint path into a group for error-label selection.
pub fn endpoint_group(path: &str) -> &'static str {
    if path.contains("keyurl") {
        "keyurl"
    } else if path.contains("input-proof") {
        "input-proof"
    } else {
        // public-decrypt, user-decrypt, delegated-user-decrypt all share
        // the same error surface (host ACL checks, revert classification).
        "decrypt"
    }
}

// ── Declarative error matrix ────────────────────────────────────────────────

/// Each entry maps (endpoint_group, http_status, is_post) to the labels
/// that can appear, with a context-specific example message per label.
///
/// The tuple is `(group, status, is_post, &[(label, example_message)])`.
type MatrixEntry = (
    &'static str,
    &'static str,
    bool,
    &'static [(&'static str, &'static str)],
);
const ERROR_MATRIX: &[MatrixEntry] = &[
    // ── keyurl (GET only, only 500) ─────────────────────────────────
    (
        "keyurl",
        "500",
        false,
        &[("internal_server_error", "Key URL not yet initialized")],
    ),
    // ── POST 400: parse/validation errors ───────────────────────────
    (
        "input-proof",
        "400",
        true,
        &[
            ("malformed_json", "Could not parse request body as JSON"),
            ("missing_fields", "Required fields are missing"),
            ("validation_failed", "Request validation failed"),
            ("request_error", "Invalid request"),
        ],
    ),
    (
        "decrypt",
        "400",
        true,
        &[
            ("malformed_json", "Could not parse request body as JSON"),
            ("missing_fields", "Required fields are missing"),
            ("validation_failed", "Request validation failed"),
            ("request_error", "Invalid request"),
            (
                "host_chain_id_not_supported",
                "Chain ID is not supported by this relayer",
            ),
        ],
    ),
    // ── GET 400: runtime errors surfaced during polling ──────────────
    (
        "input-proof",
        "400",
        false,
        &[("validation_failed", "Signature is invalid")],
    ),
    (
        "decrypt",
        "400",
        false,
        &[
            ("validation_failed", "Signature is invalid"),
            ("not_allowed_on_host_acl", "Contract is not on the host ACL"),
        ],
    ),
    // ── 404: only on GET (polling with unknown job_id) ───────────────
    (
        "input-proof",
        "404",
        false,
        &[("not_found", "Request not found")],
    ),
    (
        "decrypt",
        "404",
        false,
        &[("not_found", "Request not found")],
    ),
    // ── 429: only on POST (rate limiting) ────────────────────────────
    (
        "input-proof",
        "429",
        true,
        &[(
            "rate_limited",
            "Too many requests \u{2014} retry after the Retry-After interval",
        )],
    ),
    (
        "decrypt",
        "429",
        true,
        &[(
            "rate_limited",
            "Too many requests \u{2014} retry after the Retry-After interval",
        )],
    ),
    // ── 500 ──────────────────────────────────────────────────────────
    (
        "input-proof",
        "500",
        true,
        &[("internal_server_error", "Internal server error")],
    ),
    (
        "decrypt",
        "500",
        true,
        &[("internal_server_error", "Internal server error")],
    ),
    (
        "input-proof",
        "500",
        false,
        &[("internal_server_error", "Internal server error")],
    ),
    (
        "decrypt",
        "500",
        false,
        &[
            ("internal_server_error", "Internal server error"),
            ("host_acl_failed", "Failed to check host ACL"),
        ],
    ),
    // ── GET 503: timeout / revert errors during polling ──────────────
    (
        "input-proof",
        "503",
        false,
        &[
            ("response_timed_out", "Request processing timed out"),
            ("protocol_paused", "Protocol is paused"),
            (
                "insufficient_balance",
                "Insufficient balance for transaction",
            ),
            (
                "insufficient_allowance",
                "Insufficient allowance for transaction",
            ),
        ],
    ),
    (
        "decrypt",
        "503",
        false,
        &[
            ("response_timed_out", "Request processing timed out"),
            ("readiness_check_timed_out", "Readiness check timed out"),
            ("protocol_paused", "Protocol is paused"),
            (
                "insufficient_balance",
                "Insufficient balance for transaction",
            ),
            (
                "insufficient_allowance",
                "Insufficient allowance for transaction",
            ),
        ],
    ),
];

// ── Per-endpoint error labels ───────────────────────────────────────────────

/// Error labels possible at each (endpoint, status, method) combination.
///
/// Each tuple is `(label, example_message, dropdown_summary)`.
/// Looks up [`ERROR_MATRIX`] by (group, status, is_post).
pub fn labels_for_status(
    path: &str,
    status: &str,
    is_post: bool,
) -> Vec<(&'static str, &'static str, &'static str)> {
    let group = endpoint_group(path);

    for &(g, s, post, labels) in ERROR_MATRIX {
        if g == group && s == status && post == is_post {
            return labels
                .iter()
                .map(|&(label, msg)| (label, msg, msg))
                .collect();
        }
    }
    vec![]
}

/// Whether this label's example should include a `details` array.
pub fn has_details(label: &str) -> bool {
    label == "validation_failed" || label == "missing_fields"
}

/// Convert a snake_case label to PascalCase (for named example keys).
pub fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::endpoints::v2::types::error::{all_error_labels, UNWIRED_LABELS};

    /// Every label in the catalog must exist in `all_error_labels()`.
    #[test]
    fn catalog_labels_are_in_all_error_labels() {
        let all_labels = all_error_labels();
        let all_paths = [
            "/v2/keyurl",
            "/v2/input-proof",
            "/v2/public-decrypt",
            "/v2/user-decrypt",
            "/v2/delegated-user-decrypt",
        ];
        let all_statuses = ["400", "404", "429", "500", "503"];
        for path in &all_paths {
            for status in &all_statuses {
                for is_post in [true, false] {
                    for (label, _, _) in labels_for_status(path, status, is_post) {
                        assert!(
                            all_labels.contains(&label),
                            "Catalog label {:?} (path={}, status={}, post={}) \
                             is not in all_error_labels()",
                            label,
                            path,
                            status,
                            is_post
                        );
                    }
                }
            }
        }
    }

    /// Every label in `all_error_labels()` must appear in at least one catalog entry.
    ///
    /// Labels in `UNWIRED_LABELS` are exempt (defined but not yet wired to any handler).
    #[test]
    fn all_error_labels_appear_in_catalog() {
        let all_labels = all_error_labels();
        let all_paths = [
            "/v2/keyurl",
            "/v2/input-proof",
            "/v2/public-decrypt",
            "/v2/user-decrypt",
            "/v2/delegated-user-decrypt",
        ];
        let all_statuses = ["400", "404", "429", "500", "503"];

        let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();
        for path in &all_paths {
            for status in &all_statuses {
                for is_post in [true, false] {
                    for (label, _, _) in labels_for_status(path, status, is_post) {
                        seen.insert(label);
                    }
                }
            }
        }

        for label in &all_labels {
            if UNWIRED_LABELS.contains(label) {
                continue;
            }
            assert!(
                seen.contains(label),
                "all_error_labels() has {:?} but no catalog entry produces it",
                label
            );
        }
    }

    /// Every label in `all_error_labels()` must have non-default metadata.
    #[test]
    fn all_labels_have_metadata() {
        for label in &all_error_labels() {
            let (status, _, action) = label_metadata(label);
            assert_ne!(
                (status, action),
                ("500", "Unknown error"),
                "Label {:?} falls through to the default metadata arm — add it to ERROR_LABEL_DEFS",
                label
            );
        }
    }

    /// Catalog placement status must match `label_metadata()` status.
    ///
    /// Catches: label placed under 503 in catalog but `label_metadata` says 400.
    #[test]
    fn catalog_status_matches_metadata_status() {
        let all_paths = [
            "/v2/keyurl",
            "/v2/input-proof",
            "/v2/public-decrypt",
            "/v2/user-decrypt",
            "/v2/delegated-user-decrypt",
        ];
        let all_statuses = ["400", "404", "429", "500", "503"];
        for path in &all_paths {
            for status in &all_statuses {
                for is_post in [true, false] {
                    for (label, _, _) in labels_for_status(path, status, is_post) {
                        let (meta_status, _, _) = label_metadata(label);
                        assert_eq!(
                            meta_status, *status,
                            "Label {:?} is in catalog under status {} \
                             but label_metadata() says {} (path={}, post={})",
                            label, status, meta_status, path, is_post
                        );
                    }
                }
            }
        }
    }

    /// Every label in the matrix must exist in `ERROR_LABEL_DEFS`.
    #[test]
    fn matrix_labels_exist_in_defs() {
        for &(group, status, is_post, labels) in ERROR_MATRIX {
            for &(label, _) in labels {
                assert!(
                    label_def(label).is_some(),
                    "Matrix entry ({}, {}, {}) references {:?} \
                     which is not in ERROR_LABEL_DEFS",
                    group,
                    status,
                    is_post,
                    label
                );
            }
        }
    }

    /// The generated OpenAPI spec must contain exactly the expected named
    /// examples for each error status on every endpoint.
    #[test]
    fn spec_examples_match_catalog() {
        let spec = crate::http::middleware::openapi::build_openapi_doc();

        let paths = spec.paths.paths;
        for (path_str, path_item) in &paths {
            // Only check v2 endpoints
            if !path_str.starts_with("/v2/") {
                continue;
            }

            let operations: Vec<(bool, &utoipa::openapi::path::Operation)> = [
                (true, path_item.post.as_ref()),
                (false, path_item.get.as_ref()),
            ]
            .into_iter()
            .filter_map(|(is_post, op)| op.map(|o| (is_post, o)))
            .collect();

            for (is_post, operation) in operations {
                for (status_str, response_or_ref) in &operation.responses.responses {
                    // Only check error statuses
                    if !["400", "404", "429", "500", "503"].contains(&status_str.as_str()) {
                        continue;
                    }

                    let expected_labels: Vec<&str> =
                        labels_for_status(path_str, status_str, is_post)
                            .iter()
                            .map(|(l, _, _)| *l)
                            .collect();

                    if expected_labels.is_empty() {
                        continue;
                    }

                    // Extract example names from the spec
                    let response = match response_or_ref {
                        utoipa::openapi::RefOr::T(r) => r,
                        _ => continue,
                    };

                    let content = match response.content.get("application/json") {
                        Some(c) => c,
                        None => {
                            panic!(
                                "No application/json content for {} {} {}",
                                path_str,
                                if is_post { "POST" } else { "GET" },
                                status_str
                            );
                        }
                    };

                    let spec_names: std::collections::BTreeSet<String> =
                        content.examples.keys().cloned().collect();

                    let expected_names: std::collections::BTreeSet<String> =
                        expected_labels.iter().map(|l| to_pascal_case(l)).collect();

                    assert_eq!(
                        spec_names,
                        expected_names,
                        "Example names mismatch for {} {} {}\n  spec:     {:?}\n  expected: {:?}",
                        path_str,
                        if is_post { "POST" } else { "GET" },
                        status_str,
                        spec_names,
                        expected_names
                    );
                }
            }
        }
    }
}
