//! The Clock sysvar, as much of it as an off-chain delegation check reads: the Unix time a
//! delegation's `expires_at` is compared against, at the slot of the read that carries it.

use crate::AclError;

/// `SysvarC1ock11111111111111111111111111111111`.
pub const CLOCK_SYSVAR_ID: [u8; 32] = [
    0x06, 0xa7, 0xd5, 0x17, 0x18, 0xc7, 0x74, 0xc9, 0x28, 0x56, 0x63, 0x98, 0x69, 0x1d, 0x5e, 0xb6,
    0x8b, 0x5e, 0xb8, 0xa3, 0x9b, 0x4b, 0x6d, 0x5c, 0x73, 0x55, 0x5b, 0x21, 0x00, 0x00, 0x00, 0x00,
];

/// `Sysvar1111111111111111111111111111111111111`, the owner of every sysvar account.
pub const SYSVAR_OWNER_ID: [u8; 32] = [
    0x06, 0xa7, 0xd5, 0x17, 0x18, 0x75, 0xf7, 0x29, 0xc7, 0x3d, 0x93, 0x40, 0x8f, 0x21, 0x61, 0x20,
    0x06, 0x7e, 0xd8, 0x8c, 0x76, 0xe0, 0x8c, 0x28, 0x7f, 0xc1, 0x94, 0x60, 0x00, 0x00, 0x00, 0x00,
];

/// `slot`, `epoch_start_timestamp`, `epoch`, `leader_schedule_epoch`, `unix_timestamp`: five
/// little-endian 8-byte fields.
const CLOCK_LEN: usize = 40;
const UNIX_TIMESTAMP_OFFSET: usize = 32;

/// The Clock account data reading `unix_timestamp`, every other field zero, for test doubles of
/// an RPC node.
pub fn encode_clock(unix_timestamp: u64) -> Vec<u8> {
    let mut data = vec![0; CLOCK_LEN];
    data[UNIX_TIMESTAMP_OFFSET..].copy_from_slice(&unix_timestamp.to_le_bytes());
    data
}

/// The `unix_timestamp` of an account read at [`CLOCK_SYSVAR_ID`], given its owner and data. Only
/// the sysvar owner's account is the Clock. A time before the epoch is refused rather than cast, as
/// the host refuses it when it writes an expiry.
pub fn decode_clock_unix_timestamp(owner: &[u8; 32], data: &[u8]) -> Result<u64, AclError> {
    if *owner != SYSVAR_OWNER_ID {
        return Err(AclError::BadOwner);
    }
    let field: [u8; 8] = data
        .get(UNIX_TIMESTAMP_OFFSET..CLOCK_LEN)
        .filter(|_| data.len() == CLOCK_LEN)
        .and_then(|field| field.try_into().ok())
        .ok_or(AclError::BadAccountData)?;
    u64::try_from(i64::from_le_bytes(field)).map_err(|_| AclError::BadAccountData)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clock(unix_timestamp: i64) -> Vec<u8> {
        let mut data = Vec::new();
        for field in [7u64, 8, 9, 10] {
            data.extend_from_slice(&field.to_le_bytes());
        }
        data.extend_from_slice(&unix_timestamp.to_le_bytes());
        data
    }

    fn decode(data: &[u8]) -> Result<u64, AclError> {
        decode_clock_unix_timestamp(&SYSVAR_OWNER_ID, data)
    }

    #[test]
    fn reads_the_last_field() {
        assert_eq!(decode(&clock(1_700_000_000)), Ok(1_700_000_000));
    }

    #[test]
    fn refuses_an_account_the_sysvar_owner_does_not_own() {
        assert_eq!(
            decode_clock_unix_timestamp(&[7; 32], &clock(1_700_000_000)),
            Err(AclError::BadOwner)
        );
    }

    #[test]
    fn refuses_a_clock_of_the_wrong_size_or_before_the_epoch() {
        let mut short = clock(1);
        short.pop();
        assert_eq!(decode(&short), Err(AclError::BadAccountData));
        let mut long = clock(1);
        long.push(0);
        assert_eq!(decode(&long), Err(AclError::BadAccountData));
        assert_eq!(decode(&clock(-1)), Err(AclError::BadAccountData));
    }
}
