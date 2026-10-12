// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ProtocolConfigBase} from "./ProtocolConfigBase.sol";
import {IProtocolConfigReplica} from "./interfaces/IProtocolConfigReplica.sol";
import {IProtocolConfigBase} from "./interfaces/IProtocolConfigBase.sol";
import {KmsThresholds, KmsNodeParams, PcrValues} from "./shared/Structs.sol";
import {EPOCH_COUNTER_BASE, KMS_CONTEXT_COUNTER_BASE} from "./shared/Constants.sol";
import {UUPSUpgradeableEmptyProxy} from "./shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable} from "./shared/ACLOwnable.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

/**
 * @title ProtocolConfigReplica
 * @notice Read-replica of the canonical ProtocolConfig on the non-canonical host chains (e.g. Polygon).
 * @dev Ethereum is the canonical host and the single source of truth.
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract ProtocolConfigReplica is IProtocolConfigReplica, ProtocolConfigBase, UUPSUpgradeableEmptyProxy, ACLOwnable {
    // -----------------------------------------------------------------------------------------
    // Contract information
    // -----------------------------------------------------------------------------------------

    string private constant CONTRACT_NAME = "ProtocolConfigReplica";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    /// @dev Shared between `initializeFromCanonical` and `reinitializeV4`. Existing replica proxies
    ///      run `ProtocolConfig` at initialized version 3 (v0.14.x release) or 4 (later builds).
    ///      Version 5 is above both, so `reinitializeV4` works from either.
    uint64 private constant REINITIALIZER_VERSION = 5;

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
     * @dev Preserves both the canonical context ID and canonical epoch ID instead of allocating a
     *      fresh local epoch. This is the bootstrap path; later canonical updates use
     *      `mirrorKmsContextAndEpoch` and `mirrorKmsEpoch`.
     * @param canonicalContextId The active Ethereum KMS context ID to preserve.
     * @param canonicalEpochId The active Ethereum epoch ID to preserve.
     * @param canonicalKmsNodeParams The active Ethereum KMS node set, including MPC metadata.
     * @param canonicalThresholds The active Ethereum thresholds.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromCanonical(
        uint256 canonicalContextId,
        uint256 canonicalEpochId,
        KmsNodeParams[] calldata canonicalKmsNodeParams,
        KmsThresholds calldata canonicalThresholds
    ) public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        if (canonicalContextId < KMS_CONTEXT_COUNTER_BASE + 1) {
            revert InvalidKmsContext(canonicalContextId);
        }
        if (canonicalEpochId < EPOCH_COUNTER_BASE + 1) {
            revert InvalidKmsEpoch(canonicalEpochId);
        }

        _storeAndActivateKmsContextAndEpoch(
            canonicalContextId,
            canonicalEpochId,
            canonicalKmsNodeParams,
            canonicalThresholds
        );
    }

    /**
     * @notice Re-initializes a replica proxy upgraded from `ProtocolConfig`.
     * @dev See `REINITIALIZER_VERSION`.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV4() public virtual reinitializer(REINITIALIZER_VERSION) {}

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
     * @dev Authorization for UUPS upgrades.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyACLOwner {}
}
