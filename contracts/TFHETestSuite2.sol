// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity >=0.8.13 <0.8.20;

import "../lib/TFHE.sol";

contract TFHETestSuite2 {
    function add_euint32_euint8(uint32 a, uint8 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint8 b_proc = TFHE.asEuint8(b);
        euint32 result = TFHE.add(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function sub_euint32_euint8(uint32 a, uint8 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint8 b_proc = TFHE.asEuint8(b);
        euint32 result = TFHE.sub(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function mul_euint32_euint8(uint32 a, uint8 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint8 b_proc = TFHE.asEuint8(b);
        euint32 result = TFHE.mul(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function and_euint32_euint8(uint32 a, uint8 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint8 b_proc = TFHE.asEuint8(b);
        euint32 result = TFHE.and(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function or_euint32_euint8(uint32 a, uint8 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint8 b_proc = TFHE.asEuint8(b);
        euint32 result = TFHE.or(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function xor_euint32_euint8(uint32 a, uint8 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint8 b_proc = TFHE.asEuint8(b);
        euint32 result = TFHE.xor(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function shl_euint32_euint8(uint32 a, uint8 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint8 b_proc = TFHE.asEuint8(b);
        euint32 result = TFHE.shl(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function shr_euint32_euint8(uint32 a, uint8 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint8 b_proc = TFHE.asEuint8(b);
        euint32 result = TFHE.shr(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function eq_euint32_euint8(uint32 a, uint8 b) public view returns (bool) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint8 b_proc = TFHE.asEuint8(b);
        ebool result = TFHE.eq(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function ne_euint32_euint8(uint32 a, uint8 b) public view returns (bool) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint8 b_proc = TFHE.asEuint8(b);
        ebool result = TFHE.ne(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function ge_euint32_euint8(uint32 a, uint8 b) public view returns (bool) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint8 b_proc = TFHE.asEuint8(b);
        ebool result = TFHE.ge(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function gt_euint32_euint8(uint32 a, uint8 b) public view returns (bool) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint8 b_proc = TFHE.asEuint8(b);
        ebool result = TFHE.gt(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function le_euint32_euint8(uint32 a, uint8 b) public view returns (bool) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint8 b_proc = TFHE.asEuint8(b);
        ebool result = TFHE.le(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function lt_euint32_euint8(uint32 a, uint8 b) public view returns (bool) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint8 b_proc = TFHE.asEuint8(b);
        ebool result = TFHE.lt(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function min_euint32_euint8(uint32 a, uint8 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint8 b_proc = TFHE.asEuint8(b);
        euint32 result = TFHE.min(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function max_euint32_euint8(uint32 a, uint8 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint8 b_proc = TFHE.asEuint8(b);
        euint32 result = TFHE.max(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function add_euint32_euint16(uint32 a, uint16 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint16 b_proc = TFHE.asEuint16(b);
        euint32 result = TFHE.add(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function sub_euint32_euint16(uint32 a, uint16 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint16 b_proc = TFHE.asEuint16(b);
        euint32 result = TFHE.sub(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function mul_euint32_euint16(uint32 a, uint16 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint16 b_proc = TFHE.asEuint16(b);
        euint32 result = TFHE.mul(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function and_euint32_euint16(uint32 a, uint16 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint16 b_proc = TFHE.asEuint16(b);
        euint32 result = TFHE.and(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function or_euint32_euint16(uint32 a, uint16 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint16 b_proc = TFHE.asEuint16(b);
        euint32 result = TFHE.or(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function xor_euint32_euint16(uint32 a, uint16 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint16 b_proc = TFHE.asEuint16(b);
        euint32 result = TFHE.xor(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function shl_euint32_euint16(uint32 a, uint16 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint16 b_proc = TFHE.asEuint16(b);
        euint32 result = TFHE.shl(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function shr_euint32_euint16(uint32 a, uint16 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint16 b_proc = TFHE.asEuint16(b);
        euint32 result = TFHE.shr(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function eq_euint32_euint16(uint32 a, uint16 b) public view returns (bool) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint16 b_proc = TFHE.asEuint16(b);
        ebool result = TFHE.eq(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function ne_euint32_euint16(uint32 a, uint16 b) public view returns (bool) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint16 b_proc = TFHE.asEuint16(b);
        ebool result = TFHE.ne(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function ge_euint32_euint16(uint32 a, uint16 b) public view returns (bool) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint16 b_proc = TFHE.asEuint16(b);
        ebool result = TFHE.ge(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function gt_euint32_euint16(uint32 a, uint16 b) public view returns (bool) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint16 b_proc = TFHE.asEuint16(b);
        ebool result = TFHE.gt(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function le_euint32_euint16(uint32 a, uint16 b) public view returns (bool) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint16 b_proc = TFHE.asEuint16(b);
        ebool result = TFHE.le(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function lt_euint32_euint16(uint32 a, uint16 b) public view returns (bool) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint16 b_proc = TFHE.asEuint16(b);
        ebool result = TFHE.lt(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function min_euint32_euint16(uint32 a, uint16 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint16 b_proc = TFHE.asEuint16(b);
        euint32 result = TFHE.min(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function max_euint32_euint16(uint32 a, uint16 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint16 b_proc = TFHE.asEuint16(b);
        euint32 result = TFHE.max(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function add_euint32_euint32(uint32 a, uint32 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.add(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function sub_euint32_euint32(uint32 a, uint32 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.sub(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function mul_euint32_euint32(uint32 a, uint32 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.mul(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function and_euint32_euint32(uint32 a, uint32 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.and(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function or_euint32_euint32(uint32 a, uint32 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.or(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function xor_euint32_euint32(uint32 a, uint32 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.xor(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function shl_euint32_euint32(uint32 a, uint32 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.shl(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function shr_euint32_euint32(uint32 a, uint32 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.shr(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function eq_euint32_euint32(uint32 a, uint32 b) public view returns (bool) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint32 b_proc = TFHE.asEuint32(b);
        ebool result = TFHE.eq(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function ne_euint32_euint32(uint32 a, uint32 b) public view returns (bool) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint32 b_proc = TFHE.asEuint32(b);
        ebool result = TFHE.ne(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function ge_euint32_euint32(uint32 a, uint32 b) public view returns (bool) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint32 b_proc = TFHE.asEuint32(b);
        ebool result = TFHE.ge(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function gt_euint32_euint32(uint32 a, uint32 b) public view returns (bool) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint32 b_proc = TFHE.asEuint32(b);
        ebool result = TFHE.gt(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function le_euint32_euint32(uint32 a, uint32 b) public view returns (bool) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint32 b_proc = TFHE.asEuint32(b);
        ebool result = TFHE.le(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function lt_euint32_euint32(uint32 a, uint32 b) public view returns (bool) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint32 b_proc = TFHE.asEuint32(b);
        ebool result = TFHE.lt(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function min_euint32_euint32(uint32 a, uint32 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.min(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function max_euint32_euint32(uint32 a, uint32 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.max(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function add_euint32_uint32(uint32 a, uint32 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        uint32 b_proc = b;
        euint32 result = TFHE.add(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function add_uint32_euint32(uint32 a, uint32 b) public view returns (uint32) {
        uint32 a_proc = a;
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.add(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function sub_euint32_uint32(uint32 a, uint32 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        uint32 b_proc = b;
        euint32 result = TFHE.sub(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function sub_uint32_euint32(uint32 a, uint32 b) public view returns (uint32) {
        uint32 a_proc = a;
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.sub(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function mul_euint32_uint32(uint32 a, uint32 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        uint32 b_proc = b;
        euint32 result = TFHE.mul(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function mul_uint32_euint32(uint32 a, uint32 b) public view returns (uint32) {
        uint32 a_proc = a;
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.mul(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function div_euint32_uint32(uint32 a, uint32 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        uint32 b_proc = b;
        euint32 result = TFHE.div(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function shl_euint32_uint32(uint32 a, uint32 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        uint32 b_proc = b;
        euint32 result = TFHE.shl(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function shl_uint32_euint32(uint32 a, uint32 b) public view returns (uint32) {
        uint32 a_proc = a;
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.shl(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function shr_euint32_uint32(uint32 a, uint32 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        uint32 b_proc = b;
        euint32 result = TFHE.shr(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function shr_uint32_euint32(uint32 a, uint32 b) public view returns (uint32) {
        uint32 a_proc = a;
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.shr(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function eq_euint32_uint32(uint32 a, uint32 b) public view returns (bool) {
        euint32 a_proc = TFHE.asEuint32(a);
        uint32 b_proc = b;
        ebool result = TFHE.eq(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function eq_uint32_euint32(uint32 a, uint32 b) public view returns (bool) {
        uint32 a_proc = a;
        euint32 b_proc = TFHE.asEuint32(b);
        ebool result = TFHE.eq(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function ne_euint32_uint32(uint32 a, uint32 b) public view returns (bool) {
        euint32 a_proc = TFHE.asEuint32(a);
        uint32 b_proc = b;
        ebool result = TFHE.ne(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function ne_uint32_euint32(uint32 a, uint32 b) public view returns (bool) {
        uint32 a_proc = a;
        euint32 b_proc = TFHE.asEuint32(b);
        ebool result = TFHE.ne(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function ge_euint32_uint32(uint32 a, uint32 b) public view returns (bool) {
        euint32 a_proc = TFHE.asEuint32(a);
        uint32 b_proc = b;
        ebool result = TFHE.ge(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function ge_uint32_euint32(uint32 a, uint32 b) public view returns (bool) {
        uint32 a_proc = a;
        euint32 b_proc = TFHE.asEuint32(b);
        ebool result = TFHE.ge(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function gt_euint32_uint32(uint32 a, uint32 b) public view returns (bool) {
        euint32 a_proc = TFHE.asEuint32(a);
        uint32 b_proc = b;
        ebool result = TFHE.gt(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function gt_uint32_euint32(uint32 a, uint32 b) public view returns (bool) {
        uint32 a_proc = a;
        euint32 b_proc = TFHE.asEuint32(b);
        ebool result = TFHE.gt(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function le_euint32_uint32(uint32 a, uint32 b) public view returns (bool) {
        euint32 a_proc = TFHE.asEuint32(a);
        uint32 b_proc = b;
        ebool result = TFHE.le(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function le_uint32_euint32(uint32 a, uint32 b) public view returns (bool) {
        uint32 a_proc = a;
        euint32 b_proc = TFHE.asEuint32(b);
        ebool result = TFHE.le(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function lt_euint32_uint32(uint32 a, uint32 b) public view returns (bool) {
        euint32 a_proc = TFHE.asEuint32(a);
        uint32 b_proc = b;
        ebool result = TFHE.lt(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function lt_uint32_euint32(uint32 a, uint32 b) public view returns (bool) {
        uint32 a_proc = a;
        euint32 b_proc = TFHE.asEuint32(b);
        ebool result = TFHE.lt(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function min_euint32_uint32(uint32 a, uint32 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        uint32 b_proc = b;
        euint32 result = TFHE.min(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function min_uint32_euint32(uint32 a, uint32 b) public view returns (uint32) {
        uint32 a_proc = a;
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.min(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function max_euint32_uint32(uint32 a, uint32 b) public view returns (uint32) {
        euint32 a_proc = TFHE.asEuint32(a);
        uint32 b_proc = b;
        euint32 result = TFHE.max(a_proc, b_proc);
        return TFHE.decrypt(result);
    }

    function max_uint32_euint32(uint32 a, uint32 b) public view returns (uint32) {
        uint32 a_proc = a;
        euint32 b_proc = TFHE.asEuint32(b);
        euint32 result = TFHE.max(a_proc, b_proc);
        return TFHE.decrypt(result);
    }
}
