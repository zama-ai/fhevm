//! The topic hashes the listener subscribes by, pinned to their event signatures.
//!
//! The `eth_getLogs` filter is built from generated `SIGNATURE_HASH` constants, and the
//! generated names for overloaded events carry POSITIONAL suffixes (`UserDecryptionRequest_3`)
//! that renumber silently when the contract gains or loses an overload. A renumbered import
//! still compiles — the constant just names a different event — and the subscription quietly
//! stops matching. This suite turns that silence into a named failure: every hash the
//! decryption filter subscribes by is asserted against the keccak of a spelled-out event
//! signature, so a drifted suffix or a reshaped event fails here with both forms visible.
//!
//! The EVM signatures below are also this layer's EVM gate: the host-generic gateway overload
//! must not move them.

use alloy::primitives::{B256, keccak256};
use connector_utils::types::db::EventType;

/// Every event signature the user-decryption filter must subscribe by once this train
/// block lands: the two EVM shapes it serves today, plus the host-generic request event (a
/// `userDecryptionRequest` overload) that carries Solana requests. The list is compared in full
/// (order-insensitively), so a
/// topic appearing or disappearing fails with both lists visible.
const USER_DECRYPTION_SUBSCRIBED_SIGS: &[&str] = &[
    // The bytes32[] handles-only user-decryption shape the base binding name resolves to.
    "UserDecryptionRequest(uint256,bytes32[],address,bytes,bytes)",
    // The unified handles-only shape (RFC-023 Part 2); currently rides the positional
    // alias `UserDecryptionRequest_3`.
    "UserDecryptionRequest(uint256,(bytes32,address,address)[],(address,bytes,address[],(uint256,uint256),bytes,bytes))",
    // The host-generic request event (a `UserDecryptionRequest` overload), the one carrying
    // Solana requests.
    "UserDecryptionRequest(uint256,bytes32[],(uint256,uint256),bytes,uint8,uint8,bytes,bytes)",
];

fn sorted_hashes(sigs: &[&str]) -> Vec<B256> {
    let mut hashes: Vec<B256> = sigs.iter().map(|sig| keccak256(sig.as_bytes())).collect();
    hashes.sort();
    hashes
}

#[test]
fn the_user_decryption_subscription_pins_its_topics() {
    let mut subscribed = EventType::UserDecryptionRequest.signature_hashes();
    subscribed.sort();

    assert_eq!(
        subscribed,
        sorted_hashes(USER_DECRYPTION_SUBSCRIBED_SIGS),
        "the user-decryption subscription topics are not the pinned set; update the \
         spelled-out signatures deliberately or fix the drifted positional binding alias"
    );
}

#[test]
fn the_public_decryption_subscription_pins_its_topic() {
    // The public-decryption filter subscribes by exactly one topic; the literal pins which
    // of the overloaded shapes that is.
    let hash = EventType::PublicDecryptionRequest.signature_hash();

    assert_eq!(
        hash,
        keccak256("PublicDecryptionRequest(uint256,bytes32[],bytes)".as_bytes()),
        "the public-decryption subscription topic moved"
    );
}
