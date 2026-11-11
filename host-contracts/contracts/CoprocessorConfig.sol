// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {CoprocessorConfigBase} from "./CoprocessorConfigBase.sol";
import {ICoprocessorConfig} from "./interfaces/ICoprocessorConfig.sol";
import {ICoprocessorConfigBase} from "./interfaces/ICoprocessorConfigBase.sol";
import {Coprocessor, ChainUpgradeWindow} from "./shared/Structs.sol";
import {COPROCESSOR_CONTEXT_COUNTER_BASE} from "./shared/Constants.sol";
import {UUPSUpgradeableEmptyProxy} from "./shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable} from "./shared/ACLOwnable.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

/**
 * @title CoprocessorConfig
 * @notice Manages coprocessor sets, majority thresholds, context lifecycle, and upgrade proposals
 *         on the canonical host chain.
 * @dev Ethereum is the canonical host and the single source of truth: only this contract assigns
 *      coprocessor context IDs. Every other host chain runs `CoprocessorConfigReplica` instead.
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract CoprocessorConfig is ICoprocessorConfig, CoprocessorConfigBase, UUPSUpgradeableEmptyProxy, ACLOwnable {
    // -----------------------------------------------------------------------------------------
    // Contract information
    // -----------------------------------------------------------------------------------------

    string private constant CONTRACT_NAME = "CoprocessorConfig";
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
     * @notice Fresh deploy initializer: defines the first coprocessor context.
     * @dev The first context takes the ID `COPROCESSOR_CONTEXT_COUNTER_BASE + 1` and replaces none.
     * @param initialCoprocessors The coprocessors of the first context.
     * @param initialMajorityThreshold The majority threshold of the first context.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy(
        Coprocessor[] calldata initialCoprocessors,
        uint256 initialMajorityThreshold
    ) public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        _defineCoprocessorContext(
            COPROCESSOR_CONTEXT_COUNTER_BASE + 1,
            0,
            initialCoprocessors,
            initialMajorityThreshold
        );
    }

    // -----------------------------------------------------------------------------------------
    // State-changing functions
    // -----------------------------------------------------------------------------------------

    /// @inheritdoc ICoprocessorConfig
    function defineNewCoprocessorContext(
        Coprocessor[] calldata coprocessors,
        uint256 majorityThreshold
    ) external virtual onlyACLOwner {
        uint256 currentContextId = _getCoprocessorConfigStorage().currentCoprocessorContextId;
        _defineCoprocessorContext(currentContextId + 1, currentContextId, coprocessors, majorityThreshold);
    }

    /// @inheritdoc ICoprocessorConfig
    function destroyCoprocessorContext(uint256 coprocessorContextId) external virtual onlyACLOwner {
        _destroyCoprocessorContext(coprocessorContextId);
    }

    /// @inheritdoc ICoprocessorConfig
    function proposeCoprocessorUpgrade(
        uint256 proposalId,
        string calldata softwareVersion,
        ChainUpgradeWindow[] calldata chainUpgradeWindows,
        uint64 gwStartBlock
    ) external virtual onlyACLOwner {
        if (proposalId == 0) {
            revert InvalidProposalId();
        }
        if (bytes(softwareVersion).length == 0) {
            revert EmptySoftwareVersion();
        }
        if (chainUpgradeWindows.length == 0) {
            revert EmptyChainUpgradeWindows();
        }
        if (gwStartBlock == 0) {
            revert ZeroGwStartBlock();
        }

        for (uint256 i = 0; i < chainUpgradeWindows.length; i++) {
            ChainUpgradeWindow calldata cw = chainUpgradeWindows[i];
            if (cw.chainId == 0) {
                revert ZeroChainId();
            }
            if (cw.startBlock > cw.endBlock) {
                revert InvalidBlockWindow(cw.chainId, cw.startBlock, cw.endBlock);
            }
            for (uint256 j = 0; j < i; j++) {
                if (chainUpgradeWindows[j].chainId == cw.chainId) {
                    revert DuplicateChainId(cw.chainId);
                }
            }
        }

        emit CoprocessorUpgradeProposed(proposalId, softwareVersion, chainUpgradeWindows, gwStartBlock);
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
     * @dev Stores and activates a coprocessor context, then emits `NewCoprocessorContext`.
     */
    function _defineCoprocessorContext(
        uint256 contextId,
        uint256 previousContextId,
        Coprocessor[] calldata coprocessors,
        uint256 majorityThreshold
    ) internal virtual {
        _storeCoprocessorContext(contextId, coprocessors, majorityThreshold);
        emit NewCoprocessorContext(contextId, previousContextId, coprocessors, majorityThreshold);
    }

    /**
     * @dev Authorization for UUPS upgrades.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyACLOwner {}
}
