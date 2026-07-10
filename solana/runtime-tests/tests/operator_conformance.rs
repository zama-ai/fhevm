//! Explicit, filterable contract tests for the `fhe_eval` operator surface.
//!
//! The declarations below are the test-owned support matrix. They deliberately do not derive
//! accepted combinations from the host validators: each generated test sends one canonical plan
//! through the cleartext evaluator, whose validator calls then catch drift from this contract.

mod support;

use std::collections::HashMap;

use support::cleartext_fhe_eval::{evaluate, ClearInputs, TypedClearValue};
use zama_host::{
    FheBinaryOpCode, FheEvalArgs, FheEvalOperand, FheEvalOutput, FheEvalStep, FheTernaryOpCode,
    FheUnaryOpCode,
};

const BOOL: u8 = 0;
const U8: u8 = 2;
const U16: u8 = 3;
const U32: u8 = 4;
const U64: u8 = 5;
const U128: u8 = 6;
const U160: u8 = 7;
const U256: u8 = 8;

fn handle(seed: u8, fhe_type: u8) -> [u8; 32] {
    let mut handle = [0; 32];
    handle[0] = seed;
    handle[30] = fhe_type;
    handle
}

fn scalar(value: u64) -> [u8; 32] {
    TypedClearValue::from_u64(U64, value).value
}

fn durable(handle: [u8; 32]) -> FheEvalOperand {
    FheEvalOperand::AllowedDurable {
        handle,
        encrypted_value_index: 0,
    }
}

fn local() -> FheEvalOutput {
    FheEvalOutput::AllowedLocal
}

fn args(step: FheEvalStep) -> FheEvalArgs {
    FheEvalArgs {
        context_id: [7; 32],
        steps: vec![step],
    }
}

fn clear_inputs(values: &[([u8; 32], u8, u64)]) -> ClearInputs {
    values
        .iter()
        .map(|(handle, fhe_type, value)| (*handle, TypedClearValue::from_u64(*fhe_type, *value)))
        .collect()
}

fn value_as_u64(value: TypedClearValue) -> u64 {
    u64::from_be_bytes(value.value[24..].try_into().unwrap())
}

fn assert_output(plan: FheEvalArgs, inputs: &ClearInputs, fhe_type: u8, expected: u64) {
    assert_typed_output(plan, inputs, TypedClearValue::from_u64(fhe_type, expected));
}

fn assert_typed_output(plan: FheEvalArgs, inputs: &ClearInputs, expected: TypedClearValue) {
    let output = evaluate(&plan, inputs).expect("declared operator contract must be accepted");
    assert_eq!(output.len(), 1);
    assert_eq!(output[0], expected);
}

fn assert_rejected(plan: FheEvalArgs, inputs: &ClearInputs) {
    assert!(evaluate(&plan, inputs).is_err());
}

fn binary_case(
    op: FheBinaryOpCode,
    input_type: u8,
    output_type: u8,
    rhs_is_scalar: bool,
    lhs: u64,
    rhs: u64,
    expected: u64,
) {
    let lhs_handle = handle(1, input_type);
    let rhs_handle = handle(2, input_type);
    let inputs = clear_inputs(&[(lhs_handle, input_type, lhs), (rhs_handle, input_type, rhs)]);
    let rhs = if rhs_is_scalar {
        FheEvalOperand::Scalar(scalar(rhs))
    } else {
        durable(rhs_handle)
    };
    assert_output(
        args(FheEvalStep::Binary {
            op,
            lhs: durable(lhs_handle),
            rhs,
            output_fhe_type: output_type,
            output: local(),
        }),
        &inputs,
        output_type,
        expected,
    );
}

fn full_width_binary_case(
    op: FheBinaryOpCode,
    input_type: u8,
    output_type: u8,
    rhs_is_scalar: bool,
    lhs: [u8; 32],
    rhs: [u8; 32],
    expected: [u8; 32],
) {
    let lhs_handle = handle(1, input_type);
    let rhs_handle = handle(2, input_type);
    let mut inputs = HashMap::from([(lhs_handle, TypedClearValue::from_be_bytes(input_type, lhs))]);
    let rhs = if rhs_is_scalar {
        FheEvalOperand::Scalar(rhs)
    } else {
        inputs.insert(rhs_handle, TypedClearValue::from_be_bytes(input_type, rhs));
        durable(rhs_handle)
    };
    assert_typed_output(
        args(FheEvalStep::Binary {
            op,
            lhs: durable(lhs_handle),
            rhs,
            output_fhe_type: output_type,
            output: local(),
        }),
        &inputs,
        TypedClearValue::from_be_bytes(output_type, expected),
    );
}

macro_rules! binary_contract {
    (
        $name:ident, $op:ident, $lhs:expr, $rhs:expr, $expected:expr;
        encrypted [$($encrypted_name:ident => ($encrypted_input:expr, $encrypted_output:expr)),* $(,)?];
        scalar [$($scalar_name:ident => ($scalar_input:expr, $scalar_output:expr)),* $(,)?]
    ) => {
        mod $name {
            use super::*;

            mod encrypted {
                use super::*;
                $(
                    #[test]
                    fn $encrypted_name() {
                        binary_case(
                            FheBinaryOpCode::$op,
                            $encrypted_input,
                            $encrypted_output,
                            false,
                            $lhs,
                            $rhs,
                            $expected,
                        );
                    }
                )*
            }

            mod scalar {
                use super::*;
                $(
                    #[test]
                    fn $scalar_name() {
                        binary_case(
                            FheBinaryOpCode::$op,
                            $scalar_input,
                            $scalar_output,
                            true,
                            $lhs,
                            $rhs,
                            $expected,
                        );
                    }
                )*
            }
        }
    };
}

macro_rules! scalar_binary_contract {
    (
        $name:ident, $op:ident, $lhs:expr, $rhs:expr, $expected:expr;
        [$($type_name:ident => $fhe_type:expr),* $(,)?]
    ) => {
        mod $name {
            use super::*;

            mod scalar {
                use super::*;
                $(
                    #[test]
                    fn $type_name() {
                        binary_case(
                            FheBinaryOpCode::$op,
                            $fhe_type,
                            $fhe_type,
                            true,
                            $lhs,
                            $rhs,
                            $expected,
                        );
                    }
                )*
            }
        }
    };
}

mod binary {
    use super::*;

    macro_rules! uint8_to_uint128_both_shapes {
        ($name:ident, $op:ident, $lhs:expr, $rhs:expr, $expected:expr) => {
            binary_contract!(
                $name, $op, $lhs, $rhs, $expected;
                encrypted [
                    u8 => (U8, U8), u16 => (U16, U16), u32 => (U32, U32),
                    u64 => (U64, U64), u128 => (U128, U128)
                ];
                scalar [
                    u8 => (U8, U8), u16 => (U16, U16), u32 => (U32, U32),
                    u64 => (U64, U64), u128 => (U128, U128)
                ]
            );
        };
    }

    uint8_to_uint128_both_shapes!(add, Add, 9, 5, 14);
    uint8_to_uint128_both_shapes!(sub, Sub, 9, 5, 4);
    uint8_to_uint128_both_shapes!(mul, Mul, 9, 5, 45);
    uint8_to_uint128_both_shapes!(min, Min, 9, 5, 5);
    uint8_to_uint128_both_shapes!(max, Max, 9, 5, 9);

    scalar_binary_contract!(
        div, Div, 9, 4, 2;
        [
            u8 => U8, u16 => U16, u32 => U32, u64 => U64, u128 => U128
        ]
    );
    scalar_binary_contract!(
        rem, Rem, 9, 4, 1;
        [
            u8 => U8, u16 => U16, u32 => U32, u64 => U64, u128 => U128
        ]
    );

    macro_rules! bitwise_both_shapes {
        ($name:ident, $op:ident, $lhs:expr, $rhs:expr, $expected:expr) => {
            binary_contract!(
                $name, $op, $lhs, $rhs, $expected;
                encrypted [
                    bool => (BOOL, BOOL), u8 => (U8, U8), u16 => (U16, U16),
                    u32 => (U32, U32), u64 => (U64, U64), u128 => (U128, U128),
                    u256 => (U256, U256)
                ];
                scalar [
                    bool => (BOOL, BOOL), u8 => (U8, U8), u16 => (U16, U16),
                    u32 => (U32, U32), u64 => (U64, U64), u128 => (U128, U128),
                    u256 => (U256, U256)
                ]
            );
        };
    }

    // These vectors have the same result for Bool and every admitted integer width.
    bitwise_both_shapes!(and, And, 1, 1, 1);
    bitwise_both_shapes!(or, Or, 1, 0, 1);
    bitwise_both_shapes!(xor, Xor, 1, 0, 1);

    macro_rules! shift_both_shapes {
        ($name:ident, $op:ident, $lhs:expr, $rhs:expr, $expected:expr) => {
            binary_contract!(
                $name, $op, $lhs, $rhs, $expected;
                encrypted [
                    u8 => (U8, U8), u16 => (U16, U16), u32 => (U32, U32),
                    u64 => (U64, U64), u128 => (U128, U128), u256 => (U256, U256)
                ];
                scalar [
                    u8 => (U8, U8), u16 => (U16, U16), u32 => (U32, U32),
                    u64 => (U64, U64), u128 => (U128, U128), u256 => (U256, U256)
                ]
            );
        };
    }

    shift_both_shapes!(shl, Shl, 3, 2, 12);
    shift_both_shapes!(shr, Shr, 12, 2, 3);
    shift_both_shapes!(rotl, Rotl, 1, 1, 2);
    shift_both_shapes!(rotr, Rotr, 2, 1, 1);

    macro_rules! equality_both_shapes {
        ($name:ident, $op:ident, $lhs:expr, $rhs:expr, $expected:expr) => {
            binary_contract!(
                $name, $op, $lhs, $rhs, $expected;
                encrypted [
                    bool => (BOOL, BOOL), u8 => (U8, BOOL), u16 => (U16, BOOL),
                    u32 => (U32, BOOL), u64 => (U64, BOOL), u128 => (U128, BOOL),
                    u160 => (U160, BOOL), u256 => (U256, BOOL)
                ];
                scalar [
                    bool => (BOOL, BOOL), u8 => (U8, BOOL), u16 => (U16, BOOL),
                    u32 => (U32, BOOL), u64 => (U64, BOOL), u128 => (U128, BOOL),
                    u160 => (U160, BOOL), u256 => (U256, BOOL)
                ]
            );
        };
    }

    equality_both_shapes!(eq, Eq, 1, 1, 1);
    equality_both_shapes!(ne, Ne, 0, 1, 1);

    macro_rules! ordered_both_shapes {
        ($name:ident, $op:ident, $lhs:expr, $rhs:expr, $expected:expr) => {
            binary_contract!(
                $name, $op, $lhs, $rhs, $expected;
                encrypted [
                    u8 => (U8, BOOL), u16 => (U16, BOOL), u32 => (U32, BOOL),
                    u64 => (U64, BOOL), u128 => (U128, BOOL)
                ];
                scalar [
                    u8 => (U8, BOOL), u16 => (U16, BOOL), u32 => (U32, BOOL),
                    u64 => (U64, BOOL), u128 => (U128, BOOL)
                ]
            );
        };
    }

    ordered_both_shapes!(ge, Ge, 10, 9, 1);
    ordered_both_shapes!(gt, Gt, 10, 9, 1);
    ordered_both_shapes!(le, Le, 9, 10, 1);
    ordered_both_shapes!(lt, Lt, 9, 10, 1);
}

fn unary_case(op: FheUnaryOpCode, input_type: u8, output_type: u8, input: u64, expected: u64) {
    let input_handle = handle(1, input_type);
    assert_output(
        args(FheEvalStep::Unary {
            op,
            operand: durable(input_handle),
            output_fhe_type: output_type,
            output: local(),
        }),
        &clear_inputs(&[(input_handle, input_type, input)]),
        output_type,
        expected,
    );
}

fn unary_not_case(fhe_type: u8) {
    let input_handle = handle(1, fhe_type);
    let inputs = HashMap::from([(
        input_handle,
        TypedClearValue::from_be_bytes(fhe_type, [u8::MAX; 32]),
    )]);
    assert_output(
        args(FheEvalStep::Unary {
            op: FheUnaryOpCode::Not,
            operand: durable(input_handle),
            output_fhe_type: fhe_type,
            output: local(),
        }),
        &inputs,
        fhe_type,
        0,
    );
}

macro_rules! unary_same_type_contract {
    ($name:ident, $op:ident, $input:expr, $expected:expr; [$($type_name:ident => $fhe_type:expr),* $(,)?]) => {
        mod $name {
            use super::*;
            $(
                #[test]
                fn $type_name() {
                    unary_case(FheUnaryOpCode::$op, $fhe_type, $fhe_type, $input, $expected);
                }
            )*
        }
    };
}

macro_rules! cast_outputs {
    ($input_name:ident, $input_type:expr, $input:expr; [$($output_name:ident => $output_type:expr),* $(,)?]) => {
        mod $input_name {
            use super::*;
            $(
                #[test]
                fn $output_name() {
                    unary_case(FheUnaryOpCode::Cast, $input_type, $output_type, $input, $input);
                }
            )*
        }
    };
}

mod unary {
    use super::*;

    unary_same_type_contract!(
        neg, Neg, 0, 0;
        [u8 => U8, u16 => U16, u32 => U32, u64 => U64, u128 => U128, u256 => U256]
    );
    mod not {
        use super::*;
        macro_rules! cases {
            ($($name:ident => $fhe_type:expr),* $(,)?) => {
                $(#[test] fn $name() { unary_not_case($fhe_type); })*
            };
        }
        cases!(
            bool => BOOL, u8 => U8, u16 => U16, u32 => U32,
            u64 => U64, u128 => U128, u256 => U256
        );
    }

    mod cast {
        use super::*;

        cast_outputs!(bool, BOOL, 1; [u8 => U8, u16 => U16, u32 => U32, u64 => U64, u128 => U128, u256 => U256]);
        cast_outputs!(u8, U8, 7; [u16 => U16, u32 => U32, u64 => U64, u128 => U128, u256 => U256]);
        cast_outputs!(u16, U16, 7; [u8 => U8, u32 => U32, u64 => U64, u128 => U128, u256 => U256]);
        cast_outputs!(u32, U32, 7; [u8 => U8, u16 => U16, u64 => U64, u128 => U128, u256 => U256]);
        cast_outputs!(u64, U64, 7; [u8 => U8, u16 => U16, u32 => U32, u128 => U128, u256 => U256]);
        cast_outputs!(u128, U128, 7; [u8 => U8, u16 => U16, u32 => U32, u64 => U64, u256 => U256]);
        cast_outputs!(u256, U256, 7; [u8 => U8, u16 => U16, u32 => U32, u64 => U64, u128 => U128]);
    }
}

fn ternary_case(fhe_type: u8) {
    let control = handle(1, BOOL);
    let if_true = handle(2, fhe_type);
    let if_false = handle(3, fhe_type);
    assert_output(
        args(FheEvalStep::Ternary {
            op: FheTernaryOpCode::IfThenElse,
            control: durable(control),
            if_true: durable(if_true),
            if_false: durable(if_false),
            output_fhe_type: fhe_type,
            output: local(),
        }),
        &clear_inputs(&[
            (control, BOOL, 1),
            (if_true, fhe_type, 1),
            (if_false, fhe_type, 0),
        ]),
        fhe_type,
        1,
    );
}

macro_rules! typed_contract {
    ($helper:ident; [$($name:ident => $fhe_type:expr),* $(,)?]) => {
        $(
            #[test]
            fn $name() {
                $helper($fhe_type);
            }
        )*
    };
}

mod ternary {
    use super::*;

    mod if_then_else {
        use super::*;
        typed_contract!(ternary_case; [
            bool => BOOL, u8 => U8, u16 => U16, u32 => U32,
            u64 => U64, u128 => U128, u160 => U160, u256 => U256
        ]);
    }
}

fn trivial_encrypt_case(fhe_type: u8, expected: u64) {
    assert_output(
        args(FheEvalStep::TrivialEncrypt {
            plaintext: scalar(9),
            fhe_type,
            output: local(),
        }),
        &ClearInputs::new(),
        fhe_type,
        expected,
    );
}

fn rand_case(fhe_type: u8) {
    let plan = args(FheEvalStep::Rand {
        fhe_type,
        output: local(),
    });
    let first = evaluate(&plan, &ClearInputs::new()).expect("declared rand type must be accepted");
    let second = evaluate(&plan, &ClearInputs::new()).expect("rand mock must be repeatable");
    assert_eq!(first, second);
    assert_eq!(first[0].fhe_type, fhe_type);
    if fhe_type == BOOL {
        assert!(value_as_u64(first[0]) <= 1);
    } else {
        let leading_bytes = match fhe_type {
            U8 => 31,
            U16 => 30,
            U32 => 28,
            U64 => 24,
            U128 => 16,
            U256 => 0,
            _ => panic!("undeclared Rand type"),
        };
        assert!(first[0].value[..leading_bytes]
            .iter()
            .all(|byte| *byte == 0));
    }
}

fn bounded_rand_case(fhe_type: u8) {
    assert_output(
        args(FheEvalStep::RandBounded {
            upper_bound: scalar(1),
            fhe_type,
            output: local(),
        }),
        &ClearInputs::new(),
        fhe_type,
        0,
    );
}

mod birth {
    use super::*;

    mod trivial_encrypt {
        use super::*;

        #[test]
        fn bool() {
            trivial_encrypt_case(BOOL, 1);
        }
        macro_rules! uint_cases {
            ($($name:ident => $fhe_type:expr),* $(,)?) => {
                $(
                    #[test]
                    fn $name() {
                        trivial_encrypt_case($fhe_type, 9);
                    }
                )*
            };
        }
        uint_cases!(u8 => U8, u16 => U16, u32 => U32, u64 => U64, u128 => U128, u160 => U160, u256 => U256);
    }

    mod rand {
        use super::*;
        typed_contract!(rand_case; [
            bool => BOOL, u8 => U8, u16 => U16, u32 => U32,
            u64 => U64, u128 => U128, u256 => U256
        ]);
    }

    mod bounded_rand {
        use super::*;
        typed_contract!(bounded_rand_case; [
            u8 => U8, u16 => U16, u32 => U32,
            u64 => U64, u128 => U128, u256 => U256
        ]);
    }
}

fn sum_case(fhe_type: u8) {
    let first = handle(1, fhe_type);
    let second = handle(2, fhe_type);
    assert_output(
        args(FheEvalStep::Sum {
            operands: vec![durable(first), durable(second)],
            fhe_type,
            output: local(),
        }),
        &clear_inputs(&[(first, fhe_type, 4), (second, fhe_type, 5)]),
        fhe_type,
        9,
    );
}

fn is_in_case(fhe_type: u8) {
    let value = handle(1, fhe_type);
    let member = handle(2, fhe_type);
    assert_output(
        args(FheEvalStep::IsIn {
            value: durable(value),
            set: vec![durable(member)],
            fhe_type,
            output: local(),
        }),
        &clear_inputs(&[(value, fhe_type, 9), (member, fhe_type, 9)]),
        BOOL,
        1,
    );
}

fn mul_div_case(fhe_type: u8, factor2_is_scalar: bool) {
    let factor1 = handle(1, fhe_type);
    let factor2 = handle(2, fhe_type);
    let inputs = clear_inputs(&[(factor1, fhe_type, 9), (factor2, fhe_type, 4)]);
    let factor2 = if factor2_is_scalar {
        FheEvalOperand::Scalar(scalar(4))
    } else {
        durable(factor2)
    };
    assert_output(
        args(FheEvalStep::MulDiv {
            factor1: durable(factor1),
            factor2,
            divisor: scalar(2),
            output_fhe_type: fhe_type,
            output: local(),
        }),
        &inputs,
        fhe_type,
        18,
    );
}

mod composite {
    use super::*;

    mod sum {
        use super::*;
        typed_contract!(sum_case; [u8 => U8, u16 => U16, u32 => U32, u64 => U64, u128 => U128]);
    }

    mod is_in {
        use super::*;
        typed_contract!(is_in_case; [
            u8 => U8, u16 => U16, u32 => U32, u64 => U64,
            u128 => U128, u160 => U160, u256 => U256
        ]);
    }

    mod mul_div {
        use super::*;

        mod encrypted {
            use super::*;
            macro_rules! cases {
                ($($name:ident => $fhe_type:expr),* $(,)?) => {
                    $(#[test] fn $name() { mul_div_case($fhe_type, false); })*
                };
            }
            cases!(u8 => U8, u16 => U16, u32 => U32, u64 => U64);
        }

        mod scalar {
            use super::*;
            macro_rules! cases {
                ($($name:ident => $fhe_type:expr),* $(,)?) => {
                    $(#[test] fn $name() { mul_div_case($fhe_type, true); })*
                };
            }
            cases!(u8 => U8, u16 => U16, u32 => U32, u64 => U64);
        }
    }
}

mod edges {
    use super::*;

    mod accepted {
        use super::*;

        #[test]
        fn u8_add_wraps() {
            binary_case(FheBinaryOpCode::Add, U8, U8, true, 250, 10, 4);
        }

        #[test]
        fn u8_sub_underflows() {
            binary_case(FheBinaryOpCode::Sub, U8, U8, false, 2, 3, 255);
        }

        #[test]
        fn u8_bitwise_and_has_distinct_output() {
            binary_case(FheBinaryOpCode::And, U8, U8, false, 0b1010, 0b1100, 0b1000);
        }

        #[test]
        fn u8_bitwise_or_has_distinct_output() {
            binary_case(FheBinaryOpCode::Or, U8, U8, false, 0b1010, 0b1100, 0b1110);
        }

        #[test]
        fn u8_bitwise_xor_has_distinct_output() {
            binary_case(FheBinaryOpCode::Xor, U8, U8, false, 0b1010, 0b1100, 0b0110);
        }

        #[test]
        fn equality_can_be_false() {
            binary_case(FheBinaryOpCode::Eq, U64, BOOL, false, 9, 10, 0);
        }

        #[test]
        fn inequality_can_be_false() {
            binary_case(FheBinaryOpCode::Ne, U64, BOOL, false, 9, 9, 0);
        }

        #[test]
        fn greater_or_equal_accepts_equality() {
            binary_case(FheBinaryOpCode::Ge, U64, BOOL, false, 9, 9, 1);
        }

        #[test]
        fn greater_than_rejects_equality() {
            binary_case(FheBinaryOpCode::Gt, U64, BOOL, false, 9, 9, 0);
        }

        #[test]
        fn less_or_equal_accepts_equality() {
            binary_case(FheBinaryOpCode::Le, U64, BOOL, false, 9, 9, 1);
        }

        #[test]
        fn less_than_rejects_equality() {
            binary_case(FheBinaryOpCode::Lt, U64, BOOL, false, 9, 9, 0);
        }

        #[test]
        fn u8_nonzero_negation_is_modular() {
            unary_case(FheUnaryOpCode::Neg, U8, U8, 1, 255);
        }

        #[test]
        fn ternary_false_control_selects_false_branch() {
            let control = handle(1, BOOL);
            let if_true = handle(2, U64);
            let if_false = handle(3, U64);
            assert_output(
                args(FheEvalStep::Ternary {
                    op: FheTernaryOpCode::IfThenElse,
                    control: durable(control),
                    if_true: durable(if_true),
                    if_false: durable(if_false),
                    output_fhe_type: U64,
                    output: local(),
                }),
                &clear_inputs(&[(control, BOOL, 0), (if_true, U64, 11), (if_false, U64, 22)]),
                U64,
                22,
            );
        }

        #[test]
        fn u8_shift_amount_wraps_at_width() {
            binary_case(FheBinaryOpCode::Shl, U8, U8, true, 1, 9, 2);
        }

        #[test]
        fn u8_rotate_right_preserves_wrapped_bits() {
            binary_case(FheBinaryOpCode::Rotr, U8, U8, true, 3, 1, 0x81);
        }

        #[test]
        fn u256_bitwise_xor_preserves_high_bits() {
            let mut lhs = [0; 32];
            lhs[0] = 0x80;
            lhs[31] = 0x0f;
            let mut rhs = [0; 32];
            rhs[0] = 0x40;
            rhs[31] = 0xf0;
            let mut expected = [0; 32];
            expected[0] = 0xc0;
            expected[31] = 0xff;
            full_width_binary_case(FheBinaryOpCode::Xor, U256, U256, false, lhs, rhs, expected);
        }

        #[test]
        fn u256_scalar_bitwise_xor_reads_high_bytes() {
            let mut lhs = [0; 32];
            lhs[0] = 0x80;
            let mut scalar_rhs = [0; 32];
            scalar_rhs[0] = 0x40;
            let mut expected = [0; 32];
            expected[0] = 0xc0;
            full_width_binary_case(
                FheBinaryOpCode::Xor,
                U256,
                U256,
                true,
                lhs,
                scalar_rhs,
                expected,
            );
        }

        #[test]
        fn u256_shift_right_preserves_high_bits() {
            let mut lhs = [0; 32];
            lhs[0] = 0x80;
            let mut rhs = [0; 32];
            rhs[31] = 1;
            let mut expected = [0; 32];
            expected[0] = 0x40;
            full_width_binary_case(FheBinaryOpCode::Shr, U256, U256, false, lhs, rhs, expected);
        }

        #[test]
        fn u256_rotate_left_wraps_the_top_bit() {
            let mut lhs = [0; 32];
            lhs[0] = 0x80;
            let mut rhs = [0; 32];
            rhs[31] = 1;
            let mut expected = [0; 32];
            expected[31] = 1;
            full_width_binary_case(FheBinaryOpCode::Rotl, U256, U256, false, lhs, rhs, expected);
        }

        #[test]
        fn u160_equality_reads_bits_above_u64() {
            let mut lhs = [0; 32];
            lhs[12] = 1;
            full_width_binary_case(
                FheBinaryOpCode::Eq,
                U160,
                BOOL,
                false,
                lhs,
                [0; 32],
                [0; 32],
            );
        }

        #[test]
        fn u160_scalar_equality_reads_high_bytes() {
            let mut lhs = [0; 32];
            lhs[12] = 0x80;
            let scalar_rhs = lhs;
            let mut expected = [0; 32];
            expected[31] = 1;
            full_width_binary_case(
                FheBinaryOpCode::Eq,
                U160,
                BOOL,
                true,
                lhs,
                scalar_rhs,
                expected,
            );
        }

        #[test]
        fn u256_equality_reads_bits_above_u64() {
            let mut lhs = [0; 32];
            lhs[0] = 1;
            full_width_binary_case(
                FheBinaryOpCode::Eq,
                U256,
                BOOL,
                false,
                lhs,
                [0; 32],
                [0; 32],
            );
        }

        #[test]
        fn cast_widens_u8_to_u256() {
            unary_case(FheUnaryOpCode::Cast, U8, U256, 255, 255);
        }

        #[test]
        fn cast_truncates_u256_to_u8() {
            let input = handle(1, U256);
            let mut value = [0; 32];
            value[0] = 1;
            value[31] = 7;
            let inputs = HashMap::from([(input, TypedClearValue::from_be_bytes(U256, value))]);
            assert_output(
                args(FheEvalStep::Unary {
                    op: FheUnaryOpCode::Cast,
                    operand: durable(input),
                    output_fhe_type: U8,
                    output: local(),
                }),
                &inputs,
                U8,
                7,
            );
        }

        #[test]
        fn empty_sum_is_zero() {
            assert_output(
                args(FheEvalStep::Sum {
                    operands: vec![],
                    fhe_type: U64,
                    output: local(),
                }),
                &ClearInputs::new(),
                U64,
                0,
            );
        }

        #[test]
        fn membership_miss_is_false() {
            let value = handle(1, U160);
            let member = handle(2, U160);
            assert_output(
                args(FheEvalStep::IsIn {
                    value: durable(value),
                    set: vec![durable(member)],
                    fhe_type: U160,
                    output: local(),
                }),
                &clear_inputs(&[(value, U160, 9), (member, U160, 10)]),
                BOOL,
                0,
            );
        }

        #[test]
        fn empty_membership_set_is_false() {
            let value = handle(1, U160);
            assert_output(
                args(FheEvalStep::IsIn {
                    value: durable(value),
                    set: vec![],
                    fhe_type: U160,
                    output: local(),
                }),
                &clear_inputs(&[(value, U160, 9)]),
                BOOL,
                0,
            );
        }

        #[test]
        fn u160_membership_reads_high_bits() {
            let value = handle(1, U160);
            let member = handle(2, U160);
            let mut value_bytes = [0; 32];
            value_bytes[12] = 1;
            value_bytes[31] = 7;
            let mut member_bytes = [0; 32];
            member_bytes[13] = 1;
            member_bytes[31] = 7;
            let inputs = HashMap::from([
                (value, TypedClearValue::from_be_bytes(U160, value_bytes)),
                (member, TypedClearValue::from_be_bytes(U160, member_bytes)),
            ]);
            assert_output(
                args(FheEvalStep::IsIn {
                    value: durable(value),
                    set: vec![durable(member)],
                    fhe_type: U160,
                    output: local(),
                }),
                &inputs,
                BOOL,
                0,
            );
        }

        #[test]
        fn u256_membership_reads_high_bits() {
            let value = handle(1, U256);
            let member = handle(2, U256);
            let mut value_bytes = [0; 32];
            value_bytes[0] = 1;
            value_bytes[31] = 7;
            let mut member_bytes = [0; 32];
            member_bytes[1] = 1;
            member_bytes[31] = 7;
            let inputs = HashMap::from([
                (value, TypedClearValue::from_be_bytes(U256, value_bytes)),
                (member, TypedClearValue::from_be_bytes(U256, member_bytes)),
            ]);
            assert_output(
                args(FheEvalStep::IsIn {
                    value: durable(value),
                    set: vec![durable(member)],
                    fhe_type: U256,
                    output: local(),
                }),
                &inputs,
                BOOL,
                0,
            );
        }

        #[test]
        fn mul_div_widens_before_division() {
            let factor = handle(1, U16);
            assert_output(
                args(FheEvalStep::MulDiv {
                    factor1: durable(factor),
                    factor2: FheEvalOperand::Scalar(scalar(400)),
                    divisor: scalar(3),
                    output_fhe_type: U16,
                    output: local(),
                }),
                &clear_inputs(&[(factor, U16, 200)]),
                U16,
                26_666,
            );
        }
    }

    mod rejected {
        use super::*;

        #[test]
        fn scalar_divisor_zero() {
            let lhs = handle(1, U8);
            assert_rejected(
                args(FheEvalStep::Binary {
                    op: FheBinaryOpCode::Div,
                    lhs: durable(lhs),
                    rhs: FheEvalOperand::Scalar([0; 32]),
                    output_fhe_type: U8,
                    output: local(),
                }),
                &clear_inputs(&[(lhs, U8, 9)]),
            );
        }

        #[test]
        fn scalar_divisor_truncated_to_zero() {
            let lhs = handle(1, U8);
            let mut high_only = [0; 32];
            high_only[30] = 1;
            assert_rejected(
                args(FheEvalStep::Binary {
                    op: FheBinaryOpCode::Div,
                    lhs: durable(lhs),
                    rhs: FheEvalOperand::Scalar(high_only),
                    output_fhe_type: U8,
                    output: local(),
                }),
                &clear_inputs(&[(lhs, U8, 9)]),
            );
        }
    }
}

fn binary_plan(
    op: FheBinaryOpCode,
    lhs_type: u8,
    rhs_type: u8,
    rhs_is_scalar: bool,
    output_type: u8,
) -> (FheEvalArgs, ClearInputs) {
    let lhs = handle(1, lhs_type);
    let rhs = handle(2, rhs_type);
    let inputs = clear_inputs(&[(lhs, lhs_type, 9), (rhs, rhs_type, 3)]);
    let rhs = if rhs_is_scalar {
        FheEvalOperand::Scalar(scalar(3))
    } else {
        durable(rhs)
    };
    (
        args(FheEvalStep::Binary {
            op,
            lhs: durable(lhs),
            rhs,
            output_fhe_type: output_type,
            output: local(),
        }),
        inputs,
    )
}

mod rejected {
    use super::*;

    #[test]
    fn scalar_lhs() {
        let plan = args(FheEvalStep::Binary {
            op: FheBinaryOpCode::Add,
            lhs: FheEvalOperand::Scalar(scalar(1)),
            rhs: FheEvalOperand::Scalar(scalar(2)),
            output_fhe_type: U8,
            output: local(),
        });
        assert_rejected(plan, &ClearInputs::new());
    }

    #[test]
    fn arithmetic_u160() {
        let (plan, inputs) = binary_plan(FheBinaryOpCode::Add, U160, U160, false, U160);
        assert_rejected(plan, &inputs);
    }

    #[test]
    fn arithmetic_u256() {
        let (plan, inputs) = binary_plan(FheBinaryOpCode::Mul, U256, U256, false, U256);
        assert_rejected(plan, &inputs);
    }

    #[test]
    fn encrypted_divisor() {
        let (plan, inputs) = binary_plan(FheBinaryOpCode::Div, U64, U64, false, U64);
        assert_rejected(plan, &inputs);
    }

    #[test]
    fn encrypted_remainder_divisor() {
        let (plan, inputs) = binary_plan(FheBinaryOpCode::Rem, U64, U64, false, U64);
        assert_rejected(plan, &inputs);
    }

    #[test]
    fn bitwise_u160() {
        let (plan, inputs) = binary_plan(FheBinaryOpCode::And, U160, U160, false, U160);
        assert_rejected(plan, &inputs);
    }

    #[test]
    fn ordered_bool() {
        let (plan, inputs) = binary_plan(FheBinaryOpCode::Ge, BOOL, BOOL, false, BOOL);
        assert_rejected(plan, &inputs);
    }

    #[test]
    fn ordered_u256() {
        let (plan, inputs) = binary_plan(FheBinaryOpCode::Lt, U256, U256, false, BOOL);
        assert_rejected(plan, &inputs);
    }

    #[test]
    fn mixed_encrypted_types() {
        let (plan, inputs) = binary_plan(FheBinaryOpCode::Eq, U8, U16, false, BOOL);
        assert_rejected(plan, &inputs);
    }

    #[test]
    fn shift_bool() {
        let (plan, inputs) = binary_plan(FheBinaryOpCode::Shl, BOOL, BOOL, false, BOOL);
        assert_rejected(plan, &inputs);
    }

    #[test]
    fn same_type_cast() {
        let input = handle(1, U64);
        assert_rejected(
            args(FheEvalStep::Unary {
                op: FheUnaryOpCode::Cast,
                operand: durable(input),
                output_fhe_type: U64,
                output: local(),
            }),
            &clear_inputs(&[(input, U64, 7)]),
        );
    }

    #[test]
    fn cast_to_u160() {
        let input = handle(1, U64);
        assert_rejected(
            args(FheEvalStep::Unary {
                op: FheUnaryOpCode::Cast,
                operand: durable(input),
                output_fhe_type: U160,
                output: local(),
            }),
            &clear_inputs(&[(input, U64, 7)]),
        );
    }

    #[test]
    fn cast_to_bool() {
        let input = handle(1, U64);
        assert_rejected(
            args(FheEvalStep::Unary {
                op: FheUnaryOpCode::Cast,
                operand: durable(input),
                output_fhe_type: BOOL,
                output: local(),
            }),
            &clear_inputs(&[(input, U64, 7)]),
        );
    }

    #[test]
    fn cast_from_u160() {
        let input = handle(1, U160);
        assert_rejected(
            args(FheEvalStep::Unary {
                op: FheUnaryOpCode::Cast,
                operand: durable(input),
                output_fhe_type: U64,
                output: local(),
            }),
            &clear_inputs(&[(input, U160, 7)]),
        );
    }

    #[test]
    fn ternary_non_bool_control() {
        let control = handle(1, U8);
        let branch = handle(2, U8);
        assert_rejected(
            args(FheEvalStep::Ternary {
                op: FheTernaryOpCode::IfThenElse,
                control: durable(control),
                if_true: durable(branch),
                if_false: durable(branch),
                output_fhe_type: U8,
                output: local(),
            }),
            &clear_inputs(&[(control, U8, 1), (branch, U8, 7)]),
        );
    }

    #[test]
    fn rand_u160() {
        assert_rejected(
            args(FheEvalStep::Rand {
                fhe_type: U160,
                output: local(),
            }),
            &ClearInputs::new(),
        );
    }

    #[test]
    fn bounded_rand_bool() {
        assert_rejected(
            args(FheEvalStep::RandBounded {
                upper_bound: scalar(1),
                fhe_type: BOOL,
                output: local(),
            }),
            &ClearInputs::new(),
        );
    }

    #[test]
    fn bounded_rand_u160() {
        assert_rejected(
            args(FheEvalStep::RandBounded {
                upper_bound: scalar(1),
                fhe_type: U160,
                output: local(),
            }),
            &ClearInputs::new(),
        );
    }

    #[test]
    fn bounded_rand_non_power_of_two() {
        assert_rejected(
            args(FheEvalStep::RandBounded {
                upper_bound: scalar(3),
                fhe_type: U8,
                output: local(),
            }),
            &ClearInputs::new(),
        );
    }

    #[test]
    fn sum_u256() {
        assert_rejected(
            args(FheEvalStep::Sum {
                operands: vec![],
                fhe_type: U256,
                output: local(),
            }),
            &ClearInputs::new(),
        );
    }

    #[test]
    fn is_in_bool() {
        let value = handle(1, BOOL);
        assert_rejected(
            args(FheEvalStep::IsIn {
                value: durable(value),
                set: vec![],
                fhe_type: BOOL,
                output: local(),
            }),
            &clear_inputs(&[(value, BOOL, 1)]),
        );
    }

    #[test]
    fn mul_div_u128() {
        let factor = handle(1, U128);
        assert_rejected(
            args(FheEvalStep::MulDiv {
                factor1: durable(factor),
                factor2: FheEvalOperand::Scalar(scalar(2)),
                divisor: scalar(1),
                output_fhe_type: U128,
                output: local(),
            }),
            &clear_inputs(&[(factor, U128, 9)]),
        );
    }
}

#[test]
fn declared_binary_opcodes_are_exhaustive() {
    // This match is deliberately separate from the production validators. Adding an opcode must
    // make this test target fail to compile until its contract declaration above is updated.
    fn declared(op: FheBinaryOpCode) -> bool {
        match op {
            FheBinaryOpCode::Add
            | FheBinaryOpCode::Sub
            | FheBinaryOpCode::Mul
            | FheBinaryOpCode::Div
            | FheBinaryOpCode::Rem
            | FheBinaryOpCode::And
            | FheBinaryOpCode::Or
            | FheBinaryOpCode::Xor
            | FheBinaryOpCode::Shl
            | FheBinaryOpCode::Shr
            | FheBinaryOpCode::Rotl
            | FheBinaryOpCode::Rotr
            | FheBinaryOpCode::Eq
            | FheBinaryOpCode::Ne
            | FheBinaryOpCode::Ge
            | FheBinaryOpCode::Gt
            | FheBinaryOpCode::Le
            | FheBinaryOpCode::Lt
            | FheBinaryOpCode::Min
            | FheBinaryOpCode::Max => true,
        }
    }

    assert!(declared(FheBinaryOpCode::Add));
}

#[test]
fn declared_unary_opcodes_are_exhaustive() {
    fn declared(op: FheUnaryOpCode) -> bool {
        match op {
            FheUnaryOpCode::Neg | FheUnaryOpCode::Not | FheUnaryOpCode::Cast => true,
        }
    }

    assert!(declared(FheUnaryOpCode::Neg));
}

#[test]
fn declared_ternary_opcodes_are_exhaustive() {
    fn declared(op: FheTernaryOpCode) -> bool {
        match op {
            FheTernaryOpCode::IfThenElse => true,
        }
    }

    assert!(declared(FheTernaryOpCode::IfThenElse));
}
