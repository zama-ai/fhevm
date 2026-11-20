// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Coprocessor, ChainUpgradeWindow} from "../shared/Structs.sol";
import {ICoprocessorConfigBase} from "./ICoprocessorConfigBase.sol";

/**
 * @title Interface for the CoprocessorConfig contract.
 * @notice CoprocessorConfig manages the coprocessor set, majority threshold configuration, context
 * lifecycle, and upgrade proposals on the canonical host chain.
 * @dev Ethereum is the canonical host and source of truth. Every other host chain runs
 * `CoprocessorConfigReplica` (see `ICoprocessorConfigReplica`).
 */
interface ICoprocessorConfig is ICoprocessorConfigBase {
    // -----------------------------------------------------------------------------------------
    // Events
    // -----------------------------------------------------------------------------------------

    /**
     * @notice Emitted when a new coprocessor context is defined. It becomes current immediately.
     * @dev Carries the whole set and threshold, so readers can rebuild the registry from logs alone.
     * @param contextId The new coprocessor context ID.
     * @param previousContextId The coprocessor context it replaces, or 0 for the context defined at initialization.
     * @param coprocessors The coprocessors of the new context.
     * @param majorityThreshold The majority threshold of the new context.
     */
    event NewCoprocessorContext(
        uint256 indexed contextId,
        uint256 indexed previousContextId,
        Coprocessor[] coprocessors,
        uint256 majorityThreshold
    );

    /**
     * @notice Emitted when a coprocessor upgrade is proposed. This event drives the
     *         coprocessor software upgrade.
     * @param proposalId The new coprocessor upgrade proposal ID.
     * @param softwareVersion The coprocessor software version for the proposal.
     * @param chainUpgradeWindows The per-host-chain replay windows for the upgrade.
     * @param gwStartBlock The Gateway block at which GCS's gateway-listener resumes from.
     */
    event CoprocessorUpgradeProposed(
        uint256 indexed proposalId,
        string softwareVersion,
        ChainUpgradeWindow[] chainUpgradeWindows,
        uint64 gwStartBlock
    );

    // -----------------------------------------------------------------------------------------
    // Errors
    // -----------------------------------------------------------------------------------------

    /// @notice The coprocessor `softwareVersion` argument is the empty string.
    error EmptySoftwareVersion();

    /// @notice The `chainUpgradeWindows` array argument is empty.
    error EmptyChainUpgradeWindows();

    /// @notice A chain entry has a zero `chainId`.
    error ZeroChainId();

    /// @notice The same `chainId` appears more than once in the `chainUpgradeWindows` array.
    /// @param chainId The duplicated chain id.
    error DuplicateChainId(uint64 chainId);

    /// @notice The block window for a chain entry is invalid (`startBlock > endBlock`).
    /// @param chainId The chain id whose window is invalid.
    /// @param startBlock The provided start block.
    /// @param endBlock The provided end block.
    error InvalidBlockWindow(uint64 chainId, uint64 startBlock, uint64 endBlock);

    /// @notice The `gwStartBlock` argument is zero.
    error ZeroGwStartBlock();

    // -----------------------------------------------------------------------------------------
    // State-changing functions
    // -----------------------------------------------------------------------------------------

    /**
     * @notice Define the next coprocessor context and make it current.
     * @dev Owner only. The new ID is the current one plus one. The previous context stays valid
     *      until destroyed.
     * @param coprocessors The coprocessors of the new context.
     * @param majorityThreshold More than half the number of coprocessors, and at most that number.
     */
    function defineNewCoprocessorContext(Coprocessor[] calldata coprocessors, uint256 majorityThreshold) external;

    /**
     * @notice Destroy a past coprocessor context. Its set and threshold stay readable.
     * @dev Owner only. Reverts for the current context, an unknown ID or an already destroyed one.
     * @param coprocessorContextId The coprocessor context ID to destroy.
     */
    function destroyCoprocessorContext(uint256 coprocessorContextId) external;

    /**
     * @notice Propose a coprocessor upgrade. Assigns the next proposal ID and emits
     *         `CoprocessorUpgradeProposed`. The lifecycle of the proposal (dry-run, consensus,
     *         cutover, failure) is driven entirely off-chain.
     * @dev Owner only. A retry after a failed upgrade is a new proposal, with a new ID.
     * @param softwareVersion The coprocessor software version.
     * @param chainUpgradeWindows The per-host-chain replay windows.
     * @param gwStartBlock The Gateway block to resume from.
     */
    function proposeCoprocessorUpgrade(
        string calldata softwareVersion,
        ChainUpgradeWindow[] calldata chainUpgradeWindows,
        uint64 gwStartBlock
    ) external;
}
