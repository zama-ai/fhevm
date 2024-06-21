// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.25;

interface FhevmLib {
    function fheAdd(uint256 lhs, uint256 rhs, bytes1 scalarByte) external pure returns (uint256 result);

    function fheSub(uint256 lhs, uint256 rhs, bytes1 scalarByte) external pure returns (uint256 result);

    function fheMul(uint256 lhs, uint256 rhs, bytes1 scalarByte) external pure returns (uint256 result);

    function fheDiv(uint256 lhs, uint256 rhs, bytes1 scalarByte) external pure returns (uint256 result);

    function fheRem(uint256 lhs, uint256 rhs, bytes1 scalarByte) external pure returns (uint256 result);

    function fheBitAnd(uint256 lhs, uint256 rhs, bytes1 scalarByte) external pure returns (uint256 result);

    function fheBitOr(uint256 lhs, uint256 rhs, bytes1 scalarByte) external pure returns (uint256 result);

    function fheBitXor(uint256 lhs, uint256 rhs, bytes1 scalarByte) external pure returns (uint256 result);

    function fheShl(uint256 lhs, uint256 rhs, bytes1 scalarByte) external pure returns (uint256 result);

    function fheShr(uint256 lhs, uint256 rhs, bytes1 scalarByte) external pure returns (uint256 result);

    function fheRotl(uint256 lhs, uint256 rhs, bytes1 scalarByte) external pure returns (uint256 result);

    function fheRotr(uint256 lhs, uint256 rhs, bytes1 scalarByte) external pure returns (uint256 result);

    function fheEq(uint256 lhs, uint256 rhs, bytes1 scalarByte) external pure returns (uint256 result);

    function fheNe(uint256 lhs, uint256 rhs, bytes1 scalarByte) external pure returns (uint256 result);

    function fheGe(uint256 lhs, uint256 rhs, bytes1 scalarByte) external pure returns (uint256 result);

    function fheGt(uint256 lhs, uint256 rhs, bytes1 scalarByte) external pure returns (uint256 result);

    function fheLe(uint256 lhs, uint256 rhs, bytes1 scalarByte) external pure returns (uint256 result);

    function fheLt(uint256 lhs, uint256 rhs, bytes1 scalarByte) external pure returns (uint256 result);

    function fheMin(uint256 lhs, uint256 rhs, bytes1 scalarByte) external pure returns (uint256 result);

    function fheMax(uint256 lhs, uint256 rhs, bytes1 scalarByte) external pure returns (uint256 result);

    function fheNeg(uint256 ct) external pure returns (uint256 result);

    function fheNot(uint256 ct) external pure returns (uint256 result);

    function fhePubKey(bytes1 fromLib) external view returns (bytes memory result);

    function verifyCiphertext(
        bytes32 inputHandle,
        address callerAddress,
        address contractAddress,
        bytes memory inputProof,
        bytes1 inputType
    ) external pure returns (uint256 result);

    function cast(uint256 ct, bytes1 toType) external pure returns (uint256 result);

    function trivialEncrypt(uint256 ct, bytes1 toType) external pure returns (uint256 result);

    function fheIfThenElse(uint256 control, uint256 ifTrue, uint256 ifFalse) external pure returns (uint256 result);

    function fheArrayEq(uint256[] memory lhs, uint256[] memory rhs) external pure returns (uint256 result);

    function fheRand(bytes1 randType, uint256 seed) external view returns (uint256 result);

    function fheRandBounded(uint256 upperBound, bytes1 randType, uint256 seed) external view returns (uint256 result);
}
