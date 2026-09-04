//! Cheap, stateless request checks (no RPC, no DB). Any failure maps to `400 malformed`.

use crate::core::Config;
use alloy::primitives::B256;
use connector_utils::types::{
    extra_data::parse_extra_data,
    handle::{extract_chain_id_from_handle, extract_fhe_type_from_handle},
};
use kms_connector_api::{PublicDecryptionRequest, RequestValidity, UserDecryptionRequest};
use tfhe::FheTypes;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ValidationError {
    #[error("no handle provided")]
    NoHandles,
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
    #[error("requestValidity.{field} is too high: {value}, max is {}", i64::MAX)]
    RequestValidityOutOfRange { field: &'static str, value: u64 },
}

pub fn validate_public_decryption(
    request: &PublicDecryptionRequest,
    config: &Config,
) -> Result<(), ValidationError> {
    validate_handles(&request.ctHandles, config)?;
    validate_extra_data(&request.extraData)
}

pub fn validate_user_decryption(
    request: &UserDecryptionRequest,
    config: &Config,
) -> Result<(), ValidationError> {
    validate_handles(request.handles.iter().map(|h| &h.handle), config)?;
    if request.allowedContracts.len() > config.max_allowed_contracts {
        return Err(ValidationError::TooManyAllowedContracts(
            request.allowedContracts.len(),
            config.max_allowed_contracts,
        ));
    }
    validate_request_validity(&request.requestValidity)?;
    validate_extra_data(&request.extraData)
}

fn validate_request_validity(validity: &RequestValidity) -> Result<(), ValidationError> {
    for (field, value) in [
        ("startTimestamp", validity.startTimestamp),
        ("durationSeconds", validity.durationSeconds),
    ] {
        if i64::try_from(value).is_err() {
            return Err(ValidationError::RequestValidityOutOfRange { field, value });
        }
    }
    Ok(())
}

/// Checks the handle list: non-empty, well-formed handles of a decryptable FHE type, on a single
/// supported chain, within the total bit size budget.
fn validate_handles<'a>(
    handles: impl IntoIterator<Item = &'a B256>,
    config: &Config,
) -> Result<u64, ValidationError> {
    let (mut chain_id, mut total_bits) = (None, 0u64);
    for handle in handles {
        let fhe_type = extract_fhe_type_from_handle(handle.as_slice()).map_err(|e| {
            ValidationError::InvalidHandle {
                handle: *handle,
                reason: e.to_string(),
            }
        })?;
        let bits = fhe_type_size(*handle, fhe_type)?;
        total_bits += u64::from(bits);

        let handle_chain_id =
            extract_chain_id_from_handle(*handle).map_err(|e| ValidationError::InvalidHandle {
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
    if total_bits > config.max_decryption_request_bits {
        return Err(ValidationError::BitSizeExceeded(
            total_bits,
            config.max_decryption_request_bits,
        ));
    }
    if !config.supported_chain_ids.contains(&chain_id) {
        return Err(ValidationError::UnsupportedChainId(chain_id));
    }
    Ok(chain_id)
}

fn fhe_type_size(handle: B256, fhe_type: FheTypes) -> Result<u16, ValidationError> {
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
                handle,
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use alloy::primitives::{Address, Bytes};
    use kms_connector_api::HandleEntry;

    const EBOOL: u8 = FheTypes::Bool as u8;
    const EUINT4: u8 = FheTypes::Uint4 as u8;
    const EUINT64: u8 = FheTypes::Uint64 as u8;
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
            max_decryption_request_bits: 128,
            max_allowed_contracts: 1,
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
            signature: Bytes::from(vec![0x66; 65]),
            extraData: Bytes::from(extra_data),
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
            validate_user_decryption(&user_request(handles, vec![]), &cfg),
            Ok(())
        );
    }

    #[test]
    fn empty_handles() {
        let cfg = config();
        assert_eq!(
            validate_public_decryption(&public_request(vec![], vec![]), &cfg),
            Err(ValidationError::NoHandles)
        );
        assert_eq!(
            validate_user_decryption(&user_request(vec![], vec![]), &cfg),
            Err(ValidationError::NoHandles)
        );
    }

    #[test]
    fn bit_size_budget() {
        let cfg = config();
        // 64 + 64 + 2 = 130 > 128, but 64 ebools = 128 fit.
        let handles = vec![handle(1, EUINT64), handle(1, EUINT64), handle(1, EBOOL)];
        assert_eq!(
            validate_public_decryption(&public_request(handles, vec![]), &cfg),
            Err(ValidationError::BitSizeExceeded(130, 128))
        );
        let handles = vec![handle(1, EBOOL); 64];
        assert_eq!(
            validate_user_decryption(&user_request(handles, vec![]), &cfg),
            Ok(())
        );
    }

    #[test]
    fn non_decryptable_fhe_type() {
        let cfg = config();
        let bad = handle(1, EUINT4);
        match validate_public_decryption(&public_request(vec![bad], vec![]), &cfg) {
            Err(ValidationError::InvalidHandle { handle, reason }) => {
                assert_eq!(handle, bad);
                assert!(reason.contains("cannot be decrypted"), "{reason}");
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn too_many_allowed_contracts() {
        let cfg = config();
        let mut request = user_request(vec![handle(1, EUINT64)], vec![]);
        request.allowedContracts.push(Address::repeat_byte(0x77));
        assert_eq!(
            validate_user_decryption(&request, &cfg),
            Err(ValidationError::TooManyAllowedContracts(2, 1))
        );
    }

    #[test]
    fn request_validity_out_of_range() {
        let cfg = config();
        let mut request = user_request(vec![handle(1, EUINT64)], vec![]);
        request.requestValidity.startTimestamp = i64::MAX as u64;
        assert_eq!(validate_user_decryption(&request, &cfg), Ok(()));

        request.requestValidity.startTimestamp = i64::MAX as u64 + 1;
        assert_eq!(
            validate_user_decryption(&request, &cfg),
            Err(ValidationError::RequestValidityOutOfRange {
                field: "startTimestamp",
                value: i64::MAX as u64 + 1,
            })
        );

        request.requestValidity.startTimestamp = 1_770_000_000;
        request.requestValidity.durationSeconds = u64::MAX;
        assert_eq!(
            validate_user_decryption(&request, &cfg),
            Err(ValidationError::RequestValidityOutOfRange {
                field: "durationSeconds",
                value: u64::MAX,
            })
        );
    }

    #[test]
    fn bad_fhe_type() {
        let cfg = config();
        let bad = handle(1, BAD_FHE_TYPE);
        match validate_public_decryption(&public_request(vec![bad], vec![]), &cfg) {
            Err(ValidationError::InvalidHandle { handle, .. }) => assert_eq!(handle, bad),
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn mixed_chains() {
        let cfg = config();
        let handles = vec![handle(1, EUINT64), handle(2, EUINT64)];
        assert_eq!(
            validate_user_decryption(&user_request(handles, vec![]), &cfg),
            Err(ValidationError::MixedChainIds(1, 2))
        );
    }

    #[test]
    fn unsupported_chain() {
        let cfg = config();
        assert_eq!(
            validate_public_decryption(&public_request(vec![handle(9, EUINT64)], vec![]), &cfg),
            Err(ValidationError::UnsupportedChainId(9))
        );
    }

    #[test]
    fn extra_data_accepts_known_versions_and_trailing_bytes() {
        let cfg = config();
        let handles = vec![handle(1, EUINT64)];
        for extra_data in [
            vec![],
            vec![0x00],
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
    fn extra_data_rejects_unknown_version_and_truncation() {
        let cfg = config();
        let handles = vec![handle(1, EUINT64)];
        for extra_data in [vec![0x03], vec![0x01; 10], vec![0x02; 64]] {
            assert!(matches!(
                validate_public_decryption(&public_request(handles.clone(), extra_data), &cfg),
                Err(ValidationError::InvalidExtraData(_))
            ));
        }
    }
}
