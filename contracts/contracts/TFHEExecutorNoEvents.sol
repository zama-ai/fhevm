// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {Ownable2StepUpgradeable} from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

import {ACL} from "./ACL.sol";
import {FHEGasLimit} from "./FHEGasLimit.sol";
import {aclAdd} from "../addresses/ACLAddress.sol";
import {fheGasLimitAdd} from "../addresses/FHEGasLimitAddress.sol";
import {inputVerifierAdd} from "../addresses/InputVerifierAddress.sol";

import {FheType} from "./FheType.sol";

/**
 * @title IInputVerifier.
 */
interface IInputVerifier {
    function verifyCiphertext(
        TFHEExecutorNoEvents.ContextUserInputs memory context,
        bytes32 inputHandle,
        bytes memory inputProof
    ) external returns (bytes32);
}

/**
 * @title    TFHEExecutorNoEvents.
 * @notice   This contract implements symbolic execution on the blockchain and one of its
 *           main responsibilities is to deterministically generate ciphertext handles.
 * @dev      This contract is deployed using an UUPS proxy.
 */
contract TFHEExecutorNoEvents is UUPSUpgradeable, Ownable2StepUpgradeable {
    /// @notice         Returned when the handle is not allowed in the ACL for the account.
    /// @param handle   Handle.
    /// @param account  Address of the account.
    error ACLNotAllowed(bytes32 handle, address account);

    /// @notice Returned when the FHE operator attempts to divide by zero.
    error DivisionByZero();

    /// @notice Returned if two types are not compatible for this operation.
    error IncompatibleTypes();

    /// @notice Returned if the length of the bytes is not as expected.
    error InvalidByteLength(FheType typeOf, uint256 length);

    /// @notice Returned if the type is not the expected one.
    error InvalidType();

    /// @notice Returned if it uses the wrong overloaded function (for functions fheEq/fheNe),
    ///         which does not handle scalar.
    error IsScalar();

    /// @notice Returned if operation is supported only for a scalar (functions fheDiv/fheRem).
    error IsNotScalar();

    /// @notice Returned if the upper bound for generating randomness is not a power of two.
    error NotPowerOfTwo();

    /// @notice Returned if the second operand is not a scalar (for functions fheEq/fheNe).
    error SecondOperandIsNotScalar();

    /// @notice Returned if the type is not supported for this operation.
    error UnsupportedType();

    /**
     * @param userAddress       Address of the user.
     * @param contractAddress   Contract address.
     */
    struct ContextUserInputs {
        address userAddress;
        address contractAddress;
    }

    /// @custom:storage-location erc7201:httpz.storage.HTTPZExecutor
    struct TFHEExecutorStorage {
        /// @dev Counter used for computing handles of randomness operators. It is also used for OPRF, which is used to
        ///      generate pseudo-random ciphertexts.
        uint256 counterRand;
    }

    enum Operators {
        fheAdd,
        fheSub,
        fheMul,
        fheDiv,
        fheRem,
        fheBitAnd,
        fheBitOr,
        fheBitXor,
        fheShl,
        fheShr,
        fheRotl,
        fheRotr,
        fheEq,
        fheNe,
        fheGe,
        fheGt,
        fheLe,
        fheLt,
        fheMin,
        fheMax,
        fheNeg,
        fheNot,
        verifyCiphertext,
        cast,
        trivialEncrypt,
        fheIfThenElse,
        fheRand,
        fheRandBounded
    }

    /// @notice Handle version.
    uint8 public constant HANDLE_VERSION = 0;

    /// @notice Name of the contract.
    string private constant CONTRACT_NAME = "TFHEExecutor";

    /// @notice Major version of the contract.
    uint256 private constant MAJOR_VERSION = 0;

    /// @notice Minor version of the contract.
    uint256 private constant MINOR_VERSION = 1;

    /// @notice Patch version of the contract.
    uint256 private constant PATCH_VERSION = 0;

    /// @notice ACL.
    ACL private constant acl = ACL(aclAdd);

    /// @notice FHEGasLimit.
    FHEGasLimit private constant fheGasLimit = FHEGasLimit(fheGasLimitAdd);

    /// @notice IInputVerifier.
    IInputVerifier private constant inputVerifier = IInputVerifier(inputVerifierAdd);

    /// keccak256(abi.encode(uint256(keccak256("httpz.storage.HTTPZExecutor")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant TFHEExecutorStorageLocation =
        0x3d02b8d0de856b0609b3629cf5f3cd56c0504e3831cd53973d36422116206500;

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
    function fheAdd(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual returns (bytes32 result) {
        uint256 supportedTypes = (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128));
        _requireType(lhs, supportedTypes);
        FheType lhsType = _typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheAdd(lhsType, scalar);
        result = _binaryOp(Operators.fheAdd, lhs, rhs, scalar, lhsType);
    }

    /**
     * @notice              Computes FHESub operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheSub(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual returns (bytes32 result) {
        uint256 supportedTypes = (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128));
        _requireType(lhs, supportedTypes);
        FheType lhsType = _typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheSub(lhsType, scalar);
        result = _binaryOp(Operators.fheSub, lhs, rhs, scalar, lhsType);
    }

    /**
     * @notice              Computes FHEMul operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheMul(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual returns (bytes32 result) {
        uint256 supportedTypes = (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128));
        _requireType(lhs, supportedTypes);
        FheType lhsType = _typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheMul(lhsType, scalar);
        result = _binaryOp(Operators.fheMul, lhs, rhs, scalar, lhsType);
    }

    /**
     * @notice              Computes FHEDiv operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheDiv(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual returns (bytes32 result) {
        if (scalarByte & 0x01 != 0x01) revert IsNotScalar();
        if (rhs == 0) revert DivisionByZero();
        uint256 supportedTypes = (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128));
        _requireType(lhs, supportedTypes);
        FheType lhsType = _typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheDiv(lhsType, scalar);
        result = _binaryOp(Operators.fheDiv, lhs, rhs, scalar, lhsType);
    }

    /**
     * @notice              Computes FHERem operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheRem(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual returns (bytes32 result) {
        if (scalarByte & 0x01 != 0x01) revert IsNotScalar();
        if (rhs == 0) revert DivisionByZero();
        uint256 supportedTypes = (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128));
        _requireType(lhs, supportedTypes);
        FheType lhsType = _typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheRem(lhsType, scalar);
        result = _binaryOp(Operators.fheRem, lhs, rhs, scalar, lhsType);
    }

    /**
     * @notice              Computes FHEBitAnd operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheBitAnd(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual returns (bytes32 result) {
        uint256 supportedTypes = (1 << uint8(FheType.Bool)) +
            (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128)) +
            (1 << uint8(FheType.Uint256));
        _requireType(lhs, supportedTypes);
        FheType lhsType = _typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheBitAnd(lhsType, scalar);
        result = _binaryOp(Operators.fheBitAnd, lhs, rhs, scalar, lhsType);
    }

    /**
     * @notice              Computes FHEBitOr operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheBitOr(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual returns (bytes32 result) {
        uint256 supportedTypes = (1 << uint8(FheType.Bool)) +
            (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128)) +
            (1 << uint8(FheType.Uint256));
        _requireType(lhs, supportedTypes);
        FheType lhsType = _typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheBitOr(lhsType, scalar);
        result = _binaryOp(Operators.fheBitOr, lhs, rhs, scalar, lhsType);
    }

    /**
     * @notice              Computes FHEBitXor operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheBitXor(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual returns (bytes32 result) {
        uint256 supportedTypes = (1 << uint8(FheType.Bool)) +
            (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128)) +
            (1 << uint8(FheType.Uint256));
        _requireType(lhs, supportedTypes);
        FheType lhsType = _typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheBitXor(lhsType, scalar);
        result = _binaryOp(Operators.fheBitXor, lhs, rhs, scalar, lhsType);
    }

    /**
     * @notice              Computes FHEShl operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheShl(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual returns (bytes32 result) {
        uint256 supportedTypes = (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128)) +
            (1 << uint8(FheType.Uint256));
        _requireType(lhs, supportedTypes);
        FheType lhsType = _typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheShl(lhsType, scalar);
        result = _binaryOp(Operators.fheShl, lhs, rhs, scalar, lhsType);
    }

    /**
     * @notice              Computes FHEShr operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheShr(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual returns (bytes32 result) {
        uint256 supportedTypes = (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128)) +
            (1 << uint8(FheType.Uint256));
        _requireType(lhs, supportedTypes);
        FheType lhsType = _typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheShr(lhsType, scalar);
        result = _binaryOp(Operators.fheShr, lhs, rhs, scalar, lhsType);
    }

    /**
     * @notice              Computes FHERotl operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheRotl(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual returns (bytes32 result) {
        uint256 supportedTypes = (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128)) +
            (1 << uint8(FheType.Uint256));
        _requireType(lhs, supportedTypes);
        FheType lhsType = _typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheRotl(lhsType, scalar);
        result = _binaryOp(Operators.fheRotl, lhs, rhs, scalar, lhsType);
    }

    /**
     * @notice              Computes FHERotr operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheRotr(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual returns (bytes32 result) {
        uint256 supportedTypes = (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128)) +
            (1 << uint8(FheType.Uint256));
        _requireType(lhs, supportedTypes);
        FheType lhsType = _typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheRotr(lhsType, scalar);
        result = _binaryOp(Operators.fheRotr, lhs, rhs, scalar, lhsType);
    }

    /**
     * @notice              Computes FHEEq operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheEq(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual returns (bytes32 result) {
        uint256 supportedTypes = (1 << uint8(FheType.Bool)) +
            (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128)) +
            (1 << uint8(FheType.Uint160)) +
            (1 << uint8(FheType.Uint256)) +
            (1 << uint8(FheType.Uint512)) +
            (1 << uint8(FheType.Uint1024)) +
            (1 << uint8(FheType.Uint2048));

        _requireType(lhs, supportedTypes);
        FheType lhsType = _typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        if (scalar == 0x01 && uint8(lhsType) > 8) revert IsScalar();

        fheGasLimit.payForFheEq(lhsType, scalar);
        result = _binaryOp(Operators.fheEq, lhs, rhs, scalar, FheType.Bool);
    }

    /**
     * @notice              Computes FHEEq operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheEq(bytes32 lhs, bytes memory rhs, bytes1 scalarByte) public virtual returns (bytes32 result) {
        uint256 supportedTypes = (1 << uint8(FheType.Uint512)) +
            (1 << uint8(FheType.Uint1024)) +
            (1 << uint8(FheType.Uint2048));
        _requireType(lhs, supportedTypes);
        FheType lhsType = _typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;

        if (scalar != 0x01) revert SecondOperandIsNotScalar();
        fheGasLimit.payForFheEq(lhsType, scalar);

        if (!acl.isAllowed(lhs, msg.sender)) revert ACLNotAllowed(lhs, msg.sender);
        _checkByteLengthForEbytesTypes(rhs.length, lhsType);

        result = keccak256(abi.encodePacked(Operators.fheEq, lhs, rhs, scalar, acl, block.chainid));
        result = _appendMetadataToPrehandle(result, FheType.Bool);
        acl.allowTransient(result, msg.sender);
    }

    /**
     * @notice              Computes FHENe operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheNe(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual returns (bytes32 result) {
        uint256 supportedTypes = (1 << uint8(FheType.Bool)) +
            (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128)) +
            (1 << uint8(FheType.Uint160)) +
            (1 << uint8(FheType.Uint256)) +
            (1 << uint8(FheType.Uint512)) +
            (1 << uint8(FheType.Uint1024)) +
            (1 << uint8(FheType.Uint2048));

        _requireType(lhs, supportedTypes);
        FheType lhsType = _typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        if (scalar == 0x01 && uint8(lhsType) > 8) revert IsScalar();

        fheGasLimit.payForFheNe(lhsType, scalar);
        result = _binaryOp(Operators.fheNe, lhs, rhs, scalar, FheType.Bool);
    }

    /**
     * @notice              Computes FHENe operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheNe(bytes32 lhs, bytes memory rhs, bytes1 scalarByte) public virtual returns (bytes32 result) {
        uint256 supportedTypes = (1 << uint8(FheType.Uint512)) +
            (1 << uint8(FheType.Uint1024)) +
            (1 << uint8(FheType.Uint2048));
        _requireType(lhs, supportedTypes);
        FheType lhsType = _typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;

        if (scalar != 0x01) revert SecondOperandIsNotScalar();
        fheGasLimit.payForFheNe(lhsType, scalar);
        if (!acl.isAllowed(lhs, msg.sender)) revert ACLNotAllowed(lhs, msg.sender);
        _checkByteLengthForEbytesTypes(rhs.length, lhsType);
        result = keccak256(abi.encodePacked(Operators.fheNe, lhs, rhs, scalar, acl, block.chainid));
        result = _appendMetadataToPrehandle(result, FheType.Bool);
        acl.allowTransient(result, msg.sender);
    }

    /**
     * @notice              Computes FHEGe operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheGe(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual returns (bytes32 result) {
        uint256 supportedTypes = (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128));
        _requireType(lhs, supportedTypes);
        FheType lhsType = _typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheGe(lhsType, scalar);
        result = _binaryOp(Operators.fheGe, lhs, rhs, scalar, FheType.Bool);
    }

    /**
     * @notice              Computes FHEGt operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheGt(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual returns (bytes32 result) {
        uint256 supportedTypes = (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128));
        _requireType(lhs, supportedTypes);
        FheType lhsType = _typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheGt(lhsType, scalar);
        result = _binaryOp(Operators.fheGt, lhs, rhs, scalar, FheType.Bool);
    }

    /**
     * @notice              Computes FHELe operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheLe(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual returns (bytes32 result) {
        uint256 supportedTypes = (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128));
        _requireType(lhs, supportedTypes);
        FheType lhsType = _typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheLe(lhsType, scalar);
        result = _binaryOp(Operators.fheLe, lhs, rhs, scalar, FheType.Bool);
    }

    /**
     * @notice              Computes FHELt operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheLt(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual returns (bytes32 result) {
        uint256 supportedTypes = (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128));
        _requireType(lhs, supportedTypes);
        FheType lhsType = _typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheLt(lhsType, scalar);
        result = _binaryOp(Operators.fheLt, lhs, rhs, scalar, FheType.Bool);
    }

    /**
     * @notice              Computes FHEMin operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheMin(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual returns (bytes32 result) {
        uint256 supportedTypes = (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128));
        _requireType(lhs, supportedTypes);
        FheType lhsType = _typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheMin(lhsType, scalar);
        result = _binaryOp(Operators.fheMin, lhs, rhs, scalar, lhsType);
    }

    /**
     * @notice              Computes FHEMax operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheMax(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public virtual returns (bytes32 result) {
        uint256 supportedTypes = (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128));
        _requireType(lhs, supportedTypes);
        FheType lhsType = _typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheMax(lhsType, scalar);
        result = _binaryOp(Operators.fheMax, lhs, rhs, scalar, lhsType);
    }

    /**
     * @notice              Computes FHENeg operation.
     * @param ct            Ct
     * @return result       Result.
     */
    function fheNeg(bytes32 ct) public virtual returns (bytes32 result) {
        uint256 supportedTypes = (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128)) +
            (1 << uint8(FheType.Uint256));
        _requireType(ct, supportedTypes);
        FheType typeCt = _typeOf(ct);
        fheGasLimit.payForFheNeg(typeCt);
        result = _unaryOp(Operators.fheNeg, ct);
    }

    /**
     * @notice              Computes FHENot operation.
     * @param ct            Ct
     * @return result       Result.
     */
    function fheNot(bytes32 ct) public virtual returns (bytes32 result) {
        uint256 supportedTypes = (1 << uint8(FheType.Bool)) +
            (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128)) +
            (1 << uint8(FheType.Uint256));
        _requireType(ct, supportedTypes);
        FheType typeCt = _typeOf(ct);
        fheGasLimit.payForFheNot(typeCt);
        result = _unaryOp(Operators.fheNot, ct);
    }

    /**
     * @notice              Computes FHEIfThenElse operation.
     * @param control       Control value.
     * @param ifTrue        If true.
     * @param ifFalse       If false.
     * @return result       Result.
     */
    function fheIfThenElse(bytes32 control, bytes32 ifTrue, bytes32 ifFalse) public virtual returns (bytes32 result) {
        uint256 supportedTypes = (1 << uint8(FheType.Bool)) +
            (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128)) +
            (1 << uint8(FheType.Uint160)) +
            (1 << uint8(FheType.Uint256)) +
            (1 << uint8(FheType.Uint512)) +
            (1 << uint8(FheType.Uint1024)) +
            (1 << uint8(FheType.Uint2048));
        _requireType(ifTrue, supportedTypes);
        FheType typeCt = _typeOf(ifTrue);
        fheGasLimit.payForIfThenElse(typeCt);
        result = _ternaryOp(Operators.fheIfThenElse, control, ifTrue, ifFalse);
    }

    /**
     * @notice              Computes FHERand operation.
     * @param randType      Type for the random result.
     * @return result       Result.
     */
    function fheRand(FheType randType) public virtual returns (bytes32 result) {
        bytes16 seed = _generateSeed();
        result = _generateRand(randType, seed);
    }

    /**
     * @notice              Computes FHERandBounded operation.
     * @param upperBound    Upper bound value.
     * @param randType      Type for the random result.
     * @return result       Result.
     */
    function fheRandBounded(uint256 upperBound, FheType randType) public virtual returns (bytes32 result) {
        bytes16 seed = _generateSeed();
        result = _generateRandBounded(upperBound, randType, seed);
    }

    /**
     * @notice          Performs the casting to a target type.
     * @param ct        Value to cast.
     * @param toType    Target type.
     * @return result   Result value of the target type.
     */
    function cast(bytes32 ct, FheType toType) public virtual returns (bytes32 result) {
        if (!acl.isAllowed(ct, msg.sender)) revert ACLNotAllowed(ct, msg.sender);
        uint256 supportedTypesInput = (1 << uint8(FheType.Bool)) +
            (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128)) +
            (1 << uint8(FheType.Uint256));
        _requireType(ct, supportedTypesInput);
        uint256 supportedTypesOutput = (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128)) +
            (1 << uint8(FheType.Uint256)); // @note: unsupported casting to ebool (use fheNe instead)
        if ((1 << uint8(toType)) & supportedTypesOutput == 0) revert UnsupportedType();

        FheType typeCt = _typeOf(ct);
        /// @dev It must not cast to same type.
        if (typeCt == toType) revert InvalidType();
        fheGasLimit.payForCast(typeCt);
        result = keccak256(abi.encodePacked(Operators.cast, ct, toType, acl, block.chainid));
        result = _appendMetadataToPrehandle(result, toType);
        acl.allowTransient(result, msg.sender);
    }

    /**
     * @notice          Does trivial encryption.
     * @param pt        Value to encrypt.
     * @param toType    Target type.
     * @return result   Result value of the target type.
     */
    function trivialEncrypt(uint256 pt, FheType toType) public virtual returns (bytes32 result) {
        uint256 supportedTypes = (1 << uint8(FheType.Bool)) +
            (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128)) +
            (1 << uint8(FheType.Uint160)) +
            (1 << uint8(FheType.Uint256));

        if ((1 << uint8(toType)) & supportedTypes == 0) revert UnsupportedType();
        fheGasLimit.payForTrivialEncrypt(toType);
        result = keccak256(abi.encodePacked(Operators.trivialEncrypt, pt, toType, acl, block.chainid));
        result = _appendMetadataToPrehandle(result, toType);
        acl.allowTransient(result, msg.sender);
    }

    /**
     * @notice          Does trivial encryption.
     * @param pt        Value to encrypt.
     * @param toType    Target type.
     * @return result   Result value of the target type.
     * @dev             This is an overloaded function for ebytesXX types.
     */
    function trivialEncrypt(bytes memory pt, FheType toType) public virtual returns (bytes32 result) {
        uint256 supportedTypes = (1 << uint8(FheType.Uint512)) +
            (1 << uint8(FheType.Uint1024)) +
            (1 << uint8(FheType.Uint2048));

        if (((1 << uint8(toType)) & supportedTypes == 0)) revert UnsupportedType();
        fheGasLimit.payForTrivialEncrypt(toType);
        _checkByteLengthForEbytesTypes(pt.length, toType);
        result = keccak256(abi.encodePacked(Operators.trivialEncrypt, pt, toType, acl, block.chainid));
        result = _appendMetadataToPrehandle(result, toType);
        acl.allowTransient(result, msg.sender);
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
    ) public virtual returns (bytes32 result) {
        ContextUserInputs memory contextUserInputs = ContextUserInputs({
            userAddress: userAddress,
            contractAddress: msg.sender
        });
        FheType typeCt = _typeOf(inputHandle);
        if (inputType != typeCt) revert InvalidType();
        result = inputVerifier.verifyCiphertext(contextUserInputs, inputHandle, inputProof);
        acl.allowTransient(result, msg.sender);
    }

    /**
     * @notice Getter function for the ACL contract address.
     */
    function getACLAddress() public view virtual returns (address) {
        return address(acl);
    }

    /**
     * @notice Getter function for the FHEGasLimit contract address.
     */
    function getFHEGasLimitAddress() public view virtual returns (address) {
        return address(fheGasLimit);
    }

    /**
     * @notice Getter function for the InputVerifier contract address.
     */
    function getInputVerifierAddress() public view virtual returns (address) {
        return address(inputVerifier);
    }

    /**
     * @notice        Getter for the name and version of the contract.
     * @return string Name and the version of the contract.
     */
    function getVersion() external pure virtual returns (string memory) {
        return
            string(
                abi.encodePacked(
                    CONTRACT_NAME,
                    " v",
                    Strings.toString(MAJOR_VERSION),
                    ".",
                    Strings.toString(MINOR_VERSION),
                    ".",
                    Strings.toString(PATCH_VERSION)
                )
            );
    }

    /**
     * @dev Handle format for user inputs and ops results are as such:
     *      keccak256(keccak256(CiphertextFHEList)||index_handle)[0:20] || index_handle[21] || chainID [22:29] ||  handle_type [30] || handle_version [31]
     *      If the handle stems from computation, the index_handle must be set to 0xff.
     *      The CiphertextFHEList actually contains: 1 byte (= N) for size of handles_list, N bytes for the handles_types : 1 per handle, then the original fhe160list raw ciphertext
     */
    function _typeOf(bytes32 handle) internal pure virtual returns (FheType typeCt) {
        typeCt = FheType(uint8(handle[30]));
    }

    function _appendMetadataToPrehandle(
        bytes32 prehandle,
        FheType handleType
    ) internal view virtual returns (bytes32 result) {
        /// @dev Clear bytes 21-31.
        result = prehandle & 0xffffffffffffffffffffffffffffffffffffffffff0000000000000000000000;
        /// @dev Set byte 21 to 0xff since the new handle comes from computation.
        result = result | (bytes32(uint256(0xff)) << 80);
        /// @dev chainId is cast to uint64 first to make sure it does not take more than 8 bytes before shifting.
        /// If EIP2294 gets approved, it will force the chainID's size to be lower than MAX_UINT64.
        result = result | (bytes32(uint256(uint64(block.chainid))) << 16);
        /// @dev Insert handleType into byte 30.
        result = result | (bytes32(uint256(uint8(handleType))) << 8);
        /// @dev Insert HANDLE_VERSION into byte 31.
        result = result | bytes32(uint256(HANDLE_VERSION));
    }

    /**
     * @dev Checks the length for typeOf that are ebytes.
     */
    function _checkByteLengthForEbytesTypes(uint256 byteLength, FheType fheType) internal pure virtual {
        if (fheType == FheType.Uint512) {
            if (byteLength != 64) revert InvalidByteLength(fheType, byteLength);
        } else if (fheType == FheType.Uint1024) {
            if (byteLength != 128) revert InvalidByteLength(fheType, byteLength);
        } else if (fheType == FheType.Uint2048) {
            if (byteLength != 256) revert InvalidByteLength(fheType, byteLength);
        }
    }

    function _requireType(bytes32 handle, uint256 supportedTypes) internal pure virtual {
        FheType typeCt = _typeOf(handle);
        if ((1 << uint8(typeCt)) & supportedTypes == 0) revert UnsupportedType();
    }

    function _unaryOp(Operators op, bytes32 ct) internal virtual returns (bytes32 result) {
        if (!acl.isAllowed(ct, msg.sender)) revert ACLNotAllowed(ct, msg.sender);
        result = keccak256(abi.encodePacked(op, ct, acl, block.chainid));
        FheType typeCt = _typeOf(ct);
        result = _appendMetadataToPrehandle(result, typeCt);
        acl.allowTransient(result, msg.sender);
    }

    function _binaryOp(
        Operators op,
        bytes32 lhs,
        bytes32 rhs,
        bytes1 scalar,
        FheType resultType
    ) internal virtual returns (bytes32 result) {
        if (!acl.isAllowed(lhs, msg.sender)) revert ACLNotAllowed(lhs, msg.sender);
        if (scalar == 0x00) {
            if (!acl.isAllowed(rhs, msg.sender)) revert ACLNotAllowed(rhs, msg.sender);

            FheType rhsType = _typeOf(rhs);
            FheType lhsType = _typeOf(lhs);
            if (lhsType != rhsType) revert IncompatibleTypes();
        }
        result = keccak256(abi.encodePacked(op, lhs, rhs, scalar, acl, block.chainid));
        result = _appendMetadataToPrehandle(result, resultType);
        acl.allowTransient(result, msg.sender);
    }

    function _ternaryOp(
        Operators op,
        bytes32 lhs,
        bytes32 middle,
        bytes32 rhs
    ) internal virtual returns (bytes32 result) {
        if (!acl.isAllowed(lhs, msg.sender)) revert ACLNotAllowed(lhs, msg.sender);
        if (!acl.isAllowed(middle, msg.sender)) revert ACLNotAllowed(middle, msg.sender);
        if (!acl.isAllowed(rhs, msg.sender)) revert ACLNotAllowed(rhs, msg.sender);

        FheType lhsType = _typeOf(lhs);
        FheType middleType = _typeOf(middle);
        FheType rhsType = _typeOf(rhs);

        /// @dev lhs must be ebool
        if (lhsType != FheType.Bool) revert UnsupportedType();
        if (middleType != rhsType) revert IncompatibleTypes();

        result = keccak256(abi.encodePacked(op, lhs, middle, rhs, acl, block.chainid));
        result = _appendMetadataToPrehandle(result, middleType);
        acl.allowTransient(result, msg.sender);
    }

    function _generateSeed() internal virtual returns (bytes16 seed) {
        TFHEExecutorStorage storage $ = _getTFHEExecutorStorage();
        seed = bytes16(
            keccak256(abi.encodePacked($.counterRand, acl, block.chainid, blockhash(block.number - 1), block.timestamp))
        );
        $.counterRand++;
    }

    function _generateRand(FheType randType, bytes16 seed) internal virtual returns (bytes32 result) {
        uint256 supportedTypes = (1 << uint8(FheType.Bool)) +
            (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128)) +
            (1 << uint8(FheType.Uint256)) +
            (1 << uint8(FheType.Uint512)) +
            (1 << uint8(FheType.Uint1024)) +
            (1 << uint8(FheType.Uint2048));

        /// @dev Unsupported erandom type.
        if ((1 << uint8(randType)) & supportedTypes == 0) revert UnsupportedType();
        fheGasLimit.payForFheRand(randType);
        result = keccak256(abi.encodePacked(Operators.fheRand, randType, seed));
        result = _appendMetadataToPrehandle(result, randType);
        acl.allowTransient(result, msg.sender);
    }

    function _generateRandBounded(
        uint256 upperBound,
        FheType randType,
        bytes16 seed
    ) internal virtual returns (bytes32 result) {
        uint256 supportedTypes = (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128)) +
            (1 << uint8(FheType.Uint256));
        /// @dev Unsupported erandom type.
        if ((1 << uint8(randType)) & supportedTypes == 0) revert UnsupportedType();
        if (!_isPowerOfTwo(upperBound)) revert NotPowerOfTwo();
        fheGasLimit.payForFheRandBounded(randType);
        result = keccak256(abi.encodePacked(Operators.fheRandBounded, upperBound, randType, seed));
        result = _appendMetadataToPrehandle(result, randType);
        acl.allowTransient(result, msg.sender);
    }

    /**
     * @dev     Checks if the value is power of 2.
     * @param x Value to check.
     */
    function _isPowerOfTwo(uint256 x) internal pure virtual returns (bool) {
        return (x > 0) && ((x & (x - 1)) == 0);
    }

    /**
     * @dev Returns the HTTPZExecutor storage location.
     */
    function _getTFHEExecutorStorage() internal pure returns (TFHEExecutorStorage storage $) {
        assembly {
            $.slot := TFHEExecutorStorageLocation
        }
    }

    /**
     * @dev Should revert when `msg.sender` is not authorized to upgrade the contract.
     */
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyOwner {}
}
