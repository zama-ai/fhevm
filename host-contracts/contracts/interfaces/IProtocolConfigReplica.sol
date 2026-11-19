// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {KmsThresholds, KmsNode} from "../shared/Structs.sol";
import {IProtocolConfigBase} from "./IProtocolConfigBase.sol";

/**
 * @notice The context metadata derived from a context activation on the canonical contract.
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
 * @notice The epoch metadata derived from an epoch activation on the canonical contract.
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
     * @notice Emitted when a canonical KMS context is mirrored and activated.
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
     * @param contextId The active context ID.
     * @param epochId The mirrored canonical epoch ID.
     */
    event MirrorKmsEpoch(uint256 indexed contextId, uint256 indexed epochId);

    /**
     * @notice Emitted when a signed KMS context switch is submitted and activated.
     * @param contextId The submitted context ID.
     * @param epochId The submitted epoch ID activated for the context.
     * @param nodeConfigHash The hash of the committee and the thresholds that the signatures confirm.
     */
    event SubmitKmsContextAndEpoch(uint256 indexed contextId, uint256 indexed epochId, bytes32 nodeConfigHash);

    /**
     * @notice Emitted when a signed KMS epoch rotation is submitted and activated.
     * @param contextId The active context ID.
     * @param epochId The submitted epoch ID.
     */
    event SubmitKmsEpoch(uint256 indexed contextId, uint256 indexed epochId);

    // -----------------------------------------------------------------------------------------
    // Errors
    // -----------------------------------------------------------------------------------------

    /// @notice The context ID is not above the active one.
    /// @dev This does not prove that the replica holds the canonical state. A forged committee can hold
    ///      an ID that canonical activates later.
    /// @param contextId The submitted context ID.
    error KmsContextNotAboveActive(uint256 contextId);

    /// @notice The epoch ID is not above the active one.
    /// @dev This does not prove that the replica holds the canonical state. A forged committee can hold
    ///      an ID that canonical activates later.
    /// @param epochId The submitted epoch ID.
    error KmsEpochNotAboveActive(uint256 epochId);

    /// @notice The context ID already stores a committee. Only the mirror path can replace it.
    /// @param contextId The submitted context ID.
    error KmsContextAlreadyStored(uint256 contextId);

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
    // canonical EIP-712 domain against its stored committees. A context switch or a resharing applies
    // only on top of the active pair that its signatures name.
    // -----------------------------------------------------------------------------------------

    /**
     * @notice Applies a canonical context switch and the first epoch of the new context.
     * @dev Needs `n - t` context signatures from the active committee and an epoch signature from
     *      every new signer. Reverts with `KmsContextAlreadyStored` before any signature check if
     *      `context.contextId` stores a committee, as after a recovery that moved the replica back.
     *      Only the mirror path can then apply that ID.
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
     * @dev Needs `n - t` signatures from the active committee. Reverts if the ID is not live on the
     *      replica.
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
     * @dev Needs `n - t` signatures from the active committee. Reverts if the epoch is not Active on the
     *      replica.
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
    // Owner only, with no signature check and no ordering check, so they can move the replica back.
    // Do that only in a recovery, and never to a committee that signed an ID that the recovery
    // removes, because that committee can sign new IDs.
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
     * @dev Unlike the submit path, the ID does not need to be live on the replica.
     * @param destroyedContextId The context ID to destroy.
     */
    function mirrorKmsContextDestruction(uint256 destroyedContextId) external;

    /**
     * @notice Destroys an epoch.
     * @dev Unlike the submit path, the epoch does not need to be Active on the replica.
     * @param destroyedEpochId The epoch ID to destroy.
     */
    function mirrorKmsEpochDestruction(uint256 destroyedEpochId) external;
}
