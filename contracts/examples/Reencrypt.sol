// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/TFHE.sol";
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
        TFHE.setFHEVM(FHEVMConfig.defaultConfig());

        // Initialize and set permissions for xBool
        xBool = TFHE.asEbool(true);
        TFHE.allowThis(xBool);
        TFHE.allow(xBool, msg.sender);

        // Initialize and set permissions for xUint8
        xUint8 = TFHE.asEuint8(42);
        TFHE.allowThis(xUint8);
        TFHE.allow(xUint8, msg.sender);

        // Initialize and set permissions for xUint16
        xUint16 = TFHE.asEuint16(16);
        TFHE.allowThis(xUint16);
        TFHE.allow(xUint16, msg.sender);

        // Initialize and set permissions for xUint32
        xUint32 = TFHE.asEuint32(32);
        TFHE.allowThis(xUint32);
        TFHE.allow(xUint32, msg.sender);

        // Initialize and set permissions for xUint64
        xUint64 = TFHE.asEuint64(18446744073709551600);
        TFHE.allowThis(xUint64);
        TFHE.allow(xUint64, msg.sender);

        xUint128 = TFHE.asEuint128(145275933516363203950142179850024740765);
        TFHE.allowThis(xUint128);
        TFHE.allow(xUint128, msg.sender);

        xAddress = TFHE.asEaddress(0x8ba1f109551bD432803012645Ac136ddd64DBA72);
        TFHE.allowThis(xAddress);
        TFHE.allow(xAddress, msg.sender);

        xUint256 = TFHE.asEuint256(74285495974541385002137713624115238327312291047062397922780925695323480915729);
        TFHE.allowThis(xUint256);
        TFHE.allow(xUint256, msg.sender);

        yBytes64 = TFHE.asEbytes64(
            TFHE.padToBytes64(
                hex"19d179e0cc7e816dc944582ed4f5652f5951900098fc2e0a15a7ea4dc8cfa4e3b6c54beea5ee95e56b728762f659347ce1d4aa1b05fcc5"
            )
        );
        TFHE.allowThis(yBytes64);
        TFHE.allow(yBytes64, msg.sender);

        yBytes128 = TFHE.asEbytes128(
            TFHE.padToBytes128(
                hex"13e7819123de6e2870c7e83bb764508e22d7c3ab8a5aee6bdfb26355ef0d3f1977d651b83bf5f78634fa360aa14debdc3daa6a587b5c2fb1710ab4d6677e62a8577f2d9fecc190ad8b11c9f0a5ec3138b27da1f055437af8c90a9495dad230"
            )
        );
        TFHE.allowThis(yBytes128);
        TFHE.allow(yBytes128, msg.sender);

        yBytes256 = TFHE.asEbytes256(
            TFHE.padToBytes256(
                hex"d179e0cc7e816dc944582ed4f5652f5951900098fc2e0a15a7ea4dc8cfa4e3b6c54beea5ee95e56b728762f659347ce1d4aa1b05fcc513e7819123de6e2870c7e83bb764508e22d7c3ab8a5aee6bdfb26355ef0d3f1977d651b83bf5f78634fa360aa14debdc3daa6a587b5c2fb1710ab4d6677e62a8577f2d9fecc190ad8b11c9f0a5ec3138b27da1f055437af8c90a9495dad230"
            )
        );
        TFHE.allowThis(yBytes256);
        TFHE.allow(yBytes256, msg.sender);
    }
}
