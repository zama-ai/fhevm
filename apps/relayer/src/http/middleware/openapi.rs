#[allow(unused_imports)]
// These are used by the OpenAPI macro but not detected by the compiler
use crate::http::endpoints::v1::handlers::{
    input_proof::__path_input_proof_v1, keyurl::__path_keyurl_v1,
    public_decrypt::__path_public_decrypt_v1, user_decrypt::__path_user_decrypt_v1,
};
use crate::http::endpoints::v1::types::{
    InputProofErrorResponseJson, InputProofRequestJson, InputProofResponseJson, KeyUrlResponseJson,
    PublicDecryptErrorResponseJson, PublicDecryptRequestJson, PublicDecryptResponseJson,
    UserDecryptErrorResponseJson, UserDecryptRequestJson, UserDecryptResponseJson,
};
use crate::http::endpoints::v2::types::{
    input_proof::{
        InputProofPostResponseJson as InputProofPostResponseJsonV2,
        InputProofQueuedResult as InputProofQueuedResultV2,
        InputProofResponseJson as InputProofResponseJsonV2,
        InputProofStatusResponseJson as InputProofStatusResponseJsonV2,
    },
    public_decrypt::{
        PublicDecryptPostResponseJson as PublicDecryptPostResponseJsonV2,
        PublicDecryptQueuedResult as PublicDecryptQueuedResultV2,
        PublicDecryptResponseJson as PublicDecryptResponseJsonV2,
        PublicDecryptStatusResponseJson as PublicDecryptStatusResponseJsonV2,
    },
    user_decrypt::{
        UserDecryptErrorResponseJson as UserDecryptErrorResponseJsonV2,
        UserDecryptPostResponseJson as UserDecryptPostResponseJsonV2,
        UserDecryptQueuedResult as UserDecryptQueuedResultV2,
        UserDecryptResponseJson as UserDecryptResponseJsonV2,
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
        (url = "/v1", description = "FHEVM Relayer API v1"),
        (url = "/v2", description = "FHEVM Relayer API v2")
    ),
paths(
    crate::http::endpoints::v1::handlers::input_proof::input_proof_v1,
    crate::http::endpoints::v1::handlers::public_decrypt::public_decrypt_v1,
    crate::http::endpoints::v1::handlers::user_decrypt::user_decrypt_v1,
    crate::http::endpoints::v1::handlers::keyurl::keyurl_v1,
    crate::http::endpoints::v2::handlers::user_decrypt::user_decrypt_post_v2,
    crate::http::endpoints::v2::handlers::user_decrypt::user_decrypt_get_v2,
    crate::http::endpoints::v2::handlers::public_decrypt::public_decrypt_post_v2,
    crate::http::endpoints::v2::handlers::public_decrypt::public_decrypt_get_v2,
    crate::http::endpoints::v2::handlers::input_proof::input_proof_post_v2,
    crate::http::endpoints::v2::handlers::input_proof::input_proof_get_v2,
),
components(
    schemas(PublicDecryptRequestJson, PublicDecryptResponseJson, PublicDecryptErrorResponseJson),
    schemas(UserDecryptRequestJson, UserDecryptResponseJson, UserDecryptErrorResponseJson),
    schemas(InputProofRequestJson, InputProofResponseJson, InputProofErrorResponseJson),
    schemas(KeyUrlResponseJson),
    schemas(UserDecryptPostResponseJsonV2, UserDecryptStatusResponseJsonV2, UserDecryptQueuedResultV2, UserDecryptErrorResponseJsonV2),
    schemas(PublicDecryptPostResponseJsonV2, PublicDecryptStatusResponseJsonV2, PublicDecryptQueuedResultV2, PublicDecryptResponseJsonV2),
    schemas(InputProofPostResponseJsonV2, InputProofStatusResponseJsonV2, InputProofQueuedResultV2, InputProofResponseJsonV2),
    schemas(crate::http::endpoints::v1::types::keyurl::Response, crate::http::endpoints::v1::types::keyurl::FheKeyInfo, crate::http::endpoints::v1::types::keyurl::KeyData),
    schemas(crate::http::ErrorResponse, crate::http::ApiError, crate::http::ErrorDetail, crate::http::ErrorLabel),
    schemas(crate::http::endpoints::common::types::HandleContractPairJson, crate::http::endpoints::common::types::RequestValidityJson, crate::http::endpoints::v1::types::user_decrypt::UserDecryptResponsePayloadJson),
    schemas(crate::http::endpoints::v1::types::input_proof::InputProofResponsePayloadJson),
    schemas(crate::http::endpoints::v1::types::public_decrypt::PublicDecryptResponsePayloadJson),
    schemas(crate::http::endpoints::common::types::ChainId),
    schemas(UserDecryptResponseJsonV2),
),
tags(
    (name = "FHEVM Relayer API v1", description = "FHEVM Relayer API v1"),
    (name = "User Decrypt v2", description = "User Decrypt v2 API - Asynchronous with job tracking"),
    (name = "Public Decrypt v2", description = "Public Decrypt v2 API - Asynchronous with job tracking"),
    (name = "Input Proof v2", description = "Input Proof v2 API - Asynchronous with job tracking")
)
)]
struct ApiDoc;

/// Create OpenAPI documentation middleware that serves docs at /docs via Redoc
pub fn openapi_middleware() -> Router {
    Redoc::with_url("/docs", ApiDoc::openapi()).into()
}
