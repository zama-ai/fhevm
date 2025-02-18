// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

import "../shared/Structs.sol";

/**
 * @title ICiphertextStorage
 * @notice Interface of the CiphertextStorage contract that stores the ciphertexts allowed/requested for decryption.
 * @dev Normal 64-bit ciphertexts are only used for computations while 128-PBS ciphertexts are used for KMS concerns.
 */
interface ICiphertextStorage {
    /// @notice Emitted when a ciphertext storing consensus is reached.
    event AddCiphertext(uint256 ctHandle);

    /// @notice Error indicating that the given keyId is outdated.
    error InvalidCurrentKeyId(uint256 keyId);
    /// @notice Error indicating that the sender is not a valid Coprocessor
    error InvalidCoprocessorSender(address sender);
    /// @notice Error indicating that the given coprocessor has already authorized the add operation.
    error CoprocessorHasAlreadyAdded(address coprocessor);

    /// @notice Checks if the ciphertext for the given handle is stored in the contract.
    /// @param ctHandle The handle of the ciphertext.
    function hasCiphertext(uint256 ctHandle) external view returns (bool);

    /// @notice Checks if the given ciphertext handle is associated with the given chain ID.
    /// @param ctHandle The handle of the ciphertext.
    /// @param chainId The chain ID to check if the ciphertext is associated with.
    function isOnNetwork(uint256 ctHandle, uint256 chainId) external returns (bool);

    /// @notice Retrieves the list of 128-PBS ciphertexts for the given handles.
    /// @param ctHandles The list of handles of the ciphertexts to retrieve.
    /// @return ctMaterials The list of 128-PBS ciphertexts, its handles and its key IDs.
    function getCiphertexts(
        uint256[] calldata ctHandles
    ) external view returns (CiphertextMaterial[] memory ctMaterials);

    /// @notice Adds a new ciphertext to the storage.
    /// @param ctHandle The handle of the storing ciphertext.
    /// @param keyId The ID of the key under the ciphertext has been generated.
    /// @param chainId The chain ID of the blockchain associated to the ciphertext handle.
    /// @param ciphertext64 The normal ciphertext (64-bit) to be stored.
    /// @param ciphertext128 The 128-PBS ciphertext (128-bit) to be stored.
    function addCiphertext(
        uint256 ctHandle,
        uint256 keyId,
        uint256 chainId,
        bytes calldata ciphertext64,
        bytes calldata ciphertext128
    ) external;
}
