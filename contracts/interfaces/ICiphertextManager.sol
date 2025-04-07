// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../shared/Structs.sol";

/**
 * @title ICiphertextManager
 * @notice Interface of the CiphertextManager contract for handling ciphertext digests and the
 * URLs of the storage they are placed.
 */
interface ICiphertextManager {
    /// @notice Emitted when a consensus for the ciphertext material addition is reached.
    event AddCiphertextMaterial(
        bytes32 indexed ctHandle,
        bytes32 ciphertextDigest,
        bytes32 snsCiphertextDigest,
        address[] coprocessorTxSenderAddresses
    );

    /// @notice Error indicating that the given keyId is outdated.
    error InvalidCurrentKeyId(uint256 keyId);

    /// @notice Error indicating that the given coprocessor transaction sender has already added
    /// @notice the handle (for any chainId).
    error CoprocessorTxSenderAlreadyAdded(address coprocessorTxSenderAddress);

    /// @notice Error indicating that the given ciphertext material represented by the given handle has not
    /// @notice been added in the contract.
    error CiphertextMaterialNotFound(bytes32 ctHandle);

    /// @notice Error indicating that the given ciphertext material represented by the given handle is not
    /// @notice associated with the given chain ID.
    error CiphertextMaterialNotOnNetwork(bytes32 ctHandle, uint256 chainId);

    /// @notice Checks if the ciphertext material represented by the given handle has been added in the contract.
    /// @param ctHandle The handle of the ciphertext material.
    function checkCiphertextMaterial(bytes32 ctHandle) external view;

    /// @notice Retrieves the list of "normal" ciphertext materials for the given handles.
    /// @param ctHandles The list of handles of the ciphertexts to retrieve.
    /// @return ctMaterials The list of regular ciphertext digests, its handles and its key IDs.
    function getCiphertextMaterials(
        bytes32[] calldata ctHandles
    ) external view returns (CiphertextMaterial[] memory ctMaterials);

    /// @notice Retrieves the list of SNS ciphertext materials for the given handles.
    /// @param ctHandles The list of handles of the ciphertexts to retrieve.
    /// @return snsCtMaterials The list of SNS ciphertext digests, its handles and its key IDs.
    function getSnsCiphertextMaterials(
        bytes32[] calldata ctHandles
    ) external view returns (SnsCiphertextMaterial[] memory snsCtMaterials);

    /// @notice Adds a new ciphertext digest to the state. Also include its SNS (Switch and Squash)
    /// @notice version and other metadata.
    /// @param ctHandle The handle of the ciphertext.
    /// @param keyId The ID of the key under the ciphertext has been generated.
    /// @param ciphertextDigest The digest of the "normal" ciphertext.
    /// @param snsCiphertextDigest The digest of the SNS ciphertext.
    function addCiphertextMaterial(
        bytes32 ctHandle,
        uint256 keyId,
        bytes32 ciphertextDigest,
        bytes32 snsCiphertextDigest
    ) external;
}
