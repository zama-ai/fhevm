use super::{tests::payload, *};
use alloy_signer_local::PrivateKeySigner;

fn refresh_digests(payload: &mut ManifestPayload) {
    for block in &mut payload.detailed_range.blocks {
        block.block_content_digest = block_content_digest(
            payload.version,
            payload.coprocessor_context_id,
            payload.host_chain_id,
            block.block_number,
            block.block_hash,
            &block.ciphertexts,
        )
        .unwrap();
    }
    payload.detailed_range.digest = detailed_range_digest(
        payload.version,
        payload.coprocessor_context_id,
        payload.host_chain_id,
        payload.detailed_range.first_block_number,
        payload.detailed_range.last_block_number,
        &payload
            .detailed_range
            .blocks
            .iter()
            .map(|block| block.block_content_digest)
            .collect::<Vec<_>>(),
    );
}

#[test]
fn epoch_length_is_bounded_in_utf8_bytes() {
    for epoch in ["a".repeat(256), "é".repeat(128)] {
        let mut manifest = payload(Address::ZERO);
        manifest.consensus_epoch = epoch.clone();
        manifest.validate().unwrap();
        manifest.consensus_epoch.push('a');
        assert!(
            manifest
                .validate()
                .unwrap_err()
                .to_string()
                .contains("too long")
        );
    }
}

fn historical_payload() -> ManifestPayload {
    let mut manifest = payload(Address::ZERO);
    manifest.historical_ranges = vec![
        HistoricalRange {
            start_block_number: U256::from(40),
            end_block_number: U256::from(41),
            scale: 1,
            end_block_hash: manifest.publication_parent_block_hash,
            digest: B256::repeat_byte(1),
        },
        HistoricalRange {
            start_block_number: U256::from(36),
            end_block_number: U256::from(39),
            scale: 2,
            end_block_hash: B256::repeat_byte(0xa7),
            digest: B256::repeat_byte(2),
        },
    ];
    manifest
}

#[test]
fn full_and_oldest_truncated_history_are_valid() {
    let mut manifest = historical_payload();
    manifest.validate().unwrap();
    // The first range stays full; the second is the actual truncation under test.
    for oldest_start in [37, 38, 39] {
        manifest.historical_ranges[1].start_block_number = U256::from(oldest_start);
        manifest.validate().unwrap();
    }
    // Validation checks geometry, not the opaque signed historical roots.
    manifest.historical_ranges[0].digest = B256::repeat_byte(0xee);
    manifest.validate().unwrap();
}

#[test]
fn history_rejects_truncation_gaps_wrong_scale_and_boundary_hash() {
    for case in [
        "non-oldest truncation",
        "gap",
        "oversized oldest",
        "wrong scale",
        "boundary hash",
    ] {
        let mut manifest = historical_payload();
        match case {
            "non-oldest truncation" => {
                manifest.historical_ranges[0].start_block_number = U256::from(41)
            }
            "gap" => manifest.historical_ranges[1].end_block_number = U256::from(38),
            "oversized oldest" => manifest.historical_ranges[1].start_block_number = U256::from(35),
            "wrong scale" => manifest.historical_ranges[1].scale = 1,
            "boundary hash" => manifest.historical_ranges[0].end_block_hash = B256::ZERO,
            _ => unreachable!(),
        }
        assert!(manifest.validate().is_err(), "{case}");
    }
}

#[test]
fn detailed_range_requires_contiguous_numbers_and_parent_hashes() {
    let mut manifest = payload(Address::ZERO);
    let mut next = manifest.detailed_range.blocks[0].clone();
    next.block_number += U256::ONE;
    next.parent_block_hash = next.block_hash;
    next.block_hash = B256::repeat_byte(0xab);
    manifest.publication_block_number = next.block_number;
    manifest.publication_block_hash = next.block_hash;
    manifest.publication_parent_block_hash = next.parent_block_hash;
    manifest.detailed_range.last_block_number = next.block_number;
    manifest.detailed_range.blocks.push(next);
    refresh_digests(&mut manifest);
    manifest.validate().unwrap();
    for change_number in [true, false] {
        let mut broken = manifest.clone();
        if change_number {
            broken.detailed_range.blocks[1].block_number += U256::ONE;
            broken.publication_block_number += U256::ONE;
            broken.detailed_range.last_block_number += U256::ONE;
        } else {
            broken.detailed_range.blocks[1].parent_block_hash = B256::ZERO;
            broken.publication_parent_block_hash = B256::ZERO;
        }
        // Keep all digests and publication fields valid to isolate the lineage check.
        refresh_digests(&mut broken);
        assert_eq!(
            broken.validate(),
            Err(invalid("detailed range is not one contiguous lineage"))
        );
    }
}

#[tokio::test]
async fn verify_rejects_wrong_signer_and_malformed_signature() {
    let signer = PrivateKeySigner::random();
    let other = PrivateKeySigner::random();
    assert!(payload(signer.address()).sign(&other).await.is_err());
    let signed = payload(signer.address()).sign(&signer).await.unwrap();
    let mut wrong_signer = signed.clone();
    wrong_signer.signature = other
        .sign_hash(&signed.digest().unwrap())
        .await
        .unwrap()
        .as_bytes()
        .to_vec();
    assert!(matches!(
        wrong_signer.verify(),
        Err(ManifestError::SignerMismatch { .. })
    ));
    for signature in [vec![], vec![0; 64], vec![0; 66]] {
        let mut malformed = signed.clone();
        malformed.signature = signature;
        assert!(matches!(
            malformed.verify(),
            Err(ManifestError::MalformedSignature(_))
        ));
    }
}

#[tokio::test]
async fn verify_rejects_tampered_content_and_signed_provenance() {
    let signer = PrivateKeySigner::random();
    let signed = payload(signer.address()).sign(&signer).await.unwrap();
    for field in ["revision", "epoch", "ciphertext", "gateway key"] {
        let mut tampered = signed.clone();
        match field {
            "revision" => tampered.payload.revision += 1,
            "epoch" => tampered.payload.consensus_epoch.push_str("/other"),
            "ciphertext" | "gateway key" => {
                let CiphertextStatus::Computed {
                    ct64_digest,
                    gateway_key_id,
                    ..
                } = &mut tampered.payload.detailed_range.blocks[0].ciphertexts[0].status
                else {
                    unreachable!()
                };
                if field == "ciphertext" {
                    *ct64_digest = B256::ZERO;
                } else {
                    *gateway_key_id = None;
                }
                refresh_digests(&mut tampered.payload);
            }
            _ => unreachable!(),
        }
        tampered.payload.validate().unwrap();
        assert!(
            matches!(tampered.verify(), Err(ManifestError::SignerMismatch { .. })),
            "{field}"
        );
    }
    let mut error_payload = payload(signer.address());
    error_payload.detailed_range.blocks[0].ciphertexts =
        vec![BlockCiphertextDescriptor::from_computation_error(
            B256::repeat_byte(1),
            Some("original".into()),
        )];
    refresh_digests(&mut error_payload);
    let mut signed_error = error_payload.sign(&signer).await.unwrap();
    let CiphertextStatus::Error { error_message } =
        &mut signed_error.payload.detailed_range.blocks[0].ciphertexts[0].status
    else {
        unreachable!()
    };
    *error_message = Some("rewritten".into());
    signed_error.payload.validate().unwrap();
    assert!(matches!(
        signed_error.verify(),
        Err(ManifestError::SignerMismatch { .. })
    ));
}

#[tokio::test]
async fn json_envelope_pins_signature_shape_and_rejects_unknown_version() {
    let signer = PrivateKeySigner::random();
    let signed = payload(signer.address()).sign(&signer).await.unwrap();
    let mut json = serde_json::to_value(&signed).unwrap();
    assert!(json.get("payload").is_none());
    assert_eq!(json["version"], 1);
    let signature = json["signature"].as_str().unwrap();
    assert!(signature.starts_with("0x"));
    let bytes = hex::decode(&signature[2..]).unwrap();
    assert_eq!(bytes.len(), 65);
    assert!(matches!(bytes[64], 27 | 28));
    for version in [0, 2, 255] {
        json["version"] = version.into();
        let error = serde_json::from_value::<SignedManifest>(json.clone()).unwrap_err();
        assert!(error.to_string().contains("unsupported manifest version"));
    }
}

#[test]
fn error_and_uncomputed_digest_vectors_are_pinned() {
    // Fixed block 42 / chain 7 / context 1, handle 0x01..01, publisher zero.
    // Pin both the consensus commitment and signed-payload transcript.
    for (descriptor, content, signed_payload) in [
        (
            BlockCiphertextDescriptor::from_computation_error(
                B256::repeat_byte(1),
                Some("failure".into()),
            ),
            alloy_primitives::b256!(
                "969303e135fef608b29459537f04b9a955ad06d89f9f80815fb19fddd3c6f623"
            ),
            alloy_primitives::b256!(
                "e6862fda47ea38ec13d672d282ac0292d728e51dc476aca3ffa85f2a2f533080"
            ),
        ),
        (
            BlockCiphertextDescriptor::from_uncomputed(B256::repeat_byte(1)),
            alloy_primitives::b256!(
                "394937584792ca039fa478f6efbb63f6562541f647ae66873e00c2d3f7b5e60c"
            ),
            alloy_primitives::b256!(
                "15400eb7ebe25da4a3378d56ff365bde7c2c1a192198b725f7c58099e8ead0d3"
            ),
        ),
    ] {
        let mut manifest = payload(Address::ZERO);
        manifest.detailed_range.blocks[0].ciphertexts = vec![descriptor];
        refresh_digests(&mut manifest);
        assert_eq!(
            manifest.detailed_range.blocks[0].block_content_digest,
            content
        );
        assert_eq!(manifest.canonical_digest().unwrap(), signed_payload);
    }
}
