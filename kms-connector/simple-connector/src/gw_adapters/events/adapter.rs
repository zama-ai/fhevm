use fhevm_gateway_rust_bindings::{decryption::Decryption, kmsmanagement::KmsManagement};

/// Events that can be processed by the KMS Core
#[derive(Clone, Debug)]
pub enum KmsCoreEvent {
    /// Public decryption request with block timestamp
    PublicDecryptionRequest(Decryption::PublicDecryptionRequest, u64),
    /// Public decryption response
    PublicDecryptionResponse(Decryption::PublicDecryptionResponse),
    /// User decryption request with block timestamp
    UserDecryptionRequest(Decryption::UserDecryptionRequest, u64),
    /// User decryption response
    UserDecryptionResponse(Decryption::UserDecryptionResponse),
    /// Preprocess keygen request
    PreprocessKeygenRequest(KmsManagement::PreprocessKeygenRequest),
    /// Preprocess keygen response
    PreprocessKeygenResponse(KmsManagement::PreprocessKeygenResponse),
    /// Preprocess kskgen request
    PreprocessKskgenRequest(KmsManagement::PreprocessKskgenRequest),
    /// Preprocess kskgen response
    PreprocessKskgenResponse(KmsManagement::PreprocessKskgenResponse),
    /// Keygen request
    KeygenRequest(KmsManagement::KeygenRequest),
    /// Keygen response
    KeygenResponse(KmsManagement::KeygenResponse),
    /// CRS generation request
    CrsgenRequest(KmsManagement::CrsgenRequest),
    /// CRS generation response
    CrsgenResponse(KmsManagement::CrsgenResponse),
    /// KSK generation request
    KskgenRequest(KmsManagement::KskgenRequest),
    /// KSK generation response
    KskgenResponse(KmsManagement::KskgenResponse),
}
