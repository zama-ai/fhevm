mod support;

use std::collections::HashMap;

use support::cleartext_fhe_eval::{evaluate, ClearInputs, Handle, TypedClearValue};
use zama_host::{
    FheBinaryOpCode, FheEvalArgs, FheEvalOperand, FheEvalOutput, FheEvalStep, FheUnaryOpCode,
};

const BOOL: u8 = 0;
const U8: u8 = 2;
const U16: u8 = 3;
const U64: u8 = 5;
const U160: u8 = 7;
const U256: u8 = 8;

#[test]
fn binary_operations_follow_width_and_scalar_semantics() {
    struct BinaryCase {
        name: &'static str,
        op: FheBinaryOpCode,
        input_type: u8,
        lhs: u64,
        rhs: u64,
        rhs_is_scalar: bool,
        output_type: u8,
        expected: u64,
    }

    let cases = [
        BinaryCase {
            name: "wrapping add",
            op: FheBinaryOpCode::Add,
            input_type: U8,
            lhs: 250,
            rhs: 10,
            rhs_is_scalar: true,
            output_type: U8,
            expected: 4,
        },
        BinaryCase {
            name: "wrapping sub",
            op: FheBinaryOpCode::Sub,
            input_type: U8,
            lhs: 2,
            rhs: 3,
            rhs_is_scalar: false,
            output_type: U8,
            expected: 255,
        },
        BinaryCase {
            name: "wrapping mul",
            op: FheBinaryOpCode::Mul,
            input_type: U16,
            lhs: 300,
            rhs: 300,
            rhs_is_scalar: false,
            output_type: U16,
            expected: 24_464,
        },
        BinaryCase {
            name: "scalar div",
            op: FheBinaryOpCode::Div,
            input_type: U16,
            lhs: 100,
            rhs: 3,
            rhs_is_scalar: true,
            output_type: U16,
            expected: 33,
        },
        BinaryCase {
            name: "scalar rem",
            op: FheBinaryOpCode::Rem,
            input_type: U16,
            lhs: 100,
            rhs: 3,
            rhs_is_scalar: true,
            output_type: U16,
            expected: 1,
        },
        BinaryCase {
            name: "bitwise and",
            op: FheBinaryOpCode::And,
            input_type: U8,
            lhs: 0b1010,
            rhs: 0b1100,
            rhs_is_scalar: true,
            output_type: U8,
            expected: 0b1000,
        },
        BinaryCase {
            name: "bitwise or",
            op: FheBinaryOpCode::Or,
            input_type: U8,
            lhs: 0b1010,
            rhs: 0b1100,
            rhs_is_scalar: false,
            output_type: U8,
            expected: 0b1110,
        },
        BinaryCase {
            name: "bitwise xor",
            op: FheBinaryOpCode::Xor,
            input_type: U8,
            lhs: 0b1010,
            rhs: 0b1100,
            rhs_is_scalar: true,
            output_type: U8,
            expected: 0b0110,
        },
        BinaryCase {
            name: "left shift wraps",
            op: FheBinaryOpCode::Shl,
            input_type: U8,
            lhs: 1,
            rhs: 9,
            rhs_is_scalar: true,
            output_type: U8,
            expected: 2,
        },
        BinaryCase {
            name: "right shift wraps",
            op: FheBinaryOpCode::Shr,
            input_type: U8,
            lhs: 0x80,
            rhs: 9,
            rhs_is_scalar: true,
            output_type: U8,
            expected: 0x40,
        },
        BinaryCase {
            name: "rotate left",
            op: FheBinaryOpCode::Rotl,
            input_type: U8,
            lhs: 0x81,
            rhs: 1,
            rhs_is_scalar: true,
            output_type: U8,
            expected: 3,
        },
        BinaryCase {
            name: "rotate right",
            op: FheBinaryOpCode::Rotr,
            input_type: U8,
            lhs: 3,
            rhs: 1,
            rhs_is_scalar: true,
            output_type: U8,
            expected: 0x81,
        },
        BinaryCase {
            name: "equality",
            op: FheBinaryOpCode::Eq,
            input_type: U64,
            lhs: 9,
            rhs: 9,
            rhs_is_scalar: false,
            output_type: BOOL,
            expected: 1,
        },
        BinaryCase {
            name: "inequality",
            op: FheBinaryOpCode::Ne,
            input_type: U64,
            lhs: 9,
            rhs: 10,
            rhs_is_scalar: true,
            output_type: BOOL,
            expected: 1,
        },
        BinaryCase {
            name: "greater or equal",
            op: FheBinaryOpCode::Ge,
            input_type: U64,
            lhs: 10,
            rhs: 9,
            rhs_is_scalar: true,
            output_type: BOOL,
            expected: 1,
        },
        BinaryCase {
            name: "greater than",
            op: FheBinaryOpCode::Gt,
            input_type: U64,
            lhs: 10,
            rhs: 9,
            rhs_is_scalar: false,
            output_type: BOOL,
            expected: 1,
        },
        BinaryCase {
            name: "less or equal",
            op: FheBinaryOpCode::Le,
            input_type: U64,
            lhs: 10,
            rhs: 9,
            rhs_is_scalar: true,
            output_type: BOOL,
            expected: 0,
        },
        BinaryCase {
            name: "less than",
            op: FheBinaryOpCode::Lt,
            input_type: U64,
            lhs: 9,
            rhs: 10,
            rhs_is_scalar: false,
            output_type: BOOL,
            expected: 1,
        },
        BinaryCase {
            name: "minimum",
            op: FheBinaryOpCode::Min,
            input_type: U8,
            lhs: 9,
            rhs: 10,
            rhs_is_scalar: true,
            output_type: U8,
            expected: 9,
        },
        BinaryCase {
            name: "maximum",
            op: FheBinaryOpCode::Max,
            input_type: U8,
            lhs: 9,
            rhs: 10,
            rhs_is_scalar: false,
            output_type: U8,
            expected: 10,
        },
    ];

    for (index, case) in cases.iter().enumerate() {
        let lhs_handle = handle(index as u8 + 1, case.input_type);
        let rhs_handle = handle(index as u8 + 101, case.input_type);
        let mut inputs = inputs([(lhs_handle, case.input_type, case.lhs)]);
        if !case.rhs_is_scalar {
            inputs.insert(
                rhs_handle,
                TypedClearValue::from_u64(case.input_type, case.rhs),
            );
        }
        let rhs = if case.rhs_is_scalar {
            FheEvalOperand::Scalar(scalar(case.rhs))
        } else {
            durable(rhs_handle)
        };
        let args = args(vec![FheEvalStep::Binary {
            op: case.op,
            lhs: durable(lhs_handle),
            rhs,
            output_fhe_type: case.output_type,
            output: local(),
        }]);

        let output =
            evaluate(&args, &inputs).unwrap_or_else(|error| panic!("{}: {error}", case.name));
        assert_eq!(as_u64(output[0]), case.expected, "{}", case.name);
        assert_eq!(output[0].fhe_type, case.output_type, "{}", case.name);
    }
}

#[test]
fn unary_negation_is_modular() {
    let input = handle(1, U8);
    let plan = args(vec![FheEvalStep::Unary {
        op: FheUnaryOpCode::Neg,
        operand: durable(input),
        output_fhe_type: U8,
        output: local(),
    }]);

    let output = evaluate(&plan, &inputs([(input, U8, 1)])).unwrap();
    assert_eq!(as_u64(output[0]), 255);
}

#[test]
fn uint256_operations_preserve_the_high_half() {
    let lhs_handle = handle(1, U256);
    let rhs_handle = handle(2, U256);
    let mut lhs = [0; 32];
    lhs[0] = 0x80;
    lhs[31] = 1;
    let mut rhs = [0; 32];
    rhs[0] = 0x80;
    rhs[31] = 1;
    let inputs = HashMap::from([
        (lhs_handle, TypedClearValue::from_be_bytes(U256, lhs)),
        (rhs_handle, TypedClearValue::from_be_bytes(U256, rhs)),
    ]);
    let plan = args(vec![
        FheEvalStep::Binary {
            op: FheBinaryOpCode::Eq,
            lhs: durable(lhs_handle),
            rhs: durable(rhs_handle),
            output_fhe_type: BOOL,
            output: local(),
        },
        FheEvalStep::Unary {
            op: FheUnaryOpCode::Not,
            operand: durable(lhs_handle),
            output_fhe_type: U256,
            output: local(),
        },
        FheEvalStep::Unary {
            op: FheUnaryOpCode::Cast,
            operand: durable(lhs_handle),
            output_fhe_type: U8,
            output: local(),
        },
    ]);

    let output = evaluate(&plan, &inputs).unwrap();
    assert_eq!(as_u64(output[0]), 1);
    let expected_not = lhs.map(|byte| !byte);
    assert_eq!(output[1].value, expected_not);
    assert_eq!(as_u64(output[2]), 1);
}

#[test]
fn reductions_match_worker_shapes() {
    let a = handle(1, U16);
    let b = handle(2, U16);
    let member = handle(3, U160);
    let mut high_value = [0; 32];
    high_value[12] = 1;
    let inputs = HashMap::from([
        (a, TypedClearValue::from_u64(U16, 200)),
        (b, TypedClearValue::from_u64(U16, 100)),
        (member, TypedClearValue::from_be_bytes(U160, high_value)),
    ]);
    let plan = args(vec![
        FheEvalStep::Sum {
            operands: vec![durable(a), durable(b)],
            fhe_type: U16,
            output: local(),
        },
        FheEvalStep::Sum {
            operands: vec![],
            fhe_type: U16,
            output: local(),
        },
        FheEvalStep::IsIn {
            value: durable(member),
            set: vec![durable(member)],
            fhe_type: U160,
            output: local(),
        },
        FheEvalStep::IsIn {
            value: durable(member),
            set: vec![],
            fhe_type: U160,
            output: local(),
        },
    ]);

    let output = evaluate(&plan, &inputs).unwrap();
    assert_eq!([as_u64(output[0]), as_u64(output[1])], [300, 0]);
    assert_eq!([as_u64(output[2]), as_u64(output[3])], [1, 0]);
}

#[test]
fn mul_div_widens_before_division() {
    let factor = handle(1, U16);
    let plan = args(vec![FheEvalStep::MulDiv {
        factor1: durable(factor),
        factor2: FheEvalOperand::Scalar(scalar(400)),
        divisor: scalar(3),
        output_fhe_type: U16,
        output: local(),
    }]);

    let output = evaluate(&plan, &inputs([(factor, U16, 200)])).unwrap();
    assert_eq!(as_u64(output[0]), 26_666);
}

#[test]
fn random_steps_are_deterministic_typed_mocks() {
    let plan = FheEvalArgs {
        context_id: [9; 32],
        steps: vec![
            FheEvalStep::Rand {
                fhe_type: U256,
                output: local(),
            },
            FheEvalStep::RandBounded {
                upper_bound: scalar(16),
                fhe_type: U8,
                output: local(),
            },
        ],
    };

    let first = evaluate(&plan, &ClearInputs::new()).unwrap();
    let second = evaluate(&plan, &ClearInputs::new()).unwrap();
    assert_eq!(first, second);
    assert_eq!(first[0].fhe_type, U256);
    assert_eq!(first[1].fhe_type, U8);
    assert!(as_u64(first[1]) < 16);
}

#[test]
fn bool_plaintext_and_scalar_use_their_production_encodings() {
    let encrypted_false = handle(1, BOOL);
    let mut high_byte_only = [0; 32];
    high_byte_only[0] = 1;
    let inputs = HashMap::from([(encrypted_false, TypedClearValue::from_u64(BOOL, 0))]);
    let plan = args(vec![
        FheEvalStep::TrivialEncrypt {
            plaintext: high_byte_only,
            fhe_type: BOOL,
            output: local(),
        },
        FheEvalStep::Binary {
            op: FheBinaryOpCode::Eq,
            lhs: durable(encrypted_false),
            rhs: FheEvalOperand::Scalar(high_byte_only),
            output_fhe_type: BOOL,
            output: local(),
        },
    ]);

    let output = evaluate(&plan, &inputs).unwrap();
    // Trivial bool encryption reads the low byte; scalar bool conversion tests any non-zero byte.
    assert_eq!(as_u64(output[0]), 0);
    assert_eq!(as_u64(output[1]), 0);
}

#[test]
fn ternary_selects_the_matching_branch() {
    let if_true = handle(1, U64);
    let if_false = handle(2, U64);
    for (control, expected) in [(0, 20), (1, 10)] {
        let control_handle = handle(3, BOOL);
        let plan = args(vec![FheEvalStep::Ternary {
            op: zama_host::FheTernaryOpCode::IfThenElse,
            control: durable(control_handle),
            if_true: durable(if_true),
            if_false: durable(if_false),
            output_fhe_type: U64,
            output: local(),
        }]);
        let clear_inputs = inputs([
            (control_handle, BOOL, control),
            (if_true, U64, 10),
            (if_false, U64, 20),
        ]);

        assert_eq!(as_u64(evaluate(&plan, &clear_inputs).unwrap()[0]), expected);
    }
}

#[test]
fn malformed_plans_are_rejected() {
    let bool_handle = handle(0, BOOL);
    let u8_handle = handle(1, U8);
    let u16_handle = handle(2, U16);
    let unsupported_handle = handle(3, 1);
    let valid_inputs = inputs([
        (bool_handle, BOOL, 1),
        (u8_handle, U8, 7),
        (u16_handle, U16, 7),
    ]);
    let high_only_scalar = {
        let mut value = [0; 32];
        value[0] = 1;
        value
    };
    let cases = [
        (
            "forward local",
            args(vec![FheEvalStep::Unary {
                op: FheUnaryOpCode::Not,
                operand: local_operand(0),
                output_fhe_type: U8,
                output: local(),
            }]),
            "missing earlier local output",
        ),
        (
            "scalar lhs",
            args(vec![FheEvalStep::Binary {
                op: FheBinaryOpCode::Add,
                lhs: FheEvalOperand::Scalar(scalar(1)),
                rhs: FheEvalOperand::Scalar(scalar(2)),
                output_fhe_type: U8,
                output: local(),
            }]),
            "scalar is not valid",
        ),
        (
            "output type mismatch",
            args(vec![FheEvalStep::Binary {
                op: FheBinaryOpCode::Add,
                lhs: durable(u8_handle),
                rhs: FheEvalOperand::Scalar(scalar(1)),
                output_fhe_type: U16,
                output: local(),
            }]),
            "invalid binary operation",
        ),
        (
            "ternary control is not bool",
            args(vec![FheEvalStep::Ternary {
                op: zama_host::FheTernaryOpCode::IfThenElse,
                control: durable(u8_handle),
                if_true: durable(u8_handle),
                if_false: durable(u8_handle),
                output_fhe_type: U8,
                output: local(),
            }]),
            "invalid ternary operand types",
        ),
        (
            "ternary branch type mismatch",
            args(vec![FheEvalStep::Ternary {
                op: zama_host::FheTernaryOpCode::IfThenElse,
                control: durable(bool_handle),
                if_true: durable(u8_handle),
                if_false: durable(u16_handle),
                output_fhe_type: U8,
                output: local(),
            }]),
            "invalid ternary operand types",
        ),
        (
            "unsupported ternary output type",
            args(vec![FheEvalStep::Ternary {
                op: zama_host::FheTernaryOpCode::IfThenElse,
                control: durable(bool_handle),
                if_true: durable(unsupported_handle),
                if_false: durable(unsupported_handle),
                output_fhe_type: 1,
                output: local(),
            }]),
            "invalid ternary operation",
        ),
        (
            "divisor truncates to zero",
            args(vec![FheEvalStep::Binary {
                op: FheBinaryOpCode::Div,
                lhs: durable(u8_handle),
                rhs: FheEvalOperand::Scalar(high_only_scalar),
                output_fhe_type: U8,
                output: local(),
            }]),
            "invalid binary operation",
        ),
        (
            "invalid random bound",
            args(vec![FheEvalStep::RandBounded {
                upper_bound: scalar(3),
                fhe_type: U8,
                output: local(),
            }]),
            "invalid bounded rand",
        ),
        (
            "mixed sum types",
            args(vec![FheEvalStep::Sum {
                operands: vec![durable(u8_handle), durable(u16_handle)],
                fhe_type: U8,
                output: local(),
            }]),
            "invalid sum",
        ),
        (
            "sum operand cap",
            args(vec![FheEvalStep::Sum {
                operands: vec![durable(u8_handle); 101],
                fhe_type: U8,
                output: local(),
            }]),
            "invalid sum",
        ),
        (
            "mixed is-in types",
            args(vec![FheEvalStep::IsIn {
                value: durable(u8_handle),
                set: vec![durable(u16_handle)],
                fhe_type: U8,
                output: local(),
            }]),
            "invalid is-in",
        ),
        (
            "mul-div divisor truncates to zero",
            args(vec![FheEvalStep::MulDiv {
                factor1: durable(u8_handle),
                factor2: FheEvalOperand::Scalar(scalar(2)),
                divisor: high_only_scalar,
                output_fhe_type: U8,
                output: local(),
            }]),
            "invalid mul-div",
        ),
        (
            "unsupported random address type",
            args(vec![FheEvalStep::Rand {
                fhe_type: U160,
                output: local(),
            }]),
            "invalid rand",
        ),
        (
            "same-type cast",
            args(vec![FheEvalStep::Unary {
                op: FheUnaryOpCode::Cast,
                operand: durable(u8_handle),
                output_fhe_type: U8,
                output: local(),
            }]),
            "invalid unary operation",
        ),
    ];

    for (name, plan, expected) in cases {
        let error = evaluate(&plan, &valid_inputs).expect_err(name);
        assert!(error.contains(expected), "{name}: {error}");
    }

    let missing_input = args(vec![FheEvalStep::Unary {
        op: FheUnaryOpCode::Not,
        operand: durable(u8_handle),
        output_fhe_type: U8,
        output: local(),
    }]);
    let error = evaluate(&missing_input, &ClearInputs::new()).expect_err("missing input");
    assert!(error.contains("missing cleartext input"), "{error}");
}

#[test]
fn seeded_value_type_must_match_its_handle() {
    let input = handle(1, U8);
    let plan = args(vec![FheEvalStep::Unary {
        op: FheUnaryOpCode::Not,
        operand: durable(input),
        output_fhe_type: U8,
        output: local(),
    }]);
    let inputs = HashMap::from([(input, TypedClearValue::from_u64(U16, 1))]);

    let error = evaluate(&plan, &inputs).unwrap_err();
    assert!(error.contains("does not match"), "{error}");
}

fn args(steps: Vec<FheEvalStep>) -> FheEvalArgs {
    FheEvalArgs {
        context_id: [7; 32],
        steps,
    }
}

fn inputs<const N: usize>(values: [(Handle, u8, u64); N]) -> ClearInputs {
    values
        .into_iter()
        .map(|(handle, fhe_type, value)| (handle, TypedClearValue::from_u64(fhe_type, value)))
        .collect()
}

fn handle(id: u8, fhe_type: u8) -> Handle {
    let mut handle = [0; 32];
    handle[0] = id;
    handle[30] = fhe_type;
    handle
}

fn scalar(value: u64) -> [u8; 32] {
    TypedClearValue::from_u64(U64, value).value
}

fn durable(handle: Handle) -> FheEvalOperand {
    FheEvalOperand::AllowedDurable {
        handle,
        encrypted_value_index: 0,
    }
}

fn local_operand(producer_index: u16) -> FheEvalOperand {
    FheEvalOperand::AllowedLocal { producer_index }
}

fn local() -> FheEvalOutput {
    FheEvalOutput::AllowedLocal
}

fn as_u64(value: TypedClearValue) -> u64 {
    u64::from_be_bytes(value.value[24..].try_into().unwrap())
}
