//! Explicit test-owned contract catalog for the shipped Solana `fhe_eval` surface.
//!
//! Accepted cases are explicit rather than generated from production predicates. The macros
//! expand to ordinary named tests and can be reused by a later Mollusk target through the same
//! runner signatures.

macro_rules! binary_contract_tests {
    () => {
        mod binary {
            macro_rules! both_shapes {
                ($name:ident, $op:expr, $ty:expr, $lhs:expr, $rhs:expr, $out:expr, $expected:expr) => {
                    mod $name {
                        #[test]
                        fn encrypted() {
                            crate::run_binary($op, $ty, false, $lhs, $rhs, $out, $expected);
                        }
                        #[test]
                        fn scalar() {
                            crate::run_binary($op, $ty, true, $lhs, $rhs, $out, $expected);
                        }
                    }
                };
            }
            macro_rules! scalar_only {
                ($name:ident, $op:expr, $ty:expr, $lhs:expr, $rhs:expr, $expected:expr) => {
                    mod $name {
                        #[test]
                        fn scalar() {
                            crate::run_binary($op, $ty, true, $lhs, $rhs, $ty, $expected);
                        }
                    }
                };
            }
            mod add {
                both_shapes!(u8, zama_host::FheBinaryOpCode::Add, 2, 250, 10, 2, 4);
                both_shapes!(u16, zama_host::FheBinaryOpCode::Add, 3, 60_000, 10_000, 3, 4_464);
                both_shapes!(u32, zama_host::FheBinaryOpCode::Add, 4, 4_000_000_000, 1_000_000_000, 4, 705_032_704);
                both_shapes!(u64, zama_host::FheBinaryOpCode::Add, 5, u64::MAX - 5, 10, 5, 4);
                both_shapes!(u128, zama_host::FheBinaryOpCode::Add, 6, 100, 23, 6, 123);
            }
            mod sub {
                both_shapes!(u8, zama_host::FheBinaryOpCode::Sub, 2, 2, 3, 2, 255);
                both_shapes!(u16, zama_host::FheBinaryOpCode::Sub, 3, 100, 23, 3, 77);
                both_shapes!(u32, zama_host::FheBinaryOpCode::Sub, 4, 100, 23, 4, 77);
                both_shapes!(u64, zama_host::FheBinaryOpCode::Sub, 5, 100, 23, 5, 77);
                both_shapes!(u128, zama_host::FheBinaryOpCode::Sub, 6, 100, 23, 6, 77);
            }
            mod mul {
                both_shapes!(u8, zama_host::FheBinaryOpCode::Mul, 2, 20, 20, 2, 144);
                both_shapes!(u16, zama_host::FheBinaryOpCode::Mul, 3, 300, 300, 3, 24_464);
                both_shapes!(u32, zama_host::FheBinaryOpCode::Mul, 4, 20, 3, 4, 60);
                both_shapes!(u64, zama_host::FheBinaryOpCode::Mul, 5, 20, 3, 5, 60);
                both_shapes!(u128, zama_host::FheBinaryOpCode::Mul, 6, 20, 3, 6, 60);
            }
            mod div {
                scalar_only!(u8, zama_host::FheBinaryOpCode::Div, 2, 100, 3, 33);
                scalar_only!(u16, zama_host::FheBinaryOpCode::Div, 3, 100, 3, 33);
                scalar_only!(u32, zama_host::FheBinaryOpCode::Div, 4, 100, 3, 33);
                scalar_only!(u64, zama_host::FheBinaryOpCode::Div, 5, 100, 3, 33);
                scalar_only!(u128, zama_host::FheBinaryOpCode::Div, 6, 100, 3, 33);
            }
            mod rem {
                scalar_only!(u8, zama_host::FheBinaryOpCode::Rem, 2, 100, 9, 1);
                scalar_only!(u16, zama_host::FheBinaryOpCode::Rem, 3, 100, 9, 1);
                scalar_only!(u32, zama_host::FheBinaryOpCode::Rem, 4, 100, 9, 1);
                scalar_only!(u64, zama_host::FheBinaryOpCode::Rem, 5, 100, 9, 1);
                scalar_only!(u128, zama_host::FheBinaryOpCode::Rem, 6, 100, 9, 1);
            }
            macro_rules! bitwise_types {
                ($op:expr, $bool_expected:expr, $expected:expr) => {
                    both_shapes!(bool, $op, 0, 1, 0, 0, $bool_expected);
                    both_shapes!(u8, $op, 2, 10, 12, 2, $expected);
                    both_shapes!(u16, $op, 3, 10, 12, 3, $expected);
                    both_shapes!(u32, $op, 4, 10, 12, 4, $expected);
                    both_shapes!(u64, $op, 5, 10, 12, 5, $expected);
                    both_shapes!(u128, $op, 6, 10, 12, 6, $expected);
                    both_shapes!(u256, $op, 8, 10, 12, 8, $expected);
                };
            }
            mod and {
                bitwise_types!(zama_host::FheBinaryOpCode::And, 0, 8);
            }
            mod or {
                bitwise_types!(zama_host::FheBinaryOpCode::Or, 1, 14);
            }
            mod xor {
                bitwise_types!(zama_host::FheBinaryOpCode::Xor, 1, 6);
            }
            macro_rules! shift_types {
                ($op:expr, $lhs:expr, $rhs:expr, $expected:expr) => {
                    both_shapes!(u8, $op, 2, $lhs, $rhs, 2, $expected);
                    both_shapes!(u16, $op, 3, $lhs, $rhs, 3, $expected);
                    both_shapes!(u32, $op, 4, $lhs, $rhs, 4, $expected);
                    both_shapes!(u64, $op, 5, $lhs, $rhs, 5, $expected);
                    both_shapes!(u128, $op, 6, $lhs, $rhs, 6, $expected);
                    both_shapes!(u256, $op, 8, $lhs, $rhs, 8, $expected);
                };
            }
            mod shl {
                shift_types!(zama_host::FheBinaryOpCode::Shl, 1, 1, 2);
            }
            mod shr {
                shift_types!(zama_host::FheBinaryOpCode::Shr, 8, 1, 4);
            }
            mod rotl {
                shift_types!(zama_host::FheBinaryOpCode::Rotl, 1, 1, 2);
            }
            mod rotr {
                shift_types!(zama_host::FheBinaryOpCode::Rotr, 2, 1, 1);
            }
            macro_rules! equality_types {
                ($op:expr, $bool_lhs:expr, $bool_rhs:expr, $bool_expected:expr, $lhs:expr, $rhs:expr, $expected:expr) => {
                    both_shapes!(bool, $op, 0, $bool_lhs, $bool_rhs, 0, $bool_expected);
                    both_shapes!(u8, $op, 2, $lhs, $rhs, 0, $expected);
                    both_shapes!(u16, $op, 3, $lhs, $rhs, 0, $expected);
                    both_shapes!(u32, $op, 4, $lhs, $rhs, 0, $expected);
                    both_shapes!(u64, $op, 5, $lhs, $rhs, 0, $expected);
                    both_shapes!(u128, $op, 6, $lhs, $rhs, 0, $expected);
                    both_shapes!(u160, $op, 7, $lhs, $rhs, 0, $expected);
                    both_shapes!(u256, $op, 8, $lhs, $rhs, 0, $expected);
                };
            }
            mod eq {
                equality_types!(zama_host::FheBinaryOpCode::Eq, 1, 1, 1, 9, 9, 1);
            }
            mod ne {
                equality_types!(zama_host::FheBinaryOpCode::Ne, 0, 1, 1, 9, 10, 1);
            }
            macro_rules! ordered_types {
                ($op:expr, $lhs:expr, $rhs:expr, $expected:expr) => {
                    both_shapes!(u8, $op, 2, $lhs, $rhs, 0, $expected);
                    both_shapes!(u16, $op, 3, $lhs, $rhs, 0, $expected);
                    both_shapes!(u32, $op, 4, $lhs, $rhs, 0, $expected);
                    both_shapes!(u64, $op, 5, $lhs, $rhs, 0, $expected);
                    both_shapes!(u128, $op, 6, $lhs, $rhs, 0, $expected);
                };
            }
            mod ge {
                ordered_types!(zama_host::FheBinaryOpCode::Ge, 9, 9, 1);
            }
            mod gt {
                ordered_types!(zama_host::FheBinaryOpCode::Gt, 10, 9, 1);
            }
            mod le {
                ordered_types!(zama_host::FheBinaryOpCode::Le, 9, 9, 1);
            }
            mod lt {
                ordered_types!(zama_host::FheBinaryOpCode::Lt, 9, 10, 1);
            }
            macro_rules! arithmetic_types {
                ($op:expr, $lhs:expr, $rhs:expr, $expected:expr) => {
                    both_shapes!(u8, $op, 2, $lhs, $rhs, 2, $expected);
                    both_shapes!(u16, $op, 3, $lhs, $rhs, 3, $expected);
                    both_shapes!(u32, $op, 4, $lhs, $rhs, 4, $expected);
                    both_shapes!(u64, $op, 5, $lhs, $rhs, 5, $expected);
                    both_shapes!(u128, $op, 6, $lhs, $rhs, 6, $expected);
                };
            }
            mod min {
                arithmetic_types!(zama_host::FheBinaryOpCode::Min, 9, 10, 9);
            }
            mod max {
                arithmetic_types!(zama_host::FheBinaryOpCode::Max, 9, 10, 10);
            }
        }
    };
}

pub(crate) use binary_contract_tests;

macro_rules! unary_contract_tests {
    () => {
        mod unary {
            macro_rules! unary_case {
                ($name:ident, $op:expr, $input_ty:expr, $input:expr, $output_ty:expr, $expected:expr) => {
                    #[test]
                    fn $name() {
                        crate::run_unary($op, $input_ty, $input, $output_ty, $expected);
                    }
                };
            }
            mod neg {
                unary_case!(u8, zama_host::FheUnaryOpCode::Neg, 2, crate::be(1), 2, crate::be(255));
                unary_case!(u16, zama_host::FheUnaryOpCode::Neg, 3, crate::be(1), 3, crate::be(65_535));
                unary_case!(u32, zama_host::FheUnaryOpCode::Neg, 4, crate::be(1), 4, crate::be(4_294_967_295));
                unary_case!(u64, zama_host::FheUnaryOpCode::Neg, 5, crate::be(1), 5, crate::be(u64::MAX));
                unary_case!(u128, zama_host::FheUnaryOpCode::Neg, 6, crate::be(1), 6, [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                ]);
                unary_case!(u256, zama_host::FheUnaryOpCode::Neg, 8, crate::be(1), 8, [0xff; 32]);
            }
            mod not {
                unary_case!(bool, zama_host::FheUnaryOpCode::Not, 0, [0xff; 32], 0, [0; 32]);
                unary_case!(u8, zama_host::FheUnaryOpCode::Not, 2, [0xff; 32], 2, [0; 32]);
                unary_case!(u16, zama_host::FheUnaryOpCode::Not, 3, [0xff; 32], 3, [0; 32]);
                unary_case!(u32, zama_host::FheUnaryOpCode::Not, 4, [0xff; 32], 4, [0; 32]);
                unary_case!(u64, zama_host::FheUnaryOpCode::Not, 5, [0xff; 32], 5, [0; 32]);
                unary_case!(u128, zama_host::FheUnaryOpCode::Not, 6, [0xff; 32], 6, [0; 32]);
                unary_case!(u256, zama_host::FheUnaryOpCode::Not, 8, [0xff; 32], 8, [0; 32]);
            }
            mod cast {
                macro_rules! cast_case {
                    ($name:ident, $from:expr, $to:expr) => {
                        unary_case!($name, zama_host::FheUnaryOpCode::Cast, $from, crate::be(1), $to, crate::be(1));
                    };
                }

                cast_case!(bool_to_u8, 0, 2);
                cast_case!(bool_to_u16, 0, 3);
                cast_case!(bool_to_u32, 0, 4);
                cast_case!(bool_to_u64, 0, 5);
                cast_case!(bool_to_u128, 0, 6);
                cast_case!(bool_to_u256, 0, 8);
                cast_case!(u8_to_u16, 2, 3);
                cast_case!(u8_to_u32, 2, 4);
                cast_case!(u8_to_u64, 2, 5);
                cast_case!(u8_to_u128, 2, 6);
                cast_case!(u8_to_u256, 2, 8);
                cast_case!(u16_to_u8, 3, 2);
                cast_case!(u16_to_u32, 3, 4);
                cast_case!(u16_to_u64, 3, 5);
                cast_case!(u16_to_u128, 3, 6);
                cast_case!(u16_to_u256, 3, 8);
                cast_case!(u32_to_u8, 4, 2);
                cast_case!(u32_to_u16, 4, 3);
                cast_case!(u32_to_u64, 4, 5);
                cast_case!(u32_to_u128, 4, 6);
                cast_case!(u32_to_u256, 4, 8);
                cast_case!(u64_to_u8, 5, 2);
                cast_case!(u64_to_u16, 5, 3);
                cast_case!(u64_to_u32, 5, 4);
                cast_case!(u64_to_u128, 5, 6);
                cast_case!(u64_to_u256, 5, 8);
                cast_case!(u128_to_u8, 6, 2);
                cast_case!(u128_to_u16, 6, 3);
                cast_case!(u128_to_u32, 6, 4);
                cast_case!(u128_to_u64, 6, 5);
                cast_case!(u128_to_u256, 6, 8);
                cast_case!(u256_to_u8, 8, 2);
                cast_case!(u256_to_u16, 8, 3);
                cast_case!(u256_to_u32, 8, 4);
                cast_case!(u256_to_u64, 8, 5);
                cast_case!(u256_to_u128, 8, 6);
            }
        }
    };
}

pub(crate) use unary_contract_tests;

macro_rules! composite_contract_tests {
    () => {
        mod ternary {
            macro_rules! case {
                ($name:ident, $ty:expr, $if_true:expr, $if_false:expr, $expected:expr) => {
                    #[test]
                    fn $name() {
                        crate::run_ternary($ty, $if_true, $if_false, $expected);
                    }
                };
            }
            case!(bool, 0, 1, 0, 1);
            case!(u8, 2, 11, 22, 11);
            case!(u16, 3, 11, 22, 11);
            case!(u32, 4, 11, 22, 11);
            case!(u64, 5, 11, 22, 11);
            case!(u128, 6, 11, 22, 11);
            case!(u160, 7, 11, 22, 11);
            case!(u256, 8, 11, 22, 11);
        }
        mod trivial_encrypt {
            macro_rules! case {
                ($name:ident, $ty:expr, $expected:expr) => {
                    #[test]
                    fn $name() {
                        crate::run_trivial($ty, 37, $expected);
                    }
                };
            }
            case!(bool, 0, 1);
            case!(u8, 2, 37);
            case!(u16, 3, 37);
            case!(u32, 4, 37);
            case!(u64, 5, 37);
            case!(u128, 6, 37);
            case!(u160, 7, 37);
            case!(u256, 8, 37);
        }
        mod sum {
            macro_rules! case {
                ($name:ident, $ty:expr) => {
                    #[test]
                    fn $name() {
                        crate::run_sum($ty, 17, 29, 46);
                    }
                };
            }
            case!(u8, 2);
            case!(u16, 3);
            case!(u32, 4);
            case!(u64, 5);
            case!(u128, 6);
            #[test]
            fn empty_u64() {
                crate::run_empty_sum(5);
            }
        }
        mod is_in {
            macro_rules! cases {
                ($name:ident, $ty:expr) => {
                    mod $name {
                        #[test]
                        fn present() {
                            crate::run_is_in($ty, 29, true, 1);
                        }
                        #[test]
                        fn absent() {
                            crate::run_is_in($ty, 31, true, 0);
                        }
                    }
                };
            }
            cases!(u8, 2);
            cases!(u16, 3);
            cases!(u32, 4);
            cases!(u64, 5);
            cases!(u128, 6);
            cases!(u160, 7);
            cases!(u256, 8);
            #[test]
            fn empty_u160() {
                crate::run_is_in(7, 29, false, 0);
            }
        }
        mod mul_div {
            macro_rules! cases {
                ($name:ident, $ty:expr, $expected:expr) => {
                    mod $name {
                        #[test]
                        fn encrypted() {
                            crate::run_mul_div($ty, false, 200, 400, 3, $expected);
                        }
                        #[test]
                        fn scalar() {
                            crate::run_mul_div($ty, true, 200, 400, 3, $expected);
                        }
                    }
                };
            }
            cases!(u8, 2, 128);
            cases!(u16, 3, 26_666);
            cases!(u32, 4, 26_666);
            cases!(u64, 5, 26_666);
        }
        mod rand {
            macro_rules! case {
                ($name:ident, $ty:expr) => {
                    #[test]
                    fn $name() {
                        crate::run_rand($ty);
                    }
                };
            }
            case!(bool, 0);
            case!(u8, 2);
            case!(u16, 3);
            case!(u32, 4);
            case!(u64, 5);
            case!(u128, 6);
            case!(u256, 8);
        }
        mod rand_bounded {
            macro_rules! case {
                ($name:ident, $ty:expr) => {
                    #[test]
                    fn $name() {
                        crate::run_rand_bounded($ty, 16);
                    }
                };
            }
            case!(u8, 2);
            case!(u16, 3);
            case!(u32, 4);
            case!(u64, 5);
            case!(u128, 6);
            case!(u256, 8);
        }
    };
}

pub(crate) use composite_contract_tests;
