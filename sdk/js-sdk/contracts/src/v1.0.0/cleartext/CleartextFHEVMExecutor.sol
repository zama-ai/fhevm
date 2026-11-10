// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FHEVMExecutor} from "../host-contracts/contracts/FHEVMExecutor.sol";
import {FheType} from "./shared/LibFheType.sol";
import {ICleartextArithmetic} from "./shared/interfaces/ICleartextArithmetic.sol";
import {IPlaintexts} from "./shared/interfaces/IPlaintexts.sol";
import {Operators as CleartextOperators} from "./shared/FhevmOperatorsEnum.sol";
import {cleartextArithmeticAdd, paddedInputterAdd} from "../host-contracts/addresses/FHEVMHostAddresses.sol";

/// @notice FHEVMExecutor variant that mirrors every operation's cleartext into the cleartext layer.
/// @dev Each override runs the real symbolic op (`super`), then delegates the cleartext bookkeeping
///      to the external `CleartextArithmetic` contract, which computes the result and persists it in
///      `CleartextDB`. The executor never touches the DB — keeping the arithmetic + storage bytecode
///      out of this contract preserves EIP-170 headroom and lets multiple executors share one DB.
contract CleartextFHEVMExecutor is FHEVMExecutor, IPlaintexts {
    /// @notice Marks a cleartext (mock) implementation. Real host contracts have no such selector, so a
    ///         consumer can probe it to tell a cleartext stack from a production deployment.
    bool public constant IS_CLEARTEXT = true;

    /// @dev Handle to cleartext value mapping for local testing.
    //mapping(bytes32 => uint256) public plaintexts;
    function plaintexts(bytes32 result) public view override returns (uint256) {
        return _cleartext().plaintexts(result);
    }

    /// @dev Lets a reader tell a handle worth zero from one this stack never minted.
    function hasPlaintext(bytes32 result) public view returns (bool) {
        return _cleartext().hasPlaintext(result);
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
        _cleartext().recordBinaryOp(_op(op), result, lhs, rhs, scalarByte, _typeOf(lhs));
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
        _cleartext().recordMulDiv(result, factor1, factor2, divisor, scalarByte, resultType);
    }

    function _unaryOp(Operators op, bytes32 ct) internal override returns (bytes32 result) {
        result = super._unaryOp(op, ct);
        _cleartext().recordUnaryOp(_op(op), result, ct, _typeOf(ct));
    }

    function _ternaryOp(Operators op, bytes32 lhs, bytes32 middle, bytes32 rhs)
        internal
        override
        returns (bytes32 result)
    {
        result = super._ternaryOp(op, lhs, middle, rhs);
        _cleartext().recordTernaryOp(_op(op), result, lhs, middle, rhs);
    }

    /// @dev `fheSum` nary op (values only; no needle).
    function _naryOp(Operators op, bytes32[] calldata values, FheType resultType)
        internal
        override
        returns (bytes32 result)
    {
        result = super._naryOp(op, values, resultType);
        _cleartext().recordNaryOp(_op(op), result, bytes32(0), values, resultType);
    }

    /// @dev `fheIsIn` nary op (`value` needle + `values` set).
    function _naryOp(Operators op, bytes32 value, bytes32[] calldata values, FheType resultType)
        internal
        override
        returns (bytes32 result)
    {
        result = super._naryOp(op, value, values, resultType);
        _cleartext().recordNaryOp(_op(op), result, value, values, _typeOf(value));
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
     * @notice Getter function for the PaddedInputter (RFC-030) contract address.
     * @dev CLEARTEXT-ONLY: the stock `FHEVMExecutor` exposes none. Part of the cleartext stack for good, not a
     *      stopgap: it is how a client reaches `CleartextPaddedInputter`, and so the RFC-030 pad decryption mock
     *      (`getCleartextPadSharesMaterial`), from the ACL (ACL -> executor -> PaddedInputter). Deployment checks
     *      also read back the address the executor bakes through it. If upstream's executor gains its own getter,
     *      keep this one or make sure it returns the same address.
     */
    function getPaddedInputterAddress() public view virtual returns (address) {
        return address(paddedInputterAdd);
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
