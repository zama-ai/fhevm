// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.20;

import "../../lib/TFHE.sol";

contract TFHETestSuite5 {
    function le_euint64_euint8(bytes calldata a, bytes calldata b) public returns (bool) {
        euint64 aProc = TFHE.asEuint64(a);
        euint8 bProc = TFHE.asEuint8(b);
        ebool result = TFHE.le(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function lt_euint64_euint8(bytes calldata a, bytes calldata b) public returns (bool) {
        euint64 aProc = TFHE.asEuint64(a);
        euint8 bProc = TFHE.asEuint8(b);
        ebool result = TFHE.lt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function min_euint64_euint8(bytes calldata a, bytes calldata b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint64 result = TFHE.min(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function max_euint64_euint8(bytes calldata a, bytes calldata b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint64 result = TFHE.max(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function add_euint64_euint16(bytes calldata a, bytes calldata b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint16 bProc = TFHE.asEuint16(b);
        euint64 result = TFHE.add(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function sub_euint64_euint16(bytes calldata a, bytes calldata b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint16 bProc = TFHE.asEuint16(b);
        euint64 result = TFHE.sub(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function mul_euint64_euint16(bytes calldata a, bytes calldata b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint16 bProc = TFHE.asEuint16(b);
        euint64 result = TFHE.mul(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function and_euint64_euint16(bytes calldata a, bytes calldata b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint16 bProc = TFHE.asEuint16(b);
        euint64 result = TFHE.and(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function or_euint64_euint16(bytes calldata a, bytes calldata b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint16 bProc = TFHE.asEuint16(b);
        euint64 result = TFHE.or(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function xor_euint64_euint16(bytes calldata a, bytes calldata b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint16 bProc = TFHE.asEuint16(b);
        euint64 result = TFHE.xor(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function eq_euint64_euint16(bytes calldata a, bytes calldata b) public returns (bool) {
        euint64 aProc = TFHE.asEuint64(a);
        euint16 bProc = TFHE.asEuint16(b);
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ne_euint64_euint16(bytes calldata a, bytes calldata b) public returns (bool) {
        euint64 aProc = TFHE.asEuint64(a);
        euint16 bProc = TFHE.asEuint16(b);
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ge_euint64_euint16(bytes calldata a, bytes calldata b) public returns (bool) {
        euint64 aProc = TFHE.asEuint64(a);
        euint16 bProc = TFHE.asEuint16(b);
        ebool result = TFHE.ge(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function gt_euint64_euint16(bytes calldata a, bytes calldata b) public returns (bool) {
        euint64 aProc = TFHE.asEuint64(a);
        euint16 bProc = TFHE.asEuint16(b);
        ebool result = TFHE.gt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function le_euint64_euint16(bytes calldata a, bytes calldata b) public returns (bool) {
        euint64 aProc = TFHE.asEuint64(a);
        euint16 bProc = TFHE.asEuint16(b);
        ebool result = TFHE.le(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function lt_euint64_euint16(bytes calldata a, bytes calldata b) public returns (bool) {
        euint64 aProc = TFHE.asEuint64(a);
        euint16 bProc = TFHE.asEuint16(b);
        ebool result = TFHE.lt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function min_euint64_euint16(bytes calldata a, bytes calldata b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint16 bProc = TFHE.asEuint16(b);
        euint64 result = TFHE.min(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function max_euint64_euint16(bytes calldata a, bytes calldata b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint16 bProc = TFHE.asEuint16(b);
        euint64 result = TFHE.max(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function add_euint64_euint32(bytes calldata a, bytes calldata b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint32 bProc = TFHE.asEuint32(b);
        euint64 result = TFHE.add(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function sub_euint64_euint32(bytes calldata a, bytes calldata b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint32 bProc = TFHE.asEuint32(b);
        euint64 result = TFHE.sub(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function mul_euint64_euint32(bytes calldata a, bytes calldata b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint32 bProc = TFHE.asEuint32(b);
        euint64 result = TFHE.mul(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function and_euint64_euint32(bytes calldata a, bytes calldata b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint32 bProc = TFHE.asEuint32(b);
        euint64 result = TFHE.and(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function or_euint64_euint32(bytes calldata a, bytes calldata b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint32 bProc = TFHE.asEuint32(b);
        euint64 result = TFHE.or(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function xor_euint64_euint32(bytes calldata a, bytes calldata b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint32 bProc = TFHE.asEuint32(b);
        euint64 result = TFHE.xor(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function eq_euint64_euint32(bytes calldata a, bytes calldata b) public returns (bool) {
        euint64 aProc = TFHE.asEuint64(a);
        euint32 bProc = TFHE.asEuint32(b);
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ne_euint64_euint32(bytes calldata a, bytes calldata b) public returns (bool) {
        euint64 aProc = TFHE.asEuint64(a);
        euint32 bProc = TFHE.asEuint32(b);
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ge_euint64_euint32(bytes calldata a, bytes calldata b) public returns (bool) {
        euint64 aProc = TFHE.asEuint64(a);
        euint32 bProc = TFHE.asEuint32(b);
        ebool result = TFHE.ge(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function gt_euint64_euint32(bytes calldata a, bytes calldata b) public returns (bool) {
        euint64 aProc = TFHE.asEuint64(a);
        euint32 bProc = TFHE.asEuint32(b);
        ebool result = TFHE.gt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function le_euint64_euint32(bytes calldata a, bytes calldata b) public returns (bool) {
        euint64 aProc = TFHE.asEuint64(a);
        euint32 bProc = TFHE.asEuint32(b);
        ebool result = TFHE.le(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function lt_euint64_euint32(bytes calldata a, bytes calldata b) public returns (bool) {
        euint64 aProc = TFHE.asEuint64(a);
        euint32 bProc = TFHE.asEuint32(b);
        ebool result = TFHE.lt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function min_euint64_euint32(bytes calldata a, bytes calldata b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint32 bProc = TFHE.asEuint32(b);
        euint64 result = TFHE.min(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function max_euint64_euint32(bytes calldata a, bytes calldata b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint32 bProc = TFHE.asEuint32(b);
        euint64 result = TFHE.max(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function add_euint64_euint64(bytes calldata a, bytes calldata b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint64 bProc = TFHE.asEuint64(b);
        euint64 result = TFHE.add(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function sub_euint64_euint64(bytes calldata a, bytes calldata b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint64 bProc = TFHE.asEuint64(b);
        euint64 result = TFHE.sub(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function mul_euint64_euint64(bytes calldata a, bytes calldata b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint64 bProc = TFHE.asEuint64(b);
        euint64 result = TFHE.mul(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function and_euint64_euint64(bytes calldata a, bytes calldata b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint64 bProc = TFHE.asEuint64(b);
        euint64 result = TFHE.and(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function or_euint64_euint64(bytes calldata a, bytes calldata b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint64 bProc = TFHE.asEuint64(b);
        euint64 result = TFHE.or(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function xor_euint64_euint64(bytes calldata a, bytes calldata b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint64 bProc = TFHE.asEuint64(b);
        euint64 result = TFHE.xor(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function eq_euint64_euint64(bytes calldata a, bytes calldata b) public returns (bool) {
        euint64 aProc = TFHE.asEuint64(a);
        euint64 bProc = TFHE.asEuint64(b);
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ne_euint64_euint64(bytes calldata a, bytes calldata b) public returns (bool) {
        euint64 aProc = TFHE.asEuint64(a);
        euint64 bProc = TFHE.asEuint64(b);
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ge_euint64_euint64(bytes calldata a, bytes calldata b) public returns (bool) {
        euint64 aProc = TFHE.asEuint64(a);
        euint64 bProc = TFHE.asEuint64(b);
        ebool result = TFHE.ge(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function gt_euint64_euint64(bytes calldata a, bytes calldata b) public returns (bool) {
        euint64 aProc = TFHE.asEuint64(a);
        euint64 bProc = TFHE.asEuint64(b);
        ebool result = TFHE.gt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function le_euint64_euint64(bytes calldata a, bytes calldata b) public returns (bool) {
        euint64 aProc = TFHE.asEuint64(a);
        euint64 bProc = TFHE.asEuint64(b);
        ebool result = TFHE.le(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function lt_euint64_euint64(bytes calldata a, bytes calldata b) public returns (bool) {
        euint64 aProc = TFHE.asEuint64(a);
        euint64 bProc = TFHE.asEuint64(b);
        ebool result = TFHE.lt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function min_euint64_euint64(bytes calldata a, bytes calldata b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint64 bProc = TFHE.asEuint64(b);
        euint64 result = TFHE.min(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function max_euint64_euint64(bytes calldata a, bytes calldata b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint64 bProc = TFHE.asEuint64(b);
        euint64 result = TFHE.max(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function add_euint64_uint64(bytes calldata a, uint64 b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        uint64 bProc = b;
        euint64 result = TFHE.add(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function add_uint64_euint64(uint64 a, bytes calldata b) public returns (uint64) {
        uint64 aProc = a;
        euint64 bProc = TFHE.asEuint64(b);
        euint64 result = TFHE.add(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function sub_euint64_uint64(bytes calldata a, uint64 b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        uint64 bProc = b;
        euint64 result = TFHE.sub(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function sub_uint64_euint64(uint64 a, bytes calldata b) public returns (uint64) {
        uint64 aProc = a;
        euint64 bProc = TFHE.asEuint64(b);
        euint64 result = TFHE.sub(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function mul_euint64_uint64(bytes calldata a, uint64 b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        uint64 bProc = b;
        euint64 result = TFHE.mul(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function mul_uint64_euint64(uint64 a, bytes calldata b) public returns (uint64) {
        uint64 aProc = a;
        euint64 bProc = TFHE.asEuint64(b);
        euint64 result = TFHE.mul(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function div_euint64_uint64(bytes calldata a, uint64 b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        uint64 bProc = b;
        euint64 result = TFHE.div(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function rem_euint64_uint64(bytes calldata a, uint64 b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        uint64 bProc = b;
        euint64 result = TFHE.rem(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function eq_euint64_uint64(bytes calldata a, uint64 b) public returns (bool) {
        euint64 aProc = TFHE.asEuint64(a);
        uint64 bProc = b;
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function eq_uint64_euint64(uint64 a, bytes calldata b) public returns (bool) {
        uint64 aProc = a;
        euint64 bProc = TFHE.asEuint64(b);
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ne_euint64_uint64(bytes calldata a, uint64 b) public returns (bool) {
        euint64 aProc = TFHE.asEuint64(a);
        uint64 bProc = b;
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ne_uint64_euint64(uint64 a, bytes calldata b) public returns (bool) {
        uint64 aProc = a;
        euint64 bProc = TFHE.asEuint64(b);
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ge_euint64_uint64(bytes calldata a, uint64 b) public returns (bool) {
        euint64 aProc = TFHE.asEuint64(a);
        uint64 bProc = b;
        ebool result = TFHE.ge(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ge_uint64_euint64(uint64 a, bytes calldata b) public returns (bool) {
        uint64 aProc = a;
        euint64 bProc = TFHE.asEuint64(b);
        ebool result = TFHE.ge(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function gt_euint64_uint64(bytes calldata a, uint64 b) public returns (bool) {
        euint64 aProc = TFHE.asEuint64(a);
        uint64 bProc = b;
        ebool result = TFHE.gt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function gt_uint64_euint64(uint64 a, bytes calldata b) public returns (bool) {
        uint64 aProc = a;
        euint64 bProc = TFHE.asEuint64(b);
        ebool result = TFHE.gt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function le_euint64_uint64(bytes calldata a, uint64 b) public returns (bool) {
        euint64 aProc = TFHE.asEuint64(a);
        uint64 bProc = b;
        ebool result = TFHE.le(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function le_uint64_euint64(uint64 a, bytes calldata b) public returns (bool) {
        uint64 aProc = a;
        euint64 bProc = TFHE.asEuint64(b);
        ebool result = TFHE.le(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function lt_euint64_uint64(bytes calldata a, uint64 b) public returns (bool) {
        euint64 aProc = TFHE.asEuint64(a);
        uint64 bProc = b;
        ebool result = TFHE.lt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function lt_uint64_euint64(uint64 a, bytes calldata b) public returns (bool) {
        uint64 aProc = a;
        euint64 bProc = TFHE.asEuint64(b);
        ebool result = TFHE.lt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function min_euint64_uint64(bytes calldata a, uint64 b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        uint64 bProc = b;
        euint64 result = TFHE.min(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function min_uint64_euint64(uint64 a, bytes calldata b) public returns (uint64) {
        uint64 aProc = a;
        euint64 bProc = TFHE.asEuint64(b);
        euint64 result = TFHE.min(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function max_euint64_uint64(bytes calldata a, uint64 b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        uint64 bProc = b;
        euint64 result = TFHE.max(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function max_uint64_euint64(uint64 a, bytes calldata b) public returns (uint64) {
        uint64 aProc = a;
        euint64 bProc = TFHE.asEuint64(b);
        euint64 result = TFHE.max(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shl_euint4_uint8(bytes calldata a, uint8 b) public returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a);
        uint8 bProc = b;
        euint4 result = TFHE.shl(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shr_euint4_uint8(bytes calldata a, uint8 b) public returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a);
        uint8 bProc = b;
        euint4 result = TFHE.shr(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function rotl_euint4_uint8(bytes calldata a, uint8 b) public returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a);
        uint8 bProc = b;
        euint4 result = TFHE.rotl(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function rotr_euint4_uint8(bytes calldata a, uint8 b) public returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a);
        uint8 bProc = b;
        euint4 result = TFHE.rotr(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shl_euint8_euint8(bytes calldata a, bytes calldata b) public returns (uint8) {
        euint8 aProc = TFHE.asEuint8(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint8 result = TFHE.shl(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shl_euint8_uint8(bytes calldata a, uint8 b) public returns (uint8) {
        euint8 aProc = TFHE.asEuint8(a);
        uint8 bProc = b;
        euint8 result = TFHE.shl(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shr_euint8_euint8(bytes calldata a, bytes calldata b) public returns (uint8) {
        euint8 aProc = TFHE.asEuint8(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint8 result = TFHE.shr(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shr_euint8_uint8(bytes calldata a, uint8 b) public returns (uint8) {
        euint8 aProc = TFHE.asEuint8(a);
        uint8 bProc = b;
        euint8 result = TFHE.shr(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function rotl_euint8_euint8(bytes calldata a, bytes calldata b) public returns (uint8) {
        euint8 aProc = TFHE.asEuint8(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint8 result = TFHE.rotl(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function rotl_euint8_uint8(bytes calldata a, uint8 b) public returns (uint8) {
        euint8 aProc = TFHE.asEuint8(a);
        uint8 bProc = b;
        euint8 result = TFHE.rotl(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function rotr_euint8_euint8(bytes calldata a, bytes calldata b) public returns (uint8) {
        euint8 aProc = TFHE.asEuint8(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint8 result = TFHE.rotr(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function rotr_euint8_uint8(bytes calldata a, uint8 b) public returns (uint8) {
        euint8 aProc = TFHE.asEuint8(a);
        uint8 bProc = b;
        euint8 result = TFHE.rotr(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shl_euint16_euint8(bytes calldata a, bytes calldata b) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint16 result = TFHE.shl(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shl_euint16_uint8(bytes calldata a, uint8 b) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a);
        uint8 bProc = b;
        euint16 result = TFHE.shl(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shr_euint16_euint8(bytes calldata a, bytes calldata b) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint16 result = TFHE.shr(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shr_euint16_uint8(bytes calldata a, uint8 b) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a);
        uint8 bProc = b;
        euint16 result = TFHE.shr(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function rotl_euint16_euint8(bytes calldata a, bytes calldata b) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint16 result = TFHE.rotl(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function rotl_euint16_uint8(bytes calldata a, uint8 b) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a);
        uint8 bProc = b;
        euint16 result = TFHE.rotl(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function rotr_euint16_euint8(bytes calldata a, bytes calldata b) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint16 result = TFHE.rotr(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function rotr_euint16_uint8(bytes calldata a, uint8 b) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a);
        uint8 bProc = b;
        euint16 result = TFHE.rotr(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shl_euint32_euint8(bytes calldata a, bytes calldata b) public returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint32 result = TFHE.shl(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shl_euint32_uint8(bytes calldata a, uint8 b) public returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        uint8 bProc = b;
        euint32 result = TFHE.shl(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shr_euint32_euint8(bytes calldata a, bytes calldata b) public returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint32 result = TFHE.shr(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shr_euint32_uint8(bytes calldata a, uint8 b) public returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        uint8 bProc = b;
        euint32 result = TFHE.shr(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function rotl_euint32_euint8(bytes calldata a, bytes calldata b) public returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint32 result = TFHE.rotl(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function rotl_euint32_uint8(bytes calldata a, uint8 b) public returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        uint8 bProc = b;
        euint32 result = TFHE.rotl(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function rotr_euint32_euint8(bytes calldata a, bytes calldata b) public returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint32 result = TFHE.rotr(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function rotr_euint32_uint8(bytes calldata a, uint8 b) public returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        uint8 bProc = b;
        euint32 result = TFHE.rotr(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shl_euint64_euint8(bytes calldata a, bytes calldata b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint64 result = TFHE.shl(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shl_euint64_uint8(bytes calldata a, uint8 b) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        uint8 bProc = b;
        euint64 result = TFHE.shl(aProc, bProc);
        return TFHE.decrypt(result);
    }
}
