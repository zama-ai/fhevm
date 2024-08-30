// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../../lib/TFHE.sol";
import "../../payment/Payment.sol";

contract TFHETestSuite6 {
    ebool public resb;
    euint4 public res4;
    euint8 public res8;
    euint16 public res16;
    euint32 public res32;
    euint64 public res64;

    constructor() payable {
        Payment.depositForThis(msg.value);
    }

    function shr_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint64 result = TFHE.shr(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function shr_euint64_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint8 bProc = b;
        euint64 result = TFHE.shr(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function rotl_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint64 result = TFHE.rotl(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function rotl_euint64_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint8 bProc = b;
        euint64 result = TFHE.rotl(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function rotr_euint64_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint64 result = TFHE.rotr(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function rotr_euint64_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        uint8 bProc = b;
        euint64 result = TFHE.rotr(aProc, bProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function neg_euint4(einput a, bytes calldata inputProof) public {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint4 result = TFHE.neg(aProc);
        TFHE.allowThis(result);
        res4 = result;
    }
    function not_euint4(einput a, bytes calldata inputProof) public {
        euint4 aProc = TFHE.asEuint4(a, inputProof);
        euint4 result = TFHE.not(aProc);
        TFHE.allowThis(result);
        res4 = result;
    }
    function neg_euint8(einput a, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint8 result = TFHE.neg(aProc);
        TFHE.allowThis(result);
        res8 = result;
    }
    function not_euint8(einput a, bytes calldata inputProof) public {
        euint8 aProc = TFHE.asEuint8(a, inputProof);
        euint8 result = TFHE.not(aProc);
        TFHE.allowThis(result);
        res8 = result;
    }
    function neg_euint16(einput a, bytes calldata inputProof) public {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint16 result = TFHE.neg(aProc);
        TFHE.allowThis(result);
        res16 = result;
    }
    function not_euint16(einput a, bytes calldata inputProof) public {
        euint16 aProc = TFHE.asEuint16(a, inputProof);
        euint16 result = TFHE.not(aProc);
        TFHE.allowThis(result);
        res16 = result;
    }
    function neg_euint32(einput a, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint32 result = TFHE.neg(aProc);
        TFHE.allowThis(result);
        res32 = result;
    }
    function not_euint32(einput a, bytes calldata inputProof) public {
        euint32 aProc = TFHE.asEuint32(a, inputProof);
        euint32 result = TFHE.not(aProc);
        TFHE.allowThis(result);
        res32 = result;
    }
    function neg_euint64(einput a, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 result = TFHE.neg(aProc);
        TFHE.allowThis(result);
        res64 = result;
    }
    function not_euint64(einput a, bytes calldata inputProof) public {
        euint64 aProc = TFHE.asEuint64(a, inputProof);
        euint64 result = TFHE.not(aProc);
        TFHE.allowThis(result);
        res64 = result;
    }
}
