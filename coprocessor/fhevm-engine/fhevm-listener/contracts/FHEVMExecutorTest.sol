// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "contracts/FheType.sol";
import "contracts/FHEEvents.sol";

contract FHEVMExecutorTest is FHEEvents {
    function fheAdd(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public {
        bytes32 result = bytes32(keccak256(abi.encodePacked("fheAdd", lhs, rhs, scalarByte)));
        emit FheAdd(msg.sender, lhs, rhs, scalarByte, result);
    }
    function fheSub(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public {
        bytes32 result = bytes32(keccak256(abi.encodePacked("fheSub", lhs, rhs, scalarByte)));
        emit FheSub(msg.sender, lhs, rhs, scalarByte, result);
    }
    function fheMul(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public {
        bytes32 result = bytes32(keccak256(abi.encodePacked("fheMul", lhs, rhs, scalarByte)));
        emit FheMul(msg.sender, lhs, rhs, scalarByte, result);
    }
    function fheDiv(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public {
        bytes32 result = bytes32(keccak256(abi.encodePacked("fheDiv", lhs, rhs, scalarByte)));
        emit FheDiv(msg.sender, lhs, rhs, scalarByte, result);
    }
    function fheRem(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public {
        bytes32 result = bytes32(keccak256(abi.encodePacked("fheRem", lhs, rhs, scalarByte)));
        emit FheRem(msg.sender, lhs, rhs, scalarByte, result);
    }
    function fheBitAnd(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public {
        bytes32 result = bytes32(keccak256(abi.encodePacked("fheBitAnd", lhs, rhs, scalarByte)));
        emit FheBitAnd(msg.sender, lhs, rhs, scalarByte, result);
    }
    function fheBitOr(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public {
        bytes32 result = bytes32(keccak256(abi.encodePacked("fheBitOr", lhs, rhs, scalarByte)));
        emit FheBitOr(msg.sender, lhs, rhs, scalarByte, result);
    }
    function fheBitXor(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public {
        bytes32 result = bytes32(keccak256(abi.encodePacked("fheBitXor", lhs, rhs, scalarByte)));
        emit FheBitXor(msg.sender, lhs, rhs, scalarByte, result);
    }
    function fheShl(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public {
        bytes32 result = bytes32(keccak256(abi.encodePacked("fheShl", lhs, rhs, scalarByte)));
        emit FheShl(msg.sender, lhs, rhs, scalarByte, result);
    }
    function fheShr(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public {
        bytes32 result = bytes32(keccak256(abi.encodePacked("fheShr", lhs, rhs, scalarByte)));
        emit FheShr(msg.sender, lhs, rhs, scalarByte, result);
    }
    function fheRotl(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public {
        bytes32 result = bytes32(keccak256(abi.encodePacked("fheRotl", lhs, rhs, scalarByte)));
        emit FheRotl(msg.sender, lhs, rhs, scalarByte, result);
    }
    function fheRotr(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public {
        bytes32 result = bytes32(keccak256(abi.encodePacked("fheRotr", lhs, rhs, scalarByte)));
        emit FheRotr(msg.sender, lhs, rhs, scalarByte, result);
    }
    function fheEq(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public {
        bytes32 result = bytes32(keccak256(abi.encodePacked("fheEq", lhs, rhs, scalarByte)));
        emit FheEq(msg.sender, lhs, rhs, scalarByte, result);
    }
    function fheEq(bytes32 lhs, bytes memory rhs, bytes1 scalarByte) public {
        bytes32 result = bytes32(keccak256(abi.encodePacked("fheEqBytes", lhs, rhs, scalarByte)));
        emit FheEqBytes(msg.sender, lhs, rhs, scalarByte, result);
    }
    function fheNe(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public {
        bytes32 result = bytes32(keccak256(abi.encodePacked("fheNe", lhs, rhs, scalarByte)));
        emit FheNe(msg.sender, lhs, rhs, scalarByte, result);
    }
    function fheNe(bytes32 lhs, bytes memory rhs, bytes1 scalarByte) public {
        bytes32 result = bytes32(keccak256(abi.encodePacked("fheNeBytes", lhs, rhs, scalarByte)));
        emit FheNeBytes(msg.sender, lhs, rhs, scalarByte, result);
    }
    function fheGe(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public {
        bytes32 result = bytes32(keccak256(abi.encodePacked("fheGe", lhs, rhs, scalarByte)));
        emit FheGe(msg.sender, lhs, rhs, scalarByte, result);
    }
    function fheGt(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public {
        bytes32 result = bytes32(keccak256(abi.encodePacked("fheGt", lhs, rhs, scalarByte)));
        emit FheGt(msg.sender, lhs, rhs, scalarByte, result);
    }
    function fheLe(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public {
        bytes32 result = bytes32(keccak256(abi.encodePacked("fheLe", lhs, rhs, scalarByte)));
        emit FheLe(msg.sender, lhs, rhs, scalarByte, result);
    }
    function fheLt(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public {
        bytes32 result = bytes32(keccak256(abi.encodePacked("fheLt", lhs, rhs, scalarByte)));
        emit FheLt(msg.sender, lhs, rhs, scalarByte, result);
    }
    function fheMin(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public {
        bytes32 result = bytes32(keccak256(abi.encodePacked("fheMin", lhs, rhs, scalarByte)));
        emit FheMin(msg.sender, lhs, rhs, scalarByte, result);
    }
    function fheMax(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) public {
        bytes32 result = bytes32(keccak256(abi.encodePacked("fheMax", lhs, rhs, scalarByte)));
        emit FheMax(msg.sender, lhs, rhs, scalarByte, result);
    }
    function fheNeg(bytes32 ct) public {
        bytes32 result = bytes32(keccak256(abi.encodePacked("fheNeg", ct)));
        emit FheNeg(msg.sender, ct, result);
    }
    function fheNot(bytes32 ct) public {
        bytes32 result = bytes32(keccak256(abi.encodePacked("fheNot", ct)));
        emit FheNot(msg.sender, ct, result);
    }
    function fheIfThenElse(bytes32 control, bytes32 ifTrue, bytes32 ifFalse) public {
        bytes32 result = bytes32(keccak256(abi.encodePacked("fheIfThenElse", control, ifTrue, ifFalse)));
        emit FheIfThenElse(msg.sender, control, ifTrue, ifFalse, result);
    }
    function fheRand(FheType randType) public {
        bytes16 seed = bytes16(keccak256(abi.encodePacked(block.timestamp)));
        bytes32 result = bytes32(keccak256(abi.encodePacked("fheRand", randType, seed)));
        emit FheRand(msg.sender, randType, seed, result);
    }
    function fheRandBounded(uint256 upperBound, FheType randType) public {
        bytes16 seed = bytes16(keccak256(abi.encodePacked(block.timestamp)));
        bytes32 result = bytes32(keccak256(abi.encodePacked("fheRandBounded", upperBound, randType, seed)));
        emit FheRandBounded(msg.sender, upperBound, randType, seed, result);
    }
    function cast(bytes32 ct, FheType toType) public {
        bytes32 result = bytes32(keccak256(abi.encodePacked("cast", ct, toType)));
        emit Cast(msg.sender, ct, toType, result);
    }

    function trivialEncrypt(uint256 pt, FheType toType) public {
        bytes32 result = bytes32(keccak256(abi.encodePacked("trivialEncrypt", pt, toType)));
        emit TrivialEncrypt(msg.sender, pt, toType, result);
    }

    function trivialEncrypt(bytes memory pt, FheType toType) public {
        bytes32 result = bytes32(keccak256(abi.encodePacked("trivialEncryptBytes", pt, toType)));
        emit TrivialEncryptBytes(msg.sender, pt, toType, result);
    }

    function verifyCiphertext(
        bytes32 inputHandle,
        address userAddress,
        bytes memory inputProof,
        FheType  inputType
    ) public {
        bytes32 result = bytes32(keccak256(abi.encodePacked("verifyCiphertext", inputHandle, userAddress, inputProof, inputType)));
        emit VerifyCiphertext(msg.sender, inputHandle, userAddress, inputProof, inputType, result);
    }
}
