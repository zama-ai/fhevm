//! The user-decryption delegation record: layout, decoder, and the liveness rule.
//!
//! One byte-level implementation for every off-chain reader of the record — the KMS connector's
//! authoritative check and the relayer's advisory pre-check decode the same bytes through this
//! module, so the two cannot drift on the layout or on what "live" means. What deliberately does
//! NOT live here is PDA derivation: it needs `find_program_address` (an off-curve check), and
//! this crate stays free of solana-version-specific dependencies so the on-chain programs
//! (solana 3.x) and the connector (solana 2.x) can share it. Each consumer derives addresses
//! with its own solana-pubkey, all from the one `DELEGATION_SEED` below: the host program
//! imports the seed and the wildcard sentinel from this crate rather than restating them.
//!
//! The layout mirrors `zama-host`'s `UserDecryptionDelegation` (a fixed 130-byte account:
//! 8-byte Anchor discriminator + 122-byte body) and is pinned against the program's own
//! serializer by the host's `shared_crate_decoder_reads_what_the_program_serializes` state
//! test, which feeds `try_serialize` output of a distinct-valued record through this decoder
//! field by field; the runtime-test SDK fixtures additionally pin the seed order and the
//! record bytes as literals.

use crate::AclError;

/// Seed of the delegation record PDA: `[seed, delegator, delegate, authority]`.
pub const DELEGATION_SEED: &[u8] = b"user-decryption-delegation";

/// The sentinel a wildcard row carries in place of an encrypted value account authority.
/// No real authority can collide with it: an encrypted value account authority must sign
/// `fhe_execute`, and the sentinel has no key — and the connector independently refuses any
/// encrypted value account naming it (its resolution guard), so an account carrying the
/// sentinel never reaches a row read.
pub const WILDCARD_ENCRYPTED_VALUE_ACCOUNT_AUTHORITY: [u8; 32] = [0xff; 32];

const ANCHOR_DISCRIMINATOR_LEN: usize = 8;
const BODY_LEN: usize = 32 + 32 + 32 + 8 + 8 + 8 + 1 + 1;

/// One decoded delegation record, fields exactly as the host program wrote them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserDecryptionDelegationRecord {
    pub delegator: [u8; 32],
    pub delegate: [u8; 32],
    pub encrypted_value_account_authority: [u8; 32],
    /// The last slot the delegation is live at, inclusive. Zeroed by a revocation.
    pub expiration_slot: u64,
    /// Strictly monotonic across grants, re-grants and revocations. Authorizes nothing.
    pub delegation_counter: u64,
    /// The slot the record last changed in; a record mutates at most once per slot.
    pub last_update_slot: u64,
    /// Whether the delegator revoked it. A re-grant reinstates.
    pub revoked: bool,
    /// The record PDA's bump.
    pub bump: u8,
}

impl UserDecryptionDelegationRecord {
    /// Whether the record authorizes at `slot`: not revoked, and the expiration slot has not
    /// passed — the expiration slot itself is inside the life. This is one half of the
    /// connector's freshness rule; the snapshot-consistency half (`last_update_slot` against
    /// the observed slot) belongs to the reader's observation discipline, not to the record.
    pub fn is_live_at(&self, slot: u64) -> bool {
        !self.revoked && self.expiration_slot >= slot
    }
}

/// The eight bytes every reader matches before trusting the body:
/// `sha256("account:UserDecryptionDelegation")[..8]`, pinned as a literal so a renamed account
/// fails here rather than in a consumer that hand-rolled the hash.
pub const USER_DECRYPTION_DELEGATION_DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_LEN] =
    [0x25, 0x05, 0x8b, 0x21, 0x49, 0x35, 0x01, 0xf8];

/// Decodes an account's raw data, discriminator included, into a delegation record.
///
/// Strict on both ends: the account is exactly discriminator + body (the record is never
/// realloc-grown, unlike the encrypted value account), and every field decodes or the whole
/// account is refused.
pub fn decode_user_decryption_delegation(
    data: &[u8],
) -> Result<UserDecryptionDelegationRecord, AclError> {
    if data.len() != ANCHOR_DISCRIMINATOR_LEN + BODY_LEN {
        return Err(AclError::BadAccountData);
    }
    if data[..ANCHOR_DISCRIMINATOR_LEN] != USER_DECRYPTION_DELEGATION_DISCRIMINATOR {
        return Err(AclError::BadDiscriminator);
    }
    let body = &data[ANCHOR_DISCRIMINATOR_LEN..];
    let revoked = match body[120] {
        0 => false,
        1 => true,
        _ => return Err(AclError::BadAccountData),
    };
    Ok(UserDecryptionDelegationRecord {
        delegator: bytes32(body, 0),
        delegate: bytes32(body, 32),
        encrypted_value_account_authority: bytes32(body, 64),
        expiration_slot: u64_le(body, 96),
        delegation_counter: u64_le(body, 104),
        last_update_slot: u64_le(body, 112),
        revoked,
        bump: body[121],
    })
}

fn bytes32(body: &[u8], offset: usize) -> [u8; 32] {
    let mut out = [0; 32];
    out.copy_from_slice(&body[offset..offset + 32]);
    out
}

fn u64_le(body: &[u8], offset: usize) -> u64 {
    let mut out = [0; 8];
    out.copy_from_slice(&body[offset..offset + 8]);
    u64::from_le_bytes(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record() -> UserDecryptionDelegationRecord {
        UserDecryptionDelegationRecord {
            delegator: [0x11; 32],
            delegate: [0x22; 32],
            encrypted_value_account_authority: [0x33; 32],
            expiration_slot: 500,
            delegation_counter: 7,
            last_update_slot: 400,
            revoked: false,
            bump: 254,
        }
    }

    fn encode(record: &UserDecryptionDelegationRecord) -> Vec<u8> {
        let mut data = USER_DECRYPTION_DELEGATION_DISCRIMINATOR.to_vec();
        data.extend_from_slice(&record.delegator);
        data.extend_from_slice(&record.delegate);
        data.extend_from_slice(&record.encrypted_value_account_authority);
        data.extend_from_slice(&record.expiration_slot.to_le_bytes());
        data.extend_from_slice(&record.delegation_counter.to_le_bytes());
        data.extend_from_slice(&record.last_update_slot.to_le_bytes());
        data.push(record.revoked as u8);
        data.push(record.bump);
        data
    }

    /// The discriminator literal is the hash it claims to be. Both sides pinned: the literal is
    /// what foreign implementations compare against, the preimage says where it comes from.
    #[test]
    fn discriminator_is_the_hash_of_the_account_name() {
        let digest = crate::sha256(&[b"account:UserDecryptionDelegation"]);
        assert_eq!(
            USER_DECRYPTION_DELEGATION_DISCRIMINATOR,
            digest[..ANCHOR_DISCRIMINATOR_LEN],
        );
    }

    #[test]
    fn decodes_the_exact_layout_the_program_writes() {
        let record = record();
        assert_eq!(
            decode_user_decryption_delegation(&encode(&record)),
            Ok(record)
        );
    }

    #[test]
    fn rejects_a_foreign_discriminator() {
        let mut data = encode(&record());
        data[0] ^= 0xff;
        assert_eq!(
            decode_user_decryption_delegation(&data),
            Err(AclError::BadDiscriminator)
        );
    }

    #[test]
    fn rejects_a_record_of_the_wrong_size() {
        let mut data = encode(&record());
        data.pop();
        assert_eq!(
            decode_user_decryption_delegation(&data),
            Err(AclError::BadAccountData)
        );

        let mut grown = encode(&record());
        grown.push(0);
        assert_eq!(
            decode_user_decryption_delegation(&grown),
            Err(AclError::BadAccountData)
        );
    }

    #[test]
    fn rejects_a_boolean_that_is_neither() {
        let mut data = encode(&record());
        let revoked_offset = data.len() - 2;
        data[revoked_offset] = 2;
        assert_eq!(
            decode_user_decryption_delegation(&data),
            Err(AclError::BadAccountData)
        );
    }

    /// The expiration slot is inclusive, and revocation wins over any expiration.
    #[test]
    fn liveness_matches_the_connector_boundary() {
        let live = record();
        assert!(live.is_live_at(500));
        assert!(!live.is_live_at(501));

        let mut revoked = record();
        revoked.revoked = true;
        assert!(!revoked.is_live_at(100));
    }
}
