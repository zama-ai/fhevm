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
    OutputHandleIsAlsoInputHandle(String),
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
            Self::CiphertextHandleLongerThan64Bytes => {
                write!(f, "Found ciphertext handle longer than 64 bytes")
            }
            Self::CiphertextHandleMustBeAtLeast1Byte(handle) => {
                write!(f, "Found ciphertext handle less than 4 bytes: {handle}")
            }
            Self::UnexistingInputCiphertextsFound(handles) => {
                write!(f, "Ciphertexts not found: {:?}", handles)
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
    pub sks: tfhe::ServerKey,
    // maybe we'll need this
    #[allow(dead_code)]
    pub pks: tfhe::CompactPublicKey,
}
