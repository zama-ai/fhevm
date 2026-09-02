use crate::core::{
    db::{
        UserDecryptionResponseRow, read_user_decryption_response, upsert_user_decryption_request,
    },
    http::{
        AppState,
        decrypt::{self, DecryptionRoute},
    },
    validation::validate_user_decryption,
};
use actix_web::{
    HttpResponse,
    web::{Data, Json},
};
use alloy::primitives::B256;
use connector_utils::monitoring::otlp::PropagationContext;
use kms_connector_api::{ErrorCode, ErrorResponse, UserDecryptionRequest, UserDecryptionResponse};
use sqlx::{PgExecutor, postgres::PgQueryResult};

/// `POST /v1/user-decrypt`
#[tracing::instrument(skip_all, fields(decryption_id))]
pub async fn user_decrypt(
    state: Data<AppState>,
    body: Json<UserDecryptionRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    let request = body.into_inner();
    validate_user_decryption(&request, &state.config)
        .map_err(|e| ErrorResponse::new(ErrorCode::Malformed, e.to_string(), None))?;
    let id = request.id();
    tracing::Span::current().record("decryption_id", id.to_string());

    decrypt::handle::<UserRoute>(&state, id, &request).await
}

pub struct UserRoute;

impl DecryptionRoute for UserRoute {
    type Request = UserDecryptionRequest;
    type ResponseRow = UserDecryptionResponseRow;

    async fn read_response<'e>(
        executor: impl PgExecutor<'e>,
        id: B256,
    ) -> sqlx::Result<Option<Self::ResponseRow>> {
        read_user_decryption_response(executor, id).await
    }

    async fn upsert_request<'e>(
        executor: impl PgExecutor<'e>,
        id: B256,
        request: &Self::Request,
        otlp_ctx: &PropagationContext,
    ) -> anyhow::Result<PgQueryResult> {
        upsert_user_decryption_request(executor, id, request, otlp_ctx).await
    }

    fn error_code(response_row: &Self::ResponseRow) -> Option<ErrorCode> {
        response_row
            .error_code
            .as_deref()
            .map(|e| e.parse().unwrap_or(ErrorCode::Unknown))
    }

    fn build_response(id: B256, row: Self::ResponseRow) -> Result<HttpResponse, ErrorResponse> {
        if let Some(code) = row.error_code {
            return Err(decrypt::error_from_row(&code, row.error_details, id));
        }
        match (row.user_decrypted_shares, row.signature) {
            (Some(user_decrypted_shares), Some(signature)) => {
                Ok(HttpResponse::Ok().json(UserDecryptionResponse {
                    decryption_id: id,
                    user_decrypted_shares: user_decrypted_shares.into(),
                    signature: signature.into(),
                    extra_data: row.extra_data.into(),
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
