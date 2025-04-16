// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../../lib/HTTPZ.sol";
import "../../lib/HTTPZConfig.sol";

contract HTTPZTestSuite6 {
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

    function eq_uint16_euint16(uint16 a, externalEuint16 b, bytes calldata inputProof) public {
        uint16 aProc = a;
        euint16 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.eq(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ne_euint16_uint16(externalEuint16 a, uint16 b, bytes calldata inputProof) public {
        euint16 aProc = HTTPZ.fromExternal(a, inputProof);
        uint16 bProc = b;
        ebool result = HTTPZ.ne(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ne_uint16_euint16(uint16 a, externalEuint16 b, bytes calldata inputProof) public {
        uint16 aProc = a;
        euint16 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.ne(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ge_euint16_uint16(externalEuint16 a, uint16 b, bytes calldata inputProof) public {
        euint16 aProc = HTTPZ.fromExternal(a, inputProof);
        uint16 bProc = b;
        ebool result = HTTPZ.ge(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ge_uint16_euint16(uint16 a, externalEuint16 b, bytes calldata inputProof) public {
        uint16 aProc = a;
        euint16 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.ge(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function gt_euint16_uint16(externalEuint16 a, uint16 b, bytes calldata inputProof) public {
        euint16 aProc = HTTPZ.fromExternal(a, inputProof);
        uint16 bProc = b;
        ebool result = HTTPZ.gt(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function gt_uint16_euint16(uint16 a, externalEuint16 b, bytes calldata inputProof) public {
        uint16 aProc = a;
        euint16 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.gt(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function le_euint16_uint16(externalEuint16 a, uint16 b, bytes calldata inputProof) public {
        euint16 aProc = HTTPZ.fromExternal(a, inputProof);
        uint16 bProc = b;
        ebool result = HTTPZ.le(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function le_uint16_euint16(uint16 a, externalEuint16 b, bytes calldata inputProof) public {
        uint16 aProc = a;
        euint16 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.le(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function lt_euint16_uint16(externalEuint16 a, uint16 b, bytes calldata inputProof) public {
        euint16 aProc = HTTPZ.fromExternal(a, inputProof);
        uint16 bProc = b;
        ebool result = HTTPZ.lt(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function lt_uint16_euint16(uint16 a, externalEuint16 b, bytes calldata inputProof) public {
        uint16 aProc = a;
        euint16 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.lt(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function min_euint16_uint16(externalEuint16 a, uint16 b, bytes calldata inputProof) public {
        euint16 aProc = HTTPZ.fromExternal(a, inputProof);
        uint16 bProc = b;
        euint16 result = HTTPZ.min(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint16 = result;
    }
    function min_uint16_euint16(uint16 a, externalEuint16 b, bytes calldata inputProof) public {
        uint16 aProc = a;
        euint16 bProc = HTTPZ.fromExternal(b, inputProof);
        euint16 result = HTTPZ.min(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint16 = result;
    }
    function max_euint16_uint16(externalEuint16 a, uint16 b, bytes calldata inputProof) public {
        euint16 aProc = HTTPZ.fromExternal(a, inputProof);
        uint16 bProc = b;
        euint16 result = HTTPZ.max(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint16 = result;
    }
    function max_uint16_euint16(uint16 a, externalEuint16 b, bytes calldata inputProof) public {
        uint16 aProc = a;
        euint16 bProc = HTTPZ.fromExternal(b, inputProof);
        euint16 result = HTTPZ.max(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint16 = result;
    }
    function add_euint32_uint32(externalEuint32 a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = HTTPZ.fromExternal(a, inputProof);
        uint32 bProc = b;
        euint32 result = HTTPZ.add(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint32 = result;
    }
    function add_uint32_euint32(uint32 a, externalEuint32 b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = HTTPZ.fromExternal(b, inputProof);
        euint32 result = HTTPZ.add(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint32 = result;
    }
    function sub_euint32_uint32(externalEuint32 a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = HTTPZ.fromExternal(a, inputProof);
        uint32 bProc = b;
        euint32 result = HTTPZ.sub(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint32 = result;
    }
    function sub_uint32_euint32(uint32 a, externalEuint32 b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = HTTPZ.fromExternal(b, inputProof);
        euint32 result = HTTPZ.sub(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint32 = result;
    }
    function mul_euint32_uint32(externalEuint32 a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = HTTPZ.fromExternal(a, inputProof);
        uint32 bProc = b;
        euint32 result = HTTPZ.mul(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint32 = result;
    }
    function mul_uint32_euint32(uint32 a, externalEuint32 b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = HTTPZ.fromExternal(b, inputProof);
        euint32 result = HTTPZ.mul(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint32 = result;
    }
    function div_euint32_uint32(externalEuint32 a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = HTTPZ.fromExternal(a, inputProof);
        uint32 bProc = b;
        euint32 result = HTTPZ.div(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint32 = result;
    }
    function rem_euint32_uint32(externalEuint32 a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = HTTPZ.fromExternal(a, inputProof);
        uint32 bProc = b;
        euint32 result = HTTPZ.rem(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint32 = result;
    }
    function and_euint32_uint32(externalEuint32 a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = HTTPZ.fromExternal(a, inputProof);
        uint32 bProc = b;
        euint32 result = HTTPZ.and(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint32 = result;
    }
    function and_uint32_euint32(uint32 a, externalEuint32 b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = HTTPZ.fromExternal(b, inputProof);
        euint32 result = HTTPZ.and(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint32 = result;
    }
    function or_euint32_uint32(externalEuint32 a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = HTTPZ.fromExternal(a, inputProof);
        uint32 bProc = b;
        euint32 result = HTTPZ.or(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint32 = result;
    }
    function or_uint32_euint32(uint32 a, externalEuint32 b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = HTTPZ.fromExternal(b, inputProof);
        euint32 result = HTTPZ.or(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint32 = result;
    }
    function xor_euint32_uint32(externalEuint32 a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = HTTPZ.fromExternal(a, inputProof);
        uint32 bProc = b;
        euint32 result = HTTPZ.xor(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint32 = result;
    }
    function xor_uint32_euint32(uint32 a, externalEuint32 b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = HTTPZ.fromExternal(b, inputProof);
        euint32 result = HTTPZ.xor(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint32 = result;
    }
    function eq_euint32_uint32(externalEuint32 a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = HTTPZ.fromExternal(a, inputProof);
        uint32 bProc = b;
        ebool result = HTTPZ.eq(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function eq_uint32_euint32(uint32 a, externalEuint32 b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.eq(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ne_euint32_uint32(externalEuint32 a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = HTTPZ.fromExternal(a, inputProof);
        uint32 bProc = b;
        ebool result = HTTPZ.ne(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ne_uint32_euint32(uint32 a, externalEuint32 b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.ne(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ge_euint32_uint32(externalEuint32 a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = HTTPZ.fromExternal(a, inputProof);
        uint32 bProc = b;
        ebool result = HTTPZ.ge(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ge_uint32_euint32(uint32 a, externalEuint32 b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.ge(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function gt_euint32_uint32(externalEuint32 a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = HTTPZ.fromExternal(a, inputProof);
        uint32 bProc = b;
        ebool result = HTTPZ.gt(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function gt_uint32_euint32(uint32 a, externalEuint32 b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.gt(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function le_euint32_uint32(externalEuint32 a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = HTTPZ.fromExternal(a, inputProof);
        uint32 bProc = b;
        ebool result = HTTPZ.le(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function le_uint32_euint32(uint32 a, externalEuint32 b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.le(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function lt_euint32_uint32(externalEuint32 a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = HTTPZ.fromExternal(a, inputProof);
        uint32 bProc = b;
        ebool result = HTTPZ.lt(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function lt_uint32_euint32(uint32 a, externalEuint32 b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.lt(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function min_euint32_uint32(externalEuint32 a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = HTTPZ.fromExternal(a, inputProof);
        uint32 bProc = b;
        euint32 result = HTTPZ.min(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint32 = result;
    }
    function min_uint32_euint32(uint32 a, externalEuint32 b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = HTTPZ.fromExternal(b, inputProof);
        euint32 result = HTTPZ.min(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint32 = result;
    }
    function max_euint32_uint32(externalEuint32 a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = HTTPZ.fromExternal(a, inputProof);
        uint32 bProc = b;
        euint32 result = HTTPZ.max(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint32 = result;
    }
    function max_uint32_euint32(uint32 a, externalEuint32 b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = HTTPZ.fromExternal(b, inputProof);
        euint32 result = HTTPZ.max(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint32 = result;
    }
    function add_euint64_uint64(externalEuint64 a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.fromExternal(a, inputProof);
        uint64 bProc = b;
        euint64 result = HTTPZ.add(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint64 = result;
    }
    function add_uint64_euint64(uint64 a, externalEuint64 b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = HTTPZ.fromExternal(b, inputProof);
        euint64 result = HTTPZ.add(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint64 = result;
    }
    function sub_euint64_uint64(externalEuint64 a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.fromExternal(a, inputProof);
        uint64 bProc = b;
        euint64 result = HTTPZ.sub(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint64 = result;
    }
    function sub_uint64_euint64(uint64 a, externalEuint64 b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = HTTPZ.fromExternal(b, inputProof);
        euint64 result = HTTPZ.sub(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint64 = result;
    }
    function mul_euint64_uint64(externalEuint64 a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.fromExternal(a, inputProof);
        uint64 bProc = b;
        euint64 result = HTTPZ.mul(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint64 = result;
    }
    function mul_uint64_euint64(uint64 a, externalEuint64 b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = HTTPZ.fromExternal(b, inputProof);
        euint64 result = HTTPZ.mul(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint64 = result;
    }
    function div_euint64_uint64(externalEuint64 a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.fromExternal(a, inputProof);
        uint64 bProc = b;
        euint64 result = HTTPZ.div(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint64 = result;
    }
    function rem_euint64_uint64(externalEuint64 a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.fromExternal(a, inputProof);
        uint64 bProc = b;
        euint64 result = HTTPZ.rem(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint64 = result;
    }
    function and_euint64_uint64(externalEuint64 a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.fromExternal(a, inputProof);
        uint64 bProc = b;
        euint64 result = HTTPZ.and(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint64 = result;
    }
    function and_uint64_euint64(uint64 a, externalEuint64 b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = HTTPZ.fromExternal(b, inputProof);
        euint64 result = HTTPZ.and(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint64 = result;
    }
    function or_euint64_uint64(externalEuint64 a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.fromExternal(a, inputProof);
        uint64 bProc = b;
        euint64 result = HTTPZ.or(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint64 = result;
    }
    function or_uint64_euint64(uint64 a, externalEuint64 b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = HTTPZ.fromExternal(b, inputProof);
        euint64 result = HTTPZ.or(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint64 = result;
    }
    function xor_euint64_uint64(externalEuint64 a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.fromExternal(a, inputProof);
        uint64 bProc = b;
        euint64 result = HTTPZ.xor(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint64 = result;
    }
    function xor_uint64_euint64(uint64 a, externalEuint64 b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = HTTPZ.fromExternal(b, inputProof);
        euint64 result = HTTPZ.xor(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint64 = result;
    }
    function eq_euint64_uint64(externalEuint64 a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.fromExternal(a, inputProof);
        uint64 bProc = b;
        ebool result = HTTPZ.eq(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function eq_uint64_euint64(uint64 a, externalEuint64 b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.eq(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ne_euint64_uint64(externalEuint64 a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.fromExternal(a, inputProof);
        uint64 bProc = b;
        ebool result = HTTPZ.ne(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ne_uint64_euint64(uint64 a, externalEuint64 b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.ne(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ge_euint64_uint64(externalEuint64 a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.fromExternal(a, inputProof);
        uint64 bProc = b;
        ebool result = HTTPZ.ge(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function ge_uint64_euint64(uint64 a, externalEuint64 b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.ge(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function gt_euint64_uint64(externalEuint64 a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.fromExternal(a, inputProof);
        uint64 bProc = b;
        ebool result = HTTPZ.gt(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function gt_uint64_euint64(uint64 a, externalEuint64 b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.gt(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function le_euint64_uint64(externalEuint64 a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.fromExternal(a, inputProof);
        uint64 bProc = b;
        ebool result = HTTPZ.le(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function le_uint64_euint64(uint64 a, externalEuint64 b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.le(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function lt_euint64_uint64(externalEuint64 a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.fromExternal(a, inputProof);
        uint64 bProc = b;
        ebool result = HTTPZ.lt(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function lt_uint64_euint64(uint64 a, externalEuint64 b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = HTTPZ.fromExternal(b, inputProof);
        ebool result = HTTPZ.lt(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
    function min_euint64_uint64(externalEuint64 a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.fromExternal(a, inputProof);
        uint64 bProc = b;
        euint64 result = HTTPZ.min(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint64 = result;
    }
    function min_uint64_euint64(uint64 a, externalEuint64 b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = HTTPZ.fromExternal(b, inputProof);
        euint64 result = HTTPZ.min(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint64 = result;
    }
    function max_euint64_uint64(externalEuint64 a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = HTTPZ.fromExternal(a, inputProof);
        uint64 bProc = b;
        euint64 result = HTTPZ.max(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint64 = result;
    }
    function max_uint64_euint64(uint64 a, externalEuint64 b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = HTTPZ.fromExternal(b, inputProof);
        euint64 result = HTTPZ.max(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint64 = result;
    }
    function add_euint128_uint128(externalEuint128 a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.fromExternal(a, inputProof);
        uint128 bProc = b;
        euint128 result = HTTPZ.add(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function add_uint128_euint128(uint128 a, externalEuint128 b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = HTTPZ.fromExternal(b, inputProof);
        euint128 result = HTTPZ.add(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function sub_euint128_uint128(externalEuint128 a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.fromExternal(a, inputProof);
        uint128 bProc = b;
        euint128 result = HTTPZ.sub(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function sub_uint128_euint128(uint128 a, externalEuint128 b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = HTTPZ.fromExternal(b, inputProof);
        euint128 result = HTTPZ.sub(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function mul_euint128_uint128(externalEuint128 a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.fromExternal(a, inputProof);
        uint128 bProc = b;
        euint128 result = HTTPZ.mul(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function mul_uint128_euint128(uint128 a, externalEuint128 b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = HTTPZ.fromExternal(b, inputProof);
        euint128 result = HTTPZ.mul(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function div_euint128_uint128(externalEuint128 a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.fromExternal(a, inputProof);
        uint128 bProc = b;
        euint128 result = HTTPZ.div(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function rem_euint128_uint128(externalEuint128 a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.fromExternal(a, inputProof);
        uint128 bProc = b;
        euint128 result = HTTPZ.rem(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function and_euint128_uint128(externalEuint128 a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.fromExternal(a, inputProof);
        uint128 bProc = b;
        euint128 result = HTTPZ.and(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function and_uint128_euint128(uint128 a, externalEuint128 b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = HTTPZ.fromExternal(b, inputProof);
        euint128 result = HTTPZ.and(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function or_euint128_uint128(externalEuint128 a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.fromExternal(a, inputProof);
        uint128 bProc = b;
        euint128 result = HTTPZ.or(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function or_uint128_euint128(uint128 a, externalEuint128 b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = HTTPZ.fromExternal(b, inputProof);
        euint128 result = HTTPZ.or(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function xor_euint128_uint128(externalEuint128 a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.fromExternal(a, inputProof);
        uint128 bProc = b;
        euint128 result = HTTPZ.xor(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function xor_uint128_euint128(uint128 a, externalEuint128 b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = HTTPZ.fromExternal(b, inputProof);
        euint128 result = HTTPZ.xor(aProc, bProc);
        HTTPZ.allowThis(result);
        resEuint128 = result;
    }
    function eq_euint128_uint128(externalEuint128 a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = HTTPZ.fromExternal(a, inputProof);
        uint128 bProc = b;
        ebool result = HTTPZ.eq(aProc, bProc);
        HTTPZ.allowThis(result);
        resEbool = result;
    }
}
