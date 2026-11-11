//! `POST /v4/user-decrypt`: the connector's v1 envelope topology (`attestationType`, `payload`, `signature`)
//! with the relayer's own field names inside, validated, converted to the connector DTO and aggregated.

use std::collections::HashSet;

use alloy::primitives::{Address, B256, Bytes};
use axum::extract::{Request, State};
use axum::response::{IntoResponse, Response};
use kms_connector_api::{
    HandleEntry as ConnectorHandle, RequestValidity as ConnectorValidity, UserDecryptionPayload,
    UserDecryptionRequest,
};
use serde::{Deserialize, Deserializer};

use super::{Reply, read_json};
use crate::App;
use crate::config::HttpConfig;
use crate::endpoint::ApiError;
use crate::endpoint::summary::{Guard, Summary};
use crate::endpoint::validate::{self, Invalid};
use crate::metrics::FlowName;

pub const ATTESTATION_TYPE: &str = "eip712-unified-user-decrypt-v1";
/// The connector's longest `requestValidity.durationSeconds`: one year.
const MAX_DURATION_SECONDS: u64 = 365 * 24 * 60 * 60;

/// Request body. Hex fields are typed: length and format are checked by deserialisation.
///
/// Owned by the relayer (not the connector's DTO) so relayer-only fields can be added later.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UserDecryptRequest {
    /// `eip712-unified-user-decrypt-v1`. Forwarded as received because it is part of the connector's
    /// `decryptionId`.
    pub attestation_type: String,
    pub payload: UserDecryptPayload,
    /// The user's EIP-712 signature; empty on the ERC-1271 path. Verified by the connector, not here.
    pub signature: Bytes,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UserDecryptPayload {
    pub handles: Vec<HandleEntry>,
    pub user_address: Address,
    pub allowed_contracts: Vec<Address>,
    pub request_validity: RequestValidity,
    pub public_key: Bytes,
    pub extra_data: Bytes,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HandleEntry {
    pub ct_handle: B256,
    pub contract_address: Address,
    pub owner_address: Address,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RequestValidity {
    #[serde(deserialize_with = "u64_from_string_or_number")]
    pub start_timestamp: u64,
    #[serde(deserialize_with = "u64_from_string_or_number")]
    pub duration_seconds: u64,
}

/// The SDK sends these as decimal strings; numbers are accepted too.
fn u64_from_string_or_number<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Raw {
        Number(u64),
        Text(String),
    }
    match Raw::deserialize(deserializer)? {
        Raw::Number(value) => Ok(value),
        Raw::Text(text) => text
            .parse()
            .map_err(|_| serde::de::Error::custom("must be a decimal integer")),
    }
}

/// The current relayer's rules plus the connector's cheap handle rules. `now` in seconds since the epoch.
pub fn validate(request: &UserDecryptRequest, http: &HttpConfig, now: u64) -> Result<(), Invalid> {
    validate::expect(
        "attestationType",
        &request.attestation_type,
        ATTESTATION_TYPE,
    )?;
    let payload = &request.payload;
    validate::handles(
        "payload.handles",
        payload.handles.iter().map(|h| &h.ct_handle),
        http,
    )?;
    let max_contracts = http.max_allowed_contracts;
    if payload.allowed_contracts.len() > max_contracts {
        return Err(Invalid::new(
            "payload.allowedContracts",
            format!("at most {max_contracts} contracts"),
        )
        .with_value(format_args!(
            "{} contracts",
            payload.allowed_contracts.len()
        )));
    }
    if payload.public_key.is_empty() {
        return Err(Invalid::new("payload.publicKey", "must not be empty"));
    }
    let validity = &payload.request_validity;
    if validity.duration_seconds > MAX_DURATION_SECONDS {
        return Err(Invalid::new(
            "payload.requestValidity.durationSeconds",
            format!("at most {MAX_DURATION_SECONDS} seconds"),
        )
        .with_value(validity.duration_seconds));
    }
    if validity.start_timestamp > now {
        return Err(Invalid::new(
            "payload.requestValidity.startTimestamp",
            "must not be in the future",
        ));
    }
    if validity
        .start_timestamp
        .saturating_add(validity.duration_seconds)
        <= now
    {
        return Err(Invalid::new(
            "payload.requestValidity",
            "window has already expired",
        ));
    }
    validate::extra_data("payload.extraData", &payload.extra_data)
}

/// One request, summarized in its own `Summary`.
pub async fn handle(app: State<App>, request: Request) -> Response {
    let mut guard = Guard::new(FlowName::UserDecrypt, app.metrics.clone());
    let result = process(&app, request, &mut guard.summary).await;
    guard.disarm();
    result.unwrap_or_else(IntoResponse::into_response)
}

/// Read and parse the body (bounded by `http.body_read_timeout`), validate, convert to the connector DTO,
/// aggregate, answer.
async fn process(app: &App, request: Request, summary: &mut Summary) -> Result<Response, ApiError> {
    let request: UserDecryptRequest = read_json(app, request, summary).await?;
    summary.received(
        &app.http,
        request
            .payload
            .handles
            .iter()
            .map(|h| h.ct_handle)
            .collect(),
        handle_contracts(&request.payload),
        request.payload.allowed_contracts.clone(),
        Some(request.payload.user_address),
    );
    validate(&request, &app.http, super::now()).map_err(|e| super::invalid(summary, e))?;
    let request = UserDecryptionRequest::from(request);
    summary.forwarded(request.id());
    let result = app
        .user_decrypt
        .run(&summary.request_id, request, &mut summary.tally)
        .await;
    match result {
        Ok(output) => Ok(Reply::succeeded(summary.take(), output).into_response()),
        Err(e) => Err(ApiError::aggregation(summary, e)),
    }
}

/// The contracts of the handles, once each, in order. Linear: the handles are not validated yet.
fn handle_contracts(payload: &UserDecryptPayload) -> Vec<Address> {
    let mut seen = HashSet::new();
    payload
        .handles
        .iter()
        .map(|h| h.contract_address)
        .filter(|address| seen.insert(*address))
        .collect()
}

/// The body the connector receives (RFC-033), field by field.
impl From<UserDecryptRequest> for UserDecryptionRequest {
    fn from(request: UserDecryptRequest) -> Self {
        let payload = request.payload;
        Self {
            attestationType: request.attestation_type,
            payload: UserDecryptionPayload {
                handles: payload
                    .handles
                    .into_iter()
                    .map(|h| ConnectorHandle {
                        handle: h.ct_handle,
                        contractAddress: h.contract_address,
                        ownerAddress: h.owner_address,
                    })
                    .collect(),
                userAddress: payload.user_address,
                publicKey: payload.public_key,
                allowedContracts: payload.allowed_contracts,
                requestValidity: ConnectorValidity {
                    startTimestamp: payload.request_validity.start_timestamp,
                    durationSeconds: payload.request_validity.duration_seconds,
                },
                extraData: payload.extra_data,
            },
            signature: request.signature,
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use alloy::primitives::address;

    use super::*;
    use crate::endpoint::validate::tests::handle;
    use crate::kms_aggregator::mock::EXTRA_DATA_V2;

    pub(crate) const NOW: u64 = 1_800_000_000;

    /// The SDK's wire shape, one handle on chain 1.
    pub(crate) fn wire(start: &str, duration: &str) -> String {
        format!(
            r#"{{
              "attestationType": "eip712-unified-user-decrypt-v1",
              "payload": {{
                "handles": [{{
                  "ctHandle": "{}",
                  "contractAddress": "0x3333333333333333333333333333333333333333",
                  "ownerAddress": "0x4444444444444444444444444444444444444444"
                }}],
                "userAddress": "0x5555555555555555555555555555555555555555",
                "allowedContracts": ["0x3333333333333333333333333333333333333333"],
                "requestValidity": {{ "startTimestamp": {start}, "durationSeconds": {duration} }},
                "publicKey": "0x20002000",
                "extraData": "{EXTRA_DATA_V2}"
              }},
              "signature": "0x{}"
            }}"#,
            handle(1, 4),
            "66".repeat(65)
        )
    }

    pub(crate) fn valid() -> UserDecryptRequest {
        serde_json::from_str(&wire("\"1799999900\"", "\"600\"")).unwrap()
    }

    #[test]
    fn attestation_type_matches_the_connector_crate() {
        assert_eq!(
            ATTESTATION_TYPE,
            <&'static str>::from(kms_connector_api::AttestationType::Eip712UnifiedUserDecryptV1)
        );
    }

    #[test]
    fn wire_example_deserialises_with_string_or_number_timestamps() {
        let text = valid();
        let numbers: UserDecryptRequest = serde_json::from_str(&wire("1799999900", "600")).unwrap();
        for r in [text, numbers] {
            assert_eq!(r.payload.request_validity.start_timestamp, 1_799_999_900);
            assert_eq!(r.payload.request_validity.duration_seconds, 600);
            assert_eq!(r.payload.handles[0].ct_handle, handle(1, 4));
            assert_eq!(r.signature.len(), 65);
        }
    }

    #[test]
    fn deserialisation_rejects_unknown_fields_and_bad_hex() {
        let unknown = wire("1", "2").replace(r#""publicKey""#, r#""extra": 1, "publicKey""#);
        assert!(serde_json::from_str::<UserDecryptRequest>(&unknown).is_err());
        let short_handle = wire("1", "2").replace(&handle(1, 4).to_string(), "0x1234");
        assert!(serde_json::from_str::<UserDecryptRequest>(&short_handle).is_err());
        let bad_timestamp = wire("\"soon\"", "2");
        assert!(serde_json::from_str::<UserDecryptRequest>(&bad_timestamp).is_err());
    }

    #[test]
    fn handle_contracts_lists_each_contract_once_in_order() {
        let (a, b) = (Address::repeat_byte(0xaa), Address::repeat_byte(0xbb));
        let mut payload = valid().payload;
        let entry = payload.handles[0].clone();
        payload.handles = [a, b, a, b, a]
            .into_iter()
            .map(|contract_address| HandleEntry {
                contract_address,
                ..entry.clone()
            })
            .collect();
        assert_eq!(handle_contracts(&payload), [a, b]);
        payload.handles.clear();
        assert!(handle_contracts(&payload).is_empty());
    }

    #[test]
    fn valid_request_passes() {
        assert_eq!(
            validate(&valid(), &HttpConfig::for_tests(&[1, 137]), NOW),
            Ok(())
        );
    }

    #[test]
    fn each_rule_names_its_field() {
        type Mutation = Box<dyn Fn(&mut UserDecryptRequest)>;
        let cases: Vec<(Mutation, &str)> = vec![
            (
                Box::new(|r| r.attestation_type = "other".into()),
                "attestationType",
            ),
            (Box::new(|r| r.payload.handles.clear()), "payload.handles"),
            (
                Box::new(|r| r.payload.handles[0].ct_handle = handle(31337, 4)),
                "payload.handles[0]",
            ),
            (
                Box::new(|r| {
                    r.payload.allowed_contracts =
                        vec![address!("0x3333333333333333333333333333333333333333"); 11]
                }),
                "payload.allowedContracts",
            ),
            (
                Box::new(|r| r.payload.public_key = Bytes::new()),
                "payload.publicKey",
            ),
            (
                Box::new(|r| {
                    r.payload.request_validity.duration_seconds = MAX_DURATION_SECONDS + 1
                }),
                "payload.requestValidity.durationSeconds",
            ),
            (
                Box::new(|r| r.payload.request_validity.start_timestamp = NOW + 1),
                "payload.requestValidity.startTimestamp",
            ),
            (
                Box::new(|r| r.payload.request_validity.duration_seconds = 1),
                "payload.requestValidity",
            ),
            (
                Box::new(|r| r.payload.extra_data = Bytes::from_static(&[9])),
                "payload.extraData",
            ),
        ];
        for (mutate, field) in cases {
            let mut request = valid();
            mutate(&mut request);
            let error = validate(&request, &HttpConfig::for_tests(&[1, 137]), NOW).unwrap_err();
            assert_eq!(error.field, field, "{error}");
        }
    }

    #[test]
    fn a_limit_error_names_the_offending_value() {
        let mut request = valid();
        request.payload.allowed_contracts =
            vec![address!("0x3333333333333333333333333333333333333333"); 11];
        let error = validate(&request, &HttpConfig::for_tests(&[1, 137]), NOW).unwrap_err();
        assert_eq!(error.value.as_deref(), Some("11 contracts"));
        let mut request = valid();
        request.payload.request_validity.duration_seconds = MAX_DURATION_SECONDS + 1;
        let error = validate(&request, &HttpConfig::for_tests(&[1, 137]), NOW).unwrap_err();
        assert_eq!(error.value, Some((MAX_DURATION_SECONDS + 1).to_string()));
    }

    #[test]
    fn conversion_keeps_every_field() {
        let request = valid();
        let connector = UserDecryptionRequest::from(request.clone());
        assert_eq!(connector.attestationType, request.attestation_type);
        assert_eq!(connector.payload.handles.len(), 1);
        assert_eq!(connector.payload.handles[0].handle, handle(1, 4));
        assert_eq!(
            connector.payload.handles[0].contractAddress,
            address!("0x3333333333333333333333333333333333333333")
        );
        assert_eq!(
            connector.payload.handles[0].ownerAddress,
            address!("0x4444444444444444444444444444444444444444")
        );
        assert_eq!(connector.payload.userAddress, request.payload.user_address);
        assert_eq!(connector.payload.publicKey, request.payload.public_key);
        assert_eq!(
            connector.payload.allowedContracts,
            request.payload.allowed_contracts
        );
        assert_eq!(
            connector.payload.requestValidity.startTimestamp,
            1_799_999_900
        );
        assert_eq!(connector.payload.requestValidity.durationSeconds, 600);
        assert_eq!(connector.signature, request.signature);
        assert_eq!(connector.payload.extraData, request.payload.extra_data);
        // The connector derives the same id from this body on every node.
        assert_ne!(connector.id(), B256::ZERO);
    }
}
