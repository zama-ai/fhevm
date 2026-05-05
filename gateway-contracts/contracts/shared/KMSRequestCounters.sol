// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @notice The KMS request types.
 * @dev The first request type is deprecated.
 * See `Decryption.sol` for more details.
 * Indexes 3–6 were previously used by KMSGeneration (PrepKeygen, Keygen, Crsgen,
 * KeyReshare) before it was moved to Ethereum. They are intentionally not reused
 * here: historical requestIds with those prefixes are still served by the
 * view-only KMSGeneration getters on existing Gateway deployments.
 */
enum RequestType {
    _deprecated_, // 0: DEPRECATED
    PublicDecrypt, // 1
    UserDecrypt, // 2
    _deprecated_3_, // 3: was PrepKeygen
    _deprecated_4_, // 4: was Keygen
    _deprecated_5_, // 5: was Crsgen
    _deprecated_6_ // 6: was KeyReshare
}

// Bit position to left shift for initializing the counters
uint256 constant REQUEST_TYPE_SHIFT = 248;

// Define the counters' initial values in order to generate globally unique requestIds per request type
// for the KMS

// Public decrypt requestId format in bytes: [0000 0001 | counter_1..31]
uint256 constant PUBLIC_DECRYPT_COUNTER_BASE = uint256(RequestType.PublicDecrypt) << REQUEST_TYPE_SHIFT;

// User decrypt requestId format in bytes: [0000 0010 | counter_1..31]
uint256 constant USER_DECRYPT_COUNTER_BASE = uint256(RequestType.UserDecrypt) << REQUEST_TYPE_SHIFT;
