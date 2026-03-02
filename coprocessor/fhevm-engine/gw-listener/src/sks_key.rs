use tfhe::ServerKey;

use fhevm_engine_common::utils::{safe_deserialize_sns_key, safe_serialize_key};

pub fn extract_server_key_without_ns(sns_key: &[u8]) -> anyhow::Result<Vec<u8>> {
    // Bypass for integration tests
    #[cfg(feature = "test_bypass_key_extraction")]
    if sns_key == b"key_bytes" {
        return Ok(b"key_bytes".to_vec());
    }

    let server_key: ServerKey = safe_deserialize_sns_key(sns_key)?;

    let (
        sks,
        kskm,
        compression_key,
        decompression_key,
        noise_squashing_key,
        noise_squashing_compression_key,
        re_randomization_keyswitching_key,
        tag,
    ) = server_key.into_raw_parts();

    if noise_squashing_key.is_none() {
        anyhow::bail!("Server key does not have noise squashing");
    }
    if noise_squashing_compression_key.is_none() {
        anyhow::bail!("Server key does not have noise squashing compression");
    }
    if re_randomization_keyswitching_key.is_none() {
        anyhow::bail!("Server key does not have rerandomisation");
    }

    Ok(safe_serialize_key(&ServerKey::from_raw_parts(
        sks,
        kskm,
        compression_key,
        decompression_key,
        None,                              // noise squashing key excluded
        None,                              // noise squashing compression key excluded
        re_randomization_keyswitching_key, // rerandomisation keyswitching key excluded
        tag,
    )))
}
