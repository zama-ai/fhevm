// from zama/kms-core/core/service/src/engine/base.rs
use sha3::{
    digest::{ExtendableOutput, Update, XofReader},
    Shake256,
};

pub type DomainSep = [u8; DSEP_LEN];
pub const DSEP_LEN: usize = 8;
/// Domain separator for public key data
pub const DSEP_PUBDATA_KEY: DomainSep = *b"PDAT_KEY";
/// Domain separator for CRS (Common Reference String) data
pub const DSEP_PUBDATA_CRS: DomainSep = *b"PDAT_CRS";

fn digest(domain_separator: DomainSep, bytes: &[u8]) -> [u8; 32] {
    // see: https://github.com/zama-ai/kms/blob/664289c7c4d98df5e26d711500092d36c08ea8a2/core/threshold/src/hashing.rs#L25
    let mut hasher = Shake256::default();
    hasher.update(&domain_separator);
    hasher.update(bytes);
    let mut output_reader = hasher.finalize_xof();
    let mut digest = [0u8; 32];
    output_reader.read(&mut digest);
    digest
}

pub fn digest_key(bytes: &[u8]) -> [u8; 32] {
    // same DSEP is used for all key kind.
    // see: https://github.com/zama-ai/kms/blob/664289c7c4d98df5e26d711500092d36c08ea8a2/core/service/src/client/key_gen.rs#L147C13-L147C30
    digest(DSEP_PUBDATA_KEY, bytes)
}

pub fn digest_crs(bytes: &[u8]) -> [u8; 32] {
    digest(DSEP_PUBDATA_CRS, bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Independent known vectors from the retired KMSGenerationTest.sol fixture
    // (SHAKE256 over PDAT_KEY / PDAT_CRS || b"key_bytes"). Keep these literals;
    // do not compute them via digest_key / digest_crs.
    const KEY_BYTES: &[u8] = b"key_bytes";
    const KEY_DIGEST: [u8; 32] = [
        0x5d, 0xe8, 0xc3, 0xa0, 0x65, 0xd7, 0x48, 0xb7, 0xb7, 0xaf, 0x29, 0x1f,
        0xc3, 0x0c, 0x52, 0x85, 0x00, 0x6d, 0xaf, 0xbe, 0xad, 0x9e, 0xd5, 0x1e,
        0xb7, 0xd4, 0xdd, 0xeb, 0x4e, 0xb2, 0x4a, 0x56,
    ];
    const CRS_DIGEST: [u8; 32] = [
        0x39, 0xf1, 0xe6, 0x22, 0xf9, 0x4c, 0xe2, 0xd9, 0x28, 0xf7, 0x44, 0x6c,
        0x42, 0x4e, 0x5a, 0x7a, 0x67, 0xe1, 0xc8, 0x94, 0x0f, 0xa6, 0x95, 0xac,
        0x4a, 0x8b, 0xc0, 0xdc, 0x86, 0xd0, 0x93, 0x24,
    ];

    #[test]
    fn digest_key_matches_known_vector() {
        assert_eq!(digest_key(KEY_BYTES), KEY_DIGEST);
    }

    #[test]
    fn digest_crs_matches_known_vector() {
        assert_eq!(digest_crs(KEY_BYTES), CRS_DIGEST);
    }
}
