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

interface IInputVerifier {
    function verifyCiphertext(
        TFHEExecutor.ContextUserInputs memory context,
        bytes32 inputHandle,
        bytes memory inputProof
    ) external returns (uint256);
}

/**
 * @title    TFHEExecutor
 * @notice   This contract implements symbolic execution on the blockchain and one of its
 *           main responsibilities is to deterministically generate ciphertext handles.
 * @dev      This contract is deployed using an UUPS proxy.
 */
contract TFHEExecutor is UUPSUpgradeable, Ownable2StepUpgradeable {
    /// @notice Handle version
    uint8 public constant HANDLE_VERSION = 0;

    /// @notice Name of the contract
    string private constant CONTRACT_NAME = "TFHEExecutor";

    /// @notice Version of the contract
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    ACL private constant acl = ACL(aclAdd);
    FHEGasLimit private constant fheGasLimit = FHEGasLimit(fheGasLimitAdd);
    IInputVerifier private constant inputVerifier = IInputVerifier(inputVerifierAdd);

    /// @custom:storage-location erc7201:fhevm.storage.TFHEExecutor
    struct TFHEExecutorStorage {
        uint256 counterRand; /// @notice counter used for computing handles of randomness operators
    }

    struct ContextUserInputs {
        address aclAddress;
        address userAddress;
        address contractAddress;
    }

    // keccak256(abi.encode(uint256(keccak256("fhevm.storage.TFHEExecutor")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant TFHEExecutorStorageLocation =
        0xa436a06f0efce5ea38c956a21e24202a59b3b746d48a23fb52b4a5bc33fe3e00;

    function _getTFHEExecutorStorage() internal pure returns (TFHEExecutorStorage storage $) {
        assembly {
            $.slot := TFHEExecutorStorageLocation
        }
    }

    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyOwner {}

    /// @notice Getter function for the ACL contract address
    function getACLAddress() public view virtual returns (address) {
        return address(acl);
    }

    /// @notice Getter function for the FHEGasLimit contract address
    function getFHEGasLimitAddress() public view virtual returns (address) {
        return address(fheGasLimit);
    }

    /// @notice Getter function for the InputVerifier contract address
    function getInputVerifierAddress() public view virtual returns (address) {
        return address(inputVerifier);
    }

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @notice Initializes the contract setting `initialOwner` as the initial owner
    function initialize(address initialOwner) public initializer {
        __Ownable_init(initialOwner);
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

    function isPowerOfTwo(uint256 x) internal pure virtual returns (bool) {
        return (x > 0) && ((x & (x - 1)) == 0);
    }

    /// @dev handle format for user inputs is: keccak256(keccak256(CiphertextFHEList)||index_handle)[0:29] || index_handle || handle_type || handle_version
    /// @dev other handles format (fhe ops results) is: keccak256(keccak256(rawCiphertextFHEList)||index_handle)[0:30] || handle_type || handle_version
    /// @dev the CiphertextFHEList actually contains: 1 byte (= N) for size of handles_list, N bytes for the handles_types : 1 per handle, then the original fhe160list raw ciphertext
    function typeOf(uint256 handle) internal pure virtual returns (uint8) {
        uint8 typeCt = uint8(handle >> 8);
        return typeCt;
    }

    function appendType(uint256 prehandle, uint8 handleType) internal pure virtual returns (uint256 result) {
        result = prehandle & 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000;
        result = result | (uint256(handleType) << 8); // append type
        result = result | HANDLE_VERSION;
    }

    function requireType(uint256 handle, uint256 supportedTypes) internal pure virtual {
        uint8 typeCt = typeOf(handle);
        require((1 << typeCt) & supportedTypes > 0, "Unsupported type");
    }

    function unaryOp(Operators op, uint256 ct) internal virtual returns (uint256 result) {
        require(acl.isAllowed(ct, msg.sender), "Sender doesn't own ct on op");
        result = uint256(keccak256(abi.encodePacked(op, ct, acl, block.chainid)));
        uint8 typeCt = typeOf(ct);
        result = appendType(result, typeCt);
        acl.allowTransient(result, msg.sender);
    }

    function binaryOp(
        Operators op,
        uint256 lhs,
        uint256 rhs,
        bytes1 scalar,
        uint8 resultType
    ) internal virtual returns (uint256 result) {
        require(acl.isAllowed(lhs, msg.sender), "Sender doesn't own lhs on op");
        if (scalar == 0x00) {
            require(acl.isAllowed(rhs, msg.sender), "Sender doesn't own rhs on op");
            uint8 typeRhs = typeOf(rhs);
            uint8 typeLhs = typeOf(lhs);
            require(typeLhs == typeRhs, "Incompatible types for lhs and rhs");
        }
        result = uint256(keccak256(abi.encodePacked(op, lhs, rhs, scalar, acl, block.chainid)));
        result = appendType(result, resultType);
        acl.allowTransient(result, msg.sender);
    }

    function ternaryOp(
        Operators op,
        uint256 lhs,
        uint256 middle,
        uint256 rhs
    ) internal virtual returns (uint256 result) {
        require(acl.isAllowed(lhs, msg.sender), "Sender doesn't own lhs on op");
        require(acl.isAllowed(middle, msg.sender), "Sender doesn't own middle on op");
        require(acl.isAllowed(rhs, msg.sender), "Sender doesn't own rhs on op");
        uint8 typeLhs = typeOf(lhs);
        uint8 typeMiddle = typeOf(middle);
        uint8 typeRhs = typeOf(rhs);
        require(typeLhs == 0, "Unsupported type for lhs"); // lhs must be ebool
        require(typeMiddle == typeRhs, "Incompatible types for middle and rhs");
        result = uint256(keccak256(abi.encodePacked(op, lhs, middle, rhs, acl, block.chainid)));
        result = appendType(result, typeMiddle);
        acl.allowTransient(result, msg.sender);
    }

    function generateSeed() internal virtual returns (bytes16 seed) {
        TFHEExecutorStorage storage $ = _getTFHEExecutorStorage();
        seed = bytes16(
            keccak256(abi.encodePacked($.counterRand, acl, block.chainid, blockhash(block.number - 1), block.timestamp))
        );
        $.counterRand++;
    }

    function generateRand(bytes1 randType, bytes16 seed) internal virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 0) +
            (1 << 1) +
            (1 << 2) +
            (1 << 3) +
            (1 << 4) +
            (1 << 5) +
            (1 << 6) +
            (1 << 8) +
            (1 << 9) +
            (1 << 10) +
            (1 << 11);
        uint8 randT = uint8(randType);
        require((1 << randT) & supportedTypes > 0, "Unsupported erandom type");
        fheGasLimit.payForFheRand(randT);
        result = uint256(keccak256(abi.encodePacked(Operators.fheRand, randType, seed)));
        result = appendType(result, randT);
        acl.allowTransient(result, msg.sender);
    }

    function generateRandBounded(
        uint256 upperBound,
        bytes1 randType,
        bytes16 seed
    ) internal virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        uint8 randT = uint8(randType);
        require((1 << randT) & supportedTypes > 0, "Unsupported erandom type");
        require(isPowerOfTwo(upperBound), "UpperBound must be a power of 2");
        fheGasLimit.payForFheRandBounded(randT);
        result = uint256(keccak256(abi.encodePacked(Operators.fheRandBounded, upperBound, randType, seed)));
        result = appendType(result, randT);
        acl.allowTransient(result, msg.sender);
    }

    function fheAdd(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheAdd(lhsType, scalar);
        result = binaryOp(Operators.fheAdd, lhs, rhs, scalar, lhsType);
    }

    function fheSub(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheSub(lhsType, scalar);
        result = binaryOp(Operators.fheSub, lhs, rhs, scalar, lhsType);
    }

    function fheMul(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheMul(lhsType, scalar);
        result = binaryOp(Operators.fheMul, lhs, rhs, scalar, lhsType);
    }

    function fheDiv(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual returns (uint256 result) {
        require(scalarByte & 0x01 == 0x01, "Only fheDiv by a scalar is supported");
        require(rhs != 0, "Could not divide by 0");
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheDiv(lhsType, scalar);
        result = binaryOp(Operators.fheDiv, lhs, rhs, scalar, lhsType);
    }

    function fheRem(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual returns (uint256 result) {
        require(scalarByte & 0x01 == 0x01, "Only fheRem by a scalar is supported");
        require(rhs != 0, "Could not divide by 0");
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheRem(lhsType, scalar);
        result = binaryOp(Operators.fheRem, lhs, rhs, scalar, lhsType);
    }

    function fheBitAnd(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 0) + (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheBitAnd(lhsType, scalar);
        result = binaryOp(Operators.fheBitAnd, lhs, rhs, scalar, lhsType);
    }

    function fheBitOr(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 0) + (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheBitOr(lhsType, scalar);
        result = binaryOp(Operators.fheBitOr, lhs, rhs, scalar, lhsType);
    }

    function fheBitXor(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 0) + (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheBitXor(lhsType, scalar);
        result = binaryOp(Operators.fheBitXor, lhs, rhs, scalar, lhsType);
    }

    function fheShl(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheShl(lhsType, scalar);
        result = binaryOp(Operators.fheShl, lhs, rhs, scalar, lhsType);
    }

    function fheShr(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheShr(lhsType, scalar);
        result = binaryOp(Operators.fheShr, lhs, rhs, scalar, lhsType);
    }

    function fheRotl(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheRotl(lhsType, scalar);
        result = binaryOp(Operators.fheRotl, lhs, rhs, scalar, lhsType);
    }

    function fheRotr(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheRotr(lhsType, scalar);
        result = binaryOp(Operators.fheRotr, lhs, rhs, scalar, lhsType);
    }

    function fheEq(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 0) +
            (1 << 1) +
            (1 << 2) +
            (1 << 3) +
            (1 << 4) +
            (1 << 5) +
            (1 << 6) +
            (1 << 7) +
            (1 << 8) +
            (1 << 9) +
            (1 << 10) +
            (1 << 11);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        if (scalar == 0x01) {
            require(lhsType <= 8, "Scalar fheEq for ebytesXXX types must use the overloaded fheEq");
        }
        fheGasLimit.payForFheEq(lhsType, scalar);
        result = binaryOp(Operators.fheEq, lhs, rhs, scalar, 0);
    }

    function fheEq(uint256 lhs, bytes memory rhs, bytes1 scalarByte) public virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 9) + (1 << 10) + (1 << 11);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        require(scalar == 0x01, "Overloaded fheEq is only for scalar ebytesXXX second operand");
        fheGasLimit.payForFheEq(lhsType, scalar);
        require(acl.isAllowed(lhs, msg.sender), "Sender doesn't own lhs on op");
        uint256 lenBytesPT = rhs.length;
        if (lhsType == 9) {
            require(lenBytesPT == 64, "Bytes array length of Bytes64 should be 64");
        } else if (lhsType == 10) {
            require(lenBytesPT == 128, "Bytes array length of Bytes128 should be 128");
        } else {
            // @note: i.e lhsType == 11 thanks to the first pre-condition
            require(lenBytesPT == 256, "Bytes array length of Bytes256 should be 256");
        }
        result = uint256(keccak256(abi.encodePacked(Operators.fheEq, lhs, rhs, scalar, acl, block.chainid)));
        result = appendType(result, 0);
        acl.allowTransient(result, msg.sender);
    }

    function fheNe(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 0) +
            (1 << 1) +
            (1 << 2) +
            (1 << 3) +
            (1 << 4) +
            (1 << 5) +
            (1 << 6) +
            (1 << 7) +
            (1 << 8) +
            (1 << 9) +
            (1 << 10) +
            (1 << 11);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        if (scalar == 0x01) {
            require(lhsType <= 8, "Scalar fheNe for ebytesXXX types must use the overloaded fheNe");
        }
        fheGasLimit.payForFheNe(lhsType, scalar);
        result = binaryOp(Operators.fheNe, lhs, rhs, scalar, 0);
    }

    function fheNe(uint256 lhs, bytes memory rhs, bytes1 scalarByte) public virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 9) + (1 << 10) + (1 << 11);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        require(scalar == 0x01, "Overloaded fheNe is only for scalar ebytesXXX second operand");
        fheGasLimit.payForFheNe(lhsType, scalar);
        require(acl.isAllowed(lhs, msg.sender), "Sender doesn't own lhs on op");
        uint256 lenBytesPT = rhs.length;
        if (lhsType == 9) {
            require(lenBytesPT == 64, "Bytes array length of Bytes64 should be 64");
        } else if (lhsType == 10) {
            require(lenBytesPT == 128, "Bytes array length of Bytes128 should be 128");
        } else {
            // @note: i.e lhsType == 11 thanks to the first pre-condition
            require(lenBytesPT == 256, "Bytes array length of Bytes256 should be 256");
        }
        result = uint256(keccak256(abi.encodePacked(Operators.fheNe, lhs, rhs, scalar, acl, block.chainid)));
        result = appendType(result, 0);
        acl.allowTransient(result, msg.sender);
    }

    function fheGe(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheGe(lhsType, scalar);
        result = binaryOp(Operators.fheGe, lhs, rhs, scalar, 0);
    }

    function fheGt(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheGt(lhsType, scalar);
        result = binaryOp(Operators.fheGt, lhs, rhs, scalar, 0);
    }

    function fheLe(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheLe(lhsType, scalar);
        result = binaryOp(Operators.fheLe, lhs, rhs, scalar, 0);
    }

    function fheLt(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheLt(lhsType, scalar);
        result = binaryOp(Operators.fheLt, lhs, rhs, scalar, 0);
    }

    function fheMin(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheMin(lhsType, scalar);
        result = binaryOp(Operators.fheMin, lhs, rhs, scalar, lhsType);
    }

    function fheMax(uint256 lhs, uint256 rhs, bytes1 scalarByte) public virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fheGasLimit.payForFheMax(lhsType, scalar);
        result = binaryOp(Operators.fheMax, lhs, rhs, scalar, lhsType);
    }

    function fheNeg(uint256 ct) public virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(ct, supportedTypes);
        uint8 typeCt = typeOf(ct);
        fheGasLimit.payForFheNeg(typeCt);
        result = unaryOp(Operators.fheNeg, ct);
    }

    function fheNot(uint256 ct) public virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 0) + (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(ct, supportedTypes);
        uint8 typeCt = typeOf(ct);
        fheGasLimit.payForFheNot(typeCt);
        result = unaryOp(Operators.fheNot, ct);
    }

    function verifyCiphertext(
        bytes32 inputHandle,
        address userAddress,
        bytes memory inputProof,
        bytes1 inputType
    ) public virtual returns (uint256 result) {
        ContextUserInputs memory contextUserInputs = ContextUserInputs({
            aclAddress: address(acl),
            userAddress: userAddress,
            contractAddress: msg.sender
        });
        uint8 typeCt = typeOf(uint256(inputHandle));
        require(uint8(inputType) == typeCt, "Wrong type");
        result = inputVerifier.verifyCiphertext(contextUserInputs, inputHandle, inputProof);
        acl.allowTransient(result, msg.sender);
    }

    function cast(uint256 ct, bytes1 toType) public virtual returns (uint256 result) {
        require(acl.isAllowed(ct, msg.sender), "Sender doesn't own ct on cast");
        uint256 supportedTypesInput = (1 << 0) +
            (1 << 1) +
            (1 << 2) +
            (1 << 3) +
            (1 << 4) +
            (1 << 5) +
            (1 << 6) +
            (1 << 8);
        requireType(ct, supportedTypesInput);
        uint256 supportedTypesOutput = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8); // @note: unsupported casting to ebool (use fheNe instead)
        require((1 << uint8(toType)) & supportedTypesOutput > 0, "Unsupported output type");
        uint8 typeCt = typeOf(ct);
        require(bytes1(typeCt) != toType, "Cannot cast to same type");
        fheGasLimit.payForCast(typeCt);
        result = uint256(keccak256(abi.encodePacked(Operators.cast, ct, toType, acl, block.chainid)));
        result = appendType(result, uint8(toType));
        acl.allowTransient(result, msg.sender);
    }

    function trivialEncrypt(uint256 pt, bytes1 toType) public virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 0) +
            (1 << 1) +
            (1 << 2) +
            (1 << 3) +
            (1 << 4) +
            (1 << 5) +
            (1 << 6) +
            (1 << 7) +
            (1 << 8);
        uint8 toT = uint8(toType);
        require((1 << toT) & supportedTypes > 0, "Unsupported type");
        fheGasLimit.payForTrivialEncrypt(toT);
        result = uint256(keccak256(abi.encodePacked(Operators.trivialEncrypt, pt, toType, acl, block.chainid)));
        result = appendType(result, toT);
        acl.allowTransient(result, msg.sender);
    }

    function trivialEncrypt(bytes memory pt, bytes1 toType) public virtual returns (uint256 result) {
        // @note: overloaded function for ebytesXX types
        uint256 supportedTypes = (1 << 9) + (1 << 10) + (1 << 11);
        uint8 toT = uint8(toType);
        require((1 << toT) & supportedTypes > 0, "Unsupported type");
        fheGasLimit.payForTrivialEncrypt(toT);
        uint256 lenBytesPT = pt.length;
        if (toT == 9) {
            require(lenBytesPT == 64, "Bytes array length of Bytes64 should be 64");
        } else if (toT == 10) {
            require(lenBytesPT == 128, "Bytes array length of Bytes128 should be 128");
        } else {
            // @note: i.e toT == 11 thanks to the pre-condition above
            require(lenBytesPT == 256, "Bytes array length of Bytes256 should be 256");
        }
        result = uint256(keccak256(abi.encodePacked(Operators.trivialEncrypt, pt, toType, acl, block.chainid)));
        result = appendType(result, toT);
        acl.allowTransient(result, msg.sender);
    }

    function fheIfThenElse(uint256 control, uint256 ifTrue, uint256 ifFalse) public virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 0) +
            (1 << 1) +
            (1 << 2) +
            (1 << 3) +
            (1 << 4) +
            (1 << 5) +
            (1 << 6) +
            (1 << 7) +
            (1 << 8) +
            (1 << 9) +
            (1 << 10) +
            (1 << 11);
        requireType(ifTrue, supportedTypes);
        uint8 typeCt = typeOf(ifTrue);
        fheGasLimit.payForIfThenElse(typeCt);
        result = ternaryOp(Operators.fheIfThenElse, control, ifTrue, ifFalse);
    }

    function fheRand(bytes1 randType) public virtual returns (uint256 result) {
        bytes16 seed = generateSeed();
        result = generateRand(randType, seed);
    }

    function fheRandBounded(uint256 upperBound, bytes1 randType) public virtual returns (uint256 result) {
        bytes16 seed = generateSeed();
        result = generateRandBounded(upperBound, randType, seed);
    }

    /// @notice Getter for the name and version of the contract
    /// @return string representing the name and the version of the contract
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
}
