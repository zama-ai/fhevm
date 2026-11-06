//! `POST /v4/public-decrypt`: the current relayer's body, validated, converted to the connector DTO and
//! aggregated.

use alloy::primitives::{B256, Bytes};
use axum::extract::{Request, State};
use axum::response::{IntoResponse, Response};
use kms_connector_api::PublicDecryptionRequest;
use serde::Deserialize;

use super::{Reply, read_json};
use crate::App;
use crate::config::HttpConfig;
use crate::endpoint::ApiError;
use crate::endpoint::summary::{Guard, Summary};
use crate::endpoint::validate::{self, Invalid};
use crate::metrics::FlowName;

/// Request body. Hex fields are typed: length and format are checked by deserialisation.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PublicDecryptRequest {
    pub ciphertext_handles: Vec<B256>,
    pub extra_data: Bytes,
}

/// The connector's cheap handle rules and the `extraData` format.
pub fn validate(request: &PublicDecryptRequest, http: &HttpConfig) -> Result<(), Invalid> {
    validate::handles("ciphertextHandles", request.ciphertext_handles.iter(), http)?;
    validate::extra_data("extraData", &request.extra_data)
}

/// One request, summarized in its own `Summary`.
pub async fn handle(app: State<App>, request: Request) -> Response {
    let mut guard = Guard::new(FlowName::PublicDecrypt, app.metrics.clone());
    let result = process(&app, request, &mut guard.summary).await;
    guard.disarm();
    result.unwrap_or_else(IntoResponse::into_response)
}

/// Read and parse the body (bounded by `http.body_read_timeout`), validate, convert to the connector DTO,
/// aggregate, answer.
async fn process(app: &App, request: Request, summary: &mut Summary) -> Result<Response, ApiError> {
    let request: PublicDecryptRequest = read_json(app, request, summary).await?;
    summary.received(
        &app.http,
        request.ciphertext_handles.clone(),
        Vec::new(),
        Vec::new(),
        None,
    );
    validate(&request, &app.http).map_err(|e| super::invalid(summary, e))?;
    let request = PublicDecryptionRequest::from(request);
    summary.forwarded(request.id());
    let result = app
        .public_decrypt
        .run(&summary.request_id, request, &mut summary.tally)
        .await;
    match result {
        Ok(output) => Ok(Reply::succeeded(summary.take(), output).into_response()),
        Err(e) => Err(ApiError::aggregation(summary, e)),
    }
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
    use crate::kms_aggregator::mock::EXTRA_DATA_V2;

    pub(crate) fn wire() -> String {
        format!(
            r#"{{ "ciphertextHandles": ["{}", "{}"], "extraData": "{EXTRA_DATA_V2}" }}"#,
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
        assert_eq!(
            validate(&request, &HttpConfig::for_tests(&[1, 137])),
            Ok(())
        );
        assert!(
            serde_json::from_str::<PublicDecryptRequest>(&format!(
                r#"{{"ciphertextHandles": [], "extraData": "{EXTRA_DATA_V2}", "x": 1}}"#
            ))
            .is_err()
        );
        assert!(
            serde_json::from_str::<PublicDecryptRequest>(&format!(
                r#"{{"ciphertextHandles": ["0x12"], "extraData": "{EXTRA_DATA_V2}"}}"#
            ))
            .is_err()
        );
    }

    #[test]
    fn rules_name_their_field() {
        let mut empty = valid();
        empty.ciphertext_handles.clear();
        assert_eq!(
            validate(&empty, &HttpConfig::for_tests(&[137]))
                .unwrap_err()
                .field,
            "ciphertextHandles"
        );
        assert_eq!(
            validate(&valid(), &HttpConfig::for_tests(&[1]))
                .unwrap_err()
                .field,
            "ciphertextHandles[0]"
        );
        let mut bad_extra = valid();
        bad_extra.extra_data = Bytes::from_static(&[7]);
        assert_eq!(
            validate(&bad_extra, &HttpConfig::for_tests(&[137]))
                .unwrap_err()
                .field,
            "extraData"
        );
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
