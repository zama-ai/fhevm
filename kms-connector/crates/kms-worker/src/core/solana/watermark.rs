//! The permit validity window and the per-user invalidation watermark. Together they pin a usable
//! permit's start into `[last revocation, now]`.

use super::SolanaPubkeyBytes;
use super::failure::InvalidHostRecord;
use super::snapshot::ObservedRow;
use zama_solana_acl::decode_permit_invalidation;

/// Reads the watermark of the request signer. A user who never revoked has no record, which
/// reads as zero.
///
/// An empty System-owned account also reads as zero: anyone can fund the derivable address, and
/// reading that as a refusal would let one transfer deny all of a user's requests. Anything else
/// at the address must be the host's record for this user, or reading zero from a foreign layout
/// would resurrect revoked permits.
pub fn read_watermark(
    row: &ObservedRow,
    program_id: SolanaPubkeyBytes,
    user: SolanaPubkeyBytes,
) -> Result<u64, WatermarkFailure> {
    let Some(account) = row
        .account
        .as_ref()
        .filter(|account| !account.is_uninitialized_pda())
    else {
        return Ok(0);
    };
    let invalid = InvalidHostRecord {
        account_key: row.key,
    };
    if account.owner != program_id {
        return Err(invalid.into());
    }
    let record = decode_permit_invalidation(&account.data).map_err(|_| invalid)?;
    if record.user != user || record.bump != row.bump {
        return Err(invalid.into());
    }
    Ok(record.invalidation_watermark)
}

/// A permit that started before its signer's last revocation is dead. A permit signed in the same
/// second as the revocation survives, so a user who revokes and re-signs is not locked out.
pub fn check_not_invalidated(start_timestamp: u64, watermark: u64) -> Result<(), WatermarkFailure> {
    if start_timestamp < watermark {
        return Err(WatermarkFailure::Invalidated {
            start_timestamp,
            watermark,
        });
    }
    Ok(())
}

/// The closed window `[start, start + duration]` against the Connector's clock: the bounds the
/// Gateway applies on chain, so the two never disagree on a boundary second.
pub fn check_window(
    start_timestamp: u64,
    duration_seconds: u64,
    now_unix_seconds: u64,
) -> Result<(), WindowFailure> {
    if start_timestamp > now_unix_seconds {
        return Err(WindowFailure::NotYetValid {
            start_timestamp,
            now: now_unix_seconds,
        });
    }
    // Both operands are capped by the permit's typed rules; saturating can only close the window.
    let end = start_timestamp.saturating_add(duration_seconds);
    if now_unix_seconds > end {
        return Err(WindowFailure::Expired {
            end,
            now: now_unix_seconds,
        });
    }
    Ok(())
}

#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum WatermarkFailure {
    #[error("permit start {start_timestamp} is below the invalidation watermark {watermark}")]
    Invalidated {
        start_timestamp: u64,
        watermark: u64,
    },
    #[error(transparent)]
    InvalidHostRecord(#[from] InvalidHostRecord),
}

#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum WindowFailure {
    #[error("permit starts at {start_timestamp}, later than now {now}")]
    NotYetValid { start_timestamp: u64, now: u64 },
    #[error("permit expired at {end}, now {now}")]
    Expired { end: u64, now: u64 },
}
