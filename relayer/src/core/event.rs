use crate::core::errors::EventProcessingError;
use crate::core::job_id::JobId;
use crate::http::endpoints::v2::types::DelegatedUserDecryptRequestJson;
use crate::http::endpoints::v2::types::{
    InputProofRequestJson, PublicDecryptRequestJson, UserDecryptRequestJson,
};
use crate::http::endpoints::v3::types::AttestedUserDecryptRequestJson;
use crate::http::utils::validations::V3_ATTESTATION_TYPE_SOLANA_ED25519_V1;
use crate::orchestrator::traits::Event;
use alloy::primitives::{Address, Bytes, FixedBytes, TxHash, B256};
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

#[repr(u8)]
#[derive(Debug)]
/// Event Ids corresponding to KeyUrl events.
pub enum KeyUrlEventId {
    KeyDataUpdated = 40,
}

impl From<KeyUrlEventId> for u8 {
    fn from(e: KeyUrlEventId) -> u8 {
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
            RelayerEventData::KeyUrl(e) => e.event_id(),
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
    KeyUrl(KeyUrlEventData),
}

impl AsRef<str> for RelayerEventData {
    fn as_ref(&self) -> &str {
        match self {
            RelayerEventData::GatewayChain(gateway_event) => gateway_event.event_name(),
            RelayerEventData::PublicDecrypt(decrypt_event) => decrypt_event.event_name(),
            RelayerEventData::UserDecrypt(decrypt_event) => decrypt_event.event_name(),
            RelayerEventData::InputProof(input_event) => input_event.event_name(),
            RelayerEventData::KeyUrl(keyurl_event) => keyurl_event.event_name(),
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
    /// Unified Solana ed25519 user-decryption (attestation_type
    /// `"solana-ed25519-user-decrypt-v1"`, RFC-021): maps to
    /// `userDecryptionRequestSolana(HandleEntry[], UserDecryptionRequestSolanaPayload)`.
    /// The ed25519 auth fields (`user_identity`, `nonce`, `allowed_acl_domain_keys`) are 32-byte
    /// Solana pubkeys carried as typed fields rather than packed into `extra_data`; `extra_data`
    /// carries only the KMS context. `signature` is the ed25519 signature, verified off-chain by
    /// the KMS Connector. `allowed_acl_domain_keys` may be empty (permissive mode).
    SolanaUnifiedV1 {
        handles: Vec<HandleEntry>,
        user_identity: B256,
        allowed_acl_domain_keys: Vec<B256>,
        request_validity: RequestValiditySeconds,
        nonce: B256,
        signature: Bytes,
        public_key: Bytes,
        extra_data: Bytes,
    },
}

impl UserDecryptRequest {
    /// Short label for logs / metrics. Matches the serde tag values.
    pub fn attestation_kind(&self) -> &'static str {
        match self {
            UserDecryptRequest::LegacyDirect { .. } => "legacy_direct",
            UserDecryptRequest::LegacyDelegated { .. } => "legacy_delegated",
            UserDecryptRequest::Eip712UnifiedV1 { .. } => "eip712_unified_v1",
            UserDecryptRequest::SolanaUnifiedV1 { .. } => "solana_unified_v1",
        }
    }

    /// Whether this request uses one of the unified gateway overloads (EVM or Solana).
    pub fn is_unified(&self) -> bool {
        matches!(
            self,
            UserDecryptRequest::Eip712UnifiedV1 { .. } | UserDecryptRequest::SolanaUnifiedV1 { .. }
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
            UserDecryptRequest::Eip712UnifiedV1 { handles, .. }
            | UserDecryptRequest::SolanaUnifiedV1 { handles, .. } => {
                handles.iter().map(|h| &h.ct_handle).collect()
            }
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

        // Parse the contract chain ID first: it selects the EVM vs RFC-021 (Solana) identity shapes.
        let contracts_chain_id = parse_chain_id(&value.contracts_chain_id)?;
        let is_solana = is_solana_host_chain_id(contracts_chain_id);

        // Parse handle/contract pairs. Handles are 32-byte values on both paths; on Solana the
        // per-pair contract identity is off-gateway (enforced by the KMS solana_acl), so it is not
        // an EVM address and is left zero.
        let mut ct_handle_contract_pairs = Vec::new();
        for json_data in &value.handle_contract_pairs {
            let ct_handle = if json_data.handle.starts_with("0x") {
                // Remove the 0x prefix before parsing
                U256::from_str_radix(&json_data.handle[2..], 16)
            } else {
                U256::from_str_radix(&json_data.handle, 16)
            }
            .map_err(|e| anyhow::anyhow!("Failed to parse ctHandle: {}", e))?;

            let contract_address = if is_solana {
                Address::ZERO
            } else {
                Address::from_str(&json_data.contract_address)
                    .map_err(|e| anyhow::anyhow!("Failed to parse contractAddress: {}", e))?
            };

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

        // On Solana the contract identities are off-gateway (enforced by the KMS solana_acl), so
        // the EVM `contract_addresses` list is empty; on EVM each entry is a 20-byte address.
        let contract_addresses = if is_solana {
            Vec::new()
        } else {
            value
                .contract_addresses
                .iter()
                .map(|addr| Address::from_str(addr))
                .collect::<Result<Vec<_>, _>>()?
        };

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

        // `attestation_type` (validated at the HTTP layer) selects the gateway overload: EVM
        // EIP-712 -> `userDecryptionRequest`, Solana ed25519 (RFC-021) -> `userDecryptionRequestSolana`.
        // `signature`, `publicKey` and `extraData` are forwarded verbatim (opaque to the relayer);
        // the relayer never verifies the signature — each KMS party's connector does. The Solana auth
        // fields travel as typed `solana*` values, not packed into `extraData`.
        let payload_inner = value.attested_payload;

        // Handles are shared across both unified paths. On Solana the per-handle EVM
        // contract/owner addresses are placeholders (the ACL is enforced off-gateway via the typed
        // `solana_allowed_acl_domain_keys`); the handle bytes are authoritative on both paths.
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

        if value.attestation_type == V3_ATTESTATION_TYPE_SOLANA_ED25519_V1 {
            let user_identity = B256::from_str(payload_inner.solana_user_identity.as_deref().ok_or_else(
                || anyhow::anyhow!("solanaUserIdentity is required for the Solana ed25519 attestation type"),
            )?)
            .map_err(|e| anyhow::anyhow!("Failed to parse solanaUserIdentity: {}", e))?;

            let nonce = B256::from_str(payload_inner.solana_nonce.as_deref().ok_or_else(|| {
                anyhow::anyhow!("solanaNonce is required for the Solana ed25519 attestation type")
            })?)
            .map_err(|e| anyhow::anyhow!("Failed to parse solanaNonce: {}", e))?;

            let allowed_acl_domain_keys = payload_inner
                .solana_allowed_acl_domain_keys
                .unwrap_or_default()
                .iter()
                .map(|k| B256::from_str(k))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| {
                    anyhow::anyhow!("Failed to parse solanaAllowedAclDomainKeys: {}", e)
                })?;

            return Ok(UserDecryptRequest::SolanaUnifiedV1 {
                handles,
                user_identity,
                allowed_acl_domain_keys,
                request_validity,
                nonce,
                signature,
                public_key,
                extra_data,
            });
        }

        let allowed_contracts = payload_inner
            .allowed_contracts
            .iter()
            .map(|addr| Address::from_str(addr))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(UserDecryptRequest::Eip712UnifiedV1 {
            handles,
            user_address: Address::from_str(&payload_inner.user_address)?,
            allowed_contracts,
            request_validity,
            signature,
            public_key,
            extra_data,
        })
    }
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

        // Note: we validate extraData to be 0x00 in the http listener.
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum KeyUrlEventData {
    /// Event representing updated key data.
    KeyDataUpdated { key_data: KeyUrlData },
}

impl KeyUrlEventData {
    pub fn event_name(&self) -> &'static str {
        match self {
            KeyUrlEventData::KeyDataUpdated { .. } => "KeyUrl::KeyDataUpdated",
        }
    }

    pub fn event_id(&self) -> u8 {
        match self {
            KeyUrlEventData::KeyDataUpdated { .. } => KeyUrlEventId::KeyDataUpdated.into(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyUrlData {
    pub fhe_public_key: KeyData,
    pub crs: KeyData,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyData {
    pub data_id: String,
    pub url: String,
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
    use std::convert::TryFrom;
    use std::str::FromStr;

    // Constants for the test strings.
    const CHAIN_ID: &str = "123456";
    const CONTRACT_ADDRESS: &str = "0xAb30999D17FAAB8c95B2eCD500cFeFc8f658f15d";
    const USER_ADDRESS: &str = "0x12B064FB845C1cc05e9493856a1D637a73e944bE";
    const CIPHERTEXT: &str =
        "12B06C1cc05e9493856a1D637a74FAb30999D17FAAB8c95B2eCD500cFeFc8f658f15dB8453e944bE";
    const EXTRA_DATA: &str = "0x00";

    #[test]
    #[ignore]
    fn test_input_proof_request_conversion_() -> Result<(), Box<dyn std::error::Error>> {
        let json = InputProofRequestJson {
            contract_chain_id: CHAIN_ID.to_string(),
            contract_address: CONTRACT_ADDRESS.to_string(),
            user_address: USER_ADDRESS.to_string(),
            ciphertext_with_input_verification: CIPHERTEXT.to_string(),
            extra_data: EXTRA_DATA.to_string(),
        };

        let request = InputProofRequest::try_from(json)?;

        assert_eq!(request.contract_chain_id, CHAIN_ID.parse::<u64>()?);
        assert_eq!(
            request.contract_address,
            Address::from_str(CONTRACT_ADDRESS)?
        );
        assert_eq!(request.user_address, Address::from_str(USER_ADDRESS)?);

        let expected_bytes = hex::decode(CIPHERTEXT)?;
        assert_eq!(request.ciphetext_with_zk_proof, Bytes::from(expected_bytes));

        Ok(())
    }

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
    fn evm_input_proof_request_leaves_solana_identities_unset() {
        let json = InputProofRequestJson {
            contract_chain_id: "123456".to_string(),
            contract_address: "0xAb30999D17FAAB8c95B2eCD500cFeFc8f658f15d".to_string(),
            user_address: "0x12B064FB845C1cc05e9493856a1D637a73e944bE".to_string(),
            ciphertext_with_input_verification: "abcd".to_string(),
            extra_data: "0x00".to_string(),
        };

        let request = InputProofRequest::try_from(json).expect("EVM request should parse");

        assert!(!request.is_solana(), "no high bit ⇒ EVM host");
        assert_eq!(request.solana_contract_address, None);
        assert_eq!(request.solana_user_address, None);
        assert_ne!(request.contract_address, Address::ZERO);
    }

    /// A Solana-attestationType v3 envelope routes to the `SolanaUnifiedV1` core variant (and hence
    /// the gateway `userDecryptionRequestSolana` calldata), carrying the ed25519 auth fields
    /// (identity, nonce, allowed ACL domain keys) as TYPED values and forwarding `signature` +
    /// context-only `extraData` unchanged. The EVM-shaped fields are placeholders.
    #[test]
    fn solana_attested_user_decrypt_routes_to_typed_solana_unified() {
        use crate::http::endpoints::common::types::{HandleEntryJson, RequestValiditySecondsJson};
        use crate::http::endpoints::v3::types::Eip712UnifiedUserDecryptPayloadJson;

        // 64-byte ed25519 signature (128 hex chars), forwarded opaquely.
        let signature_hex = format!("0x{}", "ab".repeat(64));
        // Context-only extraData (v0x01: version ‖ contextId(32)) — no Solana auth data here.
        let mut extra = vec![0x01u8];
        extra.extend_from_slice(&[0u8; 32]);
        let extra_data_hex = format!("0x{}", hex::encode(&extra));
        let public_key_hex = "0x04b8e5d3".to_string();
        let identity_hex = format!("0x{}", "07".repeat(32));
        let nonce_hex = format!("0x{}", "09".repeat(32));
        let domain_key_hex = format!("0x{}", "05".repeat(32));

        let json = AttestedUserDecryptRequestJson {
            attestation_type: "solana-ed25519-user-decrypt-v1".to_string(),
            attested_payload: Eip712UnifiedUserDecryptPayloadJson {
                version: "2.0".to_string(),
                r#type: "user_decryption".to_string(),
                handles: vec![HandleEntryJson {
                    ct_handle: format!("0x{}", "11".repeat(32)),
                    contract_address: "0xAb30999D17FAAB8c95B2eCD500cFeFc8f658f15d".to_string(),
                    owner_address: "0x12B064FB845C1cc05e9493856a1D637a73e944bE".to_string(),
                }],
                // EVM-shaped fields are placeholders for Solana; the typed `solana*` fields below
                // carry the real auth data.
                user_address: "0x12B064FB845C1cc05e9493856a1D637a73e944bE".to_string(),
                allowed_contracts: vec![],
                request_validity: RequestValiditySecondsJson {
                    start_timestamp: "1700000000".to_string(),
                    duration_seconds: "604800".to_string(),
                },
                public_key: public_key_hex.clone(),
                extra_data: extra_data_hex.clone(),
                solana_user_identity: Some(identity_hex.clone()),
                solana_nonce: Some(nonce_hex.clone()),
                solana_allowed_acl_domain_keys: Some(vec![domain_key_hex.clone()]),
            },
            signature: signature_hex.clone(),
        };

        let request = UserDecryptRequest::try_from(json).expect("Solana envelope should convert");

        match request {
            UserDecryptRequest::SolanaUnifiedV1 {
                signature,
                extra_data,
                public_key,
                user_identity,
                nonce,
                allowed_acl_domain_keys,
                ..
            } => {
                assert_eq!(signature, Bytes::from_str(&signature_hex).unwrap());
                assert_eq!(
                    extra_data,
                    Bytes::from_str(&extra_data_hex).unwrap(),
                    "extraData is context-only (no Solana auth data)"
                );
                assert_eq!(public_key, Bytes::from_str(&public_key_hex).unwrap());
                assert_eq!(user_identity, B256::from_str(&identity_hex).unwrap());
                assert_eq!(nonce, B256::from_str(&nonce_hex).unwrap());
                assert_eq!(
                    allowed_acl_domain_keys,
                    vec![B256::from_str(&domain_key_hex).unwrap()]
                );
            }
            other => panic!("expected SolanaUnifiedV1, got {}", other.attestation_kind()),
        }
    }
}
