// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {KmsNode, ChainUpgradeWindow} from "../shared/Structs.sol";

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

    /**
     * @notice Emitted when a new coprocessor context is defined.
     * @param coprocessorContextId The new coprocessor context ID.
     * @param softwareVersion The coprocessor software version for the new context.
     * @param chainUpgradeWindows The per-host-chain replay windows for the upgrade.
     * @param gwStartBlock The Gateway block at which GCS's gateway-listener resumes from.
     * @param ciphertextVersion The ciphertext version the new software writes; promoted into the `versioning` singleton at cutover.
     */
    event NewCoprocessorContext(
        uint256 indexed coprocessorContextId,
        string softwareVersion,
        ChainUpgradeWindow[] chainUpgradeWindows,
        uint64 gwStartBlock,
        uint16 ciphertextVersion
    );

    /**
     * @notice Emitted when a coprocessor context is destroyed.
     * @param coprocessorContextId The destroyed coprocessor context ID.
     */
    event CoprocessorContextDestroyed(uint256 indexed coprocessorContextId);

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

    /// @notice A threshold exceeds the proof format limit (`uint8` signature count in the
    ///         `decryptionProof` payload consumed by `KMSVerifier`).
    /// @param thresholdName The name of the invalid threshold.
    /// @param threshold The invalid threshold value.
    /// @param maxAllowed The maximum value the proof format can carry.
    error ThresholdExceedsProofFormatLimit(string thresholdName, uint256 threshold, uint256 maxAllowed);

    /// @notice The KMS signer set exceeds the proof format limit (`uint8` signature count in the
    ///         `decryptionProof` payload consumed by `KMSVerifier`).
    /// @param signerCount The number of signers in the rejected set.
    /// @param maxAllowed The maximum size the proof format can carry.
    error KmsSignerSetExceedsProofFormatLimit(uint256 signerCount, uint256 maxAllowed);

    /// @notice The context ID does not exist or has been destroyed.
    /// @param kmsContextId The invalid context ID.
    error InvalidKmsContext(uint256 kmsContextId);

    /// @notice Cannot destroy the current active context.
    /// @param kmsContextId The current context ID.
    error CurrentKmsContextCannotBeDestroyed(uint256 kmsContextId);

    /// @notice The coprocessor `softwareVersion` argument is the empty string.
    error EmptySoftwareVersion();

    /// @notice The `chainUpgradeWindows` array argument is empty.
    error EmptyChainUpgradeWindows();

    /// @notice A chain entry has a zero `chainId`.
    error ZeroChainId();

    /// @notice The same `chainId` appears more than once in the `chainUpgradeWindows` array.
    /// @param chainId The duplicated chain id.
    error DuplicateChainId(uint64 chainId);

    /// @notice The block window for a chain entry is invalid (`startBlock > endBlock`).
    /// @param chainId The chain id whose window is invalid.
    /// @param startBlock The provided start block.
    /// @param endBlock The provided end block.
    error InvalidBlockWindow(uint64 chainId, uint64 startBlock, uint64 endBlock);

    /// @notice The `gwStartBlock` argument is zero.
    error ZeroGwStartBlock();

    /// @notice `ciphertextVersion` exceeds the off-chain `int16` storage range.
    /// @param ciphertextVersion The rejected value.
    error CiphertextVersionTooLarge(uint16 ciphertextVersion);

    /// @notice The coprocessor context ID does not exist or has been destroyed.
    /// @param coprocessorContextId The invalid coprocessor context ID.
    error InvalidCoprocessorContext(uint256 coprocessorContextId);

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
     * @notice Create a new coprocessor context with the given software version, replay windows, and Gateway start block.
     * @param softwareVersion The coprocessor software version.
     * @param chainUpgradeWindows The per-host-chain replay windows.
     * @param gwStartBlock The Gateway block to resume from.
     * @param ciphertextVersion The ciphertext version the new software writes.
     */
    function defineNewCoprocessorContext(
        string calldata softwareVersion,
        ChainUpgradeWindow[] calldata chainUpgradeWindows,
        uint64 gwStartBlock,
        uint16 ciphertextVersion
    ) external;

    /**
     * @notice Destroy a coprocessor context, preventing it from being used.
     * @param coprocessorContextId The context ID to destroy.
     */
    function destroyCoprocessorContext(uint256 coprocessorContextId) external;

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
     * @notice Returns the kmsGen threshold for a given context.
     * @param kmsContextId The context ID.
     * @return The kmsGen threshold for the context.
     */
    function getKmsGenThresholdForContext(uint256 kmsContextId) external view returns (uint256);

    /**
     * @notice Returns the current MPC threshold (for the active context).
     * @return The MPC threshold.
     */
    function getMpcThreshold() external view returns (uint256);

    /**
     * @notice Returns the MPC threshold for a given context.
     * @param kmsContextId The context ID.
     * @return The MPC threshold for the context.
     */
    function getMpcThresholdForContext(uint256 kmsContextId) external view returns (uint256);

    /**
     * @notice Returns the current coprocessor context ID.
     * @return The current context ID.
     */
    function getCurrentCoprocessorContextId() external view returns (uint256);

    /**
     * @notice Checks whether a coprocessor context ID is valid (exists and is not destroyed).
     * @param coprocessorContextId The context ID to check.
     * @return True if the context is valid.
     */
    function isValidCoprocessorContext(uint256 coprocessorContextId) external view returns (bool);

    /**
     * @notice Returns the contract version.
     * @return The version string.
     */
    function getVersion() external pure returns (string memory);
}
