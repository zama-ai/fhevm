// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
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

/**
 * @notice A struct that contains a ciphertext handle and a contract address that is
 * @notice expected to be allowed to decrypt this ciphertext
 */
struct CtHandleContractPair {
    /// @notice The handle of the ciphertext
    uint256 ctHandle;
    /// @notice The address of the contract
    address contractAddress;
}
