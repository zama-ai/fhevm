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
        uint256 indexed ctHandle,
        bytes32 ciphertextDigest,
        bytes32 snsCiphertextDigest,
        address[] coprocessors
    );

    /// @notice Error indicating that the given keyId is outdated.
    error InvalidCurrentKeyId(uint256 keyId);

    /// @notice Error indicating that the given coprocessor has already added the handle (for any chainId).
    error CoprocessorAlreadyAdded(address coprocessor);

    /// @notice Error indicating that the given coprocessor has not added the ctHandle and chainId.
    error CoprocessorHasNotAdded(uint256 ctHandle, uint256 chainId, address coprocessor);

    /// @notice Error indicating that the given ciphertext material represented by the given handle has not
    /// @notice been added in the contract.
    error CiphertextMaterialNotFound(uint256 ctHandle);

    /// @notice Error indicating that the given ciphertext material represented by the given handle is not
    /// @notice associated with the given chain ID.
    error CiphertextMaterialNotOnNetwork(uint256 ctHandle, uint256 chainId);

    /// @notice Returns true if the ciphertext material has reached consensus and added in the contract.
    /// @param ctHandle The handle of the ciphertext material.
    function ciphertextMaterialExists(uint256 ctHandle) external view returns (bool);

    /// @notice Checks if the ciphertext material represented by the given handle has been added in the contract.
    /// @param ctHandle The handle of the ciphertext material.
    function checkCiphertextMaterial(uint256 ctHandle) external view;

    /// @notice Checks if the given coprocessor has already added the ciphertext material.
    /// @param ctHandle The handle of the ciphertext material.
    /// @param chainId The chain ID of the blockchain associated to the ciphertext handle.
    /// @param coprocessorAddress The address of the coprocessor.
    function checkCoprocessorTxSenderHasAdded(
        uint256 ctHandle,
        uint256 chainId,
        address coprocessorAddress
    ) external view;

    /// @notice Retrieves the list of "normal" ciphertext materials for the given handles.
    /// @param ctHandles The list of handles of the ciphertexts to retrieve.
    /// @return ctMaterials The list of regular ciphertext digests, its handles and its key IDs.
    function getCiphertextMaterials(
        uint256[] calldata ctHandles
    ) external view returns (CiphertextMaterial[] memory ctMaterials);

    /// @notice Retrieves the list of SNS ciphertext materials for the given handles.
    /// @param ctHandles The list of handles of the ciphertexts to retrieve.
    /// @return snsCtMaterials The list of SNS ciphertext digests, its handles and its key IDs.
    function getSnsCiphertextMaterials(
        uint256[] calldata ctHandles
    ) external view returns (SnsCiphertextMaterial[] memory snsCtMaterials);

    /// @notice Adds a new ciphertext digest to the state. Also include its SNS (Switch and Squash)
    /// @notice version and other metadata.
    /// @param ctHandle The handle of the ciphertext.
    /// @param keyId The ID of the key under the ciphertext has been generated.
    /// @param chainId The chain ID of the blockchain associated to the ciphertext handle.
    /// @param ciphertextDigest The digest of the "normal" ciphertext.
    /// @param snsCiphertextDigest The digest of the SNS ciphertext.
    function addCiphertextMaterial(
        uint256 ctHandle,
        uint256 keyId,
        uint256 chainId,
        bytes32 ciphertextDigest,
        bytes32 snsCiphertextDigest
    ) external;
}
