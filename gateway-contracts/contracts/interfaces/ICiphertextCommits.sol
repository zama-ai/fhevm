// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../shared/Structs.sol";
import { ContextStatus } from "../shared/Enums.sol";

/**
 * @title Interface for the CiphertextCommits contract.
 * @notice The CiphertextCommits contract stores ciphertext commitments for all host chains.
 */
interface ICiphertextCommits {
    /**
     * @notice Emitted when a consensus for the ciphertext material addition is reached.
     * @param ctHandle The handle of the added ciphertext material.
     * @param ciphertextDigest The digest of the regular ciphertext.
     * @param snsCiphertextDigest The digest of the SNS ciphertext.
     * @param coprocessorContextId The context ID of the coprocessor that was part of the consensus.
     */
    event AddCiphertextMaterial(
        bytes32 indexed ctHandle,
        bytes32 ciphertextDigest,
        bytes32 snsCiphertextDigest,
        uint256 coprocessorContextId
    );

    /**
     * @notice Error indicating that the coprocessor context is no longer valid for adding the ciphertext material.
     * A context is valid if it is active or suspended.
     * @param ctHandle The handle of the ciphertext.
     * @param contextId The context ID of the coprocessor.
     * @param contextStatus The status of the coprocessor context.
     */
    error InvalidCoprocessorContextAddCiphertext(bytes32 ctHandle, uint256 contextId, ContextStatus contextStatus);

    /**
     * @notice Error indicating that the given coprocessor transaction sender has already added the handle.
     * @param ctHandle The handle of the already added ciphertext.
     * @param txSender The transaction sender address of the coprocessor that has already added the handle.
     */
    error CoprocessorAlreadyAdded(bytes32 ctHandle, address txSender);

    /**
     * @notice Error indicating that the given ciphertext material represented by the given handle has not
     * been added in the contract.
     * @param ctHandle The handle of the not found ciphertext.
     */
    error CiphertextMaterialNotFound(bytes32 ctHandle);

    /**
     * @notice Adds a new ciphertext digest to the state. Also include its Switch and Squash (SNS)
     * version and other metadata.
     * @param ctHandle The handle of the ciphertext.
     * @param keyId The ID of the key under the ciphertext has been generated.
     * @param ciphertextDigest The digest of the regular ciphertext.
     * @param snsCiphertextDigest The digest of the SNS ciphertext.
     */
    function addCiphertextMaterial(
        bytes32 ctHandle,
        uint256 keyId,
        bytes32 ciphertextDigest,
        bytes32 snsCiphertextDigest
    ) external;

    /**
     * @notice Retrieves the list of regular ciphertext materials associated to the handles.
     * @param ctHandles The list of handles to retrieve.
     * @return The list of regular ciphertext digests, its handles and its key IDs.
     */
    function getCiphertextMaterials(bytes32[] calldata ctHandles) external view returns (CiphertextMaterial[] memory);

    /**
     * @notice Retrieves the list of Switch and Squash (SNS) ciphertext materials associated to the handles.
     * @param ctHandles The list of handles to retrieve.
     * @return The list of SNS ciphertext digests, its handles and its key IDs.
     */
    function getSnsCiphertextMaterials(
        bytes32[] calldata ctHandles
    ) external view returns (SnsCiphertextMaterial[] memory);

    /**
     * @notice Retrieves the list of the storage URLs associated to the handles for which the consensus was reached.
     * @param ctHandles The list of handles to retrieve.
     * @return The list of storage URLs.
     */
    function getConsensusStorageUrls(bytes32[] calldata ctHandles) external view returns (string[][] memory);

    /**
     * @notice Indicates if the ciphertext material represented by the handle has been added in the contract.
     * @param ctHandle The handle to check.
     */
    function isCiphertextMaterialAdded(bytes32 ctHandle) external view returns (bool);

    /**
     * @notice Returns the coprocessor transaction sender addresses that were involved in the consensus
     * for an add ciphertext material.
     * @param ctHandle The handle to retrieve the coprocessor transaction sender addresses for.
     * @return The list of coprocessor transaction sender addresses.
     */
    function getConsensusTxSenders(bytes32 ctHandle) external view returns (address[] memory);

    /**
     * @notice Returns the versions of the CiphertextCommits contract in SemVer format.
     * @dev This is conventionally used for upgrade features.
     */
    function getVersion() external pure returns (string memory);
}
