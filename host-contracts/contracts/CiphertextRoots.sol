// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ICiphertextRoots} from "./interfaces/ICiphertextRoots.sol";
import {UUPSUpgradeableEmptyProxy} from "./shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable} from "./shared/ACLOwnable.sol";
import {protocolConfigAdd} from "../addresses/FHEVMHostAddresses.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

/**
 * @dev Temporary stand-in for `IProtocolConfigBase`: ProtocolConfig's coprocessor context reads
 *      (RFC 039), declared here until that interface carries them. Then delete this and import it.
 */
interface IProtocolConfigBaseShim {
    function getCurrentCoprocessorContextId() external view returns (uint256);
    function isValidCoprocessorContext(uint256 coprocessorContextId) external view returns (bool);
    function getCoprocessorMajorityThresholdForContext(uint256 coprocessorContextId) external view returns (uint256);
    function isCoprocessorTxSenderForContext(
        uint256 coprocessorContextId,
        address txSender
    ) external view returns (bool);
}

/**
 * @title CiphertextRoots contract
 * @notice The registry of ciphertext Merkle roots of RFC 034. See {ICiphertextRoots}.
 * @dev Append-only: no function, the DAO's included, modifies or deletes a posted root, a recorded
 *      epoch root or an epoch length that a recorded root was built under. An upgrade must preserve them.
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract CiphertextRoots is ICiphertextRoots, UUPSUpgradeableEmptyProxy, ACLOwnable {
    // -----------------------------------------------------------------------------------------
    // Contract information
    // -----------------------------------------------------------------------------------------

    string private constant CONTRACT_NAME = "CiphertextRoots";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    /// @dev Version used by `initializeFromEmptyProxy`'s `reinitializer` modifier.
    uint64 private constant REINITIALIZER_VERSION = 2;

    // -----------------------------------------------------------------------------------------
    // Other contract references
    // -----------------------------------------------------------------------------------------

    /// @notice The ProtocolConfig contract, source of the coprocessor contexts.
    /// @dev Use the shim until `IProtocolConfigBase` declares these functions.
    IProtocolConfigBaseShim private constant PROTOCOL_CONFIG = IProtocolConfigBaseShim(protocolConfigAdd);

    // -----------------------------------------------------------------------------------------
    // ERC-7201 namespaced storage
    // -----------------------------------------------------------------------------------------

    /// @dev The state of one epoch of a host chain.
    struct Epoch {
        /// @notice Distinct roots posted, in first-post order.
        bytes32[] postedRoots;
        /// @notice Senders of each posted root, in post order.
        mapping(bytes32 root => address[] senders) rootSenders;
        /// @notice Whether a sender already posted.
        mapping(address sender => bool posted) hasPosted;
        /// @notice The recorded root, meaningful once `recorded` is set to true.
        bytes32 root;
        /// @notice Whether a root is recorded.
        bool recorded;
    }

    /// @dev The state of one host chain.
    struct HostChain {
        /// @notice The epoch schedule, meaningful once `scheduleSet`.
        ICiphertextRoots.EpochSchedule schedule;
        /// @notice Whether the DAO set the epoch schedule.
        bool scheduleSet;
        /// @notice Whether any epoch has a recorded root.
        bool anyRootRecorded;
        /// @notice The highest epoch with a recorded root, meaningful once `anyRootRecorded`.
        uint256 highestRecordedEpoch;
        /// @notice The epochs, by number.
        mapping(uint256 epoch => Epoch) epochs;
    }

    /// @custom:storage-location erc7201:fhevm.storage.CiphertextRoots
    struct CiphertextRootsStorage {
        mapping(uint64 chainId => HostChain chain) chains;
    }

    /// @dev keccak256(abi.encode(uint256(keccak256("fhevm.storage.CiphertextRoots")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant CIPHERTEXT_ROOTS_STORAGE_LOCATION =
        0x3cb20319aa5e29e6b1c7b40fc8b1cf52b6eafa219f4e9b747b4b0c9dc1f12000;

    /// @dev In-memory helper: the current and previous coprocessor contexts, read from ProtocolConfig
    ///      once per call by `_loadContexts`.
    struct Contexts {
        uint256 currentId;
        uint256 previousId;
        bool previousValid;
        uint256 currentThreshold;
        uint256 previousThreshold;
    }

    // -----------------------------------------------------------------------------------------
    // Constructor and initialization
    // -----------------------------------------------------------------------------------------

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice Initializes the contract.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    // solhint-disable-next-line no-empty-blocks
    function initializeFromEmptyProxy() public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {}

    // -----------------------------------------------------------------------------------------
    // State-changing functions
    // -----------------------------------------------------------------------------------------

    /// @inheritdoc ICiphertextRoots
    function setEpochSchedule(uint64 chainId, uint256 startBlock, uint256 epochLength) external virtual onlyACLOwner {
        if (chainId == 0) {
            revert InvalidChainId();
        }
        if (epochLength == 0) {
            revert InvalidEpochLength();
        }
        HostChain storage chain = _getCiphertextRootsStorage().chains[chainId];
        if (chain.scheduleSet) {
            revert EpochScheduleAlreadySet(chainId);
        }
        chain.scheduleSet = true;
        EpochSchedule storage schedule = chain.schedule;
        schedule.startBlock = startBlock;
        schedule.entries.push(EpochScheduleEntry({fromEpoch: 0, epochLength: epochLength}));
        emit EpochScheduleSet(chainId, startBlock, epochLength);
    }

    /// @inheritdoc ICiphertextRoots
    function setEpochLength(uint64 chainId, uint256 fromEpoch, uint256 epochLength) external virtual onlyACLOwner {
        if (epochLength == 0) {
            revert InvalidEpochLength();
        }
        HostChain storage chain = _hostChain(chainId);
        EpochScheduleEntry[] storage entries = chain.schedule.entries;
        // The lowest accepted first epoch: above the schedule's last one and above every epoch with a
        // recorded root.
        uint256 minFromEpoch = entries[entries.length - 1].fromEpoch + 1;
        if (chain.anyRootRecorded && chain.highestRecordedEpoch >= minFromEpoch) {
            minFromEpoch = chain.highestRecordedEpoch + 1;
        }
        if (fromEpoch < minFromEpoch) {
            revert FromEpochTooLow(chainId, fromEpoch, minFromEpoch);
        }
        entries.push(EpochScheduleEntry({fromEpoch: fromEpoch, epochLength: epochLength}));
        emit EpochLengthSet(chainId, fromEpoch, epochLength);
    }

    /// @inheritdoc ICiphertextRoots
    function removeEpochLength(uint64 chainId, uint256 fromEpoch) external virtual onlyACLOwner {
        HostChain storage chain = _hostChain(chainId);
        EpochScheduleEntry[] storage entries = chain.schedule.entries;
        uint256 count = entries.length;
        // The first entry (from epoch 0) is never removed: the search starts after it.
        uint256 i = 1;
        while (i < count && entries[i].fromEpoch != fromEpoch) {
            i++;
        }
        if (i == count) {
            revert EpochLengthNotFound(chainId, fromEpoch);
        }
        if (chain.anyRootRecorded && fromEpoch <= chain.highestRecordedEpoch) {
            revert FromEpochTooLow(chainId, fromEpoch, chain.highestRecordedEpoch + 1);
        }
        // Shift the later entries one place down, then drop the duplicated last one.
        for (uint256 j = i; j < count - 1; j++) {
            entries[j] = entries[j + 1];
        }
        entries.pop();
        emit EpochLengthRemoved(chainId, fromEpoch);
    }

    /// @inheritdoc ICiphertextRoots
    function postRoot(uint64 chainId, uint256 epoch, bytes32 root) external virtual {
        HostChain storage chain = _hostChain(chainId);
        Epoch storage epochState = chain.epochs[epoch];
        if (epochState.hasPosted[msg.sender]) {
            revert RootAlreadyPosted(chainId, epoch, msg.sender);
        }
        Contexts memory contexts = _loadContexts();
        if (!_isCoprocessorTxSender(contexts, msg.sender)) {
            revert NotCoprocessorTxSender(msg.sender);
        }

        // First record a root that a context change already brought to a threshold, if any, so that
        // the first root to reach a threshold wins. Then add the post and try to record again.
        bool recorded = epochState.recorded || _tryRecordEpochRoot(chainId, epoch, contexts);

        epochState.hasPosted[msg.sender] = true;
        // A root posted for the first time joins the epoch's list of distinct roots.
        address[] storage senders = epochState.rootSenders[root];
        if (senders.length == 0) {
            epochState.postedRoots.push(root);
        }
        senders.push(msg.sender);
        emit RootPosted(chainId, epoch, msg.sender, root);

        if (!recorded) {
            _tryRecordEpochRoot(chainId, epoch, contexts);
        }
    }

    /// @inheritdoc ICiphertextRoots
    function recordEpochRoot(uint64 chainId, uint256 epoch) external virtual {
        HostChain storage chain = _hostChain(chainId);
        Epoch storage epochState = chain.epochs[epoch];
        if (epochState.recorded) {
            revert EpochRootAlreadyRecorded(chainId, epoch);
        }
        if (!_tryRecordEpochRoot(chainId, epoch, _loadContexts())) {
            revert NoRootReachesThreshold(chainId, epoch);
        }
    }

    // -----------------------------------------------------------------------------------------
    // View functions
    // -----------------------------------------------------------------------------------------

    /// @inheritdoc ICiphertextRoots
    function getEpochSchedule(uint64 chainId) external view virtual returns (EpochSchedule memory) {
        return _hostChain(chainId).schedule;
    }

    // How `getEpochOfBlock` and `getEpochBlocks` work.
    //
    // Example schedule: startBlock = 10, entries = [(0, 5), (5, 20), (8, 10)], each one
    // written as (fromEpoch, epochLength).
    //
    //  from epoch 0, 5 blocks   from epoch 5, 20 blocks          from epoch 8, 10 blocks
    //  |                        |                                |
    //  | e0 | e1 | e2 | e3 | e4 |    e5    |    e6    |    e7    |  e8  |  e9  | ...
    //  10   15   20   25   30   35         55         75         95     105    115   <- first block
    //
    // An entry starts at the block after the previous entry's last block:
    //   35 = 10 + (5 - 0) * 5      95 = 35 + (8 - 5) * 20
    //
    // getEpochOfBlock(60):
    //   (0, 5)  covers epochs 0 to 4, blocks 10 to 34: block 60 is past it   -> next entry
    //   (5, 20) covers epochs 5 to 7, blocks 35 to 94: block 60 is inside it -> stop
    //   epoch = fromEpoch + (block - entry's first block) / epochLength
    //         = 5 + (60 - 35) / 20 = 6
    //
    // getEpochBlocks(9):
    //   (0, 5)  covers epochs 0 to 4: epoch 9 is past it                -> next entry
    //   (5, 20) covers epochs 5 to 7: epoch 9 is past it                -> next entry
    //   (8, 10) is the last entry, first block 95: epoch 9 is inside it -> stop
    //   first block = entry's first block + (epoch - fromEpoch) * epochLength
    //               = 95 + (9 - 8) * 10 = 105
    //   last block  = first block + epochLength - 1 = 105 + 10 - 1 = 114

    /// @inheritdoc ICiphertextRoots
    function getEpochOfBlock(uint64 chainId, uint256 blockNumber) external view virtual returns (uint256) {
        EpochSchedule storage schedule = _hostChain(chainId).schedule;
        if (blockNumber < schedule.startBlock) {
            revert BlockBeforeStartBlock(chainId, blockNumber);
        }

        EpochScheduleEntry[] storage entries = schedule.entries;
        EpochScheduleEntry storage entry = entries[0];
        uint256 entryFirstBlock = schedule.startBlock;
        for (uint256 i = 1; i < entries.length; i++) {
            EpochScheduleEntry storage nextEntry = entries[i];

            // The next entry starts right after the current entry's epochs end.
            uint256 epochsInEntry = nextEntry.fromEpoch - entry.fromEpoch;
            uint256 nextEntryFirstBlock = entryFirstBlock + epochsInEntry * entry.epochLength;

            // If the next entry starts after the block, the block belongs to the current entry.
            if (blockNumber < nextEntryFirstBlock) {
                break;
            }

            entry = nextEntry;
            entryFirstBlock = nextEntryFirstBlock;
        }
        return entry.fromEpoch + (blockNumber - entryFirstBlock) / entry.epochLength;
    }

    /// @inheritdoc ICiphertextRoots
    function getEpochBlocks(
        uint64 chainId,
        uint256 epoch
    ) external view virtual returns (uint256 startBlock, uint256 endBlock) {
        EpochSchedule storage schedule = _hostChain(chainId).schedule;

        EpochScheduleEntry[] storage entries = schedule.entries;
        EpochScheduleEntry storage entry = entries[0];
        uint256 entryFirstBlock = schedule.startBlock;
        for (uint256 i = 1; i < entries.length; i++) {
            EpochScheduleEntry storage nextEntry = entries[i];

            // If the next entry starts after the epoch, the epoch belongs to the current entry.
            if (epoch < nextEntry.fromEpoch) {
                break;
            }

            // The next entry starts right after the current entry's epochs end.
            uint256 epochsInEntry = nextEntry.fromEpoch - entry.fromEpoch;
            entryFirstBlock += epochsInEntry * entry.epochLength;
            entry = nextEntry;
        }

        // Count the blocks from the entry's first block to the epoch.
        startBlock = entryFirstBlock + (epoch - entry.fromEpoch) * entry.epochLength;
        endBlock = startBlock + entry.epochLength - 1;
    }

    /// @inheritdoc ICiphertextRoots
    function getHighestRecordedEpoch(uint64 chainId) external view virtual returns (uint256) {
        HostChain storage chain = _hostChain(chainId);
        if (!chain.anyRootRecorded) {
            revert NoEpochRootRecorded(chainId);
        }
        return chain.highestRecordedEpoch;
    }

    /// @inheritdoc ICiphertextRoots
    function getEpochRoot(uint64 chainId, uint256 epoch) external view virtual returns (bytes32) {
        Epoch storage epochState = _hostChain(chainId).epochs[epoch];
        if (!epochState.recorded) {
            revert NoEpochRoot(chainId, epoch);
        }
        return epochState.root;
    }

    /// @inheritdoc ICiphertextRoots
    function getPostedRoots(uint64 chainId, uint256 epoch) external view virtual returns (bytes32[] memory) {
        return _hostChain(chainId).epochs[epoch].postedRoots;
    }

    /// @inheritdoc ICiphertextRoots
    function getRootSenders(
        uint64 chainId,
        uint256 epoch,
        bytes32 root
    ) external view virtual returns (address[] memory) {
        return _hostChain(chainId).epochs[epoch].rootSenders[root];
    }

    /// @inheritdoc ICiphertextRoots
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

    /// @dev Returns a host chain's state, reverting if it has no schedule.
    function _hostChain(uint64 chainId) internal view virtual returns (HostChain storage chain) {
        chain = _getCiphertextRootsStorage().chains[chainId];
        if (!chain.scheduleSet) {
            revert NoEpochSchedule(chainId);
        }
    }

    /**
     * @dev The previous context is the current ID minus one (RFC 039). It counts only while
     *      ProtocolConfig still reports it valid, i.e. until governance destroys it.
     */
    function _loadContexts() internal view virtual returns (Contexts memory contexts) {
        // The current context is always valid, no need to check explicitly here.
        contexts.currentId = PROTOCOL_CONFIG.getCurrentCoprocessorContextId();
        contexts.currentThreshold = PROTOCOL_CONFIG.getCoprocessorMajorityThresholdForContext(contexts.currentId);
        contexts.previousId = contexts.currentId - 1;
        contexts.previousValid = PROTOCOL_CONFIG.isValidCoprocessorContext(contexts.previousId);
        if (contexts.previousValid) {
            contexts.previousThreshold = PROTOCOL_CONFIG.getCoprocessorMajorityThresholdForContext(contexts.previousId);
        }
    }

    function _isCoprocessorTxSender(Contexts memory contexts, address sender) internal view virtual returns (bool) {
        if (PROTOCOL_CONFIG.isCoprocessorTxSenderForContext(contexts.currentId, sender)) {
            return true;
        }
        return contexts.previousValid && PROTOCOL_CONFIG.isCoprocessorTxSenderForContext(contexts.previousId, sender);
    }

    /**
     * @dev Counts the senders of a root that are members of the current context and of the previous
     *      one. Returns the context whose threshold is reached, the current one first.
     */
    function _reachesMajorityThreshold(
        Contexts memory contexts,
        address[] storage senders
    ) internal view virtual returns (bool reached, uint256 coprocessorContextId) {
        uint256 senderCount = senders.length;
        bool currentReachable = senderCount >= contexts.currentThreshold;
        bool previousReachable = contexts.previousValid && senderCount >= contexts.previousThreshold;
        if (!currentReachable && !previousReachable) {
            return (false, 0);
        }
        uint256 currentCount;
        uint256 previousCount;
        for (uint256 i = 0; i < senderCount; i++) {
            address sender = senders[i];
            if (currentReachable && PROTOCOL_CONFIG.isCoprocessorTxSenderForContext(contexts.currentId, sender)) {
                currentCount++;
            }
            if (previousReachable && PROTOCOL_CONFIG.isCoprocessorTxSenderForContext(contexts.previousId, sender)) {
                previousCount++;
            }
        }
        if (currentReachable && currentCount >= contexts.currentThreshold) {
            return (true, contexts.currentId);
        }
        if (previousReachable && previousCount >= contexts.previousThreshold) {
            return (true, contexts.previousId);
        }
        return (false, 0);
    }

    /**
     * @dev Records the epoch's root if a posted root reaches a majority threshold under the given
     *      contexts: the first such root in first-post order. Returns whether one was recorded.
     */
    function _tryRecordEpochRoot(
        uint64 chainId,
        uint256 epoch,
        Contexts memory contexts
    ) internal virtual returns (bool) {
        Epoch storage epochState = _hostChain(chainId).epochs[epoch];
        bytes32[] storage roots = epochState.postedRoots;
        for (uint256 i = 0; i < roots.length; i++) {
            bytes32 root = roots[i];
            (bool reached, uint256 contextId) = _reachesMajorityThreshold(contexts, epochState.rootSenders[root]);
            if (reached) {
                _recordEpochRoot(chainId, epoch, root, contextId);
                return true;
            }
        }
        return false;
    }

    /// @dev Records the epoch's root and emits `EpochRootRecorded`. Reverts if the epoch already has one.
    function _recordEpochRoot(
        uint64 chainId,
        uint256 epoch,
        bytes32 root,
        uint256 coprocessorContextId
    ) internal virtual {
        HostChain storage chain = _hostChain(chainId);
        Epoch storage epochState = chain.epochs[epoch];
        if (epochState.recorded) {
            revert EpochRootAlreadyRecorded(chainId, epoch);
        }
        epochState.root = root;
        epochState.recorded = true;
        if (!chain.anyRootRecorded) {
            chain.anyRootRecorded = true;
            chain.highestRecordedEpoch = epoch;
        } else if (epoch > chain.highestRecordedEpoch) {
            chain.highestRecordedEpoch = epoch;
        }
        emit EpochRootRecorded(chainId, epoch, root, coprocessorContextId);
    }

    /**
     * @notice Returns the CiphertextRoots storage location.
     * @dev Internal but not virtual: a derived contract that changes the storage struct must define
     *      its own getter rather than override the storage location.
     */
    function _getCiphertextRootsStorage() internal pure returns (CiphertextRootsStorage storage $) {
        // solhint-disable-next-line no-inline-assembly
        assembly {
            $.slot := CIPHERTEXT_ROOTS_STORAGE_LOCATION
        }
    }

    /**
     * @dev Authorization for UUPS upgrades.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyACLOwner {}
}
