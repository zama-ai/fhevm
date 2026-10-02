//! Isolated public-material fixture generator; never installs or activates a key.
use fhevm_engine_common::{
    keys::FhevmKeys,
    utils::{safe_deserialize_sns_key, safe_serialize_key},
};
use std::io::Write;
use tfhe::{
    core_crypto::prelude::NormalizedHammingWeightBound,
    xof_key_set::CompressedXofKeySet, Tag,
};

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() == 2 && args[0] == "--validate" {
        let bytes = std::fs::read(&args[1])?;
        let _: CompressedXofKeySet = safe_deserialize_sns_key(&bytes)?;
        println!(
            "validated independent CompressedXofKeySet ({} bytes)",
            bytes.len()
        );
        return Ok(());
    }
    anyhow::ensure!(args.len() == 1, "provide a new output path");
    let mut output = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[0])?;
    let (_, key) = CompressedXofKeySet::generate(
        FhevmKeys::new_config(),
        vec![183; 32],
        128,
        NormalizedHammingWeightBound::new(0.8).expect("valid fixture bound"),
        Tag::from("independent-migration-rejection-fixture"),
    )?;
    output.write_all(&safe_serialize_key(&key))?;
    output.sync_all()?;
    Ok(())
}
