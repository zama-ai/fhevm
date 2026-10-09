// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {KmsThresholds, KmsNodeParams, PcrValues, ChainUpgradeWindow} from "../shared/Structs.sol";
import {IKMSGeneration} from "./IKMSGeneration.sol";
import {IProtocolConfigBase} from "./IProtocolConfigBase.sol";

/**
 * @title Interface for the ProtocolConfig contract.
 * @notice ProtocolConfig manages the KMS node set, threshold configuration, and context lifecycle
 * on the canonical host chain.
 * @dev Ethereum is the canonical host and source of truth: the lifecycle/quorum functions run only
 * there. Every other host chain runs `ProtocolConfigReplica` (see `IProtocolConfigReplica`).
 */
interface IProtocolConfig is IProtocolConfigBase {
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
     * @param signer The KMS signer recovered from the ContextCreationConfirmation signature.
     * @param signature The signer's EIP-712 ContextCreationConfirmation signature.
     * @param extraData The extra data bound into the signature.
     */
    event KmsContextCreationConfirmation(
        uint256 indexed kmsContextId,
        address indexed signer,
        bytes signature,
        bytes extraData
    );

    /**
     * @notice Emitted on every successful epoch activation confirmation.
     * @param epochId The pending epoch ID being confirmed.
     * @param signer The KMS signer that confirmed.
     * @param epochMaterialHash The hash of the key and CRS results the signer agreed on.
     * @param signature The signer's EIP-712 EpochActivationConfirmation signature.
     * @param extraData The extra data bound into the signature.
     */
    event EpochActivationConfirmation(
        uint256 indexed epochId,
        address indexed signer,
        bytes32 epochMaterialHash,
        bytes signature,
        bytes extraData
    );

    /**
     * @notice Emitted on every successful KMS context destruction confirmation.
     * @param destroyedContextId The destroyed context ID.
     * @param destroyedEpochIds The epochs the KMS nodes destroyed with the context.
     * @param signer The active-committee KMS signer that confirmed.
     * @param signature The signer's EIP-712 ContextDestructionConfirmation signature.
     * @param extraData The extra data bound into the signature.
     */
    event KmsContextDestructionConfirmed(
        uint256 indexed destroyedContextId,
        uint256[] destroyedEpochIds,
        address indexed signer,
        bytes signature,
        bytes extraData
    );

    /**
     * @notice Emitted on every successful KMS epoch destruction confirmation.
     * @param destroyedEpochId The destroyed epoch ID.
     * @param signer The active-committee KMS signer that confirmed.
     * @param signature The signer's EIP-712 EpochDestructionConfirmation signature.
     * @param extraData The extra data bound into the signature.
     */
    event KmsEpochDestructionConfirmed(
        uint256 indexed destroyedEpochId,
        address indexed signer,
        bytes signature,
        bytes extraData
    );

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

    // -----------------------------------------------------------------------------------------
    // Errors
    // -----------------------------------------------------------------------------------------

    /// @notice Cannot destroy the latest active context.
    /// @param kmsContextId The latest active context ID.
    error LatestActiveKmsContextCannotBeDestroyed(uint256 kmsContextId);

    /// @notice Cannot destroy the latest active epoch.
    /// @param epochId The latest active epoch ID.
    error LatestActiveKmsEpochCannotBeDestroyed(uint256 epochId);

    /// @notice The KMS context is not pending.
    /// @param kmsContextId The context ID.
    error KmsContextNotPending(uint256 kmsContextId);

    /// @notice A context switch or epoch rotation is still settling; settle it before opening another.
    /// @param kmsContextId The latest-issued context ID.
    /// @param epochId The latest-issued epoch ID.
    error KmsLifecycleOperationInFlight(uint256 kmsContextId, uint256 epochId);

    /// @notice The recovered signer is in neither the previous nor the new committee.
    /// @param signer The recovered signer.
    /// @param kmsContextId The context ID.
    error KmsContextCreationUnauthorized(address signer, uint256 kmsContextId);

    /// @notice The signer has already confirmed creation for the KMS context.
    /// @param signer The signer address.
    /// @param kmsContextId The context ID.
    error KmsContextCreationAlreadyConfirmed(address signer, uint256 kmsContextId);

    /// @notice The recovered signer is not in the epoch's committee.
    /// @param signer The recovered signer.
    /// @param epochId The epoch ID.
    error EpochActivationUnauthorized(address signer, uint256 epochId);

    /// @notice The signer has already confirmed activation for the epoch.
    /// @param signer The signer address.
    /// @param epochId The epoch ID.
    error EpochActivationAlreadyConfirmed(address signer, uint256 epochId);

    /// @notice The epoch activation payload has no keys or no CRS, so a required attestation cannot be verified.
    /// @param epochId The epoch ID.
    error EmptyEpochActivationAttestation(uint256 epochId);

    /// @notice An epoch activation signature recovers to a different signer than the first key result.
    /// @param expectedSigner The signer recovered from the first key result.
    /// @param recoveredSigner The signer recovered from the mismatching signature.
    error EpochResultSignerMismatch(address expectedSigner, address recoveredSigner);

    /// @notice The KMS context was not destroyed with destroyKmsContext.
    /// @param kmsContextId The context ID.
    error KmsContextNotDestroyed(uint256 kmsContextId);

    /// @notice The epoch was not destroyed with destroyKmsEpoch.
    /// @param epochId The epoch ID.
    error KmsEpochNotDestroyed(uint256 epochId);

    /// @notice The recovered signer is not in the active committee.
    /// @param signer The recovered signer.
    /// @param kmsContextId The destroyed context ID.
    error KmsContextDestructionUnauthorized(address signer, uint256 kmsContextId);

    /// @notice The recovered signer is not in the active committee.
    /// @param signer The recovered signer.
    /// @param epochId The destroyed epoch ID.
    error KmsEpochDestructionUnauthorized(address signer, uint256 epochId);

    /// @notice The signer has already confirmed destruction of the KMS context.
    /// @param signer The signer address.
    /// @param kmsContextId The destroyed context ID.
    error KmsContextDestructionAlreadyConfirmed(address signer, uint256 kmsContextId);

    /// @notice The signer has already confirmed destruction of the epoch.
    /// @param signer The signer address.
    /// @param epochId The destroyed epoch ID.
    error KmsEpochDestructionAlreadyConfirmed(address signer, uint256 epochId);

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
    // View functions
    // -----------------------------------------------------------------------------------------

    /**
     * @notice Returns the context anchor recorded when NewKmsContext was emitted.
     * @param contextId The context ID.
     * @return emissionBlockNumber The block where NewKmsContext was emitted, or 0 if no anchor was recorded.
     * @return contextInfoHash Hash of the emitted context payload, or 0x0 if no anchor was recorded.
     */
    function getKmsContextAnchor(
        uint256 contextId
    ) external view returns (uint256 emissionBlockNumber, bytes32 contextInfoHash);

    /**
     * @notice Returns the previous-committee confirmation quorum cached for a context at define time.
     * @dev The `(n - t)` target that `confirmKmsContextCreation` requires from the previous committee.
     *      Returns 0 for a context that was never a switch target or whose bookkeeping was cleared on
     *      destruction.
     * @param kmsContextId The context ID.
     * @return The cached previous-committee confirmation target.
     */
    function getContextCreationPreviousTxSenderThreshold(uint256 kmsContextId) external view returns (uint256);

    // -----------------------------------------------------------------------------------------
    // State-changing functions
    // -----------------------------------------------------------------------------------------

    /**
     * @notice Create a pending KMS context and pending epoch.
     * @dev Every committee must satisfy `n >= 3t + 1`. The contract does not check it.
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
     * @notice Submit a KMS signer's ContextCreationConfirmation signature for a pending context.
     * @dev Anyone may call. The recovered signer must be in the previous or the new committee.
     * @param kmsContextId The pending context ID.
     * @param signature The signer's EIP-712 ContextCreationConfirmation signature.
     * @param extraData The extra data bound into the signature.
     */
    function confirmKmsContextCreation(
        uint256 kmsContextId,
        bytes calldata signature,
        bytes calldata extraData
    ) external;

    /**
     * @notice Submit a KMS signer's epoch results and EpochActivationConfirmation signature.
     * @dev Anyone may call. Every key and CRS signature and the EpochActivationConfirmation
     *      signature must recover to the same signer of the epoch's committee.
     * @param epochId The pending epoch ID.
     * @param keys The key results to associate with the epoch.
     * @param crsList The CRS results to associate with the epoch.
     * @param signature The signer's EIP-712 EpochActivationConfirmation signature.
     * @param extraData The extra data bound into the signature.
     */
    function confirmEpochActivation(
        uint256 epochId,
        EpochKeyResult[] calldata keys,
        EpochCrsResult[] calldata crsList,
        bytes calldata signature,
        bytes calldata extraData
    ) external;

    /**
     * @notice Submit an active-committee signer's ContextDestructionConfirmation signature.
     * @dev Anyone may call. The context must be destroyed with destroyKmsContext first.
     * @param destroyedContextId The destroyed context ID.
     * @param destroyedEpochIds The epochs the KMS nodes destroyed with the context.
     * @param signature The signer's EIP-712 ContextDestructionConfirmation signature.
     * @param extraData The extra data bound into the signature.
     */
    function confirmKmsContextDestruction(
        uint256 destroyedContextId,
        uint256[] calldata destroyedEpochIds,
        bytes calldata signature,
        bytes calldata extraData
    ) external;

    /**
     * @notice Submit an active-committee signer's EpochDestructionConfirmation signature.
     * @dev Anyone may call. The epoch must be destroyed with destroyKmsEpoch first.
     * @param destroyedEpochId The destroyed epoch ID.
     * @param signature The signer's EIP-712 EpochDestructionConfirmation signature.
     * @param extraData The extra data bound into the signature.
     */
    function confirmKmsEpochDestruction(
        uint256 destroyedEpochId,
        bytes calldata signature,
        bytes calldata extraData
    ) external;

    /**
     * @notice Destroy a KMS context, preventing it from being used.
     * @param kmsContextId The context ID to destroy.
     */
    function destroyKmsContext(uint256 kmsContextId) external;

    /**
     * @notice Destroy a superseded (non-current) KMS epoch, preventing it from being used.
     *         Also used to abort a stuck Pending epoch of an Active context — a same-set rotation whose
     *         one-shot activation confirmations diverged and can no longer reach unanimity.
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
}
