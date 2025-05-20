// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../../lib/FHE.sol";
import "../../lib/FHEVMConfig.sol";

contract FHEVMTestSuite7 {
    ebool public resEbool;
    euint8 public resEuint8;
    euint16 public resEuint16;
    euint32 public resEuint32;
    euint64 public resEuint64;
    euint128 public resEuint128;
    euint256 public resEuint256;
    ebytes64 public resEbytes64;
    ebytes128 public resEbytes128;
    ebytes256 public resEbytes256;

    constructor() {
        FHE.setCoprocessor(FHEVMConfig.defaultConfig());
    }

    function eq_uint128_euint128(uint128 a, externalEuint128 b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.eq(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function ne_euint128_uint128(externalEuint128 a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        uint128 bProc = b;
        ebool result = FHE.ne(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function ne_uint128_euint128(uint128 a, externalEuint128 b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ne(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function ge_euint128_uint128(externalEuint128 a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        uint128 bProc = b;
        ebool result = FHE.ge(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function ge_uint128_euint128(uint128 a, externalEuint128 b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ge(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function gt_euint128_uint128(externalEuint128 a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        uint128 bProc = b;
        ebool result = FHE.gt(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function gt_uint128_euint128(uint128 a, externalEuint128 b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.gt(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function le_euint128_uint128(externalEuint128 a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        uint128 bProc = b;
        ebool result = FHE.le(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function le_uint128_euint128(uint128 a, externalEuint128 b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.le(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function lt_euint128_uint128(externalEuint128 a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        uint128 bProc = b;
        ebool result = FHE.lt(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function lt_uint128_euint128(uint128 a, externalEuint128 b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.lt(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function min_euint128_uint128(externalEuint128 a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        uint128 bProc = b;
        euint128 result = FHE.min(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function min_uint128_euint128(uint128 a, externalEuint128 b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.min(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function max_euint128_uint128(externalEuint128 a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        uint128 bProc = b;
        euint128 result = FHE.max(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function max_uint128_euint128(uint128 a, externalEuint128 b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.max(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function and_euint256_uint256(externalEuint256 a, uint256 b, bytes calldata inputProof) public {
        euint256 aProc = FHE.fromExternal(a, inputProof);
        uint256 bProc = b;
        euint256 result = FHE.and(aProc, bProc);
        FHE.allowThis(result);
        resEuint256 = result;
    }
    function and_uint256_euint256(uint256 a, externalEuint256 b, bytes calldata inputProof) public {
        uint256 aProc = a;
        euint256 bProc = FHE.fromExternal(b, inputProof);
        euint256 result = FHE.and(aProc, bProc);
        FHE.allowThis(result);
        resEuint256 = result;
    }
    function or_euint256_uint256(externalEuint256 a, uint256 b, bytes calldata inputProof) public {
        euint256 aProc = FHE.fromExternal(a, inputProof);
        uint256 bProc = b;
        euint256 result = FHE.or(aProc, bProc);
        FHE.allowThis(result);
        resEuint256 = result;
    }
    function or_uint256_euint256(uint256 a, externalEuint256 b, bytes calldata inputProof) public {
        uint256 aProc = a;
        euint256 bProc = FHE.fromExternal(b, inputProof);
        euint256 result = FHE.or(aProc, bProc);
        FHE.allowThis(result);
        resEuint256 = result;
    }
    function xor_euint256_uint256(externalEuint256 a, uint256 b, bytes calldata inputProof) public {
        euint256 aProc = FHE.fromExternal(a, inputProof);
        uint256 bProc = b;
        euint256 result = FHE.xor(aProc, bProc);
        FHE.allowThis(result);
        resEuint256 = result;
    }
    function xor_uint256_euint256(uint256 a, externalEuint256 b, bytes calldata inputProof) public {
        uint256 aProc = a;
        euint256 bProc = FHE.fromExternal(b, inputProof);
        euint256 result = FHE.xor(aProc, bProc);
        FHE.allowThis(result);
        resEuint256 = result;
    }
    function eq_euint256_uint256(externalEuint256 a, uint256 b, bytes calldata inputProof) public {
        euint256 aProc = FHE.fromExternal(a, inputProof);
        uint256 bProc = b;
        ebool result = FHE.eq(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function eq_uint256_euint256(uint256 a, externalEuint256 b, bytes calldata inputProof) public {
        uint256 aProc = a;
        euint256 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.eq(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function ne_euint256_uint256(externalEuint256 a, uint256 b, bytes calldata inputProof) public {
        euint256 aProc = FHE.fromExternal(a, inputProof);
        uint256 bProc = b;
        ebool result = FHE.ne(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function ne_uint256_euint256(uint256 a, externalEuint256 b, bytes calldata inputProof) public {
        uint256 aProc = a;
        euint256 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ne(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function shl_euint8_euint8(externalEuint8 a, externalEuint8 b, bytes calldata inputProof) public {
        euint8 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint8 result = FHE.shl(aProc, bProc);
        FHE.allowThis(result);
        resEuint8 = result;
    }
    function shl_euint8_uint8(externalEuint8 a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint8 result = FHE.shl(aProc, bProc);
        FHE.allowThis(result);
        resEuint8 = result;
    }
    function shr_euint8_euint8(externalEuint8 a, externalEuint8 b, bytes calldata inputProof) public {
        euint8 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint8 result = FHE.shr(aProc, bProc);
        FHE.allowThis(result);
        resEuint8 = result;
    }
    function shr_euint8_uint8(externalEuint8 a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint8 result = FHE.shr(aProc, bProc);
        FHE.allowThis(result);
        resEuint8 = result;
    }
    function rotl_euint8_euint8(externalEuint8 a, externalEuint8 b, bytes calldata inputProof) public {
        euint8 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint8 result = FHE.rotl(aProc, bProc);
        FHE.allowThis(result);
        resEuint8 = result;
    }
    function rotl_euint8_uint8(externalEuint8 a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint8 result = FHE.rotl(aProc, bProc);
        FHE.allowThis(result);
        resEuint8 = result;
    }
    function rotr_euint8_euint8(externalEuint8 a, externalEuint8 b, bytes calldata inputProof) public {
        euint8 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint8 result = FHE.rotr(aProc, bProc);
        FHE.allowThis(result);
        resEuint8 = result;
    }
    function rotr_euint8_uint8(externalEuint8 a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint8 result = FHE.rotr(aProc, bProc);
        FHE.allowThis(result);
        resEuint8 = result;
    }
    function shl_euint16_euint8(externalEuint16 a, externalEuint8 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint16 result = FHE.shl(aProc, bProc);
        FHE.allowThis(result);
        resEuint16 = result;
    }
    function shl_euint16_uint8(externalEuint16 a, uint8 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint16 result = FHE.shl(aProc, bProc);
        FHE.allowThis(result);
        resEuint16 = result;
    }
    function shr_euint16_euint8(externalEuint16 a, externalEuint8 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint16 result = FHE.shr(aProc, bProc);
        FHE.allowThis(result);
        resEuint16 = result;
    }
    function shr_euint16_uint8(externalEuint16 a, uint8 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint16 result = FHE.shr(aProc, bProc);
        FHE.allowThis(result);
        resEuint16 = result;
    }
    function rotl_euint16_euint8(externalEuint16 a, externalEuint8 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint16 result = FHE.rotl(aProc, bProc);
        FHE.allowThis(result);
        resEuint16 = result;
    }
    function rotl_euint16_uint8(externalEuint16 a, uint8 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint16 result = FHE.rotl(aProc, bProc);
        FHE.allowThis(result);
        resEuint16 = result;
    }
    function rotr_euint16_euint8(externalEuint16 a, externalEuint8 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint16 result = FHE.rotr(aProc, bProc);
        FHE.allowThis(result);
        resEuint16 = result;
    }
    function rotr_euint16_uint8(externalEuint16 a, uint8 b, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint16 result = FHE.rotr(aProc, bProc);
        FHE.allowThis(result);
        resEuint16 = result;
    }
    function shl_euint32_euint8(externalEuint32 a, externalEuint8 b, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint32 result = FHE.shl(aProc, bProc);
        FHE.allowThis(result);
        resEuint32 = result;
    }
    function shl_euint32_uint8(externalEuint32 a, uint8 b, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint32 result = FHE.shl(aProc, bProc);
        FHE.allowThis(result);
        resEuint32 = result;
    }
    function shr_euint32_euint8(externalEuint32 a, externalEuint8 b, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint32 result = FHE.shr(aProc, bProc);
        FHE.allowThis(result);
        resEuint32 = result;
    }
    function shr_euint32_uint8(externalEuint32 a, uint8 b, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint32 result = FHE.shr(aProc, bProc);
        FHE.allowThis(result);
        resEuint32 = result;
    }
    function rotl_euint32_euint8(externalEuint32 a, externalEuint8 b, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint32 result = FHE.rotl(aProc, bProc);
        FHE.allowThis(result);
        resEuint32 = result;
    }
    function rotl_euint32_uint8(externalEuint32 a, uint8 b, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint32 result = FHE.rotl(aProc, bProc);
        FHE.allowThis(result);
        resEuint32 = result;
    }
    function rotr_euint32_euint8(externalEuint32 a, externalEuint8 b, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint32 result = FHE.rotr(aProc, bProc);
        FHE.allowThis(result);
        resEuint32 = result;
    }
    function rotr_euint32_uint8(externalEuint32 a, uint8 b, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint32 result = FHE.rotr(aProc, bProc);
        FHE.allowThis(result);
        resEuint32 = result;
    }
    function shl_euint64_euint8(externalEuint64 a, externalEuint8 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint64 result = FHE.shl(aProc, bProc);
        FHE.allowThis(result);
        resEuint64 = result;
    }
    function shl_euint64_uint8(externalEuint64 a, uint8 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint64 result = FHE.shl(aProc, bProc);
        FHE.allowThis(result);
        resEuint64 = result;
    }
    function shr_euint64_euint8(externalEuint64 a, externalEuint8 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint64 result = FHE.shr(aProc, bProc);
        FHE.allowThis(result);
        resEuint64 = result;
    }
    function shr_euint64_uint8(externalEuint64 a, uint8 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint64 result = FHE.shr(aProc, bProc);
        FHE.allowThis(result);
        resEuint64 = result;
    }
    function rotl_euint64_euint8(externalEuint64 a, externalEuint8 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint64 result = FHE.rotl(aProc, bProc);
        FHE.allowThis(result);
        resEuint64 = result;
    }
    function rotl_euint64_uint8(externalEuint64 a, uint8 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint64 result = FHE.rotl(aProc, bProc);
        FHE.allowThis(result);
        resEuint64 = result;
    }
    function rotr_euint64_euint8(externalEuint64 a, externalEuint8 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint64 result = FHE.rotr(aProc, bProc);
        FHE.allowThis(result);
        resEuint64 = result;
    }
    function rotr_euint64_uint8(externalEuint64 a, uint8 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint64 result = FHE.rotr(aProc, bProc);
        FHE.allowThis(result);
        resEuint64 = result;
    }
    function shl_euint128_euint8(externalEuint128 a, externalEuint8 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.shl(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function shl_euint128_uint8(externalEuint128 a, uint8 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint128 result = FHE.shl(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function shr_euint128_euint8(externalEuint128 a, externalEuint8 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.shr(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function shr_euint128_uint8(externalEuint128 a, uint8 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint128 result = FHE.shr(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function rotl_euint128_euint8(externalEuint128 a, externalEuint8 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.rotl(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function rotl_euint128_uint8(externalEuint128 a, uint8 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint128 result = FHE.rotl(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function rotr_euint128_euint8(externalEuint128 a, externalEuint8 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.rotr(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function rotr_euint128_uint8(externalEuint128 a, uint8 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint128 result = FHE.rotr(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function shl_euint256_euint8(externalEuint256 a, externalEuint8 b, bytes calldata inputProof) public {
        euint256 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint256 result = FHE.shl(aProc, bProc);
        FHE.allowThis(result);
        resEuint256 = result;
    }
    function shl_euint256_uint8(externalEuint256 a, uint8 b, bytes calldata inputProof) public {
        euint256 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint256 result = FHE.shl(aProc, bProc);
        FHE.allowThis(result);
        resEuint256 = result;
    }
    function shr_euint256_euint8(externalEuint256 a, externalEuint8 b, bytes calldata inputProof) public {
        euint256 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint256 result = FHE.shr(aProc, bProc);
        FHE.allowThis(result);
        resEuint256 = result;
    }
    function shr_euint256_uint8(externalEuint256 a, uint8 b, bytes calldata inputProof) public {
        euint256 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint256 result = FHE.shr(aProc, bProc);
        FHE.allowThis(result);
        resEuint256 = result;
    }
    function rotl_euint256_euint8(externalEuint256 a, externalEuint8 b, bytes calldata inputProof) public {
        euint256 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint256 result = FHE.rotl(aProc, bProc);
        FHE.allowThis(result);
        resEuint256 = result;
    }
    function rotl_euint256_uint8(externalEuint256 a, uint8 b, bytes calldata inputProof) public {
        euint256 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint256 result = FHE.rotl(aProc, bProc);
        FHE.allowThis(result);
        resEuint256 = result;
    }
    function rotr_euint256_euint8(externalEuint256 a, externalEuint8 b, bytes calldata inputProof) public {
        euint256 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint256 result = FHE.rotr(aProc, bProc);
        FHE.allowThis(result);
        resEuint256 = result;
    }
    function rotr_euint256_uint8(externalEuint256 a, uint8 b, bytes calldata inputProof) public {
        euint256 aProc = FHE.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint256 result = FHE.rotr(aProc, bProc);
        FHE.allowThis(result);
        resEuint256 = result;
    }
    function neg_euint8(externalEuint8 a, bytes calldata inputProof) public {
        euint8 aProc = FHE.fromExternal(a, inputProof);
        euint8 result = FHE.neg(aProc);
        FHE.allowThis(result);
        resEuint8 = result;
    }
    function not_euint8(externalEuint8 a, bytes calldata inputProof) public {
        euint8 aProc = FHE.fromExternal(a, inputProof);
        euint8 result = FHE.not(aProc);
        FHE.allowThis(result);
        resEuint8 = result;
    }
    function neg_euint16(externalEuint16 a, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        euint16 result = FHE.neg(aProc);
        FHE.allowThis(result);
        resEuint16 = result;
    }
    function not_euint16(externalEuint16 a, bytes calldata inputProof) public {
        euint16 aProc = FHE.fromExternal(a, inputProof);
        euint16 result = FHE.not(aProc);
        FHE.allowThis(result);
        resEuint16 = result;
    }
    function neg_euint32(externalEuint32 a, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        euint32 result = FHE.neg(aProc);
        FHE.allowThis(result);
        resEuint32 = result;
    }
    function not_euint32(externalEuint32 a, bytes calldata inputProof) public {
        euint32 aProc = FHE.fromExternal(a, inputProof);
        euint32 result = FHE.not(aProc);
        FHE.allowThis(result);
        resEuint32 = result;
    }
    function neg_euint64(externalEuint64 a, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint64 result = FHE.neg(aProc);
        FHE.allowThis(result);
        resEuint64 = result;
    }
    function not_euint64(externalEuint64 a, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint64 result = FHE.not(aProc);
        FHE.allowThis(result);
        resEuint64 = result;
    }
    function neg_euint128(externalEuint128 a, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint128 result = FHE.neg(aProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function not_euint128(externalEuint128 a, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint128 result = FHE.not(aProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function neg_euint256(externalEuint256 a, bytes calldata inputProof) public {
        euint256 aProc = FHE.fromExternal(a, inputProof);
        euint256 result = FHE.neg(aProc);
        FHE.allowThis(result);
        resEuint256 = result;
    }
    function not_euint256(externalEuint256 a, bytes calldata inputProof) public {
        euint256 aProc = FHE.fromExternal(a, inputProof);
        euint256 result = FHE.not(aProc);
        FHE.allowThis(result);
        resEuint256 = result;
    }
}
