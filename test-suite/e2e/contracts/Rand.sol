// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "./E2ECoprocessorConfigLocal.sol";

/// @notice Contract for generating random encrypted numbers
contract Rand is E2ECoprocessorConfig {
    /// @notice Encrypted unsigned integers of various sizes
    ebool public valueb;
    euint8 public value8;
    euint16 public value16;
    euint32 public value32;
    euint64 public value64;
    euint64 public value64Bounded;
    euint128 public value128;
    euint256 public value256;

    /// @notice Generate random 8-bit encrypted unsigned integer
    function generateBool() public {
        valueb = FHE.randEbool();
        FHE.makePubliclyDecryptable(valueb);
    }

    function generate8() public {
        value8 = FHE.randEuint8();
        FHE.makePubliclyDecryptable(value8);
    }

    /// @notice Generate random 8-bit encrypted unsigned integer with upper bound
    /// @param upperBound The maximum value (exclusive) for the generated number
    function generate8UpperBound(uint8 upperBound) public {
        value8 = FHE.randEuint8(upperBound);
        FHE.makePubliclyDecryptable(value8);
    }

    /// @notice Generate random 16-bit encrypted unsigned integer
    function generate16() public {
        value16 = FHE.randEuint16();
        FHE.makePubliclyDecryptable(value16);
    }

    /// @notice Generate random 16-bit encrypted unsigned integer with upper bound
    /// @param upperBound The maximum value (exclusive) for the generated number
    function generate16UpperBound(uint16 upperBound) public {
        value16 = FHE.randEuint16(upperBound);
        FHE.makePubliclyDecryptable(value16);
    }

    /// @notice Generate random 32-bit encrypted unsigned integer
    function generate32() public {
        value32 = FHE.randEuint32();
        FHE.makePubliclyDecryptable(value32);
    }

    /// @notice Generate random 32-bit encrypted unsigned integer with upper bound
    /// @param upperBound The maximum value (exclusive) for the generated number
    function generate32UpperBound(uint32 upperBound) public {
        value32 = FHE.randEuint32(upperBound);
        FHE.makePubliclyDecryptable(value32);
    }

    /// @notice Generate random 64-bit encrypted unsigned integer
    function generate64() public {
        value64 = FHE.randEuint64();
        FHE.makePubliclyDecryptable(value64);
    }

    function generate64UpperBound(uint64 upperBound) public {
        value64 = FHE.randEuint64(upperBound);
        FHE.makePubliclyDecryptable(value64);
    }

    /// @notice Generate random 64-bit encrypted unsigned integer with error handling
    /// @dev This function attempts a failing call and then generates a bounded random number
    function generate64Reverting() public {
        try this.failingCall() {} catch {}
        value64Bounded = FHE.randEuint64(1024);
        FHE.makePubliclyDecryptable(value64Bounded);
    }

    // Function that always reverts after generating a random number
    function failingCall() public {
        value64 = FHE.randEuint64();
        FHE.makePubliclyDecryptable(value64);
        revert();
    }

    function generate128() public {
        value128 = FHE.randEuint128();
        FHE.makePubliclyDecryptable(value128);
    }

    function generate128UpperBound(uint128 upperBound) public {
        value128 = FHE.randEuint128(upperBound);
        FHE.makePubliclyDecryptable(value128);
    }

    function generate256() public {
        value256 = FHE.randEuint256();
        FHE.makePubliclyDecryptable(value256);
    }

    function generate256UpperBound(uint256 upperBound) public {
        value256 = FHE.randEuint256(upperBound);
        FHE.makePubliclyDecryptable(value256);
    }
}
