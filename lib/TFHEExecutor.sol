// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "./ACL.sol";
import "./FHEPayment.sol";
import "./FhevmLib.sol";
import "./ACLAddress.sol";
import "./FHEPaymentAddress.sol";
import "./FhevmLib.sol";
import "@openzeppelin/contracts/utils/Strings.sol";

address constant EXT_TFHE_LIBRARY = address(0x000000000000000000000000000000000000005d);

contract TFHEExecutor {
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

    uint256 public counterRand = 0; // counter used for computing handles of randomness operators

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

    function isPowerOfTwo(uint256 x) internal pure returns (bool) {
        return (x > 0) && ((x & (x - 1)) == 0);
    }

    /// @dev handle format for user inputs is: keccak256(keccak256(CiphertextFHEList)||index_handle)[0:29] || index_handle || handle_type || handle_version
    /// @dev other handles format (fhe ops results) is: keccak256(keccak256(rawCiphertextFHEList)||index_handle)[0:30] || handle_type || handle_version
    /// @dev the CiphertextFHEList actually contains: 1 byte (= N) for size of handles_list, N bytes for the handles_types : 1 per handle, then the original fhe160list raw ciphertext
    function typeOf(uint256 handle) internal pure returns (uint8) {
        uint8 typeCt = uint8(handle >> 8);
        return typeCt;
    }

    function appendType(uint256 prehandle, uint8 handleType) internal pure returns (uint256 result) {
        result = prehandle & 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000;
        result = result | (uint256(handleType) << 8); // append type
        result = result | HANDLE_VERSION;
    }

    function requireType(uint256 handle, uint256 supportedTypes) internal pure {
        uint8 typeCt = typeOf(handle);
        require((1 << typeCt) & supportedTypes > 0, "Unsupported type");
    }

    function unaryOp(Operators op, uint256 ct) internal returns (uint256 result) {
        require(acl.isAllowed(ct, msg.sender), "Sender doesn't own ct on op");
        result = uint256(keccak256(abi.encodePacked(op, ct)));
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
    ) internal returns (uint256 result) {
        require(acl.isAllowed(lhs, msg.sender), "Sender doesn't own lhs on op");
        if (scalar == 0x00) {
            require(acl.isAllowed(rhs, msg.sender), "Sender doesn't own rhs on op");
            uint8 typeRhs = typeOf(rhs);
            uint8 typeLhs = typeOf(lhs);
            require(typeLhs == typeRhs, "Incompatible types for lhs and rhs");
        }
        result = uint256(keccak256(abi.encodePacked(op, lhs, rhs, scalar)));
        result = appendType(result, resultType);
        acl.allowTransient(result, msg.sender);
    }

    function ternaryOp(Operators op, uint256 lhs, uint256 middle, uint256 rhs) internal returns (uint256 result) {
        require(acl.isAllowed(lhs, msg.sender), "Sender doesn't own lhs on op");
        require(acl.isAllowed(middle, msg.sender), "Sender doesn't own middle on op");
        require(acl.isAllowed(rhs, msg.sender), "Sender doesn't own rhs on op");
        uint8 typeLhs = typeOf(lhs);
        uint8 typeMiddle = typeOf(middle);
        uint8 typeRhs = typeOf(rhs);
        require(typeLhs == 0, "Unsupported type for lhs"); // lhs must be ebool
        require(typeMiddle == typeRhs, "Incompatible types for middle and rhs");
        result = uint256(keccak256(abi.encodePacked(op, lhs, middle, rhs)));
        result = appendType(result, typeMiddle);
        acl.allowTransient(result, msg.sender);
    }

    function fheAdd(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheAdd(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheAdd, lhs, rhs, scalar, lhsType);
    }

    function fheSub(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheSub(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheSub, lhs, rhs, scalar, lhsType);
    }

    function fheMul(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheMul(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheMul, lhs, rhs, scalar, lhsType);
    }

    function fheDiv(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        require(scalarByte & 0x01 == 0x01, "Only fheDiv by a scalar is supported");
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheDiv(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheDiv, lhs, rhs, scalar, lhsType);
    }

    function fheRem(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        require(scalarByte & 0x01 == 0x01, "Only fheRem by a scalar is supported");
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheRem(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheRem, lhs, rhs, scalar, lhsType);
    }

    function fheBitAnd(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        require(scalarByte & 0x01 == 0x00, "Only fheBitAnd by a ciphertext is supported");
        uint256 supportedTypes = (1 << 0) + (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheBitAnd(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheBitAnd, lhs, rhs, scalar, lhsType);
    }

    function fheBitOr(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        require(scalarByte & 0x01 == 0x00, "Only fheBitOr by a ciphertext is supported");
        uint256 supportedTypes = (1 << 0) + (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheBitOr(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheBitOr, lhs, rhs, scalar, lhsType);
    }

    function fheBitXor(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        require(scalarByte & 0x01 == 0x00, "Only fheBitXor by a ciphertext is supported");
        uint256 supportedTypes = (1 << 0) + (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheBitXor(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheBitXor, lhs, rhs, scalar, lhsType);
    }

    function fheShl(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheShl(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheShl, lhs, rhs, scalar, lhsType);
    }

    function fheShr(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheShr(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheShr, lhs, rhs, scalar, lhsType);
    }

    function fheRotl(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheRotl(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheRotl, lhs, rhs, scalar, lhsType);
    }

    function fheRotr(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheRotr(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheRotr, lhs, rhs, scalar, lhsType);
    }

    function fheEq(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 7) + (1 << 11);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheEq(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheEq, lhs, rhs, scalar, 0);
    }

    function fheNe(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 7) + (1 << 11);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheNe(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheNe, lhs, rhs, scalar, 0);
    }

    function fheGe(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheGe(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheGe, lhs, rhs, scalar, 0);
    }

    function fheGt(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheGt(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheGt, lhs, rhs, scalar, 0);
    }

    function fheLe(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheLe(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheLe, lhs, rhs, scalar, 0);
    }

    function fheLt(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheLt(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheLt, lhs, rhs, scalar, 0);
    }

    function fheMin(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheMin(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheMin, lhs, rhs, scalar, lhsType);
    }

    function fheMax(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5);
        requireType(lhs, supportedTypes);
        uint8 lhsType = typeOf(lhs);
        bytes1 scalar = scalarByte & 0x01;
        fhePayment.payForFheMax(msg.sender, lhsType, scalar);
        result = binaryOp(Operators.fheMax, lhs, rhs, scalar, lhsType);
    }

    function fheNeg(uint256 ct) external returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5);
        requireType(ct, supportedTypes);
        uint8 typeCt = typeOf(ct);
        fhePayment.payForFheNeg(msg.sender, typeCt);
        result = unaryOp(Operators.fheNeg, ct);
    }

    function fheNot(uint256 ct) external returns (uint256 result) {
        uint256 supportedTypes = (1 << 0) + (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5);
        requireType(ct, supportedTypes);
        uint8 typeCt = typeOf(ct);
        fhePayment.payForFheNot(msg.sender, typeCt);
        result = unaryOp(Operators.fheNot, ct);
    }

    function verifyCiphertext(
        bytes32 inputHandle,
        address callerAddress,
        bytes memory inputProof,
        bytes1 inputType
    ) external returns (uint256 result) {
        result = FhevmLib(address(EXT_TFHE_LIBRARY)).verifyCiphertext(
            inputHandle,
            callerAddress,
            msg.sender,
            inputProof,
            inputType
        );
        acl.allowTransient(result, msg.sender);
    }

    function cast(uint256 ct, bytes1 toType) external returns (uint256 result) {
        require(acl.isAllowed(ct, msg.sender), "Sender doesn't own ct on cast");
        uint256 supportedTypesInput = (1 << 0) + (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5);
        requireType(ct, supportedTypesInput);
        uint256 supportedTypesOutput = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5); // @note: unsupported casting to ebool (use fheNe instead)
        require((1 << uint8(toType)) & supportedTypesOutput > 0, "Unsupported output type");
        uint8 typeCt = typeOf(ct);
        require(bytes1(typeCt) != toType, "Cannot cast to same type");
        fhePayment.payForCast(msg.sender, typeCt);
        result = uint256(keccak256(abi.encodePacked(Operators.cast, ct, toType)));
        result = appendType(result, uint8(toType));
        acl.allowTransient(result, msg.sender);
    }

    function trivialEncrypt(uint256 pt, bytes1 toType) external returns (uint256 result) {
        uint256 supportedTypes = (1 << 0) + (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 7);
        uint8 toT = uint8(toType);
        require((1 << toT) & supportedTypes > 0, "Unsupported type");
        fhePayment.payForTrivialEncrypt(msg.sender, toT);
        result = uint256(keccak256(abi.encodePacked(Operators.trivialEncrypt, pt, toType)));
        result = appendType(result, toT);
        acl.allowTransient(result, msg.sender);
    }

    function fheIfThenElse(uint256 control, uint256 ifTrue, uint256 ifFalse) external returns (uint256 result) {
        uint256 supportedTypes = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5) + (1 << 7);
        requireType(ifTrue, supportedTypes);
        uint8 typeCt = typeOf(ifTrue);
        fhePayment.payForIfThenElse(msg.sender, typeCt);
        result = ternaryOp(Operators.fheIfThenElse, control, ifTrue, ifFalse);
    }

    function fheRand(bytes1 randType) external returns (uint256 result) {
        uint256 supportedTypes = (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5);
        uint8 randT = uint8(randType);
        require((1 << randT) & supportedTypes > 0, "Unsupported erandom type");
        fhePayment.payForFheRand(msg.sender, randT);
        result = uint256(keccak256(abi.encodePacked(Operators.fheRand, randType, counterRand)));
        result = appendType(result, randT);
        acl.allowTransient(result, msg.sender);
        counterRand++;
    }

    function fheRandBounded(uint256 upperBound, bytes1 randType) external returns (uint256 result) {
        uint256 supportedTypes = (1 << 2) + (1 << 3) + (1 << 4) + (1 << 5);
        uint8 randT = uint8(randType);
        require((1 << randT) & supportedTypes > 0, "Unsupported erandom type");
        require(isPowerOfTwo(upperBound), "UpperBound must be a power of 2");
        fhePayment.payForFheRandBounded(msg.sender, randT);
        result = uint256(keccak256(abi.encodePacked(Operators.fheRandBounded, upperBound, randType, counterRand)));
        result = appendType(result, randT);
        acl.allowTransient(result, msg.sender);
        counterRand++;
    }

    /// @notice Getter for the name and version of the contract
    /// @return string representing the name and the version of the contract
    function getVersion() external pure returns (string memory) {
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
