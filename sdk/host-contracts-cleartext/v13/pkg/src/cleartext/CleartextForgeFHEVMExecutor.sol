// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FHEVMExecutor} from "../contracts/FHEVMExecutor.sol";
import {FheType} from "../contracts/shared/FheType.sol";
import {ICleartextArithmetic} from "./ICleartextArithmetic.sol";
import {cleartextArithmeticAdd} from "../addresses/FHEVMHostAddresses.sol";
import {VmSafe} from "forge-std/Vm.sol";

/// @notice FHEVMExecutor variant that mirrors every operation's cleartext into the cleartext layer.
/// @dev Each override runs the real symbolic op (`super`), then delegates the cleartext bookkeeping
///      to the external `CleartextArithmetic` contract, which computes the result and persists it in
///      `CleartextDB`. The executor never touches the DB — keeping the arithmetic + storage bytecode
///      out of this contract preserves EIP-170 headroom and lets multiple executors share one DB.
contract CleartextForgeFHEVMExecutor is FHEVMExecutor {
    VmSafe private constant vmSafe = VmSafe(address(uint160(uint256(keccak256("hevm cheat code")))));

    /// @dev Handle to cleartext value mapping for local testing.
    //mapping(bytes32 => uint256) public plaintexts;
    function plaintexts(bytes32 result) public view returns (uint256) {
        return _cleartext().plaintexts(result);
    }

    function cast(bytes32 ct, FheType toType) public override returns (bytes32 result) {
        result = super.cast(ct, toType);
        vmSafe.pauseGasMetering();
        {
            _cleartext().recordCast(result, ct, toType);
        }
        vmSafe.resumeGasMetering();
    }

    function trivialEncrypt(uint256 pt, FheType toType) public override returns (bytes32 result) {
        result = super.trivialEncrypt(pt, toType);
        vmSafe.pauseGasMetering();
        {
            _cleartext().recordTrivialEncrypt(result, pt, toType);
        }
        vmSafe.resumeGasMetering();
    }

    function verifyInput(bytes32 inputHandle, address userAddress, bytes memory inputProof, FheType inputType)
        public
        override
        returns (bytes32 result)
    {
        result = super.verifyInput(inputHandle, userAddress, inputProof, inputType);
        vmSafe.pauseGasMetering();
        {
            _cleartext().recordVerifyInput(result, inputHandle, inputProof, inputType);
        }
        vmSafe.resumeGasMetering();
    }

    function _generateRand(FheType randType, bytes16 seed) internal override returns (bytes32 result) {
        result = super._generateRand(randType, seed);
        vmSafe.pauseGasMetering();
        {
            _cleartext().recordRand(result, randType, seed);
        }
        vmSafe.resumeGasMetering();
    }

    function _generateRandBounded(uint256 upperBound, FheType randType, bytes16 seed)
        internal
        override
        returns (bytes32 result)
    {
        result = super._generateRandBounded(upperBound, randType, seed);
        vmSafe.pauseGasMetering();
        {
            _cleartext().recordRandBounded(result, upperBound, seed);
        }
        vmSafe.resumeGasMetering();
    }

    function _binaryOp(Operators op, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, FheType resultType)
        internal
        override
        returns (bytes32 result)
    {
        result = super._binaryOp(op, lhs, rhs, scalarByte, resultType);
        vmSafe.pauseGasMetering();
        {
            _cleartext().recordBinaryOp(op, result, lhs, rhs, scalarByte, _typeOf(lhs));
        }
        vmSafe.resumeGasMetering();
    }

    function _unaryOp(Operators op, bytes32 ct) internal override returns (bytes32 result) {
        result = super._unaryOp(op, ct);
        vmSafe.pauseGasMetering();
        {
            _cleartext().recordUnaryOp(op, result, ct, _typeOf(ct));
        }
        vmSafe.resumeGasMetering();
    }

    function _ternaryOp(Operators op, bytes32 lhs, bytes32 middle, bytes32 rhs)
        internal
        override
        returns (bytes32 result)
    {
        result = super._ternaryOp(op, lhs, middle, rhs);
        vmSafe.pauseGasMetering();
        {
            _cleartext().recordTernaryOp(op, result, lhs, middle, rhs);
        }
        vmSafe.resumeGasMetering();
    }

    /// @dev `fheSum` nary op (values only; no needle).
    function _naryOp(Operators op, bytes32[] calldata values, FheType resultType)
        internal
        override
        returns (bytes32 result)
    {
        result = super._naryOp(op, values, resultType);
        vmSafe.pauseGasMetering();
        {
            _cleartext().recordNaryOp(op, result, bytes32(0), values, resultType);
        }
        vmSafe.resumeGasMetering();
    }

    /// @dev `fheIsIn` nary op (`value` needle + `values` set).
    function _naryOp(Operators op, bytes32 value, bytes32[] calldata values, FheType resultType)
        internal
        override
        returns (bytes32 result)
    {
        result = super._naryOp(op, value, values, resultType);
        vmSafe.pauseGasMetering();
        {
            _cleartext().recordNaryOp(op, result, value, values, _typeOf(value));
        }
        vmSafe.resumeGasMetering();
    }

    function _cleartext() private pure returns (ICleartextArithmetic) {
        return ICleartextArithmetic(cleartextArithmeticAdd);
    }

    /**
     * @notice Getter function for the CleartextArithmetic contract address.
     */
    function getCleartextArithmeticAddress() public view virtual returns (address) {
        return address(cleartextArithmeticAdd);
    }
}
