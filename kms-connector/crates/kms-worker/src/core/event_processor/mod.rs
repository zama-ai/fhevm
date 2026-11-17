mod acl;
pub mod ciphertext;
mod context;
mod decryption;
mod error;
mod kms;
mod kms_client;
mod pad;
mod processor;
mod protocol_config;
mod rpc;

pub use acl::AclCheck;
pub use ciphertext::CiphertextManager;
pub use context::{ContextManager, DbContextManager};
pub use decryption::DecryptionProcessor;
pub use error::{ProcessingError, ProcessingErrorKind, RequestCheckError, RequestCheckKind};
pub use kms::KMSGenerationProcessor;
pub(crate) use kms_client::kms_core_endpoint;
pub use kms_client::{KmsClient, KmsPollTarget};
pub use processor::{DbEventProcessor, EventProcessor};
pub use protocol_config::{ProtocolConfigProcessor, compute_anchor_event_hash};
pub use rpc::HostRpcClient;

use kms_grpc::kms::v1::SigningSchemeType;

/// The signature schemes requested from the KMS Core.
///
/// Currently hardcoded to Ecdsa256k1 as it is the only scheme used on-chain.
const SIGNING_SCHEMES: [i32; 1] = [SigningSchemeType::Ecdsa256k1 as i32];
