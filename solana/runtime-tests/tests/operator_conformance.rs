//! Fast, test-only semantic/support-matrix conformance for Solana `fhe_execute`.
//!
//! Expected support is test-owned rather than derived from production validators. Execution uses
//! those validators and the shared cleartext evaluator. Mollusk, real TFHE, and full-stack smoke
//! are deliberately separate test tiers.

use std::collections::HashMap;

use zama_solana_test_kit::oracle::{evaluate, ClearInputs, TypedClearValue};
use zama_solana_test_kit::{binary_contract_tests, composite_contract_tests, unary_contract_tests};
use zama_host::instructions::fhe_execute::assert_ternary_operand_types;
use zama_host::{
    assert_binary_operand_types, assert_unary_operand_type, CoprocessorInputAttestation,
    FheBinaryOpCode, FheExecuteArgs, FheExecuteOperand, FheExecuteOutput, FheExecuteStep,
    FheTernaryOpCode, FheUnaryOpCode,
};

type Handle = [u8; 32];

binary_contract_tests!();
unary_contract_tests!();
composite_contract_tests!();

fn run_binary(
    op: FheBinaryOpCode,
    input_type: u8,
    scalar_rhs: bool,
    lhs: u64,
    rhs: u64,
    output_type: u8,
    expected: u64,
) {
    let lhs_handle = handle(1, input_type);
    let rhs_handle = handle(2, input_type);
    let mut inputs = HashMap::from([(lhs_handle, typed(input_type, lhs))]);
    let rhs = if scalar_rhs {
        scalar(be(rhs))
    } else {
        inputs.insert(rhs_handle, typed(input_type, rhs));
        persistent(rhs_handle)
    };
    let execution = args(vec![FheExecuteStep::Binary {
        op,
        lhs: persistent(lhs_handle),
        rhs,
        output_fhe_type: output_type,
        output: local_output(),
    }]);
    assert_eq!(
        evaluate(&execution, &inputs).unwrap(),
        vec![typed(output_type, expected)]
    );
}

fn run_unary(
    op: FheUnaryOpCode,
    input_type: u8,
    input: [u8; 32],
    output_type: u8,
    expected: [u8; 32],
) {
    let input_handle = handle(1, input_type);
    let execution = args(vec![FheExecuteStep::Unary {
        op,
        operand: persistent(input_handle),
        output_fhe_type: output_type,
        output: local_output(),
    }]);
    let inputs = HashMap::from([(
        input_handle,
        TypedClearValue::from_be_bytes(input_type, input),
    )]);
    assert_eq!(
        evaluate(&execution, &inputs).unwrap(),
        vec![TypedClearValue::from_be_bytes(output_type, expected)]
    );
}

fn run_ternary(fhe_type: u8, if_true: u64, if_false: u64, expected: u64) {
    let control = handle(1, 0);
    let true_handle = handle(2, fhe_type);
    let false_handle = handle(3, fhe_type);
    let execution = args(vec![FheExecuteStep::Ternary {
        op: FheTernaryOpCode::IfThenElse,
        control: persistent(control),
        if_true: persistent(true_handle),
        if_false: persistent(false_handle),
        output_fhe_type: fhe_type,
        output: local_output(),
    }]);
    let inputs = HashMap::from([
        (control, typed(0, 1)),
        (true_handle, typed(fhe_type, if_true)),
        (false_handle, typed(fhe_type, if_false)),
    ]);
    assert_eq!(
        evaluate(&execution, &inputs).unwrap(),
        vec![typed(fhe_type, expected)]
    );
}

fn run_trivial(fhe_type: u8, plaintext: u64, expected: u64) {
    let execution = args(vec![FheExecuteStep::TrivialEncrypt {
        plaintext: be(plaintext),
        fhe_type,
        output: local_output(),
    }]);
    assert_eq!(
        evaluate(&execution, &ClearInputs::new()).unwrap(),
        vec![typed(fhe_type, expected)]
    );
}

fn run_sum(fhe_type: u8, lhs: u64, rhs: u64, expected: u64) {
    let lhs_handle = handle(1, fhe_type);
    let rhs_handle = handle(2, fhe_type);
    let execution = args(vec![sum_step(
        vec![persistent(lhs_handle), persistent(rhs_handle)],
        fhe_type,
    )]);
    let inputs = HashMap::from([
        (lhs_handle, typed(fhe_type, lhs)),
        (rhs_handle, typed(fhe_type, rhs)),
    ]);
    assert_eq!(
        evaluate(&execution, &inputs).unwrap(),
        vec![typed(fhe_type, expected)]
    );
}

fn run_empty_sum(fhe_type: u8) {
    let execution = args(vec![sum_step(vec![], fhe_type)]);
    assert_eq!(
        evaluate(&execution, &ClearInputs::new()).unwrap(),
        vec![typed(fhe_type, 0)]
    );
}

fn run_is_in(fhe_type: u8, value: u64, include_set: bool, expected: u64) {
    let value_handle = handle(1, fhe_type);
    let member_handle = handle(2, fhe_type);
    let set = include_set
        .then(|| persistent(member_handle))
        .into_iter()
        .collect();
    let execution = args(vec![is_in_step(persistent(value_handle), set, fhe_type)]);
    let mut inputs = HashMap::from([(value_handle, typed(fhe_type, value))]);
    if include_set {
        inputs.insert(member_handle, typed(fhe_type, 29));
    }
    assert_eq!(
        evaluate(&execution, &inputs).unwrap(),
        vec![typed(0, expected)]
    );
}

fn run_mul_div(
    fhe_type: u8,
    scalar_factor2: bool,
    factor1: u64,
    factor2: u64,
    divisor: u64,
    expected: u64,
) {
    let first_handle = handle(1, fhe_type);
    let second_handle = handle(2, fhe_type);
    let mut inputs = HashMap::from([(first_handle, typed(fhe_type, factor1))]);
    let second = if scalar_factor2 {
        scalar(be(factor2))
    } else {
        inputs.insert(second_handle, typed(fhe_type, factor2));
        persistent(second_handle)
    };
    let execution = args(vec![mul_div_step(
        persistent(first_handle),
        second,
        be(divisor),
        fhe_type,
    )]);
    assert_eq!(
        evaluate(&execution, &inputs).unwrap(),
        vec![typed(fhe_type, expected)]
    );
}

fn run_rand(fhe_type: u8) {
    let execution = args(vec![FheExecuteStep::Rand {
        fhe_type,
        output: local_output(),
    }]);
    let first = evaluate(&execution, &ClearInputs::new()).unwrap();
    let second = evaluate(&execution, &ClearInputs::new()).unwrap();
    assert_eq!(first, second, "test random must be repeatable");
    assert_eq!(first[0].fhe_type, fhe_type);
    if fhe_type == 0 {
        assert_eq!(first[0].value[..31], [0; 31]);
        assert!(first[0].value[31] <= 1);
    }
}

fn run_rand_bounded(fhe_type: u8, upper_bound: u64) {
    let execution = args(vec![bounded_rand_step(be(upper_bound), fhe_type)]);
    let first = evaluate(&execution, &ClearInputs::new()).unwrap();
    let second = evaluate(&execution, &ClearInputs::new()).unwrap();
    assert_eq!(first, second, "test bounded random must be repeatable");
    assert_eq!(first[0].fhe_type, fhe_type);
    assert_eq!(first[0].value[..24], [0; 24]);
    assert!(u64::from_be_bytes(first[0].value[24..].try_into().unwrap()) < upper_bound);
}

mod edges {
    use super::*;
    #[test]
    fn binary_shl_u8_scalar_reduces_shift_modulo_width() {
        run_binary(FheBinaryOpCode::Shl, 2, true, 1, 9, 2, 2);
    }
    #[test]
    fn binary_shr_u8_scalar_reduces_shift_modulo_width() {
        run_binary(FheBinaryOpCode::Shr, 2, true, 0x80, 9, 2, 0x40);
    }
    #[test]
    fn binary_rotl_u8_scalar_wraps_high_bit() {
        run_binary(FheBinaryOpCode::Rotl, 2, true, 0x81, 1, 2, 3);
    }
    #[test]
    fn binary_rotr_u8_scalar_wraps_low_bit() {
        run_binary(FheBinaryOpCode::Rotr, 2, true, 3, 1, 2, 0x81);
    }
    #[test]
    fn binary_le_u64_scalar_false() {
        run_binary(FheBinaryOpCode::Le, 5, true, 10, 9, 0, 0);
    }
    #[test]
    fn binary_add_u8_scalar_truncates_high_bytes() {
        let input = handle(1, 2);
        let mut high_only = [0; 32];
        high_only[0] = 1;
        let execution = args(vec![binary(
            FheBinaryOpCode::Add,
            persistent(input),
            scalar(high_only),
            2,
        )]);
        assert_eq!(
            evaluate(&execution, &HashMap::from([(input, typed(2, 7))])).unwrap(),
            vec![typed(2, 7)]
        );
    }
    #[test]
    fn unary_cast_u256_to_u8_truncates_high_bytes() {
        let mut input = [0xff; 32];
        input[31] = 0x2a;
        run_unary(FheUnaryOpCode::Cast, 8, input, 2, be(0x2a));
    }
    #[test]
    fn sum_u8_wraps() {
        run_sum(2, 250, 10, 4);
    }
    #[test]
    fn bool_trivial_uses_low_byte_only() {
        let mut high_only = [0; 32];
        high_only[0] = 1;
        let execution = args(vec![FheExecuteStep::TrivialEncrypt {
            plaintext: high_only,
            fhe_type: 0,
            output: local_output(),
        }]);
        assert_eq!(
            evaluate(&execution, &ClearInputs::new()).unwrap(),
            vec![typed(0, 0)]
        );
    }
    #[test]
    fn binary_eq_bool_scalar_high_byte_is_nonzero() {
        let encrypted_false = handle(1, 0);
        let mut high_only = [0; 32];
        high_only[0] = 1;
        let execution = args(vec![binary(
            FheBinaryOpCode::Eq,
            persistent(encrypted_false),
            scalar(high_only),
            0,
        )]);
        assert_eq!(
            evaluate(&execution, &HashMap::from([(encrypted_false, typed(0, 0))])).unwrap(),
            vec![typed(0, 0)]
        );
    }
    #[test]
    fn ternary_false_selects_false_branch() {
        let control = handle(1, 0);
        let if_true = handle(2, 2);
        let if_false = handle(3, 2);
        let execution = args(vec![FheExecuteStep::Ternary {
            op: FheTernaryOpCode::IfThenElse,
            control: persistent(control),
            if_true: persistent(if_true),
            if_false: persistent(if_false),
            output_fhe_type: 2,
            output: local_output(),
        }]);
        let inputs = HashMap::from([
            (control, typed(0, 0)),
            (if_true, typed(2, 11)),
            (if_false, typed(2, 22)),
        ]);
        assert_eq!(evaluate(&execution, &inputs).unwrap(), vec![typed(2, 22)]);
    }
    #[test]
    fn sum_u8_accepts_exact_narrow_operand_cap() {
        let narrow = handle(1, 2);
        let execution = args(vec![sum_step(vec![persistent(narrow); 100], 2)]);
        assert_eq!(
            evaluate(&execution, &HashMap::from([(narrow, typed(2, 1))])).unwrap(),
            vec![typed(2, 100)]
        );
    }
    #[test]
    fn is_in_u256_accepts_exact_wide_set_cap() {
        let wide = handle(1, 8);
        let execution = args(vec![is_in_step(
            persistent(wide),
            vec![persistent(wide); 60],
            8,
        )]);
        assert_eq!(
            evaluate(&execution, &HashMap::from([(wide, typed(8, 9))])).unwrap(),
            vec![typed(0, 1)]
        );
    }
    #[test]
    fn is_in_u160_high_bit_present() {
        let value = handle(1, 7);
        let member = handle(2, 7);
        let mut high = [0; 32];
        high[12] = 1;
        let execution = args(vec![is_in_step(
            persistent(value),
            vec![persistent(member)],
            7,
        )]);
        let inputs = HashMap::from([
            (value, TypedClearValue::from_be_bytes(7, high)),
            (member, TypedClearValue::from_be_bytes(7, high)),
        ]);
        assert_eq!(evaluate(&execution, &inputs).unwrap(), vec![typed(0, 1)]);
    }
    #[test]
    fn is_in_u160_high_bit_empty_set_is_false() {
        let value = handle(1, 7);
        let mut high = [0; 32];
        high[12] = 1;
        let execution = args(vec![is_in_step(persistent(value), vec![], 7)]);
        let inputs = HashMap::from([(value, TypedClearValue::from_be_bytes(7, high))]);
        assert_eq!(evaluate(&execution, &inputs).unwrap(), vec![typed(0, 0)]);
    }
    #[test]
    fn rand_then_bounded_rand_preserves_deterministic_output_order() {
        let execution = args(vec![
            FheExecuteStep::Rand {
                fhe_type: 8,
                output: local_output(),
            },
            bounded_rand_step(be(16), 2),
        ]);
        let first = evaluate(&execution, &ClearInputs::new()).unwrap();
        let second = evaluate(&execution, &ClearInputs::new()).unwrap();
        assert_eq!(first, second);
        assert_eq!([first[0].fhe_type, first[1].fhe_type], [8, 2]);
        assert!(u64::from_be_bytes(first[1].value[24..].try_into().unwrap()) < 16);
    }
    #[test]
    fn binary_eq_u160_high_bit_differs_from_zero() {
        let high = handle(1, 7);
        let zero = handle(2, 7);
        let mut high_value = [0; 32];
        high_value[12] = 0x80;
        let execution = args(vec![binary(
            FheBinaryOpCode::Eq,
            persistent(high),
            persistent(zero),
            0,
        )]);
        let inputs = HashMap::from([
            (high, TypedClearValue::from_be_bytes(7, high_value)),
            (zero, typed(7, 0)),
        ]);
        assert_eq!(evaluate(&execution, &inputs).unwrap(), vec![typed(0, 0)]);
    }
    #[test]
    fn binary_ne_u256_distinct_high_bits() {
        let lhs = handle(1, 8);
        let rhs = handle(2, 8);
        let mut lhs_value = [0; 32];
        lhs_value[0] = 0x80;
        let mut rhs_value = [0; 32];
        rhs_value[0] = 0x40;
        let execution = args(vec![binary(
            FheBinaryOpCode::Ne,
            persistent(lhs),
            persistent(rhs),
            0,
        )]);
        let inputs = HashMap::from([
            (lhs, TypedClearValue::from_be_bytes(8, lhs_value)),
            (rhs, TypedClearValue::from_be_bytes(8, rhs_value)),
        ]);
        assert_eq!(evaluate(&execution, &inputs).unwrap(), vec![typed(0, 1)]);
    }
    #[test]
    fn binary_rotr_u256_moves_low_bit_to_high_bit() {
        let one = handle(1, 8);
        let mut expected = [0; 32];
        expected[0] = 0x80;
        let execution = args(vec![binary(
            FheBinaryOpCode::Rotr,
            persistent(one),
            scalar(be(1)),
            8,
        )]);
        assert_eq!(
            evaluate(&execution, &HashMap::from([(one, typed(8, 1))])).unwrap(),
            vec![TypedClearValue::from_be_bytes(8, expected)]
        );
    }
    #[test]
    fn binary_eq_u256_equal_patterned_high_and_low_bytes() {
        let lhs = handle(1, 8);
        let rhs = handle(2, 8);
        let mut pattern = [0; 32];
        pattern[0] = 0x80;
        pattern[31] = 1;
        let execution = args(vec![binary(
            FheBinaryOpCode::Eq,
            persistent(lhs),
            persistent(rhs),
            0,
        )]);
        let inputs = HashMap::from([
            (lhs, TypedClearValue::from_be_bytes(8, pattern)),
            (rhs, TypedClearValue::from_be_bytes(8, pattern)),
        ]);
        assert_eq!(evaluate(&execution, &inputs).unwrap(), vec![typed(0, 1)]);
    }
    #[test]
    fn unary_not_u256_patterned_bytes_has_literal_complement() {
        let mut input = [0; 32];
        input[0] = 0x80;
        input[31] = 1;
        run_unary(
            FheUnaryOpCode::Not,
            8,
            input,
            8,
            [
                0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xfe,
            ],
        );
    }
}

mod rejected {
    use super::*;
    mod closed_world {
        use super::*;
        #[test]
        fn binary_unary_and_ternary_admission_is_closed_world() {
            for op in binary_ops() {
                for input_type in u8::MIN..=u8::MAX {
                    for output_type in u8::MIN..=u8::MAX {
                        for scalar in [false, true] {
                            let rhs = if scalar { be(1) } else { handle(2, input_type) };
                            assert_eq!(
                                assert_binary_operand_types(
                                    op,
                                    handle(1, input_type),
                                    rhs,
                                    scalar,
                                    output_type,
                                )
                                .is_ok(),
                                expected_binary(op, input_type, scalar, output_type),
                                "{op:?}, input={input_type}, scalar={scalar}, output={output_type}"
                            );
                        }
                    }
                }
            }
            for op in unary_ops() {
                for input_type in u8::MIN..=u8::MAX {
                    for output_type in u8::MIN..=u8::MAX {
                        assert_eq!(
                            assert_unary_operand_type(op, handle(1, input_type), output_type)
                                .is_ok(),
                            expected_unary(op, input_type, output_type),
                            "{op:?}, input={input_type}, output={output_type}"
                        );
                    }
                }
            }
            for op in ternary_ops() {
                for fhe_type in u8::MIN..=u8::MAX {
                    for (control, if_true, if_false, output) in [
                        (fhe_type, 2, 2, 2),
                        (0, fhe_type, 2, 2),
                        (0, 2, fhe_type, 2),
                        (0, fhe_type, fhe_type, fhe_type),
                    ] {
                        assert_eq!(
                            ternary_is_accepted(op, control, if_true, if_false, output),
                            expected_ternary(op, control, if_true, if_false, output),
                            "{op:?}, control={control}, if_true={if_true}, if_false={if_false}, output={output}"
                        );
                    }
                }
            }
        }
        #[test]
        fn rem_u8_scalar_zero_after_width_truncation() {
            let lhs = handle(1, 2);
            let mut high_only = [0; 32];
            high_only[0] = 1;
            expect_error(
                args(vec![binary(
                    FheBinaryOpCode::Rem,
                    persistent(lhs),
                    scalar(high_only),
                    2,
                )]),
                HashMap::from([(lhs, typed(2, 8))]),
                "DivisionByZero",
            );
        }
        #[test]
        fn div_u8_scalar_zero_after_width_truncation() {
            let lhs = handle(1, 2);
            let mut high_only = [0; 32];
            high_only[0] = 1;
            expect_error(
                args(vec![binary(
                    FheBinaryOpCode::Div,
                    persistent(lhs),
                    scalar(high_only),
                    2,
                )]),
                HashMap::from([(lhs, typed(2, 8))]),
                "DivisionByZero",
            );
        }
        #[test]
        fn eq_u8_u16_encrypted_types_must_match() {
            let lhs = handle(1, 2);
            let rhs = handle(2, 3);
            expect_error(
                args(vec![binary(
                    FheBinaryOpCode::Eq,
                    persistent(lhs),
                    persistent(rhs),
                    0,
                )]),
                HashMap::from([(lhs, typed(2, 1)), (rhs, typed(3, 1))]),
                "BinaryOperandTypeMismatch",
            );
        }
    }
    mod ternary {
        use super::*;
        #[test]
        fn if_then_else_rejects_mixed_branch_types() {
            let control = handle(1, 0);
            let if_true = handle(2, 2);
            let if_false = handle(3, 3);
            let execution = args(vec![FheExecuteStep::Ternary {
                op: FheTernaryOpCode::IfThenElse,
                control: persistent(control),
                if_true: persistent(if_true),
                if_false: persistent(if_false),
                output_fhe_type: 2,
                output: local_output(),
            }]);
            let inputs = HashMap::from([
                (control, typed(0, 1)),
                (if_true, typed(2, 11)),
                (if_false, typed(3, 22)),
            ]);
            expect_error(execution, inputs, "invalid ternary operand types");
        }
        #[test]
        fn if_then_else_rejects_unsupported_output_before_branch_lookup() {
            let control = handle(1, 0);
            let missing_branch = handle(2, 1);
            let execution = args(vec![FheExecuteStep::Ternary {
                op: FheTernaryOpCode::IfThenElse,
                control: persistent(control),
                if_true: persistent(missing_branch),
                if_false: persistent(missing_branch),
                output_fhe_type: 1,
                output: local_output(),
            }]);
            expect_error(
                execution,
                HashMap::from([(control, typed(0, 1))]),
                "UnsupportedFheType",
            );
        }
    }
    mod composite {
        use super::*;
        #[test]
        fn trivial_encrypt_unknown_type() {
            expect_error(
                args(vec![FheExecuteStep::TrivialEncrypt {
                    plaintext: be(1),
                    fhe_type: 1,
                    output: local_output(),
                }]),
                ClearInputs::new(),
                "UnsupportedFheType",
            );
        }
        #[test]
        fn sum_u256_type() {
            let input = handle(1, 8);
            expect_error(
                args(vec![sum_step(vec![persistent(input)], 8)]),
                HashMap::from([(input, typed(8, 1))]),
                "UnsupportedFheType",
            );
        }
        #[test]
        fn sum_u8_over_narrow_cap() {
            let input = handle(1, 2);
            expect_error(
                args(vec![sum_step(vec![persistent(input); 101], 2)]),
                HashMap::from([(input, typed(2, 1))]),
                "InvalidFheExecuteAccount",
            );
        }
        #[test]
        fn is_in_bool_type() {
            let input = handle(1, 0);
            expect_error(
                args(vec![is_in_step(persistent(input), vec![], 0)]),
                HashMap::from([(input, typed(0, 1))]),
                "UnsupportedFheType",
            );
        }
        #[test]
        fn is_in_u256_over_wide_cap() {
            let input = handle(1, 8);
            expect_error(
                args(vec![is_in_step(
                    persistent(input),
                    vec![persistent(input); 61],
                    8,
                )]),
                HashMap::from([(input, typed(8, 1))]),
                "InvalidFheExecuteAccount",
            );
        }
        #[test]
        fn sum_rejects_mixed_types() {
            let u8_input = handle(1, 2);
            let u16_input = handle(2, 3);
            expect_error(
                args(vec![sum_step(
                    vec![persistent(u8_input), persistent(u16_input)],
                    2,
                )]),
                HashMap::from([(u8_input, typed(2, 1)), (u16_input, typed(3, 1))]),
                "BinaryOperandTypeMismatch",
            );
        }
        #[test]
        fn is_in_rejects_mixed_types() {
            let u8_input = handle(1, 2);
            let u16_input = handle(2, 3);
            expect_error(
                args(vec![is_in_step(
                    persistent(u8_input),
                    vec![persistent(u16_input)],
                    2,
                )]),
                HashMap::from([(u8_input, typed(2, 1)), (u16_input, typed(3, 1))]),
                "BinaryOperandTypeMismatch",
            );
        }
        #[test]
        fn mul_div_u128_output_type() {
            let input = handle(1, 6);
            expect_error(
                args(vec![mul_div_step(
                    persistent(input),
                    scalar(be(2)),
                    be(1),
                    6,
                )]),
                HashMap::from([(input, typed(6, 2))]),
                "UnsupportedFheType",
            );
        }
        #[test]
        fn mul_div_rejects_zero_divisor_after_truncation() {
            let u8_input = handle(1, 2);
            let mut high_only = [0; 32];
            high_only[0] = 1;
            expect_error(
                args(vec![mul_div_step(
                    persistent(u8_input),
                    scalar(be(2)),
                    high_only,
                    2,
                )]),
                HashMap::from([(u8_input, typed(2, 2))]),
                "MulDivDivisorZero",
            );
        }
        #[test]
        fn mul_div_rejects_mixed_factor_types() {
            let u8_input = handle(1, 2);
            let u16_input = handle(2, 3);
            expect_error(
                args(vec![mul_div_step(
                    persistent(u8_input),
                    persistent(u16_input),
                    be(1),
                    2,
                )]),
                HashMap::from([(u8_input, typed(2, 2)), (u16_input, typed(3, 2))]),
                "BinaryOperandTypeMismatch",
            );
        }
        #[test]
        fn rand_u160_type() {
            expect_error(
                args(vec![FheExecuteStep::Rand {
                    fhe_type: 7,
                    output: local_output(),
                }]),
                ClearInputs::new(),
                "UnsupportedFheType",
            );
        }
        #[test]
        fn bounded_rand_bool_type() {
            expect_error(
                args(vec![bounded_rand_step(be(2), 0)]),
                ClearInputs::new(),
                "UnsupportedFheType",
            );
        }
        #[test]
        fn bounded_rand_non_power_of_two() {
            expect_error(
                args(vec![bounded_rand_step(be(3), 2)]),
                ClearInputs::new(),
                "InvalidRandomUpperBound",
            );
        }
        #[test]
        fn bounded_rand_zero() {
            expect_error(
                args(vec![bounded_rand_step(be(0), 2)]),
                ClearInputs::new(),
                "InvalidRandomUpperBound",
            );
        }
        #[test]
        fn bounded_rand_u8_too_wide() {
            expect_error(
                args(vec![bounded_rand_step(be(512), 2)]),
                ClearInputs::new(),
                "InvalidRandomUpperBound",
            );
        }
    }
}

mod operand_sources {
    use super::*;
    #[test]
    fn verified_input_can_seed_an_operation() {
        let input = handle(1, 2);
        let execution = args(vec![binary(
            FheBinaryOpCode::Add,
            verified(input),
            scalar(be(3)),
            2,
        )]);
        assert_eq!(
            evaluate(&execution, &HashMap::from([(input, typed(2, 4))])).unwrap(),
            vec![typed(2, 7)]
        );
    }
    #[test]
    fn local_outputs_can_feed_later_steps() {
        let input = handle(1, 2);
        let execution = args(vec![
            binary(FheBinaryOpCode::Add, persistent(input), scalar(be(1)), 2),
            FheExecuteStep::Unary {
                op: FheUnaryOpCode::Not,
                operand: FheExecuteOperand::EarlierStep { producer_index: 0 },
                output_fhe_type: 2,
                output: local_output(),
            },
        ]);
        assert_eq!(
            evaluate(&execution, &HashMap::from([(input, typed(2, 1))])).unwrap(),
            vec![typed(2, 2), typed(2, 253)]
        );
    }
    #[test]
    fn self_local_is_rejected() {
        reject_local(0);
    }
    #[test]
    fn forward_local_is_rejected() {
        reject_local(1);
    }
    fn reject_local(producer_index: u8) {
        expect_error(
            args(vec![FheExecuteStep::Unary {
                op: FheUnaryOpCode::Not,
                operand: FheExecuteOperand::EarlierStep { producer_index },
                output_fhe_type: 2,
                output: local_output(),
            }]),
            ClearInputs::new(),
            "missing earlier local output",
        );
    }
    #[test]
    fn missing_persistent_seed_is_rejected() {
        let input = handle(1, 2);
        expect_error(
            args(vec![binary(
                FheBinaryOpCode::Add,
                persistent(input),
                scalar(be(1)),
                2,
            )]),
            ClearInputs::new(),
            "missing cleartext input",
        );
    }
    #[test]
    fn missing_verified_input_seed_is_rejected() {
        let input = handle(1, 2);
        expect_error(
            args(vec![binary(
                FheBinaryOpCode::Add,
                verified(input),
                scalar(be(1)),
                2,
            )]),
            ClearInputs::new(),
            "missing cleartext input",
        );
    }
    #[test]
    fn scalar_in_encrypted_position_is_rejected() {
        expect_error(
            args(vec![binary(
                FheBinaryOpCode::Add,
                scalar(be(1)),
                scalar(be(2)),
                2,
            )]),
            ClearInputs::new(),
            "scalar is not valid",
        );
    }
    #[test]
    fn seed_type_must_match_handle_type() {
        let input = handle(1, 2);
        expect_error(
            args(vec![binary(
                FheBinaryOpCode::Add,
                persistent(input),
                scalar(be(1)),
                2,
            )]),
            HashMap::from([(input, typed(3, 1))]),
            "does not match cleartext type",
        );
    }
}

fn binary_ops() -> [FheBinaryOpCode; 20] {
    use FheBinaryOpCode::*;
    [
        Add, Sub, Mul, Div, Rem, And, Or, Xor, Shl, Shr, Rotl, Rotr, Eq, Ne, Ge, Gt, Le, Lt, Min,
        Max,
    ]
}

fn expected_binary(op: FheBinaryOpCode, input: u8, scalar: bool, output: u8) -> bool {
    use FheBinaryOpCode::*;
    match op {
        Add | Sub | Mul | Min | Max => matches!(input, 2..=6) && output == input,
        Div | Rem => matches!(input, 2..=6) && scalar && output == input,
        And | Or | Xor => matches!(input, 0 | 2..=6 | 8) && output == input,
        Shl | Shr | Rotl | Rotr => matches!(input, 2..=6 | 8) && output == input,
        Eq | Ne => matches!(input, 0 | 2..=8) && output == 0,
        Ge | Gt | Le | Lt => matches!(input, 2..=6) && output == 0,
    }
}

fn unary_ops() -> [FheUnaryOpCode; 3] {
    [
        FheUnaryOpCode::Neg,
        FheUnaryOpCode::Not,
        FheUnaryOpCode::Cast,
    ]
}

fn expected_unary(op: FheUnaryOpCode, input: u8, output: u8) -> bool {
    match op {
        FheUnaryOpCode::Neg => matches!(input, 2..=6 | 8) && output == input,
        FheUnaryOpCode::Not => matches!(input, 0 | 2..=6 | 8) && output == input,
        FheUnaryOpCode::Cast => {
            matches!(input, 0 | 2..=6 | 8) && matches!(output, 2..=6 | 8) && input != output
        }
    }
}

fn ternary_ops() -> [FheTernaryOpCode; 1] {
    [FheTernaryOpCode::IfThenElse]
}

fn expected_ternary(
    op: FheTernaryOpCode,
    control: u8,
    if_true: u8,
    if_false: u8,
    output: u8,
) -> bool {
    match op {
        FheTernaryOpCode::IfThenElse => {
            control == 0 && matches!(if_true, 0 | 2..=8) && if_true == output && if_false == output
        }
    }
}

fn ternary_is_accepted(
    op: FheTernaryOpCode,
    control_type: u8,
    if_true_type: u8,
    if_false_type: u8,
    output: u8,
) -> bool {
    match op {
        FheTernaryOpCode::IfThenElse => assert_ternary_operand_types(
            handle(1, control_type),
            handle(2, if_true_type),
            handle(3, if_false_type),
            output,
        )
        .is_ok(),
    }
}

fn expect_error(execution: FheExecuteArgs, inputs: ClearInputs, expected: &str) {
    let error = evaluate(&execution, &inputs).unwrap_err();
    assert!(
        error.contains(expected),
        "expected error containing {expected:?}, got {error:?}"
    );
}

fn binary(
    op: FheBinaryOpCode,
    lhs: FheExecuteOperand,
    rhs: FheExecuteOperand,
    output_fhe_type: u8,
) -> FheExecuteStep {
    FheExecuteStep::Binary {
        op,
        lhs,
        rhs,
        output_fhe_type,
        output: local_output(),
    }
}

fn sum_step(operands: Vec<FheExecuteOperand>, fhe_type: u8) -> FheExecuteStep {
    FheExecuteStep::Sum {
        operands,
        fhe_type,
        output: local_output(),
    }
}

fn is_in_step(
    value: FheExecuteOperand,
    set: Vec<FheExecuteOperand>,
    fhe_type: u8,
) -> FheExecuteStep {
    FheExecuteStep::IsIn {
        value,
        set,
        fhe_type,
        output: local_output(),
    }
}

fn mul_div_step(
    factor1: FheExecuteOperand,
    factor2: FheExecuteOperand,
    divisor: [u8; 32],
    output_fhe_type: u8,
) -> FheExecuteStep {
    FheExecuteStep::MulDiv {
        factor1,
        factor2,
        divisor,
        output_fhe_type,
        output: local_output(),
    }
}

fn bounded_rand_step(upper_bound: [u8; 32], fhe_type: u8) -> FheExecuteStep {
    FheExecuteStep::RandBounded {
        upper_bound,
        fhe_type,
        output: local_output(),
    }
}

// FheExecution constants (operand handles, scalar values) live in the execution's interned dictionary
// and are referenced by `u8` index (fhevm-internal#1853 W7). The helpers below intern
// through a thread-local dictionary: `persistent`/`scalar` add entries while a test assembles
// steps, and `args` drains the dictionary into the finished execution. Each test runs on its own
// thread, so dictionaries never mix across tests.
std::thread_local! {
    static INTERNED_DICTIONARY: std::cell::RefCell<Vec<[u8; 32]>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

fn intern(bytes: [u8; 32]) -> u8 {
    INTERNED_DICTIONARY.with(|dictionary| {
        let mut dictionary = dictionary.borrow_mut();
        if let Some(index) = dictionary.iter().position(|entry| *entry == bytes) {
            return u8::try_from(index).expect("test dictionary fits u8");
        }
        let index = u8::try_from(dictionary.len()).expect("test dictionary fits u8");
        dictionary.push(bytes);
        index
    })
}

fn args(steps: Vec<FheExecuteStep>) -> FheExecuteArgs {
    FheExecuteArgs {
        account_count: 0,
        dictionary: INTERNED_DICTIONARY.with(|dictionary| dictionary.take()),
        steps,
    }
}

fn local_output() -> FheExecuteOutput {
    FheExecuteOutput::Transient
}

fn scalar(value: [u8; 32]) -> FheExecuteOperand {
    FheExecuteOperand::Scalar {
        value_index: intern(value),
    }
}

fn persistent(handle: Handle) -> FheExecuteOperand {
    FheExecuteOperand::StoredValue {
        handle_index: intern(handle),
        encrypted_value_index: 0,
    }
}

fn verified(input_handle: Handle) -> FheExecuteOperand {
    FheExecuteOperand::VerifiedInput {
        attestation: Box::new(CoprocessorInputAttestation {
            input_handle,
            ct_handles: vec![input_handle],
            handle_index: 0,
            user_address: [0; 32],
            contract_address: [0; 32],
            contract_chain_id: 1,
            extra_data: vec![],
            signatures: vec![[0; 65]],
        }),
    }
}

fn handle(seed: u8, fhe_type: u8) -> Handle {
    let mut handle = [0; 32];
    handle[0] = seed;
    handle[30] = fhe_type;
    handle
}

fn typed(fhe_type: u8, value: u64) -> TypedClearValue {
    TypedClearValue::from_u64(fhe_type, value)
}

fn be(value: u64) -> [u8; 32] {
    let mut bytes = [0; 32];
    bytes[24..].copy_from_slice(&value.to_be_bytes());
    bytes
}
