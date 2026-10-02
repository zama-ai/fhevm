// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {KmsThresholds, KmsNodeParams, PcrValues} from "../shared/Structs.sol";
import {IProtocolConfigBase} from "./IProtocolConfigBase.sol";

/**
 * @title Interface for the ProtocolConfigReplica contract.
 * @notice ProtocolConfigReplica is the read-replica of the canonical ProtocolConfig on every
 * non-canonical host chain (e.g. Polygon).
 * @dev It advances state only through the owner-only mirror functions, which copy already-finalized
 * Ethereum state without replaying confirmations.
 */
interface IProtocolConfigReplica is IProtocolConfigBase {
    // -----------------------------------------------------------------------------------------
    // Events
    // -----------------------------------------------------------------------------------------

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

    /// @notice The mirrored epoch ID is not strictly greater than the latest known one.
    /// @param epochId The rejected epoch ID.
    /// @param currentEpochId The latest known epoch ID.
    error NonIncreasingEpochId(uint256 epochId, uint256 currentEpochId);

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
     * @dev Imports signer/threshold state without replaying context-creation confirmations. The
     *      `contextId` and `epochId` must be strictly greater than the latest active context and
     *      latest known epoch IDs. Gaps are allowed (canonical contexts/epochs that were destroyed
     *      or never activated are simply never mirrored).
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
     * @dev Advances the active epoch without replaying epoch-activation confirmations. The
     *      `epochId` must be strictly greater than the latest known epoch ID. The context must
     *      already be the active mirrored context (mirror the context first with
     *      `mirrorKmsContextAndEpoch`).
     * @param contextId The active mirrored context the epoch belongs to.
     * @param epochId The canonical epoch ID to mirror; must exceed the latest known epoch ID.
     */
    function mirrorKmsEpoch(uint256 contextId, uint256 epochId) external;
}
