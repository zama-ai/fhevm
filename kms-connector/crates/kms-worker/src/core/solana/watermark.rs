//! The permit-invalidation watermark, and the validity window.
//!
//! Two rules share this module because they are the two halves of one property: the start of
//! any usable permit is pinned into `[last revocation, now]` at the moment of evaluation. The
//! window bounds it from above, the watermark from below.
//!
//! The watermark is host state, not a permit field, which is what makes one revocation
//! transaction kill every outstanding permit of its signer at once. Its key is the request
//! signer — the delegate, in a delegated flow: the delegate's lever kills the delegate's
//! permits, and the delegator's lever is the delegation record.
//!
//! A missing record reads as zero. There is no migration and no "not yet initialised" state:
//! a user who never revoked has revoked at time zero.

use super::snapshot::{HostSnapshot, SnapshotError};
use crate::core::solana_acl::{SolanaPubkeyBytes, anchor_account_discriminator};
use solana_pubkey::Pubkey;

/// PDA seed of the per-user invalidation record.
pub const PERMIT_INVALIDATION_SEED: &[u8] = b"permit-invalidation";

/// On-chain account name the record's discriminator is the hash of.
const PERMIT_INVALIDATION_ACCOUNT: &str = "PermitInvalidation";
/// Length of the leading account discriminator.
const DISCRIMINATOR_LEN: usize = 8;
/// The record's body: the user it belongs to, the watermark, the PDA bump.
const INVALIDATION_BODY_LEN: usize = 32 + 8 + 1;
/// Where the `user` field sits.
const USER_RANGE: std::ops::Range<usize> = DISCRIMINATOR_LEN..DISCRIMINATOR_LEN + 32;
/// Where the watermark sits.
const WATERMARK_RANGE: std::ops::Range<usize> = USER_RANGE.end..USER_RANGE.end + 8;
/// Where the PDA bump sits.
const BUMP_INDEX: usize = WATERMARK_RANGE.end;

/// The canonical invalidation-record address for a user under this deployment.
pub fn permit_invalidation_address(
    program_id: SolanaPubkeyBytes,
    user: SolanaPubkeyBytes,
) -> (SolanaPubkeyBytes, u8) {
    let (address, bump) = Pubkey::find_program_address(
        &[PERMIT_INVALIDATION_SEED, user.as_ref()],
        &Pubkey::new_from_array(program_id),
    );
    (address.to_bytes(), bump)
}

/// Reads the watermark of `user` from the snapshot; absent record reads as `0`.
///
/// "Absent" covers two observations, because a third party decides which one a never-revoked user
/// presents. The address is derivable by anyone, so anyone can transfer lamports to it and leave a
/// System-program-owned empty account where there was nothing at all. Both say the same thing —
/// the host program has never written a watermark here — and reading only the first as zero would
/// hand any sender a terminal denial of every future request of any user, for the price of one
/// transfer. The host program treats the two the same way when it reads this record.
///
/// The record's contents are checked against its address rather than trusted: the discriminator
/// must be the invalidation record's, and the `user` field must be the user whose address was
/// derived. An account that is host-owned but is something else is a rejection, not a zero —
/// reading zero from a foreign layout would silently resurrect every revoked permit. That includes
/// a System-owned account that does carry data: the donation case is the empty one, and anything
/// else at this address is a layout this function is not reading.
pub fn read_watermark(
    snapshot: &HostSnapshot,
    program_id: SolanaPubkeyBytes,
    user: SolanaPubkeyBytes,
) -> Result<u64, WatermarkFailure> {
    let (account_key, canonical_bump) = permit_invalidation_address(program_id, user);

    let Some(account) = snapshot.account(&account_key)? else {
        // A user who never revoked has revoked at time zero: there is no migration and no
        // "not yet initialised" state to distinguish.
        return Ok(0);
    };

    // A pre-funded address is the same "never written" observation as no account at all, and it is
    // not the reader's choice which of the two a never-revoked user is presented as.
    if account.is_uninitialized_pda() {
        return Ok(0);
    }

    if account.owner != program_id {
        return Err(WatermarkFailure::ForeignOwner {
            account_key,
            owner: account.owner,
            expected: program_id,
        });
    }

    // Exact length, unlike a lineage: this record is fixed-size and is never realloc-grown, so
    // a tail would mean the layout is not the one being read here rather than an account that
    // has held more than it holds now.
    let data = &account.data;
    let not_a_record = || WatermarkFailure::NotAnInvalidationRecord { account_key };
    if data.len() != DISCRIMINATOR_LEN + INVALIDATION_BODY_LEN
        || data.get(..DISCRIMINATOR_LEN)
            != Some(&anchor_account_discriminator(PERMIT_INVALIDATION_ACCOUNT)[..])
    {
        return Err(not_a_record());
    }

    // The contents are checked against the address they were read from. Trusting the address
    // alone would let one user's revocation kill another's permits if the program ever wrote a
    // record to the wrong PDA.
    let named_user: SolanaPubkeyBytes = data
        .get(USER_RANGE)
        .and_then(|bytes| SolanaPubkeyBytes::try_from(bytes).ok())
        .ok_or_else(not_a_record)?;
    if named_user != user {
        return Err(WatermarkFailure::RecordNamesAnotherUser { account_key });
    }

    // The stored bump has to be the canonical one for this address. Not an attacker check — these
    // bytes can only be written by the owning program, and the address was derived here — but a
    // record whose bump is not the one this derivation produces is not the record this reader
    // reads, and reading a watermark out of it would be reading a layout nobody promised.
    if data.get(BUMP_INDEX) != Some(&canonical_bump) {
        return Err(not_a_record());
    }

    let watermark = data
        .get(WATERMARK_RANGE)
        .and_then(|bytes| <[u8; 8]>::try_from(bytes).ok())
        .map(u64::from_le_bytes)
        .ok_or_else(not_a_record)?;
    Ok(watermark)
}

/// The invalidation rule: a permit that started before its signer's last revocation is dead.
pub fn check_not_invalidated(start_timestamp: u64, watermark: u64) -> Result<(), WatermarkFailure> {
    // The boundary is inclusive: a permit signed in the same second as the revocation survives,
    // or a user who revokes and immediately re-signs would be locked out for that second.
    if start_timestamp < watermark {
        return Err(WatermarkFailure::Invalidated {
            start_timestamp,
            watermark,
        });
    }
    Ok(())
}

/// The validity window `[start, start + duration]`, evaluated against the Connector's own wall
/// clock.
///
/// This is the second of two lines: the Gateway checks the same window against its chain's
/// block timestamp when the request is submitted. This check is self-sufficient and also
/// covers a permit that expires between Gateway acceptance and event processing. Both bounds are
/// inclusive on both sides of that pair, so the two lines agree second for second rather than
/// leaving one second where only the Connector rejects.
pub fn check_window(
    start_timestamp: u64,
    duration_seconds: u64,
    now_unix_seconds: u64,
) -> Result<(), WindowFailure> {
    // Open at the start second: the rule rejects a start later than now, not one equal to it, so
    // a permit is usable the second it names.
    if start_timestamp > now_unix_seconds {
        return Err(WindowFailure::NotYetValid {
            start_timestamp,
            now: now_unix_seconds,
        });
    }
    // Open at the end second too: the window is the closed interval `[start, start + duration]`,
    // which is the boundary the Gateway applies on chain (`start + duration < now` rejects, so the
    // end second is still inside). One rule, five implementations — a Connector that closed a
    // second early would reject a request the Gateway accepted and charged for.
    //
    // Saturating rather than checked because both operands are capped by the permit's own typed
    // rules, and a saturated end can only close the window earlier — never open it wider, which is
    // the direction that would matter.
    let end = start_timestamp.saturating_add(duration_seconds);
    if now_unix_seconds > end {
        return Err(WindowFailure::Expired {
            end,
            now: now_unix_seconds,
        });
    }
    Ok(())
}

/// Why the invalidation rule rejected a permit, or could not be evaluated.
#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum WatermarkFailure {
    /// The permit started before the signer's last revocation.
    #[error("permit start {start_timestamp} is below the invalidation watermark {watermark}")]
    Invalidated {
        /// The permit's signed start.
        start_timestamp: u64,
        /// The watermark observed in the snapshot.
        watermark: u64,
    },
    /// The invalidation address holds an account that is not an invalidation record: wrong length,
    /// wrong discriminator, or a bump other than the canonical one for its address.
    #[error("account {account_key:?} is not a decodable invalidation record")]
    NotAnInvalidationRecord {
        /// The address that was read.
        account_key: SolanaPubkeyBytes,
    },
    /// The record names a different user than the address it lives at.
    #[error("invalidation record {account_key:?} names another user")]
    RecordNamesAnotherUser {
        /// The address that was read.
        account_key: SolanaPubkeyBytes,
    },
    /// The record exists but belongs to another program.
    #[error("invalidation record {account_key:?} is owned by {owner:?}, expected {expected:?}")]
    ForeignOwner {
        /// The address that was read.
        account_key: SolanaPubkeyBytes,
        /// Who owns it.
        owner: SolanaPubkeyBytes,
        /// The deployment's program id.
        expected: SolanaPubkeyBytes,
    },
    /// The snapshot was asked for an account it never read.
    #[error(transparent)]
    Snapshot(#[from] SnapshotError),
}

/// Why the validity window rejected a permit.
#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum WindowFailure {
    /// The window has not opened yet. Without this rule the duration cap is bypassed by
    /// pushing the start forward, and the watermark by pushing it past a future increment.
    #[error("permit starts at {start_timestamp}, later than now {now}")]
    NotYetValid {
        /// The permit's signed start.
        start_timestamp: u64,
        /// The evaluation time.
        now: u64,
    },
    /// The window has closed: `now` is past the end second, which is itself still inside.
    #[error("permit expired at {end}, now {now}")]
    Expired {
        /// End of the window.
        end: u64,
        /// The evaluation time.
        now: u64,
    },
}
