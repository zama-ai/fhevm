//! `POST /v4/user-decrypt`: the connector's v1 envelope topology (`attestationType`, `payload`, `signature`)
//! with the relayer's own field names inside, validated, converted to the connector DTO and aggregated.
//! `attestationType: "solana-srfc38-user-decrypt-v1"` is the connector's Solana DTO, checked and forwarded equal
//! field by field.

use alloy::primitives::{Address, B256, Bytes};
use axum::extract::{Request, State};
use axum::response::{IntoResponse, Response};
use kms_connector_api::{
    AttestationType, HandleEntry as ConnectorHandle, RequestValidity as ConnectorValidity,
    SolanaUserDecryptionRequest, UserDecryptionPayload, UserDecryptionRequest,
};
use serde::{Deserialize, Deserializer};
use serde_json::value::RawValue;

use super::{Reply, field, invalid, parse, read_json, rejected};
use crate::App;
use crate::endpoint::ApiError;
use crate::endpoint::validate::{self, Invalid};
use crate::kms_aggregator::UserRequest;
use crate::logging::Log;

pub const ATTESTATION_TYPE: &str = "eip712-unified-user-decrypt-v1";
/// The connector's cap.
const MAX_ALLOWED_CONTRACTS: usize = 10;

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
pub fn validate(request: &UserDecryptRequest, chain_ids: &[u64], now: u64) -> Result<(), Invalid> {
    validate::expect(
        "attestationType",
        &request.attestation_type,
        ATTESTATION_TYPE,
    )?;
    let payload = &request.payload;
    validate::evm_handles(
        "payload.handles",
        payload.handles.iter().map(|h| &h.ct_handle),
        chain_ids,
    )?;
    if payload.allowed_contracts.len() > MAX_ALLOWED_CONTRACTS {
        return Err(Invalid::new(
            "payload.allowedContracts",
            format!("at most {MAX_ALLOWED_CONTRACTS} contracts"),
        ));
    }
    if payload.public_key.is_empty() {
        return Err(Invalid::new("payload.publicKey", "must not be empty"));
    }
    let validity = &payload.request_validity;
    validate::request_window(
        "payload.requestValidity",
        validity.start_timestamp,
        validity.duration_seconds,
        now,
    )?;
    validate::extra_data("payload.extraData", &payload.extra_data)
}

/// The connector's cheap handle rules, the validity window and the `extraData` format, then the Solana form
/// through the connector crate's `assemble()`. The permit's signature and program are the connector worker's.
pub fn validate_solana(
    request: &SolanaUserDecryptionRequest,
    chain_ids: &[u64],
    now: u64,
) -> Result<(), Invalid> {
    let payload = &request.payload;
    validate::handles(
        "payload.handles",
        payload.handles.iter().map(|h| &h.handle),
        chain_ids,
    )?;
    let validity = &payload.requestValidity;
    validate::request_window(
        "payload.requestValidity",
        validity.startTimestamp,
        validity.durationSeconds,
        now,
    )?;
    validate::extra_data("payload.extraData", &payload.extraData)?;
    request
        .assemble()
        .map_err(|e| Invalid::new("payload", e.to_string()))?;
    Ok(())
}

/// One request, logged step by step under its own `Log`.
pub async fn handle(app: State<App>, request: Request) -> Response {
    let mut log = Log::new("user_decrypt");
    process(&app, request, &mut log)
        .await
        .unwrap_or_else(IntoResponse::into_response)
}

/// Read the body (bounded by `http.body_read_timeout`), dispatch on its `attestationType`, validate,
/// aggregate, answer.
async fn process(app: &App, request: Request, log: &mut Log) -> Result<Response, ApiError> {
    let body = read_json(app, request, log).await?;
    let attestation = field(&body, "attestationType").and_then(|t| t.as_str()?.parse().ok());
    if attestation == Some(AttestationType::SolanaSrfc38UserDecryptV1) {
        process_solana(app, &body, log).await
    } else {
        process_evm(app, &body, log).await
    }
}

/// The Solana body: checked, then forwarded as parsed.
async fn process_solana(app: &App, body: &RawValue, log: &mut Log) -> Result<Response, ApiError> {
    let request: SolanaUserDecryptionRequest = parse(body).map_err(|e| rejected(log, e))?;
    log.received(request.payload.handles.iter().map(|h| h.handle).collect());
    validate_solana(&request, &app.http.supported_chain_ids, super::now())
        .map_err(|e| invalid(log, e))?;
    log.forwarded(request.id());
    let output = app
        .user_decrypt
        .run(&log.request_id, UserRequest::Solana(request))
        .await
        .map_err(|e| ApiError::aggregation(log, e))?;
    Ok(Reply::succeeded(log.clone(), output).into_response())
}

/// The EVM body: validated, converted to the connector DTO, aggregated.
async fn process_evm(app: &App, body: &RawValue, log: &mut Log) -> Result<Response, ApiError> {
    let request: UserDecryptRequest = parse(body).map_err(|e| rejected(log, e))?;
    log.received(
        request
            .payload
            .handles
            .iter()
            .map(|h| h.ct_handle)
            .collect(),
    );
    validate(&request, &app.http.supported_chain_ids, super::now()).map_err(|e| invalid(log, e))?;
    let request = UserDecryptionRequest::from(request);
    log.forwarded(request.id());
    let output = app
        .user_decrypt
        .run(&log.request_id, UserRequest::Evm(request))
        .await
        .map_err(|e| ApiError::aggregation(log, e))?;
    Ok(Reply::succeeded(log.clone(), output).into_response())
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
    use kms_connector_api::{SolanaHandleEntry, SolanaUserDecryptionPayload};
    use zama_solana_permit::TRANSPORT_KEY_LEN;

    use super::*;
    use crate::endpoint::validate::tests::{SOLANA, handle};

    pub(crate) const NOW: u64 = 1_800_000_000;

    /// A Solana request for one handle on `SOLANA`, valid from `start` for 600 s. Its signature is 64 zero bytes:
    /// the relayer checks only its length.
    pub(crate) fn user_body(start: u64) -> SolanaUserDecryptionRequest {
        SolanaUserDecryptionRequest {
            attestationType: AttestationType::SolanaSrfc38UserDecryptV1.to_string(),
            payload: SolanaUserDecryptionPayload {
                handles: vec![SolanaHandleEntry {
                    handle: handle(SOLANA, 4),
                    ownerAddress: B256::repeat_byte(7),
                    encryptedStore: B256::repeat_byte(3),
                }],
                userAddress: B256::repeat_byte(7),
                publicKey: Bytes::from(vec![0x5a; TRANSPORT_KEY_LEN]),
                allowedScopes: vec![],
                requestValidity: kms_connector_api::RequestValidity {
                    startTimestamp: start,
                    durationSeconds: 600,
                },
                verifyingProgramId: B256::repeat_byte(0x4c),
                extraData: Bytes::from([[2u8; 1].as_slice(), &[0xbb; 32], &[0x77; 32]].concat()),
            },
            signature: Bytes::from(vec![0; 64]),
        }
    }

    #[test]
    fn a_solana_request_passes_and_each_rule_names_its_field() {
        assert_eq!(
            validate_solana(&user_body(NOW - 10), &[1, SOLANA], NOW),
            Ok(())
        );
        type Mutation = Box<dyn Fn(&mut SolanaUserDecryptionRequest)>;
        let cases: Vec<(Mutation, &str)> = vec![
            (
                Box::new(|r| r.payload.handles[0].handle = handle(31337, 4)),
                "payload.handles[0]",
            ),
            (
                Box::new(|r| r.payload.requestValidity.startTimestamp = NOW + 1),
                "payload.requestValidity.startTimestamp",
            ),
            (
                Box::new(|r| r.payload.requestValidity.durationSeconds = 5),
                "payload.requestValidity",
            ),
            (
                Box::new(|r| r.payload.extraData = Bytes::from_static(&[9])),
                "payload.extraData",
            ),
            // A served EVM chain is not a Solana chain.
            (
                Box::new(|r| r.payload.handles[0].handle = handle(1, 4)),
                "payload",
            ),
            (
                Box::new(|r| r.payload.publicKey = Bytes::from_static(&[1])),
                "payload",
            ),
            (
                Box::new(|r| r.signature = Bytes::from_static(&[1])),
                "payload",
            ),
        ];
        for (mutate, field) in cases {
            let mut request = user_body(NOW - 10);
            mutate(&mut request);
            let error = validate_solana(&request, &[1, SOLANA], NOW).unwrap_err();
            assert_eq!(error.field, field, "{error}");
        }
    }

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
                "extraData": "0x00"
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
    fn valid_request_passes() {
        assert_eq!(validate(&valid(), &[1, 137], NOW), Ok(()));
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
            let error = validate(&request, &[1, 137], NOW).unwrap_err();
            assert_eq!(error.field, field, "{error}");
        }
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
