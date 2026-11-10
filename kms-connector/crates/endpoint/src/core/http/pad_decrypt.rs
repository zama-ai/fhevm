use crate::core::{
    db::{PadDecryptionResponseRow, read_pad_decryption_response, upsert_pad_decryption_request},
    http::{
        AppState,
        decrypt::{self, DecryptionRoute},
    },
    validation::validate_pad_decryption,
};
use actix_web::{
    HttpResponse,
    web::{Data, Json},
};
use alloy::primitives::B256;
use connector_utils::monitoring::otlp::PropagationContext;
use kms_connector_api::{
    ErrorCode, ErrorResponse, PAD_DECRYPTION_ROUTE, PadDecryptionRequest, PadDecryptionResponse,
};
use sqlx::{
    PgExecutor,
    postgres::PgQueryResult,
    types::chrono::{DateTime, Utc},
};

/// `POST /v1/pad-decrypt`
#[tracing::instrument(skip_all, fields(decryption_id))]
pub async fn pad_decrypt(
    state: Data<AppState>,
    body: Json<PadDecryptionRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    let request = body.into_inner();
    let now = Utc::now().timestamp() as u64;
    validate_pad_decryption(&request, &state.config, now)
        .map_err(|e| ErrorResponse::new(e.code(), e.to_string(), None))?;
    let id = request.id();
    tracing::Span::current().record("decryption_id", id.to_string());

    decrypt::handle::<PadRoute>(&state, id, &request).await
}

pub struct PadRoute;

impl DecryptionRoute for PadRoute {
    const ROUTE: &'static str = PAD_DECRYPTION_ROUTE;
    type Request = PadDecryptionRequest;
    type ResponseRow = PadDecryptionResponseRow;

    async fn read_response<'e>(
        executor: impl PgExecutor<'e>,
        id: B256,
    ) -> sqlx::Result<Option<Self::ResponseRow>> {
        read_pad_decryption_response(executor, id).await
    }

    async fn upsert_request<'e>(
        executor: impl PgExecutor<'e>,
        id: B256,
        request: &Self::Request,
        otlp_ctx: &PropagationContext,
    ) -> anyhow::Result<PgQueryResult> {
        upsert_pad_decryption_request(executor, id, request, otlp_ctx).await
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

    fn build_response(id: B256, row: Self::ResponseRow) -> Result<HttpResponse, ErrorResponse> {
        if let Some(code) = row.error_code {
            return Err(decrypt::error_from_row(&code, row.error_details, id));
        }
        match (row.encrypted_payload, row.signature) {
            (Some(encrypted_payload), Some(signature)) => {
                Ok(HttpResponse::Ok().json(PadDecryptionResponse {
                    decryption_id: id,
                    encrypted_payload: encrypted_payload.into(),
                    signature: signature.into(),
                }))
            }
            // The database's `payload_or_error` CHECK constraint forbids this state.
            _ => Err(ErrorResponse::new(
                ErrorCode::Unknown,
                "inconsistent response row",
                Some(id),
            )),
        }
    }
}
