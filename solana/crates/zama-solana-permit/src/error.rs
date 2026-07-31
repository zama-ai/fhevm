//! Rejection reasons.
//!
//! One variant per distinct rule violation, so a caller (and a normative vector)
//! can assert that a bad permit failed for the reason it was built to fail for,
//! rather than merely failing somehow.

use core::fmt;

/// Why a permit was rejected.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PermitError {
    /// An identity field was not exactly 32 bytes.
    IdentityWidth {
        /// Which field carried the wrong width.
        field: IdentityField,
        /// The width that arrived.
        len: usize,
    },
    /// More ACL-domain keys than the protocol allows.
    TooManyAclDomainKeys {
        /// The count that arrived.
        count: usize,
    },
    /// ACL-domain keys are not strictly ascending in byte order.
    AclDomainKeysNotAscending {
        /// Index of the key that does not exceed its predecessor.
        index: usize,
    },
    /// The same ACL-domain key appears twice.
    DuplicateAclDomainKey {
        /// Index of the repeated key.
        index: usize,
    },
    /// The validity window is zero-length or longer than a year.
    DurationOutOfRange {
        /// The duration that arrived.
        duration_seconds: u64,
    },
    /// The start is beyond the latest representable timestamp.
    StartTimestampOutOfRange {
        /// The start that arrived.
        start_timestamp: u64,
    },
    /// The transport key is not the single accepted length.
    TransportKeyLength {
        /// The length that arrived.
        len: usize,
    },
    /// The KMS routing field carries a version byte this implementation does not
    /// know, or is empty.
    UnknownKmsRoutingVersion {
        /// The version byte that arrived, absent for an empty field.
        version: Option<u8>,
    },
    /// The KMS routing field has a length that does not match its version.
    KmsRoutingLength {
        /// The version byte that arrived.
        version: u8,
        /// The length that arrived.
        len: usize,
    },
    /// The signature does not verify over the locally reconstructed envelope.
    SignatureMismatch,
    /// The user pubkey is not a usable Ed25519 verifying key.
    UnusableUserPubkey,
}

/// Which identity field a width violation was found in.
///
/// The KMS context and epoch ids are absent on purpose: they live inside the KMS
/// routing field, whose exact length is checked as a whole, so their widths cannot
/// be wrong independently. A variant for them would be unreachable.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IdentityField {
    /// The signing user.
    UserPubkey,
    /// The deployment's program id.
    VerifyingProgramId,
    /// An entry of the ACL-domain list, at this index.
    AclDomainKey(usize),
}

impl fmt::Display for PermitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IdentityWidth { field, len } => {
                write!(f, "identity {field:?} is {len} bytes, expected 32")
            }
            Self::TooManyAclDomainKeys { count } => {
                write!(f, "{count} ACL domain keys exceeds the permitted maximum")
            }
            Self::AclDomainKeysNotAscending { index } => {
                write!(f, "ACL domain key at index {index} is not above its predecessor in byte order")
            }
            Self::DuplicateAclDomainKey { index } => {
                write!(f, "ACL domain key at index {index} is a duplicate")
            }
            Self::DurationOutOfRange { duration_seconds } => {
                write!(f, "duration {duration_seconds} is outside the permitted range")
            }
            Self::StartTimestampOutOfRange { start_timestamp } => {
                write!(f, "start timestamp {start_timestamp} is beyond the latest representable time")
            }
            Self::TransportKeyLength { len } => {
                write!(f, "transport key is {len} bytes, expected the ML-KEM-512 public key size")
            }
            Self::UnknownKmsRoutingVersion { version } => match version {
                Some(version) => write!(f, "unknown KMS routing version byte {version:#04x}"),
                None => write!(f, "empty KMS routing field carries no version byte"),
            },
            Self::KmsRoutingLength { version, len } => write!(
                f,
                "KMS routing field of version {version:#04x} is {len} bytes, which is not its length"
            ),
            Self::SignatureMismatch => f.write_str("signature does not verify over the reconstructed envelope"),
            Self::UnusableUserPubkey => f.write_str("user pubkey is not a usable Ed25519 verifying key"),
        }
    }
}

impl std::error::Error for PermitError {}
