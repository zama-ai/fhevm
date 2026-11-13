// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {KmsNode, KmsThresholds} from "../shared/Structs.sol";
import {IProtocolConfigBase} from "./IProtocolConfigBase.sol";

/**
 * @notice A canonical context switch from the active context.
 * @param contextId The new context ID.
 * @param nodes The new committee, in canonical storage order.
 * @param thresholds The thresholds of the new committee.
 * @param signatures The active committee's ContextCreationConfirmation signatures.
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
 * @notice A canonical epoch activation after the active epoch.
 * @param epochId The new epoch ID.
 * @param epochMaterialHash The hash of the epoch's key and CRS results.
 * @param signatures The EpochActivationConfirmation signatures of every committee signer.
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
 * @notice Replica of the canonical ProtocolConfig on non-canonical host chains.
 */
interface IProtocolConfigReplica is IProtocolConfigBase {
    // -----------------------------------------------------------------------------------------
    // Events
    // -----------------------------------------------------------------------------------------

    /**
     * @notice Emitted by `mirrorKmsContextAndEpoch`.
     * @param contextId The activated context ID.
     * @param epochId The activated epoch ID.
     * @param nodes The stored committee.
     * @param thresholds The stored thresholds.
     */
    event MirrorKmsContextAndEpoch(
        uint256 indexed contextId,
        uint256 indexed epochId,
        KmsNode[] nodes,
        KmsThresholds thresholds
    );

    /**
     * @notice Emitted by `mirrorKmsEpoch`.
     * @param contextId The active context ID.
     * @param epochId The activated epoch ID.
     */
    event MirrorKmsEpoch(uint256 indexed contextId, uint256 indexed epochId);

    /**
     * @notice Emitted by `submitKmsContextAndEpoch`.
     * @param contextId The activated context ID.
     * @param epochId The activated epoch ID.
     */
    event SubmitKmsContextAndEpoch(uint256 indexed contextId, uint256 indexed epochId);

    /**
     * @notice Emitted by `submitKmsEpoch`.
     * @param contextId The active context ID.
     * @param epochId The activated epoch ID.
     */
    event SubmitKmsEpoch(uint256 indexed contextId, uint256 indexed epochId);

    // -----------------------------------------------------------------------------------------
    // Errors
    // -----------------------------------------------------------------------------------------

    /// @notice The context ID is not above the active one.
    /// @param contextId The submitted context ID.
    error StaleKmsContextSwitch(uint256 contextId);

    /// @notice The epoch ID is not above the active one.
    /// @param epochId The submitted epoch ID.
    error StaleKmsEpoch(uint256 epochId);

    /// @notice The context ID is above the active one. Retry after the replica passes it.
    /// @param contextId The context ID to destroy.
    error KmsContextDestructionAheadOfReplica(uint256 contextId);

    /// @notice The epoch ID is above the active one. Retry after the replica passes it.
    /// @param epochId The epoch ID to destroy.
    error KmsEpochDestructionAheadOfReplica(uint256 epochId);

    /// @notice There are fewer signatures than the quorum.
    /// @param signatureCount The number of signatures.
    /// @param quorum The required number of signatures.
    error KmsSignatureQuorumNotReached(uint256 signatureCount, uint256 quorum);

    /// @notice A signer is not in the committee that must sign.
    /// @param signer The recovered signer.
    /// @param contextId The context of the committee.
    error KmsSignerNotInCommittee(address signer, uint256 contextId);

    /// @notice Two signatures recover to the same signer.
    /// @param signer The duplicate signer.
    error DuplicateKmsSigner(address signer);

    // -----------------------------------------------------------------------------------------
    // Submit functions
    //
    // Anyone can submit canonical confirmation signatures. The replica verifies them under the
    // canonical EIP-712 domain against its stored committees, in canonical order only. The
    // submitter treats a stale revert as success.
    // -----------------------------------------------------------------------------------------

    /**
     * @notice Applies a canonical context switch and the first epoch of the new context.
     * @dev Needs `n - t` context signatures from the active committee and epoch signatures from
     *      every new signer. `context.contextId` must have no stored committee. After an owner
     *      rollback, a replay reverts with `KmsTxSenderAlreadyRegistered` and needs the mirror path.
     * @param context The new context and its signatures.
     * @param epoch The first epoch of the new context and its signatures.
     */
    function submitKmsContextAndEpoch(
        KmsContextSubmission calldata context,
        KmsEpochSubmission calldata epoch
    ) external;

    /**
     * @notice Applies a canonical resharing on the active context.
     * @dev Needs epoch signatures from every signer of the active committee.
     * @param epoch The new epoch and its signatures.
     */
    function submitKmsEpoch(KmsEpochSubmission calldata epoch) external;

    /**
     * @notice Applies a canonical context destruction.
     * @dev Needs `n - t` signatures from the active committee. Accepts an ID that was never stored.
     * @param destroyedContextId The context ID to destroy.
     * @param signatures The ContextDestructionConfirmation signatures.
     * @param extraData The extra data that every signature commits to.
     */
    function submitKmsContextDestruction(
        uint256 destroyedContextId,
        bytes[] calldata signatures,
        bytes calldata extraData
    ) external;

    /**
     * @notice Applies a canonical epoch destruction.
     * @dev Needs `n - t` signatures from the active committee. Accepts an ID that was never stored.
     * @param destroyedEpochId The epoch ID to destroy.
     * @param signatures The EpochDestructionConfirmation signatures.
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
    // Owner only, with no signature check and no ordering check. They can move the replica back
    // below a forged ID that canonical can never reach.
    // -----------------------------------------------------------------------------------------

    /**
     * @notice Activates a canonical context and epoch.
     * @dev Replaces any committee stored under `contextId`.
     * @param contextId The context ID to activate.
     * @param epochId The epoch ID to activate.
     * @param nodes The committee of the context.
     * @param thresholds The thresholds of the context.
     */
    function mirrorKmsContextAndEpoch(
        uint256 contextId,
        uint256 epochId,
        KmsNode[] calldata nodes,
        KmsThresholds calldata thresholds
    ) external;

    /**
     * @notice Activates a canonical epoch on the active context.
     * @param contextId The active context ID.
     * @param epochId The epoch ID to activate.
     */
    function mirrorKmsEpoch(uint256 contextId, uint256 epochId) external;

    /**
     * @notice Destroys a context.
     * @dev Unlike the submit path, the ID can be above the active one, for a forged ID after a rollback.
     * @param destroyedContextId The context ID to destroy.
     */
    function mirrorKmsContextDestruction(uint256 destroyedContextId) external;

    /**
     * @notice Destroys an epoch.
     * @dev Unlike the submit path, the ID can be above the active one, for a forged ID after a rollback.
     * @param destroyedEpochId The epoch ID to destroy.
     */
    function mirrorKmsEpochDestruction(uint256 destroyedEpochId) external;
}
