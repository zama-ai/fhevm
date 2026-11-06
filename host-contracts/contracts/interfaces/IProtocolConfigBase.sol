// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Coprocessor, KmsNode} from "../shared/Structs.sol";

/**
 * @title Interface for the shared ProtocolConfig reads and errors.
 * @notice Declares the read functions and errors that ProtocolConfig and ProtocolConfigReplica share.
 */
interface IProtocolConfigBase {
    // -----------------------------------------------------------------------------------------
    // Events
    // -----------------------------------------------------------------------------------------

    /**
     * @notice Emitted when a coprocessor context is destroyed. Its set and threshold stay readable.
     * @param contextId The destroyed coprocessor context ID.
     */
    event CoprocessorContextDestroyed(uint256 indexed contextId);

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

    /// @notice The KMS signer set exceeds the proof format limit (`uint8` signature count in the
    ///         `decryptionProof` payload consumed by `KMSVerifier`).
    /// @param signerCount The number of signers in the rejected set.
    /// @param maxAllowed The maximum size the proof format can carry.
    error KmsSignerSetExceedsProofFormatLimit(uint256 signerCount, uint256 maxAllowed);

    /// @notice The context ID does not exist or has been destroyed.
    /// @param kmsContextId The invalid context ID.
    error InvalidKmsContext(uint256 kmsContextId);

    /// @notice The epoch ID is invalid or not in the required lifecycle state for this operation.
    /// @param epochId The epoch ID.
    error InvalidKmsEpoch(uint256 epochId);

    /// @notice The mirrored context ID is not strictly greater than the latest activated one.
    /// @param contextId The rejected context ID.
    /// @param latestActiveKmsContextId The most recently activated mirrored context ID.
    error NonIncreasingKmsContextId(uint256 contextId, uint256 latestActiveKmsContextId);

    /// @notice The coprocessors array is empty.
    error EmptyCoprocessors();

    /// @notice A coprocessor has a null tx sender address.
    error CoprocessorNullTxSender();

    /// @notice A coprocessor has a null signer address.
    error CoprocessorNullSigner();

    /// @notice A coprocessor has an empty storage URL.
    error CoprocessorEmptyStorageUrl();

    /// @notice A coprocessor tx sender address is already registered in this context.
    /// @param txSender The duplicate tx sender address.
    error CoprocessorTxSenderAlreadyRegistered(address txSender);

    /// @notice A coprocessor signer address is already registered in this context.
    /// @param signer The duplicate signer address.
    error CoprocessorSignerAlreadyRegistered(address signer);

    /// @notice The coprocessor set is larger than an input proof can carry (`uint8` signature count).
    /// @param coprocessorCount The number of coprocessors in the rejected set.
    /// @param maxAllowed The maximum size an input proof can carry.
    error CoprocessorSetExceedsProofFormatLimit(uint256 coprocessorCount, uint256 maxAllowed);

    /// @notice The majority threshold is not more than half the number of coprocessors, or exceeds it.
    /// @param majorityThreshold The rejected threshold.
    /// @param coprocessorCount The number of coprocessors.
    error InvalidCoprocessorMajorityThreshold(uint256 majorityThreshold, uint256 coprocessorCount);

    /// @notice The coprocessor context ID was never defined, or is destroyed when a valid context is required.
    /// @param coprocessorContextId The invalid coprocessor context ID.
    error InvalidCoprocessorContext(uint256 coprocessorContextId);

    /// @notice The current coprocessor context cannot be destroyed.
    /// @param coprocessorContextId The current coprocessor context ID.
    error CurrentCoprocessorContextCannotBeDestroyed(uint256 coprocessorContextId);

    /**
     * @notice Returns the active KMS context and epoch IDs.
     * @return contextId The active context ID.
     * @return epochId The active epoch ID.
     */
    function getCurrentKmsContextAndEpoch() external view returns (uint256 contextId, uint256 epochId);

    /**
     * @notice Checks whether an epoch is active and belongs to the given KMS context.
     * @param kmsContextId The context ID the epoch must belong to.
     * @param epochId The epoch ID to check.
     * @return True if the epoch is active and owned by the context.
     */
    function isValidEpochForContext(uint256 kmsContextId, uint256 epochId) external view returns (bool);

    /**
     * @notice Returns the active KMS context ID.
     * @return The active context ID.
     */
    function getCurrentKmsContextId() external view returns (uint256);

    /**
     * @notice Returns the KMS context ID allocation counter: the latest issued context ID.
     * @dev This is the allocation frontier, always `>= getCurrentKmsContextId()`. It differs from the
     *      active context ID while a context is Pending or Created (an in-flight context switch).
     * @return The latest issued context ID.
     */
    function getCurrentKmsContextIdCounter() external view returns (uint256);

    /**
     * @notice Checks whether a KMS context ID is valid (exists, is not destroyed, and is active).
     * @param kmsContextId The context ID to check.
     * @return True if the context is valid.
     */
    function isValidKmsContext(uint256 kmsContextId) external view returns (bool);

    /**
     * @notice Checks whether a KMS context exists and has not been destroyed.
     * @dev Unlike `isValidKmsContext`, this returns true for a Pending or Created context, so it
     *      distinguishes an in-flight context switch from a destroyed or never-issued context.
     *      The v0.14 upgrade gave a state only to the context that was active at that upgrade,
     *      so this returns false for older contexts.
     * @param kmsContextId The context ID to check.
     * @return True if the context exists and is not destroyed.
     */
    function isLiveKmsContext(uint256 kmsContextId) external view returns (bool);

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
     * @dev The other threshold getters require an `Active` context. This one returns a value for any
     *      live context, whatever its state, so the kmsGen threshold stays readable even before the
     *      context becomes `Active`.
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
     * @dev The previous context is the current ID minus one.
     * @return The current coprocessor context ID.
     */
    function getCurrentCoprocessorContextId() external view returns (uint256);

    /**
     * @notice Checks whether a coprocessor context is defined and not destroyed.
     * @param coprocessorContextId The coprocessor context ID to check.
     * @return True if the context is valid.
     */
    function isValidCoprocessorContext(uint256 coprocessorContextId) external view returns (bool);

    /**
     * @notice Returns the coprocessors of a context, in definition order.
     * @dev Still returns the set of a destroyed context. Reverts with `InvalidCoprocessorContext`
     *      for an ID that was never defined.
     * @param coprocessorContextId The coprocessor context ID.
     * @return The coprocessors of the context.
     */
    function getCoprocessorsForContext(uint256 coprocessorContextId) external view returns (Coprocessor[] memory);

    /**
     * @notice Returns the majority threshold of a coprocessor context.
     * @dev Still returns the threshold of a destroyed context. Reverts with `InvalidCoprocessorContext`
     *      for an ID that was never defined.
     * @param coprocessorContextId The coprocessor context ID.
     * @return The number of matching coprocessor outputs required for consensus.
     */
    function getCoprocessorMajorityThresholdForContext(uint256 coprocessorContextId) external view returns (uint256);

    /**
     * @notice Checks whether an address is a coprocessor signer in the given context.
     * @dev Does not check that the context is valid: callers use `isValidCoprocessorContext` for that.
     * @param coprocessorContextId The coprocessor context ID.
     * @param signer The address to check.
     * @return True if the address is a signer of the context.
     */
    function isCoprocessorSignerForContext(uint256 coprocessorContextId, address signer) external view returns (bool);

    /**
     * @notice Checks whether an address is a coprocessor tx sender in the given context.
     * @dev Does not check that the context is valid: callers use `isValidCoprocessorContext` for that.
     * @param coprocessorContextId The coprocessor context ID.
     * @param txSender The address to check.
     * @return True if the address is a tx sender of the context.
     */
    function isCoprocessorTxSenderForContext(
        uint256 coprocessorContextId,
        address txSender
    ) external view returns (bool);

    /**
     * @notice Returns the contract version.
     * @return The version string.
     */
    function getVersion() external pure returns (string memory);
}
