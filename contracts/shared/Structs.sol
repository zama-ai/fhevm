// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;
/**
 * @notice Data structure used to transfer a ciphertext metadata between the Gateway L2
 * components such as CiphertextStorage, ACLManager and DecryptionManager.
 */
struct CiphertextMaterial {
    uint256 ctHandle;
    uint256 keyId;
    bytes ciphertext128;
}
