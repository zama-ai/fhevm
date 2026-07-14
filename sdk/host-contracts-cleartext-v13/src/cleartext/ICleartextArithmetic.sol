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

    /// @notice Computes a ternary op over the operands' cleartexts and records it into `result`.
    ///         The only ternary op is `fheIfThenElse(lhs, middle, rhs)`.
    function recordTernaryOp(FHEVMExecutor.Operators op, bytes32 result, bytes32 lhs, bytes32 middle, bytes32 rhs)
        external;

    /// @notice Computes an nary op over the operands' cleartexts and records it into `result`.
    ///         For `fheSum`, `value` is unused and `values` are the summands. For `fheIsIn`, `value` is
    ///         the needle and `values` are the set. v13 operators; cleartext computation not yet
    ///         implemented (reverts) — their presence is what forces a v12→v13 `CleartextArithmetic`
    ///         upgrade, since the v12 arithmetic has no such selector.
    function recordNaryOp(
        FHEVMExecutor.Operators op,
        bytes32 result,
        bytes32 value,
        bytes32[] calldata values,
        FheType fheType
    ) external;
}
