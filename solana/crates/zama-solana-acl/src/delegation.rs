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

use crate::account::{initialized, AccountView};
use crate::AclError;

/// Seed of the delegation record PDA: `[seed, delegator, delegate, program, scope]`.
pub const DELEGATION_SEED: &[u8] = b"user-decryption-delegation";

/// The application a wildcard row carries in both its `program` and its `scope` position, as
/// EVM's wildcard fills `contractAddress`. No encrypted store has this program: `0xff×32` decodes
/// to a curve point whose key no one holds, so no program can be deployed at it, and being on the
/// curve it is no PDA either. No store has it as scope: a store's scope must be an account its
/// program owns, and nothing lives at the sentinel. The host refuses a grant that sets the
/// sentinel in one position only.
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
    fn is_live_at(&self, unix_timestamp: u64) -> bool {
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

/// The account bytes of a record in this state, for test doubles of the host program.
pub fn encode_user_decryption_delegation(record: &UserDecryptionDelegationRecord) -> Vec<u8> {
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

/// Why a delegation row does not authorize.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeadRow {
    Absent,
    /// Expired at `expires_at`, or revoked when it is 0: a grant always ends after the time it
    /// was made.
    NotLive {
        expires_at: u64,
    },
}

impl core::fmt::Display for DeadRow {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Absent => f.write_str("absent"),
            Self::NotLive { expires_at: 0 } => f.write_str("revoked"),
            Self::NotLive { expires_at } => write!(f, "expired at {expires_at}"),
        }
    }
}

/// What one delegation row says at a given time.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RowVerdict {
    Live,
    Dead(DeadRow),
    /// A host-owned row the host program could not have written at this address.
    Invalid,
}

/// Judges the row read at the address `[delegator, delegate, program, scope]` and `bump` derive
/// under `host_program`, at Unix time `now`. The record must hold that tuple and bump: a reader
/// does not take the address as proof of what the record says.
pub fn judge_delegation_row(
    host_program: &[u8; 32],
    account: Option<AccountView<'_>>,
    bump: u8,
    [delegator, delegate, program, scope]: [&[u8; 32]; 4],
    now: u64,
) -> RowVerdict {
    let Some(account) = initialized(account) else {
        return RowVerdict::Dead(DeadRow::Absent);
    };
    if account.owner != host_program {
        return RowVerdict::Invalid;
    }
    let Ok(record) = decode_user_decryption_delegation(account.data) else {
        return RowVerdict::Invalid;
    };
    if !record.names(delegator, delegate, program, scope) || record.bump != bump {
        return RowVerdict::Invalid;
    }
    if record.is_live_at(now) {
        RowVerdict::Live
    } else {
        RowVerdict::Dead(DeadRow::NotLive {
            expires_at: record.expires_at,
        })
    }
}

/// One of the two rows that can authorize a delegated entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DelegationRow {
    /// The row of the encrypted store's application.
    Exact,
    /// The delegator's row for every application.
    Wildcard,
}

/// Whether a delegated entry is authorized.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DelegationVerdict {
    Authorized(DelegationRow),
    NoLiveDelegation {
        exact: DeadRow,
        wildcard: DeadRow,
    },
    /// The row fails the entry whatever the other row says.
    InvalidRow(DelegationRow),
}

/// Combines the application's row and the wildcard row, as EVM's wildcard delegation falls back.
/// Either live row authorizes, and a dead row cannot veto a live one, so narrowing a delegation
/// to fewer applications means revoking the wildcard row too. An invalid row fails the entry even
/// when the other is live.
pub fn judge_delegation(exact: RowVerdict, wildcard: RowVerdict) -> DelegationVerdict {
    match (exact, wildcard) {
        (RowVerdict::Invalid, _) => DelegationVerdict::InvalidRow(DelegationRow::Exact),
        (_, RowVerdict::Invalid) => DelegationVerdict::InvalidRow(DelegationRow::Wildcard),
        (RowVerdict::Live, _) => DelegationVerdict::Authorized(DelegationRow::Exact),
        (_, RowVerdict::Live) => DelegationVerdict::Authorized(DelegationRow::Wildcard),
        (RowVerdict::Dead(exact), RowVerdict::Dead(wildcard)) => {
            DelegationVerdict::NoLiveDelegation { exact, wildcard }
        }
    }
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
            decode_user_decryption_delegation(&encode_user_decryption_delegation(&record)),
            Ok(record)
        );
    }

    #[test]
    fn rejects_a_foreign_discriminator() {
        let mut data = encode_user_decryption_delegation(&record());
        data[0] ^= 0xff;
        assert_eq!(
            decode_user_decryption_delegation(&data),
            Err(AclError::BadDiscriminator)
        );
    }

    #[test]
    fn rejects_a_record_of_the_wrong_size() {
        let mut data = encode_user_decryption_delegation(&record());
        data.pop();
        assert_eq!(
            decode_user_decryption_delegation(&data),
            Err(AclError::BadAccountData)
        );

        let mut grown = encode_user_decryption_delegation(&record());
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

    const HOST: [u8; 32] = [0x77; 32];

    fn judge(owner: &[u8; 32], data: &[u8], bump: u8, now: u64) -> RowVerdict {
        let r = record();
        judge_delegation_row(
            &HOST,
            Some(AccountView { owner, data }),
            bump,
            [&r.delegator, &r.delegate, &r.program, &r.scope],
            now,
        )
    }

    #[test]
    fn a_row_is_live_dead_or_invalid() {
        let data = encode_user_decryption_delegation(&record());
        assert_eq!(judge(&HOST, &data, 254, 499), RowVerdict::Live);
        assert_eq!(
            judge(&HOST, &data, 254, 500),
            RowVerdict::Dead(DeadRow::NotLive { expires_at: 500 })
        );
        let r = record();
        let tuple = [&r.delegator, &r.delegate, &r.program, &r.scope];
        for absent in [
            None,
            Some(AccountView {
                owner: &[0; 32],
                data: &[],
            }),
        ] {
            assert_eq!(
                judge_delegation_row(&HOST, absent, 254, tuple, 0),
                RowVerdict::Dead(DeadRow::Absent)
            );
        }

        let mut other_tuple = record();
        other_tuple.scope = [0x45; 32];
        let invalid = [
            judge(&[0x78; 32], &data, 254, 0),
            judge(&HOST, &data[1..], 254, 0),
            judge(&HOST, &data, 253, 0),
            judge(
                &HOST,
                &encode_user_decryption_delegation(&other_tuple),
                254,
                0,
            ),
        ];
        assert_eq!(invalid, [RowVerdict::Invalid; 4]);
    }

    #[test]
    fn either_live_row_authorizes_and_an_invalid_row_vetoes() {
        use DelegationRow::{Exact, Wildcard};
        let absent = RowVerdict::Dead(DeadRow::Absent);
        let revoked = RowVerdict::Dead(DeadRow::NotLive { expires_at: 0 });
        let cases = [
            (
                (RowVerdict::Live, absent),
                DelegationVerdict::Authorized(Exact),
            ),
            (
                (revoked, RowVerdict::Live),
                DelegationVerdict::Authorized(Wildcard),
            ),
            (
                (RowVerdict::Live, RowVerdict::Invalid),
                DelegationVerdict::InvalidRow(Wildcard),
            ),
            (
                (RowVerdict::Invalid, RowVerdict::Live),
                DelegationVerdict::InvalidRow(Exact),
            ),
            (
                (absent, revoked),
                DelegationVerdict::NoLiveDelegation {
                    exact: DeadRow::Absent,
                    wildcard: DeadRow::NotLive { expires_at: 0 },
                },
            ),
        ];
        for ((exact, wildcard), expected) in cases {
            assert_eq!(judge_delegation(exact, wildcard), expected);
        }
        assert_eq!(DeadRow::NotLive { expires_at: 0 }.to_string(), "revoked");
        assert_eq!(
            DeadRow::NotLive { expires_at: 9 }.to_string(),
            "expired at 9"
        );
    }
}
