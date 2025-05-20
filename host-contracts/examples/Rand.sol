// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/FHE.sol";

import "../lib/FHEVMConfig.sol";

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
        FHE.setCoprocessor(FHEVMConfig.defaultConfig());
    }

    /// @notice Generate random 8-bit encrypted unsigned integer
    function generateBool() public {
        valueb = FHE.randEbool();
        FHE.allowThis(valueb);
    }

    function generate8() public {
        value8 = FHE.randEuint8();
        FHE.allowThis(value8);
    }

    /// @notice Generate random 8-bit encrypted unsigned integer with upper bound
    /// @param upperBound The maximum value (exclusive) for the generated number
    function generate8UpperBound(uint8 upperBound) public {
        value8 = FHE.randEuint8(upperBound);
        FHE.allowThis(value8);
    }

    /// @notice Generate random 16-bit encrypted unsigned integer
    function generate16() public {
        value16 = FHE.randEuint16();
        FHE.allowThis(value16);
    }

    /// @notice Generate random 16-bit encrypted unsigned integer with upper bound
    /// @param upperBound The maximum value (exclusive) for the generated number
    function generate16UpperBound(uint16 upperBound) public {
        value16 = FHE.randEuint16(upperBound);
        FHE.allowThis(value16);
    }

    /// @notice Generate random 32-bit encrypted unsigned integer
    function generate32() public {
        value32 = FHE.randEuint32();
        FHE.allowThis(value32);
    }

    /// @notice Generate random 32-bit encrypted unsigned integer with upper bound
    /// @param upperBound The maximum value (exclusive) for the generated number
    function generate32UpperBound(uint32 upperBound) public {
        value32 = FHE.randEuint32(upperBound);
        FHE.allowThis(value32);
    }

    /// @notice Generate random 64-bit encrypted unsigned integer
    function generate64() public {
        value64 = FHE.randEuint64();
        FHE.allowThis(value64);
    }

    function generate64UpperBound(uint64 upperBound) public {
        value64 = FHE.randEuint64(upperBound);
        FHE.allowThis(value64);
    }

    /// @notice Generate random 64-bit encrypted unsigned integer with error handling
    /// @dev This function attempts a failing call and then generates a bounded random number
    function generate64Reverting() public {
        try this.failingCall() {} catch {}
        value64Bounded = FHE.randEuint64(1024);
        FHE.allowThis(value64Bounded);
    }

    // Function that always reverts after generating a random number
    function failingCall() public {
        value64 = FHE.randEuint64();
        FHE.allowThis(value64);
        revert();
    }

    function generate128() public {
        value128 = FHE.randEuint128();
        FHE.allowThis(value128);
    }

    function generate128UpperBound(uint128 upperBound) public {
        value128 = FHE.randEuint128(upperBound);
        FHE.allowThis(value128);
    }

    function generate256() public {
        value256 = FHE.randEuint256();
        FHE.allowThis(value256);
    }

    function generate256UpperBound(uint256 upperBound) public {
        value256 = FHE.randEuint256(upperBound);
        FHE.allowThis(value256);
    }

    function generate512() public {
        value512 = FHE.randEbytes64();
        FHE.allowThis(value512);
    }

    function generate1024() public {
        value1024 = FHE.randEbytes128();
        FHE.allowThis(value1024);
    }

    function generate2048() public {
        value2048 = FHE.randEbytes256();
        FHE.allowThis(value2048);
    }
}
