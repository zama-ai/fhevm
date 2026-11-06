// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FHEVMExecutor} from "../host-contracts/contracts/FHEVMExecutor.sol";
import {FheType} from "./shared/LibFheType.sol";
import {ICleartextArithmetic} from "./shared/interfaces/ICleartextArithmetic.sol";
import {IPlaintexts} from "./shared/interfaces/IPlaintexts.sol";
import {Operators as CleartextOperators} from "./shared/FhevmOperatorsEnum.sol";
import {cleartextArithmeticAdd} from "../host-contracts/addresses/FHEVMHostAddresses.sol";
import {VmSafe} from "forge-std/Vm.sol";

/// @notice FHEVMExecutor variant that mirrors every operation's cleartext into the cleartext layer.
/// @dev Each override runs the real symbolic op (`super`), then delegates the cleartext bookkeeping
///      to the external `CleartextArithmetic` contract, which computes the result and persists it in
///      `CleartextDB`. The executor never touches the DB — keeping the arithmetic + storage bytecode
///      out of this contract preserves EIP-170 headroom and lets multiple executors share one DB.
contract CleartextForgeFHEVMExecutor is FHEVMExecutor, IPlaintexts {
    VmSafe private constant vmSafe = VmSafe(address(uint160(uint256(keccak256("hevm cheat code")))));

    /// @notice Marks the Forge-only variant, mirroring forge-std's `IS_TEST`. A constant rather than a
    ///         storage variable: these run behind proxies, where an initialized state variable would
    ///         only ever be set in the implementation's own storage and read back as `false`.
    bool public constant IS_FORGE = true;

    /// @notice Same marker as `CleartextFHEVMExecutor`: this variant extends `FHEVMExecutor` directly, so it
    ///         does not inherit it, yet it is every bit as much a cleartext implementation.
    bool public constant IS_CLEARTEXT = true;

    /// @dev Handle to cleartext value mapping for local testing.
    //mapping(bytes32 => uint256) public plaintexts;
    function plaintexts(bytes32 result) public view returns (uint256) {
        return _cleartext().plaintexts(result);
    }

    /// @dev Lets a reader tell a handle worth zero from one this stack never minted — the same `IPlaintexts`
    ///      face as `CleartextFHEVMExecutor`, so whatever reads plaintexts through the executor can ask.
    function hasPlaintext(bytes32 result) public view returns (bool) {
        return _cleartext().hasPlaintext(result);
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
            _cleartext().recordBinaryOp(_op(op), result, lhs, rhs, scalarByte, _typeOf(lhs));
        }
        vmSafe.resumeGasMetering();
    }

    /// @dev `fheMulDiv` (since v14), the widened `(factor1 * factor2) / divisor`; the divisor is always plaintext.
    function _mulDivOp(
        Operators op,
        bytes32 factor1,
        bytes32 factor2,
        bytes32 divisor,
        bytes1 scalarByte,
        FheType resultType
    ) internal override returns (bytes32 result) {
        result = super._mulDivOp(op, factor1, factor2, divisor, scalarByte, resultType);
        vmSafe.pauseGasMetering();
        {
            _cleartext().recordMulDiv(result, factor1, factor2, divisor, scalarByte, resultType);
        }
        vmSafe.resumeGasMetering();
    }

    function _unaryOp(Operators op, bytes32 ct) internal override returns (bytes32 result) {
        result = super._unaryOp(op, ct);
        vmSafe.pauseGasMetering();
        {
            _cleartext().recordUnaryOp(_op(op), result, ct, _typeOf(ct));
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
            _cleartext().recordTernaryOp(_op(op), result, lhs, middle, rhs);
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
            _cleartext().recordNaryOp(_op(op), result, bytes32(0), values, resultType);
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
            _cleartext().recordNaryOp(_op(op), result, value, values, _typeOf(value));
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

    /**
     * @dev The vendored executor's `Operators` and the cleartext layer's are two DECLARATIONS of the
     *      same list, so Solidity treats them as two types and a value crosses only through `uint8`.
     *      That crossing happens here and nowhere else, so the record* calls below read as if there
     *      were one type — and `FhevmOperators.t.sol` is what guarantees the positions still line up.
     */
    function _op(Operators op) private pure returns (CleartextOperators) {
        return CleartextOperators(uint8(op));
    }
}
