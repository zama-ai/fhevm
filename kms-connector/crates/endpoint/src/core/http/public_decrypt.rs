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
        ValidationError, is_solana_public_decryption, parse_body, validate_public_decryption,
        validate_solana_public_decryption,
    },
};
use actix_web::{
    HttpResponse,
    web::{Data, Json},
};
use alloy::primitives::B256;
use connector_utils::monitoring::otlp::PropagationContext;
use connector_utils::types::{
    db::insert_solana_public_decryption, solana_request::SolanaPublicDecryptionRequest,
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

/// The chain of its handles tells the Solana body from the EVM `v1` body.
fn validate(
    body: Value,
    config: &Config,
) -> Result<(B256, ValidatedPublicDecryption), ValidationError> {
    if is_solana_public_decryption(&body) {
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
                insert_solana_public_decryption(executor, request, Utc::now(), otlp_ctx).await
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::validation::tests::{config, handle};
    use alloy::primitives::Bytes;
    use tfhe::FheTypes;
    use zama_solana_acl::host_chain::solana_host_chain_id;

    /// The first handle's chain picks the body. A body mixing EVM and Solana handles is malformed,
    /// whichever comes first and whichever body carries them.
    #[test]
    fn the_handles_chain_picks_the_body_and_a_mix_is_malformed() {
        let solana_chain = solana_host_chain_id(12345);
        let config = Config {
            supported_chain_ids: vec![1, solana_chain],
            ..config()
        };
        let euint64 = FheTypes::Uint64 as u8;
        let (evm, solana) = (handle(1, euint64), handle(solana_chain, euint64));
        let evm_body = |handles| {
            serde_json::to_value(PublicDecryptionRequest {
                ctHandles: handles,
                extraData: Bytes::new(),
            })
            .unwrap()
        };
        let solana_body = |handles: Vec<B256>| {
            serde_json::to_value(SolanaPublicDecryptionBody {
                encryptedStores: vec![B256::repeat_byte(0x03); handles.len()],
                ctHandles: handles,
                extraData: Bytes::new(),
            })
            .unwrap()
        };

        assert!(matches!(
            validate(evm_body(vec![evm]), &config),
            Ok((_, ValidatedPublicDecryption::Evm(_)))
        ));
        assert!(matches!(
            validate(solana_body(vec![solana]), &config),
            Ok((_, ValidatedPublicDecryption::Solana(_)))
        ));
        for handles in [vec![evm, solana], vec![solana, evm]] {
            for body in [evm_body(handles.clone()), solana_body(handles.clone())] {
                let error = validate(body.clone(), &config).err();
                assert_eq!(
                    error.map(|e| e.code()),
                    Some(ErrorCode::Malformed),
                    "{body}"
                );
            }
        }
    }

    /// The parse is the stores check: an EVM body naming stores, or a Solana body without them, is
    /// malformed before anything is stored.
    #[test]
    fn stores_that_do_not_match_the_handles_chain_are_malformed() {
        let solana_chain = solana_host_chain_id(12345);
        let config = Config {
            supported_chain_ids: vec![1, solana_chain],
            ..config()
        };
        let euint64 = FheTypes::Uint64 as u8;
        let evm_with_stores = serde_json::json!({
            "ctHandles": [handle(1, euint64)],
            "extraData": "0x",
            "encryptedStores": [B256::repeat_byte(0x03)],
        });
        let solana_without_stores = serde_json::json!({
            "ctHandles": [handle(solana_chain, euint64)],
            "extraData": "0x",
        });
        for (body, expected) in [
            (evm_with_stores, "unknown field `encryptedStores`"),
            (solana_without_stores, "missing field `encryptedStores`"),
        ] {
            let Err(error) = validate(body.clone(), &config) else {
                panic!("{body} was accepted");
            };
            assert_eq!(error.code(), ErrorCode::Malformed, "{body}");
            assert!(error.to_string().contains(expected), "{body}: {error}");
        }
    }
}
