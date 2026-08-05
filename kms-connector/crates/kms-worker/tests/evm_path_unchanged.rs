//! Regression: the EVM user-decryption path is byte-identical after this series.
//!
//! The Solana work is additive by construction — a new module tree beside the EVM one — but the two
//! paths share code that neither owns: the request payload types, the `extraData` container, and the
//! signing-struct definition the EIP-712 digest is computed from. A "harmless" edit to any of those
//! moves the digest that EVM wallets sign, and a moved digest is not a test failure somewhere: it is
//! every EVM user-decryption request in production failing signature verification, and, on the KMS
//! side, parties on two versions producing different digests and failing to aggregate.
//!
//! So the values below are frozen as literals. Their authority is not that they were independently
//! derived — it is that they are what the shipped implementation produces today, which is what
//! wallets already sign and what the SDK already reproduces. The test's job is to notice movement,
//! not to prove correctness: if a change in this series alters one of them, the change has to be a
//! decision rather than a side effect.
//!
//! Frozen here:
//!
//! * the EIP-712 digest of a fixed user-decryption payload against a fixed domain — the composition
//!   of the struct definition, the type hash, the field encoding and the domain separator;
//! * the EIP-712 type string of the signed struct, which pins the field names and their order on
//!   their own, so a rename that happens to preserve one digest still fails;
//! * the domain name and version, and the ERC-1271 magic value;
//! * the `extraData` container: the bytes the EVM path emits for a context/epoch pair, and how the
//!   EVM versions parse. The Solana permit signs the same `0x02` container, so this is the one place
//!   where a Solana-motivated change could reach the EVM path directly.

use alloy::{
    primitives::{Address, Bytes, U256},
    sol_types::SolStruct,
};
use connector_utils::types::extra_data::{
    EXTRA_DATA_V0_VERSION, EXTRA_DATA_V1_VERSION, EXTRA_DATA_V2_VERSION, extra_data_v2_payload,
    parse_extra_data,
};
use fhevm_gateway_bindings::decryption::IDecryption::{
    RequestValiditySeconds, UserDecryptionRequestPayload,
};
use user_decryption_signature::{
    DEFAULT_DOMAIN_NAME, DEFAULT_DOMAIN_VERSION, ERC1271_MAGIC_VALUE,
    UserDecryptRequestVerification, compute_user_decrypt_digest, default_user_decrypt_domain,
};

/// The host chain id of the frozen fixture. An EVM chain id: the chain-kind high bit is clear.
const CHAIN_ID: u64 = 31_337;

/// The Gateway `Decryption` contract of the frozen fixture.
fn verifying_contract() -> Address {
    Address::from([0xca; 20])
}

/// The payload whose digest is frozen below.
///
/// Every field is populated, including a non-empty `extraData` and more than one allowed contract:
/// a fixture with empty dynamic fields would not notice a change in how they are encoded.
fn frozen_payload() -> UserDecryptionRequestPayload {
    UserDecryptionRequestPayload {
        userAddress: Address::from([0x11; 20]),
        publicKey: Bytes::from(vec![0xaa, 0xbb, 0xcc, 0xdd]),
        allowedContracts: vec![Address::from([0x22; 20]), Address::from([0x33; 20])],
        requestValidity: RequestValiditySeconds {
            startTimestamp: U256::from(1_700_000_000_u64),
            durationSeconds: U256::from(86_400_u64),
        },
        extraData: Bytes::from(extra_data_v2_payload(U256::from(7_u64), U256::from(9_u64))),
        signature: Bytes::default(),
    }
}

/// The digest an EVM wallet signs for the frozen payload. Moving this breaks every outstanding EVM
/// user-decryption signature.
#[test]
fn the_user_decrypt_digest_of_the_frozen_payload_is_unchanged() {
    let domain = default_user_decrypt_domain(CHAIN_ID, verifying_contract());

    let digest = compute_user_decrypt_digest(&frozen_payload(), &domain);

    assert_eq!(
        format!("0x{}", alloy::hex::encode(digest)),
        "0x20126b95a5627c50d96326d856f1b74015c552d1c48b7a7d9b3da939c84b7a95",
        "the EIP-712 digest of an unchanged EVM payload moved"
    );
}

/// The signed struct's field names and order, on their own.
///
/// The digest above would also catch a change here, but not legibly: this assertion says which field
/// moved, and it fails for a rename that leaves the encoding accidentally intact. The SDK builds its
/// signing payload from this same type string, so the two must not drift.
#[test]
fn the_signed_struct_type_string_is_unchanged() {
    assert_eq!(
        UserDecryptRequestVerification::eip712_encode_type(),
        "UserDecryptRequestVerification(address userAddress,bytes publicKey,address[] \
         allowedContracts,uint256 startTimestamp,uint256 durationSeconds,bytes extraData)",
        "the EIP-712 type of the signed request struct moved"
    );
}

/// The domain separator's inputs, and the ERC-1271 magic value the contract-signer path compares
/// against. Constants, and frozen as constants: a deployment whose domain name differs would make
/// every wallet signature for it unverifiable.
#[test]
fn the_domain_constants_and_the_erc1271_magic_value_are_unchanged() {
    assert_eq!(DEFAULT_DOMAIN_NAME, "Decryption");
    assert_eq!(DEFAULT_DOMAIN_VERSION, "1");
    assert_eq!(ERC1271_MAGIC_VALUE, [0x16, 0x26, 0xba, 0x7e]);
}

/// The `extraData` container the EVM path emits for a KMS context and epoch.
///
/// This is the one structure the Solana work touches directly: the permit signs the same `0x02`
/// container. A change to its layout for the benefit of the Solana path would silently change what
/// EVM requests carry — and, because the container is inside the signed struct, what EVM wallets
/// sign.
#[test]
fn the_extra_data_container_of_a_context_and_epoch_is_unchanged() {
    let payload = extra_data_v2_payload(U256::from(7_u64), U256::from(9_u64));

    assert_eq!(
        format!("0x{}", alloy::hex::encode(&payload)),
        "0x02000000000000000000000000000000000000000000000000000000000000000700000000000000000000000\
         00000000000000000000000000000000000000009",
        "the bytes an EVM request carries for a context/epoch pair moved"
    );
    assert_eq!(payload[0], EXTRA_DATA_V2_VERSION);
}

/// How the EVM container versions parse. The Solana path added a version to this dispatch; the
/// existing versions must come out of it exactly as before, including the absent-context case that
/// keeps older relayers working.
#[test]
fn the_evm_extra_data_versions_still_parse_as_before() {
    let empty = parse_extra_data(&[]).expect("an empty container is legal");
    assert_eq!(empty.context_id, None, "no container means no context");
    assert_eq!(empty.epoch_id, None);

    let v0 = parse_extra_data(&[EXTRA_DATA_V0_VERSION]).expect("version zero is legal");
    assert_eq!(v0.context_id, None);
    assert_eq!(v0.epoch_id, None);

    let mut v1 = vec![EXTRA_DATA_V1_VERSION];
    v1.extend_from_slice(&U256::from(7_u64).to_be_bytes::<32>());
    let v1 = parse_extra_data(&v1).expect("version one is legal");
    assert_eq!(v1.context_id, Some(U256::from(7_u64)));
    assert_eq!(v1.epoch_id, None, "version one carries no epoch");

    let v2 = parse_extra_data(&extra_data_v2_payload(U256::from(7_u64), U256::from(9_u64)))
        .expect("version two is legal");
    assert_eq!(v2.context_id, Some(U256::from(7_u64)));
    assert_eq!(v2.epoch_id, Some(U256::from(9_u64)));
}
