// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../../lib/TFHE.sol";
contract TFHETestSuite5 {
    ebool public resb;
    euint4 public res4;
    euint8 public res8;
    euint16 public res16;
    euint32 public res32;
    euint64 public res64;

    function le_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allow(result, address(this));
        resb = result;
    }
    function lt_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allow(result, address(this));
        resb = result;
    }
    function min_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint64 result = TFHE.min(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function max_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint64 result = TFHE.max(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function add_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint64 result = TFHE.add(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function sub_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint64 result = TFHE.sub(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function mul_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint64 result = TFHE.mul(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function and_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint64 result = TFHE.and(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function or_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint64 result = TFHE.or(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function xor_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint64 result = TFHE.xor(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function eq_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allow(result, address(this));
        resb = result;
    }
    function ne_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allow(result, address(this));
        resb = result;
    }
    function ge_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allow(result, address(this));
        resb = result;
    }
    function gt_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allow(result, address(this));
        resb = result;
    }
    function le_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allow(result, address(this));
        resb = result;
    }
    function lt_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allow(result, address(this));
        resb = result;
    }
    function min_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint64 result = TFHE.min(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function max_euint64_euint16(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint64 result = TFHE.max(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function add_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint64 result = TFHE.add(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function sub_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint64 result = TFHE.sub(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function mul_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint64 result = TFHE.mul(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function and_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint64 result = TFHE.and(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function or_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint64 result = TFHE.or(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function xor_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint64 result = TFHE.xor(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function eq_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allow(result, address(this));
        resb = result;
    }
    function ne_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allow(result, address(this));
        resb = result;
    }
    function ge_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allow(result, address(this));
        resb = result;
    }
    function gt_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allow(result, address(this));
        resb = result;
    }
    function le_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allow(result, address(this));
        resb = result;
    }
    function lt_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allow(result, address(this));
        resb = result;
    }
    function min_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint64 result = TFHE.min(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function max_euint64_euint32(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint64 result = TFHE.max(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function add_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.add(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function sub_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.sub(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function mul_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.mul(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function and_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.and(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function or_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.or(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function xor_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.xor(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function eq_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allow(result, address(this));
        resb = result;
    }
    function ne_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allow(result, address(this));
        resb = result;
    }
    function ge_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allow(result, address(this));
        resb = result;
    }
    function gt_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allow(result, address(this));
        resb = result;
    }
    function le_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allow(result, address(this));
        resb = result;
    }
    function lt_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allow(result, address(this));
        resb = result;
    }
    function min_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.min(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function max_euint64_euint64(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.max(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function add_euint64_uint64(einput a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint64 bProc = b;
        euint64 result = TFHE.add(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function add_uint64_euint64(uint64 a, einput b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.add(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function sub_euint64_uint64(einput a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint64 bProc = b;
        euint64 result = TFHE.sub(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function sub_uint64_euint64(uint64 a, einput b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.sub(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function mul_euint64_uint64(einput a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint64 bProc = b;
        euint64 result = TFHE.mul(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function mul_uint64_euint64(uint64 a, einput b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.mul(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function div_euint64_uint64(einput a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint64 bProc = b;
        euint64 result = TFHE.div(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function rem_euint64_uint64(einput a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint64 bProc = b;
        euint64 result = TFHE.rem(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function eq_euint64_uint64(einput a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint64 bProc = b;
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allow(result, address(this));
        resb = result;
    }
    function eq_uint64_euint64(uint64 a, einput b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        TFHE.allow(result, address(this));
        resb = result;
    }
    function ne_euint64_uint64(einput a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint64 bProc = b;
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allow(result, address(this));
        resb = result;
    }
    function ne_uint64_euint64(uint64 a, einput b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        TFHE.allow(result, address(this));
        resb = result;
    }
    function ge_euint64_uint64(einput a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint64 bProc = b;
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allow(result, address(this));
        resb = result;
    }
    function ge_uint64_euint64(uint64 a, einput b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        TFHE.allow(result, address(this));
        resb = result;
    }
    function gt_euint64_uint64(einput a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint64 bProc = b;
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allow(result, address(this));
        resb = result;
    }
    function gt_uint64_euint64(uint64 a, einput b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        TFHE.allow(result, address(this));
        resb = result;
    }
    function le_euint64_uint64(einput a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint64 bProc = b;
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allow(result, address(this));
        resb = result;
    }
    function le_uint64_euint64(uint64 a, einput b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        TFHE.allow(result, address(this));
        resb = result;
    }
    function lt_euint64_uint64(einput a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint64 bProc = b;
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allow(result, address(this));
        resb = result;
    }
    function lt_uint64_euint64(uint64 a, einput b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        TFHE.allow(result, address(this));
        resb = result;
    }
    function min_euint64_uint64(einput a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint64 bProc = b;
        euint64 result = TFHE.min(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function min_uint64_euint64(uint64 a, einput b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.min(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function max_euint64_uint64(einput a, uint64 b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint64 bProc = b;
        euint64 result = TFHE.max(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function max_uint64_euint64(uint64 a, einput b, bytes calldata inputProof) public {
        uint64 aProc = a;
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.max(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function shl_euint4_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        uint8 bProc = b;
        euint4 result = TFHE.shl(aProc, bProc);
        TFHE.allow(result, address(this));
        res4 = result;
    }
    function shr_euint4_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        uint8 bProc = b;
        euint4 result = TFHE.shr(aProc, bProc);
        TFHE.allow(result, address(this));
        res4 = result;
    }
    function rotl_euint4_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        uint8 bProc = b;
        euint4 result = TFHE.rotl(aProc, bProc);
        TFHE.allow(result, address(this));
        res4 = result;
    }
    function rotr_euint4_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        uint8 bProc = b;
        euint4 result = TFHE.rotr(aProc, bProc);
        TFHE.allow(result, address(this));
        res4 = result;
    }
    function shl_euint8_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint8 result = TFHE.shl(aProc, bProc);
        TFHE.allow(result, address(this));
        res8 = result;
    }
    function shl_euint8_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        uint8 bProc = b;
        euint8 result = TFHE.shl(aProc, bProc);
        TFHE.allow(result, address(this));
        res8 = result;
    }
    function shr_euint8_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint8 result = TFHE.shr(aProc, bProc);
        TFHE.allow(result, address(this));
        res8 = result;
    }
    function shr_euint8_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        uint8 bProc = b;
        euint8 result = TFHE.shr(aProc, bProc);
        TFHE.allow(result, address(this));
        res8 = result;
    }
    function rotl_euint8_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint8 result = TFHE.rotl(aProc, bProc);
        TFHE.allow(result, address(this));
        res8 = result;
    }
    function rotl_euint8_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        uint8 bProc = b;
        euint8 result = TFHE.rotl(aProc, bProc);
        TFHE.allow(result, address(this));
        res8 = result;
    }
    function rotr_euint8_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint8 result = TFHE.rotr(aProc, bProc);
        TFHE.allow(result, address(this));
        res8 = result;
    }
    function rotr_euint8_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        uint8 bProc = b;
        euint8 result = TFHE.rotr(aProc, bProc);
        TFHE.allow(result, address(this));
        res8 = result;
    }
    function shl_euint16_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint16 result = TFHE.shl(aProc, bProc);
        TFHE.allow(result, address(this));
        res16 = result;
    }
    function shl_euint16_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        uint8 bProc = b;
        euint16 result = TFHE.shl(aProc, bProc);
        TFHE.allow(result, address(this));
        res16 = result;
    }
    function shr_euint16_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint16 result = TFHE.shr(aProc, bProc);
        TFHE.allow(result, address(this));
        res16 = result;
    }
    function shr_euint16_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        uint8 bProc = b;
        euint16 result = TFHE.shr(aProc, bProc);
        TFHE.allow(result, address(this));
        res16 = result;
    }
    function rotl_euint16_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint16 result = TFHE.rotl(aProc, bProc);
        TFHE.allow(result, address(this));
        res16 = result;
    }
    function rotl_euint16_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        uint8 bProc = b;
        euint16 result = TFHE.rotl(aProc, bProc);
        TFHE.allow(result, address(this));
        res16 = result;
    }
    function rotr_euint16_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint16 result = TFHE.rotr(aProc, bProc);
        TFHE.allow(result, address(this));
        res16 = result;
    }
    function rotr_euint16_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        uint8 bProc = b;
        euint16 result = TFHE.rotr(aProc, bProc);
        TFHE.allow(result, address(this));
        res16 = result;
    }
    function shl_euint32_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint32 result = TFHE.shl(aProc, bProc);
        TFHE.allow(result, address(this));
        res32 = result;
    }
    function shl_euint32_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        uint8 bProc = b;
        euint32 result = TFHE.shl(aProc, bProc);
        TFHE.allow(result, address(this));
        res32 = result;
    }
    function shr_euint32_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint32 result = TFHE.shr(aProc, bProc);
        TFHE.allow(result, address(this));
        res32 = result;
    }
    function shr_euint32_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        uint8 bProc = b;
        euint32 result = TFHE.shr(aProc, bProc);
        TFHE.allow(result, address(this));
        res32 = result;
    }
    function rotl_euint32_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint32 result = TFHE.rotl(aProc, bProc);
        TFHE.allow(result, address(this));
        res32 = result;
    }
    function rotl_euint32_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        uint8 bProc = b;
        euint32 result = TFHE.rotl(aProc, bProc);
        TFHE.allow(result, address(this));
        res32 = result;
    }
    function rotr_euint32_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint32 result = TFHE.rotr(aProc, bProc);
        TFHE.allow(result, address(this));
        res32 = result;
    }
    function rotr_euint32_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        uint8 bProc = b;
        euint32 result = TFHE.rotr(aProc, bProc);
        TFHE.allow(result, address(this));
        res32 = result;
    }
    function shl_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint64 result = TFHE.shl(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
    function shl_euint64_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint8 bProc = b;
        euint64 result = TFHE.shl(aProc, bProc);
        TFHE.allow(result, address(this));
        res64 = result;
    }
}
