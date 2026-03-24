#[allow(unused_imports)]
// These are used by the OpenAPI macro but not detected by the compiler
use crate::http::endpoints::health::{
    __path_health_handler, __path_liveness_handler, __path_version_handler,
};
use crate::http::endpoints::v2::types::{
    error::{
        RelayerV2ErrorDetail, RelayerV2ResponseFailed, V2ApiError, V2ApiErrorWithDetails,
        V2ErrorLabel, V2ErrorResponseBody, V2StatusFailed, V2StatusQueued,
    },
    input_proof::{
        InputProofPostResponseJson as InputProofPostResponseJsonV2,
        InputProofQueuedResult as InputProofQueuedResultV2,
        InputProofRequestJson as InputProofRequestJsonV2,
        InputProofResponseJson as InputProofResponseJsonV2,
        InputProofSucceededStatusResponse as InputProofSucceededStatusResponseV2,
    },
    keyurl::{
        FheKeyInfo as FheKeyInfoV2, KeyData as KeyDataV2,
        KeyUrlResponseJson as KeyUrlResponseJsonV2, Response as KeyUrlResponseV2,
    },
    public_decrypt::{
        PublicDecryptPostResponseJson as PublicDecryptPostResponseJsonV2,
        PublicDecryptQueuedResult as PublicDecryptQueuedResultV2,
        PublicDecryptRequestJson as PublicDecryptRequestJsonV2,
        PublicDecryptResponseJson as PublicDecryptResponseJsonV2,
        PublicDecryptSucceededStatusResponse as PublicDecryptSucceededStatusResponseV2,
    },
    user_decrypt::{
        DelegatedUserDecryptRequestJson as DelegatedUserDecryptRequestJsonV2,
        UserDecryptPostResponseJson as UserDecryptPostResponseJsonV2,
        UserDecryptQueuedResult as UserDecryptQueuedResultV2,
        UserDecryptRequestJson as UserDecryptRequestJsonV2,
        UserDecryptResponseJson as UserDecryptResponseJsonV2,
        UserDecryptResponsePayloadJson as UserDecryptResponsePayloadJsonV2,
        UserDecryptSucceededStatusResponse as UserDecryptSucceededStatusResponseV2,
    },
};
use crate::http::openapi::expected_labels::{
    has_details, labels_for_status, to_pascal_case, ERROR_LABEL_DEFS,
};
use axum::Router;
use utoipa::openapi::RefOr;
use utoipa::{Modify, OpenApi};
use utoipa_redoc::{Redoc, Servable};

// OpenAPI documentation
//
// The `info.description` markdown is inlined here because utoipa's `#[openapi(...)]`
// attribute macro only accepts string literals — `include_str!` is not supported
// inside the macro invocation.
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Relayer API",
        description = r#"
# Relayer API

Relayer provides an interface to request input proofs and decryptions for ciphertexts on the Zama
Gateway Chain.

## Overview

### Reading FHE Public Key URLs

| Endpoint  | Method | Description                      |
|-----------|--------|----------------------------------|
| Key URL   | `GET`  | Returns FHE public key URLs immediately |

### Ciphertext Operations (POST/GET polling)

| Endpoint                 | POST to submit | GET to poll              |
|--------------------------|----------------|--------------------------|
| Input Proof              | `/v2/input-proof` | `/v2/input-proof/{jobId}` |
| User Decrypt             | `/v2/user-decrypt` | `/v2/user-decrypt/{jobId}` |
| Delegated User Decrypt   | `/v2/delegated-user-decrypt` | `/v2/delegated-user-decrypt/{jobId}` |
| Public Decrypt           | `/v2/public-decrypt` | `/v2/public-decrypt/{jobId}` |

**Polling flow:**

1. `POST /v2/{endpoint}` — returns `202` with a `jobId`.
2. `GET /v2/{endpoint}/{jobId}` — returns `202` + `Retry-After` while processing.
3. Final `GET` — returns `200` with the result, or `4xx`/`5xx` with an `error.label`.

## Authentication

Pass the `X-API-Key` header with every request. Optional on testnet, required on mainnet.

## Response Format

All responses share a common envelope:

```json
{
  "status": "queued" | "succeeded" | "failed",
  "requestId": "uuid",
  "result": { ... },
  "error": { "label": "...", "message": "...", "details": [...] }
}
```

| Field     | Present when                                          |
|-----------|-------------------------------------------------------|
| `status`  | Always                                                |
| `result`  | Success (`200`)                                       |
| `error`   | Failure — match on `error.label` (see Error Handling) |
| `details` | Validation errors (`400`) with per-field issues       |

The `Retry-After` header appears in two contexts:

| Context    | Status | Meaning                              |
|------------|--------|--------------------------------------|
| Polling    | 202    | Suggested interval before next GET   |
| Rate limit | 429    | Minimum wait before retrying POST    |

Always respect this header to avoid unnecessary requests.

<!-- ERROR_LABEL_TABLE -->

## Conventions

| Value                | Format                                |
|----------------------|---------------------------------------|
| Ethereum addresses   | `0x` + 40 hex chars                   |
| Ciphertext handles   | `0x` + 64 hex chars                   |
| Extra data           | Always `"0x00"`                       |
| Signatures (request) | Raw hex, 130 chars, no `0x` prefix    |
| Public keys          | Raw hex, min 2 chars, no `0x` prefix  |
"#,
        version = env!("CARGO_PKG_VERSION"),
        license(name = "BSD-3-Clause-Clear", url = "https://opensource.org/licenses/BSD-3-Clause-Clear"),
        contact(name = "Zama", url = "https://www.zama.ai")
    ),
    servers(
        (url = "/", description = "Current server"),
        (url = "http://localhost:3000", description = "Local development")
    ),
paths(
    // Key URL
    crate::http::endpoints::v2::handlers::keyurl::keyurl_v2,
    // Input Proof
    crate::http::endpoints::v2::handlers::input_proof::input_proof_post_v2,
    crate::http::endpoints::v2::handlers::input_proof::input_proof_get_v2,
    // User Decrypt
    crate::http::endpoints::v2::handlers::user_decrypt::user_decrypt_post_v2,
    crate::http::endpoints::v2::handlers::user_decrypt::user_decrypt_get_v2,
    // Delegated User Decrypt
    crate::http::endpoints::v2::handlers::user_decrypt::delegated_user_decrypt_post_v2,
    crate::http::endpoints::v2::handlers::user_decrypt::delegated_user_decrypt_get_v2,
    // Public Decrypt
    crate::http::endpoints::v2::handlers::public_decrypt::public_decrypt_post_v2,
    crate::http::endpoints::v2::handlers::public_decrypt::public_decrypt_get_v2,
    // Health
    crate::http::endpoints::health::liveness_handler,
    crate::http::endpoints::health::health_handler,
    crate::http::endpoints::health::version_handler,
),
components(
    // Input Proof types
    schemas(InputProofRequestJsonV2, InputProofResponseJsonV2, InputProofPostResponseJsonV2, InputProofQueuedResultV2),
    schemas(InputProofSucceededStatusResponseV2),
    // Public Decrypt types
    schemas(PublicDecryptRequestJsonV2, PublicDecryptResponseJsonV2, PublicDecryptPostResponseJsonV2, PublicDecryptQueuedResultV2),
    schemas(PublicDecryptSucceededStatusResponseV2),
    // User Decrypt types
    schemas(UserDecryptRequestJsonV2, UserDecryptResponseJsonV2, UserDecryptPostResponseJsonV2, UserDecryptQueuedResultV2, UserDecryptResponsePayloadJsonV2),
    schemas(UserDecryptSucceededStatusResponseV2),
    // Delegated User Decrypt types
    schemas(DelegatedUserDecryptRequestJsonV2),
    // Key URL types
    schemas(KeyUrlResponseJsonV2, KeyUrlResponseV2, FheKeyInfoV2, KeyDataV2),
    // V2 Error types (union + base shapes)
    schemas(V2ErrorResponseBody, V2ApiError, V2ApiErrorWithDetails, V2ErrorLabel, RelayerV2ErrorDetail),
    // V2 response wrappers (failed POST, per-status-code GET)
    schemas(RelayerV2ResponseFailed, V2StatusQueued, V2StatusFailed),
    // Common types
    schemas(crate::http::endpoints::common::types::HandleContractPairJson, crate::http::endpoints::common::types::RequestValidityJson),
    schemas(crate::http::endpoints::common::types::ChainId),
),
tags(
    (name = "Key URL", description = "Read FHE public key URLs"),
    (name = "Input Proof", description = "Verify input proofs for encrypted computations"),
    (name = "User Decrypt", description = "Decrypt ciphertexts with user-provided key shares"),
    (name = "Delegated User Decrypt", description = "Decrypt ciphertexts via delegated key shares"),
    (name = "Public Decrypt", description = "Decrypt ciphertexts using the network public key"),
    (name = "Health", description = "Liveness, readiness, and version probes")
),
modifiers(&SecurityAddon, &TagGroupModifier, &ErrorLabelEnricher, &ExampleInjector)
)]
struct ApiDoc;

/// Build the OpenAPI document from the annotated handlers and schemas.
pub fn build_openapi_doc() -> utoipa::openapi::OpenApi {
    ApiDoc::openapi()
}

/// Adds the `X-API-Key` security scheme and applies it globally.
/// Health endpoints are exempted via per-operation `security` annotations.
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};

        let components = openapi.components.get_or_insert_with(Default::default);
        components.security_schemes.insert(
            "ApiKeyAuth".to_string(),
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("X-API-Key"))),
        );

        // Apply globally — individual operations can override with `security: []`.
        openapi.security.get_or_insert_with(Vec::new).push(
            utoipa::openapi::security::SecurityRequirement::new(
                "ApiKeyAuth",
                std::iter::empty::<String>(),
            ),
        );
    }
}

struct TagGroupModifier;

impl Modify for TagGroupModifier {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        use serde_json::json;
        use utoipa::openapi::extensions::Extensions;

        let extensions = openapi.extensions.get_or_insert_with(Extensions::default);
        extensions.insert(
            "x-tagGroups".to_string(),
            json!([
                {
                    "name": "Key URL Endpoint",
                    "tags": [
                        "Key URL"
                    ]
                },
                {
                    "name": "Ciphertext Operation Endpoints",
                    "tags": [
                        "Input Proof",
                        "User Decrypt",
                        "Delegated User Decrypt",
                        "Public Decrypt"
                    ]
                },
                {
                    "name": "Health Endpoints",
                    "tags": ["Health"]
                }
            ]),
        );
    }
}

/// Enriches each V2 error response description with the scoped error labels.
///
/// Walks `openapi.paths` and, for every V2 endpoint + error status code,
/// appends a "Possible error labels: …" line derived from
/// [`labels_for_status`](crate::http::openapi::expected_labels::labels_for_status).
struct ErrorLabelEnricher;

impl Modify for ErrorLabelEnricher {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        // ── Replace placeholder with auto-generated error reference table ──
        if let Some(ref mut info) = openapi.info.description {
            if info.contains("<!-- ERROR_LABEL_TABLE -->") {
                let table = Self::build_error_table();
                *info = info.replace("<!-- ERROR_LABEL_TABLE -->", &table);
            }
        }

        // ── Append per-response label summaries ────────────────────────────
        for (path_str, path_item) in &mut openapi.paths.paths {
            if !path_str.starts_with("/v2/") {
                continue;
            }

            let operations: Vec<(bool, &mut utoipa::openapi::path::Operation)> = [
                (true, path_item.post.as_mut()),
                (false, path_item.get.as_mut()),
            ]
            .into_iter()
            .filter_map(|(is_post, op)| op.map(|o| (is_post, o)))
            .collect();

            for (is_post, operation) in operations {
                for (status, response_or_ref) in &mut operation.responses.responses {
                    let labels = labels_for_status(path_str, status, is_post);
                    if labels.is_empty() {
                        continue;
                    }
                    if let RefOr::T(response) = response_or_ref {
                        let label_list = labels
                            .iter()
                            .map(|(l, _, _)| format!("`{}`", l))
                            .collect::<Vec<_>>()
                            .join(", ");
                        let desc = response.description.trim_end().trim_end_matches('.');
                        let clause = if labels.len() == 1 {
                            format!("Error label: {}", label_list)
                        } else {
                            format!("Error label is one of: {}", label_list)
                        };
                        response.description = format!("{}. {}", desc, clause);
                    }
                }
            }
        }
    }
}

impl ErrorLabelEnricher {
    fn build_error_table() -> String {
        let mut fix = Vec::new();
        let mut retry = Vec::new();
        let mut escalate = Vec::new();

        for def in ERROR_LABEL_DEFS {
            let row = format!("| `{}` | {} | {} |", def.label, def.http_status, def.action);
            if !def.retryable && (def.http_status == "400" || def.http_status == "404") {
                fix.push(row);
            } else if def.retryable {
                retry.push(row);
            } else {
                escalate.push(row);
            }
        }

        let mut table = String::from(
            "## Error Handling\n\n\
             All error responses include a machine-readable `label` field. \
             Each error response also has one or more named examples you can expand below.\n\n",
        );

        table.push_str(
            "### Fix and retry (client errors)\n\n\
             | Label | HTTP | What to fix |\n\
             |-------|------|-------------|\n",
        );
        for row in &fix {
            table.push_str(row);
            table.push('\n');
        }

        table.push_str(
            "\n### Retry with backoff (transient errors)\n\n\
             | Label | HTTP | Guidance |\n\
             |-------|------|----------|\n",
        );
        for row in &retry {
            table.push_str(row);
            table.push('\n');
        }

        table.push_str(
            "\n### Contact operator (infrastructure)\n\n\
             | Label | HTTP | Guidance |\n\
             |-------|------|----------|\n",
        );
        for row in &escalate {
            table.push_str(row);
            table.push('\n');
        }

        table
    }
}

/// Auto-generates named `examples` for every V2 error response from the
/// [`ERROR_MATRIX`](crate::http::openapi::expected_labels::ERROR_MATRIX).
///
/// This replaces the hand-maintained `examples(...)` blocks in handler macros.
struct ExampleInjector;

impl Modify for ExampleInjector {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        use serde_json::json;
        use std::collections::BTreeMap;
        use utoipa::openapi::example::Example;

        for (path_str, path_item) in &mut openapi.paths.paths {
            if !path_str.starts_with("/v2/") {
                continue;
            }

            let operations: Vec<(bool, &mut utoipa::openapi::path::Operation)> = [
                (true, path_item.post.as_mut()),
                (false, path_item.get.as_mut()),
            ]
            .into_iter()
            .filter_map(|(is_post, op)| op.map(|o| (is_post, o)))
            .collect();

            for (is_post, operation) in operations {
                for (status, response_or_ref) in &mut operation.responses.responses {
                    if !["400", "404", "429", "500", "503"].contains(&status.as_str()) {
                        continue;
                    }

                    let labels = labels_for_status(path_str, status, is_post);
                    if labels.is_empty() {
                        continue;
                    }

                    if let RefOr::T(response) = response_or_ref {
                        let content = response
                            .content
                            .entry("application/json".to_string())
                            .or_default();

                        let mut examples: BTreeMap<String, RefOr<Example>> = BTreeMap::new();

                        for (label, message, summary) in &labels {
                            let mut error_obj = json!({
                                "label": label,
                                "message": message,
                            });

                            if has_details(label) {
                                error_obj["details"] = json!([{
                                    "field": "fieldName",
                                    "issue": "Validation issue description"
                                }]);
                            }

                            let mut envelope = json!({
                                "status": "failed",
                                "error": error_obj,
                            });

                            // POST 429 rate_limited and keyurl 503 have no requestId
                            // (constructed before a request ID is assigned).
                            let is_keyurl = path_str.contains("keyurl");
                            let include_request_id = if is_keyurl {
                                false
                            } else if is_post {
                                !(status == "429" && *label == "rate_limited")
                            } else {
                                true
                            };

                            if include_request_id {
                                envelope["requestId"] =
                                    json!("550e8400-e29b-41d4-a716-446655440000");
                            }

                            let example = Example::builder()
                                .summary(summary.to_string())
                                .value(Some(envelope))
                                .build();

                            examples.insert(to_pascal_case(label), RefOr::T(example));
                        }

                        content.examples = examples;
                    }
                }
            }
        }
    }
}

/// Create OpenAPI documentation middleware that serves docs at /docs via Redoc
pub fn openapi_middleware() -> Router {
    Redoc::with_url("/docs", ApiDoc::openapi()).into()
}
