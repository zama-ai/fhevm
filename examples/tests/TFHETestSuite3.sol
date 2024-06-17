// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.20;

import "../../lib/TFHE.sol";

contract TFHETestSuite3 {
    function min_euint16_euint4(einput a, einput b, bytes calldata inputProof) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint16 result = TFHE.min(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function max_euint16_euint4(einput a, einput b, bytes calldata inputProof) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint16 result = TFHE.max(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function add_euint16_euint8(einput a, einput b, bytes calldata inputProof) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint16 result = TFHE.add(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function sub_euint16_euint8(einput a, einput b, bytes calldata inputProof) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint16 result = TFHE.sub(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function mul_euint16_euint8(einput a, einput b, bytes calldata inputProof) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint16 result = TFHE.mul(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function and_euint16_euint8(einput a, einput b, bytes calldata inputProof) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint16 result = TFHE.and(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function or_euint16_euint8(einput a, einput b, bytes calldata inputProof) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint16 result = TFHE.or(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function xor_euint16_euint8(einput a, einput b, bytes calldata inputProof) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint16 result = TFHE.xor(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function eq_euint16_euint8(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ne_euint16_euint8(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ge_euint16_euint8(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function gt_euint16_euint8(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function le_euint16_euint8(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function lt_euint16_euint8(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function min_euint16_euint8(einput a, einput b, bytes calldata inputProof) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint16 result = TFHE.min(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function max_euint16_euint8(einput a, einput b, bytes calldata inputProof) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint16 result = TFHE.max(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function add_euint16_euint16(einput a, einput b, bytes calldata inputProof) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint16 result = TFHE.add(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function sub_euint16_euint16(einput a, einput b, bytes calldata inputProof) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint16 result = TFHE.sub(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function mul_euint16_euint16(einput a, einput b, bytes calldata inputProof) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint16 result = TFHE.mul(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function and_euint16_euint16(einput a, einput b, bytes calldata inputProof) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint16 result = TFHE.and(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function or_euint16_euint16(einput a, einput b, bytes calldata inputProof) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint16 result = TFHE.or(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function xor_euint16_euint16(einput a, einput b, bytes calldata inputProof) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint16 result = TFHE.xor(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function eq_euint16_euint16(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ne_euint16_euint16(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ge_euint16_euint16(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function gt_euint16_euint16(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function le_euint16_euint16(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function lt_euint16_euint16(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function min_euint16_euint16(einput a, einput b, bytes calldata inputProof) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint16 result = TFHE.min(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function max_euint16_euint16(einput a, einput b, bytes calldata inputProof) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint16 result = TFHE.max(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function add_euint16_euint32(einput a, einput b, bytes calldata inputProof) public returns (uint32) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.add(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function sub_euint16_euint32(einput a, einput b, bytes calldata inputProof) public returns (uint32) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.sub(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function mul_euint16_euint32(einput a, einput b, bytes calldata inputProof) public returns (uint32) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.mul(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function and_euint16_euint32(einput a, einput b, bytes calldata inputProof) public returns (uint32) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.and(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function or_euint16_euint32(einput a, einput b, bytes calldata inputProof) public returns (uint32) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.or(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function xor_euint16_euint32(einput a, einput b, bytes calldata inputProof) public returns (uint32) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.xor(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function eq_euint16_euint32(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ne_euint16_euint32(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ge_euint16_euint32(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function gt_euint16_euint32(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function le_euint16_euint32(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function lt_euint16_euint32(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function min_euint16_euint32(einput a, einput b, bytes calldata inputProof) public returns (uint32) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.min(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function max_euint16_euint32(einput a, einput b, bytes calldata inputProof) public returns (uint32) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.max(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function add_euint16_euint64(einput a, einput b, bytes calldata inputProof) public returns (uint64) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.add(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function sub_euint16_euint64(einput a, einput b, bytes calldata inputProof) public returns (uint64) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.sub(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function mul_euint16_euint64(einput a, einput b, bytes calldata inputProof) public returns (uint64) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.mul(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function and_euint16_euint64(einput a, einput b, bytes calldata inputProof) public returns (uint64) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.and(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function or_euint16_euint64(einput a, einput b, bytes calldata inputProof) public returns (uint64) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.or(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function xor_euint16_euint64(einput a, einput b, bytes calldata inputProof) public returns (uint64) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.xor(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function eq_euint16_euint64(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ne_euint16_euint64(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ge_euint16_euint64(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function gt_euint16_euint64(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function le_euint16_euint64(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function lt_euint16_euint64(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function min_euint16_euint64(einput a, einput b, bytes calldata inputProof) public returns (uint64) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.min(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function max_euint16_euint64(einput a, einput b, bytes calldata inputProof) public returns (uint64) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.max(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function add_euint16_uint16(einput a, uint16 b, bytes calldata inputProof) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        uint16 bProc = b;
        euint16 result = TFHE.add(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function add_uint16_euint16(uint16 a, einput b, bytes calldata inputProof) public returns (uint16) {
        uint16 aProc = a;
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint16 result = TFHE.add(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function sub_euint16_uint16(einput a, uint16 b, bytes calldata inputProof) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        uint16 bProc = b;
        euint16 result = TFHE.sub(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function sub_uint16_euint16(uint16 a, einput b, bytes calldata inputProof) public returns (uint16) {
        uint16 aProc = a;
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint16 result = TFHE.sub(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function mul_euint16_uint16(einput a, uint16 b, bytes calldata inputProof) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        uint16 bProc = b;
        euint16 result = TFHE.mul(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function mul_uint16_euint16(uint16 a, einput b, bytes calldata inputProof) public returns (uint16) {
        uint16 aProc = a;
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint16 result = TFHE.mul(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function div_euint16_uint16(einput a, uint16 b, bytes calldata inputProof) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        uint16 bProc = b;
        euint16 result = TFHE.div(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function rem_euint16_uint16(einput a, uint16 b, bytes calldata inputProof) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        uint16 bProc = b;
        euint16 result = TFHE.rem(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function eq_euint16_uint16(einput a, uint16 b, bytes calldata inputProof) public returns (bool) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        uint16 bProc = b;
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function eq_uint16_euint16(uint16 a, einput b, bytes calldata inputProof) public returns (bool) {
        uint16 aProc = a;
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ne_euint16_uint16(einput a, uint16 b, bytes calldata inputProof) public returns (bool) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        uint16 bProc = b;
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ne_uint16_euint16(uint16 a, einput b, bytes calldata inputProof) public returns (bool) {
        uint16 aProc = a;
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ge_euint16_uint16(einput a, uint16 b, bytes calldata inputProof) public returns (bool) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        uint16 bProc = b;
        ebool result = TFHE.ge(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ge_uint16_euint16(uint16 a, einput b, bytes calldata inputProof) public returns (bool) {
        uint16 aProc = a;
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function gt_euint16_uint16(einput a, uint16 b, bytes calldata inputProof) public returns (bool) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        uint16 bProc = b;
        ebool result = TFHE.gt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function gt_uint16_euint16(uint16 a, einput b, bytes calldata inputProof) public returns (bool) {
        uint16 aProc = a;
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function le_euint16_uint16(einput a, uint16 b, bytes calldata inputProof) public returns (bool) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        uint16 bProc = b;
        ebool result = TFHE.le(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function le_uint16_euint16(uint16 a, einput b, bytes calldata inputProof) public returns (bool) {
        uint16 aProc = a;
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function lt_euint16_uint16(einput a, uint16 b, bytes calldata inputProof) public returns (bool) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        uint16 bProc = b;
        ebool result = TFHE.lt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function lt_uint16_euint16(uint16 a, einput b, bytes calldata inputProof) public returns (bool) {
        uint16 aProc = a;
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function min_euint16_uint16(einput a, uint16 b, bytes calldata inputProof) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        uint16 bProc = b;
        euint16 result = TFHE.min(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function min_uint16_euint16(uint16 a, einput b, bytes calldata inputProof) public returns (uint16) {
        uint16 aProc = a;
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint16 result = TFHE.min(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function max_euint16_uint16(einput a, uint16 b, bytes calldata inputProof) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        uint16 bProc = b;
        euint16 result = TFHE.max(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function max_uint16_euint16(uint16 a, einput b, bytes calldata inputProof) public returns (uint16) {
        uint16 aProc = a;
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint16 result = TFHE.max(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function add_euint32_euint4(einput a, einput b, bytes calldata inputProof) public returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint32 result = TFHE.add(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function sub_euint32_euint4(einput a, einput b, bytes calldata inputProof) public returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint32 result = TFHE.sub(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function mul_euint32_euint4(einput a, einput b, bytes calldata inputProof) public returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint32 result = TFHE.mul(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function and_euint32_euint4(einput a, einput b, bytes calldata inputProof) public returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint32 result = TFHE.and(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function or_euint32_euint4(einput a, einput b, bytes calldata inputProof) public returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint32 result = TFHE.or(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function xor_euint32_euint4(einput a, einput b, bytes calldata inputProof) public returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint32 result = TFHE.xor(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function eq_euint32_euint4(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ne_euint32_euint4(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ge_euint32_euint4(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function gt_euint32_euint4(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function le_euint32_euint4(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function lt_euint32_euint4(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function min_euint32_euint4(einput a, einput b, bytes calldata inputProof) public returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint32 result = TFHE.min(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function max_euint32_euint4(einput a, einput b, bytes calldata inputProof) public returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint32 result = TFHE.max(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function add_euint32_euint8(einput a, einput b, bytes calldata inputProof) public returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint32 result = TFHE.add(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function sub_euint32_euint8(einput a, einput b, bytes calldata inputProof) public returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint32 result = TFHE.sub(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function mul_euint32_euint8(einput a, einput b, bytes calldata inputProof) public returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint32 result = TFHE.mul(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function and_euint32_euint8(einput a, einput b, bytes calldata inputProof) public returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint32 result = TFHE.and(aProc, bProc);
        return TFHE.decrypt(result);
    }
}
