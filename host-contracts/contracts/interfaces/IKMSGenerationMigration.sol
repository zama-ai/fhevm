// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {IKMSGeneration} from "./IKMSGeneration.sol";

/**
 * @title Migration interface for the host-side KMSGeneration contract.
 * @notice Isolates the migration-only ABI from the steady-state IKMSGeneration surface.
 */
interface IKMSGenerationMigration {
    /**
     * @notice Migration state for initializeFromMigration.
     * @dev Only active state is imported; historical Gateway state stays on the frozen Gateway contract.
     */
    struct MigrationState {
        uint256 prepKeygenCounter;
        uint256 keyCounter;
        uint256 crsCounter;
        uint256 activeKeyId;
        uint256 activeCrsId;
        // Active prep-keygen <-> key pairing
        uint256 activePrepKeygenId;
        uint256 activeKeyIdForPairing;
        // Active key digests / CRS digest
        IKMSGeneration.KeyDigest[] activeKeyDigests;
        bytes activeCrsDigest;
        // Consensus state for migrated active items
        address[] keyConsensusTxSenders;
        bytes32 keyConsensusDigest;
        address[] crsConsensusTxSenders;
        bytes32 crsConsensusDigest;
        address[] prepKeygenConsensusTxSenders;
        bytes32 prepKeygenConsensusDigest;
        // isRequestDone flags
        bool isPrepKeygenDone;
        bool isKeygenDone;
        bool isCrsgenDone;
        // CRS max bit length
        uint256 crsMaxBitLength;
        // Params types
        // The prep-keygen params type is also the keygen params type for the paired key lifecycle.
        IKMSGeneration.ParamsType prepKeygenParamsType;
        IKMSGeneration.ParamsType crsParamsType;
        // Extra data for migrated active requests
        bytes prepKeygenExtraData;
        bytes keygenExtraData;
        bytes crsgenExtraData;
    }

    /**
     * @notice Migration initializer: imports active state from the frozen Gateway contract.
     * @param state The active state to import.
     */
    function initializeFromMigration(MigrationState calldata state) external;
}
