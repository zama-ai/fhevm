// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import { E2ECoprocessorConfig } from "./E2ECoprocessorConfig.sol";

/// @notice Contract for demonstrating user decryption of various FHE data types
contract UserDecrypt is E2ECoprocessorConfig {
    /// @dev Encrypted boolean
    ebool public xBool;
    /// @dev Encrypted 8-bit unsigned integer
    euint8 public xUint8;
    /// @dev Encrypted 16-bit unsigned integer
    euint16 public xUint16;
    /// @dev Encrypted 32-bit unsigned integer
    euint32 public xUint32;
    /// @dev Encrypted 64-bit unsigned integer
    euint64 public xUint64;
    euint128 public xUint128;
    eaddress public xAddress;
    euint256 public xUint256;

    /// @notice Constructor to initialize encrypted values and set permissions
    constructor() {
        // Initialize and set permissions for xBool
        xBool = FHE.asEbool(true);
        FHE.allowThis(xBool);
        FHE.allow(xBool, msg.sender);

        // Initialize and set permissions for xUint8
        xUint8 = FHE.asEuint8(42);
        FHE.allowThis(xUint8);
        FHE.allow(xUint8, msg.sender);

        // Initialize and set permissions for xUint16
        xUint16 = FHE.asEuint16(16);
        FHE.allowThis(xUint16);
        FHE.allow(xUint16, msg.sender);

        // Initialize and set permissions for xUint32
        xUint32 = FHE.asEuint32(32);
        FHE.allowThis(xUint32);
        FHE.allow(xUint32, msg.sender);

        // Initialize and set permissions for xUint64
        xUint64 = FHE.asEuint64(18446744073709551600);
        FHE.allowThis(xUint64);
        FHE.allow(xUint64, msg.sender);

        xUint128 = FHE.asEuint128(145275933516363203950142179850024740765);
        FHE.allowThis(xUint128);
        FHE.allow(xUint128, msg.sender);

        xAddress = FHE.asEaddress(0x8ba1f109551bD432803012645Ac136ddd64DBA72);
        FHE.allowThis(xAddress);
        FHE.allow(xAddress, msg.sender);

        xUint256 = FHE.asEuint256(74285495974541385002137713624115238327312291047062397922780925695323480915729);
        FHE.allowThis(xUint256);
        FHE.allow(xUint256, msg.sender);
    }
}
