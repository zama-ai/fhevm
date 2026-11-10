//! `POST /v4/public-decrypt`: the current relayer's body, validated, converted to the connector DTO and
//! aggregated. A body whose first `ctHandles` entry names a Solana chain is the connector's Solana DTO, parsed,
//! checked and forwarded equal field by field.

use alloy::primitives::{B256, Bytes};
use axum::extract::{Request, State};
use axum::response::{IntoResponse, Response};
use kms_connector_api::{PublicDecryptionRequest, SolanaPublicDecryptionRequest};
use serde::Deserialize;
use serde_json::value::RawValue;
use zama_solana_acl::host_chain::{handle_chain_id, is_solana_host_chain_id};
use zama_solana_request::{SolanaRequestError, public_request_chain_id};

use super::{Reply, field, invalid, parse_raw, read_json};
use crate::App;
use crate::config::HttpConfig;
use crate::endpoint::ApiError;
use crate::endpoint::summary::{Guard, Summary};
use crate::endpoint::validate::{self, Invalid};
use crate::kms_aggregator::PublicRequest;
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
    validate::evm_handles("ciphertextHandles", request.ciphertext_handles.iter(), http)?;
    validate::extra_data("extraData", &request.extra_data)
}

/// One request, summarized in its own `Summary`.
pub async fn handle(app: State<App>, request: Request) -> Response {
    let mut guard = Guard::new(FlowName::PublicDecrypt, app.metrics.clone());
    let result = process(&app, request, &mut guard.summary).await;
    guard.disarm();
    result.unwrap_or_else(IntoResponse::into_response)
}

/// Whether the body is the Solana DTO: its first `ctHandles` entry names a Solana chain. The EVM body names its
/// handles `ciphertextHandles`, so any other body is the EVM one, whose parse refuses it.
pub fn is_solana(body: &RawValue) -> bool {
    field(body, "ctHandles")
        .and_then(|handles| handles.get(0)?.as_str()?.parse::<B256>().ok())
        .is_some_and(|handle| is_solana_host_chain_id(handle_chain_id(&handle.0)))
}

/// The connector's cheap handle rules, the `extraData` format, then the Solana form (one store per handle, the
/// handle cap, one Solana chain) through `zama-solana-request`, as the connector checks it.
pub fn validate_solana(
    request: &SolanaPublicDecryptionRequest,
    http: &HttpConfig,
) -> Result<(), Invalid> {
    validate::handles("ctHandles", request.ctHandles.iter(), http)?;
    validate::extra_data("extraData", &request.extraData)?;
    let handles: Vec<[u8; 32]> = request.ctHandles.iter().map(|h| h.0).collect();
    public_request_chain_id(&handles, request.encryptedStores.len(), &request.extraData).map_err(
        |e| {
            let field = match e {
                SolanaRequestError::StoreCount { .. } => "encryptedStores",
                SolanaRequestError::NoKmsContext => "extraData",
                _ => "ctHandles",
            };
            Invalid::new(field, e.to_string())
        },
    )?;
    Ok(())
}

/// Read the body (bounded by `http.body_read_timeout`), dispatch on its handles' chain, validate, convert the EVM
/// body to the connector DTO (the Solana body is one), aggregate, answer.
async fn process(app: &App, request: Request, summary: &mut Summary) -> Result<Response, ApiError> {
    let body: Box<RawValue> = read_json(app, request, summary).await?;
    let request = if is_solana(&body) {
        let request: SolanaPublicDecryptionRequest =
            parse_raw(&body).map_err(|reason| ApiError::malformed(summary, reason))?;
        summary.received(
            &app.http,
            request.ctHandles.clone(),
            Vec::new(),
            Vec::new(),
            None,
        );
        validate_solana(&request, &app.http).map_err(|e| invalid(summary, e))?;
        summary.forwarded(request.id());
        PublicRequest::Solana(request)
    } else {
        let request: PublicDecryptRequest =
            parse_raw(&body).map_err(|reason| ApiError::malformed(summary, reason))?;
        summary.received(
            &app.http,
            request.ciphertext_handles.clone(),
            Vec::new(),
            Vec::new(),
            None,
        );
        validate(&request, &app.http).map_err(|e| invalid(summary, e))?;
        let request = PublicDecryptionRequest::from(request);
        summary.forwarded(request.id());
        PublicRequest::Evm(request)
    };
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
    use crate::endpoint::validate::tests::{SOLANA, handle};
    use crate::kms_aggregator::mock::EXTRA_DATA_V2;

    /// A Solana public body: two handles on `SOLANA` and their stores.
    pub(crate) fn public_body() -> SolanaPublicDecryptionRequest {
        SolanaPublicDecryptionRequest {
            ctHandles: vec![handle(SOLANA, 0), handle(SOLANA, 5)],
            extraData: Bytes::from([[0x02].as_slice(), &[0xaa; 64]].concat()),
            encryptedStores: vec![B256::repeat_byte(0x30), B256::repeat_byte(0x31)],
        }
    }

    fn raw(json: &str) -> Box<RawValue> {
        RawValue::from_string(json.to_owned()).unwrap()
    }

    #[test]
    fn the_first_ct_handle_names_the_solana_body() {
        assert!(!is_solana(&raw(&wire())));
        let mut solana = serde_json::to_value(public_body()).unwrap();
        assert!(is_solana(&raw(&solana.to_string())));
        solana["ctHandles"][0] = serde_json::json!(handle(137, 0));
        assert!(!is_solana(&raw(&solana.to_string())));
    }

    #[test]
    fn each_solana_public_rule_names_its_field() {
        type Mutation = Box<dyn Fn(&mut SolanaPublicDecryptionRequest)>;
        let cases: Vec<(Mutation, &str)> = vec![
            (Box::new(|r| r.ctHandles.clear()), "ctHandles"),
            (
                Box::new(|r| r.ctHandles[1] = handle(31337, 5)),
                "ctHandles[1]",
            ),
            (
                Box::new(|r| r.extraData = Bytes::from_static(&[9])),
                "extraData",
            ),
            // Valid v0 names no KMS context.
            (
                Box::new(|r| r.extraData = Bytes::from_static(&[0])),
                "extraData",
            ),
            (
                Box::new(|r| r.encryptedStores.truncate(1)),
                "encryptedStores",
            ),
            // A served EVM chain is not a Solana chain.
            (
                Box::new(|r| r.ctHandles = vec![handle(1, 0), handle(1, 5)]),
                "ctHandles",
            ),
        ];
        for (mutate, field) in cases {
            let mut request = public_body();
            mutate(&mut request);
            let error =
                validate_solana(&request, &HttpConfig::for_tests(&[1, SOLANA])).unwrap_err();
            assert_eq!(error.field, field, "{error}");
        }
    }

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
