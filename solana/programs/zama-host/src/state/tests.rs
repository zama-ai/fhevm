//! Unit tests for state helpers and keccak256 handle derivation.
//!
//! Split out of `mod.rs` to keep that module within its form-gate size cap.

use super::*;

#[test]
fn latest_prior_bank_hash_tolerates_skipped_slots() {
    let entries = vec![(8, [8; 32]), (6, [6; 32]), (3, [3; 32])];

    assert_eq!(
        latest_prior_bank_hash_from_entries(10, entries.clone()),
        Some([8; 32])
    );
    assert_eq!(
        latest_prior_bank_hash_from_entries(8, entries.clone()),
        Some([6; 32])
    );
    assert_eq!(latest_prior_bank_hash_from_entries(3, entries), None);
}

fn host_config_with(
    chain_id: u64,
    test_shims_enabled: bool,
    mock_input_enabled: bool,
) -> HostConfig {
    HostConfig {
        admin: Pubkey::default(),
        chain_id,
        input_verifier_authority: Pubkey::default(),
        gateway_chain_id: 0,
        input_verification_contract: [0u8; 20],
        coprocessor_signer: [0u8; 20],
        decryption_contract: [0u8; 20],
        current_kms_context_id: 0,
        material_authority: Pubkey::default(),
        test_authority: Pubkey::default(),
        paused: false,
        mock_input_enabled,
        test_shims_enabled,
        grant_deny_list_enabled: false,
        updated_slot: 0,
        bump: 0,
    }
}

#[test]
fn zero_birth_entropy_requires_poc_chain_and_test_shims() {
    // Local PoC chain with the shim flag: relaxation allowed.
    assert!(host_config_with(SOLANA_POC_CHAIN_ID, true, false).zero_birth_entropy_allowed());
    // Local PoC chain without the shim flag: fails closed (drives the
    // PreviousBankHashUnavailable negative test).
    assert!(!host_config_with(SOLANA_POC_CHAIN_ID, false, false).zero_birth_entropy_allowed());
    // A deployed chain can NEVER zero birth entropy, even with the shim flag
    // toggled on by an admin — this is the security boundary.
    assert!(!host_config_with(101, true, false).zero_birth_entropy_allowed());
    assert!(!host_config_with(1, true, false).zero_birth_entropy_allowed());
}

#[test]
fn mock_input_requires_poc_chain() {
    assert!(host_config_with(SOLANA_POC_CHAIN_ID, false, true).mock_input_allowed());
    assert!(!host_config_with(SOLANA_POC_CHAIN_ID, false, false).mock_input_allowed());
    // A deployed chain can never run the mock input bind path, even if an
    // admin sets mock_input_enabled.
    assert!(!host_config_with(101, false, true).mock_input_allowed());
}

/// Bit 63 of the uint64 chain id is the reserved Solana `chain_type` marker
/// (RFC-021 / #1494). It is derived from the chain id, never a schema column.
const SOLANA_CHAIN_TYPE_BIT: u64 = 1 << 63;

fn keccak(parts: &[&[u8]]) -> [u8; 32] {
    keccak_hashv(parts).to_bytes()
}

fn assert_canonical_metadata(handle: [u8; 32], fhe_type: u8, chain_id: u64) {
    assert_eq!(
        handle[21], COMPUTED_HANDLE_MARKER,
        "byte 21 must mark a computed handle"
    );
    assert_eq!(
        &handle[22..30],
        &chain_id.to_be_bytes(),
        "bytes 22..30 must carry the big-endian uint64 chain id"
    );
    assert_eq!(handle[30], fhe_type, "byte 30 must carry the fhe type");
    assert_eq!(
        handle[31], HANDLE_VERSION,
        "byte 31 must carry the handle version"
    );
}

#[test]
fn handle_derivation_binary_uses_keccak_prehandle_with_canonical_metadata() {
    let lhs = [0x11; 32];
    let rhs = [0x22; 32];
    let chain_id = 101u64;
    let prev = [0x33; 32];
    let timestamp = 1_700_000_000i64;
    let fhe_type = 4u8;

    let handle = computed_binary_handle(
        FheBinaryOpCode::Add,
        lhs,
        rhs,
        false,
        fhe_type,
        chain_id,
        prev,
        timestamp,
    );

    // The first 21 bytes must equal the keccak256 prehandle over the exact
    // EVM-mirrored preimage (FHEVMExecutor._binaryOp), proving sha256 is gone.
    let expected_prehandle = keccak(&[
        COMPUTATION_DOMAIN_SEPARATOR,
        &[FheBinaryOpCode::Add.as_u8()],
        &lhs,
        &rhs,
        &[0u8],
        crate::ID.as_ref(),
        &chain_id.to_be_bytes(),
        &prev,
        &timestamp.to_be_bytes(),
    ]);
    assert_eq!(
        &handle[..21],
        &expected_prehandle[..21],
        "prehandle bytes must be keccak256 of the canonical preimage"
    );
    assert_canonical_metadata(handle, fhe_type, chain_id);
}

#[test]
fn handle_derivation_preserves_solana_chain_type_high_bit() {
    // A Solana host chain id sets the reserved high bit; the derived handle
    // must round-trip it verbatim through bytes 22..30.
    let chain_id = SOLANA_CHAIN_TYPE_BIT | SOLANA_POC_CHAIN_ID;
    let handle = computed_binary_handle(
        FheBinaryOpCode::Sub,
        [1; 32],
        [2; 32],
        true,
        3,
        chain_id,
        [9; 32],
        42,
    );
    assert_canonical_metadata(handle, 3, chain_id);
    assert_eq!(
        handle_chain_id(handle),
        chain_id,
        "the chain id reader must recover the high-bit chain type"
    );
    assert_ne!(
        handle_chain_id(handle) & SOLANA_CHAIN_TYPE_BIT,
        0,
        "the Solana chain-type bit must survive derivation"
    );
}

#[test]
fn handle_derivation_trivial_uses_keccak() {
    let plaintext = [0x55; 32];
    let chain_id = 7u64;
    let prev = [0x66; 32];
    let timestamp = 99i64;
    let nonce_key = [0x77; 32];
    let sequence = 5u64;
    let fhe_type = 2u8;

    let handle = computed_trivial_handle(
        plaintext, fhe_type, chain_id, prev, timestamp, nonce_key, sequence,
    );

    let expected_prehandle = keccak(&[
        COMPUTATION_DOMAIN_SEPARATOR,
        &[2],
        &plaintext,
        &[fhe_type],
        crate::ID.as_ref(),
        &chain_id.to_be_bytes(),
        &prev,
        &timestamp.to_be_bytes(),
        &nonce_key,
        &sequence.to_be_bytes(),
    ]);
    assert_eq!(&handle[..21], &expected_prehandle[..21]);
    assert_canonical_metadata(handle, fhe_type, chain_id);
}

#[test]
fn handle_derivation_bound_output_rehashes_low_bytes_with_keccak() {
    let base = computed_binary_handle(
        FheBinaryOpCode::Add,
        [1; 32],
        [2; 32],
        false,
        4,
        101,
        [3; 32],
        10,
    );
    let nonce_key = [0xAB; 32];
    let sequence = 12u64;
    let bound = computed_bound_binary_handle(
        FheBinaryOpCode::Add,
        [1; 32],
        [2; 32],
        false,
        4,
        101,
        [3; 32],
        10,
        nonce_key,
        sequence,
    );

    let expected_low = keccak(&[
        b"FHE_bound_output",
        &base,
        &nonce_key,
        &sequence.to_be_bytes(),
    ]);
    assert_eq!(
        &bound[..21],
        &expected_low[..21],
        "bound output must keccak the base handle with the output nonce"
    );
    // The metadata tail (bytes 21..32) is untouched by binding.
    assert_eq!(&bound[21..], &base[21..]);
    assert_canonical_metadata(bound, 4, 101);
}

#[test]
fn bytes32_roundtrip_preserves_solana_pubkey() {
    // A Solana address is already a canonical 32-byte host-chain address and
    // must round-trip verbatim.
    let pubkey = Pubkey::new_from_array([0x5a; 32]);
    let canonical = HostChainAddress::from_pubkey(pubkey);
    assert_eq!(canonical.as_bytes(), &pubkey.to_bytes());
    assert_eq!(canonical.to_pubkey(), pubkey);
}

#[test]
fn bytes32_roundtrip_canonicalizes_evm_address() {
    // A 20-byte EVM address canonicalizes to a left-padded bytes32 (the
    // abi.encode(address) shape RFC-021 adopts) and round-trips back.
    let evm_address = [0xab; 20];
    let canonical = HostChainAddress::from_evm_address(evm_address);

    let mut expected = [0u8; 32];
    expected[12..].copy_from_slice(&evm_address);
    assert_eq!(
        canonical.as_bytes(),
        &expected,
        "EVM address is left-padded"
    );
    assert_eq!(
        &canonical.as_bytes()[..12],
        &[0u8; 12],
        "the leading 12 bytes are the zero pad"
    );

    assert_eq!(
        canonical.to_evm_address(),
        Some(evm_address),
        "a left-padded EVM address round-trips back to 20 bytes"
    );
}

#[test]
fn bytes32_roundtrip_rejects_non_evm_solana_address_as_evm() {
    // A Solana address with a nonzero high prefix is not an EVM address and
    // must not be mistaken for one.
    let pubkey = Pubkey::new_from_array([0x11; 32]);
    let canonical = HostChainAddress::from_pubkey(pubkey);
    assert_eq!(canonical.to_evm_address(), None);
}

#[test]
fn bytes32_roundtrip_aux_assembles_128_byte_layout() {
    // Mirrors the EVM zkproof-worker ZkData::assemble, widened from 92 to 128
    // bytes: contract(32) || user(32) || acl_contract(32) || chain_id(32).
    let contract = HostChainAddress::from_bytes([0x11; 32]);
    let user = HostChainAddress::from_bytes([0x22; 32]);
    let acl_contract = HostChainAddress::from_bytes([0x33; 32]);
    let chain_id = SOLANA_CHAIN_TYPE_BIT | SOLANA_POC_CHAIN_ID;

    let aux = HostChainAuxData {
        contract,
        user,
        acl_contract,
        chain_id,
    };
    let assembled = aux.assemble();

    assert_eq!(assembled.len(), HOST_CHAIN_AUX_LEN);
    assert_eq!(HOST_CHAIN_AUX_LEN, 128, "bytes32 surface widens 92 -> 128");
    assert_eq!(&assembled[..32], contract.as_bytes());
    assert_eq!(&assembled[32..64], user.as_bytes());
    assert_eq!(&assembled[64..96], acl_contract.as_bytes());

    // Chain id is the trailing 32-byte big-endian word; the high bit must
    // survive in the most-significant byte of its u64 little-end.
    let mut expected_chain_word = [0u8; 32];
    expected_chain_word[24..].copy_from_slice(&chain_id.to_be_bytes());
    assert_eq!(&assembled[96..], &expected_chain_word);
    assert_ne!(
        assembled[120] & 0x80,
        0,
        "the Solana chain-type high bit survives into the chain-id word"
    );
}

#[test]
fn bytes32_roundtrip_aux_matches_evm_field_order_for_padded_addresses() {
    // With EVM-style left-padded addresses the canonical aux must place each
    // 20-byte address in the low 20 bytes of its 32-byte word, mirroring the
    // EVM ZkData field order (contract, user, acl_contract, chain_id).
    let contract_evm = [0x1a; 20];
    let user_evm = [0x2b; 20];
    let acl_evm = [0x3c; 20];
    let aux = HostChainAuxData {
        contract: HostChainAddress::from_evm_address(contract_evm),
        user: HostChainAddress::from_evm_address(user_evm),
        acl_contract: HostChainAddress::from_evm_address(acl_evm),
        chain_id: 1,
    };
    let assembled = aux.assemble();

    assert_eq!(&assembled[12..32], &contract_evm, "contract low 20 bytes");
    assert_eq!(&assembled[44..64], &user_evm, "user low 20 bytes");
    assert_eq!(&assembled[76..96], &acl_evm, "acl low 20 bytes");
    assert_eq!(assembled[127], 1, "chain id 1 lands in the last byte");
}

#[test]
fn handle_derivation_rand_uses_keccak() {
    let seed = [0x42; 16];
    let chain_id = 13u64;
    let fhe_type = 5u8;
    let handle = computed_rand_handle(seed, fhe_type, chain_id);
    let expected_prehandle = keccak(&[
        COMPUTATION_DOMAIN_SEPARATOR,
        &[3],
        &[fhe_type],
        &seed,
        crate::ID.as_ref(),
        &chain_id.to_be_bytes(),
    ]);
    assert_eq!(&handle[..21], &expected_prehandle[..21]);
    assert_canonical_metadata(handle, fhe_type, chain_id);
}
