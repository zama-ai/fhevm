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
        max_hcu_per_tx: 0,
        max_hcu_depth_per_tx: 0,
        hcu_block_cap_per_app: u64::MAX,
        updated_slot: 0,
        bump: 0,
    }
}

#[test]
fn zero_birth_entropy_requires_poc_chain_and_test_shims() {
    // Local PoC chain with the shim flag: relaxation allowed only in PoC-feature builds.
    assert_eq!(
        host_config_with(SOLANA_POC_CHAIN_ID, true, false).zero_birth_entropy_allowed(),
        cfg!(feature = "poc")
    );
    // Local PoC chain without the shim flag: fails closed (drives the
    // PreviousBankHashUnavailable negative test).
    assert!(!host_config_with(SOLANA_POC_CHAIN_ID, false, false).zero_birth_entropy_allowed());
    // A deployed chain can NEVER zero birth entropy, even with the shim flag
    // toggled on by an admin — this is the security boundary.
    assert!(!host_config_with(101, true, false).zero_birth_entropy_allowed());
    assert!(!host_config_with(1, true, false).zero_birth_entropy_allowed());
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
fn eval_handle_derivation_preserves_solana_chain_type_high_bit() {
    // A Solana host chain id sets the reserved high bit; the derived handle
    // must round-trip it verbatim through bytes 22..30.
    let chain_id = SOLANA_CHAIN_TYPE_BIT | SOLANA_POC_CHAIN_ID;
    let handle = computed_eval_handle(
        FheBinaryOpCode::Sub,
        [1; 32],
        [2; 32],
        true,
        3,
        chain_id,
        [9; 32],
        42,
        [7; 32],
        0,
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

/// Builds a handle carrying `fhe_type` in byte 30 (the canonical type nibble read by
/// `handle_fhe_type`); the remaining bytes only need to differ per operand.
fn typed_handle(tag: u8, fhe_type: u8) -> [u8; 32] {
    let mut handle = [tag; 32];
    handle[30] = fhe_type;
    handle
}

/// Asserts the operand-validation result failed with the given Anchor error code.
fn assert_error(result: Result<()>, expected: ZamaHostError) {
    let code = match result.unwrap_err() {
        anchor_lang::error::Error::AnchorError(err) => err.error_code_number,
        other => panic!("expected an AnchorError, got {other:?}"),
    };
    assert_eq!(code, u32::from(expected));
}

#[test]
fn assert_sum_operand_types_enforces_uniform_declared_type() {
    let a = typed_handle(1, 5);
    let b = typed_handle(2, 5);
    assert!(assert_sum_operand_types(&[a, b], 5).is_ok());
    // A single operand whose type differs from the declared type is rejected — the type-confusion
    // gap this branch closes for Sum.
    assert_error(
        assert_sum_operand_types(&[a, typed_handle(3, 4)], 5),
        ZamaHostError::BinaryOperandTypeMismatch,
    );
    // EVM/coprocessor enforce no minimum: single-operand and empty sums are valid.
    assert!(assert_sum_operand_types(&[a], 5).is_ok());
    assert!(assert_sum_operand_types(&[], 5).is_ok());
    // Non-uint declared types are still rejected.
    assert_error(
        assert_sum_operand_types(&[typed_handle(1, 0), typed_handle(2, 0)], 0),
        ZamaHostError::UnsupportedFheType,
    );
}

#[test]
fn assert_is_in_operand_types_enforces_uniform_declared_type() {
    let value = typed_handle(1, 4);
    let set = [typed_handle(2, 4), typed_handle(3, 4)];
    assert!(assert_is_in_operand_types(value, &set, 4).is_ok());
    // The full EVM/coprocessor range Uint8..Uint256 (2..=8) is accepted, including Uint256.
    assert!(assert_is_in_operand_types(
        typed_handle(1, 8),
        &[typed_handle(2, 8), typed_handle(3, 8)],
        8
    )
    .is_ok());
    // Value type mismatch and set-element type mismatch are both rejected.
    assert_error(
        assert_is_in_operand_types(typed_handle(1, 5), &set, 4),
        ZamaHostError::BinaryOperandTypeMismatch,
    );
    assert_error(
        assert_is_in_operand_types(value, &[typed_handle(2, 4), typed_handle(3, 5)], 4),
        ZamaHostError::BinaryOperandTypeMismatch,
    );
    // EVM/coprocessor enforce no minimum: an empty set is valid (membership is trivially false).
    assert!(assert_is_in_operand_types(value, &[], 4).is_ok());
    // The excluded ebool type is still rejected.
    assert_error(
        assert_is_in_operand_types(typed_handle(1, 0), &[typed_handle(2, 0)], 0),
        ZamaHostError::UnsupportedFheType,
    );
}

#[test]
fn assert_mul_div_operand_types_matches_evm_bounds() {
    let factor1 = typed_handle(1, 5);
    let factor2 = typed_handle(2, 5);
    let nonzero_divisor = typed_handle(9, 0);
    // Encrypted factor2 of the same type, and a scalar factor2 (handle ignored), are both accepted.
    assert!(assert_mul_div_operand_types(factor1, factor2, false, nonzero_divisor, 5).is_ok());
    assert!(assert_mul_div_operand_types(factor1, [0u8; 32], true, nonzero_divisor, 5).is_ok());
    // Uint128 output is rejected — EVM and the coprocessor cap mulDiv at Uint64.
    assert_error(
        assert_mul_div_operand_types(
            typed_handle(1, 6),
            typed_handle(2, 6),
            false,
            nonzero_divisor,
            6,
        ),
        ZamaHostError::UnsupportedFheType,
    );
    // factor1 / encrypted-factor2 type mismatches are rejected.
    assert_error(
        assert_mul_div_operand_types(typed_handle(1, 4), factor2, false, nonzero_divisor, 5),
        ZamaHostError::BinaryOperandTypeMismatch,
    );
    assert_error(
        assert_mul_div_operand_types(factor1, typed_handle(2, 4), false, nonzero_divisor, 5),
        ZamaHostError::BinaryOperandTypeMismatch,
    );
    // A zero plaintext divisor is rejected (EVM DivisionByZero parity).
    assert_error(
        assert_mul_div_operand_types(factor1, factor2, true, [0u8; 32], 5),
        ZamaHostError::MulDivDivisorZero,
    );
}

#[test]
fn assert_unary_operand_type_rejects_same_type_cast() {
    // Cast to a different type is allowed; a same-type cast is rejected (EVM InvalidType parity).
    assert!(assert_unary_operand_type(FheUnaryOpCode::Cast, typed_handle(1, 5), 4).is_ok());
    assert_error(
        assert_unary_operand_type(FheUnaryOpCode::Cast, typed_handle(1, 5), 5),
        ZamaHostError::UnsupportedFheType,
    );
    // EVM cast type sets: bool input casts (bool -> Uint32), and Uint256 both ways, are allowed...
    assert!(assert_unary_operand_type(FheUnaryOpCode::Cast, typed_handle(1, 0), 4).is_ok());
    assert!(assert_unary_operand_type(FheUnaryOpCode::Cast, typed_handle(1, 8), 5).is_ok());
    // ...but casting TO ebool (0) or eaddress/Uint160 (7), or FROM eaddress (7), is rejected.
    assert_error(
        assert_unary_operand_type(FheUnaryOpCode::Cast, typed_handle(1, 5), 0),
        ZamaHostError::UnsupportedFheType,
    );
    assert_error(
        assert_unary_operand_type(FheUnaryOpCode::Cast, typed_handle(1, 5), 7),
        ZamaHostError::UnsupportedFheType,
    );
    assert_error(
        assert_unary_operand_type(FheUnaryOpCode::Cast, typed_handle(1, 7), 5),
        ZamaHostError::UnsupportedFheType,
    );
    // Neg/Not require operand type == output type; Not additionally accepts ebool, Neg does not.
    assert!(assert_unary_operand_type(FheUnaryOpCode::Neg, typed_handle(1, 5), 5).is_ok());
    assert!(assert_unary_operand_type(FheUnaryOpCode::Not, typed_handle(1, 0), 0).is_ok());
    assert_error(
        assert_unary_operand_type(FheUnaryOpCode::Neg, typed_handle(1, 0), 0),
        ZamaHostError::UnsupportedFheType,
    );
}

#[test]
fn assert_binary_operand_types_matches_evm_supported_sets() {
    // Eq/Ne accept the widest set including Bool and Uint256; their output is ebool.
    assert!(assert_binary_operand_types(
        FheBinaryOpCode::Eq,
        typed_handle(1, 0),
        typed_handle(2, 0),
        false,
        0
    )
    .is_ok());
    assert!(assert_binary_operand_types(
        FheBinaryOpCode::Ne,
        typed_handle(1, 8),
        typed_handle(2, 8),
        false,
        0
    )
    .is_ok());
    // Ordered comparisons reject Bool/Uint256 (EVM fheGe supportedTypes = Uint8..Uint128).
    assert_error(
        assert_binary_operand_types(
            FheBinaryOpCode::Ge,
            typed_handle(1, 0),
            typed_handle(2, 0),
            false,
            0,
        ),
        ZamaHostError::UnsupportedFheType,
    );
    // Bitwise ops accept Uint256; the operand type must equal the output type.
    assert!(assert_binary_operand_types(
        FheBinaryOpCode::And,
        typed_handle(1, 8),
        typed_handle(2, 8),
        false,
        8
    )
    .is_ok());
    assert_error(
        assert_binary_operand_types(
            FheBinaryOpCode::And,
            typed_handle(1, 8),
            typed_handle(2, 5),
            false,
            8,
        ),
        ZamaHostError::BinaryOperandTypeMismatch,
    );
}

#[test]
fn assert_binary_div_rem_require_nonzero_scalar_divisor() {
    let lhs = typed_handle(1, 5); // euint64
    let nonzero_scalar = typed_handle(9, 0);
    // A non-zero scalar divisor is accepted for both Div and Rem.
    assert!(
        assert_binary_operand_types(FheBinaryOpCode::Div, lhs, nonzero_scalar, true, 5).is_ok()
    );
    assert!(
        assert_binary_operand_types(FheBinaryOpCode::Rem, lhs, nonzero_scalar, true, 5).is_ok()
    );
    // An encrypted divisor is rejected — division is scalar-only (EVM `IsNotScalar`).
    assert_error(
        assert_binary_operand_types(FheBinaryOpCode::Div, lhs, typed_handle(2, 5), false, 5),
        ZamaHostError::DivisorMustBeScalar,
    );
    // A zero scalar divisor is rejected.
    assert_error(
        assert_binary_operand_types(FheBinaryOpCode::Rem, lhs, [0u8; 32], true, 5),
        ZamaHostError::DivisionByZero,
    );
    // A divisor nonzero only above the euint8 width truncates to zero -> rejected.
    let mut high_only = [0u8; 32];
    high_only[30] = 0x01;
    assert_error(
        assert_binary_operand_types(FheBinaryOpCode::Div, typed_handle(1, 2), high_only, true, 2),
        ZamaHostError::DivisionByZero,
    );
}

#[test]
fn assert_mul_div_rejects_width_truncated_zero_divisor() {
    let factor1 = typed_handle(1, 2); // euint8
    let factor2 = typed_handle(2, 2);
    // Divisor nonzero only above the u8 width truncates to zero -> rejected (EVM parity).
    let mut high_only = [0u8; 32];
    high_only[30] = 0x01;
    assert_error(
        assert_mul_div_operand_types(factor1, factor2, false, high_only, 2),
        ZamaHostError::MulDivDivisorZero,
    );
    // A low-byte-nonzero divisor is accepted.
    let mut low = [0u8; 32];
    low[31] = 0x01;
    assert!(assert_mul_div_operand_types(factor1, factor2, false, low, 2).is_ok());
}

#[test]
fn assert_sum_and_is_in_enforce_coprocessor_max_operand_counts() {
    // Narrow types (Uint8..Uint32) cap at 100 operands; wider ones at 60.
    let narrow = |n| vec![typed_handle(1, 2); n];
    assert!(assert_sum_operand_types(&narrow(100), 2).is_ok());
    assert_error(
        assert_sum_operand_types(&narrow(101), 2),
        ZamaHostError::InvalidFheEvalAccount,
    );
    let wide = |n| vec![typed_handle(1, 5); n];
    assert!(assert_sum_operand_types(&wide(60), 5).is_ok());
    assert_error(
        assert_sum_operand_types(&wide(61), 5),
        ZamaHostError::InvalidFheEvalAccount,
    );
    // IsIn caps the set size (excluding the tested value) the same way.
    let value = typed_handle(1, 2);
    assert!(assert_is_in_operand_types(value, &narrow(100), 2).is_ok());
    assert_error(
        assert_is_in_operand_types(value, &narrow(101), 2),
        ZamaHostError::InvalidFheEvalAccount,
    );
}
