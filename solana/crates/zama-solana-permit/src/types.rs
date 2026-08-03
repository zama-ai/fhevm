//! Typed permit fields, their transport form, and the fixed protocol sizes.
//!
//! `PermitFields` is the validated type: it has no public constructor and no
//! public mutation, so the only way to obtain one is strict decoding of
//! `PermitWireFields` (see [`crate::validate`]). Everything downstream — the
//! canonical text, the envelope, verification — takes the validated type, which is
//! why "render a permit that was never validated" is not expressible.
//!
//! Public API surface: wallets and relayers. The transport conversions exist for the clients that
//! carry a permit over the wire, which is why some have no caller inside this repository.

use crate::error::PermitError;

/// Every permit identity is exactly this many bytes.
pub const IDENTITY_LEN: usize = 32;
/// The single accepted transport-key length: the ML-KEM-512 public key size. The
/// permit carries no variant field, so this length is what fixes the variant —
/// notably, a well-formed 1568-byte ML-KEM-1024 key is rejected.
pub const TRANSPORT_KEY_LEN: usize = 800;
/// Upper bound on the signed ACL-domain list. The empty list is also valid and
/// means permissive.
pub const MAX_ACL_DOMAIN_KEYS: usize = 10;
/// A permit must be valid for at least one second.
pub const MIN_DURATION_SECONDS: u64 = 1;
/// A permit must not be valid for more than 365 days.
pub const MAX_DURATION_SECONDS: u64 = 31_536_000;
/// Latest representable start, `9999-12-31T23:59:59Z`. Keeping the start below this
/// bound is what makes the timestamp rendering total, and what precludes a u64
/// overflow when the authorization layer adds the duration.
pub const MAX_START_TIMESTAMP: u64 = 253_402_300_799;
/// The only known version byte of the signed KMS routing field.
pub const KMS_ROUTING_VERSION_BYTE: u8 = 0x02;
/// Exact length of the signed KMS routing field at its only known version:
/// version byte plus two identities.
pub const KMS_ROUTING_EXTRA_DATA_LEN: usize = 1 + IDENTITY_LEN + IDENTITY_LEN;
/// Ed25519 signature length.
pub const SIGNATURE_LEN: usize = 64;

/// A 32-byte permit identity: a user, a program, an ACL domain, a KMS context or a
/// KMS epoch. Ordering is byte order, which is the ordering the ACL-domain list
/// rule is stated in.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Identity([u8; IDENTITY_LEN]);

impl Identity {
    /// Wraps 32 bytes. Total by construction: the width rule is enforced where a
    /// transport value of unknown width is decoded, not here.
    pub fn new(bytes: [u8; IDENTITY_LEN]) -> Self {
        Self(bytes)
    }

    /// The underlying bytes.
    pub fn as_bytes(&self) -> &[u8; IDENTITY_LEN] {
        &self.0
    }
}

/// A full transport key of the single accepted length.
#[derive(Clone, PartialEq, Eq)]
pub struct TransportKey(Box<[u8; TRANSPORT_KEY_LEN]>);

impl TransportKey {
    /// Wraps a full-length transport key.
    pub fn new(bytes: Box<[u8; TRANSPORT_KEY_LEN]>) -> Self {
        Self(bytes)
    }

    /// The full key. Consumers that need to commit to the key commit to all of it;
    /// the fingerprint is derived here, never accepted from outside.
    pub fn as_bytes(&self) -> &[u8; TRANSPORT_KEY_LEN] {
        &self.0
    }
}

// A transport key is 800 bytes of key material; printing it in a panic message is
// noise, so the debug form is elided rather than derived.
impl core::fmt::Debug for TransportKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("TransportKey(..)")
    }
}

/// The signed ACL-domain list, carrying its own validity: at most the maximum
/// count, strictly ascending in byte order, no duplicates. Empty means permissive.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AclDomainKeys(Vec<Identity>);

impl AclDomainKeys {
    /// Checks the count, the strict byte-order ascent and the absence of
    /// duplicates, and wraps the list. The only constructor.
    pub fn new(keys: Vec<Identity>) -> Result<Self, PermitError> {
        if keys.len() > MAX_ACL_DOMAIN_KEYS {
            return Err(PermitError::TooManyAclDomainKeys { count: keys.len() });
        }
        // Strict ascent between neighbors is the whole rule: a list in which every key
        // exceeds its predecessor cannot repeat a key anywhere, so there is nothing to
        // check across a distance. Equality is reported as a duplicate rather than as a
        // failed ascent, because that is the mistake a caller actually made.
        for index in 1..keys.len() {
            if keys[index] == keys[index - 1] {
                return Err(PermitError::DuplicateAclDomainKey { index });
            }
            if keys[index] < keys[index - 1] {
                return Err(PermitError::AclDomainKeysNotAscending { index });
            }
        }
        Ok(Self(keys))
    }

    /// The keys, in signed order.
    pub fn as_slice(&self) -> &[Identity] {
        &self.0
    }

    /// True for a permissive permit.
    pub fn is_permissive(&self) -> bool {
        self.0.is_empty()
    }
}

/// The signed KMS routing material, one variant per known version byte. New
/// versions are added as variants; consumers match exhaustively, so a new version
/// breaks the build instead of silently falling through.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum KmsRouting {
    /// Version `0x02`: a KMS context id and a KMS epoch id.
    ContextAndEpoch {
        /// KMS context the permit is routed to.
        kms_context_id: Identity,
        /// KMS epoch (share generation) the permit is routed to.
        kms_epoch_id: Identity,
    },
}

impl KmsRouting {
    /// Re-encodes the routing material back into its signed transport bytes.
    ///
    /// Exhaustive by construction: a new version has to be given its own encoding here,
    /// because there is no fallback arm to absorb it.
    pub fn to_extra_data(&self) -> Vec<u8> {
        match self {
            Self::ContextAndEpoch {
                kms_context_id,
                kms_epoch_id,
            } => {
                let mut out = Vec::with_capacity(KMS_ROUTING_EXTRA_DATA_LEN);
                out.push(KMS_ROUTING_VERSION_BYTE);
                out.extend_from_slice(kms_context_id.as_bytes());
                out.extend_from_slice(kms_epoch_id.as_bytes());
                out
            }
        }
    }
}

/// The eight signed permit fields, validated.
///
/// No public constructor and no public setters: the only way in is
/// [`PermitFields::decode`].
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PermitFields {
    user_pubkey: Identity,
    transport_key: TransportKey,
    allowed_acl_domain_keys: AclDomainKeys,
    start_timestamp: u64,
    duration_seconds: u64,
    verifying_program_id: Identity,
    chain_id: u64,
    extra_data: KmsRouting,
}

impl PermitFields {
    /// The signing wallet, also the sole envelope signer and the recipient.
    pub fn user_pubkey(&self) -> &Identity {
        &self.user_pubkey
    }

    /// The full transport key.
    pub fn transport_key(&self) -> &TransportKey {
        &self.transport_key
    }

    /// The signed ACL-domain scope; empty means permissive.
    pub fn allowed_acl_domain_keys(&self) -> &AclDomainKeys {
        &self.allowed_acl_domain_keys
    }

    /// Start of the validity window, unix seconds.
    pub fn start_timestamp(&self) -> u64 {
        self.start_timestamp
    }

    /// Length of the validity window, seconds.
    pub fn duration_seconds(&self) -> u64 {
        self.duration_seconds
    }

    /// Deployment: which program.
    pub fn verifying_program_id(&self) -> &Identity {
        &self.verifying_program_id
    }

    /// Deployment: which cluster.
    pub fn chain_id(&self) -> u64 {
        self.chain_id
    }

    /// Signed KMS routing material.
    pub fn extra_data(&self) -> &KmsRouting {
        &self.extra_data
    }

    /// Assembles validated fields. Crate-internal on purpose: the public way in is
    /// strict decoding.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_validated(
        user_pubkey: Identity,
        transport_key: TransportKey,
        allowed_acl_domain_keys: AclDomainKeys,
        start_timestamp: u64,
        duration_seconds: u64,
        verifying_program_id: Identity,
        chain_id: u64,
        extra_data: KmsRouting,
    ) -> Self {
        Self {
            user_pubkey,
            transport_key,
            allowed_acl_domain_keys,
            start_timestamp,
            duration_seconds,
            verifying_program_id,
            chain_id,
            extra_data,
        }
    }
}

/// The permit as it arrives over transport: widths and lengths are whatever the
/// sender chose, which is exactly why they are checked here and not assumed.
///
/// This is the shape every wire format (relayer JSON, event payload, normative
/// vectors) maps onto before decoding. Deliberately not `PermitFields`: the width
/// rules are unrepresentable once the typed form is reached.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct PermitWireFields {
    /// Claimed 32-byte user pubkey.
    pub user_pubkey: Vec<u8>,
    /// Claimed transport key of any length.
    pub transport_key: Vec<u8>,
    /// Claimed ACL-domain keys, in the sender's order.
    pub allowed_acl_domain_keys: Vec<Vec<u8>>,
    /// Claimed validity-window start.
    pub start_timestamp: u64,
    /// Claimed validity-window length.
    pub duration_seconds: u64,
    /// Claimed 32-byte verifying program id.
    pub verifying_program_id: Vec<u8>,
    /// Claimed chain id.
    pub chain_id: u64,
    /// Claimed versioned KMS routing bytes.
    pub extra_data: Vec<u8>,
}

/// An Ed25519 signature over the envelope.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Signature([u8; SIGNATURE_LEN]);

impl Signature {
    /// Wraps 64 signature bytes. Whether they verify is decided by verification,
    /// not here.
    pub fn new(bytes: [u8; SIGNATURE_LEN]) -> Self {
        Self(bytes)
    }

    /// The signature bytes.
    pub fn as_bytes(&self) -> &[u8; SIGNATURE_LEN] {
        &self.0
    }
}
