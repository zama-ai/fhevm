// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {TFHEExecutorNoEvents} from "./TFHEExecutorNoEvents.sol";
import {FheType} from "./FheType.sol";

/**
 * @title    TFHEExecutor
 * @notice   This contract inherits TFHEExecutorNoEvents and overrides its functions to emit
 *           events for all TFHE operations.
 * @dev      This contract is deployed using an UUPS proxy.
 */
contract TFHEExecutor is TFHEExecutorNoEvents {
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
    event FheEqBytes(address indexed caller, bytes32 lhs, bytes rhs, bytes1 scalarByte, bytes32 result);
    event FheNe(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result);
    event FheNeBytes(address indexed caller, bytes32 lhs, bytes rhs, bytes1 scalarByte, bytes32 result);
    event FheGe(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result);
    event FheGt(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result);
    event FheLe(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result);
    event FheLt(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result);
    event FheMin(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result);
    event FheMax(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result);
    event FheNeg(address indexed caller, bytes32 ct, bytes32 result);
    event FheNot(address indexed caller, bytes32 ct, bytes32 result);
    event VerifyCiphertext(
        address indexed caller,
        bytes32 inputHandle,
        address userAddress,
        bytes inputProof,
        FheType inputType,
        bytes32 result
    );
    event Cast(address indexed caller, bytes32 ct, FheType toType, bytes32 result);
    event TrivialEncrypt(address indexed caller, uint256 pt, FheType toType, bytes32 result);
    event TrivialEncryptBytes(address indexed caller, bytes pt, FheType toType, bytes32 result);
    event FheIfThenElse(address indexed caller, bytes32 control, bytes32 ifTrue, bytes32 ifFalse, bytes32 result);
    event FheRand(address indexed caller, FheType randType, bytes16 seed, bytes32 result);
    event FheRandBounded(address indexed caller, uint256 upperBound, FheType randType, bytes16 seed, bytes32 result);

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice              Computes FHEAdd operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheAdd(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual override returns (bytes32 result) {
        result = super.fheAdd(lhs, rhs, scalarByte);
        emit FheAdd(msg.sender, lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHESub operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheSub(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual override returns (bytes32 result) {
        result = super.fheSub(lhs, rhs, scalarByte);
        emit FheSub(msg.sender, lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHEMul operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheMul(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual override returns (bytes32 result) {
        result = super.fheMul(lhs, rhs, scalarByte);
        emit FheMul(msg.sender, lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHEDiv operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheDiv(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual override returns (bytes32 result) {
        result = super.fheDiv(lhs, rhs, scalarByte);
        emit FheDiv(msg.sender, lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHERem operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheRem(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual override returns (bytes32 result) {
        result = super.fheRem(lhs, rhs, scalarByte);
        emit FheRem(msg.sender, lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHEBitAnd operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheBitAnd(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual override returns (bytes32 result) {
        result = super.fheBitAnd(lhs, rhs, scalarByte);
        emit FheBitAnd(msg.sender, lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHEBitOr operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheBitOr(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual override returns (bytes32 result) {
        result = super.fheBitOr(lhs, rhs, scalarByte);
        emit FheBitOr(msg.sender, lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHEBitXor operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheBitXor(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual override returns (bytes32 result) {
        result = super.fheBitXor(lhs, rhs, scalarByte);
        emit FheBitXor(msg.sender, lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHEShl operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheShl(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual override returns (bytes32 result) {
        result = super.fheShl(lhs, rhs, scalarByte);
        emit FheShl(msg.sender, lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHEShr operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheShr(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual override returns (bytes32 result) {
        result = super.fheShr(lhs, rhs, scalarByte);
        emit FheShr(msg.sender, lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHERotl operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheRotl(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual override returns (bytes32 result) {
        result = super.fheRotl(lhs, rhs, scalarByte);
        emit FheRotl(msg.sender, lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHERotr operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheRotr(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual override returns (bytes32 result) {
        result = super.fheRotr(lhs, rhs, scalarByte);
        emit FheRotr(msg.sender, lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHEEq operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheEq(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual override returns (bytes32 result) {
        result = super.fheEq(lhs, rhs, scalarByte);
        emit FheEq(msg.sender, lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHEEq operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheEq(bytes32 lhs, bytes memory rhs, bytes1 scalarByte) public virtual override returns (bytes32 result) {
        result = super.fheEq(lhs, rhs, scalarByte);
        emit FheEqBytes(msg.sender, lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHENe operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheNe(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual override returns (bytes32 result) {
        result = super.fheNe(lhs, rhs, scalarByte);
        emit FheNe(msg.sender, lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHENe operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheNe(bytes32 lhs, bytes memory rhs, bytes1 scalarByte) public virtual override returns (bytes32 result) {
        result = super.fheNe(lhs, rhs, scalarByte);
        emit FheNeBytes(msg.sender, lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHEGe operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheGe(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual override returns (bytes32 result) {
        result = super.fheGe(lhs, rhs, scalarByte);
        emit FheGe(msg.sender, lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHEGt operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheGt(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual override returns (bytes32 result) {
        result = super.fheGt(lhs, rhs, scalarByte);
        emit FheGt(msg.sender, lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHELe operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheLe(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual override returns (bytes32 result) {
        result = super.fheLe(lhs, rhs, scalarByte);
        emit FheLe(msg.sender, lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHELt operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheLt(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual override returns (bytes32 result) {
        result = super.fheLt(lhs, rhs, scalarByte);
        emit FheLt(msg.sender, lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHEMin operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheMin(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual override returns (bytes32 result) {
        result = super.fheMin(lhs, rhs, scalarByte);
        emit FheMin(msg.sender, lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHEMax operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheMax(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual override returns (bytes32 result) {
        result = super.fheMax(lhs, rhs, scalarByte);
        emit FheMax(msg.sender, lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHENeg operation.
     * @param ct            Ct
     * @return result       Result.
     */
    function fheNeg(bytes32 ct) public virtual override returns (bytes32 result) {
        result = super.fheNeg(ct);
        emit FheNeg(msg.sender, ct, result);
    }

    /**
     * @notice              Computes FHENot operation.
     * @param ct            Ct
     * @return result       Result.
     */
    function fheNot(bytes32 ct) public virtual override returns (bytes32 result) {
        result = super.fheNot(ct);
        emit FheNot(msg.sender, ct, result);
    }

    /**
     * @notice              Computes FHEIfThenElse operation.
     * @param control       Control value.
     * @param ifTrue        If true.
     * @param ifFalse       If false.
     * @return result       Result.
     */
    function fheIfThenElse(
        bytes32 control,
        bytes32 ifTrue,
        bytes32 ifFalse
    ) public virtual override returns (bytes32 result) {
        result = super.fheIfThenElse(control, ifTrue, ifFalse);
        emit FheIfThenElse(msg.sender, control, ifTrue, ifFalse, result);
    }

    /**
     * @notice              Computes FHERand operation.
     * @param randType      Type for the random result.
     * @return result       Result.
     */
    function fheRand(FheType randType) public virtual override returns (bytes32 result) {
        bytes16 seed = _generateSeed();
        result = _generateRand(randType, seed);
        emit FheRand(msg.sender, randType, seed, result);
    }

    /**
     * @notice              Computes FHERandBounded operation.
     * @param upperBound    Upper bound value.
     * @param randType      Type for the random result.
     * @return result       Result.
     */
    function fheRandBounded(uint256 upperBound, FheType randType) public virtual override returns (bytes32 result) {
        bytes16 seed = _generateSeed();
        result = _generateRandBounded(upperBound, randType, seed);
        emit FheRandBounded(msg.sender, upperBound, randType, seed, result);
    }

    /**
     * @notice          Performs the casting to a target type.
     * @param ct        Value to cast.
     * @param toType    Target type.
     * @return result   Result value of the target type.
     */
    function cast(bytes32 ct, FheType toType) public virtual override returns (bytes32 result) {
        result = super.cast(ct, toType);
        emit Cast(msg.sender, ct, toType, result);
    }

    /**
     * @notice          Does trivial encryption.
     * @param pt        Value to encrypt.
     * @param toType    Target type.
     * @return result   Result value of the target type.
     */
    function trivialEncrypt(uint256 pt, FheType toType) public virtual override returns (bytes32 result) {
        result = super.trivialEncrypt(pt, toType);
        emit TrivialEncrypt(msg.sender, pt, toType, result);
    }

    /**
     * @notice          Does trivial encryption.
     * @param pt        Value to encrypt.
     * @param toType    Target type.
     * @return result   Result value of the target type.
     * @dev             This is an overloaded function for ebytesXX types.
     */
    function trivialEncrypt(bytes memory pt, FheType toType) public virtual override returns (bytes32 result) {
        result = super.trivialEncrypt(pt, toType);
        emit TrivialEncryptBytes(msg.sender, pt, toType, result);
    }

    /**
     * @notice              Verifies the ciphertext.
     * @param inputHandle   Input handle.
     * @param userAddress   Address of the user.
     * @param inputProof    Input proof.
     * @param inputType     Input type.
     * @return result       Result.
     */
    function verifyCiphertext(
        bytes32 inputHandle,
        address userAddress,
        bytes memory inputProof,
        FheType inputType
    ) public virtual override returns (bytes32 result) {
        result = super.verifyCiphertext(inputHandle, userAddress, inputProof, inputType);
        emit VerifyCiphertext(msg.sender, inputHandle, userAddress, inputProof, inputType, result);
    }
}
