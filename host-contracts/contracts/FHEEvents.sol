// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FheType} from "./shared/FheType.sol";

/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract FHEEvents {
    event FheAdd(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result);
    event FheSub(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result);
    event FheMul(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result);
    event FheDiv(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result);
    event FheRem(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result);
    event FheBitAnd(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result);
    event FheBitOr(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result);
    event FheBitXor(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result);
    event FheShl(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result);
    event FheShr(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result);
    event FheRotl(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result);
    event FheRotr(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result);
    event FheEq(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result);
    event FheNe(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result);
    event FheGe(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result);
    event FheGt(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result);
    event FheLe(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result);
    event FheLt(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result);
    event FheMin(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result);
    event FheMax(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result);
    event FheNeg(address indexed caller, bytes32 ct, bytes32 result);
    event FheNot(address indexed caller, bytes32 ct, bytes32 result);
    event VerifyInput(
        address indexed caller,
        bytes32 inputHandle,
        address userAddress,
        bytes inputProof,
        FheType inputType,
        bytes32 result
    );
    event Cast(address indexed caller, bytes32 ct, FheType toType, bytes32 result);
    event TrivialEncrypt(address indexed caller, uint256 pt, FheType toType, bytes32 result);
    event FheIfThenElse(address indexed caller, bytes32 control, bytes32 ifTrue, bytes32 ifFalse, bytes32 result);
    event FheRand(address indexed caller, FheType randType, bytes16 seed, bytes32 result);
    event FheRandBounded(address indexed caller, uint256 upperBound, FheType randType, bytes16 seed, bytes32 result);
    event FheSum(address indexed caller, bytes32[] values, bytes32 result);
    event FheIsIn(address indexed caller, bytes32 value, bytes32[] values, bytes32 result);
    event FheMulDiv(
        address indexed caller,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 divisor,
        bytes1 scalarByte,
        bytes32 result
    );
    /// @dev Sample multi-output op (proof-of-concept). Emits two result handles:
    ///      `resultValue` (index 0, same type as input) and `resultFound` (index 1, ebool).
    event FheSampleMultiOutput(address indexed caller, bytes32 ct, bytes32 resultValue, bytes32 resultFound);
    /// @dev High-arity sample multi-output op. Emits 100 result handles, ordered by
    ///      tfhe-rs result tuple index. Every entry shares the input type.
    event FheSampleMultiOutput100(address indexed caller, bytes32 ct, bytes32[] results);
}
