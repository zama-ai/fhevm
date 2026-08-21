//! The settled chain-id derivation, pinned against the public cluster registry.
//!
//! `zama-solana-chain-id-v1` is a deployment-time rule: it runs once per cluster when
//! a deployment is configured, and no running component ever recomputes it — they read
//! the chain id from configuration and check only the chain-kind bit. That makes the
//! fixture generator the one place in this repository that plays the deployer, and
//! this file the one place the rule itself is pinned.
//!
//! The registry literals below are copied from the KMS linker vector set, which froze
//! the rule and the public clusters' ids first. Byte parity with that set is the whole
//! point: a permit signed on a cluster must link on the same cluster, so the two
//! halves of the specification's fixture set have to agree on every derived id.

mod common;

use common::{bytes32, derive_chain_id, CHAIN_ID, GENESIS_HASH_HEX};

/// A cluster of the public registry: its genesis hash and the chain id the settled
/// rule assigns it.
const PUBLIC_CLUSTER_REGISTRY: [(&str, &str, u64); 3] = [
    (
        "devnet",
        "ce59db5080fc2c6d3bcf7ca90712d3c2e5e6c28f27f0dfbb9953bdb0894c03ab",
        13_493_519_385_758_132_576,
    ),
    (
        "mainnet-beta",
        "45296998a6f8e2a784db5d9f95e18fc23f70441a1039446801089879b08c7ef0",
        14_494_253_591_356_479_929,
    ),
    (
        "testnet",
        "3a132ece10305ec1830725502fa2b7e7eb8157e9123d4c1f654a71787161dc21",
        18_243_892_879_379_718_198,
    ),
];

/// Every public cluster derives to the id the registry records. A mismatch here means
/// this side and the KMS side would sign and link under different chain ids on the
/// same cluster — permits dead on arrival, silently.
#[test]
fn the_settled_derivation_reproduces_the_public_cluster_registry() {
    for (cluster, genesis_hex, chain_id) in PUBLIC_CLUSTER_REGISTRY {
        assert_eq!(
            derive_chain_id(&bytes32(genesis_hex)),
            chain_id,
            "chain id of {cluster}"
        );
    }
}

/// The fixture chain id is the derivation applied to the fixture genesis hash — the
/// constant is a literal so drift surfaces as this assertion, not as a rewritten
/// golden, but it must never be a value the rule cannot produce.
#[test]
fn the_fixture_chain_id_is_derived_from_the_fixture_genesis() {
    assert_eq!(derive_chain_id(&bytes32(GENESIS_HASH_HEX)), CHAIN_ID);
}

/// The chain-kind bit is forced, not observed: every derived id carries it, including
/// ids whose digest happened to have it clear.
#[test]
fn every_derived_chain_id_carries_the_chain_kind_bit() {
    for seed in 0u8..=255 {
        let genesis = [seed; 32];
        assert_ne!(
            derive_chain_id(&genesis) & 0x8000_0000_0000_0000,
            0,
            "genesis [{seed}; 32] derived an id without the chain-kind bit"
        );
    }
}
