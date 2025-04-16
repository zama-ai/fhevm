// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../../lib/HTTPZ.sol";
import "../../lib/HTTPZConfig.sol";

contract HTTPZTestSuite5 {
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
        HTTPZ.setCoprocessor(HTTPZConfig.defaultConfig());
    }

    function or_euint128_euint128(externalEuint128 a, externalEuint128 b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.fromExternal(a, inputProof);
        euint128 bProc = HTTPZ.fromExternal(b, inputProof);
        euint128 result = HTTPZ.or(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function xor_euint128_euint128(externalEuint128 a, externalEuint128 b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.fromExternal(a, inputProof);
        euint128 bProc = HTTPZ.fromExternal(b, inputProof);
        euint128 result = HTTPZ.xor(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function eq_euint128_euint128(externalEuint128 a, externalEuint128 b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.fromExternal(a, inputProof);
        euint128 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.eq(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ne_euint128_euint128(externalEuint128 a, externalEuint128 b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.fromExternal(a, inputProof);
        euint128 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.ne(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ge_euint128_euint128(externalEuint128 a, externalEuint128 b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.fromExternal(a, inputProof);
        euint128 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.ge(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function gt_euint128_euint128(externalEuint128 a, externalEuint128 b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.fromExternal(a, inputProof);
        euint128 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.gt(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function le_euint128_euint128(externalEuint128 a, externalEuint128 b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.fromExternal(a, inputProof);
        euint128 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.le(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function lt_euint128_euint128(externalEuint128 a, externalEuint128 b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.fromExternal(a, inputProof);
        euint128 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.lt(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function min_euint128_euint128(externalEuint128 a, externalEuint128 b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.fromExternal(a, inputProof);
        euint128 bProc = HTTPZ.fromExternal(b, inputProof);
        euint128 result = HTTPZ.min(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function max_euint128_euint128(externalEuint128 a, externalEuint128 b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.fromExternal(a, inputProof);
        euint128 bProc = HTTPZ.fromExternal(b, inputProof);
        euint128 result = HTTPZ.max(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function and_euint128_euint256(externalEuint128 a, externalEuint256 b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.fromExternal(a, inputProof);
        euint256 bProc = HTTPZ.fromExternal(b, inputProof);
        euint256 result = HTTPZ.and(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint256 = result;
    }
    function or_euint128_euint256(externalEuint128 a, externalEuint256 b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.fromExternal(a, inputProof);
        euint256 bProc = HTTPZ.fromExternal(b, inputProof);
        euint256 result = HTTPZ.or(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint256 = result;
    }
    function xor_euint128_euint256(externalEuint128 a, externalEuint256 b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.fromExternal(a, inputProof);
        euint256 bProc = HTTPZ.fromExternal(b, inputProof);
        euint256 result = HTTPZ.xor(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint256 = result;
    }
    function eq_euint128_euint256(externalEuint128 a, externalEuint256 b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.fromExternal(a, inputProof);
        euint256 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.eq(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ne_euint128_euint256(externalEuint128 a, externalEuint256 b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.fromExternal(a, inputProof);
        euint256 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.ne(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function and_euint256_euint8(externalEuint256 a, externalEuint8 b, bytes calldata inputProof) public {
        euint256 aProc = HTTPZ.fromExternal(a, inputProof);
        euint8 bProc = HTTPZ.fromExternal(b, inputProof);
        euint256 result = HTTPZ.and(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint256 = result;
    }
    function or_euint256_euint8(externalEuint256 a, externalEuint8 b, bytes calldata inputProof) public {
        euint256 aProc = HTTPZ.fromExternal(a, inputProof);
        euint8 bProc = HTTPZ.fromExternal(b, inputProof);
        euint256 result = HTTPZ.or(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint256 = result;
    }
    function xor_euint256_euint8(externalEuint256 a, externalEuint8 b, bytes calldata inputProof) public {
        euint256 aProc = HTTPZ.fromExternal(a, inputProof);
        euint8 bProc = HTTPZ.fromExternal(b, inputProof);
        euint256 result = HTTPZ.xor(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint256 = result;
    }
    function eq_euint256_euint8(externalEuint256 a, externalEuint8 b, bytes calldata inputProof) public {
        euint256 aProc = HTTPZ.fromExternal(a, inputProof);
        euint8 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.eq(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ne_euint256_euint8(externalEuint256 a, externalEuint8 b, bytes calldata inputProof) public {
        euint256 aProc = HTTPZ.fromExternal(a, inputProof);
        euint8 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.ne(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function and_euint256_euint16(externalEuint256 a, externalEuint16 b, bytes calldata inputProof) public {
        euint256 aProc = HTTPZ.fromExternal(a, inputProof);
        euint16 bProc = HTTPZ.fromExternal(b, inputProof);
        euint256 result = HTTPZ.and(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint256 = result;
    }
    function or_euint256_euint16(externalEuint256 a, externalEuint16 b, bytes calldata inputProof) public {
        euint256 aProc = HTTPZ.fromExternal(a, inputProof);
        euint16 bProc = HTTPZ.fromExternal(b, inputProof);
        euint256 result = HTTPZ.or(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint256 = result;
    }
    function xor_euint256_euint16(externalEuint256 a, externalEuint16 b, bytes calldata inputProof) public {
        euint256 aProc = HTTPZ.fromExternal(a, inputProof);
        euint16 bProc = HTTPZ.fromExternal(b, inputProof);
        euint256 result = HTTPZ.xor(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint256 = result;
    }
    function eq_euint256_euint16(externalEuint256 a, externalEuint16 b, bytes calldata inputProof) public {
        euint256 aProc = HTTPZ.fromExternal(a, inputProof);
        euint16 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.eq(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ne_euint256_euint16(externalEuint256 a, externalEuint16 b, bytes calldata inputProof) public {
        euint256 aProc = HTTPZ.fromExternal(a, inputProof);
        euint16 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.ne(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function and_euint256_euint32(externalEuint256 a, externalEuint32 b, bytes calldata inputProof) public {
        euint256 aProc = HTTPZ.fromExternal(a, inputProof);
        euint32 bProc = HTTPZ.fromExternal(b, inputProof);
        euint256 result = HTTPZ.and(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint256 = result;
    }
    function or_euint256_euint32(externalEuint256 a, externalEuint32 b, bytes calldata inputProof) public {
        euint256 aProc = HTTPZ.fromExternal(a, inputProof);
        euint32 bProc = HTTPZ.fromExternal(b, inputProof);
        euint256 result = HTTPZ.or(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint256 = result;
    }
    function xor_euint256_euint32(externalEuint256 a, externalEuint32 b, bytes calldata inputProof) public {
        euint256 aProc = HTTPZ.fromExternal(a, inputProof);
        euint32 bProc = HTTPZ.fromExternal(b, inputProof);
        euint256 result = HTTPZ.xor(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint256 = result;
    }
    function eq_euint256_euint32(externalEuint256 a, externalEuint32 b, bytes calldata inputProof) public {
        euint256 aProc = HTTPZ.fromExternal(a, inputProof);
        euint32 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.eq(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ne_euint256_euint32(externalEuint256 a, externalEuint32 b, bytes calldata inputProof) public {
        euint256 aProc = HTTPZ.fromExternal(a, inputProof);
        euint32 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.ne(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function and_euint256_euint64(externalEuint256 a, externalEuint64 b, bytes calldata inputProof) public {
        euint256 aProc = HTTPZ.fromExternal(a, inputProof);
        euint64 bProc = HTTPZ.fromExternal(b, inputProof);
        euint256 result = HTTPZ.and(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint256 = result;
    }
    function or_euint256_euint64(externalEuint256 a, externalEuint64 b, bytes calldata inputProof) public {
        euint256 aProc = HTTPZ.fromExternal(a, inputProof);
        euint64 bProc = HTTPZ.fromExternal(b, inputProof);
        euint256 result = HTTPZ.or(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint256 = result;
    }
    function xor_euint256_euint64(externalEuint256 a, externalEuint64 b, bytes calldata inputProof) public {
        euint256 aProc = HTTPZ.fromExternal(a, inputProof);
        euint64 bProc = HTTPZ.fromExternal(b, inputProof);
        euint256 result = HTTPZ.xor(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint256 = result;
    }
    function eq_euint256_euint64(externalEuint256 a, externalEuint64 b, bytes calldata inputProof) public {
        euint256 aProc = HTTPZ.fromExternal(a, inputProof);
        euint64 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.eq(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ne_euint256_euint64(externalEuint256 a, externalEuint64 b, bytes calldata inputProof) public {
        euint256 aProc = HTTPZ.fromExternal(a, inputProof);
        euint64 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.ne(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function and_euint256_euint128(externalEuint256 a, externalEuint128 b, bytes calldata inputProof) public {
        euint256 aProc = HTTPZ.fromExternal(a, inputProof);
        euint128 bProc = HTTPZ.fromExternal(b, inputProof);
        euint256 result = HTTPZ.and(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint256 = result;
    }
    function or_euint256_euint128(externalEuint256 a, externalEuint128 b, bytes calldata inputProof) public {
        euint256 aProc = HTTPZ.fromExternal(a, inputProof);
        euint128 bProc = HTTPZ.fromExternal(b, inputProof);
        euint256 result = HTTPZ.or(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint256 = result;
    }
    function xor_euint256_euint128(externalEuint256 a, externalEuint128 b, bytes calldata inputProof) public {
        euint256 aProc = HTTPZ.fromExternal(a, inputProof);
        euint128 bProc = HTTPZ.fromExternal(b, inputProof);
        euint256 result = HTTPZ.xor(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint256 = result;
    }
    function eq_euint256_euint128(externalEuint256 a, externalEuint128 b, bytes calldata inputProof) public {
        euint256 aProc = HTTPZ.fromExternal(a, inputProof);
        euint128 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.eq(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ne_euint256_euint128(externalEuint256 a, externalEuint128 b, bytes calldata inputProof) public {
        euint256 aProc = HTTPZ.fromExternal(a, inputProof);
        euint128 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.ne(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function and_euint256_euint256(externalEuint256 a, externalEuint256 b, bytes calldata inputProof) public {
        euint256 aProc = HTTPZ.fromExternal(a, inputProof);
        euint256 bProc = HTTPZ.fromExternal(b, inputProof);
        euint256 result = HTTPZ.and(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint256 = result;
    }
    function or_euint256_euint256(externalEuint256 a, externalEuint256 b, bytes calldata inputProof) public {
        euint256 aProc = HTTPZ.fromExternal(a, inputProof);
        euint256 bProc = HTTPZ.fromExternal(b, inputProof);
        euint256 result = HTTPZ.or(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint256 = result;
    }
    function xor_euint256_euint256(externalEuint256 a, externalEuint256 b, bytes calldata inputProof) public {
        euint256 aProc = HTTPZ.fromExternal(a, inputProof);
        euint256 bProc = HTTPZ.fromExternal(b, inputProof);
        euint256 result = HTTPZ.xor(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint256 = result;
    }
    function eq_euint256_euint256(externalEuint256 a, externalEuint256 b, bytes calldata inputProof) public {
        euint256 aProc = HTTPZ.fromExternal(a, inputProof);
        euint256 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.eq(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ne_euint256_euint256(externalEuint256 a, externalEuint256 b, bytes calldata inputProof) public {
        euint256 aProc = HTTPZ.fromExternal(a, inputProof);
        euint256 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.ne(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function add_euint8_uint8(externalEuint8 a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = HTTPZ.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint8 result = HTTPZ.add(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint8 = result;
    }
    function add_uint8_euint8(uint8 a, externalEuint8 b, bytes calldata inputProof) public {
        uint8 aProc = a;
        euint8 bProc = HTTPZ.fromExternal(b, inputProof);
        euint8 result = HTTPZ.add(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint8 = result;
    }
    function sub_euint8_uint8(externalEuint8 a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = HTTPZ.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint8 result = HTTPZ.sub(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint8 = result;
    }
    function sub_uint8_euint8(uint8 a, externalEuint8 b, bytes calldata inputProof) public {
        uint8 aProc = a;
        euint8 bProc = HTTPZ.fromExternal(b, inputProof);
        euint8 result = HTTPZ.sub(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint8 = result;
    }
    function mul_euint8_uint8(externalEuint8 a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = HTTPZ.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint8 result = HTTPZ.mul(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint8 = result;
    }
    function mul_uint8_euint8(uint8 a, externalEuint8 b, bytes calldata inputProof) public {
        uint8 aProc = a;
        euint8 bProc = HTTPZ.fromExternal(b, inputProof);
        euint8 result = HTTPZ.mul(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint8 = result;
    }
    function div_euint8_uint8(externalEuint8 a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = HTTPZ.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint8 result = HTTPZ.div(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint8 = result;
    }
    function rem_euint8_uint8(externalEuint8 a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = HTTPZ.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint8 result = HTTPZ.rem(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint8 = result;
    }
    function and_euint8_uint8(externalEuint8 a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = HTTPZ.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint8 result = HTTPZ.and(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint8 = result;
    }
    function and_uint8_euint8(uint8 a, externalEuint8 b, bytes calldata inputProof) public {
        uint8 aProc = a;
        euint8 bProc = HTTPZ.fromExternal(b, inputProof);
        euint8 result = HTTPZ.and(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint8 = result;
    }
    function or_euint8_uint8(externalEuint8 a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = HTTPZ.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint8 result = HTTPZ.or(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint8 = result;
    }
    function or_uint8_euint8(uint8 a, externalEuint8 b, bytes calldata inputProof) public {
        uint8 aProc = a;
        euint8 bProc = HTTPZ.fromExternal(b, inputProof);
        euint8 result = HTTPZ.or(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint8 = result;
    }
    function xor_euint8_uint8(externalEuint8 a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = HTTPZ.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint8 result = HTTPZ.xor(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint8 = result;
    }
    function xor_uint8_euint8(uint8 a, externalEuint8 b, bytes calldata inputProof) public {
        uint8 aProc = a;
        euint8 bProc = HTTPZ.fromExternal(b, inputProof);
        euint8 result = HTTPZ.xor(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint8 = result;
    }
    function eq_euint8_uint8(externalEuint8 a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = HTTPZ.fromExternal(a, inputProof);
        uint8 bProc = b;
        ebool result = HTTPZ.eq(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function eq_uint8_euint8(uint8 a, externalEuint8 b, bytes calldata inputProof) public {
        uint8 aProc = a;
        euint8 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.eq(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ne_euint8_uint8(externalEuint8 a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = HTTPZ.fromExternal(a, inputProof);
        uint8 bProc = b;
        ebool result = HTTPZ.ne(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ne_uint8_euint8(uint8 a, externalEuint8 b, bytes calldata inputProof) public {
        uint8 aProc = a;
        euint8 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.ne(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ge_euint8_uint8(externalEuint8 a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = HTTPZ.fromExternal(a, inputProof);
        uint8 bProc = b;
        ebool result = HTTPZ.ge(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ge_uint8_euint8(uint8 a, externalEuint8 b, bytes calldata inputProof) public {
        uint8 aProc = a;
        euint8 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.ge(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function gt_euint8_uint8(externalEuint8 a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = HTTPZ.fromExternal(a, inputProof);
        uint8 bProc = b;
        ebool result = HTTPZ.gt(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function gt_uint8_euint8(uint8 a, externalEuint8 b, bytes calldata inputProof) public {
        uint8 aProc = a;
        euint8 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.gt(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function le_euint8_uint8(externalEuint8 a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = HTTPZ.fromExternal(a, inputProof);
        uint8 bProc = b;
        ebool result = HTTPZ.le(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function le_uint8_euint8(uint8 a, externalEuint8 b, bytes calldata inputProof) public {
        uint8 aProc = a;
        euint8 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.le(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function lt_euint8_uint8(externalEuint8 a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = HTTPZ.fromExternal(a, inputProof);
        uint8 bProc = b;
        ebool result = HTTPZ.lt(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function lt_uint8_euint8(uint8 a, externalEuint8 b, bytes calldata inputProof) public {
        uint8 aProc = a;
        euint8 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.lt(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function min_euint8_uint8(externalEuint8 a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = HTTPZ.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint8 result = HTTPZ.min(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint8 = result;
    }
    function min_uint8_euint8(uint8 a, externalEuint8 b, bytes calldata inputProof) public {
        uint8 aProc = a;
        euint8 bProc = HTTPZ.fromExternal(b, inputProof);
        euint8 result = HTTPZ.min(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint8 = result;
    }
    function max_euint8_uint8(externalEuint8 a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = HTTPZ.fromExternal(a, inputProof);
        uint8 bProc = b;
        euint8 result = HTTPZ.max(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint8 = result;
    }
    function max_uint8_euint8(uint8 a, externalEuint8 b, bytes calldata inputProof) public {
        uint8 aProc = a;
        euint8 bProc = HTTPZ.fromExternal(b, inputProof);
        euint8 result = HTTPZ.max(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint8 = result;
    }
    function add_euint16_uint16(externalEuint16 a, uint16 b, bytes calldata inputProof) public {
        euint16 aProc = HTTPZ.fromExternal(a, inputProof);
        uint16 bProc = b;
        euint16 result = HTTPZ.add(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint16 = result;
    }
    function add_uint16_euint16(uint16 a, externalEuint16 b, bytes calldata inputProof) public {
        uint16 aProc = a;
        euint16 bProc = HTTPZ.fromExternal(b, inputProof);
        euint16 result = HTTPZ.add(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint16 = result;
    }
    function sub_euint16_uint16(externalEuint16 a, uint16 b, bytes calldata inputProof) public {
        euint16 aProc = HTTPZ.fromExternal(a, inputProof);
        uint16 bProc = b;
        euint16 result = HTTPZ.sub(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint16 = result;
    }
    function sub_uint16_euint16(uint16 a, externalEuint16 b, bytes calldata inputProof) public {
        uint16 aProc = a;
        euint16 bProc = HTTPZ.fromExternal(b, inputProof);
        euint16 result = HTTPZ.sub(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint16 = result;
    }
    function mul_euint16_uint16(externalEuint16 a, uint16 b, bytes calldata inputProof) public {
        euint16 aProc = HTTPZ.fromExternal(a, inputProof);
        uint16 bProc = b;
        euint16 result = HTTPZ.mul(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint16 = result;
    }
    function mul_uint16_euint16(uint16 a, externalEuint16 b, bytes calldata inputProof) public {
        uint16 aProc = a;
        euint16 bProc = HTTPZ.fromExternal(b, inputProof);
        euint16 result = HTTPZ.mul(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint16 = result;
    }
    function div_euint16_uint16(externalEuint16 a, uint16 b, bytes calldata inputProof) public {
        euint16 aProc = HTTPZ.fromExternal(a, inputProof);
        uint16 bProc = b;
        euint16 result = HTTPZ.div(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint16 = result;
    }
    function rem_euint16_uint16(externalEuint16 a, uint16 b, bytes calldata inputProof) public {
        euint16 aProc = HTTPZ.fromExternal(a, inputProof);
        uint16 bProc = b;
        euint16 result = HTTPZ.rem(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint16 = result;
    }
    function and_euint16_uint16(externalEuint16 a, uint16 b, bytes calldata inputProof) public {
        euint16 aProc = HTTPZ.fromExternal(a, inputProof);
        uint16 bProc = b;
        euint16 result = HTTPZ.and(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint16 = result;
    }
    function and_uint16_euint16(uint16 a, externalEuint16 b, bytes calldata inputProof) public {
        uint16 aProc = a;
        euint16 bProc = HTTPZ.fromExternal(b, inputProof);
        euint16 result = HTTPZ.and(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint16 = result;
    }
    function or_euint16_uint16(externalEuint16 a, uint16 b, bytes calldata inputProof) public {
        euint16 aProc = HTTPZ.fromExternal(a, inputProof);
        uint16 bProc = b;
        euint16 result = HTTPZ.or(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint16 = result;
    }
    function or_uint16_euint16(uint16 a, externalEuint16 b, bytes calldata inputProof) public {
        uint16 aProc = a;
        euint16 bProc = HTTPZ.fromExternal(b, inputProof);
        euint16 result = HTTPZ.or(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint16 = result;
    }
    function xor_euint16_uint16(externalEuint16 a, uint16 b, bytes calldata inputProof) public {
        euint16 aProc = HTTPZ.fromExternal(a, inputProof);
        uint16 bProc = b;
        euint16 result = HTTPZ.xor(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint16 = result;
    }
    function xor_uint16_euint16(uint16 a, externalEuint16 b, bytes calldata inputProof) public {
        uint16 aProc = a;
        euint16 bProc = HTTPZ.fromExternal(b, inputProof);
        euint16 result = HTTPZ.xor(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint16 = result;
    }
    function eq_euint16_uint16(externalEuint16 a, uint16 b, bytes calldata inputProof) public {
        euint16 aProc = HTTPZ.fromExternal(a, inputProof);
        uint16 bProc = b;
        ebool result = HTTPZ.eq(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
}
