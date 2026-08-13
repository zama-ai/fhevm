use crate::core::errors::EventProcessingError;
use crate::core::job_id::JobId;
use crate::core::solana_host_payload::{encode_host_payload, SolanaHandleWire};
use crate::http::endpoints::v2::types::DelegatedUserDecryptRequestJson;
use crate::http::endpoints::v2::types::{
    InputProofRequestJson, PublicDecryptRequestJson, UserDecryptRequestJson,
};
use crate::http::endpoints::v3::types::{
    AttestedUserDecryptRequestJson, Eip712UnifiedUserDecryptPayloadJson,
    SolanaAttestedUserDecryptRequestJson,
};
use crate::http::utils::validations::V3_ATTESTATION_TYPE_EIP712_UNIFIED_V1;

/// Solana handle-count cap (mirrors the gateway admission cap and the connector's
/// `MAX_REQUEST_HANDLES`): `3*N + 1 <= 100` snapshot accounts gives 33.
const SOLANA_MAX_REQUEST_HANDLES: usize = 33;
/// The largest sibling count a well-formed access proof can carry (mirrors the connector).
const SOLANA_MAX_ACCESS_PROOF_SIBLINGS: usize = 64;
use crate::orchestrator::traits::Event;
use alloy::primitives::{Address, Bytes, FixedBytes, TxHash};
use alloy::{primitives::U256, rpc::types::Log};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::hash::Hash;
use std::num::ParseIntError;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::info;

// TODO: add test to make sure that there is no id conflict
// TODO: verify there is no snake-case, camel-case around here

#[repr(u8)]
#[derive(Debug)]
/// Event Ids corresponding the events of GatewayChainEvent type.
pub enum GatewayChainEventId {
    EventLogRcvd = 50,
}

impl From<GatewayChainEventId> for u8 {
    fn from(e: GatewayChainEventId) -> u8 {
        e as u8
    }
}

#[repr(u8)]
#[derive(Debug)]
/// Event Ids corresponding the events of PublicDecryptEvent type.
pub enum PublicDecryptEventId {
    ReqRcvdFromUser = 10,
    ReadinessCheckPassed = 11,
    ReqSentToGw = 12,
    RespRcvdFromGw = 13,
    Failed = 14,
    RespSentToUser = 15,
    InternalFailure = 16,
    ReadinessCheckTimedOut = 17,
    ReadinessCheckFailed = 18,
}

impl From<PublicDecryptEventId> for u8 {
    fn from(e: PublicDecryptEventId) -> u8 {
        e as u8
    }
}

#[repr(u8)]
#[derive(Debug)]
/// Event Ids corresponding the events of UserDecryptEvent type.
pub enum UserDecryptEventId {
    ReqRcvdFromUser = 20,
    ReadinessCheckPassed = 21,
    ReqSentToGw = 22,
    RespRcvdFromGw = 23,
    RespSentToUser = 24,
    Failed = 25,
    InternalFailure = 26,
    ReadinessCheckTimedOut = 27,
    ReadinessCheckFailed = 28,
}

impl From<UserDecryptEventId> for u8 {
    fn from(e: UserDecryptEventId) -> u8 {
        e as u8
    }
}

#[repr(u8)]
#[derive(Debug)]
/// Event Ids corresponding the events of InputProofEvent type.
pub enum InputProofEventId {
    ReqRcvdFromUser = 30,
    ReqSentToGw = 31,
    RespRcvdFromGw = 32,
    Failed = 33,
    InternalFailure = 34,
}

impl From<InputProofEventId> for u8 {
    fn from(e: InputProofEventId) -> u8 {
        e as u8
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
/// Relayer event represents a single step in one of the different flows of the
/// relayer (such as public decryption, input proof verification and so on).
pub struct RelayerEvent {
    pub job_id: JobId,
    pub api_version: ApiVersion,
    pub data: RelayerEventData,
    pub timestamp: u64,
}

impl Display for RelayerEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}({}, {})",
            self.event_name(),
            self.job_id(),
            self.api_version
        )
    }
}

impl RelayerEvent {
    pub fn new(job_id: JobId, api_version: ApiVersion, data: RelayerEventData) -> RelayerEvent {
        let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_secs(),
            Err(_) => 0,
        };

        RelayerEvent {
            job_id,
            api_version,
            data,
            timestamp,
        }
    }

    pub fn derive_next_event(self, next_event_data: RelayerEventData) -> RelayerEvent {
        let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_secs(),
            Err(_) => 0,
        };

        RelayerEvent {
            job_id: self.job_id,
            api_version: self.api_version,
            data: next_event_data,
            timestamp,
        }
    }
}

impl Event for RelayerEvent {
    fn event_name(&self) -> &str {
        self.data.as_ref()
    }

    fn event_id(&self) -> u8 {
        match &self.data {
            RelayerEventData::GatewayChain(e) => e.event_id(),
            RelayerEventData::PublicDecrypt(e) => e.event_id(),
            RelayerEventData::UserDecrypt(e) => e.event_id(),
            RelayerEventData::InputProof(e) => e.event_id(),
        }
    }

    fn job_id(&self) -> JobId {
        self.job_id
    }

    fn timestamp(&self) -> u64 {
        self.timestamp
    }
}

#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub struct ApiVersion {
    pub category: ApiCategory,
    pub number: u8,
}

impl Display for ApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.category {
            ApiCategory::PRODUCTION => write!(f, "v{}", self.number),
            ApiCategory::EXPERIMENTAL => write!(f, "exp/v{}", self.number),
        }
    }
}

/// Api version allows for differentiating between different versions of the
/// same API. The different versions can have entirely different flows or share
/// part of the flow.
impl ApiVersion {
    pub fn new(category: ApiCategory, number: u8) -> Self {
        ApiVersion { category, number }
    }
}

/// Api category allows for differentiating between production and experimental
/// APIs.
#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub enum ApiCategory {
    PRODUCTION,
    EXPERIMENTAL,
}

/// Relayer event data represents the different categories of event data, each
/// representing a specific flow.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RelayerEventData {
    GatewayChain(GatewayChainEventData),
    PublicDecrypt(PublicDecryptEventData),
    UserDecrypt(UserDecryptEventData),
    InputProof(InputProofEventData),
}

impl AsRef<str> for RelayerEventData {
    fn as_ref(&self) -> &str {
        match self {
            RelayerEventData::GatewayChain(gateway_event) => gateway_event.event_name(),
            RelayerEventData::PublicDecrypt(decrypt_event) => decrypt_event.event_name(),
            RelayerEventData::UserDecrypt(decrypt_event) => decrypt_event.event_name(),
            RelayerEventData::InputProof(input_event) => input_event.event_name(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GatewayChainEventData {
    /// Event representing a raw blockchain event log received from gateway chain.
    EventLogRcvd { log: Log, tx_hash: TxHash },
}

impl GatewayChainEventData {
    pub fn event_name(&self) -> &'static str {
        match self {
            GatewayChainEventData::EventLogRcvd { .. } => "GatewayChain::EventLogRcvd",
        }
    }

    pub fn event_id(&self) -> u8 {
        match self {
            GatewayChainEventData::EventLogRcvd { .. } => GatewayChainEventId::EventLogRcvd.into(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PublicDecryptEventData {
    /// Event representing a public decryption request for ciphertexts from user.
    ReqRcvdFromUser {
        decrypt_request: PublicDecryptRequest,
    },

    /// Event representing that readiness check has passed for a public decryption request.
    ReadinessCheckPassed {
        decrypt_request: PublicDecryptRequest,
    },

    /// Event representing that readiness check has timed out for a public decryption request.
    ReadinessCheckTimedOut {
        decrypt_request: PublicDecryptRequest,
        error: EventProcessingError,
    },

    /// Event representing that readiness check has failed for a public decryption request.
    ReadinessCheckFailed {
        decrypt_request: PublicDecryptRequest,
        error: EventProcessingError,
    },

    /// Event representing the result of sending a public decryption request to
    /// gateway. Id will be used to map the response that will be received later
    /// to the request.
    ReqSentToGw { gw_req_reference_id: U256 },

    /// Event representing the success response received from gateway for public
    /// decryption request sent from this instance of relayer.
    RespRcvdFromGw {
        decrypt_response: PublicDecryptResponse,
    },

    /// Event representing the user decryption response sent to the user.
    RespSentToUser,

    /// Event representing the failure in processing the public decryption request.
    /// Used to notify outside internal handlers only.
    Failed { error: EventProcessingError },

    /// Event representing the internal failure in processing the public decryption request: will not notify the user directly.
    InternalFailure { error: EventProcessingError },
}

impl PublicDecryptEventData {
    pub fn event_name(&self) -> &'static str {
        match self {
            PublicDecryptEventData::ReqRcvdFromUser { .. } => "PublicDecrypt::ReqRcvdFromUser",
            PublicDecryptEventData::ReadinessCheckPassed { .. } => {
                "PublicDecrypt::ReadinessCheckPassed"
            }
            PublicDecryptEventData::ReadinessCheckTimedOut { .. } => {
                "PublicDecrypt::ReadinessCheckTimedOut"
            }
            PublicDecryptEventData::ReadinessCheckFailed { .. } => {
                "PublicDecrypt::ReadinessCheckFailed"
            }
            PublicDecryptEventData::ReqSentToGw { .. } => "PublicDecrypt::ReqSentToGw",
            PublicDecryptEventData::RespRcvdFromGw { .. } => "PublicDecrypt::RespRcvdFromGw",
            PublicDecryptEventData::RespSentToUser => "PublicDecrypt::RespSentToUser",
            PublicDecryptEventData::Failed { .. } => "PublicDecrypt::Failed",
            PublicDecryptEventData::InternalFailure { .. } => "PublicDecrypt::InternalFailure",
        }
    }

    pub fn event_id(&self) -> u8 {
        match self {
            PublicDecryptEventData::ReqRcvdFromUser { .. } => {
                PublicDecryptEventId::ReqRcvdFromUser.into()
            }
            PublicDecryptEventData::ReadinessCheckPassed { .. } => {
                PublicDecryptEventId::ReadinessCheckPassed.into()
            }
            PublicDecryptEventData::ReadinessCheckTimedOut { .. } => {
                PublicDecryptEventId::ReadinessCheckTimedOut.into()
            }
            PublicDecryptEventData::ReadinessCheckFailed { .. } => {
                PublicDecryptEventId::ReadinessCheckFailed.into()
            }
            PublicDecryptEventData::ReqSentToGw { .. } => PublicDecryptEventId::ReqSentToGw.into(),
            PublicDecryptEventData::RespRcvdFromGw { .. } => {
                PublicDecryptEventId::RespRcvdFromGw.into()
            }
            PublicDecryptEventData::RespSentToUser => PublicDecryptEventId::RespSentToUser.into(),
            PublicDecryptEventData::Failed { .. } => PublicDecryptEventId::Failed.into(),
            PublicDecryptEventData::InternalFailure { .. } => {
                PublicDecryptEventId::InternalFailure.into()
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UserDecryptEventData {
    /// Event representing a user decryption request for ciphertexts on fhevm.
    ReqRcvdFromUser { decrypt_request: UserDecryptRequest },

    /// Event representing that readiness check has passed for a user decryption request.
    ReadinessCheckPassed { decrypt_request: UserDecryptRequest },

    /// Event representing that readiness check has timed out for a public decryption request.
    ReadinessCheckTimedOut {
        decrypt_request: UserDecryptRequest,
        error: EventProcessingError,
    },

    /// Event representing that readiness check has failed for a user decryption request.
    ReadinessCheckFailed {
        decrypt_request: UserDecryptRequest,
        error: EventProcessingError,
    },

    /// Event representing the result of sending a user decryption request to
    /// gateway. Id will be used to map the response that will be received later
    /// to the request.
    ReqSentToGw { gw_req_reference_id: U256 },

    /// Event representing the success response received from gateway for user
    /// decryption sent from this instance of relayer.
    RespRcvdFromGw {
        decrypt_response: UserDecryptResponse,
    },

    /// Event representing the user decryption response sent to the user.
    RespSentToUser,

    /// Event representing the failure in processing the user decryption request.
    /// Used to notify outside internal handlers only.
    Failed { error: EventProcessingError },

    /// Event representing the internal failure in processing the user decrypt request: will not notify the user directly.
    InternalFailure { error: EventProcessingError },
}

impl UserDecryptEventData {
    pub fn event_name(&self) -> &'static str {
        match self {
            UserDecryptEventData::ReqRcvdFromUser { .. } => "UserDecrypt::ReqRcvdFromUser",
            UserDecryptEventData::ReadinessCheckPassed { .. } => {
                "UserDecrypt::ReadinessCheckPassed"
            }
            UserDecryptEventData::ReadinessCheckTimedOut { .. } => {
                "UserDecrypt::ReadinessCheckTimedOut"
            }
            UserDecryptEventData::ReadinessCheckFailed { .. } => {
                "UserDecrypt::ReadinessCheckFailed"
            }
            UserDecryptEventData::ReqSentToGw { .. } => "UserDecrypt::ReqSentToGw",
            UserDecryptEventData::RespRcvdFromGw { .. } => "UserDecrypt::RespRcvdFromGw",
            UserDecryptEventData::RespSentToUser => "UserDecrypt::RespSentToFhevm",
            UserDecryptEventData::Failed { .. } => "UserDecrypt::Failed",
            UserDecryptEventData::InternalFailure { .. } => "UserDecrypt::InternalFailure",
        }
    }

    pub fn event_id(&self) -> u8 {
        match self {
            UserDecryptEventData::ReqRcvdFromUser { .. } => {
                UserDecryptEventId::ReqRcvdFromUser.into()
            }
            UserDecryptEventData::ReadinessCheckPassed { .. } => {
                UserDecryptEventId::ReadinessCheckPassed.into()
            }
            UserDecryptEventData::ReadinessCheckTimedOut { .. } => {
                UserDecryptEventId::ReadinessCheckTimedOut.into()
            }
            UserDecryptEventData::ReadinessCheckFailed { .. } => {
                UserDecryptEventId::ReadinessCheckFailed.into()
            }
            UserDecryptEventData::ReqSentToGw { .. } => UserDecryptEventId::ReqSentToGw.into(),
            UserDecryptEventData::RespRcvdFromGw { .. } => {
                UserDecryptEventId::RespRcvdFromGw.into()
            }
            UserDecryptEventData::RespSentToUser => UserDecryptEventId::RespSentToUser.into(),
            UserDecryptEventData::Failed { .. } => UserDecryptEventId::Failed.into(),
            UserDecryptEventData::InternalFailure { .. } => {
                UserDecryptEventId::InternalFailure.into()
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct PublicDecryptRequest {
    #[serde(
        serialize_with = "crate::http::serialize_ct_handles_as_hex",
        deserialize_with = "crate::http::deserialize_ct_handles_from_hex"
    )]
    pub ct_handles: Vec<[u8; 32]>,
    pub extra_data: Bytes,
}

/// A user-decryption request. Each variant owns the complete set of
/// fields its attestation format expects on the wire and on the
/// gateway — including the `signature`, `public_key`, and `extra_data`
/// fields that all current formats happen to share. Pattern-matching
/// on the request hands the caller every field for that format in one
/// place, with no cross-format envelope.
///
/// `LegacyDirect` and `LegacyDelegated` should be removed once the
/// legacy EIP-712 formats (direct + delegated) are deprecated; at that
/// point only `Eip712UnifiedV1` remains and this enum collapses into a
/// struct.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Hash)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum UserDecryptRequest {
    /// Legacy EIP-712 direct user-decryption: maps to
    /// `userDecryptionRequest(CtHandleContractPair[], RequestValidity,
    /// ContractsInfo, address userAddress, …)` on the gateway.
    /// Should be removed once the legacy EIP-712 formats are deprecated.
    LegacyDirect {
        ct_handle_contract_pairs: Vec<HandleContractPair>,
        request_validity: RequestValidity,
        contracts_chain_id: u64,
        contract_addresses: Vec<Address>,
        user_address: Address,
        signature: Bytes,
        public_key: Bytes,
        extra_data: Bytes,
    },
    /// Legacy EIP-712 delegated user-decryption: maps to
    /// `delegatedUserDecryptionRequest(CtHandleContractPair[],
    /// RequestValidity, DelegationAccounts, ContractsInfo, …)`. Should
    /// be removed once the legacy EIP-712 formats are deprecated.
    LegacyDelegated {
        ct_handle_contract_pairs: Vec<HandleContractPair>,
        request_validity: RequestValidity,
        contracts_chain_id: u64,
        contract_addresses: Vec<Address>,
        delegator_address: Address,
        delegate_address: Address,
        signature: Bytes,
        public_key: Bytes,
        extra_data: Bytes,
    },
    /// Unified EIP-712 user-decryption (attestation_type
    /// `"eip712-unified-user-decrypt-v1"`): maps to
    /// `userDecryptionRequest(HandleEntry[], address userAddress,
    /// bytes publicKey, address[] allowedContracts,
    /// RequestValiditySeconds, …)`. `allowed_contracts` may be empty
    /// (permissive mode). Per-handle owner addresses live inside each
    /// `HandleEntry`.
    Eip712UnifiedV1 {
        handles: Vec<HandleEntry>,
        user_address: Address,
        allowed_contracts: Vec<Address>,
        request_validity: RequestValiditySeconds,
        signature: Bytes,
        public_key: Bytes,
        extra_data: Bytes,
    },
    /// Host-generic Solana user-decryption (attestation_type
    /// `"solana-srfc38-user-decrypt-v1"`): maps to the host-generic gateway
    /// `userDecryptionRequest(bytes32[] ctHandles, RequestValiditySeconds, bytes publicKey,
    /// uint8 allowedAclDomainKeyCount, uint8 hostKind, bytes extraData, bytes hostPayload)`
    /// overload with `hostKind = Solana`.
    /// Everything Solana-specific — the permit fields, the ed25519 signature, and the
    /// per-handle access evidence — is serialized into the opaque `host_payload` by the
    /// builder (canonical `0x01 ‖ borsh(body)`); the gateway never reads it, and each KMS
    /// party's connector decodes it and verifies the signature off-chain. The fields below are
    /// exactly what the gateway calldata consumes.
    SolanaSrfc38V1 {
        /// The ciphertext handles, in request order (the gateway's typed `bytes32[]`).
        ct_handles: Vec<U256>,
        /// The permit validity window, gateway-checked at admission.
        request_validity: RequestValiditySeconds,
        /// The transport (re-encryption) public key the gateway's response path validates against.
        public_key: Bytes,
        /// The declared length of the permit's signed ACL-scope list. The gateway bounds it
        /// before the fee (the EVM paths' `allowedContracts` rule, kept without reading the
        /// opaque payload); the connector admits the request only when it equals the signed
        /// list's actual length — this relayer always declares honestly.
        allowed_acl_domain_key_count: u8,
        /// The signed KMS routing bytes (version `0x02` ‖ contextId ‖ epochId).
        extra_data: Bytes,
        /// The canonical opaque host payload (`0x01 ‖ borsh(body)`).
        host_payload: Bytes,
    },
}

impl UserDecryptRequest {
    /// Short label for logs / metrics. Matches the serde tag values.
    pub fn attestation_kind(&self) -> &'static str {
        match self {
            UserDecryptRequest::LegacyDirect { .. } => "legacy_direct",
            UserDecryptRequest::LegacyDelegated { .. } => "legacy_delegated",
            UserDecryptRequest::Eip712UnifiedV1 { .. } => "eip712_unified_v1",
            UserDecryptRequest::SolanaSrfc38V1 { .. } => "solana_srfc38_v1",
        }
    }

    /// Whether this request uses one of the unified gateway overloads (EVM or Solana).
    pub fn is_unified(&self) -> bool {
        matches!(
            self,
            UserDecryptRequest::Eip712UnifiedV1 { .. } | UserDecryptRequest::SolanaSrfc38V1 { .. }
        )
    }

    /// References to the ciphertext handles, regardless of variant shape.
    pub fn ct_handles(&self) -> Vec<&U256> {
        match self {
            UserDecryptRequest::LegacyDirect {
                ct_handle_contract_pairs,
                ..
            }
            | UserDecryptRequest::LegacyDelegated {
                ct_handle_contract_pairs,
                ..
            } => ct_handle_contract_pairs
                .iter()
                .map(|p| &p.ct_handle)
                .collect(),
            UserDecryptRequest::Eip712UnifiedV1 { handles, .. } => {
                handles.iter().map(|h| &h.ct_handle).collect()
            }
            UserDecryptRequest::SolanaSrfc38V1 { ct_handles, .. } => ct_handles.iter().collect(),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Hash)]
pub struct HandleContractPair {
    #[serde(rename = "handle")]
    pub ct_handle: U256,
    #[serde(rename = "contractAddress")]
    pub contract_address: Address,
}

/// Per-handle entry for the unified EIP-712 format: carries the originating
/// contract plus the owner address used by the on-chain ACL check for
/// each handle. Sibling to `HandleContractPair` (v2 shape).
#[allow(non_snake_case)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Hash)]
pub struct HandleEntry {
    #[serde(rename = "ctHandle")]
    pub ct_handle: U256,
    #[serde(rename = "contractAddress")]
    pub contract_address: Address,
    #[serde(rename = "ownerAddress")]
    pub owner_address: Address,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Hash)]
#[allow(non_snake_case)]
pub struct RequestValidity {
    #[serde(rename = "startTimestamp")]
    pub start_timestamp: U256,
    #[serde(rename = "durationDays")]
    pub duration_days: U256,
}

/// Request-validity window in seconds (unified EIP-712 shape). Sibling to
/// `RequestValidity` (v2 days-based shape).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Hash)]
#[allow(non_snake_case)]
pub struct RequestValiditySeconds {
    #[serde(rename = "startTimestamp")]
    pub start_timestamp: U256,
    #[serde(rename = "durationSeconds")]
    pub duration_seconds: U256,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PublicDecryptResponse {
    pub gateway_request_id: U256,
    pub decrypted_value: Bytes,
    pub signatures: Vec<Bytes>,
    pub extra_data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserDecryptResponse {
    pub gateway_request_id: U256,
    pub reencrypted_shares: Vec<Bytes>,
    pub signatures: Vec<Bytes>,
    pub extra_data: String,
}

impl Display for UserDecryptResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UserDecryptResponse({})", self.gateway_request_id)
    }
}

impl TryFrom<UserDecryptRequestJson> for UserDecryptRequest {
    type Error = anyhow::Error;

    fn try_from(value: UserDecryptRequestJson) -> Result<Self, Self::Error> {
        info!("Converting UserDecryptRequestJson to UserDecryptRequest");

        let contracts_chain_id = parse_chain_id(&value.contracts_chain_id)?;
        // The DEPRECATED v2 legacy path is EVM-only. No Solana client ever shipped against it
        // (the Solana SDK action has only ever called `/v3/user-decrypt`), so reject outright
        // instead of fabricating zeroed EVM address placeholders for Solana requests.
        if is_solana_host_chain_id(contracts_chain_id) {
            anyhow::bail!(
                "Solana user decrypts are served by the v3 endpoint only; the legacy v2 path is EVM-only"
            );
        }

        let mut ct_handle_contract_pairs = Vec::new();
        for json_data in &value.handle_contract_pairs {
            let ct_handle = if json_data.handle.starts_with("0x") {
                // Remove the 0x prefix before parsing
                U256::from_str_radix(&json_data.handle[2..], 16)
            } else {
                U256::from_str_radix(&json_data.handle, 16)
            }
            .map_err(|e| anyhow::anyhow!("Failed to parse ctHandle: {}", e))?;

            let contract_address = Address::from_str(&json_data.contract_address)
                .map_err(|e| anyhow::anyhow!("Failed to parse contractAddress: {}", e))?;

            ct_handle_contract_pairs.push(HandleContractPair {
                ct_handle,
                contract_address,
            });
        }

        // Parse duration days - first try as number, then as string
        let duration_days = match value.request_validity.duration_days.parse::<u64>() {
            Ok(num) => U256::from(num),
            Err(_) => {
                // Try parsing as hex if it starts with 0x
                if value.request_validity.duration_days.starts_with("0x") {
                    U256::from_str(&value.request_validity.duration_days)?
                } else {
                    // Otherwise try as decimal string
                    U256::from_str_radix(&value.request_validity.duration_days, 10)?
                }
            }
        };

        let request_validity = RequestValidity {
            start_timestamp: U256::from_str(&value.request_validity.start_timestamp)?,
            duration_days,
        };

        let contract_addresses = value
            .contract_addresses
            .iter()
            .map(|addr| Address::from_str(addr))
            .collect::<Result<Vec<_>, _>>()?;

        // Parse extraData (validated at HTTP layer)
        let extra_data = Bytes::from_str(&value.extra_data)?;

        Ok(UserDecryptRequest::LegacyDirect {
            ct_handle_contract_pairs,
            request_validity,
            contracts_chain_id,
            contract_addresses,
            user_address: Address::from_str(&value.user_address)?,
            signature: Bytes::from_str(&value.signature)?,
            public_key: Bytes::from_str(&value.public_key)?,
            extra_data,
        })
    }
}

impl TryFrom<DelegatedUserDecryptRequestJson> for UserDecryptRequest {
    type Error = anyhow::Error;

    fn try_from(value: DelegatedUserDecryptRequestJson) -> Result<Self, Self::Error> {
        info!("Converting DelegatedUserDecryptRequestJson to UserDecryptRequest (LegacyDelegated)");

        let mut ct_handle_contract_pairs = Vec::new();
        for json_data in &value.handle_contract_pairs {
            let ct_handle = if json_data.handle.starts_with("0x") {
                // Remove the 0x prefix before parsing
                U256::from_str_radix(&json_data.handle[2..], 16)
            } else {
                U256::from_str_radix(&json_data.handle, 16)
            }
            .map_err(|e| anyhow::anyhow!("Failed to parse ctHandle: {}", e))?;

            let contract_address = Address::from_str(&json_data.contract_address)
                .map_err(|e| anyhow::anyhow!("Failed to parse contractAddress: {}", e))?;

            ct_handle_contract_pairs.push(HandleContractPair {
                ct_handle,
                contract_address,
            });
        }

        // Parse duration days - first try as number, then as string
        let duration_days = match value.duration_days.parse::<u64>() {
            Ok(num) => U256::from(num),
            Err(_) => {
                // Try parsing as hex if it starts with 0x
                if value.duration_days.starts_with("0x") {
                    U256::from_str(&value.duration_days)?
                } else {
                    // Otherwise try as decimal string
                    U256::from_str_radix(&value.duration_days, 10)?
                }
            }
        };

        // Parse contract chain ID
        let contracts_chain_id = parse_chain_id(&value.contracts_chain_id)?;

        let contract_addresses = &value
            .contract_addresses
            .iter()
            .map(|addr| Address::from_str(addr))
            .collect::<Result<Vec<_>, _>>()?;

        // Parse extraData (validated at HTTP layer)
        let extra_data = Bytes::from_str(&value.extra_data)?;

        Ok(UserDecryptRequest::LegacyDelegated {
            ct_handle_contract_pairs,
            request_validity: RequestValidity {
                start_timestamp: U256::from_str(&value.start_timestamp)?,
                duration_days,
            },
            contracts_chain_id,
            contract_addresses: contract_addresses.clone(),
            delegator_address: Address::from_str(&value.delegator_address)?,
            delegate_address: Address::from_str(&value.delegate_address)?,
            signature: Bytes::from_str(&value.signature)?,
            public_key: Bytes::from_str(&value.public_key)?,
            extra_data,
        })
    }
}

impl TryFrom<AttestedUserDecryptRequestJson> for UserDecryptRequest {
    type Error = anyhow::Error;

    fn try_from(value: AttestedUserDecryptRequestJson) -> Result<Self, Self::Error> {
        info!(
            attestation_type = %value.attestation_type,
            "Converting AttestedUserDecryptRequestJson to UserDecryptRequest"
        );

        // This envelope is the EVM EIP-712 unified path. `signature`, `publicKey` and `extraData`
        // are forwarded verbatim (opaque to the relayer); the relayer never verifies the
        // signature — the gateway does, on-chain. Solana requests arrive in their own envelope
        // (`SolanaAttestedUserDecryptRequestJson`) and are dispatched at the HTTP handler.
        let payload_inner = value.attested_payload;

        // The EVM unified path: each handle carries its EVM contract/owner addresses, which feed
        // the gateway's per-handle ACL on the decryption call. Solana requests ride their own
        // envelope and never reach this prelude.
        let mut handles = Vec::with_capacity(payload_inner.handles.len());
        for entry in &payload_inner.handles {
            let ct_handle = if let Some(rest) = entry.ct_handle.strip_prefix("0x") {
                U256::from_str_radix(rest, 16)
            } else {
                U256::from_str_radix(&entry.ct_handle, 16)
            }
            .map_err(|e| anyhow::anyhow!("Failed to parse ctHandle: {}", e))?;

            let contract_address = Address::from_str(&entry.contract_address)
                .map_err(|e| anyhow::anyhow!("Failed to parse contractAddress: {}", e))?;
            let owner_address = Address::from_str(&entry.owner_address)
                .map_err(|e| anyhow::anyhow!("Failed to parse ownerAddress: {}", e))?;

            handles.push(HandleEntry {
                ct_handle,
                contract_address,
                owner_address,
            });
        }

        let request_validity = RequestValiditySeconds {
            start_timestamp: U256::from_str(&payload_inner.request_validity.start_timestamp)?,
            duration_seconds: U256::from_str(&payload_inner.request_validity.duration_seconds)?,
        };
        let signature = Bytes::from_str(&value.signature)?;
        let public_key = Bytes::from_str(&payload_inner.public_key)?;
        let extra_data = Bytes::from_str(&payload_inner.extra_data)?;

        // Exhaustive per-protocol dispatch: an unknown attestation type is rejected HERE, not
        // only by the HTTP-layer validator — the conversion must never default a request it
        // doesn't recognize onto the EVM arm (DD-027: validation is per-protocol).
        match value.attestation_type.as_str() {
            V3_ATTESTATION_TYPE_EIP712_UNIFIED_V1 => eip712_unified_v1(
                payload_inner,
                handles,
                request_validity,
                signature,
                public_key,
                extra_data,
            ),
            other => Err(anyhow::anyhow!(
                "AttestedUserDecryptRequestJson carries a non-EVM attestationType {other:?}; \
                 expected {V3_ATTESTATION_TYPE_EIP712_UNIFIED_V1:?} (Solana requests use their own envelope)"
            )),
        }
    }
}

impl TryFrom<SolanaAttestedUserDecryptRequestJson> for UserDecryptRequest {
    type Error = anyhow::Error;

    fn try_from(value: SolanaAttestedUserDecryptRequestJson) -> Result<Self, Self::Error> {
        use borsh::BorshDeserialize;
        use zama_solana_permit::{PermitFields, PermitWireFields};

        info!(
            attestation_type = %value.attestation_type,
            "Converting SolanaAttestedUserDecryptRequestJson to UserDecryptRequest"
        );

        let payload = value.attested_payload;

        // The eight signed permit fields in transport form. Running the connector's own strict
        // decode here IS the r1–r2 pre-check: a request the relayer accepts is one the connector
        // will not reject on form (same crate, so never softer or harder). The relayer never
        // verifies the signature — each KMS party's connector does.
        let permit = PermitWireFields {
            user_pubkey: parse_0x_hex(&payload.user_pubkey, "userPubkey")?,
            transport_key: parse_0x_hex(&payload.transport_key, "transportKey")?,
            allowed_acl_domain_keys: payload
                .allowed_acl_domain_keys
                .iter()
                .map(|k| parse_0x_hex(k, "allowedAclDomainKeys"))
                .collect::<Result<Vec<_>, _>>()?,
            start_timestamp: parse_decimal_u64(
                &payload.request_validity.start_timestamp,
                "startTimestamp",
            )?,
            duration_seconds: parse_decimal_u64(
                &payload.request_validity.duration_seconds,
                "durationSeconds",
            )?,
            verifying_program_id: parse_0x_hex(
                &payload.verifying_program_id,
                "verifyingProgramId",
            )?,
            chain_id: parse_decimal_u64(&payload.chain_id, "chainId")?,
            extra_data: parse_0x_hex(&payload.extra_data, "extraData")?,
        };
        PermitFields::decode(&permit)
            .map_err(|e| anyhow::anyhow!("Solana permit failed the typed pre-check: {e}"))?;

        let signature = parse_0x_hex(&value.signature, "signature")?;
        if signature.len() != 64 {
            anyhow::bail!(
                "Solana ed25519 signature is {} bytes, expected 64",
                signature.len()
            );
        }

        if payload.handles.is_empty() {
            anyhow::bail!("Solana user-decryption request names no handles");
        }
        if payload.handles.len() > SOLANA_MAX_REQUEST_HANDLES {
            anyhow::bail!(
                "Solana user-decryption request names {} handles, exceeding the cap of {}",
                payload.handles.len(),
                SOLANA_MAX_REQUEST_HANDLES
            );
        }

        let mut ct_handles = Vec::with_capacity(payload.handles.len());
        let mut handle_wires = Vec::with_capacity(payload.handles.len());
        for (index, entry) in payload.handles.iter().enumerate() {
            let handle = parse_0x_hex_32(&entry.handle, "handle", index)?;
            let owner = parse_0x_hex_32(&entry.owner, "owner", index)?;
            let encrypted_value_id =
                parse_0x_hex_32(&entry.encrypted_value_id, "encryptedValueId", index)?;
            let proof_leaf_count = parse_decimal_u64(&entry.proof_leaf_count, "proofLeafCount")?;
            let access_proof = parse_0x_hex(&entry.access_proof, "accessProof")?;

            // accessProof form check, identical to the connector's decode so the two agree on
            // which proofs are well-formed: empty = current mode; else a strict borsh MMR proof,
            // no trailing bytes, at most 64 siblings. The bytes are forwarded verbatim.
            if !access_proof.is_empty() {
                let mut remaining = access_proof.as_slice();
                let proof =
                    zama_solana_acl::MmrProof::deserialize(&mut remaining).map_err(|_| {
                        anyhow::anyhow!("entry {index} accessProof does not decode as an MMR proof")
                    })?;
                if !remaining.is_empty() {
                    anyhow::bail!(
                        "entry {index} accessProof carries {} trailing byte(s)",
                        remaining.len()
                    );
                }
                if proof.siblings.len() > SOLANA_MAX_ACCESS_PROOF_SIBLINGS {
                    anyhow::bail!(
                        "entry {index} accessProof carries {} siblings, exceeding the cap of {}",
                        proof.siblings.len(),
                        SOLANA_MAX_ACCESS_PROOF_SIBLINGS
                    );
                }
            }

            ct_handles.push(U256::from_be_bytes::<32>(handle));
            handle_wires.push(SolanaHandleWire {
                handle: handle.to_vec(),
                owner: owner.to_vec(),
                encrypted_value_id: encrypted_value_id.to_vec(),
                proof_leaf_count,
                access_proof,
            });
        }

        let host_payload = encode_host_payload(&permit, &signature, &handle_wires)?;
        let request_validity = RequestValiditySeconds {
            start_timestamp: U256::from(permit.start_timestamp),
            duration_seconds: U256::from(permit.duration_seconds),
        };
        let public_key = Bytes::from(permit.transport_key.clone());
        let extra_data = Bytes::from(permit.extra_data.clone());
        // Honest by construction: the typed pre-check above already capped the list, so the
        // declared length is the signed list's actual length.
        let allowed_acl_domain_key_count = u8::try_from(permit.allowed_acl_domain_keys.len())
            .map_err(|_| {
                anyhow::anyhow!(
                    "the permit's ACL-scope list length {} does not fit in a u8",
                    permit.allowed_acl_domain_keys.len()
                )
            })?;

        Ok(UserDecryptRequest::SolanaSrfc38V1 {
            ct_handles,
            request_validity,
            public_key,
            allowed_acl_domain_key_count,
            extra_data,
            host_payload: Bytes::from(host_payload),
        })
    }
}

/// Parses a `0x`-hex string (the `0x` prefix optional) into bytes, naming the field on failure.
fn parse_0x_hex(value: &str, field: &str) -> Result<Vec<u8>, anyhow::Error> {
    let stripped = value.strip_prefix("0x").unwrap_or(value);
    hex::decode(stripped).map_err(|e| anyhow::anyhow!("Failed to parse {field} as 0x-hex: {e}"))
}

/// Parses a `0x`-hex string that must decode to exactly 32 bytes.
fn parse_0x_hex_32(value: &str, field: &str, index: usize) -> Result<[u8; 32], anyhow::Error> {
    let bytes = parse_0x_hex(value, field)?;
    let len = bytes.len();
    <[u8; 32]>::try_from(bytes.as_slice())
        .map_err(|_| anyhow::anyhow!("entry {index} field {field} is {len} bytes, expected 32"))
}

/// Parses a decimal string into a `u64`, naming the field on failure.
fn parse_decimal_u64(value: &str, field: &str) -> Result<u64, anyhow::Error> {
    value
        .parse::<u64>()
        .map_err(|e| anyhow::anyhow!("Failed to parse {field} as a u64: {e}"))
}

/// Builds the EVM EIP-712 variant of the unified request. Any `solana*` payload fields are
/// tolerated and ignored (wire compatibility: they have always been accepted alongside an EVM
/// attestation type).
fn eip712_unified_v1(
    payload: Eip712UnifiedUserDecryptPayloadJson,
    handles: Vec<HandleEntry>,
    request_validity: RequestValiditySeconds,
    signature: Bytes,
    public_key: Bytes,
    extra_data: Bytes,
) -> Result<UserDecryptRequest, anyhow::Error> {
    let allowed_contracts = payload
        .allowed_contracts
        .iter()
        .map(|addr| Address::from_str(addr))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(UserDecryptRequest::Eip712UnifiedV1 {
        handles,
        user_address: Address::from_str(&payload.user_address)?,
        allowed_contracts,
        request_validity,
        signature,
        public_key,
        extra_data,
    })
}

impl TryFrom<PublicDecryptRequestJson> for PublicDecryptRequest {
    type Error = anyhow::Error;

    fn try_from(value: PublicDecryptRequestJson) -> Result<Self, Self::Error> {
        info!("Converting PublicDecryptRequestJson to PublicDecryptRequest");

        let mut ct_handles = Vec::new();
        for ct_handle_hex in &value.ciphertext_handles {
            let ct_handle = if let Some(ct_handle_hex_wo_prefix) = ct_handle_hex.strip_prefix("0x")
            {
                U256::from_str_radix(ct_handle_hex_wo_prefix, 16)
            } else {
                U256::from_str_radix(ct_handle_hex, 16)
            }
            .map_err(|e| anyhow::anyhow!("Failed to parse ct_handle: {}", e))?;

            // TODO (Mano): The conversion to be bytes should happen in low level
            // code. App code should deal with with higher level types like U256.
            ct_handles.push(ct_handle.to_be_bytes());
        }

        // Parse extraData (validated at HTTP layer). It is propagated verbatim to the Gateway.
        let extra_data = Bytes::from_str(&value.extra_data)?;

        Ok(PublicDecryptRequest {
            ct_handles,
            extra_data,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum InputProofEventData {
    /// Event representing a input proof verification request from the user.
    ReqRcvdFromUser {
        input_proof_request: InputProofRequest,
    },

    /// Event representing the result of sending a input proof verification
    /// request to the gateway. Id will be used to map the response that will be
    /// received later to the request.
    ReqSentToGw { gw_req_reference_id: U256 },

    /// Event representing the response received from gateway for input
    /// proof verification request. Contains whether the proof was accepted
    /// and the response data if accepted.
    RespRcvdFromGw {
        accepted: bool,
        input_proof_response: Option<InputProofResponse>,
    },

    /// Event representing the failure in processing the input proof
    /// verification request.
    /// Used to notify outside internal handlers only.
    Failed { error: EventProcessingError },

    /// Event representing the internal failure in processing the input proof request: will not notify the user directly.
    InternalFailure { error: EventProcessingError },
}

impl InputProofEventData {
    pub fn event_name(&self) -> &'static str {
        match self {
            InputProofEventData::ReqRcvdFromUser { .. } => "Input::ReqRcvdFromUser",
            InputProofEventData::RespRcvdFromGw { .. } => "Input::RespRcvdFromGw",
            InputProofEventData::ReqSentToGw { .. } => "Input::ReqSentToGw",
            InputProofEventData::Failed { .. } => "Input::Failed",
            InputProofEventData::InternalFailure { .. } => "Input::InternalFailure",
        }
    }

    pub fn event_id(&self) -> u8 {
        match self {
            InputProofEventData::ReqRcvdFromUser { .. } => {
                InputProofEventId::ReqRcvdFromUser.into()
            }
            InputProofEventData::ReqSentToGw { .. } => InputProofEventId::ReqSentToGw.into(),
            InputProofEventData::RespRcvdFromGw { .. } => InputProofEventId::RespRcvdFromGw.into(),
            InputProofEventData::Failed { .. } => InputProofEventId::Failed.into(),
            InputProofEventData::InternalFailure { .. } => {
                InputProofEventId::InternalFailure.into()
            }
        }
    }
}

/// Chain-type high bit of a canonical RFC-021 `u64` chain id: set for Solana
/// hosts, clear for EVM. Matches `SOLANA_CHAIN_TYPE_BIT` in the coprocessor
/// (`fhevm-engine-common::chain_id`) and the js-sdk prover.
pub const SOLANA_CHAIN_TYPE_BIT: u64 = 1 << 63;

/// Whether a contract chain id denotes a Solana host (chain-type high bit set).
pub fn is_solana_host_chain_id(contract_chain_id: u64) -> bool {
    contract_chain_id & SOLANA_CHAIN_TYPE_BIT != 0
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputProofRequest {
    pub contract_chain_id: u64,
    pub contract_address: Address,
    pub user_address: Address,
    pub ciphetext_with_zk_proof: Bytes,
    pub extra_data: Bytes,
    /// Solana 32-byte host identities, set only when `contract_chain_id` carries
    /// the Solana chain-type high bit. EVM requests leave these `None` and use
    /// the 20-byte `contract_address`/`user_address` fields above. Exactly one
    /// representation is meaningful per request, decided by the chain id.
    #[serde(default)]
    pub solana_contract_address: Option<FixedBytes<32>>,
    #[serde(default)]
    pub solana_user_address: Option<FixedBytes<32>>,
}

impl InputProofRequest {
    pub fn new(
        contract_chain_id: u64,
        contract_address: Address,
        user_address: Address,
        ciphetext_with_zk_proof: Bytes,
        extra_data: Bytes,
    ) -> InputProofRequest {
        InputProofRequest {
            contract_chain_id,
            contract_address,
            user_address,
            ciphetext_with_zk_proof,
            extra_data,
            solana_contract_address: None,
            solana_user_address: None,
        }
    }

    /// Builds a Solana-host input-proof request carrying 32-byte identities. The
    /// 20-byte EVM `contract_address`/`user_address` are left zero — unused on
    /// the Solana path, which submits via `verifyProofRequestSolana`.
    pub fn new_solana(
        contract_chain_id: u64,
        contract_address: FixedBytes<32>,
        user_address: FixedBytes<32>,
        ciphetext_with_zk_proof: Bytes,
        extra_data: Bytes,
    ) -> InputProofRequest {
        InputProofRequest {
            contract_chain_id,
            contract_address: Address::ZERO,
            user_address: Address::ZERO,
            ciphetext_with_zk_proof,
            extra_data,
            solana_contract_address: Some(contract_address),
            solana_user_address: Some(user_address),
        }
    }

    /// Whether this request targets a Solana host (chain-type high bit set).
    pub fn is_solana(&self) -> bool {
        is_solana_host_chain_id(self.contract_chain_id)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputProofResponse {
    pub handles: Vec<FixedBytes<32>>,
    pub signatures: Vec<Bytes>,
}

impl InputProofResponse {
    pub fn new(handles: Vec<FixedBytes<32>>, signatures: Vec<Bytes>) -> InputProofResponse {
        InputProofResponse {
            handles,
            signatures,
        }
    }
}

impl TryFrom<InputProofRequestJson> for InputProofRequest {
    type Error = anyhow::Error;

    fn try_from(json: InputProofRequestJson) -> Result<Self, Self::Error> {
        info!("json.contractChainId: {:?}", json.contract_chain_id);
        let contract_chain_id = parse_chain_id(&json.contract_chain_id)
            .map_err(|e| anyhow::anyhow!("Error parsing contractChainId: {:?}", e))?;
        info!("contract_chain_id decoded: {:?}", contract_chain_id);

        // Should be hex string without a "0x" prefix.
        let proof_bytes = hex::decode(&json.ciphertext_with_input_verification).map_err(|e| {
            anyhow::anyhow!("Error decoding ciphertextWithInputVerification: {}", e)
        })?;
        let ciphetext_with_zk_proof = Bytes::from(proof_bytes);

        // Parse extraData (validated at HTTP layer)
        let extra_data = Bytes::from_str(&json.extra_data)?;

        // The chain-type high bit selects how the (HTTP-validated) identity
        // strings are interpreted: Solana hosts carry 32-byte base58 identities,
        // EVM hosts the usual 20-byte 0x-hex addresses.
        if is_solana_host_chain_id(contract_chain_id) {
            let contract_address =
                crate::http::utils::solana_address::decode_solana_address(&json.contract_address)
                    .map_err(|e| {
                    anyhow::anyhow!("Error parsing Solana contractAddress: {:?}", e.message)
                })?;
            let user_address =
                crate::http::utils::solana_address::decode_solana_address(&json.user_address)
                    .map_err(|e| {
                        anyhow::anyhow!("Error parsing Solana userAddress: {:?}", e.message)
                    })?;
            return Ok(InputProofRequest::new_solana(
                contract_chain_id,
                FixedBytes::<32>::from(contract_address),
                FixedBytes::<32>::from(user_address),
                ciphetext_with_zk_proof,
                extra_data,
            ));
        }

        let contract_address = Address::from_str(&json.contract_address)
            .map_err(|e| anyhow::anyhow!("Error parsing contractAddress: {:?}", e))?;

        let user_address = Address::from_str(&json.user_address)
            .map_err(|e| anyhow::anyhow!("Error parsing userAddress: {:?}", e))?;

        Ok(InputProofRequest::new(
            contract_chain_id,
            contract_address,
            user_address,
            ciphetext_with_zk_proof,
            extra_data,
        ))
    }
}

fn parse_chain_id(chain_id: &str) -> Result<u64, ParseIntError> {
    if let Some(stripped) = chain_id.strip_prefix("0x") {
        // Parse as hex if it starts with 0x
        u64::from_str_radix(stripped, 16)
    } else {
        // Parse as decimal otherwise
        chain_id.parse::<u64>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Constants for the test strings.
    const CHAIN_ID: &str = "123456";
    const CONTRACT_ADDRESS: &str = "0xAb30999D17FAAB8c95B2eCD500cFeFc8f658f15d";
    const USER_ADDRESS: &str = "0x12B064FB845C1cc05e9493856a1D637a73e944bE";
    const CIPHERTEXT: &str =
        "12B06C1cc05e9493856a1D637a74FAb30999D17FAAB8c95B2eCD500cFeFc8f658f15dB8453e944bE";
    const EXTRA_DATA: &str = "0x00";

    // Canonical 32-byte base58 Solana identities (Token program + wrapped-SOL mint).
    const SOLANA_CONTRACT: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
    const SOLANA_USER: &str = "So11111111111111111111111111111111111111112";
    // A canonical RFC-021 Solana chain id: chain-type high bit set | 12345.
    const SOLANA_CHAIN_ID_HEX: &str = "0x8000000000003039";

    #[test]
    fn solana_input_proof_request_carries_bytes32_identities() {
        use crate::http::utils::solana_address::decode_solana_address;

        let json = InputProofRequestJson {
            contract_chain_id: SOLANA_CHAIN_ID_HEX.to_string(),
            contract_address: SOLANA_CONTRACT.to_string(),
            user_address: SOLANA_USER.to_string(),
            ciphertext_with_input_verification: "abcd".to_string(),
            extra_data: "0x00".to_string(),
        };

        let request = InputProofRequest::try_from(json).expect("Solana request should parse");

        assert!(request.is_solana(), "high-bit chain id is a Solana host");
        assert_eq!(request.contract_chain_id, (1u64 << 63) | 12345);
        // 20-byte EVM fields are unused on the Solana path.
        assert_eq!(request.contract_address, Address::ZERO);
        assert_eq!(request.user_address, Address::ZERO);
        // 32-byte identities are populated from the base58 input.
        assert_eq!(
            request.solana_contract_address,
            Some(FixedBytes::<32>::from(
                decode_solana_address(SOLANA_CONTRACT).unwrap()
            ))
        );
        assert_eq!(
            request.solana_user_address,
            Some(FixedBytes::<32>::from(
                decode_solana_address(SOLANA_USER).unwrap()
            ))
        );
    }

    #[test]
    fn evm_input_proof_request_carries_evm_fields_and_leaves_solana_identities_unset() {
        let json = InputProofRequestJson {
            contract_chain_id: CHAIN_ID.to_string(),
            contract_address: CONTRACT_ADDRESS.to_string(),
            user_address: USER_ADDRESS.to_string(),
            ciphertext_with_input_verification: CIPHERTEXT.to_string(),
            extra_data: EXTRA_DATA.to_string(),
        };

        let request = InputProofRequest::try_from(json).expect("EVM request should parse");

        assert!(!request.is_solana(), "no high bit ⇒ EVM host");
        assert_eq!(request.contract_chain_id, CHAIN_ID.parse::<u64>().unwrap());
        assert_eq!(
            request.contract_address,
            CONTRACT_ADDRESS.parse::<Address>().unwrap()
        );
        assert_eq!(
            request.user_address,
            USER_ADDRESS.parse::<Address>().unwrap()
        );
        assert_eq!(
            request.ciphetext_with_zk_proof,
            Bytes::from(hex::decode(CIPHERTEXT).unwrap())
        );
        assert_eq!(request.solana_contract_address, None);
        assert_eq!(request.solana_user_address, None);
    }

    /// A `solana-srfc38-user-decrypt-v1` envelope routes to the host-generic `SolanaSrfc38V1`
    /// variant: the permit passes the typed pre-check, the whole request (permit, signature,
    /// per-handle evidence) is serialized into the opaque `host_payload`, and the fields the
    /// gateway calldata consumes are derived from the permit — the transport key as `publicKey`,
    /// the signed KMS routing as `extraData`, the handle list as `ctHandles`.
    #[test]
    fn solana_attested_user_decrypt_routes_to_host_generic() {
        use crate::http::endpoints::common::types::RequestValiditySecondsJson;
        use crate::http::endpoints::v3::types::{
            SolanaAttestedUserDecryptRequestJson, SolanaHandleJson,
            SolanaSrfc38UserDecryptPayloadJson,
        };

        let json = SolanaAttestedUserDecryptRequestJson {
            attestation_type: "solana-srfc38-user-decrypt-v1".to_string(),
            attested_payload: SolanaSrfc38UserDecryptPayloadJson {
                user_pubkey: format!("0x{}", "07".repeat(32)),
                // The transport key is length-checked (869 bytes) by the permit decode; its
                // contents are opaque here.
                transport_key: format!("0x{}", "00".repeat(869)),
                allowed_acl_domain_keys: vec![format!("0x{}", "05".repeat(32))],
                request_validity: RequestValiditySecondsJson {
                    start_timestamp: "1700000000".to_string(),
                    duration_seconds: "604800".to_string(),
                },
                verifying_program_id: format!("0x{}", "02".repeat(32)),
                // Chain id with the Solana chain-type high bit set.
                chain_id: (0x8000_0000_0000_0000u64 | 1).to_string(),
                // Signed KMS routing: version 0x02 ‖ contextId(32) ‖ epochId(32), 65 bytes.
                extra_data: format!("0x02{}{}", "0a".repeat(32), "0b".repeat(32)),
                handles: vec![SolanaHandleJson {
                    handle: format!("0x{}", "11".repeat(32)),
                    owner: format!("0x{}", "07".repeat(32)),
                    encrypted_value_id: format!("0x{}", "22".repeat(32)),
                    proof_leaf_count: "0".to_string(),
                    access_proof: "0x".to_string(),
                }],
            },
            signature: format!("0x{}", "ab".repeat(64)),
        };

        let request = UserDecryptRequest::try_from(json).expect("Solana envelope should convert");

        match request {
            UserDecryptRequest::SolanaSrfc38V1 {
                ct_handles,
                public_key,
                extra_data,
                host_payload,
                ..
            } => {
                assert_eq!(ct_handles, vec![U256::from_be_bytes::<32>([0x11; 32])]);
                // `publicKey` is the transport key verbatim (869 bytes).
                assert_eq!(public_key.len(), 869);
                // `extraData` is the signed KMS routing form (v0x02, 65 bytes).
                assert_eq!(extra_data.len(), 65);
                assert_eq!(extra_data[0], 0x02);
                // The whole request is serialized into the opaque host payload.
                assert_eq!(
                    host_payload[0], 0x01,
                    "host payload leads with its version byte"
                );
                assert!(host_payload.len() > 1);
            }
            other => panic!("expected SolanaSrfc38V1, got {}", other.attestation_kind()),
        }
    }

    /// A valid reference Solana payload as the typed JSON struct (pre-conversion).
    fn valid_solana_payload(
    ) -> crate::http::endpoints::v3::types::SolanaSrfc38UserDecryptPayloadJson {
        use crate::http::endpoints::common::types::RequestValiditySecondsJson;
        use crate::http::endpoints::v3::types::{
            SolanaHandleJson, SolanaSrfc38UserDecryptPayloadJson,
        };
        SolanaSrfc38UserDecryptPayloadJson {
            user_pubkey: format!("0x{}", "07".repeat(32)),
            transport_key: format!("0x{}", "00".repeat(869)),
            allowed_acl_domain_keys: vec![format!("0x{}", "05".repeat(32))],
            request_validity: RequestValiditySecondsJson {
                start_timestamp: "1700000000".to_string(),
                duration_seconds: "604800".to_string(),
            },
            verifying_program_id: format!("0x{}", "02".repeat(32)),
            chain_id: (0x8000_0000_0000_0000u64 | 1).to_string(),
            extra_data: format!("0x02{}{}", "0a".repeat(32), "0b".repeat(32)),
            handles: vec![SolanaHandleJson {
                handle: format!("0x{}", "11".repeat(32)),
                owner: format!("0x{}", "07".repeat(32)),
                encrypted_value_id: format!("0x{}", "22".repeat(32)),
                proof_leaf_count: "0".to_string(),
                access_proof: "0x".to_string(),
            }],
        }
    }

    fn solana_envelope(
        payload: crate::http::endpoints::v3::types::SolanaSrfc38UserDecryptPayloadJson,
    ) -> crate::http::endpoints::v3::types::SolanaAttestedUserDecryptRequestJson {
        crate::http::endpoints::v3::types::SolanaAttestedUserDecryptRequestJson {
            attestation_type: "solana-srfc38-user-decrypt-v1".to_string(),
            attested_payload: payload,
            signature: format!("0x{}", "ab".repeat(64)),
        }
    }

    #[test]
    fn solana_builder_rejects_a_malformed_access_proof() {
        // The access-proof form check mirrors the connector's decode: three arbitrary bytes are
        // not a borsh MMR proof.
        let mut payload = valid_solana_payload();
        payload.handles[0].access_proof = "0xffffff".to_string();
        let error = UserDecryptRequest::try_from(solana_envelope(payload))
            .expect_err("a malformed access proof must be rejected");
        assert!(
            error
                .to_string()
                .contains("does not decode as an MMR proof"),
            "got: {error}"
        );
    }

    #[test]
    fn solana_builder_rejects_a_wrong_length_transport_key() {
        // r1 (permit typed pre-check): the transport key length is fixed at 869; a shorter key has
        // no typed form and is rejected by the same permit decode the connector runs.
        let mut payload = valid_solana_payload();
        payload.transport_key = format!("0x{}", "00".repeat(800));
        let error = UserDecryptRequest::try_from(solana_envelope(payload))
            .expect_err("a wrong-length transport key must fail the pre-check");
        assert!(
            error.to_string().contains("typed pre-check"),
            "got: {error}"
        );
    }

    #[test]
    fn solana_payload_denies_unknown_and_evm_fields() {
        use crate::http::endpoints::v3::types::SolanaAttestedUserDecryptRequestJson;

        // The Solana payload is strict: a stray field — including the EVM `userAddress` or the
        // retired `nonce` — is refused at deserialization, not silently ignored.
        let base = serde_json::to_value(valid_solana_payload()).unwrap();
        for forbidden in ["userAddress", "nonce", "solanaNonce", "allowedContracts"] {
            let mut payload = base.clone();
            payload
                .as_object_mut()
                .unwrap()
                .insert(forbidden.to_string(), serde_json::json!("0x00"));
            let envelope = serde_json::json!({
                "attestationType": "solana-srfc38-user-decrypt-v1",
                "attestedPayload": payload,
                "signature": format!("0x{}", "ab".repeat(64)),
            });
            let parsed: Result<SolanaAttestedUserDecryptRequestJson, _> =
                serde_json::from_value(envelope);
            assert!(
                parsed.is_err(),
                "the Solana payload must reject the stray field `{forbidden}`"
            );
        }
    }

    /// A v3 envelope carrying `attestation_type`, otherwise a well-formed Solana payload. Both
    /// `solana*` fields are populated so a rejection can only come from the attestation type.
    fn attested_envelope(attestation_type: &str) -> AttestedUserDecryptRequestJson {
        use crate::http::endpoints::common::types::{HandleEntryJson, RequestValiditySecondsJson};
        use crate::http::endpoints::v3::types::Eip712UnifiedUserDecryptPayloadJson;

        let mut extra = vec![0x01u8];
        extra.extend_from_slice(&[0u8; 32]);

        AttestedUserDecryptRequestJson {
            attestation_type: attestation_type.to_string(),
            attested_payload: Eip712UnifiedUserDecryptPayloadJson {
                version: "2.0".to_string(),
                r#type: "user_decryption".to_string(),
                handles: vec![HandleEntryJson {
                    ct_handle: format!("0x{}", "11".repeat(32)),
                    contract_address: CONTRACT_ADDRESS.to_string(),
                    owner_address: USER_ADDRESS.to_string(),
                }],
                user_address: USER_ADDRESS.to_string(),
                allowed_contracts: vec![CONTRACT_ADDRESS.to_string()],
                request_validity: RequestValiditySecondsJson {
                    start_timestamp: "1700000000".to_string(),
                    duration_seconds: "604800".to_string(),
                },
                public_key: "0x04b8e5d3".to_string(),
                extra_data: format!("0x{}", hex::encode(&extra)),
            },
            signature: format!("0x{}", "ab".repeat(64)),
        }
    }

    /// An unrecognized `attestationType` is rejected by the conversion itself, not only by the
    /// HTTP-layer validator: an unknown protocol must never fall through onto the EVM arm and be
    /// served with EVM authorization rules (DD-027, validation is per-protocol).
    #[test]
    fn attested_user_decrypt_rejects_unknown_attestation_type() {
        let error =
            UserDecryptRequest::try_from(attested_envelope("solana-ed25519-user-decrypt-v3"))
                .expect_err("an unknown attestation type has no arm to route to");
        let message = error.to_string();
        assert!(
            message.contains("non-EVM attestationType"),
            "unexpected rejection reason: {message}"
        );
    }

    /// The other arm of the same dispatch: the EVM attestation type still routes to
    /// `Eip712UnifiedV1`, so the rejection above is narrow and not a blanket refusal. The
    /// `solana*` fields present in this fixture are tolerated and ignored on the EVM arm.
    #[test]
    fn attested_user_decrypt_routes_evm_attestation_type_to_eip712_unified() {
        let request =
            UserDecryptRequest::try_from(attested_envelope(V3_ATTESTATION_TYPE_EIP712_UNIFIED_V1))
                .expect("the EVM attestation type converts");
        match request {
            UserDecryptRequest::Eip712UnifiedV1 {
                user_address,
                allowed_contracts,
                ..
            } => {
                assert_eq!(user_address, USER_ADDRESS.parse::<Address>().unwrap());
                assert_eq!(
                    allowed_contracts,
                    vec![CONTRACT_ADDRESS.parse::<Address>().unwrap()]
                );
            }
            other => panic!("expected Eip712UnifiedV1, got {}", other.attestation_kind()),
        }
    }

    /// The DEPRECATED v2 user-decrypt path is EVM-only: a Solana host chain id is rejected at
    /// conversion instead of being converted with zeroed EVM address placeholders. The EVM twin of
    /// the same fixture must still convert, so the rejection is attributable to the chain id alone
    /// and not to a malformed fixture.
    #[test]
    fn legacy_v2_user_decrypt_rejects_solana_host_chain_id() {
        use crate::http::endpoints::common::types::{HandleContractPairJson, RequestValidityJson};

        let envelope = |chain_id: &str| UserDecryptRequestJson {
            handle_contract_pairs: vec![HandleContractPairJson {
                handle: format!("0x{}", "11".repeat(32)),
                contract_address: CONTRACT_ADDRESS.to_string(),
            }],
            request_validity: RequestValidityJson {
                start_timestamp: "1700000000".to_string(),
                duration_days: "1".to_string(),
            },
            contracts_chain_id: chain_id.to_string(),
            contract_addresses: vec![CONTRACT_ADDRESS.to_string()],
            user_address: USER_ADDRESS.to_string(),
            signature: "ab".repeat(65),
            public_key: "04b8e5d3".to_string(),
            extra_data: EXTRA_DATA.to_string(),
        };

        UserDecryptRequest::try_from(envelope(CHAIN_ID)).expect("the EVM twin still converts");

        let error = UserDecryptRequest::try_from(envelope(SOLANA_CHAIN_ID_HEX))
            .expect_err("a Solana host chain id has no legal v2 shape");
        let message = error.to_string();
        assert!(
            message.contains("v3 endpoint only"),
            "unexpected rejection reason: {message}"
        );
    }

    /// The Relayer must propagate `extraData` to the Gateway verbatim, without
    /// interpreting or rewriting any of its fields (version, contextId, epochId).
    #[test]
    fn test_public_decrypt_propagates_extra_data_verbatim() -> Result<(), Box<dyn std::error::Error>>
    {
        // Version 0x02: [version(1B) | contextId(32B) | epochId(32B)] = 65 bytes.
        let context_id = "00000000000000000000000000000000000000000000000000000000000000a1";
        let epoch_id = "00000000000000000000000000000000000000000000000000000000000000b2";
        let extra_data = format!("0x02{context_id}{epoch_id}");

        let json = PublicDecryptRequestJson {
            ciphertext_handles: vec![format!("0x{}", "11".repeat(32))],
            extra_data: extra_data.clone(),
        };

        let request = PublicDecryptRequest::try_from(json)?;

        // The parsed bytes must equal the raw input bytes, unchanged (verbatim propagation).
        assert_eq!(request.extra_data, Bytes::from_str(&extra_data)?);

        Ok(())
    }
}
