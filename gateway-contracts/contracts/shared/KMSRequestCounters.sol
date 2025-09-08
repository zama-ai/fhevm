// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// Define the KMS request types
// The old decryption request (a mix of public and user decryption requests) is deprecated. Is it kept
// in order to stay backward compatible and avoid overlaps between the old and new decryption request IDs.
// See `Decryption.sol` for more details.
enum RequestType {
    _deprecated_, // 0: DEPRECATED (old decryption requests)
    PublicDecrypt, // 1
    UserDecrypt, // 2
    PrepKeygen, // 3
    Keygen, // 4
    Crsgen // 5
}

// Bit position to left shift for initializing the counters
uint256 constant REQUEST_TYPE_SHIFT = 248;

// Define the counters' initial values in order to generate globally unique requestIds per request type
// for the KMS

// Public decrypt requestId format in bytes: [0000 0001 | counter_1..31]
uint256 constant PUBLIC_DECRYPT_COUNTER_BASE = uint256(RequestType.PublicDecrypt) << REQUEST_TYPE_SHIFT;

// User decrypt requestId format in bytes: [0000 0010 | counter_1..31]
uint256 constant USER_DECRYPT_COUNTER_BASE = uint256(RequestType.UserDecrypt) << REQUEST_TYPE_SHIFT;

// Preprocessing keygen requestId format in bytes: [0000 0011 | counter_1..31]
uint256 constant PREP_KEYGEN_COUNTER_BASE = uint256(RequestType.PrepKeygen) << REQUEST_TYPE_SHIFT;

// Keygen requestId format in bytes: [0000 0100 | counter_1..31]
uint256 constant KEY_COUNTER_BASE = uint256(RequestType.Keygen) << REQUEST_TYPE_SHIFT;

// CRS generation requestId format in bytes: [0000 0101 | counter_1..31]
uint256 constant CRS_COUNTER_BASE = uint256(RequestType.Crsgen) << REQUEST_TYPE_SHIFT;
