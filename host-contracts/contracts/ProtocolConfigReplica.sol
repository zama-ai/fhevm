// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ProtocolConfigBase} from "./ProtocolConfigBase.sol";
import {IProtocolConfigReplica} from "./interfaces/IProtocolConfigReplica.sol";
import {IProtocolConfigBase} from "./interfaces/IProtocolConfigBase.sol";
import {Coprocessor, KmsThresholds, KmsNodeParams, PcrValues} from "./shared/Structs.sol";
import {COPROCESSOR_CONTEXT_COUNTER_BASE, EPOCH_COUNTER_BASE, KMS_CONTEXT_COUNTER_BASE} from "./shared/Constants.sol";
import {UUPSUpgradeableEmptyProxy} from "./shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable} from "./shared/ACLOwnable.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

/**
 * @title ProtocolConfigReplica
 * @notice Read-replica of the canonical ProtocolConfig on non-canonical host chains.
 * @dev Ethereum is the canonical host and the single source of truth.
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract ProtocolConfigReplica is IProtocolConfigReplica, ProtocolConfigBase, UUPSUpgradeableEmptyProxy, ACLOwnable {
    // -----------------------------------------------------------------------------------------
    // Contract information
    // -----------------------------------------------------------------------------------------

    string private constant CONTRACT_NAME = "ProtocolConfigReplica";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 3;
    uint256 private constant PATCH_VERSION = 0;

    /// @dev Shared between `initializeFromCanonical` and `reinitializeV3`.
    uint64 private constant REINITIALIZER_VERSION = 4;

    // -----------------------------------------------------------------------------------------
    // Constructor
    // -----------------------------------------------------------------------------------------

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    // -----------------------------------------------------------------------------------------
    // Initialization
    // -----------------------------------------------------------------------------------------

    /**
     * @notice Canonical mirror initializer: seeds a non-canonical host from Ethereum's active state.
     * @dev Preserves the canonical KMS context ID, KMS epoch ID and coprocessor context ID. This is the
     *      bootstrap path. Later canonical updates use `mirrorKmsContextAndEpoch` and `mirrorKmsEpoch` for
     *      the KMS, and `mirrorCoprocessorContext` and `mirrorCoprocessorContextDestruction` for coprocessors.
     * @param canonicalKmsContextId The active Ethereum KMS context ID to preserve.
     * @param canonicalKmsEpochId The active Ethereum KMS epoch ID to preserve.
     * @param canonicalKmsNodeParams The active Ethereum KMS node set, including MPC metadata.
     * @param canonicalKmsThresholds The active Ethereum KMS thresholds.
     * @param canonicalCoprocessorContextId The current Ethereum coprocessor context ID to preserve.
     * @param canonicalCoprocessors The coprocessors of that context.
     * @param canonicalCoprocessorMajorityThreshold The majority threshold of that context.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromCanonical(
        uint256 canonicalKmsContextId,
        uint256 canonicalKmsEpochId,
        KmsNodeParams[] calldata canonicalKmsNodeParams,
        KmsThresholds calldata canonicalKmsThresholds,
        uint256 canonicalCoprocessorContextId,
        Coprocessor[] calldata canonicalCoprocessors,
        uint256 canonicalCoprocessorMajorityThreshold
    ) public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        _mirrorCoprocessorContext(
            canonicalCoprocessorContextId,
            canonicalCoprocessors,
            canonicalCoprocessorMajorityThreshold
        );

        if (canonicalKmsContextId < KMS_CONTEXT_COUNTER_BASE + 1) {
            revert InvalidKmsContext(canonicalKmsContextId);
        }
        if (canonicalKmsEpochId < EPOCH_COUNTER_BASE + 1) {
            revert InvalidKmsEpoch(canonicalKmsEpochId);
        }

        _storeAndActivateKmsContextAndEpoch(
            canonicalKmsContextId,
            canonicalKmsEpochId,
            canonicalKmsNodeParams,
            canonicalKmsThresholds
        );
    }

    /**
     * @notice Re-initializes a replica proxy upgraded from `ProtocolConfig` and mirrors the
     *         current canonical coprocessor context.
     * @dev See `REINITIALIZER_VERSION`.
     * @param canonicalCoprocessorContextId The current Ethereum coprocessor context ID to preserve.
     * @param canonicalCoprocessors The coprocessors of that context.
     * @param canonicalCoprocessorMajorityThreshold The majority threshold of that context.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV3(
        uint256 canonicalCoprocessorContextId,
        Coprocessor[] calldata canonicalCoprocessors,
        uint256 canonicalCoprocessorMajorityThreshold
    ) public virtual onlyACLOwner reinitializer(REINITIALIZER_VERSION) {
        _mirrorCoprocessorContext(
            canonicalCoprocessorContextId,
            canonicalCoprocessors,
            canonicalCoprocessorMajorityThreshold
        );
    }

    // -----------------------------------------------------------------------------------------
    // Mirror functions
    // -----------------------------------------------------------------------------------------

    /// @inheritdoc IProtocolConfigReplica
    function mirrorKmsContextAndEpoch(
        uint256 contextId,
        uint256 epochId,
        KmsNodeParams[] calldata kmsNodeParams,
        KmsThresholds calldata thresholds,
        string calldata softwareVersion,
        PcrValues[] calldata pcrValues
    ) external virtual onlyACLOwner {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        uint256 latestActiveKmsContextId = $.latestActiveKmsContextId;
        if (contextId <= latestActiveKmsContextId) {
            revert NonIncreasingKmsContextId(contextId, latestActiveKmsContextId);
        }
        uint256 currentEpochId = $.epochCounter;
        if (epochId <= currentEpochId) {
            revert NonIncreasingEpochId(epochId, currentEpochId);
        }

        _storeAndActivateKmsContextAndEpoch(contextId, epochId, kmsNodeParams, thresholds);
        emit MirrorKmsContextAndEpoch(contextId, epochId, kmsNodeParams, thresholds, softwareVersion, pcrValues);
    }

    /// @inheritdoc IProtocolConfigReplica
    function mirrorKmsEpoch(uint256 contextId, uint256 epochId) external virtual onlyACLOwner {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        if (contextId != $.latestActiveKmsContextId || !_isLiveKmsContext(contextId)) {
            revert InvalidKmsContext(contextId);
        }
        uint256 currentEpochId = $.epochCounter;
        if (epochId <= currentEpochId) {
            revert NonIncreasingEpochId(epochId, currentEpochId);
        }

        $.epochCounter = epochId;
        _activateEpoch(epochId, contextId);
        emit MirrorKmsEpoch(contextId, epochId);
    }

    /// @inheritdoc IProtocolConfigReplica
    function mirrorCoprocessorContext(
        uint256 contextId,
        Coprocessor[] calldata coprocessors,
        uint256 majorityThreshold
    ) external virtual onlyACLOwner {
        _mirrorCoprocessorContext(contextId, coprocessors, majorityThreshold);
    }

    /// @inheritdoc IProtocolConfigReplica
    function mirrorCoprocessorContextDestruction(uint256 coprocessorContextId) external virtual onlyACLOwner {
        _destroyCoprocessorContext(coprocessorContextId);
    }

    /// @inheritdoc IProtocolConfigBase
    function getVersion() external pure virtual returns (string memory) {
        return
            string(
                abi.encodePacked(
                    CONTRACT_NAME,
                    " v",
                    Strings.toString(MAJOR_VERSION),
                    ".",
                    Strings.toString(MINOR_VERSION),
                    ".",
                    Strings.toString(PATCH_VERSION)
                )
            );
    }

    // -----------------------------------------------------------------------------------------
    // Internal
    // -----------------------------------------------------------------------------------------

    /**
     * @dev Mirrors a canonical coprocessor context and makes it current. The ID must carry the
     *      coprocessor context type and be greater than the current one on this replica.
     */
    function _mirrorCoprocessorContext(
        uint256 contextId,
        Coprocessor[] calldata coprocessors,
        uint256 majorityThreshold
    ) internal virtual {
        if (contextId <= COPROCESSOR_CONTEXT_COUNTER_BASE) {
            revert InvalidCoprocessorContext(contextId);
        }
        uint256 currentContextId = _getProtocolConfigStorage().currentCoprocessorContextId;
        if (contextId <= currentContextId) {
            revert NonIncreasingCoprocessorContextId(contextId, currentContextId);
        }
        _storeCoprocessorContext(contextId, coprocessors, majorityThreshold);
        emit MirrorCoprocessorContext(contextId, coprocessors, majorityThreshold);
    }

    /**
     * @dev Authorization for UUPS upgrades.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyACLOwner {}
}
