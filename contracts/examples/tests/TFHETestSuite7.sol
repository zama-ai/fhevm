// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../../lib/TFHE.sol";
import "../../lib/FHEVMConfig.sol";

contract TFHETestSuite7 {
    ebool public resb;
    euint4 public res4;
    euint8 public res8;
    euint16 public res16;
    euint32 public res32;
    euint64 public res64;
    euint128 public res128;
    euint256 public res256;

    constructor() {
        TFHE.setFHEVM(FHEVMConfig.defaultConfig());
    }

    function add_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint64 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function sub_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint64 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function mul_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint64 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function and_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint64 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function or_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint64 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function xor_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint64 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function eq_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ne_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ge_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function gt_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function le_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function lt_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function min_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint64 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function max_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint64 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function add_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint64 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function sub_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint64 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function mul_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint64 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function and_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint64 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function or_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint64 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function xor_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint64 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function eq_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ne_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ge_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function gt_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function le_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function lt_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function min_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint64 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function max_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint64 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function add_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function sub_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function mul_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function and_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function or_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function xor_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function eq_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ne_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ge_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function gt_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function le_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function lt_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function min_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function max_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function add_euint64_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function sub_euint64_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function mul_euint64_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function and_euint64_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function or_euint64_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function xor_euint64_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function eq_euint64_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ne_euint64_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ge_euint64_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function gt_euint64_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function le_euint64_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function lt_euint64_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function min_euint64_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function max_euint64_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function add_euint64_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function sub_euint64_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function mul_euint64_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function and_euint64_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function or_euint64_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function xor_euint64_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function eq_euint64_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ne_euint64_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ge_euint64_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function gt_euint64_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function le_euint64_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function lt_euint64_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function min_euint64_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function max_euint64_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function add_euint64_uint64(einput a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint64 bProc = b;
        euint64 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function add_uint64_euint64(uint64 a, einput b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function sub_euint64_uint64(einput a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint64 bProc = b;
        euint64 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function sub_uint64_euint64(uint64 a, einput b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function mul_euint64_uint64(einput a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint64 bProc = b;
        euint64 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function mul_uint64_euint64(uint64 a, einput b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function div_euint64_uint64(einput a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint64 bProc = b;
        euint64 result = TFHE.div(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function rem_euint64_uint64(einput a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint64 bProc = b;
        euint64 result = TFHE.rem(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function and_euint64_uint64(einput a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint64 bProc = b;
        euint64 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function and_uint64_euint64(uint64 a, einput b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function or_euint64_uint64(einput a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint64 bProc = b;
        euint64 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function or_uint64_euint64(uint64 a, einput b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function xor_euint64_uint64(einput a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint64 bProc = b;
        euint64 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function xor_uint64_euint64(uint64 a, einput b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function eq_euint64_uint64(einput a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint64 bProc = b;
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function eq_uint64_euint64(uint64 a, einput b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ne_euint64_uint64(einput a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint64 bProc = b;
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ne_uint64_euint64(uint64 a, einput b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ge_euint64_uint64(einput a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint64 bProc = b;
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ge_uint64_euint64(uint64 a, einput b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
}
