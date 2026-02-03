use bigdecimal::num_bigint::BigInt;
use fhevm_engine_common::tfhe_ops::{
    does_fhe_operation_support_both_encrypted_operands, does_fhe_operation_support_scalar,
};
use fhevm_engine_common::types::{is_ebytes_type, FheOperationType, SupportedFheOperations};
use std::ops::Not;
use strum::IntoEnumIterator;

pub struct BinaryOperatorTestCase {
    pub bits: i32,
    pub operator: i32,
    pub input_types: i32,
    pub expected_output_type: i32,
    pub lhs: BigInt,
    pub rhs: BigInt,
    pub expected_output: BigInt,
    pub is_scalar: bool,
}

pub struct UnaryOperatorTestCase {
    pub bits: i32,
    pub inp: BigInt,
    pub operand: i32,
    pub operand_types: i32,
    pub expected_output: BigInt,
}

fn supported_bits() -> &'static [i32] {
    &[1, 4, 8, 16, 32, 64, 128, 160, 256, 512, 1024, 2048]
}

fn supported_bits_to_bit_type_in_db(inp: i32) -> i32 {
    match inp {
        1 => 0, // 1 bit - boolean
        4 => 1,
        8 => 2,
        16 => 3,
        32 => 4,
        64 => 5,
        128 => 6,
        160 => 7,
        256 => 8,
        512 => 9,
        1024 => 10,
        2048 => 11,
        other => panic!("unknown supported bits: {other}"),
    }
}

pub fn generate_binary_test_cases() -> Vec<BinaryOperatorTestCase> {
    let mut cases = Vec::new();
    let bit_shift_ops = [
        SupportedFheOperations::FheShl,
        SupportedFheOperations::FheShr,
        SupportedFheOperations::FheRotl,
        SupportedFheOperations::FheRotr,
    ];
    let fhe_bool_type = 0;
    let mut push_case = |bits: i32, is_scalar: bool, shift_by: i32, op: SupportedFheOperations| {
        let mut lhs = BigInt::from(6);
        let mut rhs = BigInt::from(2);
        lhs <<= shift_by;
        // don't shift by much for bit shift opts not to make result 0
        if bit_shift_ops.contains(&op) {
            rhs = BigInt::from(1);
        } else {
            rhs <<= shift_by;
        }
        let expected_output = compute_expected_binary_output(&lhs, &rhs, op);
        let operand = op as i32;
        let expected_output_type = if op.is_comparison() {
            fhe_bool_type
        } else {
            supported_bits_to_bit_type_in_db(bits)
        };
        cases.push(BinaryOperatorTestCase {
            bits,
            operator: operand,
            expected_output_type,
            input_types: supported_bits_to_bit_type_in_db(bits),
            lhs,
            rhs,
            expected_output,
            is_scalar,
        });
    };

    let mut bool_cases = Vec::new();

    for bits in supported_bits() {
        let bits = *bits;
        let mut shift_by = if bits > 4 { bits - 8 } else { 0 };
        for op in SupportedFheOperations::iter() {
            if op.op_type() != FheOperationType::Binary {
                continue;
            }

            if bits > 256 && !op.supports_ebytes_inputs() {
                continue;
            }

            if bits == 1 {
                if !op.supports_bool_inputs() {
                    continue;
                }

                let lhs = BigInt::from(0);
                let rhs = BigInt::from(1);
                let expected_output = compute_expected_binary_output(&lhs, &rhs, op);
                if does_fhe_operation_support_both_encrypted_operands(&op) {
                    bool_cases.push(BinaryOperatorTestCase {
                        bits,
                        operator: op as i32,
                        expected_output_type: fhe_bool_type,
                        input_types: supported_bits_to_bit_type_in_db(bits),
                        lhs: lhs.clone(),
                        rhs: rhs.clone(),
                        expected_output: expected_output.clone(),
                        is_scalar: false,
                    });
                }

                if does_fhe_operation_support_scalar(&op) {
                    bool_cases.push(BinaryOperatorTestCase {
                        bits,
                        operator: op as i32,
                        expected_output_type: fhe_bool_type,
                        input_types: supported_bits_to_bit_type_in_db(bits),
                        lhs,
                        rhs,
                        expected_output,
                        is_scalar: true,
                    });
                }
            } else {
                if op == SupportedFheOperations::FheMul {
                    // don't go out of bit bounds when multiplying two numbers, so we shift by less
                    shift_by /= 2;
                }
                if op.op_type() == FheOperationType::Binary {
                    if does_fhe_operation_support_both_encrypted_operands(&op) {
                        push_case(bits, false, shift_by, op);
                    }

                    if does_fhe_operation_support_scalar(&op) {
                        push_case(bits, true, shift_by, op);
                    }
                }
            }
        }
    }

    cases.extend(bool_cases);

    cases
}

pub fn generate_unary_test_cases() -> Vec<UnaryOperatorTestCase> {
    let mut cases = Vec::new();

    for bits in supported_bits() {
        let bits = *bits;
        let shift_by = bits - 3;
        let max_bits_value = (BigInt::from(1) << bits) - 1;
        for op in SupportedFheOperations::iter() {
            if bits == 1 && !op.supports_bool_inputs() {
                continue;
            }
            if is_ebytes_type(supported_bits_to_bit_type_in_db(bits) as i16)
                && !op.supports_ebytes_inputs()
            {
                continue;
            }

            if op.op_type() == FheOperationType::Unary {
                let inp = if bits == 1 {
                    BigInt::from(1)
                } else {
                    let mut res = BigInt::from(3);
                    res <<= shift_by;
                    res
                };
                let expected_output = compute_expected_unary_output(&inp, op) & &max_bits_value;
                let operand = op as i32;
                cases.push(UnaryOperatorTestCase {
                    bits,
                    operand,
                    operand_types: supported_bits_to_bit_type_in_db(bits),
                    inp,
                    expected_output,
                });
            }
        }
    }

    cases
}

fn compute_expected_unary_output(inp: &BigInt, op: SupportedFheOperations) -> BigInt {
    match op {
        SupportedFheOperations::FheNot => {
            let (_, mut bytes) = inp.to_bytes_be();
            for byte in bytes.iter_mut() {
                *byte = byte.not();
            }
            BigInt::from_bytes_be(bigdecimal::num_bigint::Sign::Plus, &bytes)
        }
        SupportedFheOperations::FheNeg => {
            let (_, mut bytes) = inp.to_bytes_be();
            for byte in bytes.iter_mut() {
                *byte = byte.not();
            }
            let num = BigInt::from_bytes_be(bigdecimal::num_bigint::Sign::Plus, &bytes);
            num + 1
        }
        other => panic!("unsupported unary operation: {:?}", other),
    }
}

fn compute_expected_binary_output(
    lhs: &BigInt,
    rhs: &BigInt,
    op: SupportedFheOperations,
) -> BigInt {
    match op {
        SupportedFheOperations::FheEq => BigInt::from(lhs.eq(rhs)),
        SupportedFheOperations::FheNe => BigInt::from(lhs.ne(rhs)),
        SupportedFheOperations::FheGe => BigInt::from(lhs.ge(rhs)),
        SupportedFheOperations::FheGt => BigInt::from(lhs.gt(rhs)),
        SupportedFheOperations::FheLe => BigInt::from(lhs.le(rhs)),
        SupportedFheOperations::FheLt => BigInt::from(lhs.lt(rhs)),
        SupportedFheOperations::FheMin => lhs.min(rhs).clone(),
        SupportedFheOperations::FheMax => lhs.max(rhs).clone(),
        SupportedFheOperations::FheAdd => lhs + rhs,
        SupportedFheOperations::FheSub => lhs - rhs,
        SupportedFheOperations::FheMul => lhs * rhs,
        SupportedFheOperations::FheDiv => lhs / rhs,
        SupportedFheOperations::FheRem => lhs % rhs,
        SupportedFheOperations::FheBitAnd => lhs & rhs,
        SupportedFheOperations::FheBitOr => lhs | rhs,
        SupportedFheOperations::FheBitXor => lhs ^ rhs,
        SupportedFheOperations::FheShl => lhs << (TryInto::<u64>::try_into(rhs).unwrap()),
        SupportedFheOperations::FheShr => lhs >> (TryInto::<u64>::try_into(rhs).unwrap()),
        // we don't shift by as much as to overlap the register
        // in tests, so should be same as bit shifts
        SupportedFheOperations::FheRotl => lhs << (TryInto::<u64>::try_into(rhs).unwrap()),
        SupportedFheOperations::FheRotr => lhs >> (TryInto::<u64>::try_into(rhs).unwrap()),
        other => panic!("unsupported binary operation: {:?}", other),
    }
}
