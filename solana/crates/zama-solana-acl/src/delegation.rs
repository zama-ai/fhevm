//! The user-decryption delegation record: layout, decoder, and the liveness rule.
//!
//! One byte-level implementation for every off-chain reader of the record — the KMS connector's
//! authoritative check and the relayer's advisory pre-check decode the same bytes through this
//! module, so the two cannot drift on the layout or on what "live" means. What deliberately does
//! NOT live here is PDA derivation: it needs `find_program_address` (an off-curve check), and
//! this crate stays free of solana-version-specific dependencies so the on-chain programs and
//! the off-chain readers can share it whatever Solana version each builds. Each consumer derives
//! addresses with its own solana-pubkey from the one seed list, [`delegation_seeds`].
//!
//! The layout mirrors `zama-host`'s `UserDecryptionDelegation` (a fixed 161-byte account:
//! 8-byte Anchor discriminator + 153-byte body) and is pinned against the program's own
//! serializer by the host's `shared_crate_decoder_reads_what_the_program_serializes` state
//! test, which feeds `try_serialize` output of a distinct-valued record through this decoder
//! field by field; the runtime-test SDK fixtures additionally pin the seed order and the
//! record bytes as literals.

use crate::AclError;

/// Seed of the delegation record PDA: `[seed, delegator, delegate, program, scope]`.
pub const DELEGATION_SEED: &[u8] = b"user-decryption-delegation";

/// The application a wildcard row carries in both its `program` and its `scope` position, as
/// EVM's wildcard fills `contractAddress`. No encrypted store has this program: `0xff×32` decodes
/// to a curve point whose key no one holds, so no program can be deployed at it, and being on the
/// curve it is no PDA either. A store may still pick `scope = 0xff×32`; the host refuses a grant
/// that sets the sentinel in one position only, so such a store is reached by the wildcard row
/// alone.
pub const WILDCARD_APP: [u8; 32] = [0xff; 32];

/// The PDA seeds of a delegation row, bump excluded: the one spelling every side derives from.
pub fn delegation_seeds<'a>(
    delegator: &'a [u8; 32],
    delegate: &'a [u8; 32],
    program: &'a [u8; 32],
    scope: &'a [u8; 32],
) -> [&'a [u8]; 5] {
    [DELEGATION_SEED, delegator, delegate, program, scope]
}

const ANCHOR_DISCRIMINATOR_LEN: usize = 8;
const BODY_LEN: usize = 32 + 32 + 32 + 32 + 8 + 8 + 8 + 1;

/// One decoded delegation record, fields exactly as the host program wrote them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserDecryptionDelegationRecord {
    pub delegator: [u8; 32],
    pub delegate: [u8; 32],
    /// The application's program, or [`WILDCARD_APP`].
    pub program: [u8; 32],
    /// The application's scope, or [`WILDCARD_APP`].
    pub scope: [u8; 32],
    /// Unix second the delegation ends at, exclusive. Zeroed by a revocation.
    pub expires_at: u64,
    /// Strictly monotonic across grants, re-grants and revocations. Authorizes nothing.
    pub delegation_counter: u64,
    /// The slot the record last changed in; a record mutates at most once per slot.
    pub last_update_slot: u64,
    /// The record PDA's bump.
    pub bump: u8,
}

impl UserDecryptionDelegationRecord {
    /// Whether the record authorizes at `unix_timestamp`: live while `expires_at` is still ahead,
    /// as EVM's `expirationDate > block.timestamp`. A revoked record holds 0, so it reads like
    /// one never granted.
    pub fn is_live_at(&self, unix_timestamp: u64) -> bool {
        self.expires_at > unix_timestamp
    }

    /// Whether the record holds this tuple. Its address derives from the same fields, but a reader
    /// does not take the address as proof of what the record says.
    pub fn names(
        &self,
        delegator: &[u8; 32],
        delegate: &[u8; 32],
        program: &[u8; 32],
        scope: &[u8; 32],
    ) -> bool {
        self.delegator == *delegator
            && self.delegate == *delegate
            && self.program == *program
            && self.scope == *scope
    }
}

/// The eight bytes every reader matches before trusting the body:
/// `sha256("account:UserDecryptionDelegation")[..8]`, pinned as a literal so a renamed account
/// fails here rather than in a consumer that hand-rolled the hash.
pub const USER_DECRYPTION_DELEGATION_DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_LEN] =
    [0x25, 0x05, 0x8b, 0x21, 0x49, 0x35, 0x01, 0xf8];

/// Decodes an account's raw data, discriminator included, into a delegation record.
///
/// Strict: the account is exactly discriminator + body (the record is never realloc-grown,
/// unlike the encrypted store).
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
    Ok(UserDecryptionDelegationRecord {
        delegator: bytes32(body, 0),
        delegate: bytes32(body, 32),
        program: bytes32(body, 64),
        scope: bytes32(body, 96),
        expires_at: u64_le(body, 128),
        delegation_counter: u64_le(body, 136),
        last_update_slot: u64_le(body, 144),
        bump: body[152],
    })
}

pub(crate) fn bytes32(body: &[u8], offset: usize) -> [u8; 32] {
    let mut out = [0; 32];
    out.copy_from_slice(&body[offset..offset + 32]);
    out
}

pub(crate) fn u64_le(body: &[u8], offset: usize) -> u64 {
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
            program: [0x33; 32],
            scope: [0x44; 32],
            expires_at: 500,
            delegation_counter: 7,
            last_update_slot: 400,
            bump: 254,
        }
    }

    fn encode(record: &UserDecryptionDelegationRecord) -> Vec<u8> {
        let mut data = USER_DECRYPTION_DELEGATION_DISCRIMINATOR.to_vec();
        data.extend_from_slice(&record.delegator);
        data.extend_from_slice(&record.delegate);
        data.extend_from_slice(&record.program);
        data.extend_from_slice(&record.scope);
        data.extend_from_slice(&record.expires_at.to_le_bytes());
        data.extend_from_slice(&record.delegation_counter.to_le_bytes());
        data.extend_from_slice(&record.last_update_slot.to_le_bytes());
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

    /// The bound is exclusive, as on EVM, and a revoked record (0) is dead at every time.
    #[test]
    fn liveness_ends_at_expires_at() {
        let live = record();
        assert!(live.is_live_at(499));
        assert!(!live.is_live_at(500));

        let mut revoked = record();
        revoked.expires_at = 0;
        assert!(!revoked.is_live_at(0));
    }
}
