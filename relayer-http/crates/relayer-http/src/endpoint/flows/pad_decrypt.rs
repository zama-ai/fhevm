//! `POST /v4/pad-decrypt` (RFC-030): the connector's v1 pad envelope (`attestationType`, `payload`, `signature`)
//! with the relayer's own types inside, validated, converted to the connector DTO and aggregated.

use std::collections::HashSet;

use alloy::primitives::{Address, B128, B256, Bytes};
use axum::extract::{Request, State};
use axum::response::{IntoResponse, Response};
use kms_connector_api::{
    MAX_PAD_DECRYPTORS, MAX_PAD_LENGTH, PAD_WITNESS_VERSION, PadDecryptionPayload,
    PadDecryptionRequest, PadEntry as ConnectorPad, PadWitness as ConnectorWitness,
    RequestValidity as ConnectorValidity,
};
use serde::Deserialize;

use super::user_decrypt::{ATTESTATION_TYPE, RequestValidity};
use super::{Reply, read_json, u64_from_string_or_number};
use crate::App;
use crate::config::HttpConfig;
use crate::endpoint::ApiError;
use crate::endpoint::summary::{Guard, Summary};
use crate::endpoint::validate::{self, Invalid};
use crate::metrics::FlowName;

/// Request body. Hex fields are typed: length and format are checked by deserialisation.
///
/// Owned by the relayer (not the connector's DTO) so the SDK's decimal-string integers are accepted.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PadDecryptRequest {
    /// `eip712-unified-user-decrypt-v1`: the RFC-016 signature of user decryption is reused as it is.
    pub attestation_type: String,
    pub payload: PadDecryptPayload,
    /// The user's EIP-712 signature; empty on the ERC-1271 path. Verified by the connector, not here.
    pub signature: Bytes,
}

/// The witness fields shared by every pad, the pads, then the RFC-016 signed fields.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PadDecryptPayload {
    pub witness_version: u8,
    pub prf_id: u16,
    #[serde(deserialize_with = "u64_from_string_or_number")]
    pub chain_id: u64,
    pub pads: Vec<PadEntry>,
    pub user_address: Address,
    pub public_key: Bytes,
    pub allowed_contracts: Vec<Address>,
    pub request_validity: RequestValidity,
    pub extra_data: Bytes,
}

/// One pad: its label, the witness authenticating it, and the number of pad bits to release.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PadEntry {
    pub label: B256,
    pub witness: PadWitness,
    pub pad_length: u16,
}

/// The per-pad fields of a label witness.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PadWitness {
    pub submitter: Address,
    pub consumer: Address,
    #[serde(deserialize_with = "u64_from_string_or_number")]
    pub expiration_timestamp: u64,
    pub decryptors: Vec<Address>,
    pub nonce: B128,
}

/// The label of `witness` as the connector derives it (`kms_connector_api::PadWitness::label`), under the payload's
/// shared fields. Panics past 255 decryptors: call it only once their count is validated.
pub fn label(payload: &PadDecryptPayload, witness: &PadWitness) -> B256 {
    ConnectorWitness::from(witness.clone()).label(
        payload.witness_version,
        payload.prf_id,
        payload.chain_id,
    )
}

/// The user-decrypt rules on the signed fields, then the connector's cheap pad rules, in its order.
/// `now` in seconds since the epoch.
pub fn validate(request: &PadDecryptRequest, http: &HttpConfig, now: u64) -> Result<(), Invalid> {
    validate::expect(
        "attestationType",
        &request.attestation_type,
        ATTESTATION_TYPE,
    )?;
    let payload = &request.payload;
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
    validate::request_validity(
        "payload.requestValidity",
        validity.start_timestamp,
        validity.duration_seconds,
        now,
    )?;
    validate::extra_data("payload.extraData", &payload.extra_data)?;
    if payload.witness_version != PAD_WITNESS_VERSION {
        return Err(Invalid::new(
            "payload.witnessVersion",
            format!("must be {PAD_WITNESS_VERSION}"),
        )
        .with_value(payload.witness_version));
    }
    if !http.supported_chain_ids.contains(&payload.chain_id) {
        return Err(Invalid::new(
            "payload.chainId",
            format!("chain id {} is not supported", payload.chain_id),
        )
        .with_value(payload.chain_id));
    }
    let max_pads = http.max_pads;
    if payload.pads.is_empty() || payload.pads.len() > max_pads {
        return Err(
            Invalid::new("payload.pads", format!("must hold 1 to {max_pads} pads"))
                .with_value(format_args!("{} pads", payload.pads.len())),
        );
    }
    let mut labels = HashSet::with_capacity(payload.pads.len());
    for (i, pad) in payload.pads.iter().enumerate() {
        if !labels.insert(pad.label) {
            return Err(
                Invalid::new(format!("payload.pads[{i}].label"), "appears twice")
                    .with_value(pad.label),
            );
        }
        pad_rules(payload, pad, now).map_err(|(field, issue)| {
            Invalid::new(format!("payload.pads[{i}]{field}"), issue).with_value(pad.label)
        })?;
    }
    Ok(())
}

/// One pad: its field suffix and the issue on failure.
fn pad_rules(
    payload: &PadDecryptPayload,
    pad: &PadEntry,
    now: u64,
) -> Result<(), (String, String)> {
    let fail = |field: &str, issue: &str| Err((field.to_owned(), issue.to_owned()));
    let witness = &pad.witness;
    if witness.submitter.is_zero() {
        return fail(".witness.submitter", "must not be the zero address");
    }
    if witness.decryptors.len() > usize::from(MAX_PAD_DECRYPTORS) {
        return fail(
            ".witness.decryptors",
            &format!("at most {MAX_PAD_DECRYPTORS} decryptors"),
        );
    }
    if let Some(j) = witness.decryptors.iter().position(|d| d.is_zero()) {
        return fail(
            &format!(".witness.decryptors[{j}]"),
            "must not be the zero address",
        );
    }
    if !(1..=MAX_PAD_LENGTH).contains(&pad.pad_length) {
        return fail(
            ".padLength",
            &format!("must be within 1..={MAX_PAD_LENGTH}"),
        );
    }
    if witness.expiration_timestamp <= now {
        return fail(".witness.expirationTimestamp", "the witness has expired");
    }
    if label(payload, witness) != pad.label {
        return fail(".label", "is not the hash of the witness");
    }
    Ok(())
}

/// One request, summarized in its own `Summary`.
pub async fn handle(app: State<App>, request: Request) -> Response {
    let mut guard = Guard::new(FlowName::PadDecrypt, app.metrics.clone());
    let result = process(&app, request, &mut guard.summary).await;
    guard.disarm();
    result.unwrap_or_else(IntoResponse::into_response)
}

/// Read and parse the body (bounded by `http.body_read_timeout`), validate, convert to the connector DTO,
/// aggregate, answer.
async fn process(app: &App, request: Request, summary: &mut Summary) -> Result<Response, ApiError> {
    let request: PadDecryptRequest = read_json(app, request, summary).await?;
    let payload = &request.payload;
    summary.received(
        &app.http,
        payload.pads.iter().map(|p| p.label).collect(),
        consumers(payload),
        payload.allowed_contracts.clone(),
        Some(payload.user_address),
    );
    summary.chain_id(payload.chain_id);
    validate(&request, &app.http, super::now()).map_err(|e| super::invalid(summary, e))?;
    let request = PadDecryptionRequest::from(request);
    summary.forwarded(request.id());
    let result = app
        .pad_decrypt
        .run(&summary.request_id, request, &mut summary.tally)
        .await;
    match result {
        Ok(output) => Ok(Reply::succeeded(summary.take(), output).into_response()),
        Err(e) => Err(ApiError::aggregation(summary, e)),
    }
}

/// The witnesses' consumers, once each, in order: the contracts of the pads. Linear: not validated yet.
fn consumers(payload: &PadDecryptPayload) -> Vec<Address> {
    let mut seen = HashSet::new();
    payload
        .pads
        .iter()
        .map(|p| p.witness.consumer)
        .filter(|address| seen.insert(*address))
        .collect()
}

/// The connector's witness, field by field.
impl From<PadWitness> for ConnectorWitness {
    fn from(witness: PadWitness) -> Self {
        Self {
            submitter: witness.submitter,
            consumer: witness.consumer,
            expirationTimestamp: witness.expiration_timestamp,
            decryptors: witness.decryptors,
            nonce: witness.nonce,
        }
    }
}

/// The body the connector receives (`kms_connector_api::PadDecryptionRequest`), field by field.
impl From<PadDecryptRequest> for PadDecryptionRequest {
    fn from(request: PadDecryptRequest) -> Self {
        let payload = request.payload;
        Self {
            attestationType: request.attestation_type,
            payload: PadDecryptionPayload {
                witnessVersion: payload.witness_version,
                prfId: payload.prf_id,
                chainId: payload.chain_id,
                pads: payload
                    .pads
                    .into_iter()
                    .map(|p| ConnectorPad {
                        label: p.label,
                        witness: p.witness.into(),
                        padLength: p.pad_length,
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
    use alloy::primitives::{address, b256};

    use super::*;
    use crate::endpoint::validate::MAX_DURATION_SECONDS;
    use crate::kms_aggregator::mock::{PAD_REQUEST_ID, PAD_REQUEST_JSON};

    pub(crate) const NOW: u64 = 1_800_000_000;

    /// The SDK's wire shape: one pad on chain 12345 expiring at `expiration`, the request valid from `start`.
    /// The label is the witness hash, filled in once the witness is parsed.
    pub(crate) fn wire(start: u64, expiration: u64) -> String {
        let placeholder = format!("0x{}", "00".repeat(32));
        let raw = format!(
            r#"{{
              "attestationType": "eip712-unified-user-decrypt-v1",
              "payload": {{
                "witnessVersion": 0,
                "prfId": 1,
                "chainId": "12345",
                "pads": [{{
                  "label": "{placeholder}",
                  "witness": {{
                    "submitter": "0x5555555555555555555555555555555555555555",
                    "consumer": "0x3333333333333333333333333333333333333333",
                    "expirationTimestamp": "{expiration}",
                    "decryptors": ["0x4444444444444444444444444444444444444444"],
                    "nonce": "0x77777777777777777777777777777777"
                  }},
                  "padLength": 64
                }}],
                "userAddress": "0x5555555555555555555555555555555555555555",
                "publicKey": "0x20002000",
                "allowedContracts": ["0x3333333333333333333333333333333333333333"],
                "requestValidity": {{ "startTimestamp": "{start}", "durationSeconds": "600" }},
                "extraData": "0x02{}"
              }},
              "signature": "0x{}"
            }}"#,
            "00".repeat(64),
            "66".repeat(65)
        );
        let request: PadDecryptRequest = serde_json::from_str(&raw).unwrap();
        let label = label(&request.payload, &request.payload.pads[0].witness);
        raw.replace(&placeholder, &label.to_string())
    }

    pub(crate) fn valid() -> PadDecryptRequest {
        serde_json::from_str(&wire(NOW - 100, NOW + 604_800)).unwrap()
    }

    fn http() -> HttpConfig {
        HttpConfig::for_tests(&[12345])
    }

    #[test]
    fn label_matches_an_independent_vector() {
        // `cast keccak` over the RFC-030 preimage of this witness, built by hand with xxd and printf.
        let mut request = valid();
        request.payload.pads[0].witness.expiration_timestamp = 1_800_604_800;
        assert_eq!(
            label(&request.payload, &request.payload.pads[0].witness),
            b256!("0xc1fde10378bc1fa1ff0c2590c35f82218689e38e289629ca4be4a8e2e4f36a4b")
        );
    }

    #[test]
    fn the_connector_vector_parses_and_converts_to_its_id() {
        let request: PadDecryptRequest = serde_json::from_str(PAD_REQUEST_JSON).unwrap();
        let payload = &request.payload;
        assert_eq!(
            label(payload, &payload.pads[0].witness),
            payload.pads[0].label
        );
        assert_eq!(PadDecryptionRequest::from(request).id(), PAD_REQUEST_ID);
    }

    #[test]
    fn wire_example_deserialises_with_string_or_number_integers() {
        let request = valid();
        assert_eq!(request.payload.chain_id, 12345);
        assert_eq!(
            request.payload.pads[0].witness.expiration_timestamp,
            NOW + 604_800
        );
        let numbers =
            wire(NOW - 100, NOW + 604_800).replace(r#""chainId": "12345""#, r#""chainId": 12345"#);
        let numbers: PadDecryptRequest = serde_json::from_str(&numbers).unwrap();
        assert_eq!(numbers.payload.chain_id, 12345);
    }

    #[test]
    fn deserialisation_rejects_unknown_fields_and_bad_widths() {
        let wire = wire(NOW - 100, NOW + 604_800);
        let unknown = wire.replace(r#""padLength": 64"#, r#""padLength": 64, "x": 1"#);
        assert!(serde_json::from_str::<PadDecryptRequest>(&unknown).is_err());
        let unknown_in_witness = wire.replace(r#""decryptors""#, r#""x": 1, "decryptors""#);
        assert!(serde_json::from_str::<PadDecryptRequest>(&unknown_in_witness).is_err());
        let short_nonce = wire.replace("0x77777777777777777777777777777777", "0x7777");
        assert!(serde_json::from_str::<PadDecryptRequest>(&short_nonce).is_err());
        let big_length = wire.replace(r#""padLength": 64"#, r#""padLength": 65536"#);
        assert!(serde_json::from_str::<PadDecryptRequest>(&big_length).is_err());
    }

    #[test]
    fn valid_request_passes() {
        assert_eq!(validate(&valid(), &http(), NOW), Ok(()));
    }

    #[test]
    fn each_rule_names_its_field() {
        type Mutation = Box<dyn Fn(&mut PadDecryptRequest)>;
        let zero = Address::ZERO;
        let cases: Vec<(Mutation, &str)> = vec![
            (
                Box::new(|r| r.attestation_type = "other".into()),
                "attestationType",
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
            (
                Box::new(|r| r.payload.extra_data = Bytes::from_static(&[0])),
                "payload.extraData",
            ),
            (
                Box::new(|r| r.payload.witness_version = 1),
                "payload.witnessVersion",
            ),
            (Box::new(|r| r.payload.chain_id = 1), "payload.chainId"),
            (Box::new(|r| r.payload.pads.clear()), "payload.pads"),
            (
                Box::new(|r| {
                    let p = r.payload.pads[0].clone();
                    r.payload.pads = vec![p; 11];
                }),
                "payload.pads",
            ),
            (
                Box::new(|r| {
                    let p = r.payload.pads[0].clone();
                    r.payload.pads.push(p);
                }),
                "payload.pads[1].label",
            ),
            (
                Box::new(move |r| r.payload.pads[0].witness.submitter = zero),
                "payload.pads[0].witness.submitter",
            ),
            (
                Box::new(move |r| r.payload.pads[0].witness.decryptors = vec![zero; 17]),
                "payload.pads[0].witness.decryptors",
            ),
            (
                Box::new(move |r| r.payload.pads[0].witness.decryptors.push(zero)),
                "payload.pads[0].witness.decryptors[1]",
            ),
            (
                Box::new(|r| r.payload.pads[0].pad_length = 0),
                "payload.pads[0].padLength",
            ),
            (
                Box::new(|r| r.payload.pads[0].pad_length = 2049),
                "payload.pads[0].padLength",
            ),
            (
                Box::new(|r| r.payload.pads[0].witness.expiration_timestamp = NOW),
                "payload.pads[0].witness.expirationTimestamp",
            ),
            (
                Box::new(|r| r.payload.pads[0].witness.nonce = B128::repeat_byte(0x78)),
                "payload.pads[0].label",
            ),
        ];
        for (mutate, field) in cases {
            let mut request = valid();
            mutate(&mut request);
            let error = validate(&request, &http(), NOW).unwrap_err();
            assert_eq!(error.field, field, "{error}");
        }
    }

    #[test]
    fn the_pad_limit_is_the_configured_one() {
        let mut http = http();
        http.max_pads = 1;
        let mut request = valid();
        let mut other = request.payload.pads[0].clone();
        other.witness.nonce = B128::repeat_byte(0x78);
        other.label = label(&request.payload, &other.witness);
        request.payload.pads.push(other);
        let error = validate(&request, &http, NOW).unwrap_err();
        assert_eq!(error.field, "payload.pads");
        assert_eq!(error.value.as_deref(), Some("2 pads"));
        http.max_pads = 2;
        assert_eq!(validate(&request, &http, NOW), Ok(()));
    }

    #[test]
    fn conversion_keeps_every_field() {
        let request = valid();
        let connector = PadDecryptionRequest::from(request.clone());
        let (p, c) = (&request.payload, &connector.payload);
        assert_eq!(connector.attestationType, request.attestation_type);
        assert_eq!(connector.signature, request.signature);
        assert_eq!(c.userAddress, p.user_address);
        assert_eq!(c.publicKey, p.public_key);
        assert_eq!(c.allowedContracts, p.allowed_contracts);
        assert_eq!(c.requestValidity.startTimestamp, NOW - 100);
        assert_eq!(c.requestValidity.durationSeconds, 600);
        assert_eq!(c.extraData, p.extra_data);
        assert_eq!(
            (c.witnessVersion, c.prfId, c.chainId),
            (p.witness_version, p.prf_id, p.chain_id)
        );
        let (pad, cpad) = (&p.pads[0], &c.pads[0]);
        assert_eq!((cpad.label, cpad.padLength), (pad.label, pad.pad_length));
        let (w, cw) = (&pad.witness, &cpad.witness);
        assert_eq!((cw.submitter, cw.consumer), (w.submitter, w.consumer));
        assert_eq!(cw.expirationTimestamp, w.expiration_timestamp);
        assert_eq!(cw.decryptors, w.decryptors);
        assert_eq!(cw.nonce, w.nonce);
        assert_eq!(c.label_of(cw), cpad.label);
    }
}
