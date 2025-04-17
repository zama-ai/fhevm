// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../../lib/FHE.sol";
import "../../lib/FHEVMConfig.sol";

contract FHEVMTestSuite4 {
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

    function and_euint64_euint64(externalEuint64 a, externalEuint64 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        euint64 result = FHE.and(aProc, bProc);
        FHE.allowThis(result);
        resEuint64 = result;
    }
    function or_euint64_euint64(externalEuint64 a, externalEuint64 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        euint64 result = FHE.or(aProc, bProc);
        FHE.allowThis(result);
        resEuint64 = result;
    }
    function xor_euint64_euint64(externalEuint64 a, externalEuint64 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        euint64 result = FHE.xor(aProc, bProc);
        FHE.allowThis(result);
        resEuint64 = result;
    }
    function eq_euint64_euint64(externalEuint64 a, externalEuint64 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.eq(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function ne_euint64_euint64(externalEuint64 a, externalEuint64 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ne(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function ge_euint64_euint64(externalEuint64 a, externalEuint64 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ge(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function gt_euint64_euint64(externalEuint64 a, externalEuint64 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.gt(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function le_euint64_euint64(externalEuint64 a, externalEuint64 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.le(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function lt_euint64_euint64(externalEuint64 a, externalEuint64 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.lt(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function min_euint64_euint64(externalEuint64 a, externalEuint64 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        euint64 result = FHE.min(aProc, bProc);
        FHE.allowThis(result);
        resEuint64 = result;
    }
    function max_euint64_euint64(externalEuint64 a, externalEuint64 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        euint64 result = FHE.max(aProc, bProc);
        FHE.allowThis(result);
        resEuint64 = result;
    }
    function add_euint64_euint128(externalEuint64 a, externalEuint128 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.add(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function sub_euint64_euint128(externalEuint64 a, externalEuint128 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.sub(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function mul_euint64_euint128(externalEuint64 a, externalEuint128 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.mul(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function and_euint64_euint128(externalEuint64 a, externalEuint128 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.and(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function or_euint64_euint128(externalEuint64 a, externalEuint128 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.or(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function xor_euint64_euint128(externalEuint64 a, externalEuint128 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.xor(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function eq_euint64_euint128(externalEuint64 a, externalEuint128 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.eq(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function ne_euint64_euint128(externalEuint64 a, externalEuint128 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ne(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function ge_euint64_euint128(externalEuint64 a, externalEuint128 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ge(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function gt_euint64_euint128(externalEuint64 a, externalEuint128 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.gt(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function le_euint64_euint128(externalEuint64 a, externalEuint128 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.le(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function lt_euint64_euint128(externalEuint64 a, externalEuint128 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.lt(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function min_euint64_euint128(externalEuint64 a, externalEuint128 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.min(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function max_euint64_euint128(externalEuint64 a, externalEuint128 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.max(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function and_euint64_euint256(externalEuint64 a, externalEuint256 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint256 bProc = FHE.fromExternal(b, inputProof);
        euint256 result = FHE.and(aProc, bProc);
        FHE.allowThis(result);
        resEuint256 = result;
    }
    function or_euint64_euint256(externalEuint64 a, externalEuint256 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint256 bProc = FHE.fromExternal(b, inputProof);
        euint256 result = FHE.or(aProc, bProc);
        FHE.allowThis(result);
        resEuint256 = result;
    }
    function xor_euint64_euint256(externalEuint64 a, externalEuint256 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint256 bProc = FHE.fromExternal(b, inputProof);
        euint256 result = FHE.xor(aProc, bProc);
        FHE.allowThis(result);
        resEuint256 = result;
    }
    function eq_euint64_euint256(externalEuint64 a, externalEuint256 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint256 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.eq(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function ne_euint64_euint256(externalEuint64 a, externalEuint256 b, bytes calldata inputProof) public {
        euint64 aProc = FHE.fromExternal(a, inputProof);
        euint256 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ne(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function add_euint128_euint8(externalEuint128 a, externalEuint8 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.add(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function sub_euint128_euint8(externalEuint128 a, externalEuint8 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.sub(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function mul_euint128_euint8(externalEuint128 a, externalEuint8 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.mul(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function and_euint128_euint8(externalEuint128 a, externalEuint8 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.and(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function or_euint128_euint8(externalEuint128 a, externalEuint8 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.or(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function xor_euint128_euint8(externalEuint128 a, externalEuint8 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.xor(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function eq_euint128_euint8(externalEuint128 a, externalEuint8 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.eq(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function ne_euint128_euint8(externalEuint128 a, externalEuint8 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ne(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function ge_euint128_euint8(externalEuint128 a, externalEuint8 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ge(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function gt_euint128_euint8(externalEuint128 a, externalEuint8 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.gt(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function le_euint128_euint8(externalEuint128 a, externalEuint8 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.le(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function lt_euint128_euint8(externalEuint128 a, externalEuint8 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.lt(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function min_euint128_euint8(externalEuint128 a, externalEuint8 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.min(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function max_euint128_euint8(externalEuint128 a, externalEuint8 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint8 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.max(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function add_euint128_euint16(externalEuint128 a, externalEuint16 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint16 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.add(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function sub_euint128_euint16(externalEuint128 a, externalEuint16 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint16 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.sub(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function mul_euint128_euint16(externalEuint128 a, externalEuint16 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint16 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.mul(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function and_euint128_euint16(externalEuint128 a, externalEuint16 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint16 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.and(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function or_euint128_euint16(externalEuint128 a, externalEuint16 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint16 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.or(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function xor_euint128_euint16(externalEuint128 a, externalEuint16 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint16 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.xor(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function eq_euint128_euint16(externalEuint128 a, externalEuint16 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint16 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.eq(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function ne_euint128_euint16(externalEuint128 a, externalEuint16 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint16 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ne(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function ge_euint128_euint16(externalEuint128 a, externalEuint16 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint16 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ge(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function gt_euint128_euint16(externalEuint128 a, externalEuint16 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint16 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.gt(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function le_euint128_euint16(externalEuint128 a, externalEuint16 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint16 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.le(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function lt_euint128_euint16(externalEuint128 a, externalEuint16 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint16 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.lt(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function min_euint128_euint16(externalEuint128 a, externalEuint16 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint16 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.min(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function max_euint128_euint16(externalEuint128 a, externalEuint16 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint16 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.max(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function add_euint128_euint32(externalEuint128 a, externalEuint32 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint32 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.add(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function sub_euint128_euint32(externalEuint128 a, externalEuint32 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint32 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.sub(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function mul_euint128_euint32(externalEuint128 a, externalEuint32 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint32 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.mul(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function and_euint128_euint32(externalEuint128 a, externalEuint32 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint32 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.and(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function or_euint128_euint32(externalEuint128 a, externalEuint32 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint32 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.or(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function xor_euint128_euint32(externalEuint128 a, externalEuint32 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint32 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.xor(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function eq_euint128_euint32(externalEuint128 a, externalEuint32 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint32 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.eq(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function ne_euint128_euint32(externalEuint128 a, externalEuint32 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint32 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ne(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function ge_euint128_euint32(externalEuint128 a, externalEuint32 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint32 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ge(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function gt_euint128_euint32(externalEuint128 a, externalEuint32 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint32 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.gt(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function le_euint128_euint32(externalEuint128 a, externalEuint32 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint32 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.le(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function lt_euint128_euint32(externalEuint128 a, externalEuint32 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint32 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.lt(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function min_euint128_euint32(externalEuint128 a, externalEuint32 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint32 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.min(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function max_euint128_euint32(externalEuint128 a, externalEuint32 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint32 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.max(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function add_euint128_euint64(externalEuint128 a, externalEuint64 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.add(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function sub_euint128_euint64(externalEuint128 a, externalEuint64 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.sub(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function mul_euint128_euint64(externalEuint128 a, externalEuint64 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.mul(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function and_euint128_euint64(externalEuint128 a, externalEuint64 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.and(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function or_euint128_euint64(externalEuint128 a, externalEuint64 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.or(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function xor_euint128_euint64(externalEuint128 a, externalEuint64 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.xor(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function eq_euint128_euint64(externalEuint128 a, externalEuint64 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.eq(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function ne_euint128_euint64(externalEuint128 a, externalEuint64 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ne(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function ge_euint128_euint64(externalEuint128 a, externalEuint64 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.ge(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function gt_euint128_euint64(externalEuint128 a, externalEuint64 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.gt(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function le_euint128_euint64(externalEuint128 a, externalEuint64 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.le(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function lt_euint128_euint64(externalEuint128 a, externalEuint64 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        ebool result = FHE.lt(aProc, bProc);
        FHE.allowThis(result);
        resEbool = result;
    }
    function min_euint128_euint64(externalEuint128 a, externalEuint64 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.min(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function max_euint128_euint64(externalEuint128 a, externalEuint64 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint64 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.max(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function add_euint128_euint128(externalEuint128 a, externalEuint128 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.add(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function sub_euint128_euint128(externalEuint128 a, externalEuint128 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.sub(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function mul_euint128_euint128(externalEuint128 a, externalEuint128 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.mul(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
    function and_euint128_euint128(externalEuint128 a, externalEuint128 b, bytes calldata inputProof) public {
        euint128 aProc = FHE.fromExternal(a, inputProof);
        euint128 bProc = FHE.fromExternal(b, inputProof);
        euint128 result = FHE.and(aProc, bProc);
        FHE.allowThis(result);
        resEuint128 = result;
    }
}
