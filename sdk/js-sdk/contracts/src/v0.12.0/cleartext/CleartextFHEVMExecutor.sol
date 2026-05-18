// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.27;

import {aclAdd, hcuLimitAdd, inputVerifierAdd} from "../host-contracts/addresses/FHEVMHostAddresses.sol";
import {FHEVMExecutor, IInputVerifier, HCULimit} from "../host-contracts/contracts/FHEVMExecutor.sol";
import {FheType} from "../host-contracts/contracts/shared/FheType.sol";
import {CleartextACL} from "./CleartextACL.sol";
import {CleartextArithmetic} from "./CleartextArithmetic.sol";
import {FheTypeBitWidth} from "./FheTypeBitWidth.sol";

//import {InputProofHelper} from "../InputProofHelper.sol";

/// @notice FHEVMExecutor variant that mirrors every operation result into a `plaintexts` mapping.
/// @dev Each override calls `super` (symbolic handle flow) then stores the cleartext.
contract CleartextFHEVMExecutor is FHEVMExecutor {
    /// @notice CleartextACL.
    CleartextACL private constant acl = CleartextACL(aclAdd);
    HCULimit private constant hcuLimit = HCULimit(hcuLimitAdd);
    IInputVerifier private constant inputVerifier = IInputVerifier(inputVerifierAdd);

    error UnsupportedCleartextBinaryOp(Operators op);
    error UnsupportedCleartextUnaryOp(Operators op);
    error UnsupportedCleartextTernaryOp(Operators op);
    error CleartextNotFoundInProof(bytes32 inputHandle);
    error CleartextVerificationMismatch(uint256 index, bytes32 expected, bytes32 extracted);
    error CleartextVerificationLengthMismatch(uint256 clearValuesLength, uint256 numHandles);
    error MalformedInputProof();

    event PersistNewCleartextInput(
        address indexed caller, address contractAddress, bytes32 inputHandle, FheType inputType, uint256 cleartext
    );

    /// @dev Handle to cleartext value mapping for local testing.
    //mapping(bytes32 => uint256) public plaintexts;
    function plaintexts(bytes32 result) public view returns (uint256) {
        return acl.plaintext(result);
    }

    function cast(bytes32 ct, FheType toType) public override returns (bytes32 result) {
        (bool allowed, uint256 cleartext) = acl.isAllowedWithCleartext(ct, msg.sender);
        if (!allowed) revert ACLNotAllowed(ct, msg.sender);

        // result = super.cast(ct, toType);
        // plaintexts[result] = CleartextArithmetic.fheCast(plaintexts[ct], uint8(toType));
        //if (!acl.isAllowed(ct, msg.sender)) revert ACLNotAllowed(ct, msg.sender);

        uint256 supportedTypesInput = (1 << uint8(FheType.Bool)) + (1 << uint8(FheType.Uint8))
            + (1 << uint8(FheType.Uint16)) + (1 << uint8(FheType.Uint32)) + (1 << uint8(FheType.Uint64))
            + (1 << uint8(FheType.Uint128)) + (1 << uint8(FheType.Uint256));
        FheType typeCt = _verifyAndReturnType(ct, supportedTypesInput);
        uint256 supportedTypesOutput = (1 << uint8(FheType.Uint8)) + (1 << uint8(FheType.Uint16))
            + (1 << uint8(FheType.Uint32)) + (1 << uint8(FheType.Uint64)) + (1 << uint8(FheType.Uint128))
            + (1 << uint8(FheType.Uint256)); // @note: unsupported casting to ebool (use fheNe instead)
        if ((1 << uint8(toType)) & supportedTypesOutput == 0) revert UnsupportedType();

        /// @dev It must not cast to same type.
        if (typeCt == toType) revert InvalidType();
        result = keccak256(abi.encodePacked(Operators.cast, ct, toType, acl, block.chainid));
        result = _appendMetadataToPrehandle(result, toType);
        hcuLimit.checkHCUForCast(toType, ct, result, msg.sender);

        //acl.allowTransient(result, msg.sender);
        acl.allowTransientWithCleartext(result, msg.sender, CleartextArithmetic.fheCast(cleartext, toType));

        emit Cast(msg.sender, ct, toType, result);
    }

    function trivialEncrypt(uint256 pt, FheType toType) public override returns (bytes32 result) {
        // result = super.trivialEncrypt(pt, toType);
        // plaintexts[result] = CleartextArithmetic.normalizePlaintextToType(pt, uint8(toType));
        uint256 supportedTypes = (1 << uint8(FheType.Bool)) + (1 << uint8(FheType.Uint8)) + (1 << uint8(FheType.Uint16))
            + (1 << uint8(FheType.Uint32)) + (1 << uint8(FheType.Uint64)) + (1 << uint8(FheType.Uint128))
            + (1 << uint8(FheType.Uint160)) + (1 << uint8(FheType.Uint256));

        if ((1 << uint8(toType)) & supportedTypes == 0) revert UnsupportedType();
        result = keccak256(abi.encodePacked(Operators.trivialEncrypt, pt, toType, acl, block.chainid));
        result = _appendMetadataToPrehandle(result, toType);
        hcuLimit.checkHCUForTrivialEncrypt(toType, result, msg.sender);
        //acl.allowTransient(result, msg.sender);
        acl.allowTransientWithCleartext(result, msg.sender, CleartextArithmetic.normalizePlaintextToType(pt, toType));

        emit TrivialEncrypt(msg.sender, pt, toType, result);
    }

    /// @notice Verifies input and extracts cleartext from the proof's extra-data suffix.
    function verifyInput(bytes32 inputHandle, address userAddress, bytes memory inputProof, FheType inputType)
        public
        override
        returns (bytes32 result)
    {
        (bool foundCleartext, uint256 cleartext) = _tryReadCleartextFromProof(inputHandle, inputProof);
        // result = super.verifyInput(inputHandle, userAddress, inputProof, inputType);
        // (bool foundCleartext, uint256 cleartext) = _tryReadCleartextFromProof(inputHandle, inputProof);
        // if (foundCleartext) {
        //     plaintexts[result] = CleartextArithmetic.normalizePlaintextToType(cleartext, uint8(inputType));
        // }
        ContextUserInputs memory contextUserInputs =
            ContextUserInputs({userAddress: userAddress, contractAddress: msg.sender});
        FheType typeCt = _typeOf(inputHandle);
        if (inputType != typeCt) revert InvalidType();
        result = inputVerifier.verifyInput(contextUserInputs, inputHandle, inputProof);
        //acl.allowTransient(result, msg.sender);
        if (foundCleartext) {
            acl.allowTransientWithCleartext(
                result, msg.sender, CleartextArithmetic.normalizePlaintextToType(cleartext, inputType)
            );
        } else {
            acl.allowTransient(result, msg.sender);
        }
        emit VerifyInput(msg.sender, inputHandle, userAddress, inputProof, inputType, result);
    }

    function _generateRand(FheType randType, bytes16 seed) internal override returns (bytes32 result) {
        // result = super._generateRand(randType, seed);
        // plaintexts[result] = CleartextArithmetic.rand(seed, FheTypeBitWidth.bitWidthForType(uint8(randType)));
        uint256 supportedTypes = (1 << uint8(FheType.Bool)) + (1 << uint8(FheType.Uint8)) + (1 << uint8(FheType.Uint16))
            + (1 << uint8(FheType.Uint32)) + (1 << uint8(FheType.Uint64)) + (1 << uint8(FheType.Uint128))
            + (1 << uint8(FheType.Uint256));

        /// @dev Unsupported erandom type.
        if ((1 << uint8(randType)) & supportedTypes == 0) revert UnsupportedType();
        result = keccak256(abi.encodePacked(Operators.fheRand, randType, seed));
        result = _appendMetadataToPrehandle(result, randType);
        hcuLimit.checkHCUForFheRand(randType, result, msg.sender);

        acl.allowTransientWithCleartext(
            result, msg.sender, CleartextArithmetic.rand(seed, FheTypeBitWidth.bitWidthForType(uint8(randType)))
        );
        //acl.allowTransient(result, msg.sender);
    }

    function _generateRandBounded(uint256 upperBound, FheType randType, bytes16 seed)
        internal
        override
        returns (bytes32 result)
    {
        // result = super._generateRandBounded(upperBound, randType, seed);
        // plaintexts[result] = CleartextArithmetic.randBounded(seed, upperBound);
        uint256 supportedTypes = (1 << uint8(FheType.Uint8)) + (1 << uint8(FheType.Uint16))
            + (1 << uint8(FheType.Uint32)) + (1 << uint8(FheType.Uint64)) + (1 << uint8(FheType.Uint128))
            + (1 << uint8(FheType.Uint256));
        /// @dev Unsupported erandom type.
        if ((1 << uint8(randType)) & supportedTypes == 0) revert UnsupportedType();
        if (!_isPowerOfTwo(upperBound)) revert NotPowerOfTwo();
        _checkBelowMaxBound(upperBound, randType);
        result = keccak256(abi.encodePacked(Operators.fheRandBounded, upperBound, randType, seed));
        result = _appendMetadataToPrehandle(result, randType);
        hcuLimit.checkHCUForFheRandBounded(randType, result, msg.sender);
        acl.allowTransientWithCleartext(result, msg.sender, CleartextArithmetic.randBounded(seed, upperBound));
        //acl.allowTransient(result, msg.sender);
    }

    function _binaryOp(Operators op, bytes32 lhs, bytes32 rhs, bytes1 scalar, FheType resultType)
        internal
        override
        returns (bytes32 result)
    {
        /// @dev at the moment at most only right operand of binary ops can be scalar, so we enforce `scalar` to be bool
        _checkBoolean(scalar);

        (bool lhsAllowed, uint256 lhsValue) = acl.isAllowedWithCleartext(lhs, msg.sender);
        if (!lhsAllowed) revert ACLNotAllowed(lhs, msg.sender);

        FheType lhsType = _typeOf(lhs);
        uint256 rhsValue;

        //if (!acl.isAllowed(lhs, msg.sender)) revert ACLNotAllowed(lhs, msg.sender);
        if (scalar == 0x00) {
            (bool rhsAllowed, uint256 rhsCleartext) = acl.isAllowedWithCleartext(rhs, msg.sender);
            if (!rhsAllowed) revert ACLNotAllowed(rhs, msg.sender);
            //if (!acl.isAllowed(rhs, msg.sender)) revert ACLNotAllowed(rhs, msg.sender);

            FheType rhsType = _typeOf(rhs);
            if (lhsType != rhsType) revert IncompatibleTypes();

            rhsValue = rhsCleartext;
        } else {
            rhsValue = uint256(rhs);
        }

        result = keccak256(abi.encodePacked(op, lhs, rhs, scalar, acl, block.chainid));
        result = _appendMetadataToPrehandle(result, resultType);
        //acl.allowTransient(result, msg.sender);

        acl.allowTransientWithCleartext(
            result, msg.sender, CleartextArithmetic.computeBinaryResult(uint8(op), lhsType, scalar, lhsValue, rhsValue)
        );

        // uint256 lhsValue = plaintexts[lhs];
        // uint256 rhsValue = _rhsValue(rhs, scalarByte);

        // result = super._binaryOp(op, lhs, rhs, scalarByte, resultType);
        // plaintexts[result] = _computeBinaryResult(op, lhs, rhs, scalarByte);
    }

    function _unaryOp(Operators op, bytes32 ct) internal override returns (bytes32 result) {
        (bool allowed, uint256 cleartext) = acl.isAllowedWithCleartext(ct, msg.sender);
        if (!allowed) revert ACLNotAllowed(ct, msg.sender);

        result = keccak256(abi.encodePacked(op, ct, acl, block.chainid));
        FheType typeCt = _typeOf(ct);
        result = _appendMetadataToPrehandle(result, typeCt);

        acl.allowTransientWithCleartext(
            result,
            msg.sender,
            CleartextArithmetic.computeUnaryResult(CleartextArithmetic.Operators(uint8(op)), typeCt, cleartext)
        );
    }

    // function _unaryOp(Operators op, bytes32 ct) internal override returns (bytes32 result) {
    //     result = super._unaryOp(op, ct);
    //     plaintexts[result] = _computeUnaryResult(op, ct);
    // }

    function _ternaryOp(Operators op, bytes32 lhs, bytes32 middle, bytes32 rhs)
        internal
        override
        returns (bytes32 result)
    {
        (bool lhsAllowed, uint256 lhsValue) = acl.isAllowedWithCleartext(lhs, msg.sender);
        if (!lhsAllowed) revert ACLNotAllowed(lhs, msg.sender);

        (bool middleAllowed, uint256 middleValue) = acl.isAllowedWithCleartext(middle, msg.sender);
        if (!middleAllowed) revert ACLNotAllowed(middle, msg.sender);

        (bool rhsAllowed, uint256 rhsValue) = acl.isAllowedWithCleartext(rhs, msg.sender);
        if (!rhsAllowed) revert ACLNotAllowed(rhs, msg.sender);

        // if (!acl.isAllowed(lhs, msg.sender)) revert ACLNotAllowed(lhs, msg.sender);
        // if (!acl.isAllowed(middle, msg.sender)) revert ACLNotAllowed(middle, msg.sender);
        // if (!acl.isAllowed(rhs, msg.sender)) revert ACLNotAllowed(rhs, msg.sender);

        FheType lhsType = _typeOf(lhs);
        FheType middleType = _typeOf(middle);
        FheType rhsType = _typeOf(rhs);

        /// @dev lhs must be ebool
        if (lhsType != FheType.Bool) revert UnsupportedType();
        if (middleType != rhsType) revert IncompatibleTypes();

        result = keccak256(abi.encodePacked(op, lhs, middle, rhs, acl, block.chainid));
        result = _appendMetadataToPrehandle(result, middleType);

        if (op != Operators.fheIfThenElse) {
            revert UnsupportedCleartextTernaryOp(op);
        }

        // stack too deep
        uint256 cleartext = CleartextArithmetic.computeTernaryResult(uint8(op), lhsValue, middleValue, rhsValue);

        //acl.allowTransient(result, msg.sender);
        acl.allowTransientWithCleartext(result, msg.sender, cleartext);

        // result = super._ternaryOp(op, lhs, middle, rhs);

        // if (op == Operators.fheIfThenElse) {
        //     plaintexts[result] = CleartextArithmetic.fheIfThenElse(plaintexts[lhs], plaintexts[middle], plaintexts[rhs]);
        // }
    }

    // function _verifyCleartexts(
    //     uint256[] memory clearValues,
    //     bytes32 salt,
    //     address userAddress,
    //     address contractAddress,
    //     bytes memory inputProof
    // ) internal view returns (bytes32[] memory extractedHandles) {
    //     if (inputProof.length < 2) revert MalformedInputProof();
    //     uint256 numHandles = uint256(uint8(inputProof[0]));

    //     if (clearValues.length != numHandles) {
    //         revert CleartextVerificationLengthMismatch(clearValues.length, numHandles);
    //     }
    //     if (inputProof.length < 2 + numHandles * 32) revert MalformedInputProof();

    //     extractedHandles = new bytes32[](numHandles);
    //     for (uint256 i = 0; i < numHandles; i++) {
    //         bytes32 extracted;
    //         assembly {
    //             extracted := mload(add(inputProof, add(34, mul(i, 32))))
    //         }
    //         extractedHandles[i] = extracted;

    //         bytes memory mockCiphertext = InputProofHelper.computeMockCiphertext(
    //             contractAddress, userAddress, address(acl), uint64(block.chainid), uint8(i), clearValues[i], salt
    //         );

    //         bytes32 expected = InputProofHelper.computeInputHandle(
    //             mockCiphertext, uint8(i), _typeOf(extracted), address(acl), uint64(block.chainid)
    //         );
    //         if (expected != extracted) {
    //             revert CleartextVerificationMismatch(i, expected, extracted);
    //         }
    //     }
    // }

    // function persistNewInput(
    //     uint256[] memory clearValues,
    //     bytes32 salt,
    //     address contractAddress,
    //     bytes memory inputProof
    // ) public virtual returns (bytes32[] memory results) {
    //     bytes32[] memory inputHandles = _verifyCleartexts(clearValues, salt, msg.sender, contractAddress, inputProof);

    //     uint256 n = inputHandles.length;
    //     if (n == 0) revert MalformedInputProof();

    //     results = new bytes32[](n);
    //     ContextUserInputs memory ctx = ContextUserInputs({userAddress: msg.sender, contractAddress: contractAddress});

    //     // One full verification — the proof cache short-circuits the rest.
    //     for (uint256 i = 0; i < n; ++i) {
    //         bytes32 h = inputHandles[i];
    //         FheType t = _typeOf(h);
    //         results[i] = inputVerifier.verifyInput(ctx, h, inputProof);
    //         acl.registerInputCleartext(h, CleartextArithmetic.normalizePlaintextToType(clearValues[i], t));
    //         emit PersistNewCleartextInput(msg.sender, contractAddress, results[i], t, clearValues[i]);
    //     }
    // }

    // function _computeBinaryResult(
    //     Operators op,
    //     uint8 fheType,
    //     //bytes32 rhs,
    //     bytes1 scalarByte,
    //     uint256 lhsValue,
    //     uint256 rhsValue
    // ) private pure returns (uint256) {
    //     // uint256 lhsValue = plaintexts[lhs];
    //     // uint256 rhsValue = _rhsValue(rhs, scalarByte);
    //     // uint8 fheType = uint8(_typeOf(lhs));

    //     if (op == Operators.fheAdd) return CleartextArithmetic.fheAdd(lhsValue, rhsValue, fheType, scalarByte);
    //     if (op == Operators.fheSub) return CleartextArithmetic.fheSub(lhsValue, rhsValue, fheType, scalarByte);
    //     if (op == Operators.fheMul) return CleartextArithmetic.fheMul(lhsValue, rhsValue, fheType, scalarByte);
    //     if (op == Operators.fheDiv) return CleartextArithmetic.fheDiv(lhsValue, rhsValue, fheType, scalarByte);
    //     if (op == Operators.fheRem) return CleartextArithmetic.fheRem(lhsValue, rhsValue, fheType, scalarByte);
    //     if (op == Operators.fheBitAnd) return CleartextArithmetic.fheBitAnd(lhsValue, rhsValue, fheType, scalarByte);
    //     if (op == Operators.fheBitOr) return CleartextArithmetic.fheBitOr(lhsValue, rhsValue, fheType, scalarByte);
    //     if (op == Operators.fheBitXor) return CleartextArithmetic.fheBitXor(lhsValue, rhsValue, fheType, scalarByte);
    //     if (op == Operators.fheShl) return CleartextArithmetic.fheShl(lhsValue, rhsValue, fheType, scalarByte);
    //     if (op == Operators.fheShr) return CleartextArithmetic.fheShr(lhsValue, rhsValue, fheType, scalarByte);
    //     if (op == Operators.fheRotl) return CleartextArithmetic.fheRotl(lhsValue, rhsValue, fheType, scalarByte);
    //     if (op == Operators.fheRotr) return CleartextArithmetic.fheRotr(lhsValue, rhsValue, fheType, scalarByte);
    //     if (op == Operators.fheEq) return CleartextArithmetic.fheEq(lhsValue, rhsValue, fheType, scalarByte);
    //     if (op == Operators.fheNe) return CleartextArithmetic.fheNe(lhsValue, rhsValue, fheType, scalarByte);
    //     if (op == Operators.fheGe) return CleartextArithmetic.fheGe(lhsValue, rhsValue, fheType, scalarByte);
    //     if (op == Operators.fheGt) return CleartextArithmetic.fheGt(lhsValue, rhsValue, fheType, scalarByte);
    //     if (op == Operators.fheLe) return CleartextArithmetic.fheLe(lhsValue, rhsValue, fheType, scalarByte);
    //     if (op == Operators.fheLt) return CleartextArithmetic.fheLt(lhsValue, rhsValue, fheType, scalarByte);
    //     if (op == Operators.fheMin) return CleartextArithmetic.fheMin(lhsValue, rhsValue, fheType, scalarByte);
    //     if (op == Operators.fheMax) return CleartextArithmetic.fheMax(lhsValue, rhsValue, fheType, scalarByte);

    //     revert UnsupportedCleartextBinaryOp(op);
    // }

    // function _computeUnaryResult(Operators op, bytes32 ct, uint256 value) private pure returns (uint256) {
    //     uint8 fheType = uint8(_typeOf(ct));

    //     if (op == Operators.fheNeg) return CleartextArithmetic.fheNeg(value, fheType);
    //     if (op == Operators.fheNot) return CleartextArithmetic.fheNot(value, fheType);

    //     revert UnsupportedCleartextUnaryOp(op);
    // }

    function _tryReadCleartextFromProof(bytes32 inputHandle, bytes memory inputProof)
        private
        pure
        returns (bool foundCleartext, uint256 cleartext)
    {
        if (inputProof.length < 2) {
            return (false, 0);
        }

        uint8 numHandles = uint8(inputProof[0]);
        uint8 numSigners = uint8(inputProof[1]);
        uint256 cleartextStart = 2 + uint256(numHandles) * 32 + uint256(numSigners) * 65;

        if (inputProof.length < cleartextStart + 32) {
            return (false, 0);
        }

        for (uint8 i = 0; i < numHandles; i++) {
            uint256 handleOffset = 2 + uint256(i) * 32;
            bytes32 handleInProof;
            assembly {
                handleInProof := mload(add(add(inputProof, 32), handleOffset))
            }

            if (handleInProof != inputHandle) {
                continue;
            }

            uint256 cleartextOffset = cleartextStart + uint256(i) * 32;
            if (inputProof.length < cleartextOffset + 32) {
                return (false, 0);
            }

            assembly {
                cleartext := mload(add(add(inputProof, 32), cleartextOffset))
            }
            return (true, cleartext);
        }

        return (false, 0);
    }
}
