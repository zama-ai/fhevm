// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

enum FheType {
    Bool,
    Uint4,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Uint128,
    Uint160,
    Uint256,
    Uint512,
    Uint1024,
    Uint2048,
    Uint2,
    Uint6,
    Uint10,
    Uint12,
    Uint14,
    Int2,
    Int4,
    Int6,
    Int8,
    Int10,
    Int12,
    Int14,
    Int16,
    Int32,
    Int64,
    Int128,
    Int160,
    Int256,
    AsciiString,
    Int512,
    Int1024,
    Int2048,
    Uint24,
    Uint40,
    Uint48,
    Uint56,
    Uint72,
    Uint80,
    Uint88,
    Uint96,
    Uint104,
    Uint112,
    Uint120,
    Uint136,
    Uint144,
    Uint152,
    Uint168,
    Uint176,
    Uint184,
    Uint192,
    Uint200,
    Uint208,
    Uint216,
    Uint224,
    Uint232,
    Uint240,
    Uint248,
    Int24,
    Int40,
    Int48,
    Int56,
    Int72,
    Int80,
    Int88,
    Int96,
    Int104,
    Int112,
    Int120,
    Int136,
    Int144,
    Int152,
    Int168,
    Int176,
    Int184,
    Int192,
    Int200,
    Int208,
    Int216,
    Int224,
    Int232,
    Int240,
    Int248
}

contract FHEVMExecutorTest {
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
        bytes1 inputType,
        bytes32 result
    );
    event Cast(address indexed caller, bytes32 ct, FheType toType, bytes32 result);
    event TrivialEncrypt(address indexed caller, uint256 pt, FheType toType, bytes32 result);
    event TrivialEncryptBytes(address indexed caller, bytes pt, FheType toType, bytes32 result);
    event FheIfThenElse(address indexed caller, bytes32 control, bytes32 ifTrue, bytes32 ifFalse, bytes32 result);
    event FheRand(address indexed caller, bytes1 randType, bytes16 seed, bytes32 result);
    event FheRandBounded(address indexed caller, uint256 upperBound, bytes1 randType, bytes16 seed, bytes32 result);

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
    function fheRand(bytes1 randType) public {
        bytes16 seed = bytes16(keccak256(abi.encodePacked(block.timestamp)));
        bytes32 result = bytes32(keccak256(abi.encodePacked("fheRand", randType, seed)));
        emit FheRand(msg.sender, randType, seed, result);
    }
    function fheRandBounded(uint256 upperBound, bytes1 randType) public {
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
        bytes1 inputType
    ) public {
        bytes32 result = bytes32(keccak256(abi.encodePacked("verifyCiphertext", inputHandle, userAddress, inputProof, inputType)));
        emit VerifyCiphertext(msg.sender, inputHandle, userAddress, inputProof, inputType, result);
    }
}
