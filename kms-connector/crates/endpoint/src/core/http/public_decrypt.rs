use crate::core::{
    Config,
    db::{
        PublicDecryptionResponseRow, read_public_decryption_response,
        upsert_public_decryption_request,
    },
    http::{
        AppState,
        decrypt::{self, DecryptionRoute},
    },
    validation::{
        ValidationError, parse_body, validate_public_decryption, validate_solana_public_decryption,
    },
};
use actix_web::{
    HttpResponse,
    web::{Data, Json},
};
use alloy::primitives::B256;
use connector_utils::monitoring::otlp::PropagationContext;
use connector_utils::types::{
    db::{RequestSource, insert_solana_public_decryption},
    solana_request::SolanaPublicDecryptionRequest,
};
use kms_connector_api::{
    ErrorCode, ErrorResponse, PublicDecryptionRequest, PublicDecryptionResponse,
    SolanaPublicDecryptionRequest as SolanaPublicDecryptionBody,
};
use serde_json::Value;
use sqlx::{
    PgExecutor,
    postgres::PgQueryResult,
    types::chrono::{DateTime, Utc},
};

pub enum ValidatedPublicDecryption {
    Evm(PublicDecryptionRequest),
    Solana(SolanaPublicDecryptionRequest),
}

/// `POST /v1/public-decrypt`
#[tracing::instrument(skip_all, fields(decryption_id))]
pub async fn public_decrypt(
    state: Data<AppState>,
    body: Json<Value>,
) -> Result<HttpResponse, ErrorResponse> {
    let (id, request) = validate(body.into_inner(), &state.config)
        .map_err(|e| ErrorResponse::new(e.code(), e.to_string(), None))?;
    tracing::Span::current().record("decryption_id", id.to_string());

    decrypt::handle::<PublicRoute>(&state, id, &request).await
}

/// A Solana body names each handle's encrypted store; an EVM body has no such field.
fn validate(
    body: Value,
    config: &Config,
) -> Result<(B256, ValidatedPublicDecryption), ValidationError> {
    if body.get("encryptedStores").is_some() {
        let request: SolanaPublicDecryptionBody = parse_body(body)?;
        let id = request.id();
        let request = validate_solana_public_decryption(id, &request, config)?;
        Ok((id, ValidatedPublicDecryption::Solana(request)))
    } else {
        let request: PublicDecryptionRequest = parse_body(body)?;
        validate_public_decryption(&request, config)?;
        Ok((request.id(), ValidatedPublicDecryption::Evm(request)))
    }
}

pub struct PublicRoute;

impl DecryptionRoute for PublicRoute {
    type Request = ValidatedPublicDecryption;
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
        match request {
            ValidatedPublicDecryption::Evm(request) => {
                upsert_public_decryption_request(executor, id, request, otlp_ctx).await
            }
            ValidatedPublicDecryption::Solana(request) => {
                insert_solana_public_decryption(
                    executor,
                    request,
                    None,
                    Utc::now(),
                    otlp_ctx,
                    RequestSource::Http,
                )
                .await
            }
        }
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
