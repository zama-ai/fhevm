use tfhe::{xof_key_set::CompressedXofKeySet, CompressedServerKey, ServerKey};

use fhevm_engine_common::utils::{
    safe_deserialize_sns_key, safe_serialize_key,
};

/// Blobs derived from a downloaded server-key payload, ready to be written
/// into `kms_key_activation_events`.
///
/// `sns_pk` / `sks_key` are the legacy decompressed pair (full ServerKey
/// and NS-stripped ServerKey). `sns_pk_compressed` / `sks_key_compressed`
/// are the matching CompressedServerKey blobs; both are `Some` when the
/// source was a CompressedXofKeySet (the dual-write path) and `None`
/// when the source was a plain ServerKey blob (the legacy path).
#[derive(Clone, Debug)]
pub(crate) struct PreparedServerKey {
    pub sns_pk: Vec<u8>,
    pub sks_key: Vec<u8>,
    pub sns_pk_compressed: Option<Vec<u8>>,
    pub sks_key_compressed: Option<Vec<u8>>,
}

/// Builds a [`PreparedServerKey`] from a serialized
/// [`tfhe::xof_key_set::CompressedXofKeySet`] payload (kms-core's new
/// keygen format). Populates both the legacy and the compressed
/// column pairs.
pub(crate) fn prepare_xof_key_set_for_db(
    key_bytes: &[u8],
) -> anyhow::Result<PreparedServerKey> {
    // Bypass for integration tests
    #[cfg(feature = "test_bypass_key_extraction")]
    if key_bytes == b"key_bytes" {
        return Ok(PreparedServerKey {
            sns_pk: b"key_bytes".to_vec(),
            sks_key: b"key_bytes".to_vec(),
            sns_pk_compressed: Some(b"key_bytes".to_vec()),
            sks_key_compressed: Some(b"key_bytes".to_vec()),
        });
    }

    let compressed_key_set: CompressedXofKeySet =
        safe_deserialize_sns_key(key_bytes)?;
    let (_seed, _compressed_public_key, compressed_server_key) =
        compressed_key_set.into_raw_parts();

    let sns_pk_compressed = safe_serialize_key(&compressed_server_key);
    let sks_key_compressed =
        extract_server_key_without_ns_from_compressed_server_key(
            compressed_server_key.clone(),
        )?;

    // Dual-write: also materialize the legacy decompressed pair so
    // coprocessor binaries that have not yet been switched to the
    // compressed-preferred read path keep working.
    let server_key = compressed_server_key.decompress();
    let sns_pk = safe_serialize_key(&server_key);
    let sks_key = extract_server_key_without_ns_from_server_key(server_key)?;

    Ok(PreparedServerKey {
        sns_pk,
        sks_key,
        sns_pk_compressed: Some(sns_pk_compressed),
        sks_key_compressed: Some(sks_key_compressed),
    })
}

/// Builds a [`PreparedServerKey`] from a serialized [`tfhe::ServerKey`]
/// payload (kms-core's pre-XOF keygen format). The compressed column
/// pair is left `None`: we cannot synthesize a `CompressedServerKey`
/// from a plain `ServerKey` without the client key, so legacy-source
/// rows stay on the legacy columns until a future keygen rotation
/// produces a CompressedXofKeySet.
pub(crate) fn prepare_legacy_server_key_for_db(
    key_bytes: &[u8],
) -> anyhow::Result<PreparedServerKey> {
    // Bypass for integration tests
    #[cfg(feature = "test_bypass_key_extraction")]
    if key_bytes == b"key_bytes" {
        return Ok(PreparedServerKey {
            sns_pk: b"key_bytes".to_vec(),
            sks_key: b"key_bytes".to_vec(),
            sns_pk_compressed: None,
            sks_key_compressed: None,
        });
    }

    let server_key: ServerKey = safe_deserialize_sns_key(key_bytes)?;
    let sks_key = extract_server_key_without_ns_from_server_key(server_key)?;
    Ok(PreparedServerKey {
        sns_pk: key_bytes.to_vec(),
        sks_key,
        sns_pk_compressed: None,
        sks_key_compressed: None,
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
        None, // noise squashing key excluded
        None, // noise squashing compression key excluded
        re_randomization_keyswitching_key,
        oprf_key,
        tag,
    )))
}

fn extract_server_key_without_ns_from_compressed_server_key(
    compressed_server_key: CompressedServerKey,
) -> anyhow::Result<Vec<u8>> {
    let (
        integer_key,
        cpk_key_switching_key_material,
        compression_key,
        decompression_key,
        noise_squashing_key,
        noise_squashing_compression_key,
        cpk_re_randomization_key,
        oprf_key,
        tag,
    ) = compressed_server_key.into_raw_parts();

    if noise_squashing_key.is_none() {
        anyhow::bail!("Compressed server key does not have noise squashing");
    }
    if noise_squashing_compression_key.is_none() {
        anyhow::bail!(
            "Compressed server key does not have noise squashing compression"
        );
    }
    if cpk_re_randomization_key.is_none() {
        anyhow::bail!("Compressed server key does not have rerandomisation");
    }

    Ok(safe_serialize_key(&CompressedServerKey::from_raw_parts(
        integer_key,
        cpk_key_switching_key_material,
        compression_key,
        decompression_key,
        None, // noise squashing key excluded
        None, // noise squashing compression key excluded
        cpk_re_randomization_key,
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
        ClientKey, CompressedCompactPublicKey, ConfigBuilder, Tag, XofSeed,
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
        // without paying for production-sized NS material or the
        // dual-write decompress.
        let compressed_xof_key_set_bytes =
            build_serialized_compressed_xof_key_set_without_ns();

        let err = prepare_xof_key_set_for_db(&compressed_xof_key_set_bytes)
            .expect_err(
                "XOF keyset without noise-squashing material must fail",
            );

        assert!(err
            .to_string()
            .contains("Compressed server key does not have noise squashing"));

        Ok(())
    }

    #[test]
    #[ignore = "generates full tfhe XOF keys and is too slow for default test runs"]
    fn compressed_xof_key_set_extracts_same_stripped_server_key(
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

        // Legacy decompressed-only path.
        let stripped_from_server_key =
            extract_legacy_server_key_without_ns(&server_key_bytes)?;
        // Legacy decompressed result derived from the XOF blob.
        let stripped_from_xof =
            extract_server_key_without_ns(&compressed_key_set_bytes)?;

        assert_eq!(stripped_from_xof, stripped_from_server_key);

        // Dual-write: prepare_xof_key_set_for_db returns both the legacy
        // decompressed pair and the new compressed pair when fed a
        // CompressedXofKeySet blob.
        let prepared = prepare_xof_key_set_for_db(&compressed_key_set_bytes)?;

        // Legacy sns_pk must deserialize back to a ServerKey.
        let _legacy_full: ServerKey =
            safe_deserialize_sns_key(&prepared.sns_pk)?;
        // Legacy sks_key matches the standalone extract result.
        assert_eq!(prepared.sks_key, stripped_from_server_key);

        let sns_pk_compressed = prepared
            .sns_pk_compressed
            .expect("compressed sns_pk should be populated for XOF input");
        let sks_key_compressed = prepared
            .sks_key_compressed
            .expect("compressed sks_key should be populated for XOF input");

        // Compressed sns_pk deserializes to a CompressedServerKey and,
        // after decompression, must round-trip to the same bytes as the
        // directly decompressed legacy sns_pk.
        let compressed_full: CompressedServerKey =
            fhevm_engine_common::utils::safe_deserialize_key(
                &sns_pk_compressed,
            )?;
        let decompressed_full = compressed_full.decompress();
        assert_eq!(safe_serialize_key(&decompressed_full), prepared.sns_pk);

        // Compressed stripped sks_key has the same NS slots zeroed and,
        // once decompressed and re-serialized, matches the legacy
        // stripped result.
        let compressed_stripped: CompressedServerKey =
            fhevm_engine_common::utils::safe_deserialize_key(
                &sks_key_compressed,
            )?;
        let (
            _csks,
            _ckskm,
            _ccompression_key,
            _cdecompression_key,
            compressed_noise_squashing_key,
            compressed_noise_squashing_compression_key,
            compressed_re_randomization_key,
            _coprf_key,
            _ctag,
        ) = compressed_stripped.clone().into_raw_parts();
        assert!(compressed_noise_squashing_key.is_none());
        assert!(compressed_noise_squashing_compression_key.is_none());
        assert!(compressed_re_randomization_key.is_some());
        let decompressed_stripped = compressed_stripped.decompress();
        assert_eq!(
            safe_serialize_key(&decompressed_stripped),
            prepared.sks_key
        );

        // The legacy stripped ServerKey also has the expected NS layout.
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
