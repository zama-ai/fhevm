// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

uint8 constant HANDLE_VERSION = 0;

/**
 * @notice The KMS request and protocol-lifecycle ID tags used in the high byte of KMS IDs.
 * @dev The ordering preserves Gateway request-type history so migrated IDs keep their meaning.
 */
enum RequestType {
    _deprecated_, // 0: DEPRECATED
    _gatewayPublicDecrypt_, // 1: reserved on Gateway
    _gatewayUserDecrypt_, // 2: reserved on Gateway
    PrepKeygen, // 3
    Keygen, // 4
    Crsgen, // 5
    _deprecatedKeyReshare_, // 6: was KeyReshare on Gateway
    KmsContext, // 7
    Epoch // 8
}

// Bit position to left shift for initializing KMS ID counters.
uint256 constant REQUEST_TYPE_SHIFT = 248;

/// @dev Base value for preprocessing keygen request IDs. Format: [0x03 type tag | 31 counter bytes].
uint256 constant PREP_KEYGEN_COUNTER_BASE = uint256(RequestType.PrepKeygen) << REQUEST_TYPE_SHIFT;

/// @dev Base value for keygen request IDs. Format: [0x04 type tag | 31 counter bytes].
uint256 constant KEY_COUNTER_BASE = uint256(RequestType.Keygen) << REQUEST_TYPE_SHIFT;

/// @dev Base value for CRS generation request IDs. Format: [0x05 type tag | 31 counter bytes].
uint256 constant CRS_COUNTER_BASE = uint256(RequestType.Crsgen) << REQUEST_TYPE_SHIFT;

/// @dev Base value for KMS context IDs. Format: [0x07 type tag | 31 counter bytes].
uint256 constant KMS_CONTEXT_COUNTER_BASE = uint256(RequestType.KmsContext) << REQUEST_TYPE_SHIFT;

/// @dev Base value for epoch IDs. Format: [0x08 type tag | 31 counter bytes].
uint256 constant EPOCH_COUNTER_BASE = uint256(RequestType.Epoch) << REQUEST_TYPE_SHIFT;

/// @dev Version byte for v1 extraData layout: [version(1)] [contextId(32)].
uint8 constant EXTRA_DATA_V1 = 0x01;

/// @dev Version byte for v2 extraData layout: [version(1)] [contextId(32)] [epochId(32)].
uint8 constant EXTRA_DATA_V2 = 0x02;

/// @dev HCU quota of every contract that was never assigned.
uint16 constant DEFAULT_QUOTA = 0;

/// @dev HCU quota of contracts the allowlist admitted and nobody assigned yet, while the default quota is stopped.
uint16 constant ALLOWLIST_QUOTA = 1;

/// @dev HCU quota of contracts both allowlisted and block-whitelisted that nobody assigned yet, while the default
///      quota is stopped. Quotas up to this one take no admin.
uint16 constant BLOCK_WHITELIST_QUOTA = 2;

/// @dev 100% in basis points: a tier threshold that admits operations until the capacity is full, and the capacity
///      scale at which the capacity is the owner's.
uint16 constant MAX_BPS = 10_000;

/// @dev An HCU quota or capacity with this maximum is not metered.
uint48 constant NO_CAP = type(uint48).max;

/// @dev HCU tier of every quota never moved to another tier. Its threshold is fixed at MAX_BPS.
uint8 constant RESERVED_TIER = 0;

/// @dev Highest HCU tier. An operation charges the meter of its tier and of every tier above it, so this bounds the
///      meters each operation reads and writes.
uint8 constant MAX_TIER = 3;

/// @dev Returns `value` at the HCU capacity scale `scaleBps`, rounded up so that a value above 0 never scales to 0.
///      HCULimit's meters and HCUConfigurator's bounds on the scale both use it.
function hcuScaled(uint256 value, uint16 scaleBps) pure returns (uint256) {
    return (value * scaleBps + MAX_BPS - 1) / MAX_BPS;
}
