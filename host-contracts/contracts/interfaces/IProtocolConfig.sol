// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {KmsNodeParams, PcrValues} from "../shared/Structs.sol";
import {IKMSGeneration} from "./IKMSGeneration.sol";
import {IProtocolConfigCommon} from "./IProtocolConfigCommon.sol";

/**
 * @title Interface for the ProtocolConfig contract.
 * @notice ProtocolConfig manages the KMS node set, threshold configuration, and context lifecycle
 * on the Ethereum host chain. It replaces the context-management duties previously held by KMSVerifier.
 */
interface IProtocolConfig is IProtocolConfigCommon {
    /**
     * @notice A signed keygen result attested by a KMS signer during epoch activation.
     * @param prepKeygenId The preprocessing keygen ID the key derives from.
     * @param keyId The generated key ID.
     * @param keyDigests The per-type digests of the generated key.
     * @param signature The signer's EIP-712 KeygenVerification signature.
     */
    struct EpochKeyResult {
        uint256 prepKeygenId;
        uint256 keyId;
        IKMSGeneration.KeyDigest[] keyDigests;
        bytes signature;
    }

    /**
     * @notice A signed CRS result attested by a KMS signer during epoch activation.
     * @param crsId The generated CRS ID.
     * @param maxBitLength The maximum bit length the CRS supports.
     * @param crsDigest The digest of the generated CRS.
     * @param signature The signer's EIP-712 CrsgenVerification signature.
     */
    struct EpochCrsResult {
        uint256 crsId;
        uint256 maxBitLength;
        bytes crsDigest;
        bytes signature;
    }

    // -----------------------------------------------------------------------------------------
    // Events
    // -----------------------------------------------------------------------------------------

    /**
     * @notice Emitted when a new KMS context is created.
     * @param contextId The new context ID.
     * @param previousContextId The active context ID superseded by the new context.
     * @param kmsNodeParams The KMS nodes registered in the context, including MPC metadata.
     * @param thresholds The thresholds for the context.
     * @param softwareVersion The KMS software version expected for the context.
     * @param pcrValues Accepted enclave PCR values for the context.
     */
    event NewKmsContext(
        uint256 indexed contextId,
        uint256 indexed previousContextId,
        KmsNodeParams[] kmsNodeParams,
        KmsThresholds thresholds,
        string softwareVersion,
        PcrValues[] pcrValues
    );

    /**
     * @notice Emitted when a new pending epoch is ready for resharing under a KMS context.
     * @dev Signals Connectors to begin resharing key/CRS material into the new epoch. Emitted both
     *      for same-set resharing (a new epoch opened under the active context) and for a context
     *      switch (once enough previous and new signers confirm the pending context was created).
     * @param kmsContextId The context that owns the pending epoch.
     * @param epochId The pending epoch ID.
     * @param previousContextId The context that holds the previous epoch's shares.
     * @param previousEpochId The active epoch superseded by the pending epoch.
     * @param materialBlockNumber Block where Connectors should read previous key/CRS material
     *        from the canonical KMSGeneration contract.
     */
    event NewKmsEpoch(
        uint256 indexed kmsContextId,
        uint256 indexed epochId,
        uint256 previousContextId,
        uint256 previousEpochId,
        uint256 materialBlockNumber
    );

    /**
     * @notice Emitted when an epoch becomes active.
     * @param kmsContextId The activated context ID.
     * @param epochId The activated epoch ID.
     * @param keys Key results included in the activation.
     * @param crsList CRS results included in the activation.
     * @param kmsNodeStorageUrls Storage URLs for nodes in the activated context.
     */
    event ActivateEpoch(
        uint256 indexed kmsContextId,
        uint256 indexed epochId,
        EpochKeyResult[] keys,
        EpochCrsResult[] crsList,
        string[] kmsNodeStorageUrls
    );

    /**
     * @notice Emitted on every successful KMS context creation confirmation.
     * @param kmsContextId The pending context ID being confirmed.
     * @param signer The KMS signer that confirmed.
     * @param isPreviousSigner Whether the signer is part of the previous active context.
     * @param isNewSigner Whether the signer is part of the new pending context.
     */
    event KmsContextCreationConfirmation(
        uint256 indexed kmsContextId,
        address indexed signer,
        bool isPreviousSigner,
        bool isNewSigner
    );

    /**
     * @notice Emitted on every successful epoch activation confirmation.
     * @param epochId The pending epoch ID being confirmed.
     * @param signer The KMS signer that confirmed.
     * @param dataHash The digest of the structured key/CRS payload the signer agreed on.
     */
    event EpochActivationConfirmation(uint256 indexed epochId, address indexed signer, bytes32 dataHash);

    /**
     * @notice Emitted when a KMS context is destroyed.
     * @param kmsContextId The destroyed context ID.
     */
    event KmsContextDestroyed(uint256 indexed kmsContextId);

    /**
     * @notice Emitted when a pending epoch under the active KMS context is aborted.
     * @param kmsContextId The active context ID that owned the pending epoch.
     * @param epochId The aborted pending epoch ID.
     */
    event PendingEpochAborted(uint256 indexed kmsContextId, uint256 indexed epochId);

    /**
     * @notice Emitted when a pending KMS context is aborted before being created.
     * @param kmsContextId The aborted pending context ID.
     */
    event PendingContextAborted(uint256 indexed kmsContextId);

    /**
     * @notice Emitted when the public decryption threshold for a KMS context is updated.
     * @param kmsContextId The updated context ID.
     * @param threshold The new public decryption threshold.
     */
    event PublicDecryptionThresholdUpdated(uint256 indexed kmsContextId, uint256 threshold);

    /**
     * @notice Emitted when the user decryption threshold for a KMS context is updated.
     * @param kmsContextId The updated context ID.
     * @param threshold The new user decryption threshold.
     */
    event UserDecryptionThresholdUpdated(uint256 indexed kmsContextId, uint256 threshold);

    /**
     * @notice Emitted when the KMS generation threshold for a KMS context is updated.
     * @param kmsContextId The updated context ID.
     * @param threshold The new KMS generation threshold.
     */
    event KmsGenThresholdUpdated(uint256 indexed kmsContextId, uint256 threshold);

    /**
     * @notice Emitted when the MPC threshold for a KMS context is updated.
     * @param kmsContextId The updated context ID.
     * @param threshold The new MPC threshold.
     */
    event MpcThresholdUpdated(uint256 indexed kmsContextId, uint256 threshold);

    /**
     * @notice Emitted when a canonical KMS context is mirrored and activated.
     * @param contextId The mirrored canonical context ID.
     * @param kmsNodeParams The KMS nodes mirrored from the canonical context, including MPC metadata.
     * @param thresholds The thresholds mirrored from the canonical context.
     * @param softwareVersion The KMS software version of the canonical context.
     * @param pcrValues Accepted enclave PCR values of the canonical context.
     */
    event MirrorKmsContext(
        uint256 indexed contextId,
        KmsNodeParams[] kmsNodeParams,
        KmsThresholds thresholds,
        string softwareVersion,
        PcrValues[] pcrValues
    );

    /**
     * @notice Emitted when a canonical KMS epoch is mirrored and activated.
     * @param contextId The active mirrored context ID.
     * @param epochId The mirrored canonical epoch ID.
     */
    event MirrorKmsEpoch(uint256 indexed contextId, uint256 indexed epochId);

    // -----------------------------------------------------------------------------------------
    // Errors
    // -----------------------------------------------------------------------------------------

    /// @notice The epoch ID is invalid or not pending.
    /// @param epochId The epoch ID.
    error InvalidEpoch(uint256 epochId);

    /// @notice A pending epoch already exists.
    /// @param epochId The pending epoch ID.
    error PendingEpochAlreadyExists(uint256 epochId);

    /// @notice A pending KMS context already exists.
    /// @param kmsContextId The pending context ID.
    error PendingKmsContextAlreadyExists(uint256 kmsContextId);

    /// @notice The KMS context is not pending.
    /// @param kmsContextId The context ID.
    error KmsContextNotPending(uint256 kmsContextId);

    /// @notice The KMS context has not reached the created state.
    /// @param kmsContextId The context ID.
    error KmsContextNotCreated(uint256 kmsContextId);

    /// @notice The caller cannot confirm creation for the KMS context.
    /// @param caller The unauthorized caller.
    /// @param kmsContextId The context ID.
    error KmsContextCreationUnauthorized(address caller, uint256 kmsContextId);

    /// @notice The signer has already confirmed creation for the KMS context.
    /// @param signer The signer address.
    /// @param kmsContextId The context ID.
    error KmsContextCreationAlreadyConfirmed(address signer, uint256 kmsContextId);

    /// @notice The caller cannot confirm activation for the epoch.
    /// @param caller The unauthorized caller.
    /// @param epochId The epoch ID.
    error EpochActivationUnauthorized(address caller, uint256 epochId);

    /// @notice The signer has already confirmed activation for the epoch.
    /// @param signer The signer address.
    /// @param epochId The epoch ID.
    error EpochActivationAlreadyConfirmed(address signer, uint256 epochId);

    /// @notice The structured activation signature does not match the caller's KMS signer.
    /// @param signer The recovered signer.
    /// @param txSender The transaction sender.
    error EpochActivationSignerDoesNotMatchTxSender(address signer, address txSender);

    /// @notice The mirrored context ID is not strictly greater than the current one.
    /// @param contextId The rejected context ID.
    /// @param currentKmsContextId The current mirrored context ID.
    error NonIncreasingKmsContextId(uint256 contextId, uint256 currentKmsContextId);

    /// @notice The mirrored epoch ID is not strictly greater than the latest known one.
    /// @param epochId The rejected epoch ID.
    /// @param currentEpochId The latest known epoch ID.
    error NonIncreasingEpochId(uint256 epochId, uint256 currentEpochId);

    // -----------------------------------------------------------------------------------------
    // State-changing functions
    // -----------------------------------------------------------------------------------------

    /**
     * @notice Create a pending KMS context and pending epoch.
     * @param kmsNodeParams The KMS nodes to register, including MPC metadata.
     * @param thresholds The thresholds for the new context.
     * @param softwareVersion The KMS software version expected for the context.
     * @param pcrValues Accepted enclave PCR values for the context.
     */
    function defineNewKmsContextAndEpoch(
        KmsNodeParams[] calldata kmsNodeParams,
        KmsThresholds calldata thresholds,
        string calldata softwareVersion,
        PcrValues[] calldata pcrValues
    ) external;

    /**
     * @notice Create a pending epoch under the current active KMS context.
     */
    function defineNewEpochForCurrentKmsContext() external;

    /**
     * @notice Confirm that a pending KMS context has been created.
     * @param kmsContextId The pending context ID.
     */
    function confirmKmsContextCreation(uint256 kmsContextId) external;

    /**
     * @notice Confirm activation of a pending epoch.
     * @param epochId The pending epoch ID.
     * @param keys The key results to associate with the epoch.
     * @param crsList The CRS results to associate with the epoch.
     */
    function confirmEpochActivation(
        uint256 epochId,
        EpochKeyResult[] calldata keys,
        EpochCrsResult[] calldata crsList
    ) external;

    /**
     * @notice Abort a pending epoch under the current active KMS context.
     * @param epochId The pending epoch ID to abort.
     */
    function abortPendingEpoch(uint256 epochId) external;

    /**
     * @notice Abort a pending KMS context before it reaches the created state.
     * @dev Reverts once the context has been confirmed into `Created` or `Active`; use
     *      `destroyKmsContext` for non-active contexts past the pending stage.
     * @param kmsContextId The pending context ID to abort.
     */
    function abortPendingContext(uint256 kmsContextId) external;

    /**
     * @notice Destroy a KMS context, preventing it from being used.
     * @param kmsContextId The context ID to destroy.
     */
    function destroyKmsContext(uint256 kmsContextId) external;

    /**
     * @notice Update the public decryption threshold for a KMS context.
     * @param kmsContextId The context ID to update.
     * @param threshold The new public decryption threshold.
     */
    function updatePublicDecryptionThresholdForContext(uint256 kmsContextId, uint256 threshold) external;

    /**
     * @notice Update the user decryption threshold for a KMS context.
     * @param kmsContextId The context ID to update.
     * @param threshold The new user decryption threshold.
     */
    function updateUserDecryptionThresholdForContext(uint256 kmsContextId, uint256 threshold) external;

    /**
     * @notice Update the KMS generation threshold for a KMS context.
     * @param kmsContextId The context ID to update.
     * @param threshold The new KMS generation threshold.
     */
    function updateKmsGenThresholdForContext(uint256 kmsContextId, uint256 threshold) external;

    /**
     * @notice Update the MPC threshold for a KMS context.
     * @param kmsContextId The context ID to update.
     * @param threshold The new MPC threshold.
     */
    function updateMpcThresholdForContext(uint256 kmsContextId, uint256 threshold) external;

    /**
     * @notice Fresh deploy initializer.
     */
    function initializeFromEmptyProxy(
        KmsNodeParams[] calldata initialKmsNodeParams,
        KmsThresholds calldata initialThresholds,
        string calldata softwareVersion,
        PcrValues[] calldata pcrValues
    ) external;

    /**
     * @notice In-place reinitializer.
     */
    function reinitializeV2(
        KmsNodeParams[] calldata kmsNodeParams,
        KmsThresholds calldata thresholds,
        string calldata softwareVersion,
        PcrValues[] calldata pcrValues
    ) external;

    // -----------------------------------------------------------------------------------------
    // Mirror functions
    // -----------------------------------------------------------------------------------------

    /**
     * @notice Mirror and immediately activate a canonical KMS context.
     * @dev Non-canonical hosts use this to import signer/threshold state without replaying
     *      context-creation confirmations. The `contextId` must be strictly greater than the
     *      current active context ID. Gaps are allowed.
     */
    function mirrorKmsContext(
        uint256 contextId,
        KmsNodeParams[] calldata kmsNodeParams,
        KmsThresholds calldata thresholds,
        string calldata softwareVersion,
        PcrValues[] calldata pcrValues
    ) external;

    /**
     * @notice Mirror and immediately activate a canonical KMS epoch for the active context.
     * @dev Non-canonical hosts use this to advance the active epoch without replaying
     *      epoch-activation confirmations. The `epochId` must be strictly greater than the
     *      latest known epoch ID. Gaps are allowed.
     */
    function mirrorKmsEpoch(uint256 contextId, uint256 epochId) external;

    /**
     * @notice Returns the active KMS context and epoch IDs.
     * @return contextId The active context ID.
     * @return epochId The active epoch ID.
     */
    function getCurrentKmsContextAndEpoch() external view returns (uint256 contextId, uint256 epochId);

    /**
     * @notice Returns the context anchor recorded when NewKmsContext was emitted.
     * @param contextId The context ID.
     * @return emissionBlockNumber The block where NewKmsContext was emitted.
     * @return contextInfoHash Hash of the emitted context payload.
     */
    function getKmsContextAnchor(
        uint256 contextId
    ) external view returns (uint256 emissionBlockNumber, bytes32 contextInfoHash);

    /**
     * @notice Checks whether an epoch is active and belongs to the given KMS context.
     * @param kmsContextId The context ID the epoch must belong to.
     * @param epochId The epoch ID to check.
     * @return True if the epoch is active and owned by the context.
     */
    function isValidEpochForContext(uint256 kmsContextId, uint256 epochId) external view returns (bool);
}
