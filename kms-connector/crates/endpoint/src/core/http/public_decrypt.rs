use crate::core::{
    db::{
        PublicDecryptionResponseRow, read_public_decryption_response,
        upsert_public_decryption_request,
    },
    http::{
        AppState,
        decrypt::{self, DecryptionRoute},
    },
    validation::validate_public_decryption,
};
use actix_web::{
    HttpResponse,
    web::{Data, Json},
};
use alloy::primitives::B256;
use connector_utils::monitoring::otlp::PropagationContext;
use kms_connector_api::{
    ErrorCode, ErrorResponse, PublicDecryptionRequest, PublicDecryptionResponse,
};
use sqlx::{
    PgExecutor,
    postgres::PgQueryResult,
    types::chrono::{DateTime, Utc},
};

/// `POST /v1/public-decrypt`
#[tracing::instrument(skip_all, fields(decryption_id))]
pub async fn public_decrypt(
    state: Data<AppState>,
    body: Json<PublicDecryptionRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    let request = body.into_inner();
    validate_public_decryption(&request, &state.config)
        .map_err(|e| ErrorResponse::new(ErrorCode::Malformed, e.to_string(), None))?;
    let id = request.id();
    tracing::Span::current().record("decryption_id", id.to_string());

    decrypt::handle::<PublicRoute>(&state, id, &request).await
}

pub struct PublicRoute;

impl DecryptionRoute for PublicRoute {
    type Request = PublicDecryptionRequest;
    type ResponseRow = PublicDecryptionResponseRow;

    async fn read_response<'e>(
        executor: impl PgExecutor<'e>,
        id: B256,
    ) -> sqlx::Result<Option<Self::ResponseRow>> {
        read_public_decryption_response(executor, id).await
    }

    async fn upsert_request<'e>(
        executor: impl PgExecutor<'e>,
        id: B256,
        request: &Self::Request,
        otlp_ctx: &PropagationContext,
    ) -> anyhow::Result<PgQueryResult> {
        upsert_public_decryption_request(executor, id, request, otlp_ctx).await
    }

    fn created_at(response_row: &Self::ResponseRow) -> DateTime<Utc> {
        response_row.created_at
    }

    fn error_code(response_row: &Self::ResponseRow) -> Option<ErrorCode> {
        response_row
            .error_code
            .as_deref()
            .map(|e| e.parse().unwrap_or(ErrorCode::Unknown))
    }

    fn build_response(
        id: B256,
        response_row: Self::ResponseRow,
    ) -> Result<HttpResponse, ErrorResponse> {
        if let Some(code) = response_row.error_code {
            return Err(decrypt::error_from_row(
                &code,
                response_row.error_details,
                id,
            ));
        }
        match (response_row.decrypted_result, response_row.signature) {
            (Some(decrypted_result), Some(signature)) => {
                Ok(HttpResponse::Ok().json(PublicDecryptionResponse {
                    decryption_id: id,
                    decrypted_result: decrypted_result.into(),
                    signature: signature.into(),
                    extra_data: response_row.extra_data.into(),
                }))
            }
            // Should be unreachable thanks to the `payload_or_error` CHECK constraint.
            _ => Err(ErrorResponse::new(
                ErrorCode::Unknown,
                "inconsistent response row",
                Some(id),
            )),
        }
    }
}
