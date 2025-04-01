// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../../lib/TFHE.sol";
import "../../lib/FHEVMConfig.sol";

contract TFHETestSuite3 {
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

    function mul_euint32_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        resEuint32 = result;
    }
    function and_euint32_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        resEuint32 = result;
    }
    function or_euint32_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        resEuint32 = result;
    }
    function xor_euint32_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        resEuint32 = result;
    }
    function eq_euint32_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function ne_euint32_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function ge_euint32_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function gt_euint32_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function le_euint32_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function lt_euint32_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function min_euint32_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        resEuint32 = result;
    }
    function max_euint32_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        resEuint32 = result;
    }
    function add_euint32_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function sub_euint32_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function mul_euint32_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function and_euint32_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function or_euint32_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function xor_euint32_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function eq_euint32_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function ne_euint32_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function ge_euint32_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function gt_euint32_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function le_euint32_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function lt_euint32_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function min_euint32_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function max_euint32_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function add_euint32_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        resEuint128 = result;
    }
    function sub_euint32_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        resEuint128 = result;
    }
    function mul_euint32_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        resEuint128 = result;
    }
    function and_euint32_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        resEuint128 = result;
    }
    function or_euint32_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        resEuint128 = result;
    }
    function xor_euint32_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        resEuint128 = result;
    }
    function eq_euint32_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function ne_euint32_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function ge_euint32_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function gt_euint32_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function le_euint32_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function lt_euint32_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function min_euint32_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        resEuint128 = result;
    }
    function max_euint32_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        resEuint128 = result;
    }
    function and_euint32_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        resEuint256 = result;
    }
    function or_euint32_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        resEuint256 = result;
    }
    function xor_euint32_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        resEuint256 = result;
    }
    function eq_euint32_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function ne_euint32_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function add_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint64 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function sub_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint64 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function mul_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint64 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function and_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint64 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function or_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint64 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function xor_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint64 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function eq_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function ne_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function ge_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function gt_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function le_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function lt_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function min_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint64 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function max_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint64 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function add_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint64 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function sub_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint64 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function mul_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint64 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function and_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint64 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function or_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint64 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function xor_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint64 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function eq_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function ne_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function ge_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function gt_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function le_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function lt_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function min_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint64 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function max_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint64 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function add_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint64 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function sub_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint64 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function mul_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint64 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function and_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint64 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function or_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint64 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function xor_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint64 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function eq_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function ne_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function ge_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function gt_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function le_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function lt_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resEbool = result;
    }
    function min_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint64 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function max_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint64 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function add_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function sub_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
    function mul_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        resEuint64 = result;
    }
}
