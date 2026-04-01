// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../../lib/FHE.sol";
import {CoprocessorSetup} from "../CoprocessorSetup.sol";

/**
 * @title  FHEVMGasProfileSuite
 * @notice Measures EVM gas and HCU for isIn and sum at different array sizes and types.
 *         Inputs are trivially encrypted on-chain — no external proof needed.
 *         The value searched in isIn (42) is always present at index 42, so the result
 *         is always true, giving a consistent worst-case-equivalent workload.
 */
contract FHEVMGasProfileSuite {
    ebool public resEbool;
    euint8 public resEuint8;
    euint16 public resEuint16;
    euint32 public resEuint32;
    euint64 public resEuint64;
    euint128 public resEuint128;

    constructor() {
        FHE.setCoprocessor(CoprocessorSetup.defaultConfig());
    }

    // -------------------------------------------------------------------------
    // isIn — euint8
    // -------------------------------------------------------------------------

    function profile_isIn_euint8(uint256 setSize) public {
        // set = [0, 1, ..., n-1]. Value 42 is present when setSize > 42.
        uint8[] memory set = new uint8[](setSize);
        for (uint256 i = 0; i < setSize; i++) {
            set[i] = uint8(i);
        }
        euint8 value = FHE.asEuint8(42);
        resEbool = FHE.isIn(value, set);
        FHE.allowThis(resEbool);
    }

    // -------------------------------------------------------------------------
    // isIn — euint16
    // -------------------------------------------------------------------------

    function profile_isIn_euint16(uint256 setSize) public {
        uint16[] memory set = new uint16[](setSize);
        for (uint256 i = 0; i < setSize; i++) {
            set[i] = uint16(i % 65536);
        }
        euint16 value = FHE.asEuint16(42);
        resEbool = FHE.isIn(value, set);
        FHE.allowThis(resEbool);
    }

    // -------------------------------------------------------------------------
    // isIn — euint32
    // -------------------------------------------------------------------------

    function profile_isIn_euint32(uint256 setSize) public {
        uint32[] memory set = new uint32[](setSize);
        for (uint256 i = 0; i < setSize; i++) {
            set[i] = uint32(i);
        }
        euint32 value = FHE.asEuint32(42);
        resEbool = FHE.isIn(value, set);
        FHE.allowThis(resEbool);
    }

    // -------------------------------------------------------------------------
    // isIn — euint128
    // -------------------------------------------------------------------------

    function profile_isIn_euint128(uint256 setSize) public {
        uint128[] memory set = new uint128[](setSize);
        for (uint256 i = 0; i < setSize; i++) {
            set[i] = uint128(i);
        }
        euint128 value = FHE.asEuint128(42);
        resEbool = FHE.isIn(value, set);
        FHE.allowThis(resEbool);
    }

    // -------------------------------------------------------------------------
    // isIn — euint64
    // -------------------------------------------------------------------------

    function profile_isIn_euint64(uint256 setSize) public {
        uint64[] memory set = new uint64[](setSize);
        for (uint256 i = 0; i < setSize; i++) {
            set[i] = uint64(i);
        }
        euint64 value = FHE.asEuint64(42);
        resEbool = FHE.isIn(value, set);
        FHE.allowThis(resEbool);
    }

    // -------------------------------------------------------------------------
    // sum — euint8
    // -------------------------------------------------------------------------

    function profile_sum_euint8(uint256 arraySize) public {
        euint8[] memory values = new euint8[](arraySize);
        for (uint256 i = 0; i < arraySize; i++) {
            values[i] = FHE.asEuint8(uint8((i % 255) + 1));
        }
        resEuint8 = FHE.sum(values);
        FHE.allowThis(resEuint8);
    }

    // -------------------------------------------------------------------------
    // sum — euint16
    // -------------------------------------------------------------------------

    function profile_sum_euint16(uint256 arraySize) public {
        euint16[] memory values = new euint16[](arraySize);
        for (uint256 i = 0; i < arraySize; i++) {
            values[i] = FHE.asEuint16(uint16((i % 65535) + 1));
        }
        resEuint16 = FHE.sum(values);
        FHE.allowThis(resEuint16);
    }

    // -------------------------------------------------------------------------
    // sum — euint32
    // -------------------------------------------------------------------------

    function profile_sum_euint32(uint256 arraySize) public {
        euint32[] memory values = new euint32[](arraySize);
        for (uint256 i = 0; i < arraySize; i++) {
            values[i] = FHE.asEuint32(uint32(i + 1));
        }
        resEuint32 = FHE.sum(values);
        FHE.allowThis(resEuint32);
    }

    // -------------------------------------------------------------------------
    // sum — euint128
    // -------------------------------------------------------------------------

    function profile_sum_euint128(uint256 arraySize) public {
        euint128[] memory values = new euint128[](arraySize);
        for (uint256 i = 0; i < arraySize; i++) {
            values[i] = FHE.asEuint128(uint128(i + 1));
        }
        resEuint128 = FHE.sum(values);
        FHE.allowThis(resEuint128);
    }

    // -------------------------------------------------------------------------
    // sum — euint64
    // -------------------------------------------------------------------------

    function profile_sum_euint64(uint256 arraySize) public {
        euint64[] memory values = new euint64[](arraySize);
        for (uint256 i = 0; i < arraySize; i++) {
            values[i] = FHE.asEuint64(uint64(i + 1));
        }
        resEuint64 = FHE.sum(values);
        FHE.allowThis(resEuint64);
    }

    // -------------------------------------------------------------------------
    // isIn — eaddress
    // -------------------------------------------------------------------------

    function profile_isIn_eaddress(uint256 setSize) public {
        address[] memory set = new address[](setSize);
        for (uint256 i = 0; i < setSize; i++) {
            set[i] = address(uint160(i + 1));
        }
        eaddress value = FHE.asEaddress(address(uint160(42)));
        resEbool = FHE.isIn(value, set);
        FHE.allowThis(resEbool);
    }

    // -------------------------------------------------------------------------
    // isIn — euint256
    // -------------------------------------------------------------------------

    function profile_isIn_euint256(uint256 setSize) public {
        uint256[] memory set = new uint256[](setSize);
        for (uint256 i = 0; i < setSize; i++) {
            set[i] = i + 1;
        }
        euint256 value = FHE.asEuint256(42);
        resEbool = FHE.isIn(value, set);
        FHE.allowThis(resEbool);
    }
}
