//! A replacement for the one-time pad of tfhe-rs, until a release has it: the 2-bit mask, its
//! compressed form (the ct64), and the XOR of Unmask. The types follow the expected tfhe-rs API.
//!
//! INSECURE: the mask holds the pad in clear, from the public cleartext PRF of the SDK. Use it only
//! in tests and in the e2e stack.

use super::{part_bit_width, PadError, PadPackage, PadUnmasker, PAD_MAX_PAD_BITS};
use crate::{tfhe_ops::trivial_encrypt_be_bytes, types::SupportedFheCiphertexts};
use alloy::primitives::{keccak256, B256};

/// Pad bits in each ciphertext of the mask: the 2-bit encoding.
pub const PAD_BITS_PER_CIPHERTEXT: usize = 2;
/// Estimated size of one compressed ciphertext of the mask. A 64-bit pad has 32 ciphertexts, so
/// about 2 kB.
pub const MOCK_CT64_BYTES_PER_CIPHERTEXT: usize = 64;
/// Domain tag of the cleartext PRF of the SDK (`LibCleartextPad`).
pub const CLEARTEXT_PAD_DOMAIN_SEPARATOR: &[u8; 8] = b"FHE_cpad";
const CLEARTEXT_BLOCK_BYTES: usize = 32;
const CT64_MAGIC: &[u8; 4] = b"MOTP";
const CT64_VERSION: u8 = 1;
/// `magic 4 | version 1 | bits per ciphertext 1 | n_bits 2` (big-endian).
const CT64_HEADER_BYTES: usize = 8;

/// The first `n_bits` of the pad of `(prf_id, label)`, least significant bit first: the cleartext
/// PRF of the SDK. Block `i` is `keccak256("FHE_cpad" ‖ prfId ‖ label ‖ le32(i))`, 256 bits.
pub fn mock_pad(prf_id: u16, label: B256, n_bits: usize) -> Vec<u8> {
    let len = n_bits.div_ceil(8);
    let mut pad = Vec::with_capacity(len.next_multiple_of(CLEARTEXT_BLOCK_BYTES));
    for i in 0..len.div_ceil(CLEARTEXT_BLOCK_BYTES) {
        let index = u32::try_from(i).expect("block index fits in 32 bits");
        pad.extend(keccak256(
            [
                CLEARTEXT_PAD_DOMAIN_SEPARATOR.as_slice(),
                &prf_id.to_be_bytes(),
                label.as_slice(),
                &index.to_le_bytes(),
            ]
            .concat(),
        ));
    }
    pad.truncate(len);
    if let Some(last) = pad.last_mut() {
        *last &= high_bits_mask(n_bits);
    }
    pad
}

/// The mask of the last byte of `n_bits` bits.
fn high_bits_mask(n_bits: usize) -> u8 {
    match n_bits % 8 {
        0 => u8::MAX,
        rest => (1u8 << rest) - 1,
    }
}

/// Mock of `OneTimePadFheSecretMask`: `⌈n_bits/2⌉` ciphertexts of 2 pad bits each.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MockOneTimePadMask {
    pad: Vec<u8>,
    n_bits: usize,
}

impl MockOneTimePadMask {
    /// Mock of `new_random(seed, n_bits)`: the PRF of the instance on `label`. `prf_id` replaces
    /// the key of the instance.
    pub fn new_random(prf_id: u16, label: B256, n_bits: usize) -> Result<Self, PadError> {
        if !(1..=PAD_MAX_PAD_BITS).contains(&n_bits) {
            return Err(PadError::BadPadLength(n_bits));
        }
        Ok(Self {
            pad: mock_pad(prf_id, label, n_bits),
            n_bits,
        })
    }

    /// Mock of the compression of the mask: the ct64.
    pub fn compress(&self) -> MockCompressedOneTimePadMask {
        let mut bytes = Vec::with_capacity(MockCompressedOneTimePadMask::size(self.n_bits));
        bytes.extend(CT64_MAGIC);
        bytes.push(CT64_VERSION);
        bytes.push(PAD_BITS_PER_CIPHERTEXT as u8);
        bytes.extend(
            u16::try_from(self.n_bits)
                .expect("at most 2048")
                .to_be_bytes(),
        );
        bytes.extend(&self.pad);
        bytes.resize(MockCompressedOneTimePadMask::size(self.n_bits), 0);
        MockCompressedOneTimePadMask { bytes }
    }
}

/// Mock of the compressed mask: the ct64 that the pad-worker stores and Unmask reads. It has the
/// estimated real size, and the pad in clear after a header. The rest is zeros, so the bytes are
/// deterministic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MockCompressedOneTimePadMask {
    bytes: Vec<u8>,
}

impl MockCompressedOneTimePadMask {
    fn size(n_bits: usize) -> usize {
        let pad = CT64_HEADER_BYTES + n_bits.div_ceil(8);
        let real = n_bits.div_ceil(PAD_BITS_PER_CIPHERTEXT) * MOCK_CT64_BYTES_PER_CIPHERTEXT;
        pad.max(real)
    }

    /// Mock of the safe deserialization with the conformance check. It accepts only bytes from
    /// `compress`.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, PadError> {
        if bytes.len() < CT64_HEADER_BYTES || &bytes[..4] != CT64_MAGIC {
            return Err(PadError::BadCt64("not a mock ct64"));
        }
        if bytes[4] != CT64_VERSION {
            return Err(PadError::BadCt64("unknown version"));
        }
        if usize::from(bytes[5]) != PAD_BITS_PER_CIPHERTEXT {
            return Err(PadError::BadCt64("not 2 bits per ciphertext"));
        }
        let n_bits = usize::from(u16::from_be_bytes([bytes[6], bytes[7]]));
        if !(1..=PAD_MAX_PAD_BITS).contains(&n_bits) {
            return Err(PadError::BadCt64("bad pad length"));
        }
        if bytes.len() != Self::size(n_bits) {
            return Err(PadError::BadCt64("bad size"));
        }
        let pad_end = CT64_HEADER_BYTES + n_bits.div_ceil(8);
        if bytes[pad_end - 1] & !high_bits_mask(n_bits) != 0
            || bytes[pad_end..].iter().any(|&b| b != 0)
        {
            return Err(PadError::BadCt64("bits after the pad"));
        }
        Ok(Self {
            bytes: bytes.to_vec(),
        })
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Mock of the decompression of only the ciphertexts of `len` pad bytes from byte `offset`.
    /// tfhe-rs uses one PBS for each ciphertext that it decompresses.
    pub fn decompress_bytes(
        &self,
        offset: usize,
        len: usize,
    ) -> Result<MockOneTimePadMask, PadError> {
        let n_bits = usize::from(u16::from_be_bytes([self.bytes[6], self.bytes[7]]));
        let end = offset + len;
        if end * 8 > n_bits {
            return Err(PadError::PadTooShort {
                pad_bits: n_bits,
                needed: end * 8,
            });
        }
        Ok(MockOneTimePadMask {
            pad: self.bytes[CT64_HEADER_BYTES + offset..CT64_HEADER_BYTES + end].to_vec(),
            n_bits: len * 8,
        })
    }
}

/// Mock of `TranscipherSession::one_time_pad`: it uses the pad bits in sequence from bit 0, one
/// part after the other.
#[derive(Debug)]
pub struct MockTranscipherSession {
    mask: MockOneTimePadMask,
    used_bytes: usize,
}

impl MockTranscipherSession {
    pub fn one_time_pad(mask: MockOneTimePadMask) -> Self {
        Self {
            mask,
            used_bytes: 0,
        }
    }

    /// The next part: `m = c ⊕ r` over the bytes of `masked`, least significant byte first, with
    /// the bits above the width of `fhe_type` cleared. tfhe-rs does the XOR under encryption. The
    /// mock does it in clear, and encrypts `m` trivially, as a value of `fhe_type`.
    pub fn transcipher(
        &mut self,
        masked: &[u8],
        fhe_type: i16,
    ) -> Result<SupportedFheCiphertexts, PadError> {
        let width = part_bit_width(fhe_type)
            .filter(|width| width.div_ceil(8) == masked.len())
            .ok_or(PadError::BadPart)?;
        let end = self.used_bytes + masked.len();
        if end * 8 > self.mask.n_bits {
            return Err(PadError::PadTooShort {
                pad_bits: self.mask.n_bits,
                needed: end * 8,
            });
        }
        let mut value: Vec<u8> = masked
            .iter()
            .zip(&self.mask.pad[self.used_bytes..end])
            .map(|(c, r)| c ^ r)
            .collect();
        *value.last_mut().expect("at least one byte") &= high_bits_mask(width);
        self.used_bytes = end;
        value.reverse();
        trivial_encrypt_be_bytes(fhe_type, &value).map_err(PadError::Encryption)
    }
}

/// [`PadUnmasker`] with the mock ct64. It decompresses the pad of the input, then does the XOR.
#[derive(Debug, Default, Clone, Copy)]
pub struct MockPadUnmasker;

impl PadUnmasker for MockPadUnmasker {
    fn unmask(
        &self,
        ct64: &[u8],
        package: &PadPackage,
        index: usize,
    ) -> Result<SupportedFheCiphertexts, PadError> {
        let part = package.parts.get(index).ok_or(PadError::BadPart)?;
        let mask = MockCompressedOneTimePadMask::from_bytes(ct64)?
            .decompress_bytes(part.offset, part.len)?;
        let masked = &package.ciphertext[part.offset..part.offset + part.len];
        MockTranscipherSession::one_time_pad(mask).transcipher(masked, part.fhe_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pad::tests::package_bytes;

    #[test]
    fn the_pad_is_the_cleartext_prf_of_the_sdk() {
        let label = B256::repeat_byte(0xab);
        let block = |i: u32| {
            keccak256(
                [
                    b"FHE_cpad".as_slice(),
                    &7u16.to_be_bytes(),
                    label.as_slice(),
                    &i.to_le_bytes(),
                ]
                .concat(),
            )
        };
        let pad = mock_pad(7, label, 512);
        assert_eq!(pad, [block(0).as_slice(), block(1).as_slice()].concat());
        // Counter mode: a shorter pad is the start of a longer one.
        assert_eq!(mock_pad(7, label, 64), pad[..8]);
        // The bits after `n_bits` are 0.
        assert_eq!(mock_pad(7, label, 12), [pad[0], pad[1] & 0x0f]);
        assert_ne!(mock_pad(8, label, 64), pad[..8]);
    }

    #[test]
    fn the_ct64_has_the_real_size_and_round_trips() {
        let mask = MockOneTimePadMask::new_random(7, B256::repeat_byte(1), 64).unwrap();
        let ct64 = mask.compress();
        assert_eq!(ct64.as_bytes().len(), 2048);
        assert_eq!(
            MockOneTimePadMask::new_random(7, B256::repeat_byte(1), 2048)
                .unwrap()
                .compress()
                .as_bytes()
                .len(),
            64 * 1024
        );
        let read = MockCompressedOneTimePadMask::from_bytes(ct64.as_bytes()).unwrap();
        assert_eq!(read.decompress_bytes(0, 8).unwrap(), mask);
        assert!(read.decompress_bytes(1, 8).is_err());
        assert!(MockOneTimePadMask::new_random(7, B256::ZERO, 0).is_err());
        assert!(MockOneTimePadMask::new_random(7, B256::ZERO, 2049).is_err());
    }

    #[test]
    fn refuses_a_ct64_that_compress_did_not_write() {
        let good = MockOneTimePadMask::new_random(7, B256::repeat_byte(1), 12)
            .unwrap()
            .compress()
            .into_bytes();
        let changed = |at: usize, value: u8| {
            let mut bytes = good.clone();
            bytes[at] = value;
            MockCompressedOneTimePadMask::from_bytes(&bytes).is_err()
        };
        assert!(changed(0, b'X'));
        assert!(changed(4, 2));
        assert!(changed(5, 4));
        assert!(changed(7, 0));
        // A bit above the 12 bits of the pad, and a byte in the filler.
        assert!(changed(
            CT64_HEADER_BYTES + 1,
            good[CT64_HEADER_BYTES + 1] | 0x10
        ));
        assert!(changed(good.len() - 1, 1));
        assert!(MockCompressedOneTimePadMask::from_bytes(&good[..good.len() - 1]).is_err());
        assert!(MockCompressedOneTimePadMask::from_bytes(&[]).is_err());
    }

    #[test]
    fn unmasks_each_part_with_the_pad() {
        let (client_key, server_key) = tfhe::generate_keys(tfhe::ConfigBuilder::default());
        tfhe::set_server_key(server_key);
        // ebool true, euint16 0xbeef, euint64 42, euint256 2^255 + 1: LSB first, as the SDK packs.
        let mut plain = vec![1u8];
        plain.extend(0xbeefu16.to_le_bytes());
        plain.extend(42u64.to_le_bytes());
        let mut big = [0u8; 32];
        big[0] = 1;
        big[31] = 0x80;
        plain.extend(big);
        let mut bytes = package_bytes(7, &[0, 3, 5, 8], &vec![0; plain.len()]);
        let label = PadPackage::parse(&bytes).unwrap().label;
        let pad = mock_pad(7, label, plain.len() * 8);
        let start = bytes.len() - plain.len();
        for (i, m) in plain.iter().enumerate() {
            bytes[start + i] = m ^ pad[i];
        }
        let package = PadPackage::parse(&bytes).unwrap();

        // A pad longer than the package works: it starts with the same bits.
        let ct64 = MockOneTimePadMask::new_random(7, label, package.pad_bits() + 64)
            .unwrap()
            .compress();
        let clear: Vec<String> = (0..package.parts.len())
            .map(|index| {
                MockPadUnmasker
                    .unmask(ct64.as_bytes(), &package, index)
                    .unwrap()
                    .decrypt(&client_key)
            })
            .collect();
        let two_255_plus_1 =
            "57896044618658097711785492504343953926634992332820282019728792003956564819969";
        assert_eq!(clear, vec!["true", "48879", "42", two_255_plus_1]);

        let short = MockOneTimePadMask::new_random(7, label, package.pad_bits() - 8)
            .unwrap()
            .compress();
        // The last input, euint256, needs the last pad byte.
        assert!(matches!(
            MockPadUnmasker.unmask(short.as_bytes(), &package, 3),
            Err(PadError::PadTooShort { .. })
        ));
        assert!(MockPadUnmasker
            .unmask(short.as_bytes(), &package, 2)
            .is_ok());

        // An ebool whose byte unmasks to 0b1111_1110: only bit 0 counts, so it is false.
        let mask = MockOneTimePadMask::new_random(7, label, 8).unwrap();
        let masked = [0b1111_1110 ^ mask.pad[0]];
        let value = MockTranscipherSession::one_time_pad(mask)
            .transcipher(&masked, 0)
            .unwrap();
        assert_eq!(value.decrypt(&client_key), "false");
    }
}
