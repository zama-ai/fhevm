#[allow(unused_imports)]
// These are used by the OpenAPI macro but not detected by the compiler
use crate::http::handlers::{
    input_proof::__path_input_proof_v1, keyurl::__path_keyurl_v1,
    public_decrypt::__path_public_decrypt_v1, user_decrypt::__path_user_decrypt_v1,
};
use crate::http::types::{
    InputProofErrorResponseJson, InputProofRequestJson, InputProofResponseJson, KeyUrlResponseJson,
    PublicDecryptErrorResponseJson, PublicDecryptRequestJson, PublicDecryptResponseJson,
    UserDecryptErrorResponseJson, UserDecryptRequestJson, UserDecryptResponseJson,
};
use axum::Router;
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};

// OpenAPI documentation
#[derive(OpenApi)]
#[openapi(
    servers((url = "/v1", description = "FHEVM Relayer API v1")),
paths(
    crate::http::handlers::input_proof::input_proof_v1,
    crate::http::handlers::public_decrypt::public_decrypt_v1,
    crate::http::handlers::user_decrypt::user_decrypt_v1,
    crate::http::handlers::keyurl::keyurl_v1,
),
components(
    schemas(PublicDecryptRequestJson, PublicDecryptResponseJson, PublicDecryptErrorResponseJson),
    schemas(UserDecryptRequestJson, UserDecryptResponseJson, UserDecryptErrorResponseJson),
    schemas(InputProofRequestJson, InputProofResponseJson, InputProofErrorResponseJson),
    schemas(KeyUrlResponseJson),
    schemas(crate::http::types::keyurl::Response, crate::http::types::keyurl::FheKeyInfo, crate::http::types::keyurl::KeyData),
    schemas(crate::http::ErrorResponse, crate::http::ApiError, crate::http::ErrorDetail, crate::http::ErrorLabel),
    schemas(crate::http::types::user_decrypt::HandleContractPairJson, crate::http::types::user_decrypt::RequestValidityJson, crate::http::types::user_decrypt::UserDecryptResponsePayloadJson),
    schemas(crate::http::types::input_proof::InputProofResponsePayloadJson),
    schemas(crate::http::types::public_decrypt::PublicDecryptResponsePayloadJson),
    schemas(crate::http::ChainId),
),
tags(
    (name = "FHEVM Relayer API", description = "FHEVM Relayer API")
)
)]
struct ApiDoc;

/// Create OpenAPI documentation middleware that serves docs at /docs via Redoc
pub fn openapi_middleware() -> Router {
    Redoc::with_url("/docs", ApiDoc::openapi()).into()
}
