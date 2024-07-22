use tfhe::prelude::FheDecrypt;

#[derive(Debug)]
pub enum CoprocessorError {
    DbError(sqlx::Error),
    Unauthorized,
    UnknownFheOperation(i32),
    UnknownFheType(i32),
    DuplicateOutputHandleInBatch(String),
    CiphertextHandleLongerThan64Bytes,
    InvalidHandle(String),
    UnexistingInputCiphertextsFound(Vec<String>),
    OutputHandleIsAlsoInputHandle(String),
    UnknownCiphertextType(i16),
}

impl std::fmt::Display for CoprocessorError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CoprocessorError::DbError(dbe) => {
                write!(f, "Coprocessor db error: {:?}", dbe)
            }
            CoprocessorError::Unauthorized => {
                write!(f, "API key unknown/invalid/not provided")
            }
            CoprocessorError::UnknownFheOperation(op) => {
                write!(f, "Unknown fhe operation: {}", op)
            }
            CoprocessorError::UnknownFheType(op) => {
                write!(f, "Unknown fhe type: {}", op)
            }
            CoprocessorError::DuplicateOutputHandleInBatch(op) => {
                write!(f, "Duplicate output handle in ciphertext batch: {}", op)
            }
            CoprocessorError::CiphertextHandleLongerThan64Bytes => {
                write!(f, "Found ciphertext handle longer than 64 bytes")
            }
            CoprocessorError::InvalidHandle(handle) => {
                write!(f, "Invalid handle found: {}", handle)
            }
            CoprocessorError::UnexistingInputCiphertextsFound(handles) => {
                write!(f, "Ciphertexts not found: {:?}", handles)
            }
            CoprocessorError::OutputHandleIsAlsoInputHandle(handle) => {
                write!(f, "Output handle is also on of the input handles: {}", handle)
            }
            CoprocessorError::UnknownCiphertextType(the_type) => {
                write!(f, "Unknown input ciphertext type: {}", the_type)
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

pub enum SupportedFheCiphertexts {
    FheBool(tfhe::FheBool),
    FheUint8(tfhe::FheUint8),
    FheUint16(tfhe::FheUint16),
    FheUint32(tfhe::FheUint32),
}

impl SupportedFheCiphertexts {
    pub fn serialize(&self) -> (i16, Vec<u8>) {
        match self {
            SupportedFheCiphertexts::FheBool(v) => (1, bincode::serialize(v).unwrap()),
            SupportedFheCiphertexts::FheUint8(v) => (2, bincode::serialize(v).unwrap()),
            SupportedFheCiphertexts::FheUint16(v) => (3, bincode::serialize(v).unwrap()),
            SupportedFheCiphertexts::FheUint32(v) => (4, bincode::serialize(v).unwrap()),
        }
    }

    pub fn decrypt(&self, client_key: &tfhe::ClientKey) -> String {
        match self {
            SupportedFheCiphertexts::FheBool(v) => v.decrypt(client_key).to_string(),
            SupportedFheCiphertexts::FheUint8(v) => FheDecrypt::<u8>::decrypt(v, client_key).to_string(),
            SupportedFheCiphertexts::FheUint16(v) => FheDecrypt::<u16>::decrypt(v, client_key).to_string(),
            SupportedFheCiphertexts::FheUint32(v) => FheDecrypt::<u32>::decrypt(v, client_key).to_string(),
        }
    }
}