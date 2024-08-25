use fhevm_engine_common::types::FhevmError;

#[derive(Debug)]
pub enum CoprocessorError {
    DbError(sqlx::Error),
    Unauthorized,
    FhevmError(FhevmError),
    DuplicateOutputHandleInBatch(String),
    CiphertextHandleLongerThan64Bytes,
    CiphertextHandleMustBeAtLeast1Byte(String),
    UnexistingInputCiphertextsFound(Vec<String>),
    AlreadyExistingResultHandlesFound(Vec<String>),
    OutputHandleIsAlsoInputHandle(String),
    DuplicateResultHandleInInputsUploaded {
        hex_handle: String,
    },
    MoreThanMaximumCompactInputCiphertextsUploaded {
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
            Self::Unauthorized => {
                write!(f, "API key unknown/invalid/not provided")
            }
            Self::DuplicateOutputHandleInBatch(op) => {
                write!(f, "Duplicate output handle in ciphertext batch: {}", op)
            }
            Self::DuplicateResultHandleInInputsUploaded { hex_handle } => {
                write!(f, "Duplicate result handle in inputs detected: {hex_handle}")
            }
            Self::MoreThanMaximumCompactInputCiphertextsUploaded { input_count, maximum_allowed } => {
                write!(f, "More than maximum input blobs uploaded, maximum allowed: {maximum_allowed}, uploaded: {input_count}")
            }
            Self::CompactInputCiphertextHasMoreCiphertextThanLimitAllows { input_blob_index, input_ciphertexts_in_blob, input_maximum_ciphertexts_allowed  } => {
                write!(f, "Input blob contains mismatching amount of ciphertexts, input blob index: {input_blob_index}, ciphertexts in blob: {input_ciphertexts_in_blob}, maximum ciphertexts in blob allowed: {input_maximum_ciphertexts_allowed}")
            }
            Self::CiphertextHandleLongerThan64Bytes => {
                write!(f, "Found ciphertext handle longer than 64 bytes")
            }
            Self::CiphertextHandleMustBeAtLeast1Byte(handle) => {
                write!(f, "Found ciphertext handle less than 4 bytes: {handle}")
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

impl From<CoprocessorError> for tonic::Status {
    fn from(err: CoprocessorError) -> Self {
        tonic::Status::from_error(Box::new(err))
    }
}

pub struct TfheTenantKeys {
    pub tenant_id: i32,
    pub sks: tfhe::ServerKey,
    // maybe we'll need this
    #[allow(dead_code)]
    pub pks: tfhe::CompactPublicKey,
}
