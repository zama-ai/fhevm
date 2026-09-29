//! Where the cleartext build keeps plaintexts inside host accounts.
//!
//! A value is a 32-byte entry: byte 0 is 1 once the value is recorded, bytes 16..32 hold it
//! big-endian. An entry nobody recorded reads as unknown, so a missing value fails loudly rather
//! than reading as zero. Every section starts with `MAGIC`; an account without it holds no
//! plaintexts because a production build wrote it.
//!
//! An `EncryptedStore` is created at the size its largest shape needs, so its Borsh encoding never
//! reaches the section that follows:
//!
//! ```text
//! MAGIC | one entry per slot | history count (u64 LE, one word) | history: (handle, entry) × 64
//! ```
//!
//! Slot entries hold the current value of each slot, which is all an execution reads. Slots are
//! only appended or replaced in place, so a slot keeps its index. The history holds the latest
//! results written to the store, slotted or not, because the store's ACL history keeps them
//! decryptable after a slot moves on or when no slot was written. It overwrites its oldest record
//! when full. Production decoders read the Borsh prefix and ignore trailing bytes
//! (`zama_solana_acl::validate_store`, and the SDK decoder, which accepts whole 32-byte words).
//!
//! A `TransientStore` is created at its production size (one System allocation), so the first
//! execution of a transaction appends `MAGIC` and one entry per result index.

use anchor_lang::prelude::*;
use zama_solana_acl::{MAX_MMR_PEAKS, MAX_STORE_SLOTS};

use super::{CleartextError, Value};
use crate::{TransientStore, MAX_TRANSIENT_RESULTS};

/// Heads every plaintext section, and marks the artifact itself as a cleartext build: deploy
/// tooling refuses a program binary that contains it.
pub const MAGIC: [u8; 32] = *b"zama-host cleartext build (test)";
const ENTRY_LEN: usize = 32;
const HISTORY_RECORD_LEN: usize = 32 + ENTRY_LEN;
/// Results an `EncryptedStore` keeps decryptable beyond its current slot values.
pub const STORE_HISTORY_LEN: usize = 64;
const RECORDED: u8 = 1;

/// Offset of an `EncryptedStore`'s plaintext section: past its largest Borsh encoding.
pub const STORE_SECTION_OFFSET: usize =
    zama_solana_acl::EncryptedStore::account_size(MAX_STORE_SLOTS, MAX_MMR_PEAKS);
const STORE_SLOTS_OFFSET: usize = STORE_SECTION_OFFSET + MAGIC.len();
const STORE_HISTORY_COUNT_OFFSET: usize = STORE_SLOTS_OFFSET + MAX_STORE_SLOTS * ENTRY_LEN;
const STORE_HISTORY_OFFSET: usize = STORE_HISTORY_COUNT_OFFSET + 32;
/// Size of an `EncryptedStore` account in the cleartext build.
pub const STORE_ACCOUNT_SIZE: usize = STORE_HISTORY_OFFSET + STORE_HISTORY_LEN * HISTORY_RECORD_LEN;

/// Offset of a `TransientStore`'s plaintext tail: its production size.
pub const TRANSIENT_TAIL_OFFSET: usize = TransientStore::SPACE;
const TRANSIENT_ENTRIES_OFFSET: usize = TRANSIENT_TAIL_OFFSET + MAGIC.len();
/// Size of a `TransientStore` once an execution has appended its tail.
pub const TRANSIENT_ACCOUNT_SIZE: usize =
    TRANSIENT_ENTRIES_OFFSET + MAX_TRANSIENT_RESULTS * ENTRY_LEN;

/// Writes the empty section of an `EncryptedStore` of [`STORE_ACCOUNT_SIZE`] bytes.
pub fn init_store_section(data: &mut [u8]) -> Result<()> {
    init_section(data, STORE_SECTION_OFFSET)
}

/// Writes the empty tail of a `TransientStore` grown to [`TRANSIENT_ACCOUNT_SIZE`].
pub fn init_transient_tail(data: &mut [u8]) -> Result<()> {
    init_section(data, TRANSIENT_TAIL_OFFSET)
}

/// Whether `data` carries an `EncryptedStore` plaintext section.
pub fn has_store_section(data: &[u8]) -> bool {
    has_section(data, STORE_SECTION_OFFSET)
}

/// The plaintext of slot `index` of an `EncryptedStore` whose slot holds `handle`.
pub fn store_value(data: &[u8], index: usize, handle: [u8; 32]) -> Result<Value> {
    require!(index < MAX_STORE_SLOTS, CleartextError::ValueUnknown);
    read_entry(
        data,
        STORE_SECTION_OFFSET,
        STORE_SLOTS_OFFSET + index * ENTRY_LEN,
        handle,
    )
}

pub fn set_store_value(data: &mut [u8], index: usize, value: Value) -> Result<()> {
    require!(index < MAX_STORE_SLOTS, CleartextError::ValueUnknown);
    write_entry(
        data,
        STORE_SECTION_OFFSET,
        STORE_SLOTS_OFFSET + index * ENTRY_LEN,
        value,
    )
}

/// The plaintext of `handle` among the latest results written to an `EncryptedStore`.
pub fn store_history_value(data: &[u8], handle: [u8; 32]) -> Result<Value> {
    let count = history_count(data)?;
    let kept = count.min(STORE_HISTORY_LEN as u64);
    for age in 0..kept {
        let offset = history_record_offset(count - 1 - age);
        if data[offset..offset + 32] == handle {
            return read_entry(data, STORE_SECTION_OFFSET, offset + 32, handle);
        }
    }
    err!(CleartextError::ValueUnknown)
}

/// Records a result written to an `EncryptedStore`, overwriting its oldest record when full.
pub fn record_store_history(data: &mut [u8], handle: [u8; 32], value: Value) -> Result<()> {
    let count = history_count(data)?;
    let offset = history_record_offset(count);
    data[offset..offset + 32].copy_from_slice(&handle);
    write_entry(data, STORE_SECTION_OFFSET, offset + 32, value)?;
    data[STORE_HISTORY_COUNT_OFFSET..STORE_HISTORY_COUNT_OFFSET + 8]
        .copy_from_slice(&(count + 1).to_le_bytes());
    Ok(())
}

/// The plaintext of transient result `index`, whose handle is `handle`.
pub fn transient_value(data: &[u8], index: usize, handle: [u8; 32]) -> Result<Value> {
    require!(index < MAX_TRANSIENT_RESULTS, CleartextError::ValueUnknown);
    read_entry(
        data,
        TRANSIENT_TAIL_OFFSET,
        TRANSIENT_ENTRIES_OFFSET + index * ENTRY_LEN,
        handle,
    )
}

pub fn set_transient_value(data: &mut [u8], index: usize, value: Value) -> Result<()> {
    require!(index < MAX_TRANSIENT_RESULTS, CleartextError::ValueUnknown);
    write_entry(
        data,
        TRANSIENT_TAIL_OFFSET,
        TRANSIENT_ENTRIES_OFFSET + index * ENTRY_LEN,
        value,
    )
}

fn init_section(data: &mut [u8], section: usize) -> Result<()> {
    data.get_mut(section..section + MAGIC.len())
        .ok_or_else(|| error!(CleartextError::ValueUnknown))?
        // Through `black_box`, so `MAGIC` stays contiguous bytes in the binary for deploy to find
        // rather than being folded into immediates.
        .copy_from_slice(core::hint::black_box(&MAGIC));
    Ok(())
}

fn has_section(data: &[u8], section: usize) -> bool {
    data.get(section..section + MAGIC.len()) == Some(&MAGIC[..])
}

fn history_count(data: &[u8]) -> Result<u64> {
    require!(
        has_store_section(data) && data.len() >= STORE_ACCOUNT_SIZE,
        CleartextError::ValueUnknown
    );
    Ok(u64::from_le_bytes(
        data[STORE_HISTORY_COUNT_OFFSET..STORE_HISTORY_COUNT_OFFSET + 8]
            .try_into()
            .unwrap(),
    ))
}

fn history_record_offset(sequence: u64) -> usize {
    STORE_HISTORY_OFFSET + (sequence % STORE_HISTORY_LEN as u64) as usize * HISTORY_RECORD_LEN
}

fn read_entry(data: &[u8], section: usize, entry: usize, handle: [u8; 32]) -> Result<Value> {
    require!(has_section(data, section), CleartextError::ValueUnknown);
    let entry = data
        .get(entry..entry + ENTRY_LEN)
        .ok_or_else(|| error!(CleartextError::ValueUnknown))?;
    require!(entry[0] == RECORDED, CleartextError::ValueUnknown);
    Value::new(
        crate::handle_fhe_type(handle),
        u128::from_be_bytes(entry[16..].try_into().unwrap()),
    )
}

fn write_entry(data: &mut [u8], section: usize, entry: usize, value: Value) -> Result<()> {
    require!(has_section(data, section), CleartextError::ValueUnknown);
    let entry = data
        .get_mut(entry..entry + ENTRY_LEN)
        .ok_or_else(|| error!(CleartextError::ValueUnknown))?;
    entry[0] = RECORDED;
    entry[16..].copy_from_slice(&value.bits.to_be_bytes());
    Ok(())
}

const _: () = assert!(STORE_ACCOUNT_SIZE <= 10_240);
// The SDK decoder accepts capacity past a store's Borsh encoding only in whole 32-byte words.
const _: () = assert!((STORE_ACCOUNT_SIZE - STORE_SECTION_OFFSET).is_multiple_of(32));

#[cfg(test)]
mod tests {
    use super::*;

    fn handle(index: u8) -> [u8; 32] {
        let mut handle = [index; 32];
        handle[30] = 5;
        handle
    }

    /// `sdk/js-sdk/src/solana/cleartext/storeValues.ts` reads stores at these offsets.
    #[test]
    fn the_sdk_reader_offsets_hold() {
        assert_eq!(
            (
                STORE_SECTION_OFFSET,
                STORE_HISTORY_COUNT_OFFSET,
                STORE_HISTORY_OFFSET,
                STORE_ACCOUNT_SIZE
            ),
            (4217, 5273, 5305, 9401)
        );
    }

    #[test]
    fn history_keeps_the_latest_results_and_forgets_older_ones_loudly() {
        let mut data = vec![0u8; STORE_ACCOUNT_SIZE];
        init_store_section(&mut data).unwrap();
        let value = |index: u8| Value::new(5, index.into()).unwrap();
        for index in 0..=STORE_HISTORY_LEN as u8 {
            record_store_history(&mut data, handle(index), value(index)).unwrap();
        }
        assert!(store_history_value(&data, handle(0)).is_err());
        for index in 1..=STORE_HISTORY_LEN as u8 {
            assert_eq!(
                store_history_value(&data, handle(index)).unwrap(),
                value(index)
            );
        }
    }

    #[test]
    fn a_store_without_a_section_holds_no_value() {
        let data = vec![0u8; STORE_ACCOUNT_SIZE];
        assert!(store_value(&data, 0, handle(1)).is_err());
        assert!(store_history_value(&data, handle(1)).is_err());
        let mut data = data;
        init_store_section(&mut data).unwrap();
        assert!(store_value(&data, 0, handle(1)).is_err());
    }
}
