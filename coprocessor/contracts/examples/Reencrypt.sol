// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/FHE.sol";
import "../lib/FHEVMConfig.sol";

/// @notice Contract for demonstrating reencryption of various FHE data types
contract Reencrypt {
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
    ebytes64 public yBytes64;
    ebytes128 public yBytes128;
    ebytes256 public yBytes256;

    /// @notice Constructor to initialize encrypted values and set permissions
    constructor() {
        // Set default FHE configuration
        FHE.setCoprocessor(FHEVMConfig.defaultConfig());

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

        yBytes64 = FHE.asEbytes64(
            FHE.padToBytes64(
                hex"19d179e0cc7e816dc944582ed4f5652f5951900098fc2e0a15a7ea4dc8cfa4e3b6c54beea5ee95e56b728762f659347ce1d4aa1b05fcc5"
            )
        );
        FHE.allowThis(yBytes64);
        FHE.allow(yBytes64, msg.sender);

        yBytes128 = FHE.asEbytes128(
            FHE.padToBytes128(
                hex"13e7819123de6e2870c7e83bb764508e22d7c3ab8a5aee6bdfb26355ef0d3f1977d651b83bf5f78634fa360aa14debdc3daa6a587b5c2fb1710ab4d6677e62a8577f2d9fecc190ad8b11c9f0a5ec3138b27da1f055437af8c90a9495dad230"
            )
        );
        FHE.allowThis(yBytes128);
        FHE.allow(yBytes128, msg.sender);

        yBytes256 = FHE.asEbytes256(
            FHE.padToBytes256(
                hex"d179e0cc7e816dc944582ed4f5652f5951900098fc2e0a15a7ea4dc8cfa4e3b6c54beea5ee95e56b728762f659347ce1d4aa1b05fcc513e7819123de6e2870c7e83bb764508e22d7c3ab8a5aee6bdfb26355ef0d3f1977d651b83bf5f78634fa360aa14debdc3daa6a587b5c2fb1710ab4d6677e62a8577f2d9fecc190ad8b11c9f0a5ec3138b27da1f055437af8c90a9495dad230"
            )
        );
        FHE.allowThis(yBytes256);
        FHE.allow(yBytes256, msg.sender);
    }
}
