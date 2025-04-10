// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../../lib/HTTPZ.sol";
import "../../lib/HTTPZConfig.sol";

contract HTTPZTestSuite4 {
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

    function and_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.asEuint64(a, inputProof);
        euint64 bProc = HTTPZ.asEuint64(b, inputProof);
        euint64 result = HTTPZ.and(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint64 = result;
    }
    function or_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.asEuint64(a, inputProof);
        euint64 bProc = HTTPZ.asEuint64(b, inputProof);
        euint64 result = HTTPZ.or(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint64 = result;
    }
    function xor_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.asEuint64(a, inputProof);
        euint64 bProc = HTTPZ.asEuint64(b, inputProof);
        euint64 result = HTTPZ.xor(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint64 = result;
    }
    function eq_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.asEuint64(a, inputProof);
        euint64 bProc = HTTPZ.asEuint64(b, inputProof);
        ebool result = HTTPZ.eq(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ne_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.asEuint64(a, inputProof);
        euint64 bProc = HTTPZ.asEuint64(b, inputProof);
        ebool result = HTTPZ.ne(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ge_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.asEuint64(a, inputProof);
        euint64 bProc = HTTPZ.asEuint64(b, inputProof);
        ebool result = HTTPZ.ge(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function gt_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.asEuint64(a, inputProof);
        euint64 bProc = HTTPZ.asEuint64(b, inputProof);
        ebool result = HTTPZ.gt(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function le_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.asEuint64(a, inputProof);
        euint64 bProc = HTTPZ.asEuint64(b, inputProof);
        ebool result = HTTPZ.le(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function lt_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.asEuint64(a, inputProof);
        euint64 bProc = HTTPZ.asEuint64(b, inputProof);
        ebool result = HTTPZ.lt(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function min_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.asEuint64(a, inputProof);
        euint64 bProc = HTTPZ.asEuint64(b, inputProof);
        euint64 result = HTTPZ.min(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint64 = result;
    }
    function max_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.asEuint64(a, inputProof);
        euint64 bProc = HTTPZ.asEuint64(b, inputProof);
        euint64 result = HTTPZ.max(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint64 = result;
    }
    function add_euint64_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.asEuint64(a, inputProof);
        euint128 bProc = HTTPZ.asEuint128(b, inputProof);
        euint128 result = HTTPZ.add(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function sub_euint64_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.asEuint64(a, inputProof);
        euint128 bProc = HTTPZ.asEuint128(b, inputProof);
        euint128 result = HTTPZ.sub(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function mul_euint64_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.asEuint64(a, inputProof);
        euint128 bProc = HTTPZ.asEuint128(b, inputProof);
        euint128 result = HTTPZ.mul(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function and_euint64_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.asEuint64(a, inputProof);
        euint128 bProc = HTTPZ.asEuint128(b, inputProof);
        euint128 result = HTTPZ.and(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function or_euint64_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.asEuint64(a, inputProof);
        euint128 bProc = HTTPZ.asEuint128(b, inputProof);
        euint128 result = HTTPZ.or(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function xor_euint64_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.asEuint64(a, inputProof);
        euint128 bProc = HTTPZ.asEuint128(b, inputProof);
        euint128 result = HTTPZ.xor(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function eq_euint64_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.asEuint64(a, inputProof);
        euint128 bProc = HTTPZ.asEuint128(b, inputProof);
        ebool result = HTTPZ.eq(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ne_euint64_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.asEuint64(a, inputProof);
        euint128 bProc = HTTPZ.asEuint128(b, inputProof);
        ebool result = HTTPZ.ne(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ge_euint64_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.asEuint64(a, inputProof);
        euint128 bProc = HTTPZ.asEuint128(b, inputProof);
        ebool result = HTTPZ.ge(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function gt_euint64_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.asEuint64(a, inputProof);
        euint128 bProc = HTTPZ.asEuint128(b, inputProof);
        ebool result = HTTPZ.gt(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function le_euint64_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.asEuint64(a, inputProof);
        euint128 bProc = HTTPZ.asEuint128(b, inputProof);
        ebool result = HTTPZ.le(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function lt_euint64_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.asEuint64(a, inputProof);
        euint128 bProc = HTTPZ.asEuint128(b, inputProof);
        ebool result = HTTPZ.lt(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function min_euint64_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.asEuint64(a, inputProof);
        euint128 bProc = HTTPZ.asEuint128(b, inputProof);
        euint128 result = HTTPZ.min(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function max_euint64_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.asEuint64(a, inputProof);
        euint128 bProc = HTTPZ.asEuint128(b, inputProof);
        euint128 result = HTTPZ.max(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function and_euint64_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.asEuint64(a, inputProof);
        euint256 bProc = HTTPZ.asEuint256(b, inputProof);
        euint256 result = HTTPZ.and(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint256 = result;
    }
    function or_euint64_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.asEuint64(a, inputProof);
        euint256 bProc = HTTPZ.asEuint256(b, inputProof);
        euint256 result = HTTPZ.or(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint256 = result;
    }
    function xor_euint64_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.asEuint64(a, inputProof);
        euint256 bProc = HTTPZ.asEuint256(b, inputProof);
        euint256 result = HTTPZ.xor(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint256 = result;
    }
    function eq_euint64_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.asEuint64(a, inputProof);
        euint256 bProc = HTTPZ.asEuint256(b, inputProof);
        ebool result = HTTPZ.eq(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ne_euint64_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.asEuint64(a, inputProof);
        euint256 bProc = HTTPZ.asEuint256(b, inputProof);
        ebool result = HTTPZ.ne(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function add_euint128_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint8 bProc = HTTPZ.asEuint8(b, inputProof);
        euint128 result = HTTPZ.add(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function sub_euint128_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint8 bProc = HTTPZ.asEuint8(b, inputProof);
        euint128 result = HTTPZ.sub(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function mul_euint128_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint8 bProc = HTTPZ.asEuint8(b, inputProof);
        euint128 result = HTTPZ.mul(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function and_euint128_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint8 bProc = HTTPZ.asEuint8(b, inputProof);
        euint128 result = HTTPZ.and(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function or_euint128_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint8 bProc = HTTPZ.asEuint8(b, inputProof);
        euint128 result = HTTPZ.or(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function xor_euint128_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint8 bProc = HTTPZ.asEuint8(b, inputProof);
        euint128 result = HTTPZ.xor(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function eq_euint128_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint8 bProc = HTTPZ.asEuint8(b, inputProof);
        ebool result = HTTPZ.eq(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ne_euint128_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint8 bProc = HTTPZ.asEuint8(b, inputProof);
        ebool result = HTTPZ.ne(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ge_euint128_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint8 bProc = HTTPZ.asEuint8(b, inputProof);
        ebool result = HTTPZ.ge(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function gt_euint128_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint8 bProc = HTTPZ.asEuint8(b, inputProof);
        ebool result = HTTPZ.gt(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function le_euint128_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint8 bProc = HTTPZ.asEuint8(b, inputProof);
        ebool result = HTTPZ.le(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function lt_euint128_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint8 bProc = HTTPZ.asEuint8(b, inputProof);
        ebool result = HTTPZ.lt(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function min_euint128_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint8 bProc = HTTPZ.asEuint8(b, inputProof);
        euint128 result = HTTPZ.min(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function max_euint128_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint8 bProc = HTTPZ.asEuint8(b, inputProof);
        euint128 result = HTTPZ.max(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function add_euint128_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint16 bProc = HTTPZ.asEuint16(b, inputProof);
        euint128 result = HTTPZ.add(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function sub_euint128_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint16 bProc = HTTPZ.asEuint16(b, inputProof);
        euint128 result = HTTPZ.sub(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function mul_euint128_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint16 bProc = HTTPZ.asEuint16(b, inputProof);
        euint128 result = HTTPZ.mul(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function and_euint128_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint16 bProc = HTTPZ.asEuint16(b, inputProof);
        euint128 result = HTTPZ.and(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function or_euint128_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint16 bProc = HTTPZ.asEuint16(b, inputProof);
        euint128 result = HTTPZ.or(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function xor_euint128_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint16 bProc = HTTPZ.asEuint16(b, inputProof);
        euint128 result = HTTPZ.xor(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function eq_euint128_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint16 bProc = HTTPZ.asEuint16(b, inputProof);
        ebool result = HTTPZ.eq(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ne_euint128_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint16 bProc = HTTPZ.asEuint16(b, inputProof);
        ebool result = HTTPZ.ne(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ge_euint128_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint16 bProc = HTTPZ.asEuint16(b, inputProof);
        ebool result = HTTPZ.ge(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function gt_euint128_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint16 bProc = HTTPZ.asEuint16(b, inputProof);
        ebool result = HTTPZ.gt(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function le_euint128_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint16 bProc = HTTPZ.asEuint16(b, inputProof);
        ebool result = HTTPZ.le(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function lt_euint128_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint16 bProc = HTTPZ.asEuint16(b, inputProof);
        ebool result = HTTPZ.lt(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function min_euint128_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint16 bProc = HTTPZ.asEuint16(b, inputProof);
        euint128 result = HTTPZ.min(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function max_euint128_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint16 bProc = HTTPZ.asEuint16(b, inputProof);
        euint128 result = HTTPZ.max(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function add_euint128_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint32 bProc = HTTPZ.asEuint32(b, inputProof);
        euint128 result = HTTPZ.add(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function sub_euint128_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint32 bProc = HTTPZ.asEuint32(b, inputProof);
        euint128 result = HTTPZ.sub(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function mul_euint128_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint32 bProc = HTTPZ.asEuint32(b, inputProof);
        euint128 result = HTTPZ.mul(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function and_euint128_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint32 bProc = HTTPZ.asEuint32(b, inputProof);
        euint128 result = HTTPZ.and(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function or_euint128_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint32 bProc = HTTPZ.asEuint32(b, inputProof);
        euint128 result = HTTPZ.or(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function xor_euint128_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint32 bProc = HTTPZ.asEuint32(b, inputProof);
        euint128 result = HTTPZ.xor(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function eq_euint128_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint32 bProc = HTTPZ.asEuint32(b, inputProof);
        ebool result = HTTPZ.eq(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ne_euint128_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint32 bProc = HTTPZ.asEuint32(b, inputProof);
        ebool result = HTTPZ.ne(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ge_euint128_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint32 bProc = HTTPZ.asEuint32(b, inputProof);
        ebool result = HTTPZ.ge(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function gt_euint128_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint32 bProc = HTTPZ.asEuint32(b, inputProof);
        ebool result = HTTPZ.gt(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function le_euint128_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint32 bProc = HTTPZ.asEuint32(b, inputProof);
        ebool result = HTTPZ.le(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function lt_euint128_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint32 bProc = HTTPZ.asEuint32(b, inputProof);
        ebool result = HTTPZ.lt(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function min_euint128_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint32 bProc = HTTPZ.asEuint32(b, inputProof);
        euint128 result = HTTPZ.min(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function max_euint128_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint32 bProc = HTTPZ.asEuint32(b, inputProof);
        euint128 result = HTTPZ.max(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function add_euint128_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint64 bProc = HTTPZ.asEuint64(b, inputProof);
        euint128 result = HTTPZ.add(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function sub_euint128_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint64 bProc = HTTPZ.asEuint64(b, inputProof);
        euint128 result = HTTPZ.sub(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function mul_euint128_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint64 bProc = HTTPZ.asEuint64(b, inputProof);
        euint128 result = HTTPZ.mul(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function and_euint128_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint64 bProc = HTTPZ.asEuint64(b, inputProof);
        euint128 result = HTTPZ.and(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function or_euint128_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint64 bProc = HTTPZ.asEuint64(b, inputProof);
        euint128 result = HTTPZ.or(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function xor_euint128_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint64 bProc = HTTPZ.asEuint64(b, inputProof);
        euint128 result = HTTPZ.xor(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function eq_euint128_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint64 bProc = HTTPZ.asEuint64(b, inputProof);
        ebool result = HTTPZ.eq(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ne_euint128_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint64 bProc = HTTPZ.asEuint64(b, inputProof);
        ebool result = HTTPZ.ne(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ge_euint128_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint64 bProc = HTTPZ.asEuint64(b, inputProof);
        ebool result = HTTPZ.ge(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function gt_euint128_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint64 bProc = HTTPZ.asEuint64(b, inputProof);
        ebool result = HTTPZ.gt(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function le_euint128_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint64 bProc = HTTPZ.asEuint64(b, inputProof);
        ebool result = HTTPZ.le(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function lt_euint128_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint64 bProc = HTTPZ.asEuint64(b, inputProof);
        ebool result = HTTPZ.lt(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function min_euint128_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint64 bProc = HTTPZ.asEuint64(b, inputProof);
        euint128 result = HTTPZ.min(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function max_euint128_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint64 bProc = HTTPZ.asEuint64(b, inputProof);
        euint128 result = HTTPZ.max(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function add_euint128_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint128 bProc = HTTPZ.asEuint128(b, inputProof);
        euint128 result = HTTPZ.add(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function sub_euint128_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint128 bProc = HTTPZ.asEuint128(b, inputProof);
        euint128 result = HTTPZ.sub(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function mul_euint128_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint128 bProc = HTTPZ.asEuint128(b, inputProof);
        euint128 result = HTTPZ.mul(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function and_euint128_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.asEuint128(a, inputProof);
        euint128 bProc = HTTPZ.asEuint128(b, inputProof);
        euint128 result = HTTPZ.and(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
}
