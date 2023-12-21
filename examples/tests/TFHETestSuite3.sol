// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity 0.8.19;

import "../../lib/TFHE.sol";

contract TFHETestSuite3 {
    function shr_euint8_euint8(bytes calldata a, bytes calldata b) public view returns (uint8) {
        euint8 aProc = TFHE.asEuint8(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint8 result = TFHE.shr(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shr_euint8_uint8(bytes calldata a, uint8 b) public view returns (uint8) {
        euint8 aProc = TFHE.asEuint8(a);
        uint8 bProc = b;
        euint8 result = TFHE.shr(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shl_euint16_euint8(bytes calldata a, bytes calldata b) public view returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint16 result = TFHE.shl(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shl_euint16_uint8(bytes calldata a, uint8 b) public view returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a);
        uint8 bProc = b;
        euint16 result = TFHE.shl(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shr_euint16_euint8(bytes calldata a, bytes calldata b) public view returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint16 result = TFHE.shr(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shr_euint16_uint8(bytes calldata a, uint8 b) public view returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a);
        uint8 bProc = b;
        euint16 result = TFHE.shr(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shl_euint32_euint8(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint32 result = TFHE.shl(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shl_euint32_uint8(bytes calldata a, uint8 b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        uint8 bProc = b;
        euint32 result = TFHE.shl(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shr_euint32_euint8(bytes calldata a, bytes calldata b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        euint8 bProc = TFHE.asEuint8(b);
        euint32 result = TFHE.shr(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shr_euint32_uint8(bytes calldata a, uint8 b) public view returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a);
        uint8 bProc = b;
        euint32 result = TFHE.shr(aProc, bProc);
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
