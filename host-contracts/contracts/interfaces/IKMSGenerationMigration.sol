// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {IKMSGeneration} from "./IKMSGeneration.sol";

/**
 * @title Migration interface for the host-side KMSGeneration contract.
 * @notice Isolates the migration-only ABI from the steady-state IKMSGeneration surface.
 */
interface IKMSGenerationMigration {
    /// @notice Thrown when migrated counters and active IDs do not describe a valid finalized state.
    error InvalidMigrationCounterState();

    /// @notice Thrown when migrated finalized consensus data is incomplete or inconsistent.
    error InvalidMigrationConsensusState(uint256 requestId);

    /// @notice Thrown when a migrated consensus tx sender is not registered in the migrated request context.
    error UnknownMigrationConsensusTxSender(uint256 requestId, address txSender);

    /// @notice Thrown when migrated key digests or CRS digest material is empty for a finalized request.
    error InvalidMigrationMaterial(uint256 requestId);

    /**
     * @notice Migration state for initializeFromMigration.
     * @dev Only active state is imported; historical Gateway state stays on the frozen Gateway contract.
     *      Migration expects already-finalized active key/prep-keygen/CRS state with registered KMS senders.
     */
    struct MigrationState {
        uint256 prepKeygenCounter;
        uint256 keyCounter;
        uint256 crsCounter;
        uint256 activeKeyId;
        uint256 activeCrsId;
        // Active prep-keygen <-> key pairing
        uint256 activePrepKeygenId;
        // Active key digests / CRS digest
        IKMSGeneration.KeyDigest[] activeKeyDigests;
        bytes activeCrsDigest;
        // Finalized consensus tx senders for migrated active items
        address[] keyConsensusTxSenders;
        bytes32 keyConsensusDigest;
        address[] crsConsensusTxSenders;
        bytes32 crsConsensusDigest;
        address[] prepKeygenConsensusTxSenders;
        bytes32 prepKeygenConsensusDigest;
        // CRS max bit length
        uint256 crsMaxBitLength;
        // Params types
        // The prep-keygen params type is also the keygen params type for the paired key lifecycle.
        IKMSGeneration.ParamsType prepKeygenParamsType;
        IKMSGeneration.ParamsType crsParamsType;
        // KMS context ID for the migrated active prep-keygen, key, and CRS state.
        uint256 contextId;
    }

    /**
     * @notice Migration initializer: imports active state from the frozen Gateway contract.
     * @param state The active state to import.
     */
    function initializeFromMigration(MigrationState calldata state) external;
}
