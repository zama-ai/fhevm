// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FHEVMExecutor} from "../contracts/FHEVMExecutor.sol";
import {FheType} from "../contracts/shared/FheType.sol";
import {ICleartextArithmetic} from "./ICleartextArithmetic.sol";
import {cleartextArithmeticAdd} from "../addresses/FHEVMHostAddresses.sol";

/// @notice FHEVMExecutor variant that mirrors every operation's cleartext into the cleartext layer.
/// @dev Each override runs the real symbolic op (`super`), then delegates the cleartext bookkeeping
///      to the external `CleartextArithmetic` contract, which computes the result and persists it in
///      `CleartextDB`. The executor never touches the DB — keeping the arithmetic + storage bytecode
///      out of this contract preserves EIP-170 headroom and lets multiple executors share one DB.
contract CleartextFHEVMExecutor is FHEVMExecutor {
    /// @dev Handle to cleartext value mapping for local testing.
    //mapping(bytes32 => uint256) public plaintexts;
    function plaintexts(bytes32 result) public view returns (uint256) {
        return _cleartext().plaintexts(result);
    }

    function cast(bytes32 ct, FheType toType) public override returns (bytes32 result) {
        result = super.cast(ct, toType);
        _cleartext().recordCast(result, ct, toType);
    }

    function trivialEncrypt(uint256 pt, FheType toType) public override returns (bytes32 result) {
        result = super.trivialEncrypt(pt, toType);
        _cleartext().recordTrivialEncrypt(result, pt, toType);
    }

    function verifyInput(bytes32 inputHandle, address userAddress, bytes memory inputProof, FheType inputType)
        public
        override
        returns (bytes32 result)
    {
        result = super.verifyInput(inputHandle, userAddress, inputProof, inputType);
        _cleartext().recordVerifyInput(result, inputHandle, inputProof, inputType);
    }

    function _generateRand(FheType randType, bytes16 seed) internal override returns (bytes32 result) {
        result = super._generateRand(randType, seed);
        _cleartext().recordRand(result, randType, seed);
    }

    function _generateRandBounded(uint256 upperBound, FheType randType, bytes16 seed)
        internal
        override
        returns (bytes32 result)
    {
        result = super._generateRandBounded(upperBound, randType, seed);
        _cleartext().recordRandBounded(result, upperBound, seed);
    }

    function _binaryOp(Operators op, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, FheType resultType)
        internal
        override
        returns (bytes32 result)
    {
        result = super._binaryOp(op, lhs, rhs, scalarByte, resultType);
        _cleartext().recordBinaryOp(op, result, lhs, rhs, scalarByte, _typeOf(lhs));
    }

    function _unaryOp(Operators op, bytes32 ct) internal override returns (bytes32 result) {
        result = super._unaryOp(op, ct);
        _cleartext().recordUnaryOp(op, result, ct, _typeOf(ct));
    }

    /// @dev `fheMulDiv` (protocol v0.14) routes through its own hook rather than `_ternaryOp`, so it
    ///      needs its own override — without it, mul-div results would never reach `CleartextDB`.
    ///      `resultType` is the caller-verified type of `factor1`; the divisor is a plaintext scalar.
    function _mulDivOp(
        Operators op,
        bytes32 factor1,
        bytes32 factor2,
        bytes32 divisor,
        bytes1 scalarByte,
        FheType resultType
    ) internal override returns (bytes32 result) {
        result = super._mulDivOp(op, factor1, factor2, divisor, scalarByte, resultType);
        _cleartext().recordMulDiv(result, factor1, factor2, divisor, scalarByte, resultType);
    }

    function _ternaryOp(Operators op, bytes32 lhs, bytes32 middle, bytes32 rhs)
        internal
        override
        returns (bytes32 result)
    {
        result = super._ternaryOp(op, lhs, middle, rhs);
        if (op == Operators.fheIfThenElse) {
            _cleartext().recordIfThenElse(result, lhs, middle, rhs);
        }
    }

    function _cleartext() private pure returns (ICleartextArithmetic) {
        return ICleartextArithmetic(cleartextArithmeticAdd);
    }
}
