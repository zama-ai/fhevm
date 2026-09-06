//! The v0 Solana user-decrypt protocol is gone, provably.
//!
//! Deletion is the easiest change to lose in review: a survived call site keeps dead
//! protocol alive silently. This suite makes the teardown a test outcome instead of a
//! review checklist — it scans the connector sources for the tokens that name the v0
//! surface and fails while any of them exists. It is red by construction until the
//! teardown lands, and it stays in the tree afterwards so the surface cannot grow back.
//!
//! What is deliberately NOT forbidden: the `0x03` extraData container and the
//! public-decrypt surface that rides it. That carrier belongs to public decrypt (its
//! teardown is a separate, gateway-interface-owned step), and the public-decrypt path
//! survives the v0 user-decrypt teardown by being re-homed, not deleted.

use std::fs;
use std::path::{Path, PathBuf};

/// The tokens that name the v0 user-decrypt surface, each with the reason it must die.
const FORBIDDEN: &[(&str, &str)] = &[
    (
        "verify_solana_user_decrypt_signature",
        "the v0 ad-hoc signature verifier; replaced by permit reconstruction in the pipeline",
    ),
    (
        "solana_user_decrypt",
        "the v0 event-processor module (its public-decrypt tenants are re-homed, not deleted)",
    ),
    (
        "solana_nonce",
        "the v0 per-request nonce; permit revocation is a host-state watermark",
    ),
    (
        "verify_delegation",
        "the v0 delegation check; the pipeline reads delegation records via the snapshot",
    ),
    (
        "DelegationCounterMismatch",
        "the v0 delegation-counter failure; the counter does not participate in authorization",
    ),
    (
        "AccessEvidence",
        "the two-path request evidence (current membership or a client-supplied proof); there is one path, an allow leaf the connector fetches",
    ),
    (
        "access_proof",
        "the client-supplied inclusion proof; proofs come from the coprocessors' leaf record",
    ),
    (
        "proof_leaf_count",
        "the client-claimed leaf count the proof was built against; the record's own count is what is compared",
    ),
    (
        "authorize_current",
        "the current-membership branch; there is no current-handle rule",
    ),
    // Spelled in two halves so the repository-wide vocabulary sweep does not read this gate's own
    // token list as a survivor.
    (
        concat!("sub", "jects"),
        "the on-chain member list; access is a sealed allow leaf per (handle, key)",
    ),
    (
        concat!("compute_sub", "ject"),
        "the derived identity of a delegated entry; the entry names its allowed key directly",
    ),
    (
        "encrypted_value_id",
        "the derived identity an entry used to claim; the entry names the account address, which its fields must derive",
    ),
    (
        "allowed_acl_domain_keys",
        "the permit's domain list; the permit signs (program, scope) pairs",
    ),
    (
        "solana_v2_fetcher",
        "the request-carried proof fetcher; the pipeline reads proofs through HostProofReader",
    ),
    (
        "SolanaAclVerifier",
        "the v2 verifier facade; authorization is the pipeline",
    ),
];

/// Source trees the gate patrols. `kms-worker` hosts the processor; `gw-listener` and
/// `connector-utils` host the transport and the shared types the v0 columns leaked into.
fn patrolled_roots() -> Vec<PathBuf> {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let crates = manifest.parent().expect("kms-worker lives in crates/");
    vec![
        manifest.join("src"),
        crates.join("gw-listener").join("src"),
        crates.join("utils").join("src"),
    ]
}

fn rust_sources(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).expect("patrolled source dir is readable") {
        let path = entry.expect("dir entry is readable").path();
        if path.is_dir() {
            rust_sources(&path, out);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            out.push(path);
        }
    }
}

#[test]
fn the_v0_user_decrypt_surface_is_gone() {
    let mut sources = Vec::new();
    for root in patrolled_roots() {
        rust_sources(&root, &mut sources);
    }
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
        "the v0 Solana user-decrypt surface is still alive:\n{}",
        survivors.join("\n")
    );
}
