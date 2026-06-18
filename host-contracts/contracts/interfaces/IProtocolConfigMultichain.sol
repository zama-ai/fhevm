// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {IProtocolConfigCommon} from "./IProtocolConfigCommon.sol";
import {KmsNodeParams, PcrValues} from "../shared/Structs.sol";

/**
 * @title Multichain ProtocolConfig mirror interface.
 * @notice Owner-driven canonical context mirroring for non-canonical host chains.
 * @dev Non-canonical chains do not run the signer-driven KMS lifecycle, so the owner mirrors
 *      already-activated canonical context data here using the canonical context ID and source
 *      provenance (chain ID, block number, and canonical `ProtocolConfig` address).
 */
interface IProtocolConfigMultichain is IProtocolConfigCommon {
    /**
     * @notice Canonical source provenance for a mirrored context.
     * @dev The three fields always travel together; grouping them avoids transposing the two
     *      adjacent `uint256` fields (`sourceChainId` / `sourceBlockNumber`) at call sites.
     * @param sourceChainId The chain ID of the canonical `ProtocolConfig`.
     * @param sourceBlockNumber The block at which the canonical context was read.
     * @param sourceProtocolConfig The canonical `ProtocolConfig` address.
     */
    struct MirroredContextSource {
        uint256 sourceChainId;
        uint256 sourceBlockNumber;
        address sourceProtocolConfig;
    }

    /**
     * @notice Emitted when a canonical KMS context is mirrored and activated.
     * @param contextId The mirrored canonical context ID.
     * @param kmsNodeParams The KMS nodes mirrored from the canonical context, including MPC metadata.
     * @param thresholds The thresholds mirrored from the canonical context.
     * @param softwareVersion The KMS software version of the canonical context.
     * @param pcrValues Accepted enclave PCR values of the canonical context.
     * @param sourceChainId The chain ID of the canonical `ProtocolConfig`.
     * @param sourceBlockNumber The block at which the canonical context was read.
     * @param sourceProtocolConfig The canonical `ProtocolConfig` address.
     */
    event MirrorKmsContext(
        uint256 indexed contextId,
        KmsNodeParams[] kmsNodeParams,
        KmsThresholds thresholds,
        string softwareVersion,
        PcrValues[] pcrValues,
        uint256 indexed sourceChainId,
        uint256 sourceBlockNumber,
        address indexed sourceProtocolConfig
    );

    /**
     * @notice Emitted when the public decryption threshold of a mirrored KMS context is updated.
     * @param contextId The mirrored context ID.
     * @param threshold The public decryption threshold mirrored from the canonical context.
     */
    event MirrorPublicDecryptionThreshold(uint256 indexed contextId, uint256 threshold);

    /**
     * @notice Emitted when the user decryption threshold of a mirrored KMS context is updated.
     * @param contextId The mirrored context ID.
     * @param threshold The user decryption threshold mirrored from the canonical context.
     */
    event MirrorUserDecryptionThreshold(uint256 indexed contextId, uint256 threshold);

    /**
     * @notice Emitted when the KMS generation threshold of a mirrored KMS context is updated.
     * @param contextId The mirrored context ID.
     * @param threshold The KMS generation threshold mirrored from the canonical context.
     */
    event MirrorKmsGenThreshold(uint256 indexed contextId, uint256 threshold);

    /**
     * @notice Emitted when the MPC threshold of a mirrored KMS context is updated.
     * @param contextId The mirrored context ID.
     * @param threshold The MPC threshold mirrored from the canonical context.
     */
    event MirrorMpcThreshold(uint256 indexed contextId, uint256 threshold);

    /**
     * @notice Emitted when a mirrored KMS context is marked destroyed.
     * @param contextId The mirrored context ID destroyed.
     * @param sourceChainId The chain ID of the canonical `ProtocolConfig`.
     * @param sourceBlockNumber The block at which the canonical destruction was read.
     * @param sourceProtocolConfig The canonical `ProtocolConfig` address.
     */
    event MirrorKmsContextDestroyed(
        uint256 indexed contextId,
        uint256 indexed sourceChainId,
        uint256 sourceBlockNumber,
        address indexed sourceProtocolConfig
    );

    /// @notice The mirrored context ID is not strictly greater than the current one.
    /// @param contextId The rejected context ID.
    /// @param currentKmsContextId The current mirrored context ID.
    error NonIncreasingKmsContextId(uint256 contextId, uint256 currentKmsContextId);

    /// @notice The canonical source `ProtocolConfig` address is the zero address.
    error InvalidSourceProtocolConfig();

    /**
     * @notice Mirror and immediately activate a canonical KMS context.
     * @dev The `contextId` must be strictly greater than the current mirrored context ID. Gaps are
     *      allowed (e.g. canonical pending contexts that were aborted or never activated); unknown
     *      gap IDs stay invalid until mirrored.
     * @param contextId The canonical context ID to mirror.
     * @param kmsNodeParams The KMS nodes to register, including MPC metadata.
     * @param thresholds The thresholds for the context.
     * @param softwareVersion The KMS software version of the canonical context.
     * @param pcrValues Accepted enclave PCR values of the canonical context.
     * @param source The canonical source provenance (`sourceProtocolConfig` must be non-zero).
     */
    function mirrorKmsContext(
        uint256 contextId,
        KmsNodeParams[] calldata kmsNodeParams,
        KmsThresholds calldata thresholds,
        string calldata softwareVersion,
        PcrValues[] calldata pcrValues,
        MirroredContextSource calldata source
    ) external;

    /**
     * @notice Mirror a canonical public decryption threshold update for an existing mirrored context.
     * @param contextId The mirrored context ID to update.
     * @param threshold The public decryption threshold mirrored from the canonical context.
     */
    function mirrorPublicDecryptionThreshold(uint256 contextId, uint256 threshold) external;

    /**
     * @notice Mirror a canonical user decryption threshold update for an existing mirrored context.
     * @param contextId The mirrored context ID to update.
     * @param threshold The user decryption threshold mirrored from the canonical context.
     */
    function mirrorUserDecryptionThreshold(uint256 contextId, uint256 threshold) external;

    /**
     * @notice Mirror a canonical KMS generation threshold update for an existing mirrored context.
     * @param contextId The mirrored context ID to update.
     * @param threshold The KMS generation threshold mirrored from the canonical context.
     */
    function mirrorKmsGenThreshold(uint256 contextId, uint256 threshold) external;

    /**
     * @notice Mirror a canonical MPC threshold update for an existing mirrored context.
     * @param contextId The mirrored context ID to update.
     * @param threshold The MPC threshold mirrored from the canonical context.
     */
    function mirrorMpcThreshold(uint256 contextId, uint256 threshold) external;

    /**
     * @notice Mirror the destruction of a canonical KMS context.
     * @param contextId The mirrored context ID to destroy.
     * @param source The canonical source provenance (`sourceProtocolConfig` must be non-zero).
     */
    function mirrorKmsContextDestruction(uint256 contextId, MirroredContextSource calldata source) external;

    /**
     * @notice Returns the canonical source provenance recorded for a mirrored context.
     * @param contextId The mirrored context ID.
     * @return source The canonical source provenance.
     */
    function getMirroredContextSource(uint256 contextId) external view returns (MirroredContextSource memory source);
}
