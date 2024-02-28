// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.20;

import "../../lib/TFHE.sol";

contract TFHETestSuite6 {
    function bin_op_mul_euint4_euint4(bytes calldata a, bytes calldata b) public view returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a);
        euint4 bProc = TFHE.asEuint4(b);
        euint4 result = aProc * bProc;
        return TFHE.decrypt(result);
    }

    function bin_op_and_euint4_euint4(bytes calldata a, bytes calldata b) public view returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a);
        euint4 bProc = TFHE.asEuint4(b);
        euint4 result = aProc & bProc;
        return TFHE.decrypt(result);
    }

    function bin_op_or_euint4_euint4(bytes calldata a, bytes calldata b) public view returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a);
        euint4 bProc = TFHE.asEuint4(b);
        euint4 result = aProc | bProc;
        return TFHE.decrypt(result);
    }

    function bin_op_xor_euint4_euint4(bytes calldata a, bytes calldata b) public view returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a);
        euint4 bProc = TFHE.asEuint4(b);
        euint4 result = aProc ^ bProc;
        return TFHE.decrypt(result);
    }

    function unary_op_neg_euint4(bytes calldata a) public view returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a);
        euint4 result = -aProc;
        return TFHE.decrypt(result);
    }

    function unary_op_not_euint4(bytes calldata a) public view returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a);
        euint4 result = ~aProc;
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

    function bin_op_add_euint64_euint64(bytes calldata a, bytes calldata b) public view returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint64 bProc = TFHE.asEuint64(b);
        euint64 result = aProc + bProc;
        return TFHE.decrypt(result);
    }

    function bin_op_sub_euint64_euint64(bytes calldata a, bytes calldata b) public view returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint64 bProc = TFHE.asEuint64(b);
        euint64 result = aProc - bProc;
        return TFHE.decrypt(result);
    }

    function bin_op_mul_euint64_euint64(bytes calldata a, bytes calldata b) public view returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint64 bProc = TFHE.asEuint64(b);
        euint64 result = aProc * bProc;
        return TFHE.decrypt(result);
    }

    function bin_op_and_euint64_euint64(bytes calldata a, bytes calldata b) public view returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint64 bProc = TFHE.asEuint64(b);
        euint64 result = aProc & bProc;
        return TFHE.decrypt(result);
    }

    function bin_op_or_euint64_euint64(bytes calldata a, bytes calldata b) public view returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint64 bProc = TFHE.asEuint64(b);
        euint64 result = aProc | bProc;
        return TFHE.decrypt(result);
    }

    function bin_op_xor_euint64_euint64(bytes calldata a, bytes calldata b) public view returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint64 bProc = TFHE.asEuint64(b);
        euint64 result = aProc ^ bProc;
        return TFHE.decrypt(result);
    }

    function unary_op_neg_euint64(bytes calldata a) public view returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint64 result = -aProc;
        return TFHE.decrypt(result);
    }

    function unary_op_not_euint64(bytes calldata a) public view returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a);
        euint64 result = ~aProc;
        return TFHE.decrypt(result);
    }
}
