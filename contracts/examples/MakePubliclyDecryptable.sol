// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/HTTPZ.sol";

import "../lib/HTTPZConfig.sol";

/// @notice Contract for testing makePubliclyDecryptable and isPubliclyDecryptable functions
contract MakePubliclyDecryptable {
    /// @notice Encrypted unsigned integers of various sizes
    ebool public valueb;
    euint8 public value8;
    ebytes256 public value2048;

    /// @notice Constructor to set FHE configuration
    constructor() {
        HTTPZ.setCoprocessor(HTTPZConfig.defaultConfig());
    }

    /// @notice make an ebool publicly decryptable
    function makePubliclyDecryptableBool() public {
        valueb = HTTPZ.asEbool(true);
        HTTPZ.makePubliclyDecryptable(valueb);
    }

    /// @notice check if an ebool is publicly decryptable
    function isPubliclyDecryptableBool() public view returns (bool) {
        return HTTPZ.isPubliclyDecryptable(valueb);
    }

    /// @notice make an euint8 publicly decryptable
    function makePubliclyDecryptableUint8() public {
        value8 = HTTPZ.asEuint8(37);
        HTTPZ.makePubliclyDecryptable(value8);
    }

    /// @notice check if an euint8 is publicly decryptable
    function isPubliclyDecryptableUint8() public view returns (bool) {
        return HTTPZ.isPubliclyDecryptable(value8);
    }

    /// @notice make an ebytes256 publicly decryptable
    function makePubliclyDecryptableBytes256() public {
        value2048 = HTTPZ.asEbytes256(HTTPZ.padToBytes256(hex"d179e0"));
        HTTPZ.makePubliclyDecryptable(value2048);
    }

    /// @notice check if an ebytes256 is publicly decryptable
    function isPubliclyDecryptableBytes256() public view returns (bool) {
        return HTTPZ.isPubliclyDecryptable(value2048);
    }
}
