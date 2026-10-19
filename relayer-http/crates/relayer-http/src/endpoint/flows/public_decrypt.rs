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

use super::{Reply, field, invalid, parse, read_json, rejected};
use crate::App;
use crate::endpoint::ApiError;
use crate::endpoint::validate::{self, Invalid};
use crate::kms_aggregator::PublicRequest;
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
    validate::evm_handles(
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
    chain_ids: &[u64],
) -> Result<(), Invalid> {
    validate::handles("ctHandles", request.ctHandles.iter(), chain_ids)?;
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

/// Read the body (bounded by `http.body_read_timeout`), dispatch on its handles' chain, validate, aggregate,
/// answer.
async fn process(app: &App, request: Request, log: &mut Log) -> Result<Response, ApiError> {
    let body = read_json(app, request, log).await?;
    if is_solana(&body) {
        process_solana(app, &body, log).await
    } else {
        process_evm(app, &body, log).await
    }
}

/// The Solana body: checked, forwarded as parsed, aggregated.
async fn process_solana(app: &App, body: &RawValue, log: &mut Log) -> Result<Response, ApiError> {
    let request: SolanaPublicDecryptionRequest = parse(body).map_err(|e| rejected(log, e))?;
    log.received(request.ctHandles.clone());
    validate_solana(&request, &app.http.supported_chain_ids).map_err(|e| invalid(log, e))?;
    log.forwarded(request.id());
    let output = app
        .public_decrypt
        .run(&log.request_id, PublicRequest::Solana(request))
        .await
        .map_err(|e| ApiError::aggregation(log, e))?;
    Ok(Reply::succeeded(log.clone(), output).into_response())
}

/// The EVM body: validated, converted to the connector DTO, aggregated.
async fn process_evm(app: &App, body: &RawValue, log: &mut Log) -> Result<Response, ApiError> {
    let request: PublicDecryptRequest = parse(body).map_err(|e| rejected(log, e))?;
    log.received(request.ciphertext_handles.clone());
    validate(&request, &app.http.supported_chain_ids).map_err(|e| invalid(log, e))?;
    let request = PublicDecryptionRequest::from(request);
    log.forwarded(request.id());
    let output = app
        .public_decrypt
        .run(&log.request_id, PublicRequest::Evm(request))
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
    use crate::endpoint::validate::tests::{SOLANA, handle};

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
            let error = validate_solana(&request, &[1, SOLANA]).unwrap_err();
            assert_eq!(error.field, field, "{error}");
        }
    }

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
