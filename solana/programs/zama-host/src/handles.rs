use anchor_lang::prelude::*;
use solana_sha256_hasher::hashv;
use solana_sysvar::slot_hashes::PodSlotHashes;

use crate::{FheBinaryOpCode, ZamaHostError};

pub(crate) const COMPUTATION_DOMAIN_SEPARATOR: &[u8] = b"FHE_comp";
pub(crate) const COMPUTED_HANDLE_MARKER: u8 = 0xff;
pub(crate) const HANDLE_VERSION: u8 = 0;

pub fn computed_binary_handle(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
) -> [u8; 32] {
    let op_byte = [op.as_u8()];
    let scalar_byte = [u8::from(scalar)];
    let chain_id_bytes = chain_id.to_be_bytes();
    let timestamp_bytes = unix_timestamp.to_be_bytes();
    let mut result = hashv(&[
        COMPUTATION_DOMAIN_SEPARATOR,
        &op_byte,
        &lhs,
        &rhs,
        &scalar_byte,
        crate::ID.as_ref(),
        &chain_id_bytes,
        &previous_bank_hash,
        &timestamp_bytes,
    ])
    .to_bytes();

    result[21..32].fill(0);
    result[21] = COMPUTED_HANDLE_MARKER;
    result[22..30].copy_from_slice(&chain_id_bytes);
    result[30] = fhe_type;
    result[31] = HANDLE_VERSION;
    result
}

pub fn computed_trivial_handle(
    plaintext: [u8; 32],
    fhe_type: u8,
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
) -> [u8; 32] {
    let chain_id_bytes = chain_id.to_be_bytes();
    let timestamp_bytes = unix_timestamp.to_be_bytes();
    let fhe_type_bytes = [fhe_type];
    let mut result = hashv(&[
        COMPUTATION_DOMAIN_SEPARATOR,
        &[2],
        &plaintext,
        &fhe_type_bytes,
        crate::ID.as_ref(),
        &chain_id_bytes,
        &previous_bank_hash,
        &timestamp_bytes,
    ])
    .to_bytes();

    result[21..32].fill(0);
    result[21] = COMPUTED_HANDLE_MARKER;
    result[22..30].copy_from_slice(&chain_id_bytes);
    result[30] = fhe_type;
    result[31] = HANDLE_VERSION;
    result
}

pub fn previous_bank_hash(current_slot: u64) -> Result<[u8; 32]> {
    let Some(previous_slot) = current_slot.checked_sub(1) else {
        return err!(ZamaHostError::PreviousBankHashUnavailable);
    };
    let slot_hashes = PodSlotHashes::fetch()
        .map_err(|_| error!(ZamaHostError::PreviousBankHashUnavailable))?;
    let hash = slot_hashes
        .get(&previous_slot)
        .ok()
        .flatten()
        .ok_or_else(|| error!(ZamaHostError::PreviousBankHashUnavailable))?;
    Ok(hash.to_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FheBinaryOpCode;

    #[test]
    fn computed_handles_are_deterministic() {
        let lhs = [1_u8; 32];
        let rhs = [2_u8; 32];
        let bank_hash = [0xAB_u8; 32];
        let first = computed_binary_handle(
            FheBinaryOpCode::Add,
            lhs,
            rhs,
            false,
            5,
            12345,
            bank_hash,
            1_700_000_000,
        );
        let second = computed_binary_handle(
            FheBinaryOpCode::Add,
            lhs,
            rhs,
            false,
            5,
            12345,
            bank_hash,
            1_700_000_000,
        );
        assert_eq!(first, second);
        assert_eq!(first[21], COMPUTED_HANDLE_MARKER);
        assert_eq!(first[30], 5);
    }

    #[test]
    fn computed_handles_differ_for_zero_and_fixture_bank_hash() {
        const FIXTURE_BANK_HASH: [u8; 32] = [
            0x5f, 0x2a, 0x1f, 0x93, 0x75, 0x45, 0x32, 0x21, 0xc6, 0x88, 0x8d, 0x6d, 0x9b, 0x3b,
            0x15, 0xfc, 0xeb, 0x69, 0xee, 0x0b, 0xfb, 0x09, 0x98, 0xba, 0x81, 0x59, 0x85, 0x2b,
            0x9a, 0x15, 0x9e, 0x2b,
        ];
        let plaintext = [9_u8; 32];
        let with_zero =
            computed_trivial_handle(plaintext, 5, 12345, [0; 32], 1_700_000_000);
        let with_fixture = computed_trivial_handle(
            plaintext,
            5,
            12345,
            FIXTURE_BANK_HASH,
            1_700_000_000,
        );
        assert_ne!(with_zero, with_fixture);
    }

    #[test]
    fn previous_bank_hash_fails_at_slot_zero() {
        assert!(previous_bank_hash(0).is_err());
    }

    #[test]
    fn different_bank_hash_changes_handle() {
        let plaintext = [9_u8; 32];
        let with_zero = computed_trivial_handle(plaintext, 5, 12345, [0; 32], 42);
        let with_hash = computed_trivial_handle(plaintext, 5, 12345, [0xCD; 32], 42);
        assert_ne!(with_zero, with_hash);
    }
}
