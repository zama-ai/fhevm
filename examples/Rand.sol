// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/TFHE.sol";

// Contract for generating random encrypted numbers
contract Rand {
    // Encrypted unsigned integers of various sizes
    euint8 public value8;
    euint16 public value16;
    euint32 public value32;
    euint64 public value64;
    euint64 public value64Bounded;

    // Constructor to set FHE configuration
    constructor() {
        TFHE.setFHEVM(FHEVMConfig.defaultConfig());
    }

    // Generate random 8-bit encrypted unsigned integer
    function generate8() public {
        value8 = TFHE.randEuint8();
        TFHE.allowThis(value8);
    }

    // Generate random 8-bit encrypted unsigned integer with upper bound
    function generate8UpperBound(uint8 upperBound) public {
        value8 = TFHE.randEuint8(upperBound);
        TFHE.allowThis(value8);
    }

    // Generate random 16-bit encrypted unsigned integer
    function generate16() public {
        value16 = TFHE.randEuint16();
        TFHE.allowThis(value16);
    }

    // Generate random 16-bit encrypted unsigned integer with upper bound
    function generate16UpperBound(uint16 upperBound) public {
        value16 = TFHE.randEuint16(upperBound);
        TFHE.allowThis(value16);
    }

    // Generate random 32-bit encrypted unsigned integer
    function generate32() public {
        value32 = TFHE.randEuint32();
        TFHE.allowThis(value32);
    }

    // Generate random 32-bit encrypted unsigned integer with upper bound
    function generate32UpperBound(uint32 upperBound) public {
        value32 = TFHE.randEuint32(upperBound);
        TFHE.allowThis(value32);
    }

    // Generate random 64-bit encrypted unsigned integer
    function generate64() public {
        value64 = TFHE.randEuint64();
        TFHE.allowThis(value64);
    }

    // Generate random 64-bit encrypted unsigned integer with upper bound
    function generate64UpperBound(uint32 upperBound) public {
        value64 = TFHE.randEuint64(upperBound);
        TFHE.allowThis(value64);
    }

    // Generate random 64-bit encrypted unsigned integer with error handling
    function generate64Reverting() public {
        try this.failingCall() {} catch {}
        value64Bounded = TFHE.randEuint64(1024);
        TFHE.allowThis(value64Bounded);
    }

    // Function that always reverts after generating a random number
    function failingCall() public {
        value64 = TFHE.randEuint64();
        TFHE.allowThis(value64);
        revert();
    }
}
