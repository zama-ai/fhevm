//! The host config singleton: layout, decoder, and the operator switches an off-chain reader
//! needs.
//!
//! One byte-level implementation, shared by the program that writes the account and the KMS
//! connector that reads it, for the same reason the delegation record has one (see
//! [`crate::delegation`]): a second hand-written copy of a field table is a copy that can drift,
//! and the field this reader wants — `paused` — sits directly beside another `bool`, so a
//! one-byte slip reads a live switch as off with every length and discriminator guard still
//! passing.
//!
//! The layout mirrors `zama-host`'s `HostConfig` (a fixed 325-byte account: 8-byte Anchor
//! discriminator + 317-byte body) and is pinned against the program's own serializer by the
//! host's `shared_crate_decoder_reads_what_the_program_serializes` state test, which feeds
//! `try_serialize` output through this decoder with `paused` and its neighboring flag set to
//! *different* values, so exactly the slip described above fails there.
//!
//! Only what an off-chain reader acts on is decoded. Everything ahead of `paused` is walked past
//! by width and everything after it by the length check, which is what makes an inserted or
//! widened field a refusal here rather than a silent misread: the account stops being 325 bytes
//! the moment the program's layout moves, and [`AclError::BadAccountData`] is what a
//! reader sees. That couples the release order in one direction — a program carrying a new
//! `HostConfig` field must not ship before the readers that decode it — and never the reverse.

use crate::AclError;

/// Seed of the singleton host config PDA: `[seed]`. Owned here so the program and every
/// off-chain reader derive the same address from one constant.
pub const HOST_CONFIG_SEED: &[u8] = b"host-config";

const ANCHOR_DISCRIMINATOR_LEN: usize = 8;
/// Upper bound on registered coprocessor signers, which fixes the width of the signer array.
const MAX_COPROCESSOR_SIGNERS: usize = 8;
/// Everything the singleton stores ahead of `paused`: the admin, the two chain ids, the input
/// verification contract, the fixed-capacity coprocessor signer set with its count and threshold,
/// the decryption contract, and the current KMS context id.
const PAUSED_OFFSET: usize = 32 + 8 + 8 + 20 + (MAX_COPROCESSOR_SIGNERS * 20) + 1 + 1 + 20 + 32;
/// Everything the singleton stores after `paused` and before its bump: the deny-list flag, the
/// three HCU knobs, and the update slot.
const BUMP_OFFSET: usize = PAUSED_OFFSET + 1 + 1 + 8 + 8 + 8 + 8;
const BODY_LEN: usize = BUMP_OFFSET + 1;

/// The eight bytes every reader matches before trusting the body:
/// `sha256("account:HostConfig")[..8]`, pinned as a literal so a renamed account fails here
/// rather than in a consumer that hand-rolled the hash.
pub const HOST_CONFIG_DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_LEN] =
    [0x43, 0x9e, 0xb0, 0xf8, 0xab, 0x93, 0xa1, 0xdc];

/// The host config singleton, as much of it as an off-chain reader acts on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HostConfigRecord {
    /// The operator's pause switch. While set, the host program refuses its production-shaped
    /// instructions and the KMS connector refuses to serve a user decryption.
    pub paused: bool,
    /// The singleton PDA's bump.
    pub bump: u8,
}

/// Builds the account bytes a `HostConfig` in this state occupies: the discriminator, the record's
/// own fields at their offsets, and zeroes for every field this crate does not read.
///
/// Public API surface: test doubles of the host program. The KMS connector's authorization
/// fixtures stand an account up at the singleton's address without linking the Anchor program
/// (which is on a different Solana major version), and a foreign implementation checking its own
/// decoder needs bytes to check it against. It is the inverse of [`decode_host_config`] over the
/// fields that function reads, and deliberately not a `HostConfig` serializer: the program owns
/// that, and the host-side pin test is what holds the two in step.
pub fn encode_host_config(record: &HostConfigRecord) -> Vec<u8> {
    let mut data = HOST_CONFIG_DISCRIMINATOR.to_vec();
    data.resize(ANCHOR_DISCRIMINATOR_LEN + BODY_LEN, 0);
    data[ANCHOR_DISCRIMINATOR_LEN + PAUSED_OFFSET] = record.paused as u8;
    data[ANCHOR_DISCRIMINATOR_LEN + BUMP_OFFSET] = record.bump;
    data
}

/// Decodes an account's raw data, discriminator included, into the config singleton.
///
/// Strict on both ends: the account is exactly discriminator + body (the singleton is fixed-size
/// and never realloc-grown), and a `bool` byte that is neither 0 nor 1 refuses the whole account
/// rather than being coerced — coercing it is how a pause switch reads as off.
pub fn decode_host_config(data: &[u8]) -> Result<HostConfigRecord, AclError> {
    if data.len() != ANCHOR_DISCRIMINATOR_LEN + BODY_LEN {
        return Err(AclError::BadAccountData);
    }
    if data[..ANCHOR_DISCRIMINATOR_LEN] != HOST_CONFIG_DISCRIMINATOR {
        return Err(AclError::BadDiscriminator);
    }
    let body = &data[ANCHOR_DISCRIMINATOR_LEN..];
    let paused = match body[PAUSED_OFFSET] {
        0 => false,
        1 => true,
        _ => return Err(AclError::BadAccountData),
    };
    Ok(HostConfigRecord {
        paused,
        bump: body[BUMP_OFFSET],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Account bytes carrying both adjacent flags, so an off-by-one offset lands on a value the
    /// assertions can tell apart. `grant_deny_list_enabled` is the flag `paused` sits beside and
    /// the one this crate does not read, so it is poked in on top of the encoder.
    fn encode(paused: u8, grant_deny_list_enabled: u8, bump: u8) -> Vec<u8> {
        let mut data = encode_host_config(&HostConfigRecord {
            paused: paused == 1,
            bump,
        });
        data[ANCHOR_DISCRIMINATOR_LEN + PAUSED_OFFSET] = paused;
        data[ANCHOR_DISCRIMINATOR_LEN + PAUSED_OFFSET + 1] = grant_deny_list_enabled;
        data
    }

    /// The discriminator literal is the hash it claims to be. Both sides pinned: the literal is
    /// what foreign implementations compare against, the preimage says where it comes from.
    #[test]
    fn discriminator_is_the_hash_of_the_account_name() {
        let digest = crate::sha256(&[b"account:HostConfig"]);
        assert_eq!(
            HOST_CONFIG_DISCRIMINATOR,
            digest[..ANCHOR_DISCRIMINATOR_LEN]
        );
    }

    /// The switch is read from its own byte, not from the flag next to it. The two cases differ
    /// only in which of the adjacent booleans is set, so an offset slip in either direction
    /// fails one of them.
    #[test]
    fn reads_paused_and_not_the_flag_beside_it() {
        assert_eq!(
            decode_host_config(&encode(1, 0, 254)),
            Ok(HostConfigRecord {
                paused: true,
                bump: 254
            })
        );
        assert_eq!(
            decode_host_config(&encode(0, 1, 254)),
            Ok(HostConfigRecord {
                paused: false,
                bump: 254
            })
        );
    }

    #[test]
    fn rejects_a_foreign_discriminator() {
        let mut data = encode(1, 0, 254);
        data[0] ^= 0xff;
        assert_eq!(decode_host_config(&data), Err(AclError::BadDiscriminator));
    }

    /// Both directions, which is what makes a layout change a refusal: a field added to
    /// `HostConfig` grows the account, and a reader that has not been taught the new layout stops
    /// rather than reading `paused` off the old offset.
    #[test]
    fn rejects_a_singleton_of_the_wrong_size() {
        let mut short = encode(1, 0, 254);
        short.pop();
        assert_eq!(decode_host_config(&short), Err(AclError::BadAccountData));

        let mut grown = encode(1, 0, 254);
        grown.push(0);
        assert_eq!(decode_host_config(&grown), Err(AclError::BadAccountData));
    }

    #[test]
    fn rejects_a_boolean_that_is_neither() {
        assert_eq!(
            decode_host_config(&encode(2, 0, 254)),
            Err(AclError::BadAccountData)
        );
    }
}
