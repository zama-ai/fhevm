// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../../lib/TFHE.sol";
import "../../lib/FHEVMConfig.sol";

contract TFHETestSuite10 {
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
    function or_euint256_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint256 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function xor_euint256_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint256 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function eq_euint256_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ne_euint256_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ge_euint256_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function gt_euint256_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function le_euint256_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function lt_euint256_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function min_euint256_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint256 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function max_euint256_euint128(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint128 bProc = TFHE.asEuint128(b, inputProof);
        euint256 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function add_euint256_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function sub_euint256_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function mul_euint256_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function and_euint256_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function or_euint256_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function xor_euint256_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function eq_euint256_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ne_euint256_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ge_euint256_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function gt_euint256_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function le_euint256_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function lt_euint256_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function min_euint256_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function max_euint256_euint256(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function add_euint256_uint256(einput a, uint256 b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        uint256 bProc = b;
        euint256 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function add_uint256_euint256(uint256 a, einput b, bytes calldata inputProof) public {
        uint256 aProc = a;
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function sub_euint256_uint256(einput a, uint256 b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        uint256 bProc = b;
        euint256 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function sub_uint256_euint256(uint256 a, einput b, bytes calldata inputProof) public {
        uint256 aProc = a;
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function mul_euint256_uint256(einput a, uint256 b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        uint256 bProc = b;
        euint256 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function mul_uint256_euint256(uint256 a, einput b, bytes calldata inputProof) public {
        uint256 aProc = a;
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function div_euint256_uint256(einput a, uint256 b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        uint256 bProc = b;
        euint256 result = TFHE.div(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function rem_euint256_uint256(einput a, uint256 b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        uint256 bProc = b;
        euint256 result = TFHE.rem(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function and_euint256_uint256(einput a, uint256 b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        uint256 bProc = b;
        euint256 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function and_uint256_euint256(uint256 a, einput b, bytes calldata inputProof) public {
        uint256 aProc = a;
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function or_euint256_uint256(einput a, uint256 b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        uint256 bProc = b;
        euint256 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function or_uint256_euint256(uint256 a, einput b, bytes calldata inputProof) public {
        uint256 aProc = a;
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function xor_euint256_uint256(einput a, uint256 b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        uint256 bProc = b;
        euint256 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function xor_uint256_euint256(uint256 a, einput b, bytes calldata inputProof) public {
        uint256 aProc = a;
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function eq_euint256_uint256(einput a, uint256 b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        uint256 bProc = b;
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function eq_uint256_euint256(uint256 a, einput b, bytes calldata inputProof) public {
        uint256 aProc = a;
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ne_euint256_uint256(einput a, uint256 b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        uint256 bProc = b;
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ne_uint256_euint256(uint256 a, einput b, bytes calldata inputProof) public {
        uint256 aProc = a;
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ge_euint256_uint256(einput a, uint256 b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        uint256 bProc = b;
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ge_uint256_euint256(uint256 a, einput b, bytes calldata inputProof) public {
        uint256 aProc = a;
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function gt_euint256_uint256(einput a, uint256 b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        uint256 bProc = b;
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function gt_uint256_euint256(uint256 a, einput b, bytes calldata inputProof) public {
        uint256 aProc = a;
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function le_euint256_uint256(einput a, uint256 b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        uint256 bProc = b;
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function le_uint256_euint256(uint256 a, einput b, bytes calldata inputProof) public {
        uint256 aProc = a;
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function lt_euint256_uint256(einput a, uint256 b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        uint256 bProc = b;
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function lt_uint256_euint256(uint256 a, einput b, bytes calldata inputProof) public {
        uint256 aProc = a;
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function min_euint256_uint256(einput a, uint256 b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        uint256 bProc = b;
        euint256 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function min_uint256_euint256(uint256 a, einput b, bytes calldata inputProof) public {
        uint256 aProc = a;
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function max_euint256_uint256(einput a, uint256 b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        uint256 bProc = b;
        euint256 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function max_uint256_euint256(uint256 a, einput b, bytes calldata inputProof) public {
        uint256 aProc = a;
        euint256 bProc = TFHE.asEuint256(b, inputProof);
        euint256 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function shl_euint4_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        uint8 bProc = b;
        euint4 result = TFHE.shl(aProc, bProc);
        TFHE.allowThis(result);
        res4 = result;
    }
    function shr_euint4_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        uint8 bProc = b;
        euint4 result = TFHE.shr(aProc, bProc);
        TFHE.allowThis(result);
        res4 = result;
    }
    function rotl_euint4_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        uint8 bProc = b;
        euint4 result = TFHE.rotl(aProc, bProc);
        TFHE.allowThis(result);
        res4 = result;
    }
    function rotr_euint4_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        uint8 bProc = b;
        euint4 result = TFHE.rotr(aProc, bProc);
        TFHE.allowThis(result);
        res4 = result;
    }
}
