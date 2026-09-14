//! `POST /v4/exp/user-decrypt`: the current relayer's v3 attestation envelope, validated, converted to the connector
//! DTO and aggregated.

use alloy::primitives::{Address, B256, Bytes};
use kms_connector_api::{HandleEntry as ConnectorHandle, RequestValidity as ConnectorValidity};
use serde::{Deserialize, Deserializer};

use crate::endpoint::validate::{self, Invalid};

pub const ATTESTATION_TYPE: &str = "eip712-unified-user-decrypt-v1";
pub const PAYLOAD_VERSION: &str = "2.0";
pub const PAYLOAD_TYPE: &str = "user_decryption";
/// The connector's cap.
const MAX_ALLOWED_CONTRACTS: usize = 10;

/// Request body. Hex fields are typed: length and format are checked by deserialisation.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UserDecryptRequest {
    /// `eip712-unified-user-decrypt-v1`.
    pub attestation_type: String,
    pub attested_payload: UserDecryptPayload,
    /// The user's EIP-712 signature; empty on the ERC-1271 path. Verified by the connector, not here.
    pub signature: Bytes,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UserDecryptPayload {
    /// `2.0`.
    pub version: String,
    /// `user_decryption`.
    #[serde(rename = "type")]
    pub kind: String,
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
    let payload = &request.attested_payload;
    validate::expect("attestedPayload.version", &payload.version, PAYLOAD_VERSION)?;
    validate::expect("attestedPayload.type", &payload.kind, PAYLOAD_TYPE)?;
    validate::handles(
        "attestedPayload.handles",
        payload.handles.iter().map(|h| &h.ct_handle),
        chain_ids,
    )?;
    if payload.allowed_contracts.len() > MAX_ALLOWED_CONTRACTS {
        return Err(Invalid::new(
            "attestedPayload.allowedContracts",
            format!("at most {MAX_ALLOWED_CONTRACTS} contracts"),
        ));
    }
    if payload.public_key.is_empty() {
        return Err(Invalid::new(
            "attestedPayload.publicKey",
            "must not be empty",
        ));
    }
    let validity = &payload.request_validity;
    if validity.start_timestamp > now {
        return Err(Invalid::new(
            "attestedPayload.requestValidity.startTimestamp",
            "must not be in the future",
        ));
    }
    if validity
        .start_timestamp
        .saturating_add(validity.duration_seconds)
        <= now
    {
        return Err(Invalid::new(
            "attestedPayload.requestValidity",
            "window has already expired",
        ));
    }
    validate::extra_data("attestedPayload.extraData", &payload.extra_data)
}

/// The body the connector receives (RFC-033), field by field.
impl From<UserDecryptRequest> for kms_connector_api::UserDecryptionRequest {
    fn from(request: UserDecryptRequest) -> Self {
        let payload = request.attested_payload;
        Self {
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
            signature: request.signature,
            extraData: payload.extra_data,
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use alloy::primitives::address;

    use super::*;
    use crate::endpoint::validate::tests::handle;

    pub(crate) const NOW: u64 = 1_800_000_000;

    /// The SDK's wire shape, one handle on chain 1.
    pub(crate) fn wire(start: &str, duration: &str) -> String {
        format!(
            r#"{{
              "attestationType": "eip712-unified-user-decrypt-v1",
              "attestedPayload": {{
                "version": "2.0",
                "type": "user_decryption",
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
    fn wire_example_deserialises_with_string_or_number_timestamps() {
        let text = valid();
        let numbers: UserDecryptRequest = serde_json::from_str(&wire("1799999900", "600")).unwrap();
        for r in [text, numbers] {
            assert_eq!(
                r.attested_payload.request_validity.start_timestamp,
                1_799_999_900
            );
            assert_eq!(r.attested_payload.request_validity.duration_seconds, 600);
            assert_eq!(r.attested_payload.handles[0].ct_handle, handle(1, 4));
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
            (
                Box::new(|r| r.attested_payload.version = "1.0".into()),
                "attestedPayload.version",
            ),
            (
                Box::new(|r| r.attested_payload.kind = "public".into()),
                "attestedPayload.type",
            ),
            (
                Box::new(|r| r.attested_payload.handles.clear()),
                "attestedPayload.handles",
            ),
            (
                Box::new(|r| r.attested_payload.handles[0].ct_handle = handle(31337, 4)),
                "attestedPayload.handles[0]",
            ),
            (
                Box::new(|r| {
                    r.attested_payload.allowed_contracts =
                        vec![address!("0x3333333333333333333333333333333333333333"); 11]
                }),
                "attestedPayload.allowedContracts",
            ),
            (
                Box::new(|r| r.attested_payload.public_key = Bytes::new()),
                "attestedPayload.publicKey",
            ),
            (
                Box::new(|r| r.attested_payload.request_validity.start_timestamp = NOW + 1),
                "attestedPayload.requestValidity.startTimestamp",
            ),
            (
                Box::new(|r| r.attested_payload.request_validity.duration_seconds = 1),
                "attestedPayload.requestValidity",
            ),
            (
                Box::new(|r| r.attested_payload.extra_data = Bytes::from_static(&[9])),
                "attestedPayload.extraData",
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
        let connector = kms_connector_api::UserDecryptionRequest::from(request.clone());
        assert_eq!(connector.handles.len(), 1);
        assert_eq!(connector.handles[0].handle, handle(1, 4));
        assert_eq!(
            connector.handles[0].contractAddress,
            address!("0x3333333333333333333333333333333333333333")
        );
        assert_eq!(
            connector.handles[0].ownerAddress,
            address!("0x4444444444444444444444444444444444444444")
        );
        assert_eq!(connector.userAddress, request.attested_payload.user_address);
        assert_eq!(connector.publicKey, request.attested_payload.public_key);
        assert_eq!(
            connector.allowedContracts,
            request.attested_payload.allowed_contracts
        );
        assert_eq!(connector.requestValidity.startTimestamp, 1_799_999_900);
        assert_eq!(connector.requestValidity.durationSeconds, 600);
        assert_eq!(connector.signature, request.signature);
        assert_eq!(connector.extraData, request.attested_payload.extra_data);
        // The connector derives the same id from this body on every node.
        assert_ne!(connector.id(), B256::ZERO);
    }
}
