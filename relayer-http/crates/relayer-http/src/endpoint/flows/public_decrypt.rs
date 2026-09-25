//! `POST /v4/exp/public-decrypt`: the current relayer's body, validated, converted to the connector DTO and
//! aggregated.

use alloy::primitives::{B256, Bytes};
use axum::extract::{Request, State};
use axum::response::{IntoResponse, Response};
use kms_connector_api::PublicDecryptionRequest;
use serde::Deserialize;

use super::{Reply, read_json};
use crate::App;
use crate::endpoint::ApiError;
use crate::endpoint::validate::{self, Invalid};
use crate::logging::Log;

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

/// One request, logged step by step under its own `Log`.
pub async fn handle(app: State<App>, request: Request) -> Response {
    let mut log = Log::new("public_decrypt");
    process(&app, request, &mut log)
        .await
        .unwrap_or_else(IntoResponse::into_response)
}

/// Read and parse the body (bounded by `http.body_read_timeout`), validate, convert to the connector DTO,
/// aggregate, answer.
async fn process(app: &App, request: Request, log: &mut Log) -> Result<Response, ApiError> {
    let request: PublicDecryptRequest = read_json(app, request, log).await?;
    log.received(request.ciphertext_handles.clone());
    validate(&request, &app.http.supported_chain_ids).map_err(|e| {
        log.validation_failed(&e.field, &e.issue);
        ApiError::malformed(log, e.to_string())
    })?;
    let request = PublicDecryptionRequest::from(request);
    log.forwarded(request.id());
    let output = app
        .public_decrypt
        .run(&log.request_id, request)
        .await
        .map_err(|e| ApiError::aggregation(log, e))?;
    Ok(Reply::succeeded(log.clone(), output).into_response())
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
