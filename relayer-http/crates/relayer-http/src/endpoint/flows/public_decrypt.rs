//! `POST /v4/exp/public-decrypt`: the current relayer's body, validated, converted to the connector DTO and
//! aggregated.

use alloy::primitives::{B256, Bytes};
use axum::Json;
use axum::extract::State;
use axum::extract::rejection::JsonRejection;
use axum::response::{IntoResponse, Response};
use kms_connector_api::PublicDecryptionRequest;
use serde::Deserialize;
use tracing::info;

use super::Reply;
use crate::App;
use crate::endpoint::ApiError;
use crate::endpoint::validate::{self, Invalid};

/// Request body. Hex fields are typed: length and format are checked by deserialisation.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PublicDecryptRequest {
    pub ciphertext_handles: Vec<B256>,
    pub extra_data: Bytes,
}

/// The connector's cheap handle rules and the `extraData` format.
pub fn validate(request: &PublicDecryptRequest, chain_ids: &[u64]) -> Result<(), Invalid> {
    validate::handles(
        "ciphertextHandles",
        request.ciphertext_handles.iter(),
        chain_ids,
    )?;
    validate::extra_data("extraData", &request.extra_data)
}

/// One request: mint the id, parse (axum did it: `body` is `Err` for invalid JSON, an unknown field, a wrong
/// content type or an oversized body), validate, convert to the connector DTO, aggregate, answer.
pub async fn handle(
    app: State<App>,
    body: Result<Json<PublicDecryptRequest>, JsonRejection>,
) -> Result<Response, ApiError> {
    let request_id = super::request_id();
    let Json(request) = body.map_err(|e| ApiError::malformed(&request_id, e.body_text()))?;
    validate(&request, &app.http.supported_chain_ids)
        .map_err(|e| ApiError::malformed(&request_id, e.to_string()))?;
    let request = PublicDecryptionRequest::from(request);
    info!(
        request_id,
        decryption_id = %request.id(),
        handles = request.ctHandles.len(),
        "public decrypt request"
    );
    let output = app
        .public_decrypt
        .run(&request_id, request)
        .await
        .map_err(|e| ApiError::aggregation(&request_id, e))?;
    Ok(Reply::succeeded(request_id, output).into_response())
}

/// The body the connector receives (RFC-033).
impl From<PublicDecryptRequest> for PublicDecryptionRequest {
    fn from(request: PublicDecryptRequest) -> Self {
        Self {
            ctHandles: request.ciphertext_handles,
            extraData: request.extra_data,
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::endpoint::validate::tests::handle;

    pub(crate) fn wire() -> String {
        format!(
            r#"{{ "ciphertextHandles": ["{}", "{}"], "extraData": "0x00" }}"#,
            handle(137, 0),
            handle(137, 5)
        )
    }

    pub(crate) fn valid() -> PublicDecryptRequest {
        serde_json::from_str(&wire()).unwrap()
    }

    #[test]
    fn wire_example_deserialises_and_validates() {
        let request = valid();
        assert_eq!(request.ciphertext_handles.len(), 2);
        assert_eq!(validate(&request, &[1, 137]), Ok(()));
        assert!(
            serde_json::from_str::<PublicDecryptRequest>(
                r#"{"ciphertextHandles": [], "extraData": "0x00", "x": 1}"#
            )
            .is_err()
        );
        assert!(
            serde_json::from_str::<PublicDecryptRequest>(
                r#"{"ciphertextHandles": ["0x12"], "extraData": "0x00"}"#
            )
            .is_err()
        );
    }

    #[test]
    fn rules_name_their_field() {
        let mut empty = valid();
        empty.ciphertext_handles.clear();
        assert_eq!(
            validate(&empty, &[137]).unwrap_err().field,
            "ciphertextHandles"
        );
        assert_eq!(
            validate(&valid(), &[1]).unwrap_err().field,
            "ciphertextHandles[0]"
        );
        let mut bad_extra = valid();
        bad_extra.extra_data = Bytes::from_static(&[7]);
        assert_eq!(validate(&bad_extra, &[137]).unwrap_err().field, "extraData");
    }

    #[test]
    fn conversion_keeps_every_field() {
        let request = valid();
        let connector = PublicDecryptionRequest::from(request.clone());
        assert_eq!(connector.ctHandles, request.ciphertext_handles);
        assert_eq!(connector.extraData, request.extra_data);
        assert_ne!(connector.id(), B256::ZERO);
    }
}
