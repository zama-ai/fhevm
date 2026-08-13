//! The v0 (RFC-021 ed25519) Solana user-decrypt surface is gone from the relayer, provably.
//!
//! The relayer used to carry Solana auth fields as optional EVM-payload fields, route them through
//! a `SolanaUnifiedV1` variant, and build the named `userDecryptionRequestSolana` gateway calldata.
//! Block 6 replaces all of that with the host-generic `solana-srfc38-user-decrypt-v1` path. A
//! survived reference keeps the dead surface alive silently, so this gate scans the relayer sources
//! for the symbols that named it and fails while any of them exists.
//!
//! The retired attestation *tag* (`solana-ed25519-user-decrypt-v2`) is deliberately NOT scanned
//! for: it lives on as a rejection assertion in `validate_v3_attestation_type`'s tests, which is a
//! stronger statement than its absence — the relayer actively refuses it.

use std::fs;
use std::path::{Path, PathBuf};

/// The symbols that named the v0 Solana user-decrypt surface, each with the reason it must die.
const FORBIDDEN: &[(&str, &str)] = &[
    (
        "SolanaUnifiedV1",
        "the v0 core request variant; replaced by the host-generic SolanaSrfc38V1",
    ),
    (
        "solana_unified_v1",
        "the v0 request builder; replaced by TryFrom<SolanaAttestedUserDecryptRequestJson>",
    ),
    (
        "userDecryptionRequestSolana",
        "the v0 named gateway calldata; replaced by the host-generic userDecryptionRequest overload",
    ),
    (
        "UserDecryptionRequestSolana",
        "the v0 named gateway event; the host-generic event is a UserDecryptionRequest overload",
    ),
    (
        "solana_user_identity",
        "a v0 typed Solana field on the EVM payload; the permit carries the identity now",
    ),
    (
        "solana_nonce",
        "the v0 per-request nonce; replay is bounded by the permit validity window",
    ),
    (
        "solana_allowed_acl_domain_keys",
        "a v0 typed Solana field on the EVM payload; the ACL scope rides in the permit",
    ),
];

fn rust_sources(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).expect("relayer source dir is readable") {
        let path = entry.expect("dir entry is readable").path();
        if path.is_dir() {
            rust_sources(&path, out);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            out.push(path);
        }
    }
}

#[test]
fn the_v0_solana_user_decrypt_surface_is_gone() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut sources = Vec::new();
    rust_sources(&src, &mut sources);
    assert!(
        sources.len() > 20,
        "the gate found suspiciously few sources ({}) — did the crate layout move?",
        sources.len()
    );

    let mut survivors = Vec::new();
    for path in &sources {
        let text = fs::read_to_string(path).expect("source file is readable");
        for (token, reason) in FORBIDDEN {
            for (index, line) in text.lines().enumerate() {
                if line.contains(token) {
                    survivors.push(format!(
                        "{}:{} carries `{token}` — {reason}",
                        path.display(),
                        index + 1
                    ));
                }
            }
        }
    }

    assert!(
        survivors.is_empty(),
        "the v0 Solana user-decrypt surface is still alive in the relayer:\n{}",
        survivors.join("\n")
    );
}
