//! Strict decoding of the transport form into typed fields.
//!
//! Everything checkable without live state is checked here, before any text is
//! rendered and before any signature is looked at: identity widths, the ACL-domain
//! count and ordering, the validity-window bounds, the transport-key length, and
//! the version and length of the KMS routing field. A permit that survives this
//! step renders totally — the renderer is not allowed to fail.
//!
//! Rules are applied in the order the fields are signed in, which is also the order
//! they are declared in. A permit breaking several rules is rejected by whichever
//! comes first in that order; nothing normative rides on that choice — every
//! normative vector carries exactly one violation — but the order is fixed rather
//! than incidental, so a diagnostic reproduces.

use crate::{
    error::{IdentityField, PermitError},
    types::{
        AclDomainKeys, Identity, KmsRouting, PermitFields, PermitWireFields, TransportKey,
        IDENTITY_LEN, KMS_ROUTING_EXTRA_DATA_LEN, KMS_ROUTING_VERSION_BYTE, MAX_DURATION_SECONDS,
        MAX_START_TIMESTAMP, MIN_DURATION_SECONDS, TRANSPORT_KEY_LEN,
    },
};

impl PermitFields {
    /// Decodes the transport form, rejecting anything that violates the typed form.
    pub fn decode(wire: &PermitWireFields) -> Result<Self, PermitError> {
        let user_pubkey = decode_identity(&wire.user_pubkey, IdentityField::UserPubkey)?;

        // The conversion is the length rule: a transport key of any other length has no
        // typed form to land in.
        let transport_key: Box<[u8; TRANSPORT_KEY_LEN]> = wire
            .transport_key
            .clone()
            .into_boxed_slice()
            .try_into()
            .map_err(|bytes: Box<[u8]>| PermitError::TransportKeyLength { len: bytes.len() })?;
        let transport_key = TransportKey::new(transport_key);

        // Widths first, by index, so a malformed entry is named by its position; the
        // count and the ordering are then the list type's own rules.
        let mut keys = Vec::with_capacity(wire.allowed_acl_domain_keys.len());
        for (index, key) in wire.allowed_acl_domain_keys.iter().enumerate() {
            keys.push(decode_identity(key, IdentityField::AclDomainKey(index))?);
        }
        let allowed_acl_domain_keys = AclDomainKeys::new(keys)?;

        let start_timestamp = wire.start_timestamp;
        if start_timestamp > MAX_START_TIMESTAMP {
            return Err(PermitError::StartTimestampOutOfRange { start_timestamp });
        }

        let duration_seconds = wire.duration_seconds;
        if !(MIN_DURATION_SECONDS..=MAX_DURATION_SECONDS).contains(&duration_seconds) {
            return Err(PermitError::DurationOutOfRange { duration_seconds });
        }

        let verifying_program_id = decode_identity(
            &wire.verifying_program_id,
            IdentityField::VerifyingProgramId,
        )?;

        let extra_data = decode_kms_routing(&wire.extra_data)?;

        Ok(Self::from_validated(
            user_pubkey,
            transport_key,
            allowed_acl_domain_keys,
            start_timestamp,
            duration_seconds,
            verifying_program_id,
            wire.chain_id,
            extra_data,
        ))
    }
}

/// Decodes one identity field, naming it if the width is wrong.
fn decode_identity(bytes: &[u8], field: IdentityField) -> Result<Identity, PermitError> {
    <[u8; IDENTITY_LEN]>::try_from(bytes)
        .map(Identity::new)
        .map_err(|_| PermitError::IdentityWidth {
            field,
            len: bytes.len(),
        })
}

/// Parses the signed KMS routing field.
///
/// The version byte decides the length, and the length is exact: a field long enough to
/// *contain* the routing material with room to spare would be a second encoding of the
/// same routing, and two encodings of one meaning is what makes implementations diverge.
/// An unknown version is rejected here rather than carried forward, which is what keeps
/// rendering total — the renderer only ever sees versions it can render.
fn decode_kms_routing(bytes: &[u8]) -> Result<KmsRouting, PermitError> {
    let Some((&version, _)) = bytes.split_first() else {
        return Err(PermitError::UnknownKmsRoutingVersion { version: None });
    };

    match version {
        KMS_ROUTING_VERSION_BYTE => {
            let bytes: [u8; KMS_ROUTING_EXTRA_DATA_LEN] =
                bytes
                    .try_into()
                    .map_err(|_| PermitError::KmsRoutingLength {
                        version,
                        len: bytes.len(),
                    })?;

            let mut kms_context_id = [0u8; IDENTITY_LEN];
            let mut kms_epoch_id = [0u8; IDENTITY_LEN];
            kms_context_id.copy_from_slice(&bytes[1..1 + IDENTITY_LEN]);
            kms_epoch_id.copy_from_slice(&bytes[1 + IDENTITY_LEN..]);

            Ok(KmsRouting::ContextAndEpoch {
                kms_context_id: Identity::new(kms_context_id),
                kms_epoch_id: Identity::new(kms_epoch_id),
            })
        }
        // Not a wildcard over an enum: the routing versions this protocol knows are a set
        // of byte values, and every value outside it is this one rejection. The typed
        // routing form *is* an enum, and every match on it is exhaustive.
        version => Err(PermitError::UnknownKmsRoutingVersion {
            version: Some(version),
        }),
    }
}
