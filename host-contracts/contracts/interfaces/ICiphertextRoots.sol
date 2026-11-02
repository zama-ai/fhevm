// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @title Interface for the CiphertextRoots contract.
 * @notice CiphertextRoots is the append-only registry of the ciphertext Merkle roots coprocessor
 * nodes post per (host chain, epoch), as specified by RFC 034. A root becomes the epoch's root once
 * the majority threshold of one coprocessor context, read from ProtocolConfig, posted it.
 * @dev Deployed on the canonical host chain only, next to ProtocolConfig and KMSGeneration.
 */
interface ICiphertextRoots {
    /**
     * @notice An epoch length of a host chain's schedule: epochs from `fromEpoch` on are
     * `epochLength` host chain blocks each, until the next epoch length.
     * @param fromEpoch The first epoch with this length.
     * @param epochLength The number of host chain blocks per epoch.
     */
    struct EpochLength {
        uint256 fromEpoch;
        uint256 epochLength;
    }

    /**
     * @notice A host chain's epoch schedule.
     * @param startBlock The first host chain block of epoch 0. Blocks before it belong to no epoch.
     * @param epochLengths The epoch lengths in `fromEpoch` order, the first one from epoch 0.
     */
    struct EpochSchedule {
        uint256 startBlock;
        EpochLength[] epochLengths;
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
     * @param fromEpoch The first epoch with this length.
     * @param epochLength The number of host chain blocks per epoch.
     */
    event EpochLengthSet(uint64 indexed chainId, uint256 fromEpoch, uint256 epochLength);

    /**
     * @notice Emitted when the DAO removes a host chain's epoch length.
     * @param chainId The host chain ID.
     * @param fromEpoch The first epoch the removed length applied to.
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
     * @param coprocessorContextId The coprocessor context whose threshold the root reached, the
     *        current one if both the current and the previous context's are reached.
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

    /// @notice The first epoch is not after both the schedule's last one and the highest epoch
    ///         with a recorded root.
    /// @param chainId The host chain ID.
    /// @param fromEpoch The rejected first epoch.
    /// @param minFromEpoch The lowest accepted first epoch.
    error FromEpochTooLow(uint64 chainId, uint256 fromEpoch, uint256 minFromEpoch);

    /// @notice The schedule has no removable epoch length from this epoch. The first one never is.
    /// @param chainId The host chain ID.
    /// @param fromEpoch The first epoch given.
    error EpochLengthNotFound(uint64 chainId, uint256 fromEpoch);

    /// @notice An epoch at or after `fromEpoch` already has a recorded root, so the epoch length stays.
    /// @param chainId The host chain ID.
    /// @param fromEpoch The first epoch of the epoch length.
    error EpochLengthInUse(uint64 chainId, uint256 fromEpoch);

    /// @notice `ProtocolConfig` reports no current coprocessor context.
    error NoCoprocessorContext();

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

    // -----------------------------------------------------------------------------------------
    // View functions
    // -----------------------------------------------------------------------------------------

    /**
     * @notice Returns a host chain's epoch schedule.
     * @param chainId The host chain ID.
     * @return The epoch schedule.
     */
    function getEpochSchedule(uint64 chainId) external view returns (EpochSchedule memory);

    /**
     * @notice Returns the recorded root of a (host chain, epoch), if any.
     * @param chainId The host chain ID.
     * @param epoch The epoch number.
     * @return root The recorded root, zero if none.
     * @return recorded Whether a root is recorded.
     */
    function getEpochRoot(uint64 chainId, uint256 epoch) external view returns (bytes32 root, bool recorded);

    /**
     * @notice Returns the distinct roots posted for a (host chain, epoch), in first-post order.
     * @param chainId The host chain ID.
     * @param epoch The epoch number.
     * @return The posted roots.
     */
    function getPostedRoots(uint64 chainId, uint256 epoch) external view returns (bytes32[] memory);

    /**
     * @notice Returns the senders that posted a given root for a (host chain, epoch), in post order.
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
     * @param chainId The host chain ID.
     * @param startBlock The first host chain block of epoch 0.
     * @param epochLength The number of host chain blocks per epoch, from epoch 0.
     */
    function setEpochSchedule(uint64 chainId, uint256 startBlock, uint256 epochLength) external;

    /**
     * @notice Sets a host chain's epoch length from an epoch on. DAO only.
     * @dev `fromEpoch` must be after the schedule's last one and after the highest epoch with a
     *      recorded root. The DAO picks it a few epochs ahead of the host chain.
     * @param chainId The host chain ID.
     * @param fromEpoch The first epoch with this length.
     * @param epochLength The number of host chain blocks per epoch.
     */
    function setEpochLength(uint64 chainId, uint256 fromEpoch, uint256 epochLength) external;

    /**
     * @notice Removes a host chain's epoch length, named by its first epoch. DAO only.
     * @dev Allowed while no epoch at or after `fromEpoch` has a recorded root; the first one, from
     *      epoch 0, never is.
     * @param chainId The host chain ID.
     * @param fromEpoch The first epoch of the epoch length to remove.
     */
    function removeEpochLength(uint64 chainId, uint256 fromEpoch) external;

    /**
     * @notice Posts the sender's root for a (host chain, epoch).
     * @dev The sender must be a tx sender of the current or, while still valid, the previous
     *      coprocessor context, and may post once per (host chain, epoch); late posts are accepted.
     *      Records the first posted root that reaches a context's majority threshold, if none is yet.
     * @param chainId The host chain ID.
     * @param epoch The epoch number.
     * @param root The root of the sender's tree for the epoch.
     */
    function postRoot(uint64 chainId, uint256 epoch, bytes32 root) external;

    /**
     * @notice Records the first posted root that reaches a majority threshold, without posting.
     * @dev Permissionless, for when a context switch makes an already posted root reach a threshold
     *      with no post to trigger the check. Reverts if a root is recorded or none reaches a threshold.
     * @param chainId The host chain ID.
     * @param epoch The epoch number.
     */
    function recordEpochRoot(uint64 chainId, uint256 epoch) external;
}
