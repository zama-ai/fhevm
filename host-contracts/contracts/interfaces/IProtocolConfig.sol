// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {KmsNode} from "../shared/Structs.sol";

/**
 * @title Interface for the ProtocolConfig contract.
 * @notice ProtocolConfig manages the KMS node set, threshold configuration, and context lifecycle
 * on the Ethereum host chain. It replaces the context-management duties previously held by KMSVerifier.
 */
interface IProtocolConfig {
    /**
     * @notice Thresholds used for KMS consensus.
     * @param publicDecryption Minimum signatures required for public decryption verification.
     * @param userDecryption Minimum signatures required for user decryption verification.
     * @param kmsGen Minimum signatures required for key/CRS generation consensus.
     * @param mpc Minimum signatures required for MPC computation quorums.
     */
    struct KmsThresholds {
        uint256 publicDecryption;
        uint256 userDecryption;
        uint256 kmsGen;
        uint256 mpc;
    }

    // -----------------------------------------------------------------------------------------
    // Events
    // -----------------------------------------------------------------------------------------

    /**
     * @notice Emitted when a new KMS context is created.
     * @param kmsContextId The new context ID.
     * @param kmsNodes The KMS nodes registered in the context.
     * @param thresholds The thresholds for the context.
     */
    event NewKmsContext(uint256 indexed kmsContextId, KmsNode[] kmsNodes, KmsThresholds thresholds);

    /**
     * @notice Emitted when a KMS context is destroyed.
     * @param kmsContextId The destroyed context ID.
     */
    event KmsContextDestroyed(uint256 indexed kmsContextId);

    // -----------------------------------------------------------------------------------------
    // Errors
    // -----------------------------------------------------------------------------------------

    /// @notice The KMS nodes array is empty.
    error EmptyKmsNodes();

    /// @notice A KMS node has a null tx sender address.
    error KmsNodeNullTxSender();

    /// @notice A KMS node has a null signer address.
    error KmsNodeNullSigner();

    /// @notice A KMS tx sender address is already registered in this context.
    /// @param txSender The duplicate tx sender address.
    error KmsTxSenderAlreadyRegistered(address txSender);

    /// @notice A KMS signer address is already registered in this context.
    /// @param signer The duplicate signer address.
    error KmsSignerAlreadyRegistered(address signer);

    /// @notice A threshold is zero.
    /// @param thresholdName The name of the invalid threshold.
    error InvalidNullThreshold(string thresholdName);

    /// @notice A threshold exceeds the node count.
    /// @param thresholdName The name of the invalid threshold.
    /// @param threshold The invalid threshold value.
    /// @param nodeCount The number of nodes.
    error InvalidHighThreshold(string thresholdName, uint256 threshold, uint256 nodeCount);

    /// @notice The context ID does not exist or has been destroyed.
    /// @param kmsContextId The invalid context ID.
    error InvalidKmsContext(uint256 kmsContextId);

    /// @notice Cannot destroy the current active context.
    /// @param kmsContextId The current context ID.
    error CurrentKmsContextCannotBeDestroyed(uint256 kmsContextId);

    /// @notice A key management request is currently in flight on KMSGeneration.
    error KeyManagementRequestInFlight();

    // -----------------------------------------------------------------------------------------
    // State-changing functions
    // -----------------------------------------------------------------------------------------

    /**
     * @notice Create a new KMS context with the given nodes and thresholds.
     * @param kmsNodes The KMS nodes to register.
     * @param thresholds The thresholds for the new context.
     */
    function defineNewKmsContext(KmsNode[] calldata kmsNodes, KmsThresholds calldata thresholds) external;

    /**
     * @notice Destroy a KMS context, preventing it from being used.
     * @param kmsContextId The context ID to destroy.
     */
    function destroyKmsContext(uint256 kmsContextId) external;

    /**
     * @notice Returns the current active KMS context ID.
     * @return The current context ID.
     */
    function getCurrentKmsContextId() external view returns (uint256);

    /**
     * @notice Checks whether a KMS context ID is valid (exists and is not destroyed).
     * @param kmsContextId The context ID to check.
     * @return True if the context is valid.
     */
    function isValidKmsContext(uint256 kmsContextId) external view returns (bool);

    /**
     * @notice Returns the signer addresses for the current active context.
     * @return The list of signer addresses.
     */
    function getKmsSigners() external view returns (address[] memory);

    /**
     * @notice Returns the signer addresses for a given context.
     * @param kmsContextId The context ID.
     * @return The list of signer addresses.
     */
    function getKmsSignersForContext(uint256 kmsContextId) external view returns (address[] memory);

    /**
     * @notice Checks whether an address is a signer in the current active context.
     * @param signer The address to check.
     * @return True if the address is a signer in the current context.
     */
    function isKmsSigner(address signer) external view returns (bool);

    /**
     * @notice Checks whether an address is a signer in the given context.
     * @param kmsContextId The context ID.
     * @param signer The address to check.
     * @return True if the address is a signer.
     */
    function isKmsSignerForContext(uint256 kmsContextId, address signer) external view returns (bool);

    /**
     * @notice Returns the KMS nodes for a given context.
     * @param kmsContextId The context ID.
     * @return The list of KMS nodes.
     */
    function getKmsNodesForContext(uint256 kmsContextId) external view returns (KmsNode[] memory);

    /**
     * @notice Checks whether an address is a tx sender in the given context.
     * @param kmsContextId The context ID.
     * @param txSender The address to check.
     * @return True if the address is a KMS tx sender.
     */
    function isKmsTxSenderForContext(uint256 kmsContextId, address txSender) external view returns (bool);

    /**
     * @notice Returns the KmsNode metadata for a tx sender in the given context.
     * @param kmsContextId The context ID.
     * @param txSender The tx sender address.
     * @return The KmsNode struct.
     */
    function getKmsNodeForContext(uint256 kmsContextId, address txSender) external view returns (KmsNode memory);

    /**
     * @notice Returns the current public decryption threshold (for the active context).
     * @return The public decryption threshold.
     */
    function getPublicDecryptionThreshold() external view returns (uint256);

    /**
     * @notice Returns the public decryption threshold for a given context.
     * @param kmsContextId The context ID.
     * @return The public decryption threshold for the context.
     */
    function getPublicDecryptionThresholdForContext(uint256 kmsContextId) external view returns (uint256);

    /**
     * @notice Returns the current user decryption threshold (for the active context).
     * @return The user decryption threshold.
     */
    function getUserDecryptionThreshold() external view returns (uint256);

    /**
     * @notice Returns the user decryption threshold for a given context.
     * @param kmsContextId The context ID.
     * @return The user decryption threshold for the context.
     */
    function getUserDecryptionThresholdForContext(uint256 kmsContextId) external view returns (uint256);

    /**
     * @notice Returns the current kmsGen threshold (for the active context).
     * @return The kmsGen threshold.
     */
    function getKmsGenThreshold() external view returns (uint256);

    /**
     * @notice Returns the current MPC threshold (for the active context).
     * @return The MPC threshold.
     */
    function getMpcThreshold() external view returns (uint256);

    /**
     * @notice Returns the contract version.
     * @return The version string.
     */
    function getVersion() external pure returns (string memory);
}
