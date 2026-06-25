// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

uint8 constant HANDLE_VERSION = 0;

// ----------------------------------------------------------------------------------------------
// KMS identifier type-tag family
//
// Every KMS-related identifier embeds a one-byte type tag in its most-significant byte, followed by
// a 31-byte counter: [type tag | counter_1..31]. The tags share a single namespace and must stay
// mutually distinct so identifiers from different families can never collide. The preprocessing-keygen
// (0x03), keygen (0x04) and CRS-generation (0x05) tags are applied in `KMSGeneration.sol`; the KMS
// context tag (0x07) is below. Keep this list and `KMSGeneration.sol` in sync as the single source.
// ----------------------------------------------------------------------------------------------

/// @dev Bit shift placing a one-byte type tag in the most-significant byte of a 32-byte identifier.
uint256 constant REQUEST_TYPE_SHIFT = 248;

/// @dev Type tag for KMS context IDs (must stay distinct from the KMSGeneration tags 0x03/0x04/0x05).
uint8 constant KMS_CONTEXT_REQUEST_TYPE = 0x07;

/// @dev Base value for KMS context IDs. Format: [0x07 type tag | 31 counter bytes].
uint256 constant KMS_CONTEXT_COUNTER_BASE = uint256(KMS_CONTEXT_REQUEST_TYPE) << REQUEST_TYPE_SHIFT;
