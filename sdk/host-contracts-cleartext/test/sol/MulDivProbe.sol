// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FheType} from "../../src/contracts/shared/FheType.sol";

interface IExecutor {
    function trivialEncrypt(uint256 pt, FheType toType) external returns (bytes32);
    function fheMulDiv(bytes32 factor1, bytes32 factor2, bytes32 divisor, bytes1 scalarByte)
        external
        returns (bytes32);
}

/**
 * @title MulDivProbe
 * @notice Test-only probe that exercises `fheMulDiv` end to end in a SINGLE transaction.
 * @dev The executor grants operand access via `ACL.allowTransient`, which lives in transient storage
 *      and is gone by the next transaction. An EOA therefore cannot encrypt in one tx and mul-div in
 *      the next — the `fheMulDiv` ACL check would revert. Doing both from one contract call keeps the
 *      transient grants alive, which is the only way to drive the op from outside.
 *
 *      Not shipped: this lives under `test/`, so it is compiled by `forge build` but excluded from
 *      the published package.
 */
contract MulDivProbe {
    /// @dev `fheMulDiv` bitmask: bit 0 (divisor) is always set; bit 1 marks `factor2` as scalar.
    bytes1 private constant FACTOR2_ENCRYPTED = 0x01;
    bytes1 private constant FACTOR2_SCALAR = 0x03;

    IExecutor private immutable EXECUTOR;

    /// @notice Handle produced by the most recent `run`, so callers can read it back after the tx.
    bytes32 public lastResult;

    constructor(address executor) {
        EXECUTOR = IExecutor(executor);
    }

    /// @notice Encrypts `a` (and `b`, unless `factor2Scalar`), then records `mulDiv(a, b, divisor)`.
    function run(uint256 a, uint256 b, uint256 divisor, FheType fheType, bool factor2Scalar)
        external
        returns (bytes32 result)
    {
        bytes32 factor1 = EXECUTOR.trivialEncrypt(a, fheType);
        bytes32 factor2 = factor2Scalar ? bytes32(b) : EXECUTOR.trivialEncrypt(b, fheType);
        bytes1 scalarByte = factor2Scalar ? FACTOR2_SCALAR : FACTOR2_ENCRYPTED;

        result = EXECUTOR.fheMulDiv(factor1, factor2, bytes32(divisor), scalarByte);
        lastResult = result;
    }
}
