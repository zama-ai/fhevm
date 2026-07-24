// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {KmsNode, KmsNodeParams, PcrValues, ChainUpgradeWindow} from "../shared/Structs.sol";
import {IKMSGeneration} from "./IKMSGeneration.sol";

/**
 * @title Interface for the ProtocolConfig contract.
 * @notice ProtocolConfig manages the KMS node set, threshold configuration, and context lifecycle
 * on the host chains.
 * @dev Ethereum is the canonical host and source of truth: the lifecycle/quorum functions run only
 * there. The same contract is deployed on every other host chain as a read-replica, advancing state
 * only through the owner-only mirror functions, which copy already-finalized Ethereum state without
 * replaying confirmations.
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
     * @param txSender The KMS tx sender that confirmed.
     * @param isPreviousTxSender Whether the tx sender is part of the previous active context.
     * @param isNewTxSender Whether the tx sender is part of the new pending context.
     */
    event KmsContextCreationConfirmation(
        uint256 indexed kmsContextId,
        address indexed txSender,
        bool isPreviousTxSender,
        bool isNewTxSender
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
     * @notice Emitted when a KMS epoch is destroyed.
     * @param epochId The destroyed epoch ID.
     */
    event KmsEpochDestroyed(uint256 indexed epochId);

    /**
     * @notice Emitted when a coprocessor upgrade is proposed. This event drives the
     *         coprocessor software upgrade.
     * @param proposalId Caller-supplied identifier for this upgrade attempt.
     * @param softwareVersion The coprocessor software version for the proposal.
     * @param chainUpgradeWindows The per-host-chain replay windows for the upgrade.
     * @param gwStartBlock The Gateway block at which GCS's gateway-listener resumes from.
     */
    event CoprocessorUpgradeProposed(
        uint256 indexed proposalId,
        string softwareVersion,
        ChainUpgradeWindow[] chainUpgradeWindows,
        uint64 gwStartBlock
    );

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
     * @param epochId The mirrored canonical epoch ID activated for the context.
     * @param kmsNodeParams The KMS nodes mirrored from the canonical context, including MPC metadata.
     * @param thresholds The thresholds mirrored from the canonical context.
     * @param softwareVersion The KMS software version of the canonical context.
     * @param pcrValues Accepted enclave PCR values of the canonical context.
     */
    event MirrorKmsContextAndEpoch(
        uint256 indexed contextId,
        uint256 indexed epochId,
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

    /// @notice Cannot destroy the latest active context.
    /// @param kmsContextId The latest active context ID.
    error LatestActiveKmsContextCannotBeDestroyed(uint256 kmsContextId);

    /// @notice Cannot destroy the latest active epoch.
    /// @param epochId The latest active epoch ID.
    error LatestActiveKmsEpochCannotBeDestroyed(uint256 epochId);

    /// @notice The epoch ID is invalid or not in the required lifecycle state for this operation.
    /// @param epochId The epoch ID.
    error InvalidKmsEpoch(uint256 epochId);

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

    /// @notice The tx sender has already confirmed creation for the KMS context.
    /// @param txSender The tx sender address.
    /// @param kmsContextId The context ID.
    error KmsContextCreationAlreadyConfirmed(address txSender, uint256 kmsContextId);

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

    /// @notice The mirrored context ID is not strictly greater than the latest activated one.
    /// @param contextId The rejected context ID.
    /// @param latestActiveKmsContextId The most recently activated mirrored context ID.
    error NonIncreasingKmsContextId(uint256 contextId, uint256 latestActiveKmsContextId);

    /// @notice The mirrored epoch ID is not strictly greater than the latest known one.
    /// @param epochId The rejected epoch ID.
    /// @param currentEpochId The latest known epoch ID.
    error NonIncreasingEpochId(uint256 epochId, uint256 currentEpochId);

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

    /// @notice The supplied `proposalId` is zero.
    error InvalidProposalId();

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
     * @notice Destroy a KMS context, preventing it from being used.
     * @param kmsContextId The context ID to destroy.
     */
    function destroyKmsContext(uint256 kmsContextId) external;

    /**
     * @notice Destroy a superseded (non-current) KMS epoch, preventing it from being used.
     * @param epochId The epoch ID to destroy.
     */
    function destroyKmsEpoch(uint256 epochId) external;

    /**
     * @notice Propose a coprocessor upgrade. Emits `CoprocessorUpgradeProposed` and does not
     *         change any on-chain state — the lifecycle of the proposal (dry-run, consensus,
     *         cutover, failure) is driven entirely off-chain.
     * @param proposalId Caller-supplied identifier for this upgrade attempt. Must be non-zero.
     *        Uniqueness across calls is the caller's responsibility; the contract does not enforce it.
     * @param softwareVersion The coprocessor software version.
     * @param chainUpgradeWindows The per-host-chain replay windows.
     * @param gwStartBlock The Gateway block to resume from.
     */
    function proposeCoprocessorUpgrade(
        uint256 proposalId,
        string calldata softwareVersion,
        ChainUpgradeWindow[] calldata chainUpgradeWindows,
        uint64 gwStartBlock
    ) external;

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

    // -----------------------------------------------------------------------------------------
    // Mirror functions
    //
    // The write path for non-canonical replicas. Ethereum's quorum has already finalized the state
    // these import, so they skip the confirmation flow and land it as Active. Owner-only; the
    // operator must fan each Ethereum rotation out to every replica in order. The strictly-increasing
    // ID checks guard against rollback, not skipped calls.
    // -----------------------------------------------------------------------------------------

    /**
     * @notice Mirror and immediately activate a canonical KMS context.
     * @dev Non-canonical hosts use this to import signer/threshold state without replaying
     *      context-creation confirmations. The `contextId` and `epochId` must be strictly greater
     *      than the latest active context and latest known epoch IDs. Gaps are allowed (canonical
     *      contexts/epochs that were destroyed or never activated are simply never mirrored).
     * @param contextId The canonical context ID to mirror; must exceed the current active context ID.
     * @param epochId The canonical epoch ID to activate for the mirrored context.
     * @param kmsNodeParams The KMS nodes from the canonical context, including MPC metadata.
     * @param thresholds The thresholds from the canonical context.
     * @param softwareVersion The KMS software version of the canonical context.
     * @param pcrValues Accepted enclave PCR values of the canonical context.
     */
    function mirrorKmsContextAndEpoch(
        uint256 contextId,
        uint256 epochId,
        KmsNodeParams[] calldata kmsNodeParams,
        KmsThresholds calldata thresholds,
        string calldata softwareVersion,
        PcrValues[] calldata pcrValues
    ) external;

    /**
     * @notice Mirror and immediately activate a canonical KMS epoch for the active context.
     * @dev Non-canonical hosts use this to advance the active epoch without replaying
     *      epoch-activation confirmations. The `epochId` must be strictly greater than the
     *      latest known epoch ID. The context must already be the active mirrored context
     *      (mirror the context first with `mirrorKmsContextAndEpoch`).
     * @param contextId The active mirrored context the epoch belongs to.
     * @param epochId The canonical epoch ID to mirror; must exceed the latest known epoch ID.
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

    /**
     * @notice Returns the active KMS context ID.
     * @return The active context ID.
     */
    function getCurrentKmsContextId() external view returns (uint256);

    /**
     * @notice Checks whether a KMS context ID is valid (exists, is not destroyed, and is active).
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
     * @dev Unlike the peer per-context threshold getters, this getter resolves the context through
     *      the live check, so it also returns a value for a `Created` (not yet `Active`) or resharing
     *      context. This is deliberate, so the key-generation threshold stays readable during
     *      resharing. A non-active context's threshold is not authoritative for the active committee.
     *      Consumers must not treat it as such.
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
     * @notice Returns the contract version.
     * @return The version string.
     */
    function getVersion() external pure returns (string memory);
}
