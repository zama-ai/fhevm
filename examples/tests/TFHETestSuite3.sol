// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity 0.8.19;

import "../../lib/TFHE.sol";

contract TFHETestSuite3 {
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

    function shl_euint32_uint32(bytes calldata a, uint32 b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        uint32 bProc = b;
        euint32 result = TFHE.shl(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shl_uint32_euint32(uint32 a, bytes calldata b) public view returns (uint32) {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b);
        euint32 result = TFHE.shl(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shr_euint32_uint32(bytes calldata a, uint32 b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        uint32 bProc = b;
        euint32 result = TFHE.shr(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shr_uint32_euint32(uint32 a, bytes calldata b) public view returns (uint32) {
        uint32 aProc = a;
        euint32 bProc = TFHE.asEuint32(b);
        euint32 result = TFHE.shr(aProc, bProc);
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

    function neg_euint8(bytes calldata a) public view returns (uint8) {
        euint8 aProc = TFHE.asEuint8(a);
        euint8 result = TFHE.neg(aProc);
        return TFHE.decrypt(result);
    }

    function not_euint8(bytes calldata a) public view returns (uint8) {
        euint8 aProc = TFHE.asEuint8(a);
        euint8 result = TFHE.not(aProc);
        return TFHE.decrypt(result);
    }

    function neg_euint16(bytes calldata a) public view returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a);
        euint16 result = TFHE.neg(aProc);
        return TFHE.decrypt(result);
    }

    function not_euint16(bytes calldata a) public view returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a);
        euint16 result = TFHE.not(aProc);
        return TFHE.decrypt(result);
    }

    function neg_euint32(bytes calldata a) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint32 result = TFHE.neg(aProc);
        return TFHE.decrypt(result);
    }

    function not_euint32(bytes calldata a) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint32 result = TFHE.not(aProc);
        return TFHE.decrypt(result);
    }

    function bin_op_add_euint8_euint8(bytes calldata a, bytes calldata b) public view returns (uint8) {
        euint8 aProc = TFHE.asEuint8(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint8 result = aProc + bProc;
        return TFHE.decrypt(result);
    }

    function bin_op_sub_euint8_euint8(bytes calldata a, bytes calldata b) public view returns (uint8) {
        euint8 aProc = TFHE.asEuint8(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint8 result = aProc - bProc;
        return TFHE.decrypt(result);
    }

    function bin_op_mul_euint8_euint8(bytes calldata a, bytes calldata b) public view returns (uint8) {
        euint8 aProc = TFHE.asEuint8(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint8 result = aProc * bProc;
        return TFHE.decrypt(result);
    }

    function bin_op_and_euint8_euint8(bytes calldata a, bytes calldata b) public view returns (uint8) {
        euint8 aProc = TFHE.asEuint8(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint8 result = aProc & bProc;
        return TFHE.decrypt(result);
    }

    function bin_op_or_euint8_euint8(bytes calldata a, bytes calldata b) public view returns (uint8) {
        euint8 aProc = TFHE.asEuint8(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint8 result = aProc | bProc;
        return TFHE.decrypt(result);
    }

    function bin_op_xor_euint8_euint8(bytes calldata a, bytes calldata b) public view returns (uint8) {
        euint8 aProc = TFHE.asEuint8(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint8 result = aProc ^ bProc;
        return TFHE.decrypt(result);
    }

    function unary_op_neg_euint8(bytes calldata a) public view returns (uint8) {
        euint8 aProc = TFHE.asEuint8(a);
        euint8 result = -aProc;
        return TFHE.decrypt(result);
    }

    function unary_op_not_euint8(bytes calldata a) public view returns (uint8) {
        euint8 aProc = TFHE.asEuint8(a);
        euint8 result = ~aProc;
        return TFHE.decrypt(result);
    }

    function bin_op_add_euint16_euint16(bytes calldata a, bytes calldata b) public view returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a);
        euint16 bProc = TFHE.asEuint16(b);
        euint16 result = aProc + bProc;
        return TFHE.decrypt(result);
    }

    function bin_op_sub_euint16_euint16(bytes calldata a, bytes calldata b) public view returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a);
        euint16 bProc = TFHE.asEuint16(b);
        euint16 result = aProc - bProc;
        return TFHE.decrypt(result);
    }

    function bin_op_mul_euint16_euint16(bytes calldata a, bytes calldata b) public view returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a);
        euint16 bProc = TFHE.asEuint16(b);
        euint16 result = aProc * bProc;
        return TFHE.decrypt(result);
    }

    function bin_op_and_euint16_euint16(bytes calldata a, bytes calldata b) public view returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a);
        euint16 bProc = TFHE.asEuint16(b);
        euint16 result = aProc & bProc;
        return TFHE.decrypt(result);
    }

    function bin_op_or_euint16_euint16(bytes calldata a, bytes calldata b) public view returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a);
        euint16 bProc = TFHE.asEuint16(b);
        euint16 result = aProc | bProc;
        return TFHE.decrypt(result);
    }

    function bin_op_xor_euint16_euint16(bytes calldata a, bytes calldata b) public view returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a);
        euint16 bProc = TFHE.asEuint16(b);
        euint16 result = aProc ^ bProc;
        return TFHE.decrypt(result);
    }

    function unary_op_neg_euint16(bytes calldata a) public view returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a);
        euint16 result = -aProc;
        return TFHE.decrypt(result);
    }

    function unary_op_not_euint16(bytes calldata a) public view returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a);
        euint16 result = ~aProc;
        return TFHE.decrypt(result);
    }

    function bin_op_add_euint32_euint32(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint32 bProc = TFHE.asEuint32(b);
        euint32 result = aProc + bProc;
        return TFHE.decrypt(result);
    }

    function bin_op_sub_euint32_euint32(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint32 bProc = TFHE.asEuint32(b);
        euint32 result = aProc - bProc;
        return TFHE.decrypt(result);
    }

    function bin_op_mul_euint32_euint32(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint32 bProc = TFHE.asEuint32(b);
        euint32 result = aProc * bProc;
        return TFHE.decrypt(result);
    }

    function bin_op_and_euint32_euint32(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint32 bProc = TFHE.asEuint32(b);
        euint32 result = aProc & bProc;
        return TFHE.decrypt(result);
    }

    function bin_op_or_euint32_euint32(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint32 bProc = TFHE.asEuint32(b);
        euint32 result = aProc | bProc;
        return TFHE.decrypt(result);
    }

    function bin_op_xor_euint32_euint32(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint32 bProc = TFHE.asEuint32(b);
        euint32 result = aProc ^ bProc;
        return TFHE.decrypt(result);
    }

    function unary_op_neg_euint32(bytes calldata a) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint32 result = -aProc;
        return TFHE.decrypt(result);
    }

    function unary_op_not_euint32(bytes calldata a) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint32 result = ~aProc;
        return TFHE.decrypt(result);
    }
}
