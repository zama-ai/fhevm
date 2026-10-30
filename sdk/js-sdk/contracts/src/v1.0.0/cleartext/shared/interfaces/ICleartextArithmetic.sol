// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FheType} from "../LibFheType.sol";
import {Operators} from "../FhevmOperatorsEnum.sol";
import {IPlaintexts} from "./IPlaintexts.sol";

/**
 * @title ICleartextArithmetic
 * @notice The cleartext computation + persistence layer. `CleartextFHEVMExecutor` calls these
 *         `record*` methods after each symbolic op; this contract reads the operand cleartexts from
 *         `CleartextDB`, computes the result (mirroring FHE bit-width semantics), and writes it back.
 * @dev The executor never touches `CleartextDB` directly — this contract is the DB's sole writer,
 *      which keeps the arithmetic + storage bytecode out of the executor (EIP-170 headroom).
 */
interface ICleartextArithmetic is IPlaintexts {
    /// @notice Records `fheCast(ct)` into `result`.
    function recordCast(bytes32 result, bytes32 ct, FheType toType) external;

    /// @notice Records the normalized `pt` into `result`.
    function recordTrivialEncrypt(bytes32 result, uint256 pt, FheType toType) external;

    /// @notice Records the cleartext extracted from `inputProof` (if present) into `result`.
    function recordVerifyInput(bytes32 result, bytes32 inputHandle, bytes memory inputProof, FheType inputType) external;

    /// @notice Records a pseudo-random value (clamped to `randType`'s width) into `result`.
    function recordRand(bytes32 result, FheType randType, bytes16 seed) external;

    /// @notice Records a bounded pseudo-random value into `result`.
    function recordRandBounded(bytes32 result, uint256 upperBound, bytes16 seed) external;

    /// @notice Computes a binary op over the operands' cleartexts and records it into `result`.
    function recordBinaryOp(Operators op, bytes32 result, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, FheType fheType)
        external;

    /// @notice Computes `(factor1 * factor2) / divisor` with a 256-bit intermediate and records it into
    ///         `result`. Since v14: `divisor` is always plaintext; `factor2` is plaintext iff bit 1 of
    ///         `scalarByte` is set (`0x03`). A zero divisor never arrives here: the executor reverts first.
    function recordMulDiv(
        bytes32 result,
        bytes32 factor1,
        bytes32 factor2,
        bytes32 divisor,
        bytes1 scalarByte,
        FheType fheType
    ) external;

    /// @notice Computes a unary op (`fheNeg` / `fheNot`) over `ct`'s cleartext and records it.
    function recordUnaryOp(Operators op, bytes32 result, bytes32 ct, FheType fheType) external;

    /// @notice Computes a ternary op over the operands' cleartexts and records it into `result`.
    ///         The only ternary op is `fheIfThenElse(lhs, middle, rhs)`.
    function recordTernaryOp(Operators op, bytes32 result, bytes32 lhs, bytes32 middle, bytes32 rhs) external;

    /// @notice Computes an nary op over the operands' cleartexts and records it into `result`.
    ///         For `fheSum`, `value` is unused (pass bytes32(0)) and `values` are the summands. For
    ///         `fheIsIn`, `value` is the needle and `values` are the set. v13 operators — their presence
    ///         is what forces a v12->v13 `CleartextArithmetic` upgrade, since the v12 arithmetic has no
    ///         such selector.
    function recordNaryOp(Operators op, bytes32 result, bytes32 value, bytes32[] memory values, FheType fheType)
        external;
}
