//! SlotHashes sysvar sourcing for off-chain handle derivation.
//!
//! The zama-host program derives compute handles using `previous_bank_hash` —
//! the bank hash of the most recent slot below the executing slot, read from the
//! SlotHashes sysvar. To recompute handles off-chain (the reconstruction path),
//! the listener streams the SlotHashes sysvar account over Yellowstone and parses
//! the same layout here. Kept feature-independent so the gRPC transport can
//! source it without pulling the reconstruction (`solana-reconstruct`) deps.

/// Address of the SlotHashes sysvar account (subscribe to this over Yellowstone).
pub const SLOT_HASHES_SYSVAR: &str =
    "SysvarS1otHashes111111111111111111111111111";

/// Address of the Clock sysvar account. Handle derivation uses
/// `Clock.unix_timestamp`, which differs from the RPC `getBlockTime`/block-meta
/// value, so the reconstruction path must source it from this account's data.
pub const CLOCK_SYSVAR: &str = "SysvarC1ock11111111111111111111111111111111";

/// Parses `Clock.unix_timestamp` (i64) from the Clock sysvar account data.
/// Layout: slot u64, epoch_start_timestamp i64, epoch u64,
/// leader_schedule_epoch u64, then `unix_timestamp` i64 at offset 32.
pub fn clock_unix_timestamp(data: &[u8]) -> Option<i64> {
    Some(i64::from_le_bytes(data.get(32..40)?.try_into().ok()?))
}

/// Extracts the `previous_bank_hash` the program would read for a transaction at
/// `current_slot`, from a snapshot of the SlotHashes sysvar account data.
///
/// SlotHashes is laid out as `[u64 count][(u64 slot, [u8; 32] hash) * count]`,
/// ordered newest-first, where `hash` is the bank hash of that slot. The program
/// (`zama_host::state::previous_bank_hash`) returns the hash of the most recent
/// entry whose slot is below `current_slot`; this mirrors that scan off-chain.
/// Returns `None` if no prior slot exists or the buffer is short/malformed.
pub fn previous_bank_hash_from_slot_hashes(
    data: &[u8],
    current_slot: u64,
) -> Option<[u8; 32]> {
    const ENTRY_LEN: usize = 40; // u64 slot + 32-byte hash
    let count = u64::from_le_bytes(data.get(0..8)?.try_into().ok()?) as usize;
    for index in 0..count {
        let offset = 8 + index * ENTRY_LEN;
        let slot =
            u64::from_le_bytes(data.get(offset..offset + 8)?.try_into().ok()?);
        if slot < current_slot {
            let mut hash = [0u8; 32];
            hash.copy_from_slice(data.get(offset + 8..offset + ENTRY_LEN)?);
            return Some(hash);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn slot_hashes_buf(entries: &[(u64, [u8; 32])]) -> Vec<u8> {
        let mut buf = (entries.len() as u64).to_le_bytes().to_vec();
        for (slot, hash) in entries {
            buf.extend_from_slice(&slot.to_le_bytes());
            buf.extend_from_slice(hash);
        }
        buf
    }

    #[test]
    fn previous_bank_hash_picks_newest_prior_slot() {
        // Newest-first; slot 9 is skipped.
        let buf = slot_hashes_buf(&[
            (10, [10u8; 32]),
            (8, [8u8; 32]),
            (7, [7u8; 32]),
        ]);
        // tx at slot 11 -> newest prior is slot 10.
        assert_eq!(
            previous_bank_hash_from_slot_hashes(&buf, 11),
            Some([10u8; 32])
        );
        // tx at slot 10 -> 10 is not < 10, so newest prior is slot 8 (9 skipped).
        assert_eq!(
            previous_bank_hash_from_slot_hashes(&buf, 10),
            Some([8u8; 32])
        );
        // No slot below 7.
        assert_eq!(previous_bank_hash_from_slot_hashes(&buf, 7), None);
        // Short / empty buffers fail closed.
        assert_eq!(previous_bank_hash_from_slot_hashes(&[], 5), None);
        assert_eq!(previous_bank_hash_from_slot_hashes(&[0u8; 4], 5), None);
    }
}
