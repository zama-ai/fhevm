// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../../lib/TFHE.sol";
import "../../lib/FHEVMConfig.sol";

contract TFHETestSuite9 {
    ebool public resb;
    euint8 public res8;
    euint16 public res16;
    euint32 public res32;
    euint64 public res64;
    euint128 public res128;
    euint256 public res256;

    constructor() {
        TFHE.setFHEVM(FHEVMConfig.defaultConfig());
    }

    function rotl_euint128_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint128 result = TFHE.rotl(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function rotl_euint128_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        uint8 bProc = b;
        euint128 result = TFHE.rotl(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function rotr_euint128_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint128 result = TFHE.rotr(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function rotr_euint128_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        uint8 bProc = b;
        euint128 result = TFHE.rotr(aProc, bProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function shl_euint256_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint256 result = TFHE.shl(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function shl_euint256_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        uint8 bProc = b;
        euint256 result = TFHE.shl(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function shr_euint256_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint256 result = TFHE.shr(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function shr_euint256_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        uint8 bProc = b;
        euint256 result = TFHE.shr(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function rotl_euint256_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint256 result = TFHE.rotl(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function rotl_euint256_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        uint8 bProc = b;
        euint256 result = TFHE.rotl(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function rotr_euint256_euint8(einput a, einput b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint8 bProc = TFHE.asEuint8(b, inputProof);
        euint256 result = TFHE.rotr(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function rotr_euint256_uint8(einput a, uint8 b, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        uint8 bProc = b;
        euint256 result = TFHE.rotr(aProc, bProc);
        TFHE.allowThis(result);
        res256 = result;
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
    function neg_euint128(einput a, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        euint128 result = TFHE.neg(aProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function not_euint128(einput a, bytes calldata inputProof) public {
        euint128 aProc = TFHE.asEuint128(a, inputProof);
        euint128 result = TFHE.not(aProc);
        TFHE.allowThis(result);
        res128 = result;
    }
    function neg_euint256(einput a, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint256 result = TFHE.neg(aProc);
        TFHE.allowThis(result);
        res256 = result;
    }
    function not_euint256(einput a, bytes calldata inputProof) public {
        euint256 aProc = TFHE.asEuint256(a, inputProof);
        euint256 result = TFHE.not(aProc);
        TFHE.allowThis(result);
        res256 = result;
    }
}
