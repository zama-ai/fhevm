// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "./ACL.sol";
import "./FHEPayment.sol";
import "./ACLAddress.sol";
import "./FHEPaymentAddress.sol";
import "./InputVerifierAddress.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import "@openzeppelin/contracts-upgradeable/utils/cryptography/EIP712Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";

interface IInputVerifier {
    function verifyCiphertext(
        TFHEExecutor.ContextUserInputs memory context,
        bytes32 inputHandle,
        bytes memory inputProof
    ) external returns (uint256);
}

contract TFHEExecutor is UUPSUpgradeable, Ownable2StepUpgradeable {
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
    event FheRand(bytes1 randType, uint256 result);
    event FheRandBounded(uint256 upperBound, bytes1 randType, uint256 result);

    /// @notice Handle version
    uint8 public constant HANDLE_VERSION = 0;

    /// @notice Name of the contract
    string private constant CONTRACT_NAME = "TFHEExecutor";

    /// @notice Version of the contract
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    ACL private constant acl = ACL(aclAdd);
    FHEPayment private constant fhePayment = FHEPayment(fhePaymentAdd);
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

    /// @notice Getter function for the FHEPayment contract address
    function getFHEPaymentAddress() public view virtual returns (address) {
        return address(fhePayment);
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
    function initialize(address initialOwner) external initializer {
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

    function fheAdd(uint256 lhs, uint256 rhs, bytes1 scalarByte) external virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheAdd(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheAdd, lhs, rhs, scalar, lhsType);
        emit FheAdd(lhs, rhs, scalarByte, result);
    }

    function fheSub(uint256 lhs, uint256 rhs, bytes1 scalarByte) external virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheSub(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheSub, lhs, rhs, scalar, lhsType);
        emit FheSub(lhs, rhs, scalarByte, result);
    }

    function fheMul(uint256 lhs, uint256 rhs, bytes1 scalarByte) external virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheMul(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheMul, lhs, rhs, scalar, lhsType);
        emit FheMul(lhs, rhs, scalarByte, result);
    }

    function fheDiv(uint256 lhs, uint256 rhs, bytes1 scalarByte) external virtual returns (uint256 result) {
        require(scalarByte & 0x01 == 0x01, "Only fheDiv by a scalar is supported");
        require(rhs != 0, "Could not divide by 0");
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheDiv(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheDiv, lhs, rhs, scalar, lhsType);
        emit FheDiv(lhs, rhs, scalarByte, result);
    }

    function fheRem(uint256 lhs, uint256 rhs, bytes1 scalarByte) external virtual returns (uint256 result) {
        require(scalarByte & 0x01 == 0x01, "Only fheRem by a scalar is supported");
        require(rhs != 0, "Could not divide by 0");
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheRem(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheRem, lhs, rhs, scalar, lhsType);
        emit FheRem(lhs, rhs, scalarByte, result);
    }

    function fheBitAnd(uint256 lhs, uint256 rhs, bytes1 scalarByte) external virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 0) + (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheBitAnd(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheBitAnd, lhs, rhs, scalar, lhsType);
        emit FheBitAnd(lhs, rhs, scalarByte, result);
    }

    function fheBitOr(uint256 lhs, uint256 rhs, bytes1 scalarByte) external virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 0) + (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheBitOr(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheBitOr, lhs, rhs, scalar, lhsType);
        emit FheBitOr(lhs, rhs, scalarByte, result);
    }

    function fheBitXor(uint256 lhs, uint256 rhs, bytes1 scalarByte) external virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 0) + (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheBitXor(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheBitXor, lhs, rhs, scalar, lhsType);
        emit FheBitXor(lhs, rhs, scalarByte, result);
    }

    function fheShl(uint256 lhs, uint256 rhs, bytes1 scalarByte) external virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheShl(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheShl, lhs, rhs, scalar, lhsType);
        emit FheShl(lhs, rhs, scalarByte, result);
    }

    function fheShr(uint256 lhs, uint256 rhs, bytes1 scalarByte) external virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheShr(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheShr, lhs, rhs, scalar, lhsType);
        emit FheShr(lhs, rhs, scalarByte, result);
    }

    function fheRotl(uint256 lhs, uint256 rhs, bytes1 scalarByte) external virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheRotl(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheRotl, lhs, rhs, scalar, lhsType);
        emit FheRotl(lhs, rhs, scalarByte, result);
    }

    function fheRotr(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheRotr(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheRotr, lhs, rhs, scalar, lhsType);
        emit FheRotr(lhs, rhs, scalarByte, result);
    }

    function fheEq(uint256 lhs, uint256 rhs, bytes1 scalarByte) external virtual returns (uint256 result) {
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
        fhePayment.payForFheEq(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheEq, lhs, rhs, scalar, 0);
        emit FheEq(lhs, rhs, scalarByte, result);
    }

    function fheEq(uint256 lhs, bytes memory rhs, bytes1 scalarByte) external virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 9) + (1 << 10) + (1 << 11);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        require(scalar == 0x01, "Overloaded fheEq is only for scalar ebytesXXX second operand");
        fhePayment.payForFheEq(msg.sender, lhsType, scalar);
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
        emit FheEqBytes(lhs, rhs, scalarByte, result);
    }

    function fheNe(uint256 lhs, uint256 rhs, bytes1 scalarByte) external virtual returns (uint256 result) {
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
        fhePayment.payForFheNe(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheNe, lhs, rhs, scalar, 0);
        emit FheNe(lhs, rhs, scalarByte, result);
    }

    function fheNe(uint256 lhs, bytes memory rhs, bytes1 scalarByte) external virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 9) + (1 << 10) + (1 << 11);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        require(scalar == 0x01, "Overloaded fheNe is only for scalar ebytesXXX second operand");
        fhePayment.payForFheNe(msg.sender, lhsType, scalar);
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
        emit FheNeBytes(lhs, rhs, scalarByte, result);
    }

    function fheGe(uint256 lhs, uint256 rhs, bytes1 scalarByte) external virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheGe(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheGe, lhs, rhs, scalar, 0);
        emit FheGe(lhs, rhs, scalarByte, result);
    }

    function fheGt(uint256 lhs, uint256 rhs, bytes1 scalarByte) external virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheGt(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheGt, lhs, rhs, scalar, 0);
        emit FheGt(lhs, rhs, scalarByte, result);
    }

    function fheLe(uint256 lhs, uint256 rhs, bytes1 scalarByte) external virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheLe(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheLe, lhs, rhs, scalar, 0);
        emit FheLe(lhs, rhs, scalarByte, result);
    }

    function fheLt(uint256 lhs, uint256 rhs, bytes1 scalarByte) external virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheLt(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheLt, lhs, rhs, scalar, 0);
        emit FheLt(lhs, rhs, scalarByte, result);
    }

    function fheMin(uint256 lhs, uint256 rhs, bytes1 scalarByte) external virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheMin(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheMin, lhs, rhs, scalar, lhsType);
        emit FheMin(lhs, rhs, scalarByte, result);
    }

    function fheMax(uint256 lhs, uint256 rhs, bytes1 scalarByte) external virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheMax(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheMax, lhs, rhs, scalar, lhsType);
        emit FheMax(lhs, rhs, scalarByte, result);
    }

    function fheNeg(uint256 ct) external virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(ct, supportedTypes);
        uint8 typeCt = typeOf(ct);
        fhePayment.payForFheNeg(msg.sender, typeCt);
        result = unaryOp(Operators.fheNeg, ct);
        emit FheNeg(ct, result);
    }

    function fheNot(uint256 ct) external virtual returns (uint256 result) {
        uint256 supportedTypes = (1 << 0) + (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        requireType(ct, supportedTypes);
        uint8 typeCt = typeOf(ct);
        fhePayment.payForFheNot(msg.sender, typeCt);
        result = unaryOp(Operators.fheNot, ct);
        emit FheNot(ct, result);
    }

    function verifyCiphertext(
        bytes32 inputHandle,
        address userAddress,
        bytes memory inputProof,
        bytes1 inputType
    ) external virtual returns (uint256 result) {
        ContextUserInputs memory contextUserInputs = ContextUserInputs({
            aclAddress: address(acl),
            userAddress: userAddress,
            contractAddress: msg.sender
        });
        uint8 typeCt = typeOf(uint256(inputHandle));
        require(uint8(inputType) == typeCt, "Wrong type");
        result = inputVerifier.verifyCiphertext(contextUserInputs, inputHandle, inputProof);
        acl.allowTransient(result, msg.sender);
        emit VerifyCiphertext(inputHandle, userAddress, inputProof, inputType, result);
    }

    function cast(uint256 ct, bytes1 toType) external virtual returns (uint256 result) {
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
        fhePayment.payForCast(msg.sender, typeCt);
        result = uint256(keccak256(abi.encodePacked(Operators.cast, ct, toType, acl, block.chainid)));
        result = appendType(result, uint8(toType));
        acl.allowTransient(result, msg.sender);
        emit Cast(ct, toType, result);
    }

    function trivialEncrypt(uint256 pt, bytes1 toType) external virtual returns (uint256 result) {
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
        fhePayment.payForTrivialEncrypt(msg.sender, toT);
        result = uint256(keccak256(abi.encodePacked(Operators.trivialEncrypt, pt, toType, acl, block.chainid)));
        result = appendType(result, toT);
        acl.allowTransient(result, msg.sender);
        emit TrivialEncrypt(pt, toType, result);
    }

    function trivialEncrypt(bytes memory pt, bytes1 toType) external virtual returns (uint256 result) {
        // @note: overloaded function for ebytesXX types
        uint256 supportedTypes = (1 << 9) + (1 << 10) + (1 << 11);
        uint8 toT = uint8(toType);
        require((1 << toT) & supportedTypes > 0, "Unsupported type");
        fhePayment.payForTrivialEncrypt(msg.sender, toT);
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
        emit TrivialEncryptBytes(pt, toType, result);
    }

    function fheIfThenElse(uint256 control, uint256 ifTrue, uint256 ifFalse) external virtual returns (uint256 result) {
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
        fhePayment.payForIfThenElse(msg.sender, typeCt);
        result = ternaryOp(Operators.fheIfThenElse, control, ifTrue, ifFalse);
        emit FheIfThenElse(control, ifTrue, ifFalse, result);
    }

    function fheRand(bytes1 randType) external virtual returns (uint256 result) {
        TFHEExecutorStorage storage $ = _getTFHEExecutorStorage();
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
        fhePayment.payForFheRand(msg.sender, randT);
        bytes16 seed = bytes16(
            keccak256(abi.encodePacked($.counterRand, acl, block.chainid, blockhash(block.number - 1), block.timestamp))
        );
        result = uint256(keccak256(abi.encodePacked(Operators.fheRand, randType, seed)));
        result = appendType(result, randT);
        acl.allowTransient(result, msg.sender);
        $.counterRand++;
        emit FheRand(randType, result);
    }

    function fheRandBounded(uint256 upperBound, bytes1 randType) external virtual returns (uint256 result) {
        TFHEExecutorStorage storage $ = _getTFHEExecutorStorage();
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6) + (1 << 8);
        uint8 randT = uint8(randType);
        require((1 << randT) & supportedTypes > 0, "Unsupported erandom type");
        require(isPowerOfTwo(upperBound), "UpperBound must be a power of 2");
        fhePayment.payForFheRandBounded(msg.sender, randT);
        bytes16 seed = bytes16(
            keccak256(abi.encodePacked($.counterRand, acl, block.chainid, blockhash(block.number - 1), block.timestamp))
        );
        result = uint256(keccak256(abi.encodePacked(Operators.fheRandBounded, upperBound, randType, seed)));
        result = appendType(result, randT);
        acl.allowTransient(result, msg.sender);
        $.counterRand++;
        emit FheRandBounded(upperBound, randType, result);
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
