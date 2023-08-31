// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity >=0.8.13 <0.8.20;

import "../lib/TFHE.sol";

contract TFHETestSuite1 {
    function add_euint8_euint8(uint8 a, uint8 b) public view returns (uint8) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint8 b_proc = TFHE.asEuint8(b);
        euint8 result = TFHE.add(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function sub_euint8_euint8(uint8 a, uint8 b) public view returns (uint8) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint8 b_proc = TFHE.asEuint8(b);
        euint8 result = TFHE.sub(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function mul_euint8_euint8(uint8 a, uint8 b) public view returns (uint8) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint8 b_proc = TFHE.asEuint8(b);
        euint8 result = TFHE.mul(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function and_euint8_euint8(uint8 a, uint8 b) public view returns (uint8) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint8 b_proc = TFHE.asEuint8(b);
        euint8 result = TFHE.and(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function or_euint8_euint8(uint8 a, uint8 b) public view returns (uint8) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint8 b_proc = TFHE.asEuint8(b);
        euint8 result = TFHE.or(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function xor_euint8_euint8(uint8 a, uint8 b) public view returns (uint8) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint8 b_proc = TFHE.asEuint8(b);
        euint8 result = TFHE.xor(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function shl_euint8_euint8(uint8 a, uint8 b) public view returns (uint8) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint8 b_proc = TFHE.asEuint8(b);
        euint8 result = TFHE.shl(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function shr_euint8_euint8(uint8 a, uint8 b) public view returns (uint8) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint8 b_proc = TFHE.asEuint8(b);
        euint8 result = TFHE.shr(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function eq_euint8_euint8(uint8 a, uint8 b) public view returns (bool) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint8 b_proc = TFHE.asEuint8(b);
        ebool result = TFHE.eq(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function ne_euint8_euint8(uint8 a, uint8 b) public view returns (bool) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint8 b_proc = TFHE.asEuint8(b);
        ebool result = TFHE.ne(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function ge_euint8_euint8(uint8 a, uint8 b) public view returns (bool) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint8 b_proc = TFHE.asEuint8(b);
        ebool result = TFHE.ge(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function gt_euint8_euint8(uint8 a, uint8 b) public view returns (bool) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint8 b_proc = TFHE.asEuint8(b);
        ebool result = TFHE.gt(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function le_euint8_euint8(uint8 a, uint8 b) public view returns (bool) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint8 b_proc = TFHE.asEuint8(b);
        ebool result = TFHE.le(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function lt_euint8_euint8(uint8 a, uint8 b) public view returns (bool) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint8 b_proc = TFHE.asEuint8(b);
        ebool result = TFHE.lt(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function min_euint8_euint8(uint8 a, uint8 b) public view returns (uint8) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint8 b_proc = TFHE.asEuint8(b);
        euint8 result = TFHE.min(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function max_euint8_euint8(uint8 a, uint8 b) public view returns (uint8) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint8 b_proc = TFHE.asEuint8(b);
        euint8 result = TFHE.max(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function add_euint8_euint16(uint8 a, uint16 b) public view returns (uint16) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint16 b_proc = TFHE.asEuint16(b);
        euint16 result = TFHE.add(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function sub_euint8_euint16(uint8 a, uint16 b) public view returns (uint16) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint16 b_proc = TFHE.asEuint16(b);
        euint16 result = TFHE.sub(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function mul_euint8_euint16(uint8 a, uint16 b) public view returns (uint16) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint16 b_proc = TFHE.asEuint16(b);
        euint16 result = TFHE.mul(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function and_euint8_euint16(uint8 a, uint16 b) public view returns (uint16) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint16 b_proc = TFHE.asEuint16(b);
        euint16 result = TFHE.and(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function or_euint8_euint16(uint8 a, uint16 b) public view returns (uint16) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint16 b_proc = TFHE.asEuint16(b);
        euint16 result = TFHE.or(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function xor_euint8_euint16(uint8 a, uint16 b) public view returns (uint16) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint16 b_proc = TFHE.asEuint16(b);
        euint16 result = TFHE.xor(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function shl_euint8_euint16(uint8 a, uint16 b) public view returns (uint16) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint16 b_proc = TFHE.asEuint16(b);
        euint16 result = TFHE.shl(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function shr_euint8_euint16(uint8 a, uint16 b) public view returns (uint16) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint16 b_proc = TFHE.asEuint16(b);
        euint16 result = TFHE.shr(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function eq_euint8_euint16(uint8 a, uint16 b) public view returns (bool) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint16 b_proc = TFHE.asEuint16(b);
        ebool result = TFHE.eq(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function ne_euint8_euint16(uint8 a, uint16 b) public view returns (bool) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint16 b_proc = TFHE.asEuint16(b);
        ebool result = TFHE.ne(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function ge_euint8_euint16(uint8 a, uint16 b) public view returns (bool) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint16 b_proc = TFHE.asEuint16(b);
        ebool result = TFHE.ge(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function gt_euint8_euint16(uint8 a, uint16 b) public view returns (bool) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint16 b_proc = TFHE.asEuint16(b);
        ebool result = TFHE.gt(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function le_euint8_euint16(uint8 a, uint16 b) public view returns (bool) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint16 b_proc = TFHE.asEuint16(b);
        ebool result = TFHE.le(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function lt_euint8_euint16(uint8 a, uint16 b) public view returns (bool) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint16 b_proc = TFHE.asEuint16(b);
        ebool result = TFHE.lt(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function min_euint8_euint16(uint8 a, uint16 b) public view returns (uint16) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint16 b_proc = TFHE.asEuint16(b);
        euint16 result = TFHE.min(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function max_euint8_euint16(uint8 a, uint16 b) public view returns (uint16) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint16 b_proc = TFHE.asEuint16(b);
        euint16 result = TFHE.max(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function add_euint8_euint32(uint8 a, uint32 b) public view returns (uint32) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.add(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function sub_euint8_euint32(uint8 a, uint32 b) public view returns (uint32) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.sub(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function mul_euint8_euint32(uint8 a, uint32 b) public view returns (uint32) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.mul(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function and_euint8_euint32(uint8 a, uint32 b) public view returns (uint32) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.and(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function or_euint8_euint32(uint8 a, uint32 b) public view returns (uint32) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.or(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function xor_euint8_euint32(uint8 a, uint32 b) public view returns (uint32) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.xor(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function shl_euint8_euint32(uint8 a, uint32 b) public view returns (uint32) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.shl(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function shr_euint8_euint32(uint8 a, uint32 b) public view returns (uint32) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.shr(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function eq_euint8_euint32(uint8 a, uint32 b) public view returns (bool) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint32 b_proc = TFHE.asEuint32(b);
        ebool result = TFHE.eq(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function ne_euint8_euint32(uint8 a, uint32 b) public view returns (bool) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint32 b_proc = TFHE.asEuint32(b);
        ebool result = TFHE.ne(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function ge_euint8_euint32(uint8 a, uint32 b) public view returns (bool) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint32 b_proc = TFHE.asEuint32(b);
        ebool result = TFHE.ge(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function gt_euint8_euint32(uint8 a, uint32 b) public view returns (bool) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint32 b_proc = TFHE.asEuint32(b);
        ebool result = TFHE.gt(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function le_euint8_euint32(uint8 a, uint32 b) public view returns (bool) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint32 b_proc = TFHE.asEuint32(b);
        ebool result = TFHE.le(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function lt_euint8_euint32(uint8 a, uint32 b) public view returns (bool) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint32 b_proc = TFHE.asEuint32(b);
        ebool result = TFHE.lt(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function min_euint8_euint32(uint8 a, uint32 b) public view returns (uint32) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.min(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function max_euint8_euint32(uint8 a, uint32 b) public view returns (uint32) {
        euint8 a_proc = TFHE.asEuint8(a);
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.max(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function add_euint8_uint8(uint8 a, uint8 b) public view returns (uint8) {
        euint8 a_proc = TFHE.asEuint8(a);
        uint8 b_proc = b;
        euint8 result = TFHE.add(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function add_uint8_euint8(uint8 a, uint8 b) public view returns (uint8) {
        uint8 a_proc = a;
        euint8 b_proc = TFHE.asEuint8(b);
        euint8 result = TFHE.add(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function sub_euint8_uint8(uint8 a, uint8 b) public view returns (uint8) {
        euint8 a_proc = TFHE.asEuint8(a);
        uint8 b_proc = b;
        euint8 result = TFHE.sub(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function sub_uint8_euint8(uint8 a, uint8 b) public view returns (uint8) {
        uint8 a_proc = a;
        euint8 b_proc = TFHE.asEuint8(b);
        euint8 result = TFHE.sub(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function mul_euint8_uint8(uint8 a, uint8 b) public view returns (uint8) {
        euint8 a_proc = TFHE.asEuint8(a);
        uint8 b_proc = b;
        euint8 result = TFHE.mul(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function mul_uint8_euint8(uint8 a, uint8 b) public view returns (uint8) {
        uint8 a_proc = a;
        euint8 b_proc = TFHE.asEuint8(b);
        euint8 result = TFHE.mul(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function div_euint8_uint8(uint8 a, uint8 b) public view returns (uint8) {
        euint8 a_proc = TFHE.asEuint8(a);
        uint8 b_proc = b;
        euint8 result = TFHE.div(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function shl_euint8_uint8(uint8 a, uint8 b) public view returns (uint8) {
        euint8 a_proc = TFHE.asEuint8(a);
        uint8 b_proc = b;
        euint8 result = TFHE.shl(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function shl_uint8_euint8(uint8 a, uint8 b) public view returns (uint8) {
        uint8 a_proc = a;
        euint8 b_proc = TFHE.asEuint8(b);
        euint8 result = TFHE.shl(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function shr_euint8_uint8(uint8 a, uint8 b) public view returns (uint8) {
        euint8 a_proc = TFHE.asEuint8(a);
        uint8 b_proc = b;
        euint8 result = TFHE.shr(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function shr_uint8_euint8(uint8 a, uint8 b) public view returns (uint8) {
        uint8 a_proc = a;
        euint8 b_proc = TFHE.asEuint8(b);
        euint8 result = TFHE.shr(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function eq_euint8_uint8(uint8 a, uint8 b) public view returns (bool) {
        euint8 a_proc = TFHE.asEuint8(a);
        uint8 b_proc = b;
        ebool result = TFHE.eq(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function eq_uint8_euint8(uint8 a, uint8 b) public view returns (bool) {
        uint8 a_proc = a;
        euint8 b_proc = TFHE.asEuint8(b);
        ebool result = TFHE.eq(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function ne_euint8_uint8(uint8 a, uint8 b) public view returns (bool) {
        euint8 a_proc = TFHE.asEuint8(a);
        uint8 b_proc = b;
        ebool result = TFHE.ne(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function ne_uint8_euint8(uint8 a, uint8 b) public view returns (bool) {
        uint8 a_proc = a;
        euint8 b_proc = TFHE.asEuint8(b);
        ebool result = TFHE.ne(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function ge_euint8_uint8(uint8 a, uint8 b) public view returns (bool) {
        euint8 a_proc = TFHE.asEuint8(a);
        uint8 b_proc = b;
        ebool result = TFHE.ge(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function ge_uint8_euint8(uint8 a, uint8 b) public view returns (bool) {
        uint8 a_proc = a;
        euint8 b_proc = TFHE.asEuint8(b);
        ebool result = TFHE.ge(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function gt_euint8_uint8(uint8 a, uint8 b) public view returns (bool) {
        euint8 a_proc = TFHE.asEuint8(a);
        uint8 b_proc = b;
        ebool result = TFHE.gt(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function gt_uint8_euint8(uint8 a, uint8 b) public view returns (bool) {
        uint8 a_proc = a;
        euint8 b_proc = TFHE.asEuint8(b);
        ebool result = TFHE.gt(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function le_euint8_uint8(uint8 a, uint8 b) public view returns (bool) {
        euint8 a_proc = TFHE.asEuint8(a);
        uint8 b_proc = b;
        ebool result = TFHE.le(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function le_uint8_euint8(uint8 a, uint8 b) public view returns (bool) {
        uint8 a_proc = a;
        euint8 b_proc = TFHE.asEuint8(b);
        ebool result = TFHE.le(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function lt_euint8_uint8(uint8 a, uint8 b) public view returns (bool) {
        euint8 a_proc = TFHE.asEuint8(a);
        uint8 b_proc = b;
        ebool result = TFHE.lt(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function lt_uint8_euint8(uint8 a, uint8 b) public view returns (bool) {
        uint8 a_proc = a;
        euint8 b_proc = TFHE.asEuint8(b);
        ebool result = TFHE.lt(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function min_euint8_uint8(uint8 a, uint8 b) public view returns (uint8) {
        euint8 a_proc = TFHE.asEuint8(a);
        uint8 b_proc = b;
        euint8 result = TFHE.min(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function min_uint8_euint8(uint8 a, uint8 b) public view returns (uint8) {
        uint8 a_proc = a;
        euint8 b_proc = TFHE.asEuint8(b);
        euint8 result = TFHE.min(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function max_euint8_uint8(uint8 a, uint8 b) public view returns (uint8) {
        euint8 a_proc = TFHE.asEuint8(a);
        uint8 b_proc = b;
        euint8 result = TFHE.max(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function max_uint8_euint8(uint8 a, uint8 b) public view returns (uint8) {
        uint8 a_proc = a;
        euint8 b_proc = TFHE.asEuint8(b);
        euint8 result = TFHE.max(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function add_euint16_euint8(uint16 a, uint8 b) public view returns (uint16) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint8 b_proc = TFHE.asEuint8(b);
        euint16 result = TFHE.add(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function sub_euint16_euint8(uint16 a, uint8 b) public view returns (uint16) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint8 b_proc = TFHE.asEuint8(b);
        euint16 result = TFHE.sub(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function mul_euint16_euint8(uint16 a, uint8 b) public view returns (uint16) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint8 b_proc = TFHE.asEuint8(b);
        euint16 result = TFHE.mul(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function and_euint16_euint8(uint16 a, uint8 b) public view returns (uint16) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint8 b_proc = TFHE.asEuint8(b);
        euint16 result = TFHE.and(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function or_euint16_euint8(uint16 a, uint8 b) public view returns (uint16) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint8 b_proc = TFHE.asEuint8(b);
        euint16 result = TFHE.or(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function xor_euint16_euint8(uint16 a, uint8 b) public view returns (uint16) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint8 b_proc = TFHE.asEuint8(b);
        euint16 result = TFHE.xor(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function shl_euint16_euint8(uint16 a, uint8 b) public view returns (uint16) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint8 b_proc = TFHE.asEuint8(b);
        euint16 result = TFHE.shl(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function shr_euint16_euint8(uint16 a, uint8 b) public view returns (uint16) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint8 b_proc = TFHE.asEuint8(b);
        euint16 result = TFHE.shr(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function eq_euint16_euint8(uint16 a, uint8 b) public view returns (bool) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint8 b_proc = TFHE.asEuint8(b);
        ebool result = TFHE.eq(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function ne_euint16_euint8(uint16 a, uint8 b) public view returns (bool) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint8 b_proc = TFHE.asEuint8(b);
        ebool result = TFHE.ne(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function ge_euint16_euint8(uint16 a, uint8 b) public view returns (bool) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint8 b_proc = TFHE.asEuint8(b);
        ebool result = TFHE.ge(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function gt_euint16_euint8(uint16 a, uint8 b) public view returns (bool) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint8 b_proc = TFHE.asEuint8(b);
        ebool result = TFHE.gt(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function le_euint16_euint8(uint16 a, uint8 b) public view returns (bool) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint8 b_proc = TFHE.asEuint8(b);
        ebool result = TFHE.le(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function lt_euint16_euint8(uint16 a, uint8 b) public view returns (bool) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint8 b_proc = TFHE.asEuint8(b);
        ebool result = TFHE.lt(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function min_euint16_euint8(uint16 a, uint8 b) public view returns (uint16) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint8 b_proc = TFHE.asEuint8(b);
        euint16 result = TFHE.min(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function max_euint16_euint8(uint16 a, uint8 b) public view returns (uint16) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint8 b_proc = TFHE.asEuint8(b);
        euint16 result = TFHE.max(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function add_euint16_euint16(uint16 a, uint16 b) public view returns (uint16) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint16 b_proc = TFHE.asEuint16(b);
        euint16 result = TFHE.add(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function sub_euint16_euint16(uint16 a, uint16 b) public view returns (uint16) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint16 b_proc = TFHE.asEuint16(b);
        euint16 result = TFHE.sub(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function mul_euint16_euint16(uint16 a, uint16 b) public view returns (uint16) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint16 b_proc = TFHE.asEuint16(b);
        euint16 result = TFHE.mul(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function and_euint16_euint16(uint16 a, uint16 b) public view returns (uint16) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint16 b_proc = TFHE.asEuint16(b);
        euint16 result = TFHE.and(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function or_euint16_euint16(uint16 a, uint16 b) public view returns (uint16) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint16 b_proc = TFHE.asEuint16(b);
        euint16 result = TFHE.or(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function xor_euint16_euint16(uint16 a, uint16 b) public view returns (uint16) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint16 b_proc = TFHE.asEuint16(b);
        euint16 result = TFHE.xor(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function shl_euint16_euint16(uint16 a, uint16 b) public view returns (uint16) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint16 b_proc = TFHE.asEuint16(b);
        euint16 result = TFHE.shl(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function shr_euint16_euint16(uint16 a, uint16 b) public view returns (uint16) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint16 b_proc = TFHE.asEuint16(b);
        euint16 result = TFHE.shr(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function eq_euint16_euint16(uint16 a, uint16 b) public view returns (bool) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint16 b_proc = TFHE.asEuint16(b);
        ebool result = TFHE.eq(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function ne_euint16_euint16(uint16 a, uint16 b) public view returns (bool) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint16 b_proc = TFHE.asEuint16(b);
        ebool result = TFHE.ne(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function ge_euint16_euint16(uint16 a, uint16 b) public view returns (bool) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint16 b_proc = TFHE.asEuint16(b);
        ebool result = TFHE.ge(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function gt_euint16_euint16(uint16 a, uint16 b) public view returns (bool) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint16 b_proc = TFHE.asEuint16(b);
        ebool result = TFHE.gt(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function le_euint16_euint16(uint16 a, uint16 b) public view returns (bool) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint16 b_proc = TFHE.asEuint16(b);
        ebool result = TFHE.le(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function lt_euint16_euint16(uint16 a, uint16 b) public view returns (bool) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint16 b_proc = TFHE.asEuint16(b);
        ebool result = TFHE.lt(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function min_euint16_euint16(uint16 a, uint16 b) public view returns (uint16) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint16 b_proc = TFHE.asEuint16(b);
        euint16 result = TFHE.min(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function max_euint16_euint16(uint16 a, uint16 b) public view returns (uint16) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint16 b_proc = TFHE.asEuint16(b);
        euint16 result = TFHE.max(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function add_euint16_euint32(uint16 a, uint32 b) public view returns (uint32) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.add(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function sub_euint16_euint32(uint16 a, uint32 b) public view returns (uint32) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.sub(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function mul_euint16_euint32(uint16 a, uint32 b) public view returns (uint32) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.mul(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function and_euint16_euint32(uint16 a, uint32 b) public view returns (uint32) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.and(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function or_euint16_euint32(uint16 a, uint32 b) public view returns (uint32) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.or(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function xor_euint16_euint32(uint16 a, uint32 b) public view returns (uint32) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.xor(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function shl_euint16_euint32(uint16 a, uint32 b) public view returns (uint32) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.shl(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function shr_euint16_euint32(uint16 a, uint32 b) public view returns (uint32) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.shr(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function eq_euint16_euint32(uint16 a, uint32 b) public view returns (bool) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint32 b_proc = TFHE.asEuint32(b);
        ebool result = TFHE.eq(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function ne_euint16_euint32(uint16 a, uint32 b) public view returns (bool) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint32 b_proc = TFHE.asEuint32(b);
        ebool result = TFHE.ne(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function ge_euint16_euint32(uint16 a, uint32 b) public view returns (bool) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint32 b_proc = TFHE.asEuint32(b);
        ebool result = TFHE.ge(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function gt_euint16_euint32(uint16 a, uint32 b) public view returns (bool) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint32 b_proc = TFHE.asEuint32(b);
        ebool result = TFHE.gt(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function le_euint16_euint32(uint16 a, uint32 b) public view returns (bool) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint32 b_proc = TFHE.asEuint32(b);
        ebool result = TFHE.le(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function lt_euint16_euint32(uint16 a, uint32 b) public view returns (bool) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint32 b_proc = TFHE.asEuint32(b);
        ebool result = TFHE.lt(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function min_euint16_euint32(uint16 a, uint32 b) public view returns (uint32) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.min(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function max_euint16_euint32(uint16 a, uint32 b) public view returns (uint32) {
        euint16 a_proc = TFHE.asEuint16(a);
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.max(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function add_euint16_uint16(uint16 a, uint16 b) public view returns (uint16) {
        euint16 a_proc = TFHE.asEuint16(a);
        uint16 b_proc = b;
        euint16 result = TFHE.add(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function add_uint16_euint16(uint16 a, uint16 b) public view returns (uint16) {
        uint16 a_proc = a;
        euint16 b_proc = TFHE.asEuint16(b);
        euint16 result = TFHE.add(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function sub_euint16_uint16(uint16 a, uint16 b) public view returns (uint16) {
        euint16 a_proc = TFHE.asEuint16(a);
        uint16 b_proc = b;
        euint16 result = TFHE.sub(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function sub_uint16_euint16(uint16 a, uint16 b) public view returns (uint16) {
        uint16 a_proc = a;
        euint16 b_proc = TFHE.asEuint16(b);
        euint16 result = TFHE.sub(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function mul_euint16_uint16(uint16 a, uint16 b) public view returns (uint16) {
        euint16 a_proc = TFHE.asEuint16(a);
        uint16 b_proc = b;
        euint16 result = TFHE.mul(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function mul_uint16_euint16(uint16 a, uint16 b) public view returns (uint16) {
        uint16 a_proc = a;
        euint16 b_proc = TFHE.asEuint16(b);
        euint16 result = TFHE.mul(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function div_euint16_uint16(uint16 a, uint16 b) public view returns (uint16) {
        euint16 a_proc = TFHE.asEuint16(a);
        uint16 b_proc = b;
        euint16 result = TFHE.div(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function shl_euint16_uint16(uint16 a, uint16 b) public view returns (uint16) {
        euint16 a_proc = TFHE.asEuint16(a);
        uint16 b_proc = b;
        euint16 result = TFHE.shl(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function shl_uint16_euint16(uint16 a, uint16 b) public view returns (uint16) {
        uint16 a_proc = a;
        euint16 b_proc = TFHE.asEuint16(b);
        euint16 result = TFHE.shl(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function shr_euint16_uint16(uint16 a, uint16 b) public view returns (uint16) {
        euint16 a_proc = TFHE.asEuint16(a);
        uint16 b_proc = b;
        euint16 result = TFHE.shr(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function shr_uint16_euint16(uint16 a, uint16 b) public view returns (uint16) {
        uint16 a_proc = a;
        euint16 b_proc = TFHE.asEuint16(b);
        euint16 result = TFHE.shr(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function eq_euint16_uint16(uint16 a, uint16 b) public view returns (bool) {
        euint16 a_proc = TFHE.asEuint16(a);
        uint16 b_proc = b;
        ebool result = TFHE.eq(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function eq_uint16_euint16(uint16 a, uint16 b) public view returns (bool) {
        uint16 a_proc = a;
        euint16 b_proc = TFHE.asEuint16(b);
        ebool result = TFHE.eq(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function ne_euint16_uint16(uint16 a, uint16 b) public view returns (bool) {
        euint16 a_proc = TFHE.asEuint16(a);
        uint16 b_proc = b;
        ebool result = TFHE.ne(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function ne_uint16_euint16(uint16 a, uint16 b) public view returns (bool) {
        uint16 a_proc = a;
        euint16 b_proc = TFHE.asEuint16(b);
        ebool result = TFHE.ne(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function ge_euint16_uint16(uint16 a, uint16 b) public view returns (bool) {
        euint16 a_proc = TFHE.asEuint16(a);
        uint16 b_proc = b;
        ebool result = TFHE.ge(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function ge_uint16_euint16(uint16 a, uint16 b) public view returns (bool) {
        uint16 a_proc = a;
        euint16 b_proc = TFHE.asEuint16(b);
        ebool result = TFHE.ge(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function gt_euint16_uint16(uint16 a, uint16 b) public view returns (bool) {
        euint16 a_proc = TFHE.asEuint16(a);
        uint16 b_proc = b;
        ebool result = TFHE.gt(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function gt_uint16_euint16(uint16 a, uint16 b) public view returns (bool) {
        uint16 a_proc = a;
        euint16 b_proc = TFHE.asEuint16(b);
        ebool result = TFHE.gt(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function le_euint16_uint16(uint16 a, uint16 b) public view returns (bool) {
        euint16 a_proc = TFHE.asEuint16(a);
        uint16 b_proc = b;
        ebool result = TFHE.le(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function le_uint16_euint16(uint16 a, uint16 b) public view returns (bool) {
        uint16 a_proc = a;
        euint16 b_proc = TFHE.asEuint16(b);
        ebool result = TFHE.le(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function lt_euint16_uint16(uint16 a, uint16 b) public view returns (bool) {
        euint16 a_proc = TFHE.asEuint16(a);
        uint16 b_proc = b;
        ebool result = TFHE.lt(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function lt_uint16_euint16(uint16 a, uint16 b) public view returns (bool) {
        uint16 a_proc = a;
        euint16 b_proc = TFHE.asEuint16(b);
        ebool result = TFHE.lt(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function min_euint16_uint16(uint16 a, uint16 b) public view returns (uint16) {
        euint16 a_proc = TFHE.asEuint16(a);
        uint16 b_proc = b;
        euint16 result = TFHE.min(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function min_uint16_euint16(uint16 a, uint16 b) public view returns (uint16) {
        uint16 a_proc = a;
        euint16 b_proc = TFHE.asEuint16(b);
        euint16 result = TFHE.min(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function max_euint16_uint16(uint16 a, uint16 b) public view returns (uint16) {
        euint16 a_proc = TFHE.asEuint16(a);
        uint16 b_proc = b;
        euint16 result = TFHE.max(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function max_uint16_euint16(uint16 a, uint16 b) public view returns (uint16) {
        uint16 a_proc = a;
        euint16 b_proc = TFHE.asEuint16(b);
        euint16 result = TFHE.max(a_proc, b_proc);
        return TFHE.decrypt(result);
    }
}
