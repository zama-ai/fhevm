use std::sync::Arc;

use fhevm_engine_common::types::FhevmError;
use scheduler::dfg::types::SchedulerError;

#[derive(Debug)]
pub enum CoprocessorError {
    DbError(sqlx::Error),
    SchedulerError(SchedulerError),
    Unauthorized,
    FhevmError(FhevmError),
    DuplicateOutputHandleInBatch(String),
    CiphertextHandleLongerThan256Bytes,
    CiphertextHandleMustBeAtLeast1Byte(String),
    UnexistingInputCiphertextsFound(Vec<String>),
    AlreadyExistingResultHandlesFound(Vec<String>),
    OutputHandleIsAlsoInputHandle(String),
    CannotParseTenantEthereumAddress {
        bad_address: String,
        parsing_error: String,
    },
    CannotParseEthereumAddress {
        bad_address: String,
        parsing_error: String,
    },
    Eip712SigningFailure {
        error: String,
    },
    DuplicateResultHandleInInputsUploaded {
        hex_handle: String,
    },
    MoreThanMaximumCompactInputCiphertextsUploaded {
        input_count: usize,
        maximum_allowed: usize,
    },
    MoreThanMaximumCiphertextsAttemptedToDownload {
        input_count: usize,
        maximum_allowed: usize,
    },
    CompactInputCiphertextHasMoreCiphertextThanLimitAllows {
        input_blob_index: usize,
        input_ciphertexts_in_blob: usize,
        input_maximum_ciphertexts_allowed: usize,
    },
    ComputationInputIsUndefined {
        computation_output_handle: String,
        computation_inputs_index: usize,
    },
    TooManyCiphertextsInBatch {
        maximum_allowed: usize,
        got: usize,
    },
    CiphertextComputationDependencyLoopDetected {
        uncomputable_output_handle: String,
        uncomputable_handle_dependency: String,
    },
}

impl std::fmt::Display for CoprocessorError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::DbError(dbe) => {
                write!(f, "Coprocessor db error: {:?}", dbe)
            }
            Self::SchedulerError(se) => {
                write!(f, "Coprocessor scheduler error: {:?}", se)
            }
            Self::Unauthorized => {
                write!(f, "API key unknown/invalid/not provided")
            }
            Self::DuplicateOutputHandleInBatch(op) => {
                write!(f, "Duplicate output handle in ciphertext batch: {}", op)
            }
            Self::DuplicateResultHandleInInputsUploaded { hex_handle } => {
                write!(
                    f,
                    "Duplicate result handle in inputs detected: {hex_handle}"
                )
            }
            Self::MoreThanMaximumCompactInputCiphertextsUploaded {
                input_count,
                maximum_allowed,
            } => {
                write!(f, "More than maximum input blobs uploaded, maximum allowed: {maximum_allowed}, uploaded: {input_count}")
            }
            Self::MoreThanMaximumCiphertextsAttemptedToDownload {
                input_count,
                maximum_allowed,
            } => {
                write!(f, "Requested more than maximum ciphertexts allowed to download, maximum allowed: {maximum_allowed}, requested: {input_count}")
            }
            Self::CompactInputCiphertextHasMoreCiphertextThanLimitAllows {
                input_blob_index,
                input_ciphertexts_in_blob,
                input_maximum_ciphertexts_allowed,
            } => {
                write!(f, "Input blob contains too many ciphertexts, input blob index: {input_blob_index}, ciphertexts in blob: {input_ciphertexts_in_blob}, maximum ciphertexts in blob allowed: {input_maximum_ciphertexts_allowed}")
            }
            Self::CiphertextHandleLongerThan256Bytes => {
                write!(f, "Found ciphertext handle longer than 256 bytes")
            }
            Self::CiphertextHandleMustBeAtLeast1Byte(handle) => {
                write!(f, "Found ciphertext handle is empty: {handle}")
            }
            Self::UnexistingInputCiphertextsFound(handles) => {
                write!(f, "Ciphertexts not found: {:?}", handles)
            }
            Self::AlreadyExistingResultHandlesFound(e) => {
                write!(f, "Handles not found in the database: {:?}", e)
            }
            Self::OutputHandleIsAlsoInputHandle(handle) => {
                write!(
                    f,
                    "Output handle is also on of the input handles: {}",
                    handle
                )
            }
            Self::CannotParseTenantEthereumAddress {
                bad_address,
                parsing_error,
            } => {
                write!(
                    f,
                    "Cannot parse tenant ethereum verifying contract address: {}, error: {}",
                    bad_address, parsing_error,
                )
            }
            Self::CannotParseEthereumAddress {
                bad_address,
                parsing_error,
            } => {
                write!(
                    f,
                    "Cannot parse ethereum address: {}, error: {}",
                    bad_address, parsing_error,
                )
            }
            Self::Eip712SigningFailure { error } => {
                write!(f, "Error when signing EIP712 hash: {}", error,)
            }
            Self::CiphertextComputationDependencyLoopDetected {
                uncomputable_output_handle,
                uncomputable_handle_dependency,
            } => {
                write!(f, "fhe computation with output handle {uncomputable_output_handle} with dependency {:?} has circular dependency and is uncomputable", uncomputable_handle_dependency)
            }
            Self::TooManyCiphertextsInBatch {
                maximum_allowed,
                got,
            } => {
                write!(
                    f,
                    "maximum ciphertexts exceeded in batch, maximum: {maximum_allowed}, got: {got}"
                )
            }
            Self::ComputationInputIsUndefined {
                computation_output_handle,
                computation_inputs_index,
            } => {
                write!(f, "computation has undefined input, output handle: {computation_output_handle}, input index: {computation_inputs_index}")
            }
            Self::FhevmError(e) => {
                write!(f, "fhevm error: {:?}", e)
            }
        }
    }
}

impl std::error::Error for CoprocessorError {}

impl From<sqlx::Error> for CoprocessorError {
    fn from(err: sqlx::Error) -> Self {
        CoprocessorError::DbError(err)
    }
}

impl From<SchedulerError> for CoprocessorError {
    fn from(err: SchedulerError) -> Self {
        CoprocessorError::SchedulerError(err)
    }
}

impl From<CoprocessorError> for tonic::Status {
    fn from(err: CoprocessorError) -> Self {
        tonic::Status::from_error(Box::new(err))
    }
}

pub struct TfheTenantKeys {
    pub tenant_id: i32,
    pub chain_id: i32,
    pub verifying_contract_address: String,
    pub acl_contract_address: String,
    pub sks: tfhe::ServerKey,
    #[cfg(feature = "gpu")]
    pub csks: tfhe::CompressedServerKey,
    #[cfg(feature = "gpu")]
    pub gpu_sks: tfhe::CudaServerKey,

    // only used in tests, that's why we put dead_code
    #[allow(dead_code)]
    pub pks: tfhe::CompactPublicKey,
    #[allow(dead_code)]
    pub public_params: Arc<tfhe::zk::CompactPkeCrs>,
}
