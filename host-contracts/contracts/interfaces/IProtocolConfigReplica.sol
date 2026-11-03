// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {KmsNode, KmsThresholds} from "../shared/Structs.sol";
import {IProtocolConfigBase} from "./IProtocolConfigBase.sol";

/**
 * @notice A canonical context switch, with the ContextCreationConfirmation signatures of the
 *         previous committee. The previous context is the active one.
 * @param contextId The new canonical context ID.
 * @param nodes The new committee, as stored on the canonical.
 * @param thresholds The thresholds of the new committee.
 * @param signatures The previous committee's signatures over one ContextCreationConfirmation.
 * @param extraData The extra data that every signature commits to.
 */
struct KmsContextSubmission {
    uint256 contextId;
    KmsNode[] nodes;
    KmsThresholds thresholds;
    bytes[] signatures;
    bytes extraData;
}

/**
 * @notice A canonical epoch activation, with the EpochActivationConfirmation signatures of the
 *         epoch's committee. The previous epoch is the active one.
 * @param epochId The new canonical epoch ID.
 * @param epochMaterialHash The hash of the epoch's key and CRS results.
 * @param signatures The signatures of every signer of the committee over one EpochActivationConfirmation.
 * @param extraData The extra data that every signature commits to.
 */
struct KmsEpochSubmission {
    uint256 epochId;
    bytes32 epochMaterialHash;
    bytes[] signatures;
    bytes extraData;
}

/**
 * @title Interface for the ProtocolConfigReplica contract.
 * @notice Read-replica of the canonical ProtocolConfig on non-canonical host chains.
 */
interface IProtocolConfigReplica is IProtocolConfigBase {
    // -----------------------------------------------------------------------------------------
    // Events
    // -----------------------------------------------------------------------------------------

    /**
     * @notice Emitted when the owner mirrors and activates a canonical KMS context.
     * @param contextId The mirrored canonical context ID.
     * @param epochId The mirrored canonical epoch ID activated for the context.
     * @param nodes The KMS nodes mirrored from the canonical context.
     * @param thresholds The thresholds mirrored from the canonical context.
     */
    event MirrorKmsContextAndEpoch(
        uint256 indexed contextId,
        uint256 indexed epochId,
        KmsNode[] nodes,
        KmsThresholds thresholds
    );

    /**
     * @notice Emitted when a canonical KMS epoch is mirrored and activated.
     * @param contextId The active mirrored context ID.
     * @param epochId The mirrored canonical epoch ID.
     */
    event MirrorKmsEpoch(uint256 indexed contextId, uint256 indexed epochId);

    /**
     * @notice Emitted when a signed context switch is applied.
     * @param contextId The activated context ID.
     * @param epochId The activated epoch ID.
     */
    event KmsContextSwitchSubmitted(uint256 indexed contextId, uint256 indexed epochId);

    /**
     * @notice Emitted when a signed same-set resharing is applied.
     * @param contextId The active context ID.
     * @param epochId The activated epoch ID.
     */
    event KmsKeyResharingSubmitted(uint256 indexed contextId, uint256 indexed epochId);

    // -----------------------------------------------------------------------------------------
    // Errors
    // -----------------------------------------------------------------------------------------

    /// @notice The context switch is already applied. The context ID is not above the active one.
    /// @param contextId The submitted context ID.
    error KmsContextSwitchAlreadyApplied(uint256 contextId);

    /// @notice The resharing is already applied. The epoch ID is not above the active one.
    /// @param epochId The submitted epoch ID.
    error KmsKeyResharingAlreadyApplied(uint256 epochId);

    /// @notice The context ID is already destroyed.
    /// @param contextId The destroyed context ID.
    error KmsContextAlreadyDestroyed(uint256 contextId);

    /// @notice The epoch ID is already destroyed.
    /// @param epochId The destroyed epoch ID.
    error KmsEpochAlreadyDestroyed(uint256 epochId);

    /// @notice The submitted context ID was destroyed. It cannot be activated through a submission.
    /// @param contextId The destroyed context ID.
    error DestroyedKmsContextCannotBeActivated(uint256 contextId);

    /// @notice The submitted epoch ID was destroyed. It cannot be activated through a submission.
    /// @param epochId The destroyed epoch ID.
    error DestroyedKmsEpochCannotBeActivated(uint256 epochId);

    /// @notice The epoch ID is not strictly greater than the active one.
    /// @param epochId The rejected epoch ID.
    /// @param activeEpochId The active epoch ID.
    error NonIncreasingEpochId(uint256 epochId, uint256 activeEpochId);

    /// @notice The destroyed context ID is at or above the active one. Retry after the replica moves
    ///         past it.
    /// @param contextId The destroyed context ID.
    /// @param activeContextId The active context ID.
    error KmsContextDestructionAheadOfReplica(uint256 contextId, uint256 activeContextId);

    /// @notice The destroyed epoch ID is at or above the active one. Retry after the replica moves
    ///         past it.
    /// @param epochId The destroyed epoch ID.
    /// @param activeEpochId The active epoch ID.
    error KmsEpochDestructionAheadOfReplica(uint256 epochId, uint256 activeEpochId);

    /// @notice Fewer signatures than the quorum.
    /// @param signatureCount The number of submitted signatures.
    /// @param quorum The required number of signatures.
    error KmsSignatureQuorumNotReached(uint256 signatureCount, uint256 quorum);

    /// @notice A signature recovers to an address outside the committee that must sign it.
    /// @param signer The recovered signer.
    /// @param contextId The context of the committee.
    error KmsSignerNotInCommittee(address signer, uint256 contextId);

    /// @notice Two signatures recover to the same signer.
    /// @param signer The duplicate signer.
    error DuplicateKmsSigner(address signer);

    /// @notice The canonical EIP-712 domain has a zero chain ID or a zero address.
    /// @param chainId The canonical chain ID.
    /// @param protocolConfig The canonical ProtocolConfig address.
    error InvalidCanonicalDomain(uint64 chainId, address protocolConfig);

    // -----------------------------------------------------------------------------------------
    // Submit functions
    //
    // The permissionless write path. Anyone may submit the canonical's confirmation signatures. The
    // replica verifies them under the canonical's EIP-712 domain against its own stored committees.
    // Context switches and resharings apply in canonical order only. The replica builds the signed
    // previous IDs from its active pair, so a submission out of order reverts with a signer error. A
    // submission that is already applied reverts with a dedicated error, which the submitter treats
    // as success. Only the owner functions can activate a destroyed ID.
    // -----------------------------------------------------------------------------------------

    /**
     * @notice Applies a canonical context switch and the first epoch of the new context.
     * @dev The switch builds on the active context and epoch. Verifies `n - t`
     *      ContextCreationConfirmation signatures of the previous committee. Then stores the submitted
     *      committee under `context.contextId`, replacing any committee stored under that ID. Then
     *      verifies the EpochActivationConfirmation signatures of every new signer.
     *      Every context signature must recover to a previous-committee signer. Leave out the
     *      signatures of signers that are only in the new committee. Reverts with
     *      `NonIncreasingEpochId` if the epoch ID is not above the active one. Reverts with
     *      `DestroyedKmsContextCannotBeActivated` or `DestroyedKmsEpochCannotBeActivated` if either ID was destroyed.
     * @param context The context switch and its signatures.
     * @param epoch The first epoch of the new context and its signatures.
     */
    function submitKmsContextSwitch(KmsContextSubmission calldata context, KmsEpochSubmission calldata epoch) external;

    /**
     * @notice Applies a canonical same-set resharing.
     * @dev The resharing builds on the active context and epoch. Verifies the
     *      EpochActivationConfirmation signatures of every signer of the active committee. Reverts
     *      with `DestroyedKmsEpochCannotBeActivated` if the epoch ID was destroyed.
     * @param epoch The new epoch and its signatures.
     */
    function submitKmsKeyResharing(KmsEpochSubmission calldata epoch) external;

    /**
     * @notice Applies a canonical context destruction.
     * @dev Verifies `n - t` ContextDestructionConfirmation signatures of the active committee.
     *      Accepts an ID that was never stored, because the canonical can destroy a context that never
     *      reached this replica. Reverts with `KmsContextDestructionAheadOfReplica` for an ID at or
     *      above the active one, even if the owner already destroyed it. The submitter retries it
     *      after the replica moves past that ID.
     * @param destroyedContextId The destroyed context ID.
     * @param signatures The signatures over one ContextDestructionConfirmation.
     * @param extraData The extra data that every signature commits to.
     */
    function submitKmsContextDestruction(
        uint256 destroyedContextId,
        bytes[] calldata signatures,
        bytes calldata extraData
    ) external;

    /**
     * @notice Applies a canonical epoch destruction.
     * @dev Verifies `n - t` EpochDestructionConfirmation signatures of the active committee. Accepts an
     *      ID that was never stored. Reverts with `KmsEpochDestructionAheadOfReplica` for an ID at or
     *      above the active one, even if the owner already destroyed it. The submitter retries it
     *      after the replica moves past that ID.
     * @param destroyedEpochId The destroyed epoch ID.
     * @param signatures The signatures over one EpochDestructionConfirmation.
     * @param extraData The extra data that every signature commits to.
     */
    function submitKmsEpochDestruction(
        uint256 destroyedEpochId,
        bytes[] calldata signatures,
        bytes calldata extraData
    ) external;

    // -----------------------------------------------------------------------------------------
    // Mirror functions
    //
    // The owner path. The repo mirror tasks use it for rotations. The owner also uses it for
    // recovery. The owner functions do not check signatures. `mirrorKmsContextAndEpoch` and
    // `mirrorKmsEpoch` have no ordering check on the IDs, so they can move the replica back.
    // Activation replaces the Destroyed state of the pair.
    // -----------------------------------------------------------------------------------------

    /**
     * @notice Mirror and immediately activate a canonical KMS context.
     * @dev Replaces any committee stored under `contextId`.
     * @param contextId The canonical context ID to mirror.
     * @param epochId The canonical epoch ID to activate for the mirrored context.
     * @param nodes The KMS nodes of the canonical context.
     * @param thresholds The thresholds of the canonical context.
     */
    function mirrorKmsContextAndEpoch(
        uint256 contextId,
        uint256 epochId,
        KmsNode[] calldata nodes,
        KmsThresholds calldata thresholds
    ) external;

    /**
     * @notice Mirror and immediately activate a canonical KMS epoch for the active context.
     * @dev The context must already be the active mirrored context (mirror the context first with
     *      `mirrorKmsContextAndEpoch`).
     * @param contextId The active mirrored context the epoch belongs to.
     * @param epochId The canonical epoch ID to mirror.
     */
    function mirrorKmsEpoch(uint256 contextId, uint256 epochId) external;

    /**
     * @notice Mirror a canonical context destruction.
     * @dev The active context cannot be destroyed.
     * @param destroyedContextId The destroyed context ID.
     */
    function mirrorKmsContextDestruction(uint256 destroyedContextId) external;

    /**
     * @notice Mirror a canonical epoch destruction.
     * @dev The active epoch cannot be destroyed.
     * @param destroyedEpochId The destroyed epoch ID.
     */
    function mirrorKmsEpochDestruction(uint256 destroyedEpochId) external;
}
