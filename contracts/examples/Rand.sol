// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/HTTPZ.sol";

import "../lib/HTTPZConfig.sol";

/// @notice Contract for generating random encrypted numbers
contract Rand {
    /// @notice Encrypted unsigned integers of various sizes
    ebool public valueb;
    euint8 public value8;
    euint16 public value16;
    euint32 public value32;
    euint64 public value64;
    euint64 public value64Bounded;
    euint128 public value128;
    euint256 public value256;
    ebytes64 public value512;
    ebytes128 public value1024;
    ebytes256 public value2048;

    /// @notice Constructor to set FHE configuration
    constructor() {
        HTTPZ.setCoprocessor(HTTPZConfig.defaultConfig());
    }

    /// @notice Generate random 8-bit encrypted unsigned integer
    function generateBool() public {
        valueb = HTTPZ.randEbool();
        HTTPZ.allowThis(valueb);
    }

    function generate8() public {
        value8 = HTTPZ.randEuint8();
        HTTPZ.allowThis(value8);
    }

    /// @notice Generate random 8-bit encrypted unsigned integer with upper bound
    /// @param upperBound The maximum value (exclusive) for the generated number
    function generate8UpperBound(uint8 upperBound) public {
        value8 = HTTPZ.randEuint8(upperBound);
        HTTPZ.allowThis(value8);
    }

    /// @notice Generate random 16-bit encrypted unsigned integer
    function generate16() public {
        value16 = HTTPZ.randEuint16();
        HTTPZ.allowThis(value16);
    }

    /// @notice Generate random 16-bit encrypted unsigned integer with upper bound
    /// @param upperBound The maximum value (exclusive) for the generated number
    function generate16UpperBound(uint16 upperBound) public {
        value16 = HTTPZ.randEuint16(upperBound);
        HTTPZ.allowThis(value16);
    }

    /// @notice Generate random 32-bit encrypted unsigned integer
    function generate32() public {
        value32 = HTTPZ.randEuint32();
        HTTPZ.allowThis(value32);
    }

    /// @notice Generate random 32-bit encrypted unsigned integer with upper bound
    /// @param upperBound The maximum value (exclusive) for the generated number
    function generate32UpperBound(uint32 upperBound) public {
        value32 = HTTPZ.randEuint32(upperBound);
        HTTPZ.allowThis(value32);
    }

    /// @notice Generate random 64-bit encrypted unsigned integer
    function generate64() public {
        value64 = HTTPZ.randEuint64();
        HTTPZ.allowThis(value64);
    }

    function generate64UpperBound(uint64 upperBound) public {
        value64 = HTTPZ.randEuint64(upperBound);
        HTTPZ.allowThis(value64);
    }

    /// @notice Generate random 64-bit encrypted unsigned integer with error handling
    /// @dev This function attempts a failing call and then generates a bounded random number
    function generate64Reverting() public {
        try this.failingCall() {} catch {}
        value64Bounded = HTTPZ.randEuint64(1024);
        HTTPZ.allowThis(value64Bounded);
    }

    // Function that always reverts after generating a random number
    function failingCall() public {
        value64 = HTTPZ.randEuint64();
        HTTPZ.allowThis(value64);
        revert();
    }

    function generate128() public {
        value128 = HTTPZ.randEuint128();
        HTTPZ.allowThis(value128);
    }

    function generate128UpperBound(uint128 upperBound) public {
        value128 = HTTPZ.randEuint128(upperBound);
        HTTPZ.allowThis(value128);
    }

    function generate256() public {
        value256 = HTTPZ.randEuint256();
        HTTPZ.allowThis(value256);
    }

    function generate256UpperBound(uint256 upperBound) public {
        value256 = HTTPZ.randEuint256(upperBound);
        HTTPZ.allowThis(value256);
    }

    function generate512() public {
        value512 = HTTPZ.randEbytes64();
        HTTPZ.allowThis(value512);
    }

    function generate1024() public {
        value1024 = HTTPZ.randEbytes128();
        HTTPZ.allowThis(value1024);
    }

    function generate2048() public {
        value2048 = HTTPZ.randEbytes256();
        HTTPZ.allowThis(value2048);
    }
}
