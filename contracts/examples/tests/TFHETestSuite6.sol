// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../../lib/TFHE.sol";
import "../../lib/FHEVMConfig.sol";

contract TFHETestSuite6 {
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

    function le_euint32_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function lt_euint32_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function min_euint32_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function max_euint32_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function add_euint32_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function sub_euint32_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function mul_euint32_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function and_euint32_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function or_euint32_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function xor_euint32_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function eq_euint32_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ne_euint32_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ge_euint32_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function gt_euint32_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function le_euint32_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function lt_euint32_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function min_euint32_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function max_euint32_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function add_euint32_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function sub_euint32_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function mul_euint32_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function and_euint32_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function or_euint32_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function xor_euint32_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function eq_euint32_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ne_euint32_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ge_euint32_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function gt_euint32_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function le_euint32_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function lt_euint32_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function min_euint32_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function max_euint32_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function add_euint32_uint32(einput a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        uint32 bProc = b;
        euint32 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        res32 = result;
    }
    function add_uint32_euint32(uint32 a, einput b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        res32 = result;
    }
    function sub_euint32_uint32(einput a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        uint32 bProc = b;
        euint32 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        res32 = result;
    }
    function sub_uint32_euint32(uint32 a, einput b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        res32 = result;
    }
    function mul_euint32_uint32(einput a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        uint32 bProc = b;
        euint32 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        res32 = result;
    }
    function mul_uint32_euint32(uint32 a, einput b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        res32 = result;
    }
    function div_euint32_uint32(einput a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        uint32 bProc = b;
        euint32 result = TFHE.div(aProc, bProc);
        TFHE.allowThis(result);
        res32 = result;
    }
    function rem_euint32_uint32(einput a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        uint32 bProc = b;
        euint32 result = TFHE.rem(aProc, bProc);
        TFHE.allowThis(result);
        res32 = result;
    }
    function and_euint32_uint32(einput a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        uint32 bProc = b;
        euint32 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        res32 = result;
    }
    function and_uint32_euint32(uint32 a, einput b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        res32 = result;
    }
    function or_euint32_uint32(einput a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        uint32 bProc = b;
        euint32 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        res32 = result;
    }
    function or_uint32_euint32(uint32 a, einput b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        res32 = result;
    }
    function xor_euint32_uint32(einput a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        uint32 bProc = b;
        euint32 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        res32 = result;
    }
    function xor_uint32_euint32(uint32 a, einput b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        res32 = result;
    }
    function eq_euint32_uint32(einput a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        uint32 bProc = b;
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function eq_uint32_euint32(uint32 a, einput b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ne_euint32_uint32(einput a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        uint32 bProc = b;
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ne_uint32_euint32(uint32 a, einput b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ge_euint32_uint32(einput a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        uint32 bProc = b;
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ge_uint32_euint32(uint32 a, einput b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function gt_euint32_uint32(einput a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        uint32 bProc = b;
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function gt_uint32_euint32(uint32 a, einput b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function le_euint32_uint32(einput a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        uint32 bProc = b;
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function le_uint32_euint32(uint32 a, einput b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function lt_euint32_uint32(einput a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        uint32 bProc = b;
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function lt_uint32_euint32(uint32 a, einput b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function min_euint32_uint32(einput a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        uint32 bProc = b;
        euint32 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        res32 = result;
    }
    function min_uint32_euint32(uint32 a, einput b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        res32 = result;
    }
    function max_euint32_uint32(einput a, uint32 b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        uint32 bProc = b;
        euint32 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        res32 = result;
    }
    function max_uint32_euint32(uint32 a, einput b, bytes calldata inputProof) public {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        res32 = result;
    }
    function add_euint64_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint64 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function sub_euint64_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint64 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function mul_euint64_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint64 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function and_euint64_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint64 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function or_euint64_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint64 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function xor_euint64_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint64 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function eq_euint64_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ne_euint64_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ge_euint64_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function gt_euint64_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function le_euint64_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function lt_euint64_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function min_euint64_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint64 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function max_euint64_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint64 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function add_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint64 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function sub_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint64 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function mul_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint64 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function and_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint64 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function or_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint64 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function xor_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint64 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function eq_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ne_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ge_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function gt_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function le_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function lt_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function min_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint64 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function max_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint64 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
}
