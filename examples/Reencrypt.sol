// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/TFHE.sol";

// Contract for demonstrating reencryption of various FHE data types
contract Reencrypt {
    // Encrypted boolean
    ebool public xBool;
    // Encrypted 4-bit unsigned integer
    euint4 public xUint4;
    // Encrypted 8-bit unsigned integer
    euint8 public xUint8;
    // Encrypted 16-bit unsigned integer
    euint16 public xUint16;
    // Encrypted 32-bit unsigned integer
    euint32 public xUint32;
    // Encrypted 64-bit unsigned integer
    euint64 public xUint64;
    // Encrypted Ethereum address
    eaddress public xAddress;
    // Encrypted 256-bit bytes
    ebytes256 public yBytes256;

    // Constructor to initialize encrypted values and set permissions
    constructor() {
        // Set default FHE configuration
        TFHE.setFHEVM(FHEVMConfig.defaultConfig());

        // Initialize and set permissions for xBool
        xBool = TFHE.asEbool(true);
        TFHE.allowThis(xBool);
        TFHE.allow(xBool, msg.sender);

        // Initialize and set permissions for xUint4
        xUint4 = TFHE.asEuint4(4);
        TFHE.allowThis(xUint4);
        TFHE.allow(xUint4, msg.sender);

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

        // Initialize and set permissions for xAddress
        xAddress = TFHE.asEaddress(0x8ba1f109551bD432803012645Ac136ddd64DBA72);
        TFHE.allowThis(xAddress);
        TFHE.allow(xAddress, msg.sender);
    }

    // Function to set and allow access to encrypted 256-bit bytes
    function setEBytes256(einput inputHandleEBytes256, bytes memory inputProofEBytes256) external {
        yBytes256 = TFHE.asEbytes256(inputHandleEBytes256, inputProofEBytes256);
        TFHE.allowThis(yBytes256);
        TFHE.allow(yBytes256, msg.sender);
    }
}
