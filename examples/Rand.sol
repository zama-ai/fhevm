// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.20;

import "../lib/TFHE.sol";

contract Rand {
    euint8 value8;
    euint16 value16;
    euint32 value32;
    euint64 value64;
    uint8 public value8Decrypted;
    uint16 public value16Decrypted;
    uint32 public value32Decrypted;
    uint64 public value64Decrypted;

    function generate8() public {
        value8 = TFHE.randEuint8();
    }

    function generate8UpperBound(uint8 upperBound) public {
        value8 = TFHE.randEuint8(upperBound);
    }

    function generate16() public {
        value16 = TFHE.randEuint16();
    }

    function generate16UpperBound(uint16 upperBound) public {
        value16 = TFHE.randEuint16(upperBound);
    }

    function generate32() public {
        value32 = TFHE.randEuint32();
    }

    function generate32UpperBound(uint32 upperBound) public {
        value32 = TFHE.randEuint32(upperBound);
    }

    function generate64() public {
        value64 = TFHE.randEuint64();
    }

    function generate64UpperBound(uint32 upperBound) public {
        value64 = TFHE.randEuint64(upperBound);
    }

    function decrypt8() public view returns (uint8) {
        return TFHE.decrypt(value8);
    }

    function decrypt16() public view returns (uint16) {
        return TFHE.decrypt(value16);
    }

    function decrypt32() public view returns (uint32) {
        return TFHE.decrypt(value32);
    }

    function decrypt64() public view returns (uint64) {
        return TFHE.decrypt(value64);
    }

    function decryptAndStore8() public {
        value8Decrypted = TFHE.decrypt(value8);
    }

    function decryptAndStore16() public {
        value16Decrypted = TFHE.decrypt(value16);
    }

    function decryptAndStore32() public {
        value32Decrypted = TFHE.decrypt(value32);
    }

    function decryptAndStore64() public {
        value64Decrypted = TFHE.decrypt(value64);
    }

    // Must fail.
    function generate8InView() public view {
        TFHE.randEuint8();
    }

    // Must fail.
    function generate8UpperBoundInView(uint8 upperBound) public view {
        TFHE.randEuint8(upperBound);
    }

    // Must fail.
    function generate16InView() public view {
        TFHE.randEuint16();
    }

    // Must fail.
    function generate16UpperBoundInView(uint16 upperBound) public view {
        TFHE.randEuint16(upperBound);
    }

    // Must fail.
    function generate32InView() public view {
        TFHE.randEuint32();
    }

    // Must fail.
    function generate32UpperBoundInView(uint32 upperBound) public view {
        TFHE.randEuint32(upperBound);
    }

    // Must fail.
    function generate64InView() public view {
        TFHE.randEuint64();
    }

    // Must fail.
    function generate64UpperBoundInView(uint32 upperBound) public view {
        TFHE.randEuint64(upperBound);
    }
}
