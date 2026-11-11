// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {CoprocessorConfigBase} from "./CoprocessorConfigBase.sol";
import {ICoprocessorConfigReplica} from "./interfaces/ICoprocessorConfigReplica.sol";
import {ICoprocessorConfigBase} from "./interfaces/ICoprocessorConfigBase.sol";
import {Coprocessor} from "./shared/Structs.sol";
import {COPROCESSOR_CONTEXT_COUNTER_BASE} from "./shared/Constants.sol";
import {UUPSUpgradeableEmptyProxy} from "./shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable} from "./shared/ACLOwnable.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

/**
 * @title CoprocessorConfigReplica
 * @notice Read-replica of the canonical CoprocessorConfig on non-canonical host chains.
 * @dev Ethereum is the canonical host and the single source of truth.
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract CoprocessorConfigReplica is
    ICoprocessorConfigReplica,
    CoprocessorConfigBase,
    UUPSUpgradeableEmptyProxy,
    ACLOwnable
{
    // -----------------------------------------------------------------------------------------
    // Contract information
    // -----------------------------------------------------------------------------------------

    string private constant CONTRACT_NAME = "CoprocessorConfigReplica";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    uint64 private constant REINITIALIZER_VERSION = 2;

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
     * @notice Canonical mirror initializer: seeds a non-canonical host with Ethereum's current
     *         coprocessor context.
     * @dev Preserves the canonical context ID. Later canonical contexts use `mirrorCoprocessorContext`.
     * @param canonicalContextId The current Ethereum coprocessor context ID to preserve.
     * @param canonicalCoprocessors The coprocessors of that context.
     * @param canonicalMajorityThreshold The majority threshold of that context.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromCanonical(
        uint256 canonicalContextId,
        Coprocessor[] calldata canonicalCoprocessors,
        uint256 canonicalMajorityThreshold
    ) public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        _mirrorCoprocessorContext(canonicalContextId, canonicalCoprocessors, canonicalMajorityThreshold);
    }

    // -----------------------------------------------------------------------------------------
    // Mirror functions
    // -----------------------------------------------------------------------------------------

    /// @inheritdoc ICoprocessorConfigReplica
    function mirrorCoprocessorContext(
        uint256 contextId,
        Coprocessor[] calldata coprocessors,
        uint256 majorityThreshold
    ) external virtual onlyACLOwner {
        _mirrorCoprocessorContext(contextId, coprocessors, majorityThreshold);
    }

    /// @inheritdoc ICoprocessorConfigReplica
    function mirrorCoprocessorContextDestruction(uint256 coprocessorContextId) external virtual onlyACLOwner {
        _destroyCoprocessorContext(coprocessorContextId);
    }

    // -----------------------------------------------------------------------------------------
    // Read functions
    // -----------------------------------------------------------------------------------------

    /// @inheritdoc ICoprocessorConfigBase
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
        uint256 currentContextId = _getCoprocessorConfigStorage().currentCoprocessorContextId;
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
