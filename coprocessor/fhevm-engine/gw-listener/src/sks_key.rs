use tfhe::ServerKey;

use fhevm_engine_common::utils::{safe_deserialize_sns_key, safe_serialize_key};

pub fn extract_server_key_without_ns(sns_key: &[u8]) -> anyhow::Result<Vec<u8>> {
    // for integration tests
    if sns_key == "key_bytes".as_bytes() {
        return Ok("key_bytes".as_bytes().to_vec());
    }

    let server_key: ServerKey = safe_deserialize_sns_key(sns_key)?;

    let (sks, kskm, compression_key, decompression_key, _, noise_squashing_key, tag) =
        server_key.into_raw_parts();

    if noise_squashing_key.is_none() {
        anyhow::bail!("Server key does not have noise squashing");
    }

    Ok(safe_serialize_key(&ServerKey::from_raw_parts(
        sks,
        kskm,
        compression_key,
        decompression_key,
        None, // noise squashing key excluded
        None, // noise squashing compression key excluded
        tag,
    )))
}
