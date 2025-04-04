// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../../lib/TFHE.sol";
import "../../lib/FHEVMConfig.sol";

contract TFHETestSuite7 {
    ebool public resb;
    euint8 public res8;
    euint16 public res16;
    euint32 public res32;
    euint64 public res64;
    euint128 public res128;
    euint256 public res256;

    constructor() {
        TFHE.setFHEVM(FHEVMConfig.defaultConfig());
    }

    function add_euint128_uint128(einput a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        uint128 bProc = b;
        euint128 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function add_uint128_euint128(uint128 a, einput b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function sub_euint128_uint128(einput a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        uint128 bProc = b;
        euint128 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function sub_uint128_euint128(uint128 a, einput b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function mul_euint128_uint128(einput a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        uint128 bProc = b;
        euint128 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function mul_uint128_euint128(uint128 a, einput b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function div_euint128_uint128(einput a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        uint128 bProc = b;
        euint128 result = TFHE.div(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function rem_euint128_uint128(einput a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        uint128 bProc = b;
        euint128 result = TFHE.rem(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function and_euint128_uint128(einput a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        uint128 bProc = b;
        euint128 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function and_uint128_euint128(uint128 a, einput b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function or_euint128_uint128(einput a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        uint128 bProc = b;
        euint128 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function or_uint128_euint128(uint128 a, einput b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function xor_euint128_uint128(einput a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        uint128 bProc = b;
        euint128 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function xor_uint128_euint128(uint128 a, einput b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function eq_euint128_uint128(einput a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        uint128 bProc = b;
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function eq_uint128_euint128(uint128 a, einput b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ne_euint128_uint128(einput a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        uint128 bProc = b;
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ne_uint128_euint128(uint128 a, einput b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ge_euint128_uint128(einput a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        uint128 bProc = b;
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ge_uint128_euint128(uint128 a, einput b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function gt_euint128_uint128(einput a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        uint128 bProc = b;
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function gt_uint128_euint128(uint128 a, einput b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function le_euint128_uint128(einput a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        uint128 bProc = b;
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function le_uint128_euint128(uint128 a, einput b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function lt_euint128_uint128(einput a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        uint128 bProc = b;
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function lt_uint128_euint128(uint128 a, einput b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function min_euint128_uint128(einput a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        uint128 bProc = b;
        euint128 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function min_uint128_euint128(uint128 a, einput b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function max_euint128_uint128(einput a, uint128 b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        uint128 bProc = b;
        euint128 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function max_uint128_euint128(uint128 a, einput b, bytes calldata inputProof) public {
        uint128 aProc = a;
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint128 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function add_euint256_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint256 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function sub_euint256_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint256 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function mul_euint256_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint256 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function and_euint256_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint256 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function or_euint256_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint256 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function xor_euint256_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint256 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function eq_euint256_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ne_euint256_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ge_euint256_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function gt_euint256_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function le_euint256_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function lt_euint256_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function min_euint256_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint256 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function max_euint256_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint256 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function add_euint256_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint256 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function sub_euint256_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint256 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function mul_euint256_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint256 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function and_euint256_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint256 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function or_euint256_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint256 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function xor_euint256_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint256 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function eq_euint256_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ne_euint256_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ge_euint256_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function gt_euint256_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function le_euint256_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function lt_euint256_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function min_euint256_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint256 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function max_euint256_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint256 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function add_euint256_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint256 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function sub_euint256_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint256 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function mul_euint256_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint256 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function and_euint256_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint256 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function or_euint256_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint256 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function xor_euint256_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint256 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function eq_euint256_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ne_euint256_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ge_euint256_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function gt_euint256_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function le_euint256_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function lt_euint256_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function min_euint256_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint256 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function max_euint256_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint256 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function add_euint256_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint256 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function sub_euint256_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint256 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function mul_euint256_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint256 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function and_euint256_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint256 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function or_euint256_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint256 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function xor_euint256_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint256 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function eq_euint256_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ne_euint256_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ge_euint256_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function gt_euint256_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function le_euint256_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function lt_euint256_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function min_euint256_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint256 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function max_euint256_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint256 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function add_euint256_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint256 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function sub_euint256_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint256 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function mul_euint256_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint256 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function and_euint256_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint256 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
}
