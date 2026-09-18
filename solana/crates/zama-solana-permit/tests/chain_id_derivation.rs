//! The settled chain-id encoding, pinned against the public cluster registry.
//!
//! Type byte `0x01` plus the first seven bytes of genesis. No running component
//! recomputes it — they read the chain id from configuration and check the type byte.

mod common;

use common::{bytes32, derive_chain_id, CHAIN_ID, GENESIS_HASH_HEX};

/// A cluster of the public registry: its genesis hash and the encoded chain id.
const PUBLIC_CLUSTER_REGISTRY: [(&str, &str, u64); 3] = [
    (
        "devnet",
        "ce59db5080fc2c6d3bcf7ca90712d3c2e5e6c28f27f0dfbb9953bdb0894c03ab",
        0x01ce_59db_5080_fc2c,
    ),
    (
        "mainnet-beta",
        "45296998a6f8e2a784db5d9f95e18fc23f70441a1039446801089879b08c7ef0",
        0x0145_2969_98a6_f8e2,
    ),
    (
        "testnet",
        "3a132ece10305ec1830725502fa2b7e7eb8157e9123d4c1f654a71787161dc21",
        0x013a_132e_ce10_305e,
    ),
];

#[test]
fn the_settled_encoding_reproduces_the_public_cluster_registry() {
    for (cluster, genesis_hex, chain_id) in PUBLIC_CLUSTER_REGISTRY {
        assert_eq!(
            derive_chain_id(&bytes32(genesis_hex)),
            chain_id,
            "chain id of {cluster}"
        );
    }
}

#[test]
fn the_fixture_chain_id_is_derived_from_the_fixture_genesis() {
    assert_eq!(derive_chain_id(&bytes32(GENESIS_HASH_HEX)), CHAIN_ID);
}

#[test]
fn every_derived_chain_id_has_solana_type_byte() {
    for seed in 0u8..=255 {
        let genesis = [seed; 32];
        assert_eq!(
            derive_chain_id(&genesis) >> 56,
            0x01,
            "genesis [{seed}; 32] derived an id without type byte 0x01"
        );
    }
}
