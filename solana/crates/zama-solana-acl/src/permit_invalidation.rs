//! The per-user permit invalidation record: layout and decoder.
//!
//! Mirrors `zama-host`'s `PermitInvalidation` (8-byte Anchor discriminator + 41-byte body), whose
//! discriminator literal the host pins in its own state test. A permit that starts before the
//! record's watermark is dead.

use crate::delegation::{bytes32, u64_le};
use crate::AclError;

/// Seed of the per-user record PDA: `[seed, user]`.
pub const PERMIT_INVALIDATION_SEED: &[u8] = b"permit-invalidation";

const ANCHOR_DISCRIMINATOR_LEN: usize = 8;
const BODY_LEN: usize = 32 + 8 + 1;

/// `sha256("account:PermitInvalidation")[..8]`.
pub const PERMIT_INVALIDATION_DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_LEN] =
    [0xec, 0x8b, 0xdb, 0xa9, 0xb9, 0x22, 0xe9, 0x88];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PermitInvalidationRecord {
    pub user: [u8; 32],
    /// Unix seconds of the user's last revocation.
    pub invalidation_watermark: u64,
    pub bump: u8,
}

/// The account bytes of a record in this state, for test doubles of the host program.
pub fn encode_permit_invalidation(record: &PermitInvalidationRecord) -> Vec<u8> {
    let mut data = PERMIT_INVALIDATION_DISCRIMINATOR.to_vec();
    data.extend_from_slice(&record.user);
    data.extend_from_slice(&record.invalidation_watermark.to_le_bytes());
    data.push(record.bump);
    data
}

/// Decodes an account's raw data, discriminator included. The record is fixed-size and never
/// realloc-grown, so any other length is refused.
pub fn decode_permit_invalidation(data: &[u8]) -> Result<PermitInvalidationRecord, AclError> {
    if data.len() != ANCHOR_DISCRIMINATOR_LEN + BODY_LEN {
        return Err(AclError::BadAccountData);
    }
    if data[..ANCHOR_DISCRIMINATOR_LEN] != PERMIT_INVALIDATION_DISCRIMINATOR {
        return Err(AclError::BadDiscriminator);
    }
    let body = &data[ANCHOR_DISCRIMINATOR_LEN..];
    Ok(PermitInvalidationRecord {
        user: bytes32(body, 0),
        invalidation_watermark: u64_le(body, 32),
        bump: body[40],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The account the host program wrote, as committed for every consumer of this layout.
    #[test]
    fn decodes_the_account_the_host_program_wrote() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../test-fixtures/permit/permit_invalidation_account_v1.json"
        ))
        .unwrap();
        let data = hex::decode(fixture["account"]["data_hex"].as_str().unwrap()).unwrap();
        let record = decode_permit_invalidation(&data).unwrap();
        assert_eq!(
            fixture["address"]["seeds"][1]["hex"],
            hex::encode(record.user)
        );
        assert_eq!(
            fixture["fields"][2]["value"],
            record.invalidation_watermark.to_string()
        );
        assert_eq!(fixture["address"]["bump"], record.bump);
        assert_eq!(encode_permit_invalidation(&record), data);
    }

    #[test]
    fn round_trips_and_refuses_other_layouts() {
        let record = PermitInvalidationRecord {
            user: [0x11; 32],
            invalidation_watermark: 0x0102_0304_0506_0708,
            bump: 254,
        };
        let data = encode_permit_invalidation(&record);
        assert_eq!(decode_permit_invalidation(&data), Ok(record));

        let mut grown = data.clone();
        grown.push(0);
        assert_eq!(
            decode_permit_invalidation(&grown),
            Err(AclError::BadAccountData)
        );
        let mut foreign = data;
        foreign[0] ^= 0xff;
        assert_eq!(
            decode_permit_invalidation(&foreign),
            Err(AclError::BadDiscriminator)
        );
    }
}
