//! Cheap, stateless request checks (no RPC, no DB). Any failure maps to `400 Bad Request`.

use crate::core::Config;
use alloy::primitives::{Address, B256, U256};
use connector_utils::types::{
    extra_data::parse_extra_data,
    handle::{extract_chain_id_from_handle, extract_fhe_type_from_handle},
    solana_request::{SolanaPublicDecryptionRequest, SolanaUserDecryptionRequestV1},
};
use kms_connector_api::{
    AttestationType, ErrorCode, MAX_PAD_DECRYPTORS, MAX_PAD_LENGTH,
    MAX_USER_DECRYPTION_DURATION_SECONDS, PAD_WITNESS_VERSION, PadDecryptionRequest,
    PublicDecryptionRequest, RequestValidity,
    SolanaPublicDecryptionRequest as SolanaPublicDecryptionBody, SolanaUserDecryptionRequest,
    UserDecryptionRequest,
};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::{collections::HashSet, ops::RangeInclusive};
use tfhe::FheTypes;
use thiserror::Error;
use zama_solana_acl::host_chain::{handle_chain_id, is_solana_host_chain_id};

/// The maximum total bit size of the handles of a single decryption request. Mirrors
/// `MAX_DECRYPTION_REQUEST_BITS` of the `Decryption` gateway contract.
const MAX_DECRYPTION_REQUEST_BITS: u64 = 2048;

/// The accepted `requestValidity.startTimestamp` values: must fit in an `i64` (DB storage).
const START_TIMESTAMP_RANGE: RangeInclusive<u64> = 0..=i64::MAX as u64;

/// The accepted `requestValidity.durationSeconds` values. Mirrors the bounds of the `Decryption`
/// gateway contract.
const DURATION_SECONDS_RANGE: RangeInclusive<u64> = 1..=MAX_USER_DECRYPTION_DURATION_SECONDS;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ValidationError {
    #[error("no handle provided")]
    NoHandles,
    #[error("too many handles: {0}, maximum is {1}")]
    TooManyHandles(usize, usize),
    #[error("total bit size of the handles too large: {0} bits, maximum is {1}")]
    BitSizeExceeded(u64, u64),
    #[error("too many allowed contracts: {0}, maximum is {1}")]
    TooManyAllowedContracts(usize, usize),
    #[error("invalid handle {handle}: {reason}")]
    InvalidHandle { handle: B256, reason: String },
    #[error("handles resolve to different chain ids ({0} and {1})")]
    MixedChainIds(u64, u64),
    #[error("unsupported chain id: {0}")]
    UnsupportedChainId(u64),
    #[error("invalid extraData: {0}")]
    InvalidExtraData(String),
    #[error("requestValidity.{field} is out of range: {value}, expected {min}..={max}")]
    RequestValidityOutOfRange {
        field: &'static str,
        value: u64,
        min: u64,
        max: u64,
    },
    #[error(
        "unsupported attestationType, supported: [{supported}]",
        supported = supported_attestation_types()
    )]
    UnsupportedAttestationType,
    #[error("invalid body: {0}")]
    InvalidBody(String),
    #[error("invalid Solana request: {0}")]
    InvalidSolanaRequest(String),
    #[error("chain id {0} is a Solana host chain, whose requests use the Solana body")]
    SolanaChainInEvmRequest(u64),
    #[error("{field} must not be the zero address")]
    ZeroAddress { field: &'static str },
    #[error("unsupported witnessVersion: {0}, supported: [{PAD_WITNESS_VERSION}]")]
    UnsupportedWitnessVersion(u8),
    #[error("no pad provided")]
    NoPads,
    #[error("too many pads: {0}, maximum is {1}")]
    TooManyPads(usize, usize),
    #[error("duplicate pad label {0}")]
    DuplicateLabel(B256),
    #[error("pad {label}: {reason}")]
    InvalidPad { label: B256, reason: String },
    #[error("pad {label}: label does not match its witness")]
    LabelMismatch { label: B256 },
}

fn supported_attestation_types() -> String {
    AttestationType::supported().collect::<Vec<_>>().join(", ")
}

impl ValidationError {
    /// The error code this failure is answered with.
    pub fn code(&self) -> ErrorCode {
        match self {
            Self::UnsupportedAttestationType => ErrorCode::UnsupportedAttestationType,
            Self::InvalidBody(_)
            | Self::InvalidSolanaRequest(_)
            | Self::SolanaChainInEvmRequest(_)
            | Self::NoHandles
            | Self::TooManyHandles(..)
            | Self::BitSizeExceeded(..)
            | Self::TooManyAllowedContracts(..)
            | Self::InvalidHandle { .. }
            | Self::MixedChainIds(..)
            | Self::UnsupportedChainId(_)
            | Self::InvalidExtraData(_)
            | Self::ZeroAddress { .. }
            | Self::RequestValidityOutOfRange { .. }
            | Self::UnsupportedWitnessVersion(_)
            | Self::NoPads
            | Self::TooManyPads(..)
            | Self::DuplicateLabel(_)
            | Self::InvalidPad { .. }
            | Self::LabelMismatch { .. } => ErrorCode::Malformed,
        }
    }
}

pub fn validate_public_decryption(
    request: &PublicDecryptionRequest,
    config: &Config,
) -> Result<(), ValidationError> {
    validate_handles(&request.ctHandles, config)?;
    validate_extra_data(&request.extraData)
}

/// Checks the Solana body as [`validate_public_decryption`] checks the EVM one, and pairs each
/// handle with its store.
pub fn validate_solana_public_decryption(
    id: B256,
    request: &SolanaPublicDecryptionBody,
    config: &Config,
) -> Result<SolanaPublicDecryptionRequest, ValidationError> {
    validate_handles(&request.ctHandles, config)?;
    validate_extra_data(&request.extraData)?;
    SolanaPublicDecryptionRequest::new(
        U256::from_be_bytes(id.0),
        &request.ctHandles,
        &request.encryptedStores,
        request.extraData.to_vec(),
    )
    .map_err(|e| ValidationError::InvalidSolanaRequest(e.to_string()))
}

/// Stateless checks of the RFC-016 permit fields shared by the user and pad decrypt routes.
fn validate_permit(
    attestation_type: &str,
    user_address: Address,
    allowed_contracts_len: usize,
    request_validity: &RequestValidity,
    config: &Config,
) -> Result<(), ValidationError> {
    // Both bodies carry an EIP-712 permit; the Solana scheme has its own body.
    if attestation_type.parse() != Ok(AttestationType::Eip712UnifiedUserDecryptV1) {
        return Err(ValidationError::UnsupportedAttestationType);
    }
    validate_user_address(user_address)?;
    if allowed_contracts_len > config.max_allowed_contracts {
        return Err(ValidationError::TooManyAllowedContracts(
            allowed_contracts_len,
            config.max_allowed_contracts,
        ));
    }
    validate_request_validity(request_validity)
}

pub fn validate_user_decryption(
    request: &UserDecryptionRequest,
    config: &Config,
) -> Result<(), ValidationError> {
    let payload = &request.payload;
    validate_permit(
        &request.attestationType,
        payload.userAddress,
        payload.allowedContracts.len(),
        &payload.requestValidity,
        config,
    )?;
    validate_evm_handles(payload.handles.iter().map(|h| &h.handle), config)?;
    validate_extra_data(&payload.extraData)
}

pub fn validate_pad_decryption(
    request: &PadDecryptionRequest,
    config: &Config,
    now: u64,
) -> Result<(), ValidationError> {
    let payload = &request.payload;
    validate_permit(
        &request.attestationType,
        payload.userAddress,
        payload.allowedContracts.len(),
        &payload.requestValidity,
        config,
    )?;
    validate_extra_data_v2(&payload.extraData)?;

    if payload.witnessVersion != PAD_WITNESS_VERSION {
        return Err(ValidationError::UnsupportedWitnessVersion(
            payload.witnessVersion,
        ));
    }
    if is_solana_host_chain_id(payload.chainId) {
        return Err(ValidationError::SolanaChainInEvmRequest(payload.chainId));
    }
    if !config.supported_chain_ids.contains(&payload.chainId) {
        return Err(ValidationError::UnsupportedChainId(payload.chainId));
    }
    let max_pads = config.max_pads.get();
    match payload.pads.len() {
        0 => return Err(ValidationError::NoPads),
        n if n > max_pads => return Err(ValidationError::TooManyPads(n, max_pads)),
        _ => (),
    }

    let mut labels = HashSet::with_capacity(payload.pads.len());
    for pad in &payload.pads {
        if !labels.insert(pad.label) {
            return Err(ValidationError::DuplicateLabel(pad.label));
        }
        let invalid = |reason: String| ValidationError::InvalidPad {
            label: pad.label,
            reason,
        };
        let witness = &pad.witness;
        if witness.submitter == Address::ZERO {
            return Err(invalid("submitter must not be the zero address".into()));
        }
        if witness.decryptors.len() > usize::from(MAX_PAD_DECRYPTORS) {
            return Err(invalid(format!(
                "too many decryptors: {}, maximum is {MAX_PAD_DECRYPTORS}",
                witness.decryptors.len()
            )));
        }
        if witness.decryptors.contains(&Address::ZERO) {
            return Err(invalid(
                "decryptors must not contain the zero address".into(),
            ));
        }
        if !(1..=MAX_PAD_LENGTH).contains(&pad.padLength) {
            return Err(invalid(format!(
                "padLength is out of range: {}, expected 1..={MAX_PAD_LENGTH}",
                pad.padLength
            )));
        }
        if witness.expirationTimestamp <= now {
            return Err(invalid(format!(
                "expired at {}, now is {now}",
                witness.expirationTimestamp
            )));
        }

        if payload.label_of(witness) != pad.label {
            return Err(ValidationError::LabelMismatch { label: pad.label });
        }
    }
    Ok(())
}

/// The zero address is what `ecrecover` yields on failure: no request can be signed for it.
fn validate_user_address(user_address: Address) -> Result<(), ValidationError> {
    if user_address == Address::ZERO {
        return Err(ValidationError::ZeroAddress {
            field: "userAddress",
        });
    }
    Ok(())
}

/// Checks the Solana payload as [`validate_user_decryption`] checks the EIP-712 one. The permit's
/// chain id is the one the handles embed. The signature is verified by the worker.
pub fn validate_solana_user_decryption(
    id: B256,
    request: &SolanaUserDecryptionRequest,
    config: &Config,
) -> Result<SolanaUserDecryptionRequestV1, ValidationError> {
    let payload = &request.payload;
    validate_handles(payload.handles.iter().map(|h| &h.handle), config)?;
    validate_request_validity(&payload.requestValidity)?;
    validate_extra_data(&payload.extraData)?;
    let request = request
        .assemble()
        .map_err(|e| ValidationError::InvalidSolanaRequest(e.to_string()))?;
    Ok(SolanaUserDecryptionRequestV1 {
        decryption_id: U256::from_be_bytes(id.0),
        request,
    })
}

/// Read before the payload, so an unsupported scheme is reported as such whatever shape its
/// payload has.
pub fn attestation_type(body: &Value) -> Result<AttestationType, ValidationError> {
    body.get("attestationType")
        .and_then(Value::as_str)
        .ok_or_else(|| ValidationError::InvalidBody("attestationType must be a string".into()))?
        .parse()
        .map_err(|_| ValidationError::UnsupportedAttestationType)
}

/// Whether a public-decryption body is the Solana one: its first handle names a Solana chain. A
/// body without a readable first handle is the EVM one, whose parse refuses it.
pub fn is_solana_public_decryption(body: &Value) -> bool {
    body.get("ctHandles")
        .and_then(|handles| handles.get(0))
        .and_then(Value::as_str)
        .and_then(|handle| handle.parse::<B256>().ok())
        .is_some_and(|handle| is_solana_host_chain_id(handle_chain_id(&handle.0)))
}

pub fn parse_body<T: DeserializeOwned>(body: Value) -> Result<T, ValidationError> {
    serde_json::from_value(body).map_err(|e| ValidationError::InvalidBody(e.to_string()))
}

fn validate_request_validity(validity: &RequestValidity) -> Result<(), ValidationError> {
    for (field, value, range) in [
        (
            "startTimestamp",
            validity.startTimestamp,
            START_TIMESTAMP_RANGE,
        ),
        (
            "durationSeconds",
            validity.durationSeconds,
            DURATION_SECONDS_RANGE,
        ),
    ] {
        if !range.contains(&value) {
            return Err(ValidationError::RequestValidityOutOfRange {
                field,
                value,
                min: *range.start(),
                max: *range.end(),
            });
        }
    }
    Ok(())
}

/// Checks EVM request handles. A Solana chain id names a host this body cannot authorize on.
fn validate_evm_handles<'a, I>(handles: I, config: &Config) -> Result<(), ValidationError>
where
    I: IntoIterator<Item = &'a B256>,
    I::IntoIter: ExactSizeIterator,
{
    let chain_id = validate_handles(handles, config)?;
    if is_solana_host_chain_id(chain_id) {
        return Err(ValidationError::SolanaChainInEvmRequest(chain_id));
    }
    Ok(())
}

/// Checks the handle list: non-empty, within the handle count limit, well-formed handles of a
/// decryptable FHE type, on a single supported chain, within the total bit size budget.
fn validate_handles<'a, I>(handles: I, config: &Config) -> Result<u64, ValidationError>
where
    I: IntoIterator<Item = &'a B256>,
    I::IntoIter: ExactSizeIterator,
{
    let handles = handles.into_iter();
    let max_handles = config.max_handles_per_decryption.get();
    if handles.len() > max_handles {
        return Err(ValidationError::TooManyHandles(handles.len(), max_handles));
    }

    let (mut chain_id, mut total_bits) = (None, 0u64);
    for handle in handles {
        let fhe_type =
            extract_fhe_type_from_handle(handle).map_err(|e| ValidationError::InvalidHandle {
                handle: *handle,
                reason: e.to_string(),
            })?;
        let bits = fhe_type_size(handle, fhe_type)?;
        total_bits += u64::from(bits);

        let handle_chain_id =
            extract_chain_id_from_handle(handle).map_err(|e| ValidationError::InvalidHandle {
                handle: *handle,
                reason: e.to_string(),
            })?;
        match chain_id {
            None => chain_id = Some(handle_chain_id),
            Some(expected) if expected != handle_chain_id => {
                return Err(ValidationError::MixedChainIds(expected, handle_chain_id));
            }
            Some(_) => {}
        }
    }

    let Some(chain_id) = chain_id else {
        return Err(ValidationError::NoHandles);
    };
    if total_bits > MAX_DECRYPTION_REQUEST_BITS {
        return Err(ValidationError::BitSizeExceeded(
            total_bits,
            MAX_DECRYPTION_REQUEST_BITS,
        ));
    }
    if !config.supported_chain_ids.contains(&chain_id) {
        return Err(ValidationError::UnsupportedChainId(chain_id));
    }
    Ok(chain_id)
}

fn fhe_type_size(handle: &B256, fhe_type: FheTypes) -> Result<u16, ValidationError> {
    let size = match fhe_type {
        FheTypes::Bool => 2,
        FheTypes::Uint8 => 8,
        FheTypes::Uint16 => 16,
        FheTypes::Uint32 => 32,
        FheTypes::Uint64 => 64,
        FheTypes::Uint128 => 128,
        FheTypes::Uint160 => 160,
        FheTypes::Uint256 => 256,
        _ => {
            return Err(ValidationError::InvalidHandle {
                handle: *handle,
                reason: format!("FHE type {fhe_type:?} cannot be decrypted"),
            });
        }
    };
    Ok(size)
}

fn validate_extra_data(extra_data: &[u8]) -> Result<(), ValidationError> {
    parse_extra_data(extra_data)
        .map(|_| ())
        .map_err(|e| ValidationError::InvalidExtraData(format!("{e:#}")))
}

/// Pad decryption only accepts a `v2` `extraData`, which names the epoch to serve it under.
fn validate_extra_data_v2(extra_data: &[u8]) -> Result<(), ValidationError> {
    let parsed = parse_extra_data(extra_data)
        .map_err(|e| ValidationError::InvalidExtraData(format!("{e:#}")))?;
    if parsed.epoch_id.is_none() {
        return Err(ValidationError::InvalidExtraData(
            "pad decryption requires a v2 extraData (context and epoch)".into(),
        ));
    }
    Ok(())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use alloy::primitives::{Bytes, FixedBytes};
    use connector_utils::types::solana_request::MAX_REQUEST_HANDLES;
    use kms_connector_api::{
        AttestationType, HandleEntry, PadDecryptionPayload, PadEntry, PadWitness,
        SolanaHandleEntry, SolanaUserDecryptionPayload, UserDecryptionPayload,
    };
    use std::num::NonZeroUsize;

    const EBOOL: u8 = FheTypes::Bool as u8;
    const EUINT4: u8 = FheTypes::Uint4 as u8;
    const EUINT64: u8 = FheTypes::Uint64 as u8;
    const EUINT256: u8 = FheTypes::Uint256 as u8;
    /// A type byte no `FheTypes` variant maps to.
    const BAD_FHE_TYPE: u8 = 0xff;

    /// Builds a handle with the given chain id and FHE type byte at the protocol offsets.
    pub fn handle(chain_id: u64, fhe_type: u8) -> B256 {
        let mut bytes = [0x11u8; 32];
        bytes[22..30].copy_from_slice(&chain_id.to_be_bytes());
        bytes[30] = fhe_type;
        bytes[31] = 0; // version
        B256::from(bytes)
    }

    pub fn config() -> Config {
        Config {
            supported_chain_ids: vec![1, 2],
            max_allowed_contracts: 1,
            max_handles_per_decryption: NonZeroUsize::new(3).unwrap(),
            ..Config::default()
        }
    }

    /// A well-formed v2 `extraData` (version, context id, epoch id).
    fn extra_data_v2() -> Vec<u8> {
        [vec![0x02], vec![0xaa; 32], vec![0xbb; 32]].concat()
    }

    pub fn public_request(handles: Vec<B256>, extra_data: Vec<u8>) -> PublicDecryptionRequest {
        PublicDecryptionRequest {
            ctHandles: handles,
            extraData: Bytes::from(extra_data),
        }
    }

    pub fn user_request(handles: Vec<B256>, extra_data: Vec<u8>) -> UserDecryptionRequest {
        UserDecryptionRequest {
            attestationType: AttestationType::Eip712UnifiedUserDecryptV1.to_string(),
            payload: UserDecryptionPayload {
                handles: handles
                    .into_iter()
                    .map(|handle| HandleEntry {
                        handle,
                        contractAddress: Address::repeat_byte(0x33),
                        ownerAddress: Address::repeat_byte(0x44),
                    })
                    .collect(),
                userAddress: Address::repeat_byte(0x55),
                publicKey: Bytes::from(vec![0x20; 32]),
                allowedContracts: vec![Address::repeat_byte(0x33)],
                requestValidity: RequestValidity {
                    startTimestamp: 1_770_000_000,
                    durationSeconds: 300,
                },
                extraData: Bytes::from(extra_data),
            },
            signature: Bytes::from(vec![0x66; 65]),
        }
    }

    /// A Solana user decryption of `handles`, all with one owner and one store.
    fn solana_user_request(
        handles: Vec<B256>,
        start_timestamp: u64,
        extra_data: Vec<u8>,
    ) -> SolanaUserDecryptionRequest {
        SolanaUserDecryptionRequest {
            attestationType: AttestationType::SolanaSrfc38UserDecryptV1.to_string(),
            payload: SolanaUserDecryptionPayload {
                handles: handles
                    .into_iter()
                    .map(|handle| SolanaHandleEntry {
                        handle,
                        ownerAddress: B256::repeat_byte(0x01),
                        encryptedStore: B256::repeat_byte(0x03),
                    })
                    .collect(),
                userAddress: B256::repeat_byte(0x01),
                publicKey: Bytes::from(vec![0x20; 32]),
                allowedScopes: vec![],
                requestValidity: RequestValidity {
                    startTimestamp: start_timestamp,
                    durationSeconds: 300,
                },
                verifyingProgramId: B256::repeat_byte(0x07),
                extraData: Bytes::from(extra_data),
            },
            signature: Bytes::from(vec![0x66; 64]),
        }
    }

    /// A Solana public decryption of `handles`, one store each.
    fn solana_public_request(handles: Vec<B256>) -> SolanaPublicDecryptionBody {
        SolanaPublicDecryptionBody {
            encryptedStores: vec![B256::repeat_byte(0x03); handles.len()],
            ctHandles: handles,
            extraData: Bytes::from(extra_data_v2()),
        }
    }

    /// A Solana public decryption names its KMS context: kms-worker checks its node's membership in
    /// it before any proof read.
    #[test]
    fn a_solana_public_decryption_without_a_kms_context_is_malformed() {
        let solana_chain = zama_solana_acl::host_chain::solana_host_chain_id(12345);
        let cfg = Config {
            supported_chain_ids: vec![solana_chain],
            ..config()
        };
        let mut request = solana_public_request(vec![handle(solana_chain, EUINT64)]);
        // The extraData check every body shares refuses them before the Solana form check.
        for without_context in [vec![], vec![0x00]] {
            request.extraData = Bytes::from(without_context);
            let error = validate_solana_public_decryption(B256::ZERO, &request, &cfg).unwrap_err();
            assert!(
                matches!(&error, ValidationError::InvalidExtraData(_)),
                "{error}"
            );
            assert_eq!(error.code(), ErrorCode::Malformed);
        }
        for with_context in [[vec![0x01], vec![0xaa; 32]].concat(), extra_data_v2()] {
            request.extraData = Bytes::from(with_context);
            validate_solana_public_decryption(B256::ZERO, &request, &cfg).unwrap();
        }
    }

    #[test]
    fn valid_requests_pass() {
        let cfg = config();
        let handles = vec![handle(1, EUINT64), handle(1, EUINT64)];
        assert_eq!(
            validate_public_decryption(&public_request(handles.clone(), extra_data_v2()), &cfg),
            Ok(())
        );
        assert_eq!(
            validate_user_decryption(&user_request(handles, extra_data_v2()), &cfg),
            Ok(())
        );
    }

    #[test]
    fn empty_handles() {
        let cfg = config();
        assert_eq!(
            validate_public_decryption(&public_request(vec![], extra_data_v2()), &cfg),
            Err(ValidationError::NoHandles)
        );
        assert_eq!(
            validate_user_decryption(&user_request(vec![], extra_data_v2()), &cfg),
            Err(ValidationError::NoHandles)
        );
    }

    #[test]
    fn too_many_handles() {
        let cfg = config();
        let handles = vec![handle(1, EBOOL); 3];
        assert_eq!(
            validate_public_decryption(&public_request(handles.clone(), extra_data_v2()), &cfg),
            Ok(())
        );
        assert_eq!(
            validate_user_decryption(&user_request(handles, extra_data_v2()), &cfg),
            Ok(())
        );

        let handles = vec![handle(1, EBOOL); 4];
        assert_eq!(
            validate_public_decryption(&public_request(handles.clone(), extra_data_v2()), &cfg),
            Err(ValidationError::TooManyHandles(4, 3))
        );
        assert_eq!(
            validate_user_decryption(&user_request(handles, extra_data_v2()), &cfg),
            Err(ValidationError::TooManyHandles(4, 3))
        );
    }

    #[test]
    fn bit_size_budget() {
        let mut cfg = config();
        cfg.max_handles_per_decryption = NonZeroUsize::new(9).unwrap();
        // 8 * 256 = 2048 fits exactly, one more bit exceeds the budget.
        let mut handles = vec![handle(1, EUINT256); 8];
        handles.push(handle(1, EBOOL));
        assert_eq!(
            validate_public_decryption(&public_request(handles, extra_data_v2()), &cfg),
            Err(ValidationError::BitSizeExceeded(2050, 2048))
        );
        let handles = vec![handle(1, EUINT256); 8];
        assert_eq!(
            validate_user_decryption(&user_request(handles, extra_data_v2()), &cfg),
            Ok(())
        );
    }

    #[test]
    fn non_decryptable_fhe_type() {
        let cfg = config();
        let bad = handle(1, EUINT4);
        match validate_public_decryption(&public_request(vec![bad], extra_data_v2()), &cfg) {
            Err(ValidationError::InvalidHandle { handle, reason }) => {
                assert_eq!(handle, bad);
                assert!(reason.contains("cannot be decrypted"), "{reason}");
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn zero_user_address_is_rejected() {
        let mut request = user_request(vec![handle(1, EUINT64)], extra_data_v2());
        request.payload.userAddress = Address::ZERO;
        let error = validate_user_decryption(&request, &config()).unwrap_err();
        assert_eq!(
            error,
            ValidationError::ZeroAddress {
                field: "userAddress"
            }
        );
        assert_eq!(error.code(), ErrorCode::Malformed);
    }

    #[test]
    fn too_many_allowed_contracts() {
        let cfg = config();
        let mut request = user_request(vec![handle(1, EUINT64)], extra_data_v2());
        request
            .payload
            .allowedContracts
            .push(Address::repeat_byte(0x77));
        assert_eq!(
            validate_user_decryption(&request, &cfg),
            Err(ValidationError::TooManyAllowedContracts(2, 1))
        );
    }

    #[test]
    fn request_validity_out_of_range() {
        let cfg = config();
        let (start, duration) = (START_TIMESTAMP_RANGE, DURATION_SECONDS_RANGE);
        let in_range = [
            ("startTimestamp", *start.start()),
            ("startTimestamp", *start.end()),
            ("durationSeconds", *duration.start()),
            ("durationSeconds", *duration.end()),
        ];
        let out_of_range = [
            ("startTimestamp", *start.end() + 1, &start),
            ("durationSeconds", *duration.start() - 1, &duration),
            ("durationSeconds", *duration.end() + 1, &duration),
        ];
        let request_with = |field, value| {
            let mut request = user_request(vec![handle(1, EUINT64)], extra_data_v2());
            let validity = &mut request.payload.requestValidity;
            match field {
                "startTimestamp" => validity.startTimestamp = value,
                _ => validity.durationSeconds = value,
            }
            request
        };

        for (field, value) in in_range {
            assert_eq!(
                validate_user_decryption(&request_with(field, value), &cfg),
                Ok(())
            );
        }
        for (field, value, range) in out_of_range {
            assert_eq!(
                validate_user_decryption(&request_with(field, value), &cfg),
                Err(ValidationError::RequestValidityOutOfRange {
                    field,
                    value,
                    min: *range.start(),
                    max: *range.end(),
                })
            );
        }
    }

    #[test]
    fn unsupported_attestation_type_whatever_the_payload() {
        let body = serde_json::json!({
            "attestationType": "random_attestation_type",
            "payload": { "shape": "unknown" },
        });
        let error = attestation_type(&body).unwrap_err();
        assert_eq!(error, ValidationError::UnsupportedAttestationType);
        assert_eq!(error.code(), ErrorCode::UnsupportedAttestationType);

        let error = attestation_type(&serde_json::json!({ "payload": {} })).unwrap_err();
        assert!(matches!(error, ValidationError::InvalidBody(_)));
        assert_eq!(error.code(), ErrorCode::Malformed);
        assert_eq!(ValidationError::NoHandles.code(), ErrorCode::Malformed);
    }

    /// The timestamp the pad test requests are built around.
    const NOW: u64 = 1_770_000_000;

    /// A valid pad entry for `chain_id`, made unique by `nonce`.
    pub fn pad_entry(chain_id: u64, nonce: u8) -> PadEntry {
        let witness = PadWitness {
            submitter: Address::repeat_byte(0x55),
            consumer: Address::repeat_byte(0x33),
            expirationTimestamp: NOW + 3600,
            decryptors: vec![Address::repeat_byte(0x66)],
            nonce: FixedBytes::repeat_byte(nonce),
        };
        PadEntry {
            label: witness.label(PAD_WITNESS_VERSION, 0, chain_id),
            witness,
            padLength: 64,
        }
    }

    pub fn pad_request(pads: Vec<PadEntry>, extra_data: Vec<u8>) -> PadDecryptionRequest {
        PadDecryptionRequest {
            attestationType: AttestationType::Eip712UnifiedUserDecryptV1.to_string(),
            payload: PadDecryptionPayload {
                witnessVersion: PAD_WITNESS_VERSION,
                prfId: 0,
                chainId: 1,
                pads,
                userAddress: Address::repeat_byte(0x55),
                publicKey: Bytes::from(vec![0x20; 32]),
                allowedContracts: vec![Address::repeat_byte(0x33)],
                requestValidity: RequestValidity {
                    startTimestamp: NOW - 60,
                    durationSeconds: 300,
                },
                extraData: Bytes::from(extra_data),
            },
            signature: Bytes::from(vec![0x66; 65]),
        }
    }

    fn valid_pad_request() -> PadDecryptionRequest {
        pad_request(vec![pad_entry(1, 1), pad_entry(1, 2)], extra_data_v2())
    }

    /// Validates `valid_pad_request()` once modified by `mutate`.
    fn validate_pad_with(
        mutate: impl FnOnce(&mut PadDecryptionRequest),
    ) -> Result<(), ValidationError> {
        let mut request = valid_pad_request();
        mutate(&mut request);
        validate_pad_decryption(&request, &config(), NOW)
    }

    /// Re-derives the label of every pad, so a witness mutation only trips the targeted check.
    fn relabel(request: &mut PadDecryptionRequest) {
        let payload = &mut request.payload;
        let (version, prf_id, chain_id) = (payload.witnessVersion, payload.prfId, payload.chainId);
        for pad in &mut payload.pads {
            pad.label = pad.witness.label(version, prf_id, chain_id);
        }
    }

    #[test]
    fn valid_pad_request_passes() {
        assert_eq!(validate_pad_with(|_| {}), Ok(()));
        // No decryptor, permissive signature, extreme pad lengths.
        assert_eq!(
            validate_pad_with(|r| {
                r.payload.allowedContracts.clear();
                r.payload.pads[0].witness.decryptors.clear();
                r.payload.pads[0].padLength = 1;
                r.payload.pads[1].padLength = MAX_PAD_LENGTH;
                relabel(r);
            }),
            Ok(())
        );
    }

    #[test]
    fn pad_request_structure() {
        for attestation_type in [
            "wrong-attestation".to_string(),
            AttestationType::SolanaSrfc38UserDecryptV1.to_string(),
        ] {
            assert_eq!(
                validate_pad_with(|r| r.attestationType = attestation_type),
                Err(ValidationError::UnsupportedAttestationType)
            );
        }
        assert_eq!(
            validate_pad_with(|r| r.payload.userAddress = Address::ZERO),
            Err(ValidationError::ZeroAddress {
                field: "userAddress"
            })
        );
        assert_eq!(
            validate_pad_with(|r| r.payload.allowedContracts.push(Address::repeat_byte(0x34))),
            Err(ValidationError::TooManyAllowedContracts(2, 1))
        );
        assert!(matches!(
            validate_pad_with(|r| r.payload.requestValidity.durationSeconds = 0),
            Err(ValidationError::RequestValidityOutOfRange { .. })
        ));
        assert_eq!(
            validate_pad_with(|r| {
                r.payload.witnessVersion = 1;
                relabel(r);
            }),
            Err(ValidationError::UnsupportedWitnessVersion(1))
        );
        assert_eq!(
            validate_pad_with(|r| {
                r.payload.chainId = 9;
                relabel(r);
            }),
            Err(ValidationError::UnsupportedChainId(9))
        );
    }

    #[test]
    fn pad_request_requires_v2_extra_data() {
        for extra_data in [vec![], [vec![0x01], vec![0xaa; 32]].concat()] {
            assert!(matches!(
                validate_pad_with(|r| r.payload.extraData = Bytes::from(extra_data)),
                Err(ValidationError::InvalidExtraData(_))
            ));
        }
    }

    #[test]
    fn pad_count_bounds() {
        assert_eq!(
            validate_pad_with(|r| r.payload.pads.clear()),
            Err(ValidationError::NoPads)
        );
        let mut cfg = config();
        cfg.max_pads = NonZeroUsize::new(2).unwrap();
        let request = pad_request((1..=2).map(|n| pad_entry(1, n)).collect(), extra_data_v2());
        assert_eq!(validate_pad_decryption(&request, &cfg, NOW), Ok(()));
        let request = pad_request((1..=3).map(|n| pad_entry(1, n)).collect(), extra_data_v2());
        assert_eq!(
            validate_pad_decryption(&request, &cfg, NOW),
            Err(ValidationError::TooManyPads(3, 2))
        );
    }

    #[test]
    fn duplicate_labels_are_rejected() {
        let duplicate = pad_entry(1, 1);
        assert_eq!(
            validate_pad_with(|r| r.payload.pads.push(duplicate.clone())),
            Err(ValidationError::DuplicateLabel(duplicate.label))
        );
    }

    #[test]
    fn pad_entry_structure() {
        let is_invalid_pad = |result, needle: &str| match result {
            Err(ValidationError::InvalidPad { reason, .. }) => {
                assert!(reason.contains(needle), "{reason}")
            }
            other => panic!("unexpected: {other:?}"),
        };
        is_invalid_pad(
            validate_pad_with(|r| {
                r.payload.pads[1].witness.submitter = Address::ZERO;
                relabel(r);
            }),
            "submitter",
        );
        is_invalid_pad(
            validate_pad_with(|r| {
                r.payload.pads[1].witness.decryptors.push(Address::ZERO);
                relabel(r);
            }),
            "decryptors",
        );
        for pad_length in [0, MAX_PAD_LENGTH + 1] {
            is_invalid_pad(
                validate_pad_with(|r| r.payload.pads[1].padLength = pad_length),
                "padLength",
            );
        }
        is_invalid_pad(
            validate_pad_with(|r| {
                r.payload.pads[1].witness.decryptors =
                    vec![Address::repeat_byte(0x66); usize::from(MAX_PAD_DECRYPTORS) + 1];
            }),
            "too many decryptors",
        );
        is_invalid_pad(
            validate_pad_with(|r| {
                r.payload.pads[1].witness.expirationTimestamp = NOW;
                relabel(r);
            }),
            "expired",
        );
    }

    #[test]
    fn label_must_match_its_witness() {
        // A witness field changed after the label was derived.
        let mut request = valid_pad_request();
        request.payload.pads[1].witness.consumer = Address::repeat_byte(0x34);
        let label = request.payload.pads[1].label;
        assert_eq!(
            validate_pad_decryption(&request, &config(), NOW),
            Err(ValidationError::LabelMismatch { label })
        );
        // A shared field changed after the labels were derived.
        let mut request = valid_pad_request();
        request.payload.prfId = 1;
        let label = request.payload.pads[0].label;
        assert_eq!(
            validate_pad_decryption(&request, &config(), NOW),
            Err(ValidationError::LabelMismatch { label })
        );
    }

    #[test]
    fn solana_payload_gets_the_eip712_payload_checks() {
        let cfg = config();
        let solana_request = |start_timestamp, extra_data| {
            solana_user_request(vec![handle(1, EUINT64)], start_timestamp, extra_data)
        };
        let validate = |request: SolanaUserDecryptionRequest| {
            validate_solana_user_decryption(request.id(), &request, &cfg)
        };
        assert!(matches!(
            validate(solana_request(1_770_000_000, vec![0x03])),
            Err(ValidationError::InvalidExtraData(_))
        ));
        assert!(matches!(
            validate(solana_request(i64::MAX as u64 + 1, extra_data_v2())),
            Err(ValidationError::RequestValidityOutOfRange { .. })
        ));
    }

    #[test]
    fn an_evm_request_on_a_solana_host_is_malformed() {
        let solana_chain = zama_solana_acl::host_chain::solana_host_chain_id(12345);
        let cfg = Config {
            supported_chain_ids: vec![solana_chain],
            ..config()
        };
        let solana = handle(solana_chain, EUINT64);

        let error = validate_user_decryption(&user_request(vec![solana], extra_data_v2()), &cfg)
            .unwrap_err();
        assert_eq!(
            error,
            ValidationError::SolanaChainInEvmRequest(solana_chain)
        );
        assert_eq!(error.code(), ErrorCode::Malformed);

        let mut pad = pad_request(vec![pad_entry(solana_chain, 1)], extra_data_v2());
        pad.payload.chainId = solana_chain;
        assert_eq!(
            validate_pad_decryption(&pad, &cfg, NOW),
            Err(ValidationError::SolanaChainInEvmRequest(solana_chain))
        );
    }

    #[test]
    fn a_solana_request_over_the_handle_cap_is_malformed() {
        // Ebool handles keep the request within the bit budget, and the connector's own handle
        // limit is above the Solana cap, so the Solana cap refuses it.
        let solana_chain = zama_solana_acl::host_chain::solana_host_chain_id(12345);
        let over = MAX_REQUEST_HANDLES + 1;
        let cfg = Config {
            supported_chain_ids: vec![solana_chain],
            max_handles_per_decryption: NonZeroUsize::new(over).unwrap(),
            ..config()
        };
        let handles = vec![handle(solana_chain, EBOOL); over];
        let public = solana_public_request(handles.clone());
        let user = solana_user_request(handles, 1_770_000_000, extra_data_v2());
        let too_many = ValidationError::InvalidSolanaRequest(format!(
            "solana request names {over} handles, expected at most {MAX_REQUEST_HANDLES}"
        ));
        for error in [
            validate_solana_public_decryption(B256::ZERO, &public, &cfg).unwrap_err(),
            validate_solana_user_decryption(user.id(), &user, &cfg).unwrap_err(),
        ] {
            assert_eq!(error, too_many);
            assert_eq!(error.code(), ErrorCode::Malformed);
        }
    }

    #[test]
    fn bad_fhe_type() {
        let cfg = config();
        let bad = handle(1, BAD_FHE_TYPE);
        match validate_public_decryption(&public_request(vec![bad], extra_data_v2()), &cfg) {
            Err(ValidationError::InvalidHandle { handle, .. }) => assert_eq!(handle, bad),
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn mixed_chains() {
        let cfg = config();
        let handles = vec![handle(1, EUINT64), handle(2, EUINT64)];
        assert_eq!(
            validate_user_decryption(&user_request(handles, extra_data_v2()), &cfg),
            Err(ValidationError::MixedChainIds(1, 2))
        );
    }

    #[test]
    fn unsupported_chain() {
        let cfg = config();
        assert_eq!(
            validate_public_decryption(
                &public_request(vec![handle(9, EUINT64)], extra_data_v2()),
                &cfg
            ),
            Err(ValidationError::UnsupportedChainId(9))
        );
    }

    #[test]
    fn extra_data_accepts_known_versions_and_trailing_bytes() {
        let cfg = config();
        let handles = vec![handle(1, EUINT64)];
        for extra_data in [
            [vec![0x01], vec![0xaa; 32]].concat(),
            extra_data_v2(),
            [extra_data_v2(), vec![0xff; 7]].concat(),
        ] {
            assert_eq!(
                validate_public_decryption(&public_request(handles.clone(), extra_data), &cfg),
                Ok(())
            );
        }
    }

    #[test]
    fn extra_data_rejects_missing_unknown_version_and_truncation() {
        let cfg = config();
        let handles = vec![handle(1, EUINT64)];
        for extra_data in [
            vec![],
            vec![0x00],
            vec![0x03],
            vec![0x01; 10],
            vec![0x02; 64],
        ] {
            assert!(matches!(
                validate_public_decryption(&public_request(handles.clone(), extra_data), &cfg),
                Err(ValidationError::InvalidExtraData(_))
            ));
        }
    }
}
