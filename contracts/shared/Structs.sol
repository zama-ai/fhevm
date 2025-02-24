// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;
/**
 * @notice Data structure used to transfer a SNS ciphertext and some of its metadata between
 * the Gateway L2 contracts.
 */
struct SnsCiphertextMaterial {
    uint256 ctHandle;
    uint256 keyId;
    bytes snsCiphertext;
}

/**
 * @notice Data structure used to transfer a regular ciphertext and some of its metadata between
 * the Gateway L2 contracts.
 */
struct CiphertextMaterial {
    uint256 ctHandle;
    uint256 keyId;
    bytes ciphertext;
}
