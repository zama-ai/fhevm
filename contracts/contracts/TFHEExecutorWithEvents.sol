// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {TFHEExecutor} from "./TFHEExecutor.sol";

/**
 * @title    TFHEExecutorWithEvents
 * @notice   This contract inherits TFHEExecutor and overrides its functions to emit
 *           events for all TFHE operations.
 * @dev      This contract is deployed using an UUPS proxy.
 */
contract TFHEExecutorWithEvents is TFHEExecutor {
    event FheAdd(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result);
    event FheSub(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result);
    event FheMul(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result);
    event FheDiv(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result);
    event FheRem(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result);
    event FheBitAnd(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result);
    event FheBitOr(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result);
    event FheBitXor(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result);
    event FheShl(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result);
    event FheShr(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result);
    event FheRotl(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result);
    event FheRotr(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result);
    event FheEq(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result);
    event FheEqBytes(uint256 lhs, bytes rhs, bytes1 scalarByte, uint256 result);
    event FheNe(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result);
    event FheNeBytes(uint256 lhs, bytes rhs, bytes1 scalarByte, uint256 result);
    event FheGe(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result);
    event FheGt(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result);
    event FheLe(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result);
    event FheLt(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result);
    event FheMin(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result);
    event FheMax(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result);
    event FheNeg(uint256 ct, uint256 result);
    event FheNot(uint256 ct, uint256 result);
    event VerifyCiphertext(
        bytes32 inputHandle,
        address userAddress,
        bytes inputProof,
        bytes1 inputType,
        uint256 result
    );
    event Cast(uint256 ct, bytes1 toType, uint256 result);
    event TrivialEncrypt(uint256 pt, bytes1 toType, uint256 result);
    event TrivialEncryptBytes(bytes pt, bytes1 toType, uint256 result);
    event FheIfThenElse(uint256 control, uint256 ifTrue, uint256 ifFalse, uint256 result);
    event FheRand(bytes1 randType, bytes16 seed, uint256 result);
    event FheRandBounded(uint256 upperBound, bytes1 randType, bytes16 seed, uint256 result);

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
    function fheAdd(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual override returns (uint256 result) {
        result = super.fheAdd(lhs, rhs, scalarByte);
        emit FheAdd(lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHESub operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheSub(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual override returns (uint256 result) {
        result = super.fheSub(lhs, rhs, scalarByte);
        emit FheSub(lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHEMul operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheMul(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual override returns (uint256 result) {
        result = super.fheMul(lhs, rhs, scalarByte);
        emit FheMul(lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHEDiv operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheDiv(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual override returns (uint256 result) {
        result = super.fheDiv(lhs, rhs, scalarByte);
        emit FheDiv(lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHERem operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheRem(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual override returns (uint256 result) {
        result = super.fheRem(lhs, rhs, scalarByte);
        emit FheRem(lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHEBitAnd operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheBitAnd(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual override returns (uint256 result) {
        result = super.fheBitAnd(lhs, rhs, scalarByte);
        emit FheBitAnd(lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHEBitOr operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheBitOr(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual override returns (uint256 result) {
        result = super.fheBitOr(lhs, rhs, scalarByte);
        emit FheBitOr(lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHEBitXor operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheBitXor(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual override returns (uint256 result) {
        result = super.fheBitXor(lhs, rhs, scalarByte);
        emit FheBitXor(lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHEShl operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheShl(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual override returns (uint256 result) {
        result = super.fheShl(lhs, rhs, scalarByte);
        emit FheShl(lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHEShr operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheShr(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual override returns (uint256 result) {
        result = super.fheShr(lhs, rhs, scalarByte);
        emit FheShr(lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHERotl operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheRotl(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual override returns (uint256 result) {
        result = super.fheRotl(lhs, rhs, scalarByte);
        emit FheRotl(lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHERotr operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheRotr(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual override returns (uint256 result) {
        result = super.fheRotr(lhs, rhs, scalarByte);
        emit FheRotr(lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHEEq operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheEq(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual override returns (uint256 result) {
        result = super.fheEq(lhs, rhs, scalarByte);
        emit FheEq(lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHEEq operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheEq(uint256 lhs, bytes memory rhs, bytes1 scalarByte) public virtual override returns (uint256 result) {
        result = super.fheEq(lhs, rhs, scalarByte);
        emit FheEqBytes(lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHENe operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheNe(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual override returns (uint256 result) {
        result = super.fheNe(lhs, rhs, scalarByte);
        emit FheNe(lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHENe operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheNe(uint256 lhs, bytes memory rhs, bytes1 scalarByte) public virtual override returns (uint256 result) {
        result = super.fheNe(lhs, rhs, scalarByte);
        emit FheNeBytes(lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHEGe operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheGe(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual override returns (uint256 result) {
        result = super.fheGe(lhs, rhs, scalarByte);
        emit FheGe(lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHEGt operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheGt(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual override returns (uint256 result) {
        result = super.fheGt(lhs, rhs, scalarByte);
        emit FheGt(lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHELe operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheLe(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual override returns (uint256 result) {
        result = super.fheLe(lhs, rhs, scalarByte);
        emit FheLe(lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHELt operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheLt(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual override returns (uint256 result) {
        result = super.fheLt(lhs, rhs, scalarByte);
        emit FheLt(lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHEMin operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheMin(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual override returns (uint256 result) {
        result = super.fheMin(lhs, rhs, scalarByte);
        emit FheMin(lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHEMax operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheMax(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual override returns (uint256 result) {
        result = super.fheMax(lhs, rhs, scalarByte);
        emit FheMax(lhs, rhs, scalarByte, result);
    }

    /**
     * @notice              Computes FHENeg operation.
     * @param ct            Ct
     * @return result       Result.
     */
    function fheNeg(uint256 ct) public virtual override returns (uint256 result) {
        result = super.fheNeg(ct);
        emit FheNeg(ct, result);
    }

    /**
     * @notice              Computes FHENot operation.
     * @param ct            Ct
     * @return result       Result.
     */
    function fheNot(uint256 ct) public virtual override returns (uint256 result) {
        result = super.fheNot(ct);
        emit FheNot(ct, result);
    }

    /**
     * @notice              Computes FHEIfThenElse operation.
     * @param control       Control value.
     * @param ifTrue        If true.
     * @param ifFalse       If false.
     * @return result       Result.
     */
    function fheIfThenElse(
        uint256 control,
        uint256 ifTrue,
        uint256 ifFalse
    ) public virtual override returns (uint256 result) {
        result = super.fheIfThenElse(control, ifTrue, ifFalse);
        emit FheIfThenElse(control, ifTrue, ifFalse, result);
    }

    /**
     * @notice              Computes FHERand operation.
     * @param randType      Type for the random result.
     * @return result       Result.
     */
    function fheRand(bytes1 randType) public virtual override returns (uint256 result) {
        bytes16 seed = _generateSeed();
        result = _generateRand(randType, seed);
        emit FheRand(randType, seed, result);
    }

    /**
     * @notice              Computes FHERandBounded operation.
     * @param upperBound    Upper bound value.
     * @param randType      Type for the random result.
     * @return result       Result.
     */
    function fheRandBounded(uint256 upperBound, bytes1 randType) public virtual override returns (uint256 result) {
        bytes16 seed = _generateSeed();
        result = _generateRandBounded(upperBound, randType, seed);
        emit FheRandBounded(upperBound, randType, seed, result);
    }

    /**
     * @notice          Performs the casting to a target type.
     * @param ct        Value to cast.
     * @param toType    Target type.
     * @return result   Result value of the target type.
     */
    function cast(uint256 ct, bytes1 toType) public virtual override returns (uint256 result) {
        result = super.cast(ct, toType);
        emit Cast(ct, toType, result);
    }

    /**
     * @notice          Does trivial encryption.
     * @param pt        Value to encrypt.
     * @param toType    Target type.
     * @return result   Result value of the target type.
     */
    function trivialEncrypt(uint256 pt, bytes1 toType) public virtual override returns (uint256 result) {
        result = super.trivialEncrypt(pt, toType);
        emit TrivialEncrypt(pt, toType, result);
    }

    /**
     * @notice          Does trivial encryption.
     * @param pt        Value to encrypt.
     * @param toType    Target type.
     * @return result   Result value of the target type.
     * @dev             This is an overloaded function for ebytesXX types.
     */
    function trivialEncrypt(bytes memory pt, bytes1 toType) public virtual override returns (uint256 result) {
        result = super.trivialEncrypt(pt, toType);
        emit TrivialEncryptBytes(pt, toType, result);
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
        bytes1 inputType
    ) public virtual override returns (uint256 result) {
        result = super.verifyCiphertext(inputHandle, userAddress, inputProof, inputType);
        emit VerifyCiphertext(inputHandle, userAddress, inputProof, inputType, result);
    }
}
