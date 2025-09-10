// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {Ownable2StepUpgradeable} from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import {UUPSUpgradeableEmptyProxy} from "./shared/UUPSUpgradeableEmptyProxy.sol";

import {ACL} from "./ACL.sol";
import {HCULimit} from "./HCULimit.sol";
import {aclAdd, hcuLimitAdd, inputVerifierAdd} from "../addresses/FHEVMHostAddresses.sol";

import {FheType} from "./shared/FheType.sol";
import {HANDLE_VERSION} from "./shared/Constants.sol";
import {FHEEvents} from "./FHEEvents.sol";
import {ACLChecks} from "./shared/ACLChecks.sol";

/**
 * @title IInputVerifier.
 */
interface IInputVerifier {
    function verifyCiphertext(
        FHEVMExecutor.ContextUserInputs memory context,
        bytes32 inputHandle,
        bytes memory inputProof
    ) external returns (bytes32);
}

/**
 * @title    FHEVMExecutor.
 * @notice   This contract implements symbolic execution on the blockchain and one of its
 *           main responsibilities is to deterministically generate ciphertext handles.
 * @dev      This contract is deployed using an UUPS proxy.
 */
contract FHEVMExecutor is UUPSUpgradeableEmptyProxy, Ownable2StepUpgradeable, FHEEvents, ACLChecks {
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

    /// @custom:storage-location erc7201:fhevm.storage.FHEVMExecutor
    struct FHEVMExecutorStorage {
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

    /// @notice Name of the contract.
    string private constant CONTRACT_NAME = "FHEVMExecutor";

    /// @notice Major version of the contract.
    uint256 private constant MAJOR_VERSION = 0;

    /// @notice Minor version of the contract.
    uint256 private constant MINOR_VERSION = 3;

    /// @notice Patch version of the contract.
    uint256 private constant PATCH_VERSION = 0;

    /// @notice ACL.
    ACL private constant acl = ACL(aclAdd);

    /// @notice hcuLimit.
    HCULimit private constant hcuLimit = HCULimit(hcuLimitAdd);

    /// @notice IInputVerifier.
    IInputVerifier private constant inputVerifier = IInputVerifier(inputVerifierAdd);

    /// Constant used for making sure the version number used in the `reinitializer` modifier is
    /// identical between `initializeFromEmptyProxy` and the `reinitializeVX` method
    uint64 private constant REINITIALIZER_VERSION = 4;

    /// keccak256(abi.encode(uint256(keccak256("fhevm.storage.FHEVMExecutor")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant FHEVMExecutorStorageLocation =
        0x4613e1771f6b755d243e536fb5a23c5b15e2826575fee921e8fe7a22a760c800;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice  Initializes the contract.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy() public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        __Ownable_init(owner());
    }

    /**
     * @notice Re-initializes the contract from V1.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV3() public virtual reinitializer(REINITIALIZER_VERSION) {}

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
        FheType lhsType = _verifyAndReturnType(lhs, supportedTypes);
        bytes1 scalar = scalarByte & 0x01;
        result = _binaryOp(Operators.fheAdd, lhs, rhs, scalar, lhsType);
        hcuLimit.checkHCUForFheAdd(lhsType, scalar, lhs, rhs, result);
        emit FheAdd(msg.sender, lhs, rhs, scalarByte, result);
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
        FheType lhsType = _verifyAndReturnType(lhs, supportedTypes);
        bytes1 scalar = scalarByte & 0x01;
        result = _binaryOp(Operators.fheSub, lhs, rhs, scalar, lhsType);
        hcuLimit.checkHCUForFheSub(lhsType, scalar, lhs, rhs, result);
        emit FheSub(msg.sender, lhs, rhs, scalarByte, result);
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
        FheType lhsType = _verifyAndReturnType(lhs, supportedTypes);
        bytes1 scalar = scalarByte & 0x01;
        result = _binaryOp(Operators.fheMul, lhs, rhs, scalar, lhsType);
        hcuLimit.checkHCUForFheMul(lhsType, scalar, lhs, rhs, result);
        emit FheMul(msg.sender, lhs, rhs, scalarByte, result);
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
        FheType lhsType = _verifyAndReturnType(lhs, supportedTypes);
        bytes1 scalar = scalarByte & 0x01;
        result = _binaryOp(Operators.fheDiv, lhs, rhs, scalar, lhsType);
        hcuLimit.checkHCUForFheDiv(lhsType, scalar, lhs, rhs, result);
        emit FheDiv(msg.sender, lhs, rhs, scalarByte, result);
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
        FheType lhsType = _verifyAndReturnType(lhs, supportedTypes);
        bytes1 scalar = scalarByte & 0x01;
        result = _binaryOp(Operators.fheRem, lhs, rhs, scalar, lhsType);
        hcuLimit.checkHCUForFheRem(lhsType, scalar, lhs, rhs, result);
        emit FheRem(msg.sender, lhs, rhs, scalarByte, result);
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
        FheType lhsType = _verifyAndReturnType(lhs, supportedTypes);
        bytes1 scalar = scalarByte & 0x01;
        result = _binaryOp(Operators.fheBitAnd, lhs, rhs, scalar, lhsType);
        hcuLimit.checkHCUForFheBitAnd(lhsType, scalar, lhs, rhs, result);
        emit FheBitAnd(msg.sender, lhs, rhs, scalarByte, result);
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
        FheType lhsType = _verifyAndReturnType(lhs, supportedTypes);
        bytes1 scalar = scalarByte & 0x01;
        result = _binaryOp(Operators.fheBitOr, lhs, rhs, scalar, lhsType);
        hcuLimit.checkHCUForFheBitOr(lhsType, scalar, lhs, rhs, result);
        emit FheBitOr(msg.sender, lhs, rhs, scalarByte, result);
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
        FheType lhsType = _verifyAndReturnType(lhs, supportedTypes);
        bytes1 scalar = scalarByte & 0x01;
        result = _binaryOp(Operators.fheBitXor, lhs, rhs, scalar, lhsType);
        hcuLimit.checkHCUForFheBitXor(lhsType, scalar, lhs, rhs, result);
        emit FheBitXor(msg.sender, lhs, rhs, scalarByte, result);
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
        FheType lhsType = _verifyAndReturnType(lhs, supportedTypes);
        bytes1 scalar = scalarByte & 0x01;
        result = _binaryOp(Operators.fheShl, lhs, rhs, scalar, lhsType);
        hcuLimit.checkHCUForFheShl(lhsType, scalar, lhs, rhs, result);
        emit FheShl(msg.sender, lhs, rhs, scalarByte, result);
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
        FheType lhsType = _verifyAndReturnType(lhs, supportedTypes);
        bytes1 scalar = scalarByte & 0x01;
        result = _binaryOp(Operators.fheShr, lhs, rhs, scalar, lhsType);
        hcuLimit.checkHCUForFheShr(lhsType, scalar, lhs, rhs, result);
        emit FheShr(msg.sender, lhs, rhs, scalarByte, result);
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
        FheType lhsType = _verifyAndReturnType(lhs, supportedTypes);
        bytes1 scalar = scalarByte & 0x01;
        result = _binaryOp(Operators.fheRotl, lhs, rhs, scalar, lhsType);
        hcuLimit.checkHCUForFheRotl(lhsType, scalar, lhs, rhs, result);
        emit FheRotl(msg.sender, lhs, rhs, scalarByte, result);
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
        FheType lhsType = _verifyAndReturnType(lhs, supportedTypes);
        bytes1 scalar = scalarByte & 0x01;
        result = _binaryOp(Operators.fheRotr, lhs, rhs, scalar, lhsType);
        hcuLimit.checkHCUForFheRotr(lhsType, scalar, lhs, rhs, result);
        emit FheRotr(msg.sender, lhs, rhs, scalarByte, result);
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
            (1 << uint8(FheType.Uint256));
        FheType lhsType = _verifyAndReturnType(lhs, supportedTypes);
        bytes1 scalar = scalarByte & 0x01;

        result = _binaryOp(Operators.fheEq, lhs, rhs, scalar, FheType.Bool);
        hcuLimit.checkHCUForFheEq(lhsType, scalar, lhs, rhs, result);
        emit FheEq(msg.sender, lhs, rhs, scalarByte, result);
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
            (1 << uint8(FheType.Uint256));
        FheType lhsType = _verifyAndReturnType(lhs, supportedTypes);
        bytes1 scalar = scalarByte & 0x01;

        result = _binaryOp(Operators.fheNe, lhs, rhs, scalar, FheType.Bool);
        hcuLimit.checkHCUForFheNe(lhsType, scalar, lhs, rhs, result);
        emit FheNe(msg.sender, lhs, rhs, scalarByte, result);
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
        FheType lhsType = _verifyAndReturnType(lhs, supportedTypes);
        bytes1 scalar = scalarByte & 0x01;
        result = _binaryOp(Operators.fheGe, lhs, rhs, scalar, FheType.Bool);
        hcuLimit.checkHCUForFheGe(lhsType, scalar, lhs, rhs, result);
        emit FheGe(msg.sender, lhs, rhs, scalarByte, result);
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
        FheType lhsType = _verifyAndReturnType(lhs, supportedTypes);
        bytes1 scalar = scalarByte & 0x01;
        result = _binaryOp(Operators.fheGt, lhs, rhs, scalar, FheType.Bool);
        hcuLimit.checkHCUForFheGt(lhsType, scalar, lhs, rhs, result);
        emit FheGt(msg.sender, lhs, rhs, scalarByte, result);
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
        FheType lhsType = _verifyAndReturnType(lhs, supportedTypes);
        bytes1 scalar = scalarByte & 0x01;
        result = _binaryOp(Operators.fheLe, lhs, rhs, scalar, FheType.Bool);
        hcuLimit.checkHCUForFheLe(lhsType, scalar, lhs, rhs, result);
        emit FheLe(msg.sender, lhs, rhs, scalarByte, result);
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
        FheType lhsType = _verifyAndReturnType(lhs, supportedTypes);
        bytes1 scalar = scalarByte & 0x01;
        result = _binaryOp(Operators.fheLt, lhs, rhs, scalar, FheType.Bool);
        hcuLimit.checkHCUForFheLt(lhsType, scalar, lhs, rhs, result);
        emit FheLt(msg.sender, lhs, rhs, scalarByte, result);
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
        FheType lhsType = _verifyAndReturnType(lhs, supportedTypes);
        bytes1 scalar = scalarByte & 0x01;
        result = _binaryOp(Operators.fheMin, lhs, rhs, scalar, lhsType);
        hcuLimit.checkHCUForFheMin(lhsType, scalar, lhs, rhs, result);
        emit FheMin(msg.sender, lhs, rhs, scalarByte, result);
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
        FheType lhsType = _verifyAndReturnType(lhs, supportedTypes);
        bytes1 scalar = scalarByte & 0x01;
        result = _binaryOp(Operators.fheMax, lhs, rhs, scalar, lhsType);
        hcuLimit.checkHCUForFheMax(lhsType, scalar, lhs, rhs, result);
        emit FheMax(msg.sender, lhs, rhs, scalarByte, result);
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
        FheType typeCt = _verifyAndReturnType(ct, supportedTypes);
        result = _unaryOp(Operators.fheNeg, ct);
        hcuLimit.checkHCUForFheNeg(typeCt, ct, result);
        emit FheNeg(msg.sender, ct, result);
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
        FheType typeCt = _verifyAndReturnType(ct, supportedTypes);
        result = _unaryOp(Operators.fheNot, ct);
        hcuLimit.checkHCUForFheNot(typeCt, ct, result);
        emit FheNot(msg.sender, ct, result);
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
            (1 << uint8(FheType.Uint256));
        FheType typeCt = _verifyAndReturnType(ifTrue, supportedTypes);
        result = _ternaryOp(Operators.fheIfThenElse, control, ifTrue, ifFalse);
        hcuLimit.checkHCUForIfThenElse(typeCt, control, ifTrue, ifFalse, result);
        emit FheIfThenElse(msg.sender, control, ifTrue, ifFalse, result);
    }

    /**
     * @notice              Computes FHERand operation.
     * @param randType      Type for the random result.
     * @return result       Result.
     */
    function fheRand(FheType randType) public virtual returns (bytes32 result) {
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
    function fheRandBounded(uint256 upperBound, FheType randType) public virtual returns (bytes32 result) {
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
    function cast(bytes32 ct, FheType toType) public virtual returns (bytes32 result) {
        if (!acl.isAllowed(ct, msg.sender)) revert ACLNotAllowed(ct, msg.sender);
        uint256 supportedTypesInput = (1 << uint8(FheType.Bool)) +
            (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128)) +
            (1 << uint8(FheType.Uint256));
        FheType typeCt = _verifyAndReturnType(ct, supportedTypesInput);
        uint256 supportedTypesOutput = (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128)) +
            (1 << uint8(FheType.Uint256)); // @note: unsupported casting to ebool (use fheNe instead)
        if ((1 << uint8(toType)) & supportedTypesOutput == 0) revert UnsupportedType();

        /// @dev It must not cast to same type.
        if (typeCt == toType) revert InvalidType();
        result = keccak256(abi.encodePacked(Operators.cast, ct, toType, acl, block.chainid));
        result = _appendMetadataToPrehandle(result, toType);
        hcuLimit.checkHCUForCast(toType, ct, result);
        acl.allowTransient(result, msg.sender);
        emit Cast(msg.sender, ct, toType, result);
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
        result = keccak256(abi.encodePacked(Operators.trivialEncrypt, pt, toType, acl, block.chainid));
        result = _appendMetadataToPrehandle(result, toType);
        hcuLimit.checkHCUForTrivialEncrypt(toType, result);
        acl.allowTransient(result, msg.sender);
        emit TrivialEncrypt(msg.sender, pt, toType, result);
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
        emit VerifyCiphertext(msg.sender, inputHandle, userAddress, inputProof, inputType, result);
    }

    /**
     * @notice Getter function for the ACL contract address.
     */
    function getACLAddress() public view virtual returns (address) {
        return address(acl);
    }

    /**
     * @notice Getter function for the HCULimit contract address.
     */
    function getHCULimitAddress() public view virtual returns (address) {
        return address(hcuLimit);
    }

    /**
     * @notice Getter function for the InputVerifier contract address.
     */
    function getInputVerifierAddress() public view virtual returns (address) {
        return address(inputVerifier);
    }

    /**
     * @notice        Getter for the handle version.
     * @return uint8 The current version for new handles.
     */
    function getHandleVersion() external pure virtual returns (uint8) {
        return HANDLE_VERSION;
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

    function _verifyAndReturnType(
        bytes32 handle,
        uint256 supportedTypes
    ) internal pure virtual returns (FheType typeCt) {
        typeCt = _typeOf(handle);
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
        FHEVMExecutorStorage storage $ = _getFHEVMExecutorStorage();
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
            (1 << uint8(FheType.Uint256));

        /// @dev Unsupported erandom type.
        if ((1 << uint8(randType)) & supportedTypes == 0) revert UnsupportedType();
        result = keccak256(abi.encodePacked(Operators.fheRand, randType, seed));
        result = _appendMetadataToPrehandle(result, randType);
        hcuLimit.checkHCUForFheRand(randType, result);
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
        result = keccak256(abi.encodePacked(Operators.fheRandBounded, upperBound, randType, seed));
        result = _appendMetadataToPrehandle(result, randType);
        hcuLimit.checkHCUForFheRandBounded(randType, result);
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
     * @dev Returns the FHEVMExecutor storage location.
     */
    function _getFHEVMExecutorStorage() internal pure returns (FHEVMExecutorStorage storage $) {
        assembly {
            $.slot := FHEVMExecutorStorageLocation
        }
    }

    /**
     * @dev Should revert when `msg.sender` is not authorized to upgrade the contract.
     */
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyACLOwner {}
}
