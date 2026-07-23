// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FheType} from "../contracts/shared/FheType.sol";
import {FHEVMExecutor} from "../contracts/FHEVMExecutor.sol";

/**
 * @title ICleartextArithmetic
 * @notice The cleartext computation + persistence layer. `CleartextFHEVMExecutor` calls these
 *         `record*` methods after each symbolic op; this contract reads the operand cleartexts from
 *         `CleartextDB`, computes the result (mirroring FHE bit-width semantics), and writes it back.
 * @dev The executor never touches `CleartextDB` directly — this contract is the DB's sole writer,
 *      which keeps the arithmetic + storage bytecode out of the executor (EIP-170 headroom).
 */
interface ICleartextArithmetic {
    /// @notice Reads the cleartext value stored for `handle` from `CleartextDB` (compat accessor
    ///         mirroring the executor's former public `plaintexts` mapping).
    function plaintexts(bytes32 handle) external view returns (uint256);

    /// @notice Records `fheCast(ct)` into `result`.
    function recordCast(bytes32 result, bytes32 ct, FheType toType) external;

    /// @notice Records the normalized `pt` into `result`.
    function recordTrivialEncrypt(bytes32 result, uint256 pt, FheType toType) external;

    /// @notice Records the cleartext extracted from `inputProof` (if present) into `result`.
    function recordVerifyInput(bytes32 result, bytes32 inputHandle, bytes memory inputProof, FheType inputType)
        external;

    /// @notice Records a pseudo-random value (clamped to `randType`'s width) into `result`.
    function recordRand(bytes32 result, FheType randType, bytes16 seed) external;

    /// @notice Records a bounded pseudo-random value into `result`.
    function recordRandBounded(bytes32 result, uint256 upperBound, bytes16 seed) external;

    /// @notice Computes a binary op over the operands' cleartexts and records it into `result`.
    function recordBinaryOp(
        FHEVMExecutor.Operators op,
        bytes32 result,
        bytes32 lhs,
        bytes32 rhs,
        bytes1 scalarByte,
        FheType fheType
    ) external;

    /// @notice Computes a unary op (`fheNeg` / `fheNot`) over `ct`'s cleartext and records it.
    function recordUnaryOp(FHEVMExecutor.Operators op, bytes32 result, bytes32 ct, FheType fheType) external;

    /// @notice Computes `fheMulDiv` — `(factor1 * factor2) / divisor` — over the operands' cleartexts
    ///         and records it into `result`. Protocol v0.14 and later.
    /// @dev `scalarByte` is `fheMulDiv`'s own bitmask, not the binary-op one: `0x01` means `factor2`
    ///      is ENCRYPTED, `0x03` means it is a scalar. `divisor` is always a plaintext scalar.
    function recordMulDiv(
        bytes32 result,
        bytes32 factor1,
        bytes32 factor2,
        bytes32 divisor,
        bytes1 scalarByte,
        FheType fheType
    ) external;

    /// @notice Computes `fheIfThenElse(lhs, middle, rhs)` over their cleartexts and records it.
    function recordIfThenElse(bytes32 result, bytes32 lhs, bytes32 middle, bytes32 rhs) external;
}
