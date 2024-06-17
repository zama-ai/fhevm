// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.20;

import "../../lib/TFHE.sol";

contract TFHETestSuite1 {
    function add_euint4_euint4(einput a, einput b, bytes calldata inputProof) public returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint4 result = TFHE.add(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function sub_euint4_euint4(einput a, einput b, bytes calldata inputProof) public returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint4 result = TFHE.sub(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function mul_euint4_euint4(einput a, einput b, bytes calldata inputProof) public returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint4 result = TFHE.mul(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function and_euint4_euint4(einput a, einput b, bytes calldata inputProof) public returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint4 result = TFHE.and(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function or_euint4_euint4(einput a, einput b, bytes calldata inputProof) public returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint4 result = TFHE.or(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function xor_euint4_euint4(einput a, einput b, bytes calldata inputProof) public returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint4 result = TFHE.xor(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function eq_euint4_euint4(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ne_euint4_euint4(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ge_euint4_euint4(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function gt_euint4_euint4(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function le_euint4_euint4(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function lt_euint4_euint4(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function min_euint4_euint4(einput a, einput b, bytes calldata inputProof) public returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint4 result = TFHE.min(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function max_euint4_euint4(einput a, einput b, bytes calldata inputProof) public returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint4 result = TFHE.max(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function add_euint4_euint8(einput a, einput b, bytes calldata inputProof) public returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint8 result = TFHE.add(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function sub_euint4_euint8(einput a, einput b, bytes calldata inputProof) public returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint8 result = TFHE.sub(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function mul_euint4_euint8(einput a, einput b, bytes calldata inputProof) public returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint8 result = TFHE.mul(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function and_euint4_euint8(einput a, einput b, bytes calldata inputProof) public returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint8 result = TFHE.and(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function or_euint4_euint8(einput a, einput b, bytes calldata inputProof) public returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint8 result = TFHE.or(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function xor_euint4_euint8(einput a, einput b, bytes calldata inputProof) public returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint8 result = TFHE.xor(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function eq_euint4_euint8(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ne_euint4_euint8(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ge_euint4_euint8(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function gt_euint4_euint8(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function le_euint4_euint8(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function lt_euint4_euint8(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function min_euint4_euint8(einput a, einput b, bytes calldata inputProof) public returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint8 result = TFHE.min(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function max_euint4_euint8(einput a, einput b, bytes calldata inputProof) public returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint8 result = TFHE.max(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function add_euint4_euint16(einput a, einput b, bytes calldata inputProof) public returns (uint16) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint16 result = TFHE.add(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function sub_euint4_euint16(einput a, einput b, bytes calldata inputProof) public returns (uint16) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint16 result = TFHE.sub(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function mul_euint4_euint16(einput a, einput b, bytes calldata inputProof) public returns (uint16) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint16 result = TFHE.mul(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function and_euint4_euint16(einput a, einput b, bytes calldata inputProof) public returns (uint16) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint16 result = TFHE.and(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function or_euint4_euint16(einput a, einput b, bytes calldata inputProof) public returns (uint16) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint16 result = TFHE.or(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function xor_euint4_euint16(einput a, einput b, bytes calldata inputProof) public returns (uint16) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint16 result = TFHE.xor(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function eq_euint4_euint16(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ne_euint4_euint16(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ge_euint4_euint16(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function gt_euint4_euint16(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function le_euint4_euint16(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function lt_euint4_euint16(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function min_euint4_euint16(einput a, einput b, bytes calldata inputProof) public returns (uint16) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint16 result = TFHE.min(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function max_euint4_euint16(einput a, einput b, bytes calldata inputProof) public returns (uint16) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint16 bProc = TFHE.asEuint16(b, inputProof);
        euint16 result = TFHE.max(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function add_euint4_euint32(einput a, einput b, bytes calldata inputProof) public returns (uint32) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.add(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function sub_euint4_euint32(einput a, einput b, bytes calldata inputProof) public returns (uint32) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.sub(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function mul_euint4_euint32(einput a, einput b, bytes calldata inputProof) public returns (uint32) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.mul(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function and_euint4_euint32(einput a, einput b, bytes calldata inputProof) public returns (uint32) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.and(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function or_euint4_euint32(einput a, einput b, bytes calldata inputProof) public returns (uint32) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.or(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function xor_euint4_euint32(einput a, einput b, bytes calldata inputProof) public returns (uint32) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.xor(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function eq_euint4_euint32(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ne_euint4_euint32(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ge_euint4_euint32(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function gt_euint4_euint32(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function le_euint4_euint32(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function lt_euint4_euint32(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function min_euint4_euint32(einput a, einput b, bytes calldata inputProof) public returns (uint32) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.min(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function max_euint4_euint32(einput a, einput b, bytes calldata inputProof) public returns (uint32) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint32 bProc = TFHE.asEuint32(b, inputProof);
        euint32 result = TFHE.max(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function add_euint4_euint64(einput a, einput b, bytes calldata inputProof) public returns (uint64) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.add(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function sub_euint4_euint64(einput a, einput b, bytes calldata inputProof) public returns (uint64) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.sub(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function mul_euint4_euint64(einput a, einput b, bytes calldata inputProof) public returns (uint64) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.mul(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function and_euint4_euint64(einput a, einput b, bytes calldata inputProof) public returns (uint64) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.and(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function or_euint4_euint64(einput a, einput b, bytes calldata inputProof) public returns (uint64) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.or(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function xor_euint4_euint64(einput a, einput b, bytes calldata inputProof) public returns (uint64) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.xor(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function eq_euint4_euint64(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ne_euint4_euint64(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ge_euint4_euint64(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function gt_euint4_euint64(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function le_euint4_euint64(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function lt_euint4_euint64(einput a, einput b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function min_euint4_euint64(einput a, einput b, bytes calldata inputProof) public returns (uint64) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.min(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function max_euint4_euint64(einput a, einput b, bytes calldata inputProof) public returns (uint64) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint64 bProc = TFHE.asEuint64(b, inputProof);
        euint64 result = TFHE.max(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function add_euint4_uint8(einput a, uint8 b, bytes calldata inputProof) public returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        uint8 bProc = b;
        euint4 result = TFHE.add(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function add_uint8_euint4(uint8 a, einput b, bytes calldata inputProof) public returns (uint8) {
        uint8 aProc = a;
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint4 result = TFHE.add(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function sub_euint4_uint8(einput a, uint8 b, bytes calldata inputProof) public returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        uint8 bProc = b;
        euint4 result = TFHE.sub(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function sub_uint8_euint4(uint8 a, einput b, bytes calldata inputProof) public returns (uint8) {
        uint8 aProc = a;
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint4 result = TFHE.sub(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function mul_euint4_uint8(einput a, uint8 b, bytes calldata inputProof) public returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        uint8 bProc = b;
        euint4 result = TFHE.mul(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function mul_uint8_euint4(uint8 a, einput b, bytes calldata inputProof) public returns (uint8) {
        uint8 aProc = a;
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint4 result = TFHE.mul(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function div_euint4_uint8(einput a, uint8 b, bytes calldata inputProof) public returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        uint8 bProc = b;
        euint4 result = TFHE.div(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function rem_euint4_uint8(einput a, uint8 b, bytes calldata inputProof) public returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        uint8 bProc = b;
        euint4 result = TFHE.rem(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function eq_euint4_uint8(einput a, uint8 b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        uint8 bProc = b;
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function eq_uint8_euint4(uint8 a, einput b, bytes calldata inputProof) public returns (bool) {
        uint8 aProc = a;
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.eq(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ne_euint4_uint8(einput a, uint8 b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        uint8 bProc = b;
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ne_uint8_euint4(uint8 a, einput b, bytes calldata inputProof) public returns (bool) {
        uint8 aProc = a;
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.ne(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ge_euint4_uint8(einput a, uint8 b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        uint8 bProc = b;
        ebool result = TFHE.ge(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function ge_uint8_euint4(uint8 a, einput b, bytes calldata inputProof) public returns (bool) {
        uint8 aProc = a;
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.ge(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function gt_euint4_uint8(einput a, uint8 b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        uint8 bProc = b;
        ebool result = TFHE.gt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function gt_uint8_euint4(uint8 a, einput b, bytes calldata inputProof) public returns (bool) {
        uint8 aProc = a;
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.gt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function le_euint4_uint8(einput a, uint8 b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        uint8 bProc = b;
        ebool result = TFHE.le(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function le_uint8_euint4(uint8 a, einput b, bytes calldata inputProof) public returns (bool) {
        uint8 aProc = a;
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.le(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function lt_euint4_uint8(einput a, uint8 b, bytes calldata inputProof) public returns (bool) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        uint8 bProc = b;
        ebool result = TFHE.lt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function lt_uint8_euint4(uint8 a, einput b, bytes calldata inputProof) public returns (bool) {
        uint8 aProc = a;
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        ebool result = TFHE.lt(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function min_euint4_uint8(einput a, uint8 b, bytes calldata inputProof) public returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        uint8 bProc = b;
        euint4 result = TFHE.min(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function min_uint8_euint4(uint8 a, einput b, bytes calldata inputProof) public returns (uint8) {
        uint8 aProc = a;
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint4 result = TFHE.min(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function max_euint4_uint8(einput a, uint8 b, bytes calldata inputProof) public returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        uint8 bProc = b;
        euint4 result = TFHE.max(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function max_uint8_euint4(uint8 a, einput b, bytes calldata inputProof) public returns (uint8) {
        uint8 aProc = a;
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint4 result = TFHE.max(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function add_euint8_euint4(einput a, einput b, bytes calldata inputProof) public returns (uint8) {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint8 result = TFHE.add(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function sub_euint8_euint4(einput a, einput b, bytes calldata inputProof) public returns (uint8) {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint8 result = TFHE.sub(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function mul_euint8_euint4(einput a, einput b, bytes calldata inputProof) public returns (uint8) {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint8 result = TFHE.mul(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function and_euint8_euint4(einput a, einput b, bytes calldata inputProof) public returns (uint8) {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint8 result = TFHE.and(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function or_euint8_euint4(einput a, einput b, bytes calldata inputProof) public returns (uint8) {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint8 result = TFHE.or(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function xor_euint8_euint4(einput a, einput b, bytes calldata inputProof) public returns (uint8) {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint4 bProc = TFHE.asEuint4(b, inputProof);
        euint8 result = TFHE.xor(aProc, bProc);
        return TFHE.decrypt(result);
    }
}
