//! The normative leaf vectors: generator and runner in one place.
//!
//! The set is built in memory from the shared crate and compared against the committed
//! `solana/test-fixtures/leaves/leaves_v1.json`; the same build is written to disk when the
//! update gate is set (`bash scripts/update-leaf-vectors.sh`). A regeneration that changes a
//! committed byte is a protocol change: the coprocessor's stored leaves and every proof the KMS
//! has verified depend on these bytes.

#[path = "../../../test-fixtures/leaves/leaf_vectors.rs"]
mod schema;

use schema::{LeafEvent, LeafVector, LeafVectorFile, Prefixes, ProofVector, LEAF_VECTOR_SCHEMA};
use std::path::{Path, PathBuf};
use zama_solana_acl::{mmr_verify, reconstruct, EncryptedValueAccountEvent, MmrProof};

const UPDATE_ENV: &str = "ZAMA_UPDATE_LEAF_VECTORS";

fn vector_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../test-fixtures/leaves/leaves_v1.json")
}

fn h(tag: u8) -> [u8; 32] {
    [tag; 32]
}

fn hex32(bytes: &[u8; 32]) -> String {
    hex::encode(bytes)
}

fn from_hex32(s: &str) -> [u8; 32] {
    hex::decode(s).unwrap().try_into().unwrap()
}

fn to_event(event: &LeafEvent) -> EncryptedValueAccountEvent {
    match event {
        LeafEvent::Allowed { handle, key } => EncryptedValueAccountEvent::Allowed {
            handle: from_hex32(handle),
            key: from_hex32(key),
        },
        LeafEvent::MarkedPublic { handle } => EncryptedValueAccountEvent::MarkedPublic {
            handle: from_hex32(handle),
        },
    }
}

fn allowed(handle: u8, key: u8) -> LeafEvent {
    LeafEvent::Allowed {
        handle: hex32(&h(handle)),
        key: hex32(&h(key)),
    }
}

fn public(handle: u8) -> LeafEvent {
    LeafEvent::MarkedPublic {
        handle: hex32(&h(handle)),
    }
}

/// Fills in leaves, peaks and the proof for `proof_index` from the events, using the crate.
fn vector(
    id: &str,
    comment: &str,
    account: u8,
    events: Vec<LeafEvent>,
    proof_index: u64,
) -> LeafVector {
    let acct = h(account);
    let native: Vec<_> = events.iter().map(to_event).collect();
    let reconstructed = reconstruct(acct, &native);
    let proof = reconstructed.build_proof(proof_index).unwrap();
    LeafVector {
        id: id.to_string(),
        comment: comment.to_string(),
        encrypted_value_account: hex32(&acct),
        events,
        leaves: reconstructed.leaves.iter().map(hex32).collect(),
        leaf_count: reconstructed.leaf_count.to_string(),
        peaks: reconstructed.peaks.iter().map(hex32).collect(),
        proof: ProofVector {
            leaf_index: proof.leaf_index.to_string(),
            siblings: proof.siblings.iter().map(hex32).collect(),
        },
    }
}

fn build() -> LeafVectorFile {
    LeafVectorFile {
        schema: LEAF_VECTOR_SCHEMA.to_string(),
        description: "Leaf commitments, MMR peaks and one inclusion proof per encrypted value \
                      account history, as the zama-host program seals them. Every implementation \
                      that hashes ACL leaves must reproduce these bytes."
            .to_string(),
        regenerate_with: "bash scripts/update-leaf-vectors.sh".to_string(),
        hash: "keccak256".to_string(),
        prefixes: Prefixes {
            historical_access_leaf: "ZAMA_HIST_ACCESS_LEAF_V1".to_string(),
            public_decrypt_leaf: "ZAMA_PUBLIC_DECRYPT_LEAF_V1".to_string(),
            mmr_leaf_node: "ZAMA_MMR_LEAF_V1".to_string(),
            mmr_node: "ZAMA_MMR_NODE_V1".to_string(),
        },
        vectors: vec![
            vector(
                "single-allow",
                "One allow on the created handle: a one-leaf MMR whose proof has no siblings.",
                0xAC,
                vec![allowed(0x10, 0x01)],
                0,
            ),
            vector(
                "create-two-allows-then-public-update",
                "Create allowing two keys, then an update that allows one key and goes public: \
                 four leaves, two mountains, proof for the public leaf.",
                0xAC,
                vec![
                    allowed(0x10, 0x01),
                    allowed(0x10, 0x02),
                    allowed(0x11, 0x01),
                    public(0x11),
                ],
                3,
            ),
            vector(
                "same-events-other-account",
                "The previous history under another account address: every leaf differs, \
                 because the account is bound into each commitment.",
                0xBB,
                vec![
                    allowed(0x10, 0x01),
                    allowed(0x10, 0x02),
                    allowed(0x11, 0x01),
                    public(0x11),
                ],
                3,
            ),
            vector(
                "seventeen-allows",
                "Seventeen allows across three handles: peaks of heights 4 and 0, proof for a \
                 leaf deep in the first mountain.",
                0xAC,
                (0..17u8).map(|i| allowed(0x10 + i / 8, 0x20 + i)).collect(),
                5,
            ),
        ],
    }
}

#[test]
fn committed_vectors_match_the_generator() {
    let built = build();
    let path = vector_path();
    let rendered = serde_json::to_string_pretty(&built).unwrap() + "\n";
    if std::env::var_os(UPDATE_ENV).is_some() {
        std::fs::write(&path, &rendered).unwrap();
        return;
    }
    let committed = std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "{}: {e}; run `bash scripts/update-leaf-vectors.sh`",
            path.display()
        )
    });
    assert_eq!(
        committed, rendered,
        "committed leaf vectors differ from the crate; a protocol change must regenerate them"
    );
}

/// The runner every implementation mirrors: recompute leaves and peaks from the events and
/// verify the proof against the peaks.
#[test]
fn every_committed_vector_recomputes_and_verifies() {
    let file: LeafVectorFile =
        serde_json::from_str(&std::fs::read_to_string(vector_path()).unwrap()).unwrap();
    assert_eq!(file.schema, LEAF_VECTOR_SCHEMA);
    assert_eq!(file.hash, "keccak256");
    for v in &file.vectors {
        let acct = from_hex32(&v.encrypted_value_account);
        let events: Vec<_> = v.events.iter().map(to_event).collect();
        let reconstructed = reconstruct(acct, &events);
        let leaves: Vec<_> = v.leaves.iter().map(|s| from_hex32(s)).collect();
        let peaks: Vec<_> = v.peaks.iter().map(|s| from_hex32(s)).collect();
        assert_eq!(reconstructed.leaves, leaves, "{}", v.id);
        assert_eq!(reconstructed.peaks, peaks, "{}", v.id);
        assert_eq!(
            reconstructed.leaf_count.to_string(),
            v.leaf_count,
            "{}",
            v.id
        );
        let proof = MmrProof {
            leaf_index: v.proof.leaf_index.parse().unwrap(),
            siblings: v.proof.siblings.iter().map(|s| from_hex32(s)).collect(),
        };
        let leaf = leaves[proof.leaf_index as usize];
        assert!(
            mmr_verify(&peaks, reconstructed.leaf_count, leaf, &proof),
            "{}",
            v.id
        );
    }
}
