// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

uint8 constant HANDLE_VERSION = 0;

/// @dev Base value for KMS context IDs. Format: [0x07 type tag | 31 counter bytes].
/// See KMSRequestCounters on Gateway for the shared counter scheme.
uint256 constant KMS_CONTEXT_COUNTER_BASE = uint256(0x07) << 248;
