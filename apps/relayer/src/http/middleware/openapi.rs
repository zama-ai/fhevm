use crate::http::endpoints::v2::types::{
    input_proof::{
        InputProofPostResponseJson as InputProofPostResponseJsonV2,
        InputProofQueuedResult as InputProofQueuedResultV2,
        InputProofRequestJson as InputProofRequestJsonV2,
        InputProofResponseJson as InputProofResponseJsonV2,
        InputProofStatusResponseJson as InputProofStatusResponseJsonV2,
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
        PublicDecryptStatusResponseJson as PublicDecryptStatusResponseJsonV2,
    },
    user_decrypt::{
        DelegatedUserDecryptRequestJson as DelegatedUserDecryptRequestJsonV2,
        UserDecryptPostResponseJson as UserDecryptPostResponseJsonV2,
        UserDecryptQueuedResult as UserDecryptQueuedResultV2,
        UserDecryptRequestJson as UserDecryptRequestJsonV2,
        UserDecryptResponseJson as UserDecryptResponseJsonV2,
        UserDecryptResponsePayloadJson as UserDecryptResponsePayloadJsonV2,
        UserDecryptStatusResponseJson as UserDecryptStatusResponseJsonV2,
    },
};
use axum::Router;
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};

// OpenAPI documentation
#[derive(OpenApi)]
#[openapi(
    servers(
        (url = "/v2", description = "FHEVM Relayer API v2")
    ),
paths(
    crate::http::endpoints::v2::handlers::user_decrypt::user_decrypt_post_v2,
    crate::http::endpoints::v2::handlers::user_decrypt::user_decrypt_get_v2,
    crate::http::endpoints::v2::handlers::public_decrypt::public_decrypt_post_v2,
    crate::http::endpoints::v2::handlers::public_decrypt::public_decrypt_get_v2,
    crate::http::endpoints::v2::handlers::input_proof::input_proof_post_v2,
    crate::http::endpoints::v2::handlers::input_proof::input_proof_get_v2,
    crate::http::endpoints::v2::handlers::keyurl::keyurl_v2,
),
components(
    schemas(InputProofRequestJsonV2, InputProofResponseJsonV2, InputProofPostResponseJsonV2, InputProofStatusResponseJsonV2, InputProofQueuedResultV2),
    schemas(PublicDecryptRequestJsonV2, PublicDecryptResponseJsonV2, PublicDecryptPostResponseJsonV2, PublicDecryptStatusResponseJsonV2, PublicDecryptQueuedResultV2),
    schemas(UserDecryptRequestJsonV2, UserDecryptResponseJsonV2, UserDecryptPostResponseJsonV2, UserDecryptStatusResponseJsonV2, UserDecryptQueuedResultV2, UserDecryptResponsePayloadJsonV2),
    schemas(DelegatedUserDecryptRequestJsonV2),
    schemas(KeyUrlResponseJsonV2, KeyUrlResponseV2, FheKeyInfoV2, KeyDataV2),
    schemas(crate::http::ErrorResponse, crate::http::ApiError, crate::http::ErrorDetail, crate::http::ErrorLabel),
    schemas(crate::http::endpoints::common::types::HandleContractPairJson, crate::http::endpoints::common::types::RequestValidityJson),
    schemas(crate::http::endpoints::common::types::ChainId),
),
tags(
    (name = "User Decrypt v2", description = "User Decrypt v2 API - Asynchronous with job tracking"),
    (name = "Public Decrypt v2", description = "Public Decrypt v2 API - Asynchronous with job tracking"),
    (name = "Input Proof v2", description = "Input Proof v2 API - Asynchronous with job tracking"),
    (name = "Key URL v2", description = "Key URL v2 API - FHE key material URLs")
)
)]
struct ApiDoc;

/// Create OpenAPI documentation middleware that serves docs at /docs via Redoc
pub fn openapi_middleware() -> Router {
    Redoc::with_url("/docs", ApiDoc::openapi()).into()
}
