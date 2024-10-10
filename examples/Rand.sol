// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/TFHE.sol";

/// @notice Contract for generating random encrypted numbers
contract Rand {
    /// @notice Encrypted unsigned integers of various sizes
    euint8 public value8;
    euint16 public value16;
    euint32 public value32;
    euint64 public value64;
    euint64 public value64Bounded;

    /// @notice Constructor to set FHE configuration
    constructor() {
        TFHE.setFHEVM(FHEVMConfig.defaultConfig());
    }

    /// @notice Generate random 8-bit encrypted unsigned integer
    function generate8() public {
        value8 = TFHE.randEuint8();
        TFHE.allowThis(value8);
    }

    /// @notice Generate random 8-bit encrypted unsigned integer with upper bound
    /// @param upperBound The maximum value (exclusive) for the generated number
    function generate8UpperBound(uint8 upperBound) public {
        value8 = TFHE.randEuint8(upperBound);
        TFHE.allowThis(value8);
    }

    /// @notice Generate random 16-bit encrypted unsigned integer
    function generate16() public {
        value16 = TFHE.randEuint16();
        TFHE.allowThis(value16);
    }

    /// @notice Generate random 16-bit encrypted unsigned integer with upper bound
    /// @param upperBound The maximum value (exclusive) for the generated number
    function generate16UpperBound(uint16 upperBound) public {
        value16 = TFHE.randEuint16(upperBound);
        TFHE.allowThis(value16);
    }

    /// @notice Generate random 32-bit encrypted unsigned integer
    function generate32() public {
        value32 = TFHE.randEuint32();
        TFHE.allowThis(value32);
    }

    /// @notice Generate random 32-bit encrypted unsigned integer with upper bound
    /// @param upperBound The maximum value (exclusive) for the generated number
    function generate32UpperBound(uint32 upperBound) public {
        value32 = TFHE.randEuint32(upperBound);
        TFHE.allowThis(value32);
    }

    /// @notice Generate random 64-bit encrypted unsigned integer
    function generate64() public {
        value64 = TFHE.randEuint64();
        TFHE.allowThis(value64);
    }

    /// @notice Generate random 64-bit encrypted unsigned integer with upper bound
    /// @param upperBound The maximum value (exclusive) for the generated number
    function generate64UpperBound(uint32 upperBound) public {
        value64 = TFHE.randEuint64(upperBound);
        TFHE.allowThis(value64);
    }

    /// @notice Generate random 64-bit encrypted unsigned integer with error handling
    /// @dev This function attempts a failing call and then generates a bounded random number
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
