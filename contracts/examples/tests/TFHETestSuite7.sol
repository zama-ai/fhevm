// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../../lib/TFHE.sol";
import "../../lib/FHEVMConfig.sol";

contract TFHETestSuite7 {
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
        TFHE.setFHEVM(FHEVMConfig.defaultConfig());
    }

    function eq_uint128_euint128(uint128 a, einput b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function ne_euint128_uint128(einput a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        uint128 bProc = b;
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function ne_uint128_euint128(uint128 a, einput b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function ge_euint128_uint128(einput a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        uint128 bProc = b;
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function ge_uint128_euint128(uint128 a, einput b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function gt_euint128_uint128(einput a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        uint128 bProc = b;
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function gt_uint128_euint128(uint128 a, einput b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function le_euint128_uint128(einput a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        uint128 bProc = b;
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function le_uint128_euint128(uint128 a, einput b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function lt_euint128_uint128(einput a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        uint128 bProc = b;
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function lt_uint128_euint128(uint128 a, einput b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function min_euint128_uint128(einput a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        uint128 bProc = b;
        euint128 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        resEuint128 = result;
    }
    function min_uint128_euint128(uint128 a, einput b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        resEuint128 = result;
    }
    function max_euint128_uint128(einput a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        uint128 bProc = b;
        euint128 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        resEuint128 = result;
    }
    function max_uint128_euint128(uint128 a, einput b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        resEuint128 = result;
    }
    function and_euint256_uint256(einput a, uint256 b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        uint256 bProc = b;
        euint256 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        resEuint256 = result;
    }
    function and_uint256_euint256(uint256 a, einput b, bytes calldata inputProof) public {
        uint256 aProc = a;
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        resEuint256 = result;
    }
    function or_euint256_uint256(einput a, uint256 b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        uint256 bProc = b;
        euint256 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        resEuint256 = result;
    }
    function or_uint256_euint256(uint256 a, einput b, bytes calldata inputProof) public {
        uint256 aProc = a;
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        resEuint256 = result;
    }
    function xor_euint256_uint256(einput a, uint256 b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        uint256 bProc = b;
        euint256 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        resEuint256 = result;
    }
    function xor_uint256_euint256(uint256 a, einput b, bytes calldata inputProof) public {
        uint256 aProc = a;
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        resEuint256 = result;
    }
    function eq_euint256_uint256(einput a, uint256 b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        uint256 bProc = b;
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function eq_uint256_euint256(uint256 a, einput b, bytes calldata inputProof) public {
        uint256 aProc = a;
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function ne_euint256_uint256(einput a, uint256 b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        uint256 bProc = b;
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function ne_uint256_euint256(uint256 a, einput b, bytes calldata inputProof) public {
        uint256 aProc = a;
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function shl_euint8_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint8 result = TFHE.shl(aProc, bProc);
        TFHE.allowThis(result);
        resEuint8 = result;
    }
    function shl_euint8_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        uint8 bProc = b;
        euint8 result = TFHE.shl(aProc, bProc);
        TFHE.allowThis(result);
        resEuint8 = result;
    }
    function shr_euint8_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint8 result = TFHE.shr(aProc, bProc);
        TFHE.allowThis(result);
        resEuint8 = result;
    }
    function shr_euint8_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        uint8 bProc = b;
        euint8 result = TFHE.shr(aProc, bProc);
        TFHE.allowThis(result);
        resEuint8 = result;
    }
    function rotl_euint8_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint8 result = TFHE.rotl(aProc, bProc);
        TFHE.allowThis(result);
        resEuint8 = result;
    }
    function rotl_euint8_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        uint8 bProc = b;
        euint8 result = TFHE.rotl(aProc, bProc);
        TFHE.allowThis(result);
        resEuint8 = result;
    }
    function rotr_euint8_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint8 result = TFHE.rotr(aProc, bProc);
        TFHE.allowThis(result);
        resEuint8 = result;
    }
    function rotr_euint8_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        uint8 bProc = b;
        euint8 result = TFHE.rotr(aProc, bProc);
        TFHE.allowThis(result);
        resEuint8 = result;
    }
    function shl_euint16_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint16 result = TFHE.shl(aProc, bProc);
        TFHE.allowThis(result);
        resEuint16 = result;
    }
    function shl_euint16_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        uint8 bProc = b;
        euint16 result = TFHE.shl(aProc, bProc);
        TFHE.allowThis(result);
        resEuint16 = result;
    }
    function shr_euint16_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint16 result = TFHE.shr(aProc, bProc);
        TFHE.allowThis(result);
        resEuint16 = result;
    }
    function shr_euint16_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        uint8 bProc = b;
        euint16 result = TFHE.shr(aProc, bProc);
        TFHE.allowThis(result);
        resEuint16 = result;
    }
    function rotl_euint16_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint16 result = TFHE.rotl(aProc, bProc);
        TFHE.allowThis(result);
        resEuint16 = result;
    }
    function rotl_euint16_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        uint8 bProc = b;
        euint16 result = TFHE.rotl(aProc, bProc);
        TFHE.allowThis(result);
        resEuint16 = result;
    }
    function rotr_euint16_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint16 result = TFHE.rotr(aProc, bProc);
        TFHE.allowThis(result);
        resEuint16 = result;
    }
    function rotr_euint16_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        uint8 bProc = b;
        euint16 result = TFHE.rotr(aProc, bProc);
        TFHE.allowThis(result);
        resEuint16 = result;
    }
    function shl_euint32_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint32 result = TFHE.shl(aProc, bProc);
        TFHE.allowThis(result);
        resEuint32 = result;
    }
    function shl_euint32_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        uint8 bProc = b;
        euint32 result = TFHE.shl(aProc, bProc);
        TFHE.allowThis(result);
        resEuint32 = result;
    }
    function shr_euint32_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint32 result = TFHE.shr(aProc, bProc);
        TFHE.allowThis(result);
        resEuint32 = result;
    }
    function shr_euint32_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        uint8 bProc = b;
        euint32 result = TFHE.shr(aProc, bProc);
        TFHE.allowThis(result);
        resEuint32 = result;
    }
    function rotl_euint32_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint32 result = TFHE.rotl(aProc, bProc);
        TFHE.allowThis(result);
        resEuint32 = result;
    }
    function rotl_euint32_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        uint8 bProc = b;
        euint32 result = TFHE.rotl(aProc, bProc);
        TFHE.allowThis(result);
        resEuint32 = result;
    }
    function rotr_euint32_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint32 result = TFHE.rotr(aProc, bProc);
        TFHE.allowThis(result);
        resEuint32 = result;
    }
    function rotr_euint32_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        uint8 bProc = b;
        euint32 result = TFHE.rotr(aProc, bProc);
        TFHE.allowThis(result);
        resEuint32 = result;
    }
    function shl_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint64 result = TFHE.shl(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function shl_euint64_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint8 bProc = b;
        euint64 result = TFHE.shl(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function shr_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint64 result = TFHE.shr(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function shr_euint64_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint8 bProc = b;
        euint64 result = TFHE.shr(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function rotl_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint64 result = TFHE.rotl(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function rotl_euint64_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint8 bProc = b;
        euint64 result = TFHE.rotl(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function rotr_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint64 result = TFHE.rotr(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function rotr_euint64_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint8 bProc = b;
        euint64 result = TFHE.rotr(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function shl_euint128_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint128 result = TFHE.shl(aProc, bProc);
        TFHE.allowThis(result);
        resEuint128 = result;
    }
    function shl_euint128_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        uint8 bProc = b;
        euint128 result = TFHE.shl(aProc, bProc);
        TFHE.allowThis(result);
        resEuint128 = result;
    }
    function shr_euint128_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint128 result = TFHE.shr(aProc, bProc);
        TFHE.allowThis(result);
        resEuint128 = result;
    }
    function shr_euint128_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        uint8 bProc = b;
        euint128 result = TFHE.shr(aProc, bProc);
        TFHE.allowThis(result);
        resEuint128 = result;
    }
    function rotl_euint128_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint128 result = TFHE.rotl(aProc, bProc);
        TFHE.allowThis(result);
        resEuint128 = result;
    }
    function rotl_euint128_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        uint8 bProc = b;
        euint128 result = TFHE.rotl(aProc, bProc);
        TFHE.allowThis(result);
        resEuint128 = result;
    }
    function rotr_euint128_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint128 result = TFHE.rotr(aProc, bProc);
        TFHE.allowThis(result);
        resEuint128 = result;
    }
    function rotr_euint128_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        uint8 bProc = b;
        euint128 result = TFHE.rotr(aProc, bProc);
        TFHE.allowThis(result);
        resEuint128 = result;
    }
    function shl_euint256_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint256 result = TFHE.shl(aProc, bProc);
        TFHE.allowThis(result);
        resEuint256 = result;
    }
    function shl_euint256_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        uint8 bProc = b;
        euint256 result = TFHE.shl(aProc, bProc);
        TFHE.allowThis(result);
        resEuint256 = result;
    }
    function shr_euint256_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint256 result = TFHE.shr(aProc, bProc);
        TFHE.allowThis(result);
        resEuint256 = result;
    }
    function shr_euint256_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        uint8 bProc = b;
        euint256 result = TFHE.shr(aProc, bProc);
        TFHE.allowThis(result);
        resEuint256 = result;
    }
    function rotl_euint256_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint256 result = TFHE.rotl(aProc, bProc);
        TFHE.allowThis(result);
        resEuint256 = result;
    }
    function rotl_euint256_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        uint8 bProc = b;
        euint256 result = TFHE.rotl(aProc, bProc);
        TFHE.allowThis(result);
        resEuint256 = result;
    }
    function rotr_euint256_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint256 result = TFHE.rotr(aProc, bProc);
        TFHE.allowThis(result);
        resEuint256 = result;
    }
    function rotr_euint256_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        uint8 bProc = b;
        euint256 result = TFHE.rotr(aProc, bProc);
        TFHE.allowThis(result);
        resEuint256 = result;
    }
    function neg_euint8(einput a, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint8 result = TFHE.neg(aProc);
        TFHE.allowThis(result);
        resEuint8 = result;
    }
    function not_euint8(einput a, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint8 result = TFHE.not(aProc);
        TFHE.allowThis(result);
        resEuint8 = result;
    }
    function neg_euint16(einput a, bytes calldata inputProof) public {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint16 result = TFHE.neg(aProc);
        TFHE.allowThis(result);
        resEuint16 = result;
    }
    function not_euint16(einput a, bytes calldata inputProof) public {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint16 result = TFHE.not(aProc);
        TFHE.allowThis(result);
        resEuint16 = result;
    }
    function neg_euint32(einput a, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint32 result = TFHE.neg(aProc);
        TFHE.allowThis(result);
        resEuint32 = result;
    }
    function not_euint32(einput a, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint32 result = TFHE.not(aProc);
        TFHE.allowThis(result);
        resEuint32 = result;
    }
    function neg_euint64(einput a, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 result = TFHE.neg(aProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function not_euint64(einput a, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 result = TFHE.not(aProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function neg_euint128(einput a, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        euint128 result = TFHE.neg(aProc);
        TFHE.allowThis(result);
        resEuint128 = result;
    }
    function not_euint128(einput a, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        euint128 result = TFHE.not(aProc);
        TFHE.allowThis(result);
        resEuint128 = result;
    }
    function neg_euint256(einput a, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint256 result = TFHE.neg(aProc);
        TFHE.allowThis(result);
        resEuint256 = result;
    }
    function not_euint256(einput a, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint256 result = TFHE.not(aProc);
        TFHE.allowThis(result);
        resEuint256 = result;
    }
}
