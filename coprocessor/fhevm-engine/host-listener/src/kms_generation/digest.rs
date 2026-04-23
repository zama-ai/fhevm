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
