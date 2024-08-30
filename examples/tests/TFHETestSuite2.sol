// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../../lib/TFHE.sol";
import "../../payment/Payment.sol";

contract TFHETestSuite2 {
    ebool public resb;
    euint4 public res4;
    euint8 public res8;
    euint16 public res16;
    euint32 public res32;
    euint64 public res64;

    constructor() payable {
        Payment.depositForThis(msg.value);
    }

    function eq_euint8_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ne_euint8_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ge_euint8_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function gt_euint8_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function le_euint8_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function lt_euint8_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function min_euint8_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint8 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        res8 = result;
    }
    function max_euint8_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint8 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        res8 = result;
    }
    function add_euint8_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint8 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        res8 = result;
    }
    function sub_euint8_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint8 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        res8 = result;
    }
    function mul_euint8_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint8 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        res8 = result;
    }
    function and_euint8_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint8 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        res8 = result;
    }
    function or_euint8_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint8 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        res8 = result;
    }
    function xor_euint8_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint8 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        res8 = result;
    }
    function eq_euint8_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ne_euint8_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ge_euint8_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function gt_euint8_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function le_euint8_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function lt_euint8_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function min_euint8_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint8 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        res8 = result;
    }
    function max_euint8_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint8 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        res8 = result;
    }
    function add_euint8_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint16 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        res16 = result;
    }
    function sub_euint8_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint16 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        res16 = result;
    }
    function mul_euint8_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint16 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        res16 = result;
    }
    function and_euint8_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint16 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        res16 = result;
    }
    function or_euint8_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint16 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        res16 = result;
    }
    function xor_euint8_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint16 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        res16 = result;
    }
    function eq_euint8_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ne_euint8_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ge_euint8_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function gt_euint8_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function le_euint8_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function lt_euint8_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function min_euint8_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint16 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        res16 = result;
    }
    function max_euint8_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint16 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        res16 = result;
    }
    function add_euint8_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        res32 = result;
    }
    function sub_euint8_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        res32 = result;
    }
    function mul_euint8_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        res32 = result;
    }
    function and_euint8_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        res32 = result;
    }
    function or_euint8_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        res32 = result;
    }
    function xor_euint8_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        res32 = result;
    }
    function eq_euint8_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ne_euint8_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ge_euint8_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function gt_euint8_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function le_euint8_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function lt_euint8_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function min_euint8_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        res32 = result;
    }
    function max_euint8_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        res32 = result;
    }
    function add_euint8_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function sub_euint8_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function mul_euint8_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function and_euint8_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function or_euint8_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function xor_euint8_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function eq_euint8_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ne_euint8_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ge_euint8_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function gt_euint8_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function le_euint8_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function lt_euint8_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function min_euint8_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function max_euint8_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function add_euint8_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        uint8 bProc = b;
        euint8 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        res8 = result;
    }
    function add_uint8_euint8(uint8 a, einput b, bytes calldata inputProof) public {
        uint8 aProc = a;
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint8 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        res8 = result;
    }
    function sub_euint8_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        uint8 bProc = b;
        euint8 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        res8 = result;
    }
    function sub_uint8_euint8(uint8 a, einput b, bytes calldata inputProof) public {
        uint8 aProc = a;
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint8 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        res8 = result;
    }
    function mul_euint8_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        uint8 bProc = b;
        euint8 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        res8 = result;
    }
    function mul_uint8_euint8(uint8 a, einput b, bytes calldata inputProof) public {
        uint8 aProc = a;
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint8 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        res8 = result;
    }
    function div_euint8_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        uint8 bProc = b;
        euint8 result = TFHE.div(aProc, bProc);
        TFHE.allowThis(result);
        res8 = result;
    }
    function rem_euint8_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        uint8 bProc = b;
        euint8 result = TFHE.rem(aProc, bProc);
        TFHE.allowThis(result);
        res8 = result;
    }
    function eq_euint8_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        uint8 bProc = b;
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function eq_uint8_euint8(uint8 a, einput b, bytes calldata inputProof) public {
        uint8 aProc = a;
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ne_euint8_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        uint8 bProc = b;
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ne_uint8_euint8(uint8 a, einput b, bytes calldata inputProof) public {
        uint8 aProc = a;
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ge_euint8_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        uint8 bProc = b;
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ge_uint8_euint8(uint8 a, einput b, bytes calldata inputProof) public {
        uint8 aProc = a;
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function gt_euint8_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        uint8 bProc = b;
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function gt_uint8_euint8(uint8 a, einput b, bytes calldata inputProof) public {
        uint8 aProc = a;
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function le_euint8_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        uint8 bProc = b;
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function le_uint8_euint8(uint8 a, einput b, bytes calldata inputProof) public {
        uint8 aProc = a;
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function lt_euint8_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        uint8 bProc = b;
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function lt_uint8_euint8(uint8 a, einput b, bytes calldata inputProof) public {
        uint8 aProc = a;
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function min_euint8_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        uint8 bProc = b;
        euint8 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        res8 = result;
    }
    function min_uint8_euint8(uint8 a, einput b, bytes calldata inputProof) public {
        uint8 aProc = a;
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint8 result = TFHE.min(aProc, bProc);
        TFHE.allowThis(result);
        res8 = result;
    }
    function max_euint8_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        uint8 bProc = b;
        euint8 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        res8 = result;
    }
    function max_uint8_euint8(uint8 a, einput b, bytes calldata inputProof) public {
        uint8 aProc = a;
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint8 result = TFHE.max(aProc, bProc);
        TFHE.allowThis(result);
        res8 = result;
    }
    function add_euint16_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint16 result = TFHE.add(aProc, bProc);
        TFHE.allowThis(result);
        res16 = result;
    }
    function sub_euint16_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint16 result = TFHE.sub(aProc, bProc);
        TFHE.allowThis(result);
        res16 = result;
    }
    function mul_euint16_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint16 result = TFHE.mul(aProc, bProc);
        TFHE.allowThis(result);
        res16 = result;
    }
    function and_euint16_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint16 result = TFHE.and(aProc, bProc);
        TFHE.allowThis(result);
        res16 = result;
    }
    function or_euint16_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint16 result = TFHE.or(aProc, bProc);
        TFHE.allowThis(result);
        res16 = result;
    }
    function xor_euint16_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint16 result = TFHE.xor(aProc, bProc);
        TFHE.allowThis(result);
        res16 = result;
    }
    function eq_euint16_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ne_euint16_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function ge_euint16_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function gt_euint16_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function le_euint16_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
    function lt_euint16_euint4(einput a, einput b, bytes calldata inputProof) public {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allowThis(result);
        resb = result;
    }
}
