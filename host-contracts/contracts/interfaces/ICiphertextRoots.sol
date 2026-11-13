// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @title Interface for the CiphertextRoots contract.
 * @notice CiphertextRoots is the append-only registry of the ciphertext Merkle roots coprocessor
 * nodes post per (host chain, epoch), as specified by RFC 034. A root becomes the epoch's root once
 * the majority threshold of one coprocessor context posted it.
 * @dev Deployed on the canonical host chain only, next to ProtocolConfig and KMSGeneration. "DAO
 * only" below means the owner of the ACL contract, which is the DAO. `ACLOwnable` enforces it.
 */
interface ICiphertextRoots {
    /**
     * @notice An entry of a host chain's epoch schedule: epochs from `fromEpoch` on are
     * `epochLength` host chain blocks each, until the next entry.
     * @param fromEpoch The first epoch with this epoch length.
     * @param epochLength The number of host chain blocks per epoch.
     */
    struct EpochScheduleEntry {
        uint256 fromEpoch;
        uint256 epochLength;
    }

    /**
     * @notice A host chain's epoch schedule.
     * @param startBlock The first host chain block of epoch 0. Blocks before it belong to no epoch.
     * @param entries The entries in `fromEpoch` order; the first has `fromEpoch` 0.
     */
    struct EpochSchedule {
        uint256 startBlock;
        EpochScheduleEntry[] entries;
    }

    // -----------------------------------------------------------------------------------------
    // Events
    // -----------------------------------------------------------------------------------------

    /**
     * @notice Emitted when the DAO sets a host chain's epoch schedule.
     * @param chainId The host chain ID.
     * @param startBlock The first host chain block of epoch 0.
     * @param epochLength The number of host chain blocks per epoch.
     */
    event EpochScheduleSet(uint64 indexed chainId, uint256 startBlock, uint256 epochLength);

    /**
     * @notice Emitted when the DAO sets a host chain's epoch length from an epoch on.
     * @param chainId The host chain ID.
     * @param fromEpoch The first epoch with this epoch length.
     * @param epochLength The number of host chain blocks per epoch.
     */
    event EpochLengthSet(uint64 indexed chainId, uint256 fromEpoch, uint256 epochLength);

    /**
     * @notice Emitted when the DAO removes an entry of a host chain's epoch schedule.
     * @param chainId The host chain ID.
     * @param fromEpoch The first epoch of the removed entry.
     */
    event EpochLengthRemoved(uint64 indexed chainId, uint256 fromEpoch);

    /**
     * @notice Emitted on every accepted post.
     * @param chainId The host chain ID.
     * @param epoch The epoch number.
     * @param sender The coprocessor tx sender that posted.
     * @param root The posted root.
     */
    event RootPosted(uint64 indexed chainId, uint256 indexed epoch, address indexed sender, bytes32 root);

    /**
     * @notice Emitted once per (host chain, epoch), when a root reaches a context's majority threshold.
     * @param chainId The host chain ID.
     * @param epoch The epoch number.
     * @param root The recorded root.
     * @param coprocessorContextId The coprocessor context whose majority threshold the root reached,
     *        the current context when the root reaches the thresholds of both the current and the
     *        previous context.
     */
    event EpochRootRecorded(uint64 indexed chainId, uint256 indexed epoch, bytes32 root, uint256 coprocessorContextId);

    // -----------------------------------------------------------------------------------------
    // Errors
    // -----------------------------------------------------------------------------------------

    /// @notice The host chain ID is zero.
    error InvalidChainId();

    /// @notice The epoch length is zero.
    error InvalidEpochLength();

    /// @notice The host chain already has an epoch schedule.
    /// @param chainId The host chain ID.
    error EpochScheduleAlreadySet(uint64 chainId);

    /// @notice The host chain has no epoch schedule.
    /// @param chainId The host chain ID.
    error NoEpochSchedule(uint64 chainId);

    /// @notice The host chain block is before the schedule's start block, so it belongs to no epoch.
    /// @param chainId The host chain ID.
    /// @param blockNumber The host chain block number.
    error BlockBeforeStartBlock(uint64 chainId, uint256 blockNumber);

    /// @notice The entry's first epoch is not after the highest epoch with a recorded root or,
    ///         when setting one, not after the first epoch of the schedule's last entry.
    /// @param chainId The host chain ID.
    /// @param fromEpoch The rejected first epoch.
    /// @param minFromEpoch The lowest accepted first epoch.
    error FromEpochTooLow(uint64 chainId, uint256 fromEpoch, uint256 minFromEpoch);

    /// @notice No entry starts at this epoch, or it is the first entry, which cannot be removed.
    /// @param chainId The host chain ID.
    /// @param fromEpoch The first epoch given.
    error EpochLengthNotFound(uint64 chainId, uint256 fromEpoch);

    /// @notice The sender is not a tx sender of the current coprocessor context nor of the previous one.
    /// @param sender The rejected sender.
    error NotCoprocessorTxSender(address sender);

    /// @notice The sender already posted for this (host chain, epoch).
    /// @param chainId The host chain ID.
    /// @param epoch The epoch number.
    /// @param sender The sender.
    error RootAlreadyPosted(uint64 chainId, uint256 epoch, address sender);

    /// @notice The (host chain, epoch) already has a recorded root.
    /// @param chainId The host chain ID.
    /// @param epoch The epoch number.
    error EpochRootAlreadyRecorded(uint64 chainId, uint256 epoch);

    /// @notice No posted root of the (host chain, epoch) reaches a majority threshold.
    /// @param chainId The host chain ID.
    /// @param epoch The epoch number.
    error NoRootReachesThreshold(uint64 chainId, uint256 epoch);

    /// @notice The (host chain, epoch) has no recorded root.
    /// @param chainId The host chain ID.
    /// @param epoch The epoch number.
    error NoEpochRoot(uint64 chainId, uint256 epoch);

    /// @notice No epoch of the host chain has a recorded root.
    /// @param chainId The host chain ID.
    error NoEpochRootRecorded(uint64 chainId);

    // -----------------------------------------------------------------------------------------
    // View functions
    // -----------------------------------------------------------------------------------------

    /**
     * @notice Returns a host chain's epoch schedule.
     * @dev Reverts with `NoEpochSchedule` if the host chain has no schedule.
     * @param chainId The host chain ID.
     * @return The epoch schedule.
     */
    function getEpochSchedule(uint64 chainId) external view returns (EpochSchedule memory);

    /**
     * @notice Returns the epoch of a host chain block.
     * @dev The schedule can change; this returns the epoch per the schedule at the time of the
     *      call. Reverts with `NoEpochSchedule` if the host chain has no schedule, and with
     *      `BlockBeforeStartBlock` if the block is before the schedule's start block.
     * @param chainId The host chain ID.
     * @param blockNumber The host chain block number.
     * @return The epoch number.
     */
    function getEpochOfBlock(uint64 chainId, uint256 blockNumber) external view returns (uint256);

    /**
     * @notice Returns the host chain blocks of an epoch.
     * @dev The schedule can change; this returns the blocks per the schedule at the time of the
     *      call. Reverts with `NoEpochSchedule` if the host chain has no schedule.
     * @param chainId The host chain ID.
     * @param epoch The epoch number.
     * @return startBlock The epoch's first host chain block.
     * @return endBlock The epoch's last host chain block.
     */
    function getEpochBlocks(uint64 chainId, uint256 epoch) external view returns (uint256 startBlock, uint256 endBlock);

    /**
     * @notice Returns the highest epoch with a recorded root for a host chain.
     * @dev Reverts with `NoEpochSchedule` if the host chain has no schedule, and with
     *      `NoEpochRootRecorded` if none of its epochs has a recorded root.
     * @param chainId The host chain ID.
     * @return The epoch number.
     */
    function getHighestRecordedEpoch(uint64 chainId) external view returns (uint256);

    /**
     * @notice Returns the recorded root of a (host chain, epoch).
     * @dev Reverts with `NoEpochSchedule` if the host chain has no schedule, and with `NoEpochRoot`
     *      if the epoch has no recorded root.
     * @param chainId The host chain ID.
     * @param epoch The epoch number.
     * @return The recorded root.
     */
    function getEpochRoot(uint64 chainId, uint256 epoch) external view returns (bytes32);

    /**
     * @notice Returns the distinct roots posted for a (host chain, epoch), in first-post order.
     * @dev Empty if none. Reverts with `NoEpochSchedule` if the host chain has no schedule.
     * @param chainId The host chain ID.
     * @param epoch The epoch number.
     * @return The posted roots.
     */
    function getPostedRoots(uint64 chainId, uint256 epoch) external view returns (bytes32[] memory);

    /**
     * @notice Returns the senders that posted a given root for a (host chain, epoch), in post order.
     * @dev Any posted root, recorded or not. Empty if none. Reverts with `NoEpochSchedule` if the host
     *      chain has no schedule.
     * @param chainId The host chain ID.
     * @param epoch The epoch number.
     * @param root The root.
     * @return The senders.
     */
    function getRootSenders(uint64 chainId, uint256 epoch, bytes32 root) external view returns (address[] memory);

    /**
     * @notice Returns the contract version.
     * @return The version string.
     */
    function getVersion() external pure returns (string memory);

    // -----------------------------------------------------------------------------------------
    // State-changing functions
    // -----------------------------------------------------------------------------------------

    /**
     * @notice Sets a host chain's epoch schedule. DAO only, once per host chain.
     * @dev Reverts with `InvalidChainId` for chain ID 0, `InvalidEpochLength` for epoch length 0 and
     *      `EpochScheduleAlreadySet` if the host chain has a schedule.
     * @param chainId The host chain ID.
     * @param startBlock The first host chain block of epoch 0.
     * @param epochLength The number of host chain blocks per epoch, from epoch 0.
     */
    function setEpochSchedule(uint64 chainId, uint256 startBlock, uint256 epochLength) external;

    /**
     * @notice Sets a host chain's epoch length from an epoch on. DAO only.
     * @dev Adds an entry to the schedule. `fromEpoch` must be after the first epoch of the
     *      schedule's last entry and after the highest epoch with a recorded root. The DAO picks
     *      it a few epochs ahead of the host chain. Reverts with
     *      `NoEpochSchedule` if the host chain has no schedule, `InvalidEpochLength` for epoch length 0
     *      and `FromEpochTooLow` if `fromEpoch` breaks the rule above.
     * @param chainId The host chain ID.
     * @param fromEpoch The first epoch with this epoch length.
     * @param epochLength The number of host chain blocks per epoch.
     */
    function setEpochLength(uint64 chainId, uint256 fromEpoch, uint256 epochLength) external;

    /**
     * @notice Removes the entry of a host chain's epoch schedule that starts at `fromEpoch`. DAO
     *      only.
     * @dev Allowed while no epoch at or after `fromEpoch` has a recorded root. The first entry, set
     *      by `setEpochSchedule`, cannot be removed. The DAO removes only entries the host chain has
     *      not reached.
     *      Reverts with `NoEpochSchedule` if the host chain has no schedule, `EpochLengthNotFound` if
     *      `fromEpoch` is 0 or no entry starts at it, and `FromEpochTooLow` if an epoch at or after
     *      it has a recorded root.
     * @param chainId The host chain ID.
     * @param fromEpoch The first epoch of the entry to remove.
     */
    function removeEpochLength(uint64 chainId, uint256 fromEpoch) external;

    /**
     * @notice Posts the sender's root for a (host chain, epoch).
     * @dev The sender must be a tx sender of the current or, while still valid, the previous
     *      coprocessor context, and may post once per (host chain, epoch), for any epoch, however
     *      late the post.
     *      Records the first posted root that reaches a context's majority threshold, if none is yet.
     *      Reverts with `NoEpochSchedule` if the host chain has no schedule, `RootAlreadyPosted` if
     *      the sender posted for the epoch and `NotCoprocessorTxSender` if the sender is in neither
     *      context.
     * @param chainId The host chain ID.
     * @param epoch The epoch number.
     * @param root The root of the sender's tree for the epoch.
     */
    function postRoot(uint64 chainId, uint256 epoch, bytes32 root) external;

    /**
     * @notice Records the first posted root that reaches a majority threshold.
     * @dev Permissionless, anyone can call it.
     *      Reverts with `NoEpochSchedule` if the host chain has no schedule, `EpochRootAlreadyRecorded`
     *      if the epoch has a recorded root and `NoRootReachesThreshold` if no posted root reaches a
     *      threshold.
     * @param chainId The host chain ID.
     * @param epoch The epoch number.
     */
    function recordEpochRoot(uint64 chainId, uint256 epoch) external;
}
