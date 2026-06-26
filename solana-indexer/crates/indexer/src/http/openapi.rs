//! utoipa OpenAPI aggregation for the indexer endpoints, mirroring relayer openapi.

use utoipa::OpenApi;

use crate::http::endpoints::{build_proof, health, lineage_leaf};

#[derive(OpenApi)]
#[openapi(
    paths(
        build_proof::handle,
        lineage_leaf::handle,
        health::liveness,
        health::healthz,
        health::version,
    ),
    components(schemas(
        build_proof::BuildProofRequest,
        build_proof::BuildProofResponse,
        lineage_leaf::LeafResponse,
        health::VersionResponse,
        crate::http::error::ErrorBody,
    ))
)]
pub struct ApiDoc;
