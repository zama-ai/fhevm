// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity 0.8.19;

import "../../lib/TFHE.sol";

contract TFHETestSuite2 {
    function eq_euint16_euint32(bytes calldata a, bytes calldata b) public view returns (bool) {
        euint16 aProc = TFHE.asEuint16(a);
        euint32 bProc = TFHE.asEuint32(b);
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ne_euint16_euint32(bytes calldata a, bytes calldata b) public view returns (bool) {
        euint16 aProc = TFHE.asEuint16(a);
        euint32 bProc = TFHE.asEuint32(b);
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ge_euint16_euint32(bytes calldata a, bytes calldata b) public view returns (bool) {
        euint16 aProc = TFHE.asEuint16(a);
        euint32 bProc = TFHE.asEuint32(b);
        ebool result = TFHE.ge(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function gt_euint16_euint32(bytes calldata a, bytes calldata b) public view returns (bool) {
        euint16 aProc = TFHE.asEuint16(a);
        euint32 bProc = TFHE.asEuint32(b);
        ebool result = TFHE.gt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function le_euint16_euint32(bytes calldata a, bytes calldata b) public view returns (bool) {
        euint16 aProc = TFHE.asEuint16(a);
        euint32 bProc = TFHE.asEuint32(b);
        ebool result = TFHE.le(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function lt_euint16_euint32(bytes calldata a, bytes calldata b) public view returns (bool) {
        euint16 aProc = TFHE.asEuint16(a);
        euint32 bProc = TFHE.asEuint32(b);
        ebool result = TFHE.lt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function min_euint16_euint32(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint16 aProc = TFHE.asEuint16(a);
        euint32 bProc = TFHE.asEuint32(b);
        euint32 result = TFHE.min(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function max_euint16_euint32(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint16 aProc = TFHE.asEuint16(a);
        euint32 bProc = TFHE.asEuint32(b);
        euint32 result = TFHE.max(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function add_euint16_uint16(bytes calldata a, uint16 b) public view returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a);
        uint16 bProc = b;
        euint16 result = TFHE.add(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function add_uint16_euint16(uint16 a, bytes calldata b) public view returns (uint16) {
        uint16 aProc = a;
        euint16 bProc = TFHE.asEuint16(b);
        euint16 result = TFHE.add(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function sub_euint16_uint16(bytes calldata a, uint16 b) public view returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a);
        uint16 bProc = b;
        euint16 result = TFHE.sub(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function sub_uint16_euint16(uint16 a, bytes calldata b) public view returns (uint16) {
        uint16 aProc = a;
        euint16 bProc = TFHE.asEuint16(b);
        euint16 result = TFHE.sub(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function mul_euint16_uint16(bytes calldata a, uint16 b) public view returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a);
        uint16 bProc = b;
        euint16 result = TFHE.mul(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function mul_uint16_euint16(uint16 a, bytes calldata b) public view returns (uint16) {
        uint16 aProc = a;
        euint16 bProc = TFHE.asEuint16(b);
        euint16 result = TFHE.mul(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function div_euint16_uint16(bytes calldata a, uint16 b) public view returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a);
        uint16 bProc = b;
        euint16 result = TFHE.div(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function rem_euint16_uint16(bytes calldata a, uint16 b) public view returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a);
        uint16 bProc = b;
        euint16 result = TFHE.rem(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function eq_euint16_uint16(bytes calldata a, uint16 b) public view returns (bool) {
        euint16 aProc = TFHE.asEuint16(a);
        uint16 bProc = b;
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function eq_uint16_euint16(uint16 a, bytes calldata b) public view returns (bool) {
        uint16 aProc = a;
        euint16 bProc = TFHE.asEuint16(b);
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ne_euint16_uint16(bytes calldata a, uint16 b) public view returns (bool) {
        euint16 aProc = TFHE.asEuint16(a);
        uint16 bProc = b;
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ne_uint16_euint16(uint16 a, bytes calldata b) public view returns (bool) {
        uint16 aProc = a;
        euint16 bProc = TFHE.asEuint16(b);
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ge_euint16_uint16(bytes calldata a, uint16 b) public view returns (bool) {
        euint16 aProc = TFHE.asEuint16(a);
        uint16 bProc = b;
        ebool result = TFHE.ge(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ge_uint16_euint16(uint16 a, bytes calldata b) public view returns (bool) {
        uint16 aProc = a;
        euint16 bProc = TFHE.asEuint16(b);
        ebool result = TFHE.ge(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function gt_euint16_uint16(bytes calldata a, uint16 b) public view returns (bool) {
        euint16 aProc = TFHE.asEuint16(a);
        uint16 bProc = b;
        ebool result = TFHE.gt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function gt_uint16_euint16(uint16 a, bytes calldata b) public view returns (bool) {
        uint16 aProc = a;
        euint16 bProc = TFHE.asEuint16(b);
        ebool result = TFHE.gt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function le_euint16_uint16(bytes calldata a, uint16 b) public view returns (bool) {
        euint16 aProc = TFHE.asEuint16(a);
        uint16 bProc = b;
        ebool result = TFHE.le(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function le_uint16_euint16(uint16 a, bytes calldata b) public view returns (bool) {
        uint16 aProc = a;
        euint16 bProc = TFHE.asEuint16(b);
        ebool result = TFHE.le(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function lt_euint16_uint16(bytes calldata a, uint16 b) public view returns (bool) {
        euint16 aProc = TFHE.asEuint16(a);
        uint16 bProc = b;
        ebool result = TFHE.lt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function lt_uint16_euint16(uint16 a, bytes calldata b) public view returns (bool) {
        uint16 aProc = a;
        euint16 bProc = TFHE.asEuint16(b);
        ebool result = TFHE.lt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function min_euint16_uint16(bytes calldata a, uint16 b) public view returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a);
        uint16 bProc = b;
        euint16 result = TFHE.min(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function min_uint16_euint16(uint16 a, bytes calldata b) public view returns (uint16) {
        uint16 aProc = a;
        euint16 bProc = TFHE.asEuint16(b);
        euint16 result = TFHE.min(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function max_euint16_uint16(bytes calldata a, uint16 b) public view returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a);
        uint16 bProc = b;
        euint16 result = TFHE.max(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function max_uint16_euint16(uint16 a, bytes calldata b) public view returns (uint16) {
        uint16 aProc = a;
        euint16 bProc = TFHE.asEuint16(b);
        euint16 result = TFHE.max(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function add_euint32_euint8(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint32 result = TFHE.add(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function sub_euint32_euint8(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint32 result = TFHE.sub(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function mul_euint32_euint8(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint32 result = TFHE.mul(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function and_euint32_euint8(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint32 result = TFHE.and(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function or_euint32_euint8(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint32 result = TFHE.or(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function xor_euint32_euint8(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint32 result = TFHE.xor(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function eq_euint32_euint8(bytes calldata a, bytes calldata b) public view returns (bool) {
        euint32 aProc = TFHE.asEuint32(a);
        euint8 bProc = TFHE.asEuint8(b);
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ne_euint32_euint8(bytes calldata a, bytes calldata b) public view returns (bool) {
        euint32 aProc = TFHE.asEuint32(a);
        euint8 bProc = TFHE.asEuint8(b);
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ge_euint32_euint8(bytes calldata a, bytes calldata b) public view returns (bool) {
        euint32 aProc = TFHE.asEuint32(a);
        euint8 bProc = TFHE.asEuint8(b);
        ebool result = TFHE.ge(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function gt_euint32_euint8(bytes calldata a, bytes calldata b) public view returns (bool) {
        euint32 aProc = TFHE.asEuint32(a);
        euint8 bProc = TFHE.asEuint8(b);
        ebool result = TFHE.gt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function le_euint32_euint8(bytes calldata a, bytes calldata b) public view returns (bool) {
        euint32 aProc = TFHE.asEuint32(a);
        euint8 bProc = TFHE.asEuint8(b);
        ebool result = TFHE.le(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function lt_euint32_euint8(bytes calldata a, bytes calldata b) public view returns (bool) {
        euint32 aProc = TFHE.asEuint32(a);
        euint8 bProc = TFHE.asEuint8(b);
        ebool result = TFHE.lt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function min_euint32_euint8(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint32 result = TFHE.min(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function max_euint32_euint8(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint32 result = TFHE.max(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function add_euint32_euint16(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint16 bProc = TFHE.asEuint16(b);
        euint32 result = TFHE.add(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function sub_euint32_euint16(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint16 bProc = TFHE.asEuint16(b);
        euint32 result = TFHE.sub(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function mul_euint32_euint16(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint16 bProc = TFHE.asEuint16(b);
        euint32 result = TFHE.mul(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function and_euint32_euint16(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint16 bProc = TFHE.asEuint16(b);
        euint32 result = TFHE.and(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function or_euint32_euint16(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint16 bProc = TFHE.asEuint16(b);
        euint32 result = TFHE.or(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function xor_euint32_euint16(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint16 bProc = TFHE.asEuint16(b);
        euint32 result = TFHE.xor(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function eq_euint32_euint16(bytes calldata a, bytes calldata b) public view returns (bool) {
        euint32 aProc = TFHE.asEuint32(a);
        euint16 bProc = TFHE.asEuint16(b);
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ne_euint32_euint16(bytes calldata a, bytes calldata b) public view returns (bool) {
        euint32 aProc = TFHE.asEuint32(a);
        euint16 bProc = TFHE.asEuint16(b);
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ge_euint32_euint16(bytes calldata a, bytes calldata b) public view returns (bool) {
        euint32 aProc = TFHE.asEuint32(a);
        euint16 bProc = TFHE.asEuint16(b);
        ebool result = TFHE.ge(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function gt_euint32_euint16(bytes calldata a, bytes calldata b) public view returns (bool) {
        euint32 aProc = TFHE.asEuint32(a);
        euint16 bProc = TFHE.asEuint16(b);
        ebool result = TFHE.gt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function le_euint32_euint16(bytes calldata a, bytes calldata b) public view returns (bool) {
        euint32 aProc = TFHE.asEuint32(a);
        euint16 bProc = TFHE.asEuint16(b);
        ebool result = TFHE.le(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function lt_euint32_euint16(bytes calldata a, bytes calldata b) public view returns (bool) {
        euint32 aProc = TFHE.asEuint32(a);
        euint16 bProc = TFHE.asEuint16(b);
        ebool result = TFHE.lt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function min_euint32_euint16(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint16 bProc = TFHE.asEuint16(b);
        euint32 result = TFHE.min(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function max_euint32_euint16(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint16 bProc = TFHE.asEuint16(b);
        euint32 result = TFHE.max(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function add_euint32_euint32(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint32 bProc = TFHE.asEuint32(b);
        euint32 result = TFHE.add(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function sub_euint32_euint32(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint32 bProc = TFHE.asEuint32(b);
        euint32 result = TFHE.sub(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function mul_euint32_euint32(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint32 bProc = TFHE.asEuint32(b);
        euint32 result = TFHE.mul(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function and_euint32_euint32(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint32 bProc = TFHE.asEuint32(b);
        euint32 result = TFHE.and(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function or_euint32_euint32(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint32 bProc = TFHE.asEuint32(b);
        euint32 result = TFHE.or(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function xor_euint32_euint32(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint32 bProc = TFHE.asEuint32(b);
        euint32 result = TFHE.xor(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function eq_euint32_euint32(bytes calldata a, bytes calldata b) public view returns (bool) {
        euint32 aProc = TFHE.asEuint32(a);
        euint32 bProc = TFHE.asEuint32(b);
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ne_euint32_euint32(bytes calldata a, bytes calldata b) public view returns (bool) {
        euint32 aProc = TFHE.asEuint32(a);
        euint32 bProc = TFHE.asEuint32(b);
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ge_euint32_euint32(bytes calldata a, bytes calldata b) public view returns (bool) {
        euint32 aProc = TFHE.asEuint32(a);
        euint32 bProc = TFHE.asEuint32(b);
        ebool result = TFHE.ge(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function gt_euint32_euint32(bytes calldata a, bytes calldata b) public view returns (bool) {
        euint32 aProc = TFHE.asEuint32(a);
        euint32 bProc = TFHE.asEuint32(b);
        ebool result = TFHE.gt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function le_euint32_euint32(bytes calldata a, bytes calldata b) public view returns (bool) {
        euint32 aProc = TFHE.asEuint32(a);
        euint32 bProc = TFHE.asEuint32(b);
        ebool result = TFHE.le(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function lt_euint32_euint32(bytes calldata a, bytes calldata b) public view returns (bool) {
        euint32 aProc = TFHE.asEuint32(a);
        euint32 bProc = TFHE.asEuint32(b);
        ebool result = TFHE.lt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function min_euint32_euint32(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint32 bProc = TFHE.asEuint32(b);
        euint32 result = TFHE.min(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function max_euint32_euint32(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint32 bProc = TFHE.asEuint32(b);
        euint32 result = TFHE.max(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function add_euint32_uint32(bytes calldata a, uint32 b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        uint32 bProc = b;
        euint32 result = TFHE.add(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function add_uint32_euint32(uint32 a, bytes calldata b) public view returns (uint32) {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b);
        euint32 result = TFHE.add(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function sub_euint32_uint32(bytes calldata a, uint32 b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        uint32 bProc = b;
        euint32 result = TFHE.sub(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function sub_uint32_euint32(uint32 a, bytes calldata b) public view returns (uint32) {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b);
        euint32 result = TFHE.sub(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function mul_euint32_uint32(bytes calldata a, uint32 b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        uint32 bProc = b;
        euint32 result = TFHE.mul(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function mul_uint32_euint32(uint32 a, bytes calldata b) public view returns (uint32) {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b);
        euint32 result = TFHE.mul(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function div_euint32_uint32(bytes calldata a, uint32 b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        uint32 bProc = b;
        euint32 result = TFHE.div(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function rem_euint32_uint32(bytes calldata a, uint32 b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        uint32 bProc = b;
        euint32 result = TFHE.rem(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function eq_euint32_uint32(bytes calldata a, uint32 b) public view returns (bool) {
        euint32 aProc = TFHE.asEuint32(a);
        uint32 bProc = b;
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function eq_uint32_euint32(uint32 a, bytes calldata b) public view returns (bool) {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b);
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ne_euint32_uint32(bytes calldata a, uint32 b) public view returns (bool) {
        euint32 aProc = TFHE.asEuint32(a);
        uint32 bProc = b;
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ne_uint32_euint32(uint32 a, bytes calldata b) public view returns (bool) {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b);
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ge_euint32_uint32(bytes calldata a, uint32 b) public view returns (bool) {
        euint32 aProc = TFHE.asEuint32(a);
        uint32 bProc = b;
        ebool result = TFHE.ge(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ge_uint32_euint32(uint32 a, bytes calldata b) public view returns (bool) {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b);
        ebool result = TFHE.ge(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function gt_euint32_uint32(bytes calldata a, uint32 b) public view returns (bool) {
        euint32 aProc = TFHE.asEuint32(a);
        uint32 bProc = b;
        ebool result = TFHE.gt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function gt_uint32_euint32(uint32 a, bytes calldata b) public view returns (bool) {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b);
        ebool result = TFHE.gt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function le_euint32_uint32(bytes calldata a, uint32 b) public view returns (bool) {
        euint32 aProc = TFHE.asEuint32(a);
        uint32 bProc = b;
        ebool result = TFHE.le(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function le_uint32_euint32(uint32 a, bytes calldata b) public view returns (bool) {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b);
        ebool result = TFHE.le(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function lt_euint32_uint32(bytes calldata a, uint32 b) public view returns (bool) {
        euint32 aProc = TFHE.asEuint32(a);
        uint32 bProc = b;
        ebool result = TFHE.lt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function lt_uint32_euint32(uint32 a, bytes calldata b) public view returns (bool) {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b);
        ebool result = TFHE.lt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function min_euint32_uint32(bytes calldata a, uint32 b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        uint32 bProc = b;
        euint32 result = TFHE.min(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function min_uint32_euint32(uint32 a, bytes calldata b) public view returns (uint32) {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b);
        euint32 result = TFHE.min(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function max_euint32_uint32(bytes calldata a, uint32 b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        uint32 bProc = b;
        euint32 result = TFHE.max(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function max_uint32_euint32(uint32 a, bytes calldata b) public view returns (uint32) {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b);
        euint32 result = TFHE.max(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shl_euint8_euint8(bytes calldata a, bytes calldata b) public view returns (uint8) {
        euint8 aProc = TFHE.asEuint8(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint8 result = TFHE.shl(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shl_euint8_uint8(bytes calldata a, uint8 b) public view returns (uint8) {
        euint8 aProc = TFHE.asEuint8(a);
        uint8 bProc = b;
        euint8 result = TFHE.shl(aProc, bProc);
        return TFHE.decrypt(result);
    }
}
