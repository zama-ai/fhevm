use tfhe::{xof_key_set::CompressedXofKeySet, ServerKey};

use fhevm_engine_common::utils::{
    safe_deserialize_sns_key, safe_serialize_key,
};

/// Blobs derived from a downloaded server-key payload, ready to be written
/// into `kms_key_activation_events`.
///
/// `sns_pk` / `sks_key` are the legacy decompressed pair (full ServerKey
/// and NS-stripped ServerKey). `compressed_xof_keyset` is the raw
/// `CompressedXofKeySet` blob as published by kms-core; it is `Some`
/// when the source was a CompressedXofKeySet and `None` when the
/// source was a plain ServerKey blob (the legacy path).
///
/// The compressed blob has to be stored unmodified: subkey
/// decompression advances a shared XOF state, so dropping any subkey
/// (e.g. noise-squashing) before decompression diverges the remaining
/// subkeys. Anything that wants an NS-stripped view of the compressed
/// keyset has to decompress the whole thing first and strip
/// afterwards.
#[derive(Clone, Debug)]
pub(crate) struct PreparedServerKey {
    pub sns_pk: Vec<u8>,
    pub sks_key: Vec<u8>,
    pub compressed_xof_keyset: Option<Vec<u8>>,
}

/// Builds a [`PreparedServerKey`] from a serialized
/// [`tfhe::xof_key_set::CompressedXofKeySet`] payload (kms-core's new
/// keygen format). Dual-writes: populates the legacy decompressed
/// `sns_pk` / `sks_key` pair so existing readers keep working, and
/// stores the input bytes verbatim under `compressed_xof_keyset`.
pub(crate) fn prepare_xof_key_set_for_db(
    key_bytes: &[u8],
) -> anyhow::Result<PreparedServerKey> {
    // Bypass for integration tests
    #[cfg(feature = "test_bypass_key_extraction")]
    if key_bytes == b"key_bytes" {
        return Ok(PreparedServerKey {
            sns_pk: b"key_bytes".to_vec(),
            sks_key: b"key_bytes".to_vec(),
            compressed_xof_keyset: Some(b"key_bytes".to_vec()),
        });
    }

    let compressed_key_set: CompressedXofKeySet =
        safe_deserialize_sns_key(key_bytes)?;

    // Decompress the whole keyset in one pass so the XOF state advances
    // over every subkey, then strip NS material from the decompressed
    // ServerKey. The compressed blob itself is kept verbatim so
    // sns-worker (and GPU readers) can re-do the same whole-keyset
    // decompression at consumption time.
    //
    // The decompressed public key is intentionally discarded: for keys
    // that came from a kms-core migration, the legacy `/PublicKey`
    // blob in S3 is preserved unchanged and is the one clients
    // encrypt under (so it's also the one the zkproof-worker must
    // verify against). The public key embedded in the
    // CompressedXofKeySet diverges from it post-migration, so any
    // attempt to cross-check the two would fire on every migrated
    // key.
    let (_public_key, server_key) =
        compressed_key_set.decompress()?.into_raw_parts();
    let sns_pk = safe_serialize_key(&server_key);
    let sks_key = extract_server_key_without_ns_from_server_key(server_key)?;

    Ok(PreparedServerKey {
        sns_pk,
        sks_key,
        compressed_xof_keyset: Some(key_bytes.to_vec()),
    })
}

/// Builds a [`PreparedServerKey`] from a serialized [`tfhe::ServerKey`]
/// payload (kms-core's pre-XOF keygen format). The compressed column
/// is left `None`: we cannot synthesize a `CompressedXofKeySet` from a
/// plain `ServerKey`, so legacy-source rows stay on the legacy columns
/// until a future keygen rotation produces a CompressedXofKeySet.
pub(crate) fn prepare_legacy_server_key_for_db(
    key_bytes: &[u8],
) -> anyhow::Result<PreparedServerKey> {
    // Bypass for integration tests
    #[cfg(feature = "test_bypass_key_extraction")]
    if key_bytes == b"key_bytes" {
        return Ok(PreparedServerKey {
            sns_pk: b"key_bytes".to_vec(),
            sks_key: b"key_bytes".to_vec(),
            compressed_xof_keyset: None,
        });
    }

    let server_key: ServerKey = safe_deserialize_sns_key(key_bytes)?;
    let sks_key = extract_server_key_without_ns_from_server_key(server_key)?;
    Ok(PreparedServerKey {
        sns_pk: key_bytes.to_vec(),
        sks_key,
        compressed_xof_keyset: None,
    })
}

/// Decodes a serialized `CompressedXofKeySet`, decompresses the
/// embedded server key, strips noise-squashing material, and returns
/// the safe-serialized result. Only used by the ignored E2E test
/// below to cross-check the surgery against the decompress-first
/// path; production code calls [`prepare_xof_key_set_for_db`].
#[cfg(test)]
pub(crate) fn extract_server_key_without_ns(
    key_bytes: &[u8],
) -> anyhow::Result<Vec<u8>> {
    let compressed_key_set: CompressedXofKeySet =
        safe_deserialize_sns_key(key_bytes)?;
    let (_public_key, server_key) =
        compressed_key_set.decompress()?.into_raw_parts();
    extract_server_key_without_ns_from_server_key(server_key)
}

/// Same as [`extract_server_key_without_ns`] but consumes a plain
/// [`tfhe::ServerKey`] blob (legacy kms-core format).
#[cfg(test)]
pub(crate) fn extract_legacy_server_key_without_ns(
    key_bytes: &[u8],
) -> anyhow::Result<Vec<u8>> {
    let server_key: ServerKey = safe_deserialize_sns_key(key_bytes)?;
    extract_server_key_without_ns_from_server_key(server_key)
}

fn extract_server_key_without_ns_from_server_key(
    server_key: ServerKey,
) -> anyhow::Result<Vec<u8>> {
    let (
        sks,
        kskm,
        compression_key,
        decompression_key,
        noise_squashing_key,
        noise_squashing_compression_key,
        re_randomization_keyswitching_key,
        oprf_key,
        tag,
    ) = server_key.into_raw_parts();

    if noise_squashing_key.is_none() {
        anyhow::bail!("Server key is missing the noise squashing key");
    }
    if noise_squashing_compression_key.is_none() {
        anyhow::bail!(
            "Server key is missing the noise squashing compression key"
        );
    }
    if re_randomization_keyswitching_key.is_none() {
        anyhow::bail!("Server key is missing rerandomisation keyswitching key");
    }

    Ok(safe_serialize_key(&ServerKey::from_raw_parts(
        sks,
        kskm,
        compression_key,
        decompression_key,
        None, // noise squashing key excluded
        None, // noise squashing compression key excluded
        re_randomization_keyswitching_key,
        oprf_key,
        tag,
    )))
}

#[cfg(test)]
mod test {
    use super::*;
    use fhevm_engine_common::keys::FhevmKeys;
    use tfhe::core_crypto::prelude::NormalizedHammingWeightBound;
    use tfhe::{
        ClientKey, CompressedCompactPublicKey, CompressedServerKey,
        ConfigBuilder, Tag, XofSeed,
    };

    fn build_serialized_compressed_xof_key_set_without_ns() -> Vec<u8> {
        let config = ConfigBuilder::default().build();
        let client_key = ClientKey::generate(config);
        let compressed_server_key = CompressedServerKey::new(&client_key);
        let compressed_public_key =
            CompressedCompactPublicKey::new(&client_key);
        let seed = XofSeed::new(vec![0u8; 32], *b"TFHE_GEN");

        let xof_key_set = CompressedXofKeySet::from_raw_parts(
            seed,
            compressed_public_key,
            compressed_server_key,
        );

        safe_serialize_key(&xof_key_set)
    }

    #[test]
    fn prepare_xof_key_set_for_db_rejects_compressed_xof_without_ns(
    ) -> anyhow::Result<()> {
        // Keep default CI on a real CompressedXofKeySet decode path
        // without paying for production-sized NS material. The check
        // now fires post-decompression against the materialized
        // ServerKey, so the error string is the decompressed-side one.
        let compressed_xof_key_set_bytes =
            build_serialized_compressed_xof_key_set_without_ns();

        let err = prepare_xof_key_set_for_db(&compressed_xof_key_set_bytes)
            .expect_err(
                "XOF keyset without noise-squashing material must fail",
            );

        assert!(err
            .to_string()
            .contains("Server key is missing the noise squashing"));

        Ok(())
    }

    #[test]
    #[ignore = "generates full tfhe XOF keys and is too slow for default test runs"]
    fn compressed_xof_key_set_round_trips_through_prepared_blobs(
    ) -> anyhow::Result<()> {
        let (_client_key, compressed_key_set) = CompressedXofKeySet::generate(
            FhevmKeys::new_config(),
            vec![7; 32],
            128,
            NormalizedHammingWeightBound::new(0.8).unwrap(),
            Tag::from("host-listener-xof-test"),
        )?;

        let (_public_key, server_key) =
            compressed_key_set.decompress()?.into_raw_parts();
        let server_key_bytes = safe_serialize_key(&server_key);
        let compressed_key_set_bytes = safe_serialize_key(&compressed_key_set);

        // Standalone "strip NS from a fully decompressed ServerKey"
        // helper as the baseline.
        let stripped_from_server_key =
            extract_legacy_server_key_without_ns(&server_key_bytes)?;
        // Strip-after-whole-keyset-decompress through the helper used
        // by the e2e test path.
        let stripped_from_xof =
            extract_server_key_without_ns(&compressed_key_set_bytes)?;

        assert_eq!(stripped_from_xof, stripped_from_server_key);

        // prepare_xof_key_set_for_db returns the legacy decompressed
        // pair and the raw CompressedXofKeySet bytes.
        let prepared = prepare_xof_key_set_for_db(&compressed_key_set_bytes)?;

        // Legacy sns_pk must deserialize back to a ServerKey.
        let _legacy_full: ServerKey =
            safe_deserialize_sns_key(&prepared.sns_pk)?;
        // Legacy sks_key matches the standalone strip result.
        assert_eq!(prepared.sks_key, stripped_from_server_key);

        // The compressed column is the verbatim input — nothing in
        // the prepare step rewrites it.
        let compressed_blob = prepared
            .compressed_xof_keyset
            .expect("compressed xof keyset should be populated for XOF input");
        assert_eq!(compressed_blob, compressed_key_set_bytes);

        // A consumer that re-decompresses the whole keyset must get
        // back the same full ServerKey as the legacy sns_pk.
        let kxs: CompressedXofKeySet =
            safe_deserialize_sns_key(&compressed_blob)?;
        let (_public_key, decompressed_full) =
            kxs.decompress()?.into_raw_parts();
        assert_eq!(safe_serialize_key(&decompressed_full), prepared.sns_pk);

        // The legacy stripped ServerKey has the expected NS layout.
        let stripped_server_key: ServerKey =
            safe_deserialize_sns_key(&stripped_from_xof)?;
        let (
            _sks,
            _kskm,
            _compression_key,
            _decompression_key,
            noise_squashing_key,
            noise_squashing_compression_key,
            re_randomization_keyswitching_key,
            _oprf_key,
            _tag,
        ) = stripped_server_key.into_raw_parts();

        assert!(noise_squashing_key.is_none());
        assert!(noise_squashing_compression_key.is_none());
        assert!(re_randomization_keyswitching_key.is_some());

        Ok(())
    }
}
