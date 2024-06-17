// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.20;

import "../../lib/TFHE.sol";

contract TFHETestSuite6 {
    function shr_euint64_euint8(einput a, einput b, bytes calldata inputProof) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint64 result = TFHE.shr(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function shr_euint64_uint8(einput a, uint8 b, bytes calldata inputProof) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint8 bProc = b;
        euint64 result = TFHE.shr(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function rotl_euint64_euint8(einput a, einput b, bytes calldata inputProof) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint64 result = TFHE.rotl(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function rotl_euint64_uint8(einput a, uint8 b, bytes calldata inputProof) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint8 bProc = b;
        euint64 result = TFHE.rotl(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function rotr_euint64_euint8(einput a, einput b, bytes calldata inputProof) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint64 result = TFHE.rotr(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function rotr_euint64_uint8(einput a, uint8 b, bytes calldata inputProof) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint8 bProc = b;
        euint64 result = TFHE.rotr(aProc, bProc);
        return TFHE.decrypt(result);
    }

    function neg_euint4(einput a, bytes calldata inputProof) public returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint4 result = TFHE.neg(aProc);
        return TFHE.decrypt(result);
    }

    function not_euint4(einput a, bytes calldata inputProof) public returns (uint8) {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint4 result = TFHE.not(aProc);
        return TFHE.decrypt(result);
    }

    function neg_euint8(einput a, bytes calldata inputProof) public returns (uint8) {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint8 result = TFHE.neg(aProc);
        return TFHE.decrypt(result);
    }

    function not_euint8(einput a, bytes calldata inputProof) public returns (uint8) {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint8 result = TFHE.not(aProc);
        return TFHE.decrypt(result);
    }

    function neg_euint16(einput a, bytes calldata inputProof) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint16 result = TFHE.neg(aProc);
        return TFHE.decrypt(result);
    }

    function not_euint16(einput a, bytes calldata inputProof) public returns (uint16) {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint16 result = TFHE.not(aProc);
        return TFHE.decrypt(result);
    }

    function neg_euint32(einput a, bytes calldata inputProof) public returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint32 result = TFHE.neg(aProc);
        return TFHE.decrypt(result);
    }

    function not_euint32(einput a, bytes calldata inputProof) public returns (uint32) {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint32 result = TFHE.not(aProc);
        return TFHE.decrypt(result);
    }

    function neg_euint64(einput a, bytes calldata inputProof) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 result = TFHE.neg(aProc);
        return TFHE.decrypt(result);
    }

    function not_euint64(einput a, bytes calldata inputProof) public returns (uint64) {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 result = TFHE.not(aProc);
        return TFHE.decrypt(result);
    }
}
