use crate::core::{
    db::{
        UserDecryptionResponseRow, read_user_decryption_response, upsert_user_decryption_request,
    },
    http::{
        AppState,
        decrypt::{self, DecryptionRoute},
    },
    validation::{validate_solana_user_decryption, validate_user_decryption},
};
use actix_web::{
    HttpResponse,
    web::{Data, Json},
};
use alloy::primitives::{B256, U256};
use connector_utils::monitoring::otlp::PropagationContext;
use connector_utils::types::{
    db::{RequestSource, insert_solana_user_decryption},
    solana_request::SolanaUserDecryptionRequestV1,
};
use kms_connector_api::SolanaUserDecryptionRequest;
use kms_connector_api::{ErrorCode, ErrorResponse, UserDecryptionRequest, UserDecryptionResponse};
use serde::Deserialize;
use sqlx::{
    PgExecutor,
    postgres::PgQueryResult,
    types::chrono::{DateTime, Utc},
};

#[derive(Deserialize)]
#[serde(untagged)]
pub enum UserDecryptionBody {
    Eip712(UserDecryptionRequest),
    Solana(SolanaUserDecryptionRequest),
}

pub enum ValidatedUserDecryption {
    Eip712(UserDecryptionRequest),
    Solana(SolanaUserDecryptionRequestV1),
}

/// `POST /v1/user-decrypt`
#[tracing::instrument(skip_all, fields(decryption_id))]
pub async fn user_decrypt(
    state: Data<AppState>,
    body: Json<UserDecryptionBody>,
) -> Result<HttpResponse, ErrorResponse> {
    let validated = match body.into_inner() {
        UserDecryptionBody::Eip712(request) => validate_user_decryption(&request, &state.config)
            .map(|()| (request.id(), ValidatedUserDecryption::Eip712(request))),
        UserDecryptionBody::Solana(request) => {
            validate_solana_user_decryption(&request, &state.config).map(|body| {
                let id = request.id();
                (
                    id,
                    ValidatedUserDecryption::Solana(SolanaUserDecryptionRequestV1 {
                        decryption_id: U256::from_be_bytes(id.0),
                        request: body,
                    }),
                )
            })
        }
    };
    let (id, request) = validated.map_err(|e| ErrorResponse::new(e.code(), e.to_string(), None))?;
    tracing::Span::current().record("decryption_id", id.to_string());

    decrypt::handle::<UserRoute>(&state, id, &request).await
}

pub struct UserRoute;

impl DecryptionRoute for UserRoute {
    type Request = ValidatedUserDecryption;
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
        match request {
            ValidatedUserDecryption::Eip712(request) => {
                upsert_user_decryption_request(executor, id, request, otlp_ctx).await
            }
            ValidatedUserDecryption::Solana(request) => {
                insert_solana_user_decryption(
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
