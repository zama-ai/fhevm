//! Writes the `fhevm-keys/xof-keyset` and `fhevm-keys/xof-cks` test fixtures. See
//! `fhevm-keys/README.md`.
use fhevm_engine_common::{keys::FhevmKeys, utils::safe_serialize_key};
use tfhe::{
    core_crypto::prelude::NormalizedHammingWeightBound,
    shortint::parameters::v1_8::transciphering::V1_8_TRANSCIPHERING_PARAM_DEDICATED_OPRF,
    xof_key_set::CompressedXofKeySet, Tag,
};

fn main() -> anyhow::Result<()> {
    let dir = std::env::args()
        .nth(1)
        .unwrap_or("../fhevm-keys".to_string());
    // The coprocessor parameters, with the transciphering key (dedicated OPRF) of the KMS keygen.
    let config = FhevmKeys::new_config_builder()
        .enable_transciphering(V1_8_TRANSCIPHERING_PARAM_DEDICATED_OPRF)
        .build();
    let (client_key, keyset) = CompressedXofKeySet::generate(
        config,
        vec![0; 32],
        128,
        NormalizedHammingWeightBound::new(0.8).expect("valid fixture bound"),
        Tag::from("fhevm-keys-xof-fixture"),
    )?;
    std::fs::write(format!("{dir}/xof-keyset"), safe_serialize_key(&keyset))?;
    std::fs::write(format!("{dir}/xof-cks"), safe_serialize_key(&client_key))?;
    println!("wrote {dir}/xof-keyset and {dir}/xof-cks");
    Ok(())
}
